//! Embedded Rust Utilities

#![no_std]

mod reg;
pub use reg::{Reg, RegPtr};

mod mmio;
pub use mmio::{PtrRo, PtrWo, PtrRw};

mod peripheral;
// pub use peripheral::{Peripheral, PeripheralGuard};

mod atomic;
pub use atomic::AtomicOption;
