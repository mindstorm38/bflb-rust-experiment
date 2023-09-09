//! Clock management for BL808.
//! 
//! Three types of clock sources:
//! - Crystal (24, 32, 38.4, 40 MHz)
//! - Crystal 32 kHz
//! - RC 32 kHz
//! - RC 32 MHz
//! - PLL (various...)
//! 
//! Glossary:
//! - CLK: Clock
//! - XTAL: Crystal
//! - BCLK: Bus Block
//! - PBCLK: Peripheral Bus Clock
//! - HCLK: AHB Clock 
//! - PLL: Phase-Locked Loop
//! 
//! This module provides clock demultiplexers, clock gates and dividers. 
//! Naming convention for function follows these rules:
//! - Demultiplexer: `set_<name>_sel(&mut self, sel: <enum>)`, 
//!   `get_<name>_sel(&self) -> <enum>`.
//! - Gate: `set_<name>_enable(&mut self, enable: bool)`,
//!   `is_<name>_enable(&self) -> bool`.
//! - Divider: `set_<name>_div(&mut self, div: u32)`,
//!   `get_<name>_div(&self) -> u32`.
//! 
//! All "set" functions in these modules are unsafe, therefore you must ensure that all
//! your modifications on clocks will not create undefined behaviors.
//! 
//! Sources:
//! - https://github.com/bouffalolab/bl_mcu_sdk/blob/master/drivers/soc/bl808/std/src/bl808_clock.c
//! - Clock diagram in the Datasheet

#![allow(unsafe_op_in_unsafe_fn)]

use embedded_util::PtrRw;

use crate::arch::bl808::{self, CpuRtc, HBN, AON, GLB};

pub mod analog;
pub mod mcu;
pub mod mm;
pub mod uart;
pub mod i2c;
pub mod pwm;
pub mod dma;
pub mod pll;


/// Get the machine timer RTC register for the current core.
fn get_mtimer_rtc_reg() -> PtrRw<CpuRtc> {
    #[cfg(feature = "bl808-m0")]
    { bl808::MCU_MISC.cpu_mtimer_rtc() }
    #[cfg(feature = "bl808-d0")]
    { bl808::MM_MISC.cpu_mtimer_rtc() }
    #[cfg(feature = "bl808-lp")]
    { bl808::PDS.cpu_mtimer_rtc() }
}

/// Enable and configure the machine timer clock.
pub unsafe fn enable_mtimer_clock(div: u32) {
    debug_assert_ne!(div, 0, "divider must be nonzero");
    let rtc = get_mtimer_rtc_reg();
    rtc.modify(|reg| reg.enable().set(0));
    rtc.modify(|reg| reg.divider().set(div - 1));
    rtc.modify(|reg| reg.enable().set(1));
}

/// Disable the machine timer clock.
pub unsafe fn disable_mtimer_clock() {
    let rtc = get_mtimer_rtc_reg();
    rtc.modify(|reg| reg.enable().set(0));
}

/// Get machine timer divider.
pub fn get_mtimer_div() -> u32 {
    get_mtimer_rtc_reg().get().divider().get() + 1
}

/// Get the source frequency of the machine timer clock, without RTC divider.
/// To get the real frequency of the machine timer, use [`get_mtimer_freq`].
pub fn get_mtimer_source_freq() -> u32 {
    #[cfg(feature = "bl808-m0")]
    { mcu::get_m0_cpu_freq() }
    #[cfg(feature = "bl808-d0")]
    { todo!() }
    #[cfg(feature = "bl808-lp")]
    { todo!() }
}

/// Get the real frequency of the machine timer clock.
pub fn get_mtimer_freq() -> u32 {
    get_mtimer_source_freq() / get_mtimer_div()
}


/// Get the soc crystal type.
/// 
/// *Note that this is only informational, and not used by hardware,
/// because the crystal clock is physically selected outside chip.*
/// 
/// Use [`get_xtal_freq`] to get the real frequency.
pub fn get_xtal_type() -> XtalType {
    let mut rsv3 = HBN.rsv3().get();
    if rsv3.xtal_flag().get() == 0x58 {
        rsv3.xtal_type().get().into()
    } else {
        XtalType::None
    }
}

