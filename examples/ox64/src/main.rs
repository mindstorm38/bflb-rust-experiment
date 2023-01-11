#![no_std]
#![no_main]

use bflb_hal::bl808::clock::{Clocks, XtalType, Mux2, UartSel};
use bflb_hal::bl808::time::CoreTimer;
use bflb_hal::bl808::uart::{Uart, UartPort, UartConfig};
use bflb_hal::bl808::gpio::Pin;
use bflb_hal::bl808::{get_core_id, CoreId, CoreM0};

use emhal::time::Timer;

use bflb_rt::IrqNum;

use core::sync::atomic::{AtomicBool, Ordering};
use core::fmt::Write;


static CLOCKS: Clocks<CoreM0> = Clocks::new(CoreM0);
static CORET: CoreTimer<CoreM0> = CoreTimer::new(CoreM0);


bflb_rt::entry!(main);

fn main() {

    // CHIP.cpu().halt_d0();
    // CHIP.cpu().halt_lp();

    // CLOCK INIT

    CLOCKS.set_xtal_type(XtalType::Mhz40);
    CLOCKS.enable_xtal().unwrap();
    
    match get_core_id().unwrap() {
        CoreId::M0 | CoreId::LP => {
            CLOCKS.set_xclk_sel(Mux2::Sel0);    // RC32M
            CLOCKS.set_m0_root_sel(Mux2::Sel0); // xclock
            CLOCKS.set_m0_cpu_div(1);
            CLOCKS.set_m0_secondary_div(1);
            CLOCKS.set_m0_secondary_div_act_pulse(true);
            while !CLOCKS.get_m0_secondary_prot_done() {}
            CLOCKS.set_lp_cpu_div(1);
            CLOCKS.set_lp_cpu_div_act_pulse(true);
            while !CLOCKS.get_lp_cpu_prot_done() {}
            // timer.sleep_dummy_nop();
        }
        CoreId::D0 => {
            CLOCKS.set_mm_xclk_sel(Mux2::Sel0); // RC32M
            CLOCKS.set_d0_root_sel(Mux2::Sel0); // MM xclock
            CLOCKS.set_d0_cpu_div(1);
            CLOCKS.set_d0_secondary_div(1);
            CLOCKS.set_d0_secondary_div_act_pulse(true);
            while !CLOCKS.get_d0_secondary_prot_done() {}
        }
    }

    // Note that we don't activate any PLL.

    CLOCKS.set_xclk_sel(Mux2::Sel1);    // Xtal
    CLOCKS.set_mm_xclk_sel(Mux2::Sel1); // Xtal

    // SETUP CORE TIMER

    CORET.init(&CLOCKS);

    // PERIPHERAL INIT

    CLOCKS.set_uart_enable(false);
    CLOCKS.set_d0_cpu_div(1);
    CLOCKS.set_uart_sel(UartSel::Xclock);
    CLOCKS.set_uart_enable(true);
    CLOCKS.set_uart0_enable(true);
    
    // CONSOLE INIT

    let mut uart0 = Uart::new(UartPort::Port0);
    uart0.attach_tx(Pin::new(14));
    uart0.attach_rx(Pin::new(15));
    uart0.init(&UartConfig::new(115200), &CLOCKS);
    uart0.start();

    let mtimer_int = bflb_rt::get_interrupt(IrqNum::MachineTimer);
    mtimer_int.set_handler(mtimer_handler);
    mtimer_int.set_enable(true);
    mtimer_int.set_level(255);

    CORET.set_time(0);
    CORET.set_time_cmp(1_000_000);

    loop {
        
        if INTERRUPTED.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            write!(uart0, "Interrupted! RTC time: {:?}, timecmp: {}\r\n", CORET.now(), CORET.get_time_cmp()).unwrap();
        }

    }

}


static INTERRUPTED: AtomicBool = AtomicBool::new(false);
fn mtimer_handler(_code: usize) {
    INTERRUPTED.store(true, Ordering::Relaxed);
    CORET.set_time_cmp(CORET.get_time_cmp() + 1_000_000);
}
