//! Architecture-agnostic cache management functions. The provided functions are unsafe
//! because you have to manually ensure that given addresses and calls will not break
//! memory safety in the program.

use crate::arch::riscv;


#[cfg(target_pointer_width = "32")]
const LINE_SIZE: usize = 32;
#[cfg(target_pointer_width = "64")]
const LINE_SIZE: usize = 64;


/// Invalid the whole L1 data cache memory of the given pointer and size.
pub unsafe fn l1d_invalidate(addr: usize, size: usize) {

    // Note: there is no need to align the given address, because the instruction just
    // state that the cache line of this address will be given, so if we increment this
    // address by the cache line size, we clear the next one, and so on...
    let mut addr = addr;
    let end_addr = addr + size;

    unsafe { riscv::std::fence(); }
    while addr < end_addr {
        unsafe { riscv::theadcmo::dcache_ipa(addr); }
        addr += LINE_SIZE;
    }
    unsafe { riscv::std::fence(); }

}
