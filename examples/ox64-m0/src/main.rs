#![no_std]
#![no_main]

extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
use alloc::boxed::Box;
use core::fmt::Write;

use bflb_rt::hal;

use hal::clock::{XtalType, UartSel, McuRootSel, XclkSel, MmXclkSel};
use hal::cache::CacheAligned;
use hal::uart::UartConfig;
use hal::Peripherals;
use hal::interrupt;
use hal::time;


#[no_mangle]
pub fn main() {

    let peripherals = Peripherals::take();
    let mut clocks = peripherals.clocks;
    let mut cpu_control = peripherals.cpu_control;
    let timer = peripherals.timer;

    clocks.set_d0_cpu_enable(false);
    cpu_control.reset_d0();

    // CLOCK INIT

    clocks.set_xtal_type(XtalType::Mhz40);
    clocks.enable_xtal().unwrap();
    
    clocks.set_xclk_sel(XclkSel::Rc32m);
    clocks.set_mcu_root_sel(McuRootSel::Xclk);
    clocks.set_m0_cpu_div(1);
    clocks.set_mcu_pbclk_div(1);
    clocks.set_mcu_pbclk_div_act_pulse(true);
    while !clocks.get_mcu_pbclk_prot_done() {}
    clocks.set_lp_cpu_div(1);
    clocks.set_lp_cpu_div_act_pulse(true);
    while !clocks.get_lp_cpu_prot_done() {}

    // On D0:
    // clocks.set_mm_xclk_sel(Mux2::Sel0); // RC32M
    // clocks.set_d0_root_sel(Mux2::Sel0); // MM xclock
    // clocks.set_d0_cpu_div(1);
    // clocks.set_d0_secondary_div(1);
    // clocks.set_d0_secondary_div_act_pulse(true);
    // while !clocks.get_d0_secondary_prot_done() {}

    // Note that we don't activate any PLL.

    clocks.set_xclk_sel(XclkSel::Xtal);
    clocks.set_mm_xclk_sel(MmXclkSel::Xtal);

    // PERIPHERAL INIT

    clocks.set_dma_enable(true);
    
    clocks.setup_mcu_uart(UartSel::Xclk, 1, true);
    clocks.set_mcu_uart0_enable(true);
    
    clocks.set_adc_dac_enable(true);
    
    timer.init(&mut clocks);
    
    // INTERRUPT INIT
    
    // interrupts.set_enabled(interrupt::MACHINE_TIMER, true);
    // interrupts.set_enabled(interrupt::DMA0_ALL, true);
    // interrupts.set_enabled(interrupt::DMA1_ALL, true);

    // CONSOLE INIT
    let mut uart = peripherals.uart.p0.init_duplex(
        peripherals.gpio.p14, 
        peripherals.gpio.p15, 
        &UartConfig::new(115200), 
        &clocks
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
