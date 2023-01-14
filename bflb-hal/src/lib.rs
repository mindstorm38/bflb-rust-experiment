//! BouffaloLab Hardware Abstraction Layers.

#![no_std]

pub mod peripheral;
pub mod register;


#[cfg(feature = "bl808_m0")]
mod bl808_m0;
#[cfg(feature = "bl808_m0")]
pub use bl808_m0::*;
