//! DMA (Direct Memory Access) peripherals.

use core::sync::atomic::AtomicBool;

// use crate::bl808::{DMA0, DMA1, DMA2, dma};

use embedded_util::{Peripheral, atomic};


/// Represent an exclusive access to a DMA channel on a particular port.
pub struct DmaChannel<const PORT: u8, const CHANNEL: u8>(());

impl<const PORT: u8, const CHANNEL: u8> Peripheral for DmaChannel<PORT, CHANNEL> {
    
    unsafe fn taken() -> &'static AtomicBool {

        debug_assert!(PORT < 3, "invalid dma port {PORT}");
        debug_assert!(CHANNEL < (if PORT == 1 { 4 } else { 8 }), "invalid dma channel {CHANNEL} for port {PORT}");

        static TAKEN_ARR: [AtomicBool; 20] = atomic::atomic_bool_array(false);

        match PORT {
            0 => &TAKEN_ARR[0  + CHANNEL as usize],
            2 => &TAKEN_ARR[8  + CHANNEL as usize],
            1 => &TAKEN_ARR[16 + CHANNEL as usize],
            _ => unreachable!()
        }

    }

    unsafe fn new() -> Self {
        Self(())
    }

}


impl<const PORT: u8, const CHANNEL: u8> DmaChannel<PORT, CHANNEL> {

    // TODO:

}