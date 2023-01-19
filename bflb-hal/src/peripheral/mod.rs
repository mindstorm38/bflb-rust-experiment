//! Actual HAL modules, most are common to all chips and some are feature-gated
//! to some specific chips.

pub mod clock;
pub mod time;

pub mod gpio;
pub mod dma;
pub mod uart;
pub mod usb;

pub mod irq;

// pub mod camera;
// pub mod mjpeg;
