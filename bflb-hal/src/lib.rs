//! BouffaloLab Hardware Abstraction Layers.

#![no_std]

#[cfg(any(feature = "bl808-m0", feature = "bl808-d0", feature = "bl808-lp"))]
pub mod bl808;

// Currently, only BL808 chip is implemented so we simplify feature gates by 
// putting them here.
#[cfg(any(feature = "bl808-m0", feature = "bl808-d0", feature = "bl808-lp"))]
mod peripheral;
#[cfg(any(feature = "bl808-m0", feature = "bl808-d0", feature = "bl808-lp"))]
pub use peripheral::*;

/// Re-export of the RISC-V HAL library.
pub use riscv_hal as riscv;
