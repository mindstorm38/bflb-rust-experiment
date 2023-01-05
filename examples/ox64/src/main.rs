#![no_std]
#![no_main]

use bflb_hal::bl808::clock::{Clocks, XtalType, Mux2, UartSel};
use bflb_hal::bl808::time::CoreTimer;
use bflb_hal::bl808::uart::{Uart, UartPort, UartConfig};
use bflb_hal::bl808::gpio::Pin;
use bflb_hal::bl808::{get_core_id, CoreId, CoreM0};

use emhal::time::Timer;

use core::fmt::Write;
use core::time::Duration;


bflb_rt::entry!(main);


fn main() {

    let core_id = CoreM0;

    let mut clocks = Clocks::new(core_id);
    let coret = CoreTimer::new(core_id);

    // CHIP.cpu().halt_d0();
    // CHIP.cpu().halt_lp();

    // CLOCK INIT

    clocks.set_xtal_type(XtalType::Mhz40);
    clocks.enable_xtal().unwrap();
    
    match get_core_id().unwrap() {
        CoreId::M0 | CoreId::LP => {
            clocks.set_xclk_sel(Mux2::Sel0);    // RC32M
            clocks.set_m0_root_sel(Mux2::Sel0); // xclock
            clocks.set_m0_cpu_div(1);
            clocks.set_m0_secondary_div(1);
            clocks.set_m0_secondary_div_act_pulse(true);
            while !clocks.get_m0_secondary_prot_done() {}
            clocks.set_lp_cpu_div(1);
            clocks.set_lp_cpu_div_act_pulse(true);
            while !clocks.get_lp_cpu_prot_done() {}
            // timer.sleep_dummy_nop();
        }
        CoreId::D0 => {
            clocks.set_mm_xclk_sel(Mux2::Sel0); // RC32M
            clocks.set_d0_root_sel(Mux2::Sel0); // MM xclock
            clocks.set_d0_cpu_div(1);
            clocks.set_d0_secondary_div(1);
            clocks.set_d0_secondary_div_act_pulse(true);
            while !clocks.get_d0_secondary_prot_done() {}
        }
    }

    // Note that we don't activate any PLL.

    clocks.set_xclk_sel(Mux2::Sel1);    // Xtal
    clocks.set_mm_xclk_sel(Mux2::Sel1); // Xtal

    // SETUP CORE TIMER

    coret.init(&mut clocks);

    // PERIPHERAL INIT

    clocks.set_uart_enable(false);
    clocks.set_d0_cpu_div(1);
    clocks.set_uart_sel(UartSel::Xclock);
    clocks.set_uart_enable(true);
    clocks.set_uart0_enable(true);
    
    // CONSOLE INIT

    let mut uart0 = Uart::new(UartPort::Port0);
    uart0.attach_tx(Pin::new(14));
    uart0.attach_rx(Pin::new(15));
    uart0.init(&UartConfig::new(115200), &clocks);
    uart0.start();

    loop {

        write!(uart0, "RTC time: {:?}\r\n", coret.time()).unwrap();

        // Simple echo.
        while let Some(byte) = uart0.read_byte() {
            uart0.write_byte(byte);
        }

        coret.sleep(Duration::from_secs(1));

    }

}
