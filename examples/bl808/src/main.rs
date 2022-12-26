#![no_std]
#![no_main]

use bflb_hal::bl808::{Bl808, CpuId};
use bflb_hal::bl808::clock::{Clocks, XtalType, Mux2, UartRefClock};
use bflb_hal::bl808::gpio::{Gpio, GpioConfig};


emrt::entry! {
    main();
}

fn main() {

    let gpio = Gpio::new();
    let clocks = Clocks::new();

    // CHIP.cpu().halt_d0();
    // CHIP.cpu().halt_lp();

    // CLOCK INIT

    clocks.set_xtal_type(XtalType::Mhz40);
    clocks.enable_xtal().unwrap();
    
    match Bl808::get_cpu_id().unwrap() {
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

    clocks.enable_uart0_clock();
    clocks.set_uart_clock(true, UartRefClock::Xclock, 1);

    // CONSOLE INIT

    gpio.init(18, &GpioConfig::with_toggle_output());
    gpio.set_toggle(18);

    loop {
        
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
