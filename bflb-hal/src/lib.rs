//! BouffaloLab Hardware Abstraction Layers.

#![no_std]

// extern crate alloc;

#[cfg(any(feature = "bl808_m0", feature = "bl808_d0", feature = "bl808_lp"))]
pub mod bl808;

// Currently, only BL808 chip is implemented so we simplify feature gates by 
// putting them here.
#[cfg(any(feature = "bl808_m0", feature = "bl808_d0", feature = "bl808_lp"))]
mod peripheral;
#[cfg(any(feature = "bl808_m0", feature = "bl808_d0", feature = "bl808_lp"))]
pub use peripheral::*;
