//! Provide RISC-V-specific registers and MMIO structures. Supporting additional 
//! extensions.

#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::asm;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub mod clic;

#[cfg(any(feature = "bl808-d0"))]
pub mod plic;

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0"))]
pub mod xtheadcmo;


/// Data synchronization fence.
#[inline]
pub unsafe fn fence() {
    asm!("fence")
}

/// Instruction synchronization fence.
#[inline]
pub unsafe fn ifence() {
    asm!("fence.i")
}
