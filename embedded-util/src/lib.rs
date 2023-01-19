//! Embedded Rust Utilities

#![no_std]

pub mod atomic;

mod reg;
pub use reg::{Reg, RegPtr};

mod mmio;
pub use mmio::{PtrRo, PtrWo, PtrRw};

mod peripheral;
pub use peripheral::{Peripheral, PeripheralGuard};
