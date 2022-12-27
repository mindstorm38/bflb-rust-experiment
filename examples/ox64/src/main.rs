#![no_std]
#![no_main]

use bflb_hal::bl808::clock::{Clocks, XtalType, Mux2, UartSel};
use bflb_hal::bl808::uart::{Uart, UartPort, UartConfig};
use bflb_hal::bl808::gpio::{Pin, PinMode};
use bflb_hal::bl808::{get_cpu_id, CpuId};
use core::fmt::Write;


emrt::entry! {
    main();
}

fn main() {

    let clocks = Clocks::new();

    // CHIP.cpu().halt_d0();
    // CHIP.cpu().halt_lp();

    // CLOCK INIT

    clocks.set_xtal_type(XtalType::Mhz40);
    clocks.enable_xtal().unwrap();
    
    match get_cpu_id().unwrap() {
        CpuId::M0 | CpuId::LP => {
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
        CpuId::D0 => {
            clocks.set_mm_xclk_sel(Mux2::Sel0); // RC32M
            clocks.set_d0_root_sel(Mux2::Sel0); // MM xclock
            clocks.set_d0_cpu_div(1);
            clocks.set_d0_secondary_div(1);
            clocks.set_d0_secondary_div_act_pulse(true);
            while !clocks.get_d0_secondary_prot_done() {}
        }
    }

    // Note that we don't activate any PLL.

    // timer.sleep_arch(Duration::from_micros(75)).unwrap();

    clocks.set_xclk_sel(Mux2::Sel1);    // Xtal
    clocks.set_mm_xclk_sel(Mux2::Sel1); // Xtal

    // timer.sleep_dummy_nop();

    clocks.enable_mtimer_clock((clocks.get_mtimer_source_freq().unwrap() / 1_000_000) as u16).unwrap();

    // PERIPHERAL INIT

    clocks.set_uart_enable(false);
    clocks.set_d0_cpu_div(1);
    clocks.set_uart_sel(UartSel::Xclock);
    clocks.set_uart_enable(true);
    clocks.set_uart0_enable(true);
    
    // CONSOLE INIT

    let mut uart0 = Uart::new(UartPort::Port0);
    let mut pin14 = Pin::new(14);
    let mut pin15 = Pin::new(15);
    uart0.attach_tx(&mut pin14);
    uart0.attach_rx(&mut pin15);
    uart0.init(&clocks, &UartConfig::new(115200));

    let mut pin18 = Pin::with_mode(18, PinMode::Output);
    let mut state = false;
    loop {
        
        if state {
            pin18.set_high();
            state = false;
        } else {
            pin18.set_low();
            state = true;
        }

        write!(uart0, "hello world from m0 in rust\r\n").unwrap();

        for _ in 0..1_000_000 {
            unsafe { core::arch::asm!("nop"); }
        }

    }
    
    // let mut state = false;

    // loop {

    //     gpio.set_normal(21, state);
    //     state = !state;

    //     timer.sleep_arch(Duration::from_millis(10));

    //     for _ in 0..1_000_000_000 {
    //         timer.sleep_dummy_nop();
    //     }

    // }

}
