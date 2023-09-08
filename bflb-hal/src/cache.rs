//! Architecture-agnostic cache management functions. The provided functions are unsafe
//! because you have to manually ensure that given addresses and calls will not break
//! memory safety in the program.

use crate::arch::riscv;


#[cfg(any(feature = "bl808-m0"))]
pub const LINE_SIZE: usize = 32;
#[cfg(any(feature = "bl808-d0"))]
pub const LINE_SIZE: usize = 64;


/// Invalid the L1 data cache memory of the given pointer and size.
/// 
/// If given size if 0, no cache line will be invalidated.
/// 
/// **This function is unsafe** because invalidating a cache line will not write back any
/// dirty cache line, therefore you must ensure that the designated cache lines are not 
/// dirty in order to avoid undefined behavior later in the program.
pub unsafe fn l1d_invalidate(addr: usize, size: usize) {

    // Note: there is no need to align the given address, because the instruction just
    // state that the cache line of this address will be given, so if we increment this
    // address by the cache line size, we clear the next one, and so on...
    let mut addr = addr;
    let end_addr = addr + size;

    unsafe { riscv::fence(); }
    while addr < end_addr {
        unsafe { riscv::xtheadcmo::dcache_ipa(addr); }
        addr += LINE_SIZE;
    }
    unsafe { riscv::fence(); }

}

/// Clean the L1 data cache memory of the given pointer and size. This basically write 
/// back all dirty cache lines.
/// 
/// If given size if 0, no cache line will be cleaned.
pub fn l1d_clean(addr: usize, size: usize) {

    let mut addr = addr;
    let end_addr = addr + size;

    unsafe { riscv::fence(); }
    while addr < end_addr {
        unsafe { riscv::xtheadcmo::dcache_cpal1(addr); }
        addr += LINE_SIZE;
    }
    unsafe { riscv::fence(); }

}
