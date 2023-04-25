#![no_std]
#![no_main]

use bflb_hal::Peripherals;

use bflb_hal::clock::{Clocks, XtalType, UartSel, McuRootSel, XclkSel, MmXclkSel};
use bflb_hal::cpu::CpuControl;
use bflb_hal::uart::{UartAccess, UartConfig};
use bflb_hal::time::CoreTimer;
use bflb_hal::gpio::PinAccess;
use bflb_hal::dma::DmaAccess;

use bflb_rt::spawn;


#[link_section = ".data"] // Loaded in RAM
static DMA_MESSAGE: &'static str = "Hello world from DMA!\n";


bflb_rt::entry!(main);

fn main() {

    let peripherals = Peripherals::take();
    let mut clocks = peripherals.clocks;
    let mut cpu_control = peripherals.cpu_control;
    let mut core_timer = peripherals.core_timer;

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
    // clocks.setup_adc(sel, div, enable);

    // CONSOLE INIT

    let uart_tx = peripherals.gpio.p14.into_alternate();
    let uart_tx = peripherals.gpio.p15.into_alternate();

    

    let (
        uart_tx, 
        mut uart_rx
    ) = peripherals.uart.p0.into_duplex(uart_tx, uart_rx, &UartConfig::new(115200), &clocks);

    // let adc_ch0 = AdcChannel::with_ground(
    //     PinAccess::<17>::take().into_alternate());
    // let adc_ch1 = AdcChannel::with_ground(
    //     PinAccess::<5>::take().into_alternate());

    // let adc_channels = (adc_ch0, adc_ch1);

    // let mut adc = AdcAccess::take()
    //     .into_scan(&AdcConfig::default(), adc_channels);

    // adc.poll();
    // let adc_channels = adc.finish();

    // DMA

    let (_, mut uart_tx, _) = peripherals.dma.p0.c0
        .into_transfer(DMA_MESSAGE, uart_tx)
        .wait_destruct();
    
    // ADC TEST
    
    // let _ = writeln!(uart_tx, "ADC values: {}, {}", adc_channels.0.raw_value(), adc_channels.1.raw_value());

    // INTS
    
    core_timer.init(&mut clocks);
    core_timer.set_time(0);
    core_timer.set_time_cmp(1_000_000);
    // timer.free();

    // let mut mtimer_int = Interrupt::<MACHINE_TIMER>::take();
    // mtimer_int.set_handler(mtimer_handler);
    // mtimer_int.set_enabled(true);
    // mtimer_int.set_level(255);

    // GPIO

    let mut d0 = peripherals.gpio.p33.into_output();
    let mut d1 = peripherals.gpio.p32.into_output();
    let mut d2 = peripherals.gpio.p21.into_output();
    let mut d3 = peripherals.gpio.p20.into_output();
    let mut sel0 = peripherals.gpio.p23.into_output();
    let mut sel1 = peripherals.gpio.p22.into_output();

    d0.set_low();
    d1.set_low();
    d2.set_low();
    d3.set_low();

    sel0.set_high();
    sel1.set_high();

    let mut write_digit = |digit: u32| {
        if digit & 0b0001 != 0 { d0.set_open() } else { d0.set_low() }
        if digit & 0b0010 != 0 { d1.set_open() } else { d1.set_low() }
        if digit & 0b0100 != 0 { d2.set_open() } else { d2.set_low() }
        if digit & 0b1000 != 0 { d3.set_open() } else { d3.set_low() }
    };

    let mut write_num = |num: u32| {

        const DELAY: u64 = 6_000;

        write_digit(num % 10);
        sel0.set_low();
        timer.wait_time(DELAY);
        sel0.set_high();
        timer.wait_time(DELAY);

        write_digit((num / 10) % 10);
        sel1.set_low();
        timer.wait_time(DELAY);
        sel1.set_high();
        timer.wait_time(DELAY);

    };

    // LOOP

    // let mut counter = 0;

    loop {

        while let Some(b) = uart_rx.read_byte() {
            uart_tx.write_byte(b);
        }

        write_num((timer.get_time() / 100_000) as u32);
        
        // if INTERRUPTED.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
        //     mtimer_int.without(|| {

        //         counter += 1;

        //         let timer = CoreTimer::take();
        //         let _ = writeln!(uart_tx, "Interrupted! RTC time: {}", timer.get_time());
        //         timer.free();

        //     });
        // }

    }

}


// static INTERRUPTED: AtomicBool = AtomicBool::new(false);

// fn mtimer_handler(_code: usize) {
//     let mut timer = CoreTimer::take();
//     let time_cmp = timer.get_time_cmp();
//     timer.set_time_cmp(time_cmp + 100_000);
//     INTERRUPTED.store(true, Ordering::Relaxed);
//     timer.free();
// }
