#![no_std]
#![no_main]

use bflb_hal::clock::{Clocks, XtalType, Mux2, UartClockSel};
use bflb_hal::cpu::CpuControl;
use bflb_hal::uart::{UartAccess, UartConfig};
use bflb_hal::time::CoreTimer;
use bflb_hal::gpio::PinAccess;
use bflb_hal::dma::DmaAccess;
use bflb_hal::irq::IrqNum;

use embedded_util::Peripheral;

use core::sync::atomic::{AtomicBool, Ordering};
use core::fmt::Write;


#[link_section = ".data"] // Loaded in RAM
static DMA_MESSAGE: &'static str = "Hello world from DMA!\n";


bflb_rt::entry!(main);

fn main() {

    let mut clocks = Clocks::take();
    let mut cpu_control = CpuControl::take();

    // CHIP.cpu().halt_d0();
    // CHIP.cpu().halt_lp();

    clocks.set_d0_enable(false);
    cpu_control.reset_d0();

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

    clocks.set_dma_enable(true);
    clocks.set_mcu_uart0_enable(true);
    clocks.setup_mcu_uart(UartClockSel::Xclock, 1, true);
    
    // CONSOLE INIT

    let uart_tx = PinAccess::<14>::take();
    let uart_rx = PinAccess::<15>::take();

    let (
        uart_tx, 
        mut uart_rx
    ) = UartAccess::<0>::take()
        .into_duplex(uart_tx, uart_rx, &UartConfig::new(115200), &clocks);

    // DMA

    let dma_transfer = DmaAccess::<0, 0>::take()
        .into_transfer(DMA_MESSAGE, uart_tx);

    let (
        _src, 
        mut uart_tx, 
        _dma,
    ) = dma_transfer.wait();

    // INTS

    let mut timer = CoreTimer::borrow();
    timer.init(&mut clocks);
    timer.set_time(0);
    timer.set_time_cmp(1_000_000);
    drop(timer);

    let mtimer_int = bflb_rt::get_interrupt(IrqNum::MachineTimer);
    mtimer_int.set_handler(mtimer_handler);
    mtimer_int.set_enable(true);
    mtimer_int.set_level(255);

    // LOOP

    loop {

        while let Some(b) = uart_rx.read_byte() {
            uart_tx.write_byte(b);
        }
        
        if INTERRUPTED.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            mtimer_int.set_enable(false);
            {
                let _ = writeln!(uart_tx, "Interrupted! RTC time: {}", CoreTimer::borrow().get_time());
            }
            mtimer_int.set_enable(true);
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
