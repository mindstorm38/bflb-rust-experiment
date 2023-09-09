//! Clock functions for Analogic/Digital peripherals.

use crate::arch::bl808::GLB;


/// Enable or disable common clock gate for ADC/DAC.
pub unsafe fn set_adc_dac_enable(enable: bool) {
    GLB.cgen_cfg1().modify(|reg| reg.cgen_s1_gpip().set(enable as _));
}

pub unsafe fn set_adc_div_enable(enable: bool) {
    GLB.adc_cfg0().modify(|reg| reg.gpadc_32m_div_en().set(enable as _));
}

pub fn get_adc_sel() -> AdcClockSel {
    match GLB.adc_cfg0().get().gpadc_32m_clk_sel().get() {
        0 => AdcClockSel::AudioPll,
        1 => AdcClockSel::Xclk,
        _ => unreachable!()
    }
}

pub unsafe fn set_adc_sel(sel: AdcClockSel) {
    GLB.adc_cfg0().modify(|reg| reg.gpadc_32m_clk_sel().set(sel as _));
}

pub fn get_adc_div() -> u32 {
    GLB.adc_cfg0().get().gpadc_32m_clk_div().get() + 1
}

pub unsafe fn set_adc_div(div: u32) {
    GLB.adc_cfg0().modify(|reg| reg.gpadc_32m_clk_div().set(div - 1));
}

#[inline(never)]
pub unsafe fn setup_adc(sel: AdcClockSel, div: u32, enable: bool) {
    set_adc_div_enable(false);
    set_adc_div(div);
    set_adc_sel(sel);
    set_adc_div_enable(enable);
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdcClockSel {
    AudioPll = 0,
    Xclk = 1,
}
