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

    let misalignment = addr % LINE_SIZE;
    let mut aligned_size = size + misalignment;
    let mut aligned_addr = addr - misalignment;

    unsafe {
        riscv::std::fence();
        while aligned_size > 0 {
            riscv::theadcmo::dcache_ipa(aligned_addr);
            aligned_addr += LINE_SIZE;
            aligned_size -= LINE_SIZE;
        }
        riscv::std::fence();
    }

}
