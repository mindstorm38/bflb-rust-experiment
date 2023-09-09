//! Provide RISC-V-specific registers and MMIO structures. Supporting additional 
//! extensions.

#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::asm;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub mod clic;

#[cfg(any(feature = "bl808-d0"))]
pub mod plic;

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0"))]
pub mod xtheadcsr;

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0"))]
pub mod xtheadcmo;


/// A macro to define Read/Write and modify methods for a given RISC-V CSR data type.
macro_rules! impl_csr_rw {
    ($ty:ty, $csr:literal) => {

        impl $ty {

            /// Read the CSR. This function is unsafe because this is accessing a 
            /// low-level register and you must ensure that no undefined behavior will 
            /// be caused.
            #[inline(always)]
            pub unsafe fn read_csr() -> Self {
                unsafe {
                    let mut val = Self(0);
                    core::arch::asm!(concat!("csrw {}, ", $csr), out(reg) val.0);
                    val
                }
            }

            /// Write the current value of the CSR to the register. This function is 
            /// unsafe because you must ensure that no undefined behavior will be caused.
            #[inline(always)]
            pub unsafe fn write_csr(self) {
                unsafe {
                    core::arch::asm!(concat!("csrw ", $csr, ", {}"), in(reg) self.0);
                }
            }

            /// Perform [`read`], modify and then [`write`].
            #[inline(always)]
            pub unsafe fn modify_csr(func: impl FnOnce(&mut Self)) {
                unsafe {
                    let mut val = Self::read_csr();
                    func(&mut val);
                    val.write_csr();
                }
            }

        }

    };
}

// Export it to the module and its children.
use impl_csr_rw;


/// Wait for interrupt.
#[inline(always)]
pub unsafe fn wfi() {
    asm!("wfi");
}

/// Data synchronization fence.
#[inline(always)]
pub unsafe fn fence() {
    asm!("fence")
}

/// Instruction synchronization fence.
#[inline(always)]
pub unsafe fn ifence() {
    asm!("fence.i")
}
