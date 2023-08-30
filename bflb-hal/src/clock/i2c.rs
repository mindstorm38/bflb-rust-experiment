//! Clock functions for I2C peripheral.


use crate::bl808::{GLB, MM_GLB};
use super::Clocks;

impl Clocks {

    pub fn set_mcu_i2c_enable(&mut self, enable: bool) {
        GLB.i2c_cfg0().modify(|reg| reg.i2c_clk_en().set(enable as _));
    }

    #[doc(alias = "Clock_Get_I2C_Clk_Sel_Val")]
    pub fn get_mcu_i2c_sel(&self) -> McuI2cSel {
        match GLB.i2c_cfg0().get().i2c_clk_sel().get() {
            0 => McuI2cSel::McuPbclk,
            1 => McuI2cSel::Xclk,
            _ => unreachable!()
        }
    }

    pub fn set_mcu_i2c_sel(&mut self, sel: McuI2cSel) {
        GLB.i2c_cfg0().modify(|reg| reg.i2c_clk_sel().set(sel as _));
    }

    pub fn get_mcu_i2c_div(&self) -> u32 {
        GLB.i2c_cfg0().get().i2c_clk_div().get() + 1
    }

    pub fn set_mcu_i2c_div(&mut self, div: u32) {
        GLB.i2c_cfg0().modify(|reg| reg.i2c_clk_div().set(div - 1));
    }

    pub fn get_mcu_i2c_freq(&self) -> u32 {
        let freq = match self.get_mcu_i2c_sel() {
            McuI2cSel::McuPbclk => self.get_mcu_pbclk_freq(),
            McuI2cSel::Xclk => self.get_xclk_freq(),
        };
        freq / self.get_mcu_i2c_div()
    }

    /// Setup the global clock for MCU I2C controllers (0, 1).
    #[inline(never)]
    pub fn setup_mcu_i2c(&mut self, clock_sel: McuI2cSel, div: u32, enable: bool) {
        self.set_mcu_i2c_enable(false);
        self.set_mcu_i2c_sel(clock_sel);
        self.set_mcu_i2c_div(div);
        self.set_mcu_i2c_enable(enable);
    }

}

impl Clocks {

    #[doc(alias = "Clock_Get_DSP_I2C_Clk_Sel_Val")]
    pub fn get_mm_i2c_sel(&self) -> MmI2cSel {
        match MM_GLB.mm_clk_ctrl_cpu().get().i2c_clk_sel().get() {
            0 => MmI2cSel::MmBclk1,
            1 => MmI2cSel::MmXclk,
            _ => unreachable!()
        }
    }

    pub fn set_mm_i2c_sel(&mut self, sel: MmI2cSel) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.i2c_clk_sel().set(sel as _));
    }

    #[doc(alias = "Clock_Get_DSP_I2C0_Div_Val")]
    pub fn get_mm_i2c0_div(&self) -> u32 {
        MM_GLB.mm_clk_ctrl_peri().get().i2c0_clk_div().get() + 1
    }

    pub fn set_mm_i2c0_div(&mut self, div: u32) {
        MM_GLB.mm_clk_ctrl_peri().modify(|reg| reg.i2c0_clk_div().set(div - 1));
    }

    #[doc(alias = "Clock_Get_DSP_I2C1_Div_Val")]
    pub fn get_mm_i2c1_div(&self) -> u32 {
        MM_GLB.mm_clk_ctrl_peri3().get().i2c1_clk_div().get() + 1
    }

    pub fn set_mm_i2c1_div(&mut self, div: u32) {
        MM_GLB.mm_clk_ctrl_peri3().modify(|reg| reg.i2c1_clk_div().set(div - 1));
    }

    fn get_mm_i2c_base_freq(&self) -> u32 {
        match self.get_mm_i2c_sel() {
            MmI2cSel::MmBclk1 => self.get_mm_bclk1_freq(),
            MmI2cSel::MmXclk => self.get_mm_xclk_freq(),
        }
    }

    pub fn get_mm_i2c0_freq(&self) -> u32 {
        self.get_mm_i2c_base_freq() / self.get_mm_i2c0_div()
    }

    pub fn get_mm_i2c1_freq(&self) -> u32 {
        self.get_mm_i2c_base_freq() / self.get_mm_i2c1_div()
    }

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
