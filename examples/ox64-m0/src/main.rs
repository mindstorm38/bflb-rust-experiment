#![no_std]
#![no_main]

extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
use alloc::boxed::Box;
use core::fmt::Write;

use bflb_rt::hal;

use hal::cache::CacheAligned;
use hal::uart::UartConfig;
use hal::Peripherals;
use hal::time;


#[link_section = ".data"] // Loaded in RAM
static DMA_MESSAGE: [u8; 22] = *b"Hello world from DMA!\n";

static INTERRUPTED: AtomicBool = AtomicBool::new(false);


#[no_mangle]
pub fn main() {

    let peripherals = Peripherals::take();

    // CONSOLE INIT
    let mut uart = peripherals.uart.p0.init_duplex(
        peripherals.gpio.p14, 
        peripherals.gpio.p15, 
        &UartConfig::new(115200), 
    );

    // hal::clock::debug_clock_diagram(&mut uart).unwrap();

    let dma = peripherals.dma.p0.c0;

    let dst = Box::new(CacheAligned([0u8; 22]));

    dma.into_transfer(&DMA_MESSAGE, dst)
        .wait_callback(move |_, dst, _| {

            INTERRUPTED.store(true, Ordering::Relaxed);

            let _ = writeln!(uart, "dst: {:?}", core::str::from_utf8(&**dst).unwrap());

            // LOOP
            time::wait_callback(0, move || {

                let mut time_ms = time::get_time() / 1_000;

                let minutes = time_ms / 60_000;
                time_ms -= minutes * 60_000;

                let seconds = time_ms / 1_000;
                time_ms -= seconds * 1_000;

                let _ = writeln!(uart, "[{:02}:{:02}.{:03}] DMA interrupted: {}", minutes, seconds, time_ms, INTERRUPTED.load(Ordering::Relaxed));

                // Callback again in 1 second.
                Some(1_000_000)

            });

        });
    
}