/// Set the soc crystal type.
/// 
/// *Note that this is only informational, and not used by hardware,
/// because the crystal clock is physically selected outside chip.*
pub unsafe fn set_xtal_type(typ: XtalType) {
    HBN.rsv3().modify(|reg| {
        reg.xtal_flag().set(0x58);
        reg.xtal_type().set(typ as _);
    });
}

/// Get the socket's crystal frequency in Hz.
pub fn get_xtal_freq() -> u32 {
    match get_xtal_type() {
        XtalType::None      => 0,
        XtalType::Mhz24     => 24_000_000,
        XtalType::Mhz32     => 32_000_000,
        XtalType::Mhz38p4   => 38_400_000,
        XtalType::Mhz40     => 40_000_000,
        XtalType::Mhz26     => 26_000_000,
    }
}

/// Power on crystal clock and wait for it being enabled.
pub unsafe fn enable_xtal() -> Result<(), ()> {
    
    AON.rf_top_aon().modify(|reg| {
        reg.pu_xtal_aon().set(1);
        reg.pu_xtal_buf_aon().set(1);
    });

    let mut timeout = 0;

    loop {

        // self.chip.timer().sleep_arch(Duration::from_micros(10))?;

        if AON.tsen().get().xtal_rdy().get() != 0 {
            break;
        }

        timeout += 1;
        if timeout >= 120 {
            return Err(())
        }

    }

    Ok(())

}


/// Get the selector for the main xclock freq.
pub fn get_xclk_sel() -> XclkSel {
    match HBN.glb().get().xclk_sel().get() {
        0 => XclkSel::Rc32m,
        1 => XclkSel::Xtal,
        _ => unreachable!()
    }
}

/// Set the selector for the main xclock freq.
pub unsafe fn set_xclk_sel(sel: XclkSel) {
    HBN.glb().modify(|reg| reg.xclk_sel().set(sel as _));
}

/// Get the main xclock frequency.
pub fn get_xclk_freq() -> u32 {
    match get_xclk_sel() {
        XclkSel::Rc32m => 32_000_000,
        XclkSel::Xtal => get_xtal_freq(),
    }
}


/// Get the selector the F32k clock.
pub fn get_f32k_sel() -> F32kSel {
    match HBN.glb().get().f32k_sel().get() {
        0 => F32kSel::Rc32k,
        1 => F32kSel::Xtal32k,
        2 | 3 => F32kSel::Dig32k,
        _ => unreachable!()
    }
}

/// Set the selector the F32k clock.
/// - 0 - RC 32 kHz
/// - 1 - Crystal 32 kHz
/// - 2/3 - Crystal divided
pub unsafe fn set_f32k_sel(sel: F32kSel) {
    HBN.glb().modify(|reg| reg.f32k_sel().set(sel as _));
}

/// Get the frequency for F32K clock, expected to be 32kHz
/// but can vary if sourced from crystal clock.
pub fn get_f32k_freq() -> u32 {
    match get_f32k_sel() {
        F32kSel::Rc32k => 32_000,
        F32kSel::Xtal32k => 32_000,
        F32kSel::Dig32k => {
            let div = GLB.dig_clk_cfg0().get().dig_32k_div().get() + 1;
            get_xtal_freq() / div
        }
    }
}


/// All external crystal clock frequencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum XtalType {
    None = 0,
    Mhz24 = 1,
    Mhz32 = 2,
    Mhz38p4 = 3,
    Mhz40 = 4,
    Mhz26 = 5,
}

impl From<u32> for XtalType {
    fn from(n: u32) -> Self {
        match n {
            1 => Self::Mhz24,
            2 => Self::Mhz32,
            3 => Self::Mhz38p4,
            4 => Self::Mhz40,
            5 => Self::Mhz26,
            // 6 => Self::Mhz32Rc, // TODO: Check if relevant (because it's only used to select RC32M)
            _ => Self::None
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XclkSel {
    Rc32m = 0,
    Xtal = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F32kSel {
    Rc32k = 0,
    Xtal32k = 1,
    Dig32k = 2,
}
