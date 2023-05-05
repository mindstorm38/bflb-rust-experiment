//! BouffaloLab Hardware Abstraction Layers.

#![no_std]


#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
#[cfg(not(target_arch = "riscv32"))]
compile_error!("bl808 m0 chip requires 'riscv32' target architecture");

#[cfg(feature = "bl808-d0")]
#[cfg(not(target_arch = "riscv64"))]
compile_error!("bl808 d0 chip requires 'riscv64' target architecture");


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
