#![no_std]
#![no_main]

use core::fmt::Write;

use bflb_rt::{spawn, wait};
use bflb_rt::hal;

use hal::Peripherals;

use hal::clock::{XtalType, UartSel, McuRootSel, XclkSel, MmXclkSel};
use hal::uart::UartConfig;
use hal::interrupt::MACHINE_TIMER;


#[link_section = ".data"] // Loaded in RAM
static DMA_MESSAGE: &'static str = "Hello world from DMA!\n";


bflb_rt::entry!(main);

fn main() {

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
    
    // TIMER INIT
    
    timer.init(&mut clocks);
    interrupts.set_enabled(MACHINE_TIMER, true);

    // CONSOLE INIT

    let uart_tx = peripherals.gpio.p14.into_alternate();
    let uart_rx = peripherals.gpio.p15.into_alternate();

    let (
        uart_tx, 
        _uart_rx
    ) = peripherals.uart.p0.into_duplex(uart_tx, uart_rx, &UartConfig::new(115200), &clocks);

    // DMA

    let (_, mut uart_tx, _) = peripherals.dma.p0.c0
        .into_transfer(DMA_MESSAGE, uart_tx)
        .wait_destruct();
    
    // LOOP

    spawn(async move {
        
        loop {

            let _ = writeln!(uart_tx, "RTC time: {}", timer.get_time());
            timer.wait(1_000_000).await;

        }

    });

    // Run a wait for the end of spawned tasks (should not end).
    wait();

}
