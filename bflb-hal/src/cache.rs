//! Architecture-agnostic cache management functions. The provided functions are unsafe
//! because you have to manually ensure that given addresses and calls will not break
//! memory safety in the program.

#![allow(unsafe_op_in_unsafe_fn)]

use core::ops::{Deref, DerefMut};

use crate::hart::{data_sync, inst_sync};
use crate::arch::riscv::xtheadcsr::Mhcr;
use crate::arch::riscv;


#[cfg(any(feature = "bl808-m0"))]
pub const LINE_SIZE: usize = 32;
#[cfg(any(feature = "bl808-d0"))]
pub const LINE_SIZE: usize = 64;


/// Enable the instruction cache.
#[inline]
pub unsafe fn enable_inst() {
    data_sync();
    inst_sync();
    riscv::xtheadcmo::icache_iall();
    Mhcr::modify_csr(|mhcr| mhcr.ie().fill());
    data_sync();
    inst_sync();
}

/// Disable the instruction cache.
#[inline]
pub unsafe fn disable_inst() {
    data_sync();
    inst_sync();
    Mhcr::modify_csr(|mhcr| mhcr.ie().clear());
    riscv::xtheadcmo::icache_iall();
    data_sync();
    inst_sync();
}

/// Invalidate the whole instruction cache, this will force next instructions to be 
/// fetched from cache or memory.
#[inline]
pub unsafe fn invalidate_inst() {
    data_sync();
    inst_sync();
    riscv::xtheadcmo::icache_iall();
    data_sync();
    inst_sync();
}


/// Enable the data cache.
#[inline]
pub unsafe fn enable_data() {
    data_sync();
    inst_sync();
    riscv::xtheadcmo::dcache_iall();
    Mhcr::modify_csr(|mhcr| {
        mhcr.de().fill();
        mhcr.wb().fill();
        mhcr.wa().fill();
        mhcr.rs().fill();
        mhcr.bpe().fill();
        mhcr.btb().fill();
    });
    data_sync();
    inst_sync();
}

/// Disable the data cache.
#[inline]
pub unsafe fn disable_data() {
    data_sync();
    inst_sync();
    Mhcr::modify_csr(|mhcr| mhcr.de().clear());
    riscv::xtheadcmo::dcache_iall();
    data_sync();
    inst_sync();
}

/// Invalidate the whole data cache. This will cause all subsequent memory accesses to
/// update the cache from memory. **You must be careful because this can invalidate
/// assumptions made by the compiler and therefore create undefined behaviors that are
/// really hard to debug.**
#[inline]
pub unsafe fn invalidate_data() {
    data_sync();
    riscv::xtheadcmo::dcache_iall();
    data_sync();
}

/// Clean the whole data cache. This will write-back all dirty cache line to the memory.
#[inline]
pub unsafe fn clean_data() {
    data_sync();
    riscv::xtheadcmo::dcache_call();
    data_sync();
}

/// Clean and then invalidate data. Read the documentation of both [`invalidate_data`] and
/// [`clean_data`] for details.
#[inline]
pub unsafe fn clean_invalidate_data() {
    data_sync();
    riscv::xtheadcmo::dcache_ciall();
    data_sync();
}

/// Invalidate the data cache for the given range. 
/// Please read [`invalidate_data`] for details.
#[inline]
pub unsafe fn invalidate_data_range(addr: usize, size: usize) {

    let mut addr = addr;
    let end_addr = addr + size;

    data_sync();
    while addr < end_addr {
        riscv::xtheadcmo::dcache_ipa(addr);
        addr += LINE_SIZE;
    }
    data_sync();

}

/// Invalidate the data cache for the given range. 
/// Please read [`clear_data`] for details.
#[inline]
pub unsafe fn clean_data_range(addr: usize, size: usize) {

    let mut addr = addr;
    let end_addr = addr + size;

    data_sync();
    while addr < end_addr {
        riscv::xtheadcmo::dcache_cpa(addr);
        addr += LINE_SIZE;
    }
    data_sync();

}

/// Clean and then invalidate the data cache for the given range. 
/// Please read [`clean_invalidate_data`] for details.
#[inline]
pub unsafe fn clean_invalidate_data_range(addr: usize, size: usize) {

    let mut addr = addr;
    let end_addr = addr + size;

    data_sync();
    while addr < end_addr {
        riscv::xtheadcmo::dcache_cipa(addr);
        addr += LINE_SIZE;
    }
    data_sync();

}

/// A wrapper for type that force it to be aligned to cache line and force this structure
/// to have a size multiple of the cache line size. This allows fixing unsound issues that
/// may appear.
#[cfg_attr(feature = "bl808-m0", repr(align(32)))]
#[cfg_attr(feature = "bl808-d0", repr(align(64)))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheAligned<T: ?Sized>(pub T);

impl<T: ?Sized> Deref for CacheAligned<T> {
    
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl<T: ?Sized> DerefMut for CacheAligned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
