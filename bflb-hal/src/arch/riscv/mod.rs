//! Provide RISC-V-specific registers and MMIO structures. Supporting additional 
//! extensions.

pub mod std;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub mod clic;

#[cfg(any(feature = "bl808-d0"))]
pub mod plic;

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0", feature = "bl808-lp"))]
pub mod theadcmo;
