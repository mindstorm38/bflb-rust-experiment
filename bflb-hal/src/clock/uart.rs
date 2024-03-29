//! Clock functions for UART peripheral.

use crate::arch::bl808::{GLB, HBN};

use super::mcu::get_mcu_pbclk_freq;
use super::get_xclk_freq;


/// Enable clock gate for MCU UART0 controller.
pub unsafe fn set_mcu_uart0_enable(enable: bool) {
    GLB.cgen_cfg1().modify(|reg| reg.cgen_s1a_uart0().set(enable as _));
}

/// Enable clock gate for MCU UART1 controller.
pub unsafe fn set_mcu_uart1_enable(enable: bool) {
    GLB.cgen_cfg1().modify(|reg| reg.cgen_s1a_uart1().set(enable as _));
}

/// Enable clock gate for MCU UART2 controller.
pub unsafe fn set_mcu_uart2_enable(enable: bool) {
    GLB.cgen_cfg1().modify(|reg| reg.cgen_s1a_uart2().set(enable as _));
}

/// Enable global clock gate for MCU UART controllers (0, 1, 2).
pub unsafe fn set_mcu_uart_enable(enable: bool) {
    GLB.uart_cfg0().modify(|reg| reg.uart_clk_en().set(enable as _));
}

/// Get global clock selector for MCU UART controllers (0, 1, 2).
pub fn get_mcu_uart_sel() -> UartSel {
    let mut reg = HBN.glb().get();
    let sel_raw = (reg.uart_clk_sel2().get() << 1) | reg.uart_clk_sel().get();
    match sel_raw {
        0 => UartSel::McuPbclk,
        1 => UartSel::Pll160,
        2 => UartSel::Xclk,
        _ => unreachable!("invalid uart clock selector")
    }
}

/// Set global clock selector for MCU UART controllers (0, 1, 2).
pub unsafe fn set_mcu_uart_sel(clock_sel: UartSel) {
    HBN.glb().modify(|reg| {
        let val = clock_sel as u32;
        reg.uart_clk_sel2().set((val >> 1) & 1);
        reg.uart_clk_sel().set(val & 1);
    });
}

/// Get global clock divider for MCU UART controllers (0, 1, 2).
pub fn get_mcu_uart_div() -> u32 {
    GLB.uart_cfg0().get().uart_clk_div().get() + 1
}

/// Set global clock divider for MCU UART controllers (0, 1, 2).
pub unsafe fn set_mcu_uart_div(div: u32) {
    GLB.uart_cfg0().modify(|reg| reg.uart_clk_div().set(div - 1));
}

/// Set global clock frequency for MCU UART controllers (0, 1, 2).
pub fn get_mcu_uart_freq() -> u32 {
    let freq = match get_mcu_uart_sel() {
        UartSel::McuPbclk => get_mcu_pbclk_freq(),
        UartSel::Pll160 => todo!("pll160 is not implemented"),
        UartSel::Xclk => get_xclk_freq(),
    };
    freq / get_mcu_uart_div()
}

/// Setup the global clock for MCU UART controllers (0, 1, 2).
#[inline(never)]
pub unsafe fn setup_mcu_uart(clock_sel: UartSel, div: u32, enable: bool) {
    set_mcu_uart_enable(false);
    set_mcu_uart_sel(clock_sel);
    set_mcu_uart_div(div);
    set_mcu_uart_enable(enable);
}


/// Selector for UART clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartSel {
    McuPbclk = 0,
    Pll160 = 1,
    Xclk = 2,
}
