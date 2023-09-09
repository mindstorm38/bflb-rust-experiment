//! This modules contains all architecture and chip specific registers and MMIO 
//! structures. This module is not intended to provide a uniform abstraction.

// Always included because all supported chips are RISC-V.
pub mod riscv;

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0", feature = "bl808-lp"))]
pub mod bl808;
