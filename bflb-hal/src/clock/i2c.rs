//! Clock functions for I2C peripheral.
//! 
//! This module contains both MCU and MM subsystems I²C controls.

use crate::arch::bl808::{GLB, MM_GLB};

use super::mm::{get_mm_bclk1_freq, get_mm_xclk_freq};
use super::mcu::get_mcu_pbclk_freq;
use super::get_xclk_freq;


pub unsafe fn set_mcu_i2c_enable(enable: bool) {
    GLB.i2c_cfg0().modify(|reg| reg.i2c_clk_en().set(enable as _));
}

#[doc(alias = "Clock_Get_I2C_Clk_Sel_Val")]
pub fn get_mcu_i2c_sel() -> McuI2cSel {
    match GLB.i2c_cfg0().get().i2c_clk_sel().get() {
        0 => McuI2cSel::McuPbclk,
        1 => McuI2cSel::Xclk,
        _ => unreachable!()
    }
}

pub unsafe fn set_mcu_i2c_sel(sel: McuI2cSel) {
    GLB.i2c_cfg0().modify(|reg| reg.i2c_clk_sel().set(sel as _));
}

pub fn get_mcu_i2c_div() -> u32 {
    GLB.i2c_cfg0().get().i2c_clk_div().get() + 1
}

pub unsafe fn set_mcu_i2c_div(div: u32) {
    GLB.i2c_cfg0().modify(|reg| reg.i2c_clk_div().set(div - 1));
}

pub fn get_mcu_i2c_freq() -> u32 {
    let freq = match get_mcu_i2c_sel() {
        McuI2cSel::McuPbclk => get_mcu_pbclk_freq(),
        McuI2cSel::Xclk => get_xclk_freq(),
    };
    freq / get_mcu_i2c_div()
}

/// Setup the global clock for MCU I2C controllers (0, 1).
#[inline(never)]
pub unsafe fn setup_mcu_i2c(clock_sel: McuI2cSel, div: u32, enable: bool) {
    set_mcu_i2c_enable(false);
    set_mcu_i2c_sel(clock_sel);
    set_mcu_i2c_div(div);
    set_mcu_i2c_enable(enable);
}


#[doc(alias = "Clock_Get_DSP_I2C_Clk_Sel_Val")]
pub fn get_mm_i2c_sel() -> MmI2cSel {
    match MM_GLB.mm_clk_ctrl_cpu().get().i2c_clk_sel().get() {
        0 => MmI2cSel::MmBclk1,
        1 => MmI2cSel::MmXclk,
        _ => unreachable!()
    }
}

pub unsafe fn set_mm_i2c_sel(sel: MmI2cSel) {
    MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.i2c_clk_sel().set(sel as _));
}

#[doc(alias = "Clock_Get_DSP_I2C0_Div_Val")]
pub fn get_mm_i2c0_div() -> u32 {
    MM_GLB.mm_clk_ctrl_peri().get().i2c0_clk_div().get() + 1
}

pub unsafe fn set_mm_i2c0_div(div: u32) {
    MM_GLB.mm_clk_ctrl_peri().modify(|reg| reg.i2c0_clk_div().set(div - 1));
}

#[doc(alias = "Clock_Get_DSP_I2C1_Div_Val")]
pub fn get_mm_i2c1_div() -> u32 {
    MM_GLB.mm_clk_ctrl_peri3().get().i2c1_clk_div().get() + 1
}

pub unsafe fn set_mm_i2c1_div(div: u32) {
    MM_GLB.mm_clk_ctrl_peri3().modify(|reg| reg.i2c1_clk_div().set(div - 1));
}

pub fn get_mm_i2c_base_freq() -> u32 {
    match get_mm_i2c_sel() {
        MmI2cSel::MmBclk1 => get_mm_bclk1_freq(),
        MmI2cSel::MmXclk => get_mm_xclk_freq(),
    }
}

pub fn get_mm_i2c0_freq() -> u32 {
    get_mm_i2c_base_freq() / get_mm_i2c0_div()
}

pub fn get_mm_i2c1_freq() -> u32 {
    get_mm_i2c_base_freq() / get_mm_i2c1_div()
}


/// Selector for MCU I2C clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McuI2cSel {
    McuPbclk = 0,
    Xclk = 1,
}

/// Selector for MM I2C clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmI2cSel {
    MmBclk1 = 0,
    MmXclk = 1,
}
