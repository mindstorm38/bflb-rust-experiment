#![no_std]
#![no_main]

use core::fmt::Write;

use bflb_rt::hal;

use hal::Peripherals;

use hal::clock::{XtalType, UartSel, McuRootSel, XclkSel, MmXclkSel};
use hal::uart::UartConfig;
use hal::interrupt;
use hal::time;


#[link_section = ".data"] // Loaded in RAM
static DMA_MESSAGE: &'static str = "Hello world from DMA!\n";


#[no_mangle]
pub fn main() {

    let peripherals = Peripherals::take();
    let mut clocks = peripherals.clocks;
    let mut cpu_control = peripherals.cpu_control;
    let mut interrupts = peripherals.interrupts;
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
    
    interrupts.set_enabled(interrupt::MACHINE_TIMER, true);
    // interrupts.set_enabled(interrupt::DMA0_ALL, true);
    // interrupts.set_enabled(interrupt::DMA1_ALL, true);

    // CONSOLE INIT
    let mut uart = peripherals.uart.p0.init_duplex(
        peripherals.gpio.p14, 
        peripherals.gpio.p15, 
        &UartConfig::new(115200), 
        &clocks
    );

    let _ = writeln!(uart, "RTC time: {}", time::get_time());
    let _ = writeln!(uart, "Heap: {:p} -> {:p}", 
        unsafe { &bflb_rt::sym::_ld_heap_start }, 
        unsafe { &bflb_rt::sym::_ld_heap_end });
    
    // // LOOP
    // time::wait_callback(0, move || {
    //     let _ = writeln!(uart, "RTC time: {}", time::get_time());
    //     Some(1_000_000)
    // });

    loop {
        hal::hart::spin_loop();
    }

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
