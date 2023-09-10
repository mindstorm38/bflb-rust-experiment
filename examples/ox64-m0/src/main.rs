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


#[no_mangle]
pub fn main() {

    let peripherals = Peripherals::take();

    // CONSOLE INIT
    let mut uart = peripherals.uart.p0.init_duplex(
        peripherals.gpio.p14, 
        peripherals.gpio.p15, 
        &UartConfig::new(115200), 
    );

    static INTERRUPTED: AtomicBool = AtomicBool::new(false);

    let dest = Box::new(CacheAligned([0u8; 22]));

    #[link_section = ".data"] // Loaded in RAM
    static DMA_MESSAGE: &'static str = "Hello world from DMA!\n";

    let (_, dest, _dma) = peripherals.dma.p0.c0
        .into_transfer(DMA_MESSAGE, dest)
        .wait();

    let _ = write!(uart, "src: {DMA_MESSAGE}");
    let _ = write!(uart, "dst: {}", core::str::from_utf8(&**dest).unwrap());

    // let (_, mut uart, _) = peripherals.dma.p0.c0
    //     .into_transfer(DMA_MESSAGE, uart)
    //     .wait();

    // .wait_callback(|_, mut uart, _| {
    //     let _ = writeln!(uart, "RTC time: {}", time::get_time());
    // });

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

    // // DMA
    // peripherals.dma.p0.c0
    //     .into_transfer(DMA_MESSAGE, uart)
    //     .wait_callback(move |_, mut uart, _| {

    //         // LOOP
    //         time::wait_callback(0, move || {
    //             let _ = writeln!(uart, "RTC time: {}", time::get_time());
    //             Some(1_000_000)
    //         });

    //     });
    
}
