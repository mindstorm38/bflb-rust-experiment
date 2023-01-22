#![no_std]
#![no_main]

use bflb_hal::clock::{Clocks, XtalType, Mux2, UartClockSel};
use bflb_hal::uart::{UartAccess, UartConfig};
use bflb_hal::dma::{DmaAccess, DmaConfig, DmaDirection, DmaPeripheral, DmaEndpointConfig, DmaBurstSize, DmaDataWidth};
use bflb_hal::time::CoreTimer;
use bflb_hal::gpio::PinAccess;
use bflb_hal::irq::IrqNum;

use bflb_hal::bl808::{UART0, GLB};

use embedded_util::Peripheral;

use core::sync::atomic::{AtomicBool, Ordering};
use core::fmt::Write;


#[link_section = ".data"] // Loaded in RAM
static DMA_MESSAGE: &'static str = "Hello world!";


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

    clocks.setup_mcu_uart(UartClockSel::Xclock, 1, true);
    clocks.set_mcu_uart0_enable(true);
    
    // CONSOLE INIT

    let uart_tx = PinAccess::<14>::take();
    let uart_rx = PinAccess::<15>::take();
    let mut uart = UartAccess::<0>::take()
        .into_duplex(uart_tx, uart_rx, &UartConfig::new(115200), &clocks);

    // let uart_dma = DmaAccess::<0, 0>::take()
    //     .into_channel(&DmaConfig {
    //         direction: DmaDirection::MemoryToPeripheral(DmaPeripheral::Uart0Tx),
    //         src: DmaEndpointConfig {
    //             addr: DMA_MESSAGE.as_ptr() as usize as _,
    //             incr: true,
    //             burst_size: DmaBurstSize::Incr1,
    //             data_width: DmaDataWidth::Word,
    //         },
    //         dst: DmaEndpointConfig {
    //             addr: UART0.fifo_rdata(),
    //             incr: todo!(),
    //             burst_size: todo!(),
    //             data_width: todo!(),
    //         },
    //         size: todo!(),
    //     });
    
    // loop {}

    let mut timer = CoreTimer::borrow();
    timer.init(&mut clocks);
    timer.set_time(0);
    timer.set_time_cmp(1_000_000);
    drop(timer);

    let mtimer_int = bflb_rt::get_interrupt(IrqNum::MachineTimer);
    mtimer_int.set_handler(mtimer_handler);
    mtimer_int.set_enable(true);
    mtimer_int.set_level(255);

    let _ = writeln!(uart, "- GLB.uart_cfg1: {:08X}", GLB.uart_cfg1().get().0);
    let _ = writeln!(uart, "- GLB.uart_cfg2: {:08X}", GLB.uart_cfg2().get().0);

    loop {

        // while let Some(b) = uart.read_byte() {
        //     let _ = writeln!(uart, "Received: {}", b as char);
        // }
        
        if INTERRUPTED.compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            mtimer_int.set_enable(false);
            {
                let _ = writeln!(uart, "Interrupted! RTC time: {}", CoreTimer::borrow().get_time());
                let _ = writeln!(uart, "UART debug:");
                let _ = writeln!(uart, "- rx_fifo_count: {:?}", UART0.fifo_cfg1().get());
                let _ = writeln!(uart, "- status:        {:?}", UART0.status().get());
                let _ = writeln!(uart, "- rdata:         {:02X}", UART0.fifo_rdata().get());
                let _ = writeln!(uart, "- GLB.uart_cfg1: {:08X}", GLB.uart_cfg1().get().0);
                let _ = writeln!(uart, "- GLB.uart_cfg2: {:08X}", GLB.uart_cfg2().get().0);
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
