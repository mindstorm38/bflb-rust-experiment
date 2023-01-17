#![no_std]
#![no_main]

use bflb_hal::clock::{Clocks, XtalType, Mux2, UartSel};
use bflb_hal::uart::{UartPort, UartConfig};
use bflb_hal::time::CoreTimer;
use bflb_hal::gpio::PinPort;
use bflb_hal::irq::IrqNum;

use embedded_util::Peripheral;

use core::sync::atomic::{AtomicBool, Ordering};
use core::fmt::Write;


bflb_rt::entry!(main);

fn main() {

    let mut clocks = Clocks::borrow();

    // CHIP.cpu().halt_d0();
    // CHIP.cpu().halt_lp();

    // CLOCK INIT

    clocks.set_xtal_type(XtalType::Mhz40);
    clocks.enable_xtal().unwrap();
    
    clocks.set_xclk_sel(Mux2::Sel0);    // RC32M
    clocks.set_m0_root_sel(Mux2::Sel0); // xclock
    clocks.set_m0_cpu_div(1);
    clocks.set_m0_secondary_div(1);
    clocks.set_m0_secondary_div_act_pulse(true);
    while !clocks.get_m0_secondary_prot_done() {}
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

    clocks.set_xclk_sel(Mux2::Sel1);    // Xtal
    clocks.set_mm_xclk_sel(Mux2::Sel1); // Xtal

    // PERIPHERAL INIT

    clocks.set_uart_enable(false);
    clocks.set_d0_cpu_div(1);
    clocks.set_uart_sel(UartSel::Xclock);
    clocks.set_uart_enable(true);
    clocks.set_uart0_enable(true);
    
    // CONSOLE INIT

    let uart_tx = PinPort::<14>::take();
    let uart_rx = PinPort::<15>::take();
    let mut uart = UartPort::<0>::take()
        .into_duplex(uart_tx, uart_rx, &UartConfig::new(115200), &clocks);

    let mut timer = CoreTimer::borrow();
    timer.init(&mut clocks);
    timer.set_time(0);
    timer.set_time_cmp(1_000_000);
    drop(timer);

    let mtimer_int = bflb_rt::get_interrupt(IrqNum::MachineTimer);
    mtimer_int.set_handler(mtimer_handler);
    mtimer_int.set_enable(true);
    mtimer_int.set_level(255);

    loop {
        
        if INTERRUPTED.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            let timer = CoreTimer::borrow();
            write!(uart, "Interrupted! RTC time: {}\r\n", timer.get_time()).unwrap();
        }

    }

}


static INTERRUPTED: AtomicBool = AtomicBool::new(false);
fn mtimer_handler(_code: usize) {
    let mut timer = CoreTimer::borrow();
    let time_cmp = timer.get_time_cmp();
    timer.set_time_cmp(time_cmp + 1_000_000);
    INTERRUPTED.store(true, Ordering::Relaxed);
}
