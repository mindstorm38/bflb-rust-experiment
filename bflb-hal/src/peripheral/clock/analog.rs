//! Clock functions for Analogic/Digital peripherals.

use crate::bl808::GLB;
use super::Clocks;


impl Clocks {

    /// Enable or disable common clock gate for ADC/DAC.
    pub fn set_adc_dac_enable(&mut self, enable: bool) {
        GLB.cgen_cfg1().modify(|reg| reg.cgen_s1_gpip().set(enable as _));
    }

    pub fn set_adc_div_enable(&mut self, enable: bool) {
        GLB.adc_cfg0().modify(|reg| reg.gpadc_32m_div_en().set(enable as _));
    }

    pub fn get_adc_sel(&self) -> AdcClockSel {
        match GLB.adc_cfg0().get().gpadc_32m_clk_sel().get() {
            0 => AdcClockSel::AudioPll,
            1 => AdcClockSel::Xclk,
            _ => unreachable!()
        }
    }

    pub fn set_adc_sel(&mut self, sel: AdcClockSel) {
        GLB.adc_cfg0().modify(|reg| reg.gpadc_32m_clk_sel().set(sel as _));
    }

    pub fn get_adc_div(&self) -> u32 {
        GLB.adc_cfg0().get().gpadc_32m_clk_div().get() + 1
    }

    pub fn set_adc_div(&mut self, div: u32) {
        GLB.adc_cfg0().modify(|reg| reg.gpadc_32m_clk_div().set(div - 1));
    }

    pub fn setup_adc(&mut self, sel: AdcClockSel, div: u32, enable: bool) {
        self.set_adc_div_enable(false);
        self.set_adc_div(div);
        self.set_adc_sel(sel);
        self.set_adc_div_enable(enable);
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdcClockSel {
    AudioPll = 0,
    Xclk = 1,
}
