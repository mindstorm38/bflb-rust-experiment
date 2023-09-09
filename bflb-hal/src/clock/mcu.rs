//! MCU subsystems related clocks.

use crate::arch::bl808::{PDS, HBN, GLB};

use super::pll::{PllAudioDiv, get_cpu_pll_freq, get_audio_pll_freq, get_wifi_pll_freq};
use super::get_xclk_freq;


/// Get the selector for the PLL MCU freq.
pub fn get_mcu_pll_sel() -> McuPllSel {
    match PDS.cpu_core_cfg1().get().pll_sel().get() {
        0 => McuPllSel::CpuPll,
        1 => McuPllSel::AudioPll,
        2 => McuPllSel::WifiPll240,
        3 => McuPllSel::WifiPll320,
        _ => unreachable!()
    }
}

pub unsafe fn set_mcu_pll_sel(sel: McuPllSel) {
    PDS.cpu_core_cfg1().modify(|reg| reg.pll_sel().set(sel as _));
}

/// Get the frequency output from MCU multiplexer.
pub fn get_mcu_pll_freq() -> u32 {
    match get_mcu_pll_sel() {
        McuPllSel::CpuPll => get_cpu_pll_freq(400_000_000),
        McuPllSel::AudioPll => get_audio_pll_freq(PllAudioDiv::Div1),
        McuPllSel::WifiPll240 => get_wifi_pll_freq(240_000_000),
        McuPllSel::WifiPll320 => get_wifi_pll_freq(320_000_000),
    }
}


/// Get the selector for the main MCU freq.
pub fn get_mcu_root_sel() -> McuRootSel {
    match HBN.glb().get().mcu_root_sel().get() {
        0 => McuRootSel::Xclk,
        1 => McuRootSel::McuPll,
        _ => unreachable!()
    }
}

/// Set the selector for the main MCU freq.
pub unsafe fn set_mcu_root_sel(sel: McuRootSel) {
    HBN.glb().modify(|reg| reg.mcu_root_sel().set(sel as _));
}

/// Get the frequency for M0 root clock.
pub fn get_mcu_root_freq() -> u32 {
    match get_mcu_root_sel() {
        McuRootSel::Xclk => get_xclk_freq(),
        McuRootSel::McuPll => get_mcu_pll_freq(),
    }
}


/// Get the divider for M0 CPU clock.
pub fn get_m0_cpu_div() -> u32 {
    GLB.sys_cfg0().get().hclk_div().get() + 1
}

/// Set the divider for M0 CPU clock.
pub unsafe fn set_m0_cpu_div(div: u32) {
    GLB.sys_cfg0().modify(|reg| reg.hclk_div().set(div - 1));
}

/// Get the frequency for M0 CPU clock.
pub fn get_m0_cpu_freq() -> u32 {
    let root_freq = get_mcu_root_freq();
    root_freq / get_m0_cpu_div()
}

/// Enable or not the M0 core clock gate.
pub unsafe fn set_m0_enable(enable: bool) {
    PDS.cpu_core_cfg1().modify(|reg| {
        reg.mcu1_clk_en().set(enable as _);
    });
}


/// Get the divider for M0 secondary clock.
pub fn get_mcu_pbclk_div() -> u32 {
    GLB.sys_cfg0().get().bclk_div().get() + 1
}

/// Get the divider for M0 secondary clock.
pub unsafe fn set_mcu_pbclk_div(div: u32) {
    GLB.sys_cfg0().modify(|reg| reg.bclk_div().set(div - 1));
}

pub unsafe fn set_mcu_pbclk_div_act_pulse(act: bool) {
    GLB.sys_cfg1().modify(|reg| reg.bclk_div_act_pulse().set(act as _));
}

pub fn get_mcu_pbclk_prot_done() -> bool {
    GLB.sys_cfg1().get().sts_bclk_prot_done().get() != 0
}

#[doc(alias = "Clock_System_Clock_Get(BL_SYSTEM_CLOCK_MCU_PBCLK)")]
pub fn get_mcu_pbclk_freq() -> u32 {
    get_m0_cpu_freq() / get_mcu_pbclk_div()
}


/// Get the divider for LP CPU clock.
pub fn get_lp_cpu_div() -> u32 {
    PDS.cpu_core_cfg7().get().pico_div().get()
}

/// Set the divider for LP CPU clock.
pub unsafe fn set_lp_cpu_div(div: u32) {
    PDS.cpu_core_cfg7().modify(|reg| reg.pico_div().set(div));
}

pub unsafe fn set_lp_cpu_div_act_pulse(act: bool) {
    GLB.sys_cfg1().modify(|reg| reg.pico_clk_div_act_pulse().set(act as _));
}

pub fn get_lp_cpu_prot_done() -> bool {
    GLB.sys_cfg1().get().sts_pico_clk_prot_done().get() != 0
}

/// Get the frequency for LP CPU clock.
pub fn get_lp_cpu_freq() -> u32 {
    get_mcu_pbclk_freq() / get_lp_cpu_div()
}

/// Enable or not the LP core clock gate.
pub unsafe fn set_lp_enable(enable: bool) {
    PDS.cpu_core_cfg0().modify(|reg| {
        reg.pico_clk_en().set(enable as _);
    });
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McuPllSel {
    CpuPll = 0,
    AudioPll = 1,
    WifiPll240 = 2,
    WifiPll320 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McuRootSel {
    Xclk = 0,
    McuPll = 1,
}
