//! This module provides standard RISC-V instruction wrappers.

#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::asm;


#[inline]
pub unsafe fn fence() {
    asm!("fence")
}
