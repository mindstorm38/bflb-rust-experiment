//! Clock management for BL808.
//! 
//! Three types of clock sources:
//! - Crystal (24, 32, 38.4, 40 MHz)
//! - Crystal 32 kHz
//! - RC 32 kHz
//! - RC 32 MHz
//! - PLL (various...)
//! 
//! Sources:
//! - https://github.com/bouffalolab/bl_mcu_sdk/blob/master/drivers/soc/bl808/std/src/bl808_clock.c
//! - Clock diagram in the Datasheet

use emhal::mmio::PtrRw;

use super::mmio::CpuRtc;
use super::CpuId;


/// Interface for clock management of the chip.
#[derive(Clone, Copy)]
pub struct ChipClock<'a> {
    chip: &'a super::Chip,
}

impl super::Chip {
    /// Access the clock management to the clock management subsystem.
    #[inline(always)]
    pub fn clock(&self) -> ChipClock<'_> {
        ChipClock { chip: self }
    }
}


/// High-level mtimer methods.
impl ChipClock<'_> {

    /// Get the machine timer RTC register for the current core.
    pub fn get_mtimer_rtc_reg(&self) -> Result<PtrRw<CpuRtc>, ()> {
        Ok(match self.chip.get_cpu_id()? {
            CpuId::M0 => self.chip.mcu_misc.cpu_mtimer_rtc(),
            CpuId::D0 => self.chip.mm_misc.cpu_mtimer_rtc(),
            CpuId::LP => self.chip.pds.cpu_mtimer_rtc(),
        })
    }

    /// Enable and configure the machine timer clock.
    pub fn enable_mtimer_clock(&self, div: u16) -> Result<(), ()> {
        debug_assert_ne!(div, 0, "divider must be nonzero");
        let rtc = self.get_mtimer_rtc_reg()?;
        rtc.modify(|reg| reg.enable().set(0));
        rtc.modify(|reg| reg.divider().set(div as u32 - 1));
        rtc.modify(|reg| reg.enable().set(1));
        Ok(())
    }

    /// Disable the machine timer clock.
    pub fn disable_mtimer_clock(&self) -> Result<(), ()> {
        let rtc = self.get_mtimer_rtc_reg()?;
        rtc.modify(|reg| reg.enable().set(0));
        Ok(())
    }

    /// Get the source frequency of the machine timer clock, 
    /// without RTC divider.
    /// To get the real frequency of the machine timer, use [`get_mtimer_freq`].
    pub fn get_mtimer_source_freq(&self) -> Result<u32, ()> {
        Ok(match self.chip.get_cpu_id()? {
            CpuId::M0 => self.get_m0_cpu_freq(),
            CpuId::D0 => todo!(),
            CpuId::LP => todo!(),
        })
    }

    /// Get the real frequency of the machine timer clock.
    pub fn get_mtimer_freq(&self) -> Result<u32, ()> {
        let (rtc, freq) = match self.chip.get_cpu_id()? {
            CpuId::M0 => (self.chip.mcu_misc.cpu_mtimer_rtc(), self.get_m0_cpu_freq()),
            CpuId::D0 => todo!(),
            CpuId::LP => todo!(),
        };
        Ok(freq / (rtc.get().divider().get() + 1))
    }

}


/// Methods to configure the crystal clock.
impl ChipClock<'_> {

    /// Get the socket's crystal type.
    /// 
    /// Use [`get_xtal_freq`] to get the real frequency.
    pub fn get_xtal_type(&self) -> CrystalClockType {
        let mut rsv3 = self.chip.hbn.rsv3().get();
        if rsv3.xtal_flag().get() == 0x58 {
            rsv3.xtal_type().get().into()
        } else {
            CrystalClockType::None
        }
    }

    pub fn set_xtal_type(&self, typ: CrystalClockType) {
        self.chip.hbn.rsv3().modify(|reg| {
            reg.xtal_flag().set(0x58);
            reg.xtal_type().set(typ as _);
        });
    }

    /// Get the socket's crystal frequency in Hz.
    pub fn get_xtal_freq(&self) -> u32 {
        match self.get_xtal_type() {
            CrystalClockType::None      => 0,
            CrystalClockType::Mhz24     => 24_000_000,
            CrystalClockType::Mhz32     => 32_000_000,
            CrystalClockType::Mhz38p4   => 38_400_000,
            CrystalClockType::Mhz40     => 40_000_000,
            CrystalClockType::Mhz26     => 26_000_000,
            CrystalClockType::Mhz32Rc   => 32_000_000,
        }
    }

}


/// Methods to configure `xclk`.
impl ChipClock<'_> {

    /// Get the selector for the main xclock freq.
    pub fn get_xclk_sel(&self) -> Mux2 {
        self.chip.hbn.glb().get().xclk_sel().get().into()
    }

    /// Get the main xclock frequency output from the multiplexer between:
    /// - 0 / RC 32MHz
    /// - 1 / Crystal
    pub fn get_xclk_freq(&self) -> u32 {
        match self.get_xclk_sel() {
            Mux2::Sel0 => 32_000_000,
            Mux2::Sel1 => self.get_xtal_freq(),
        }
    }

}


/// Methods for F32k clock.
impl ChipClock<'_> {

    /// Get the selector the F32k clock.
    pub fn get_f32k_sel(&self) -> Mux4 {
        self.chip.hbn.glb().get().f32k_sel().get().into()
    }

    /// Get the frequency for F32K clock, expected to be 32kHz
    /// but can vary if sourced from crystal clock.
    pub fn get_f32k_freq(&self) -> u32 {
        match self.get_f32k_sel() {
            Mux4::Sel0 | Mux4::Sel1 => 32_000,
            Mux4::Sel2 | Mux4::Sel3 => {
                let div = self.chip.glb.dig_clk_cfg0().get().dig_32k_div().get() + 1;
                self.get_xtal_freq() / div
            }
        }
    }

}


/// Methods for M0 (part of MCU) clocks.
impl ChipClock<'_> {

    /// Get the selector for the PLL MCU freq.
    pub fn get_m0_pll_sel(&self) -> Mux4 {
        self.chip.pds.cpu_core_cfg1().get().pll_sel().get().into()
    }

    /// Get the frequency output from MCU multiplexer 
    pub fn get_m0_pll_freq(&self) -> u32 {
        match self.get_m0_pll_sel() {
            Mux4::Sel0 => self.get_cpu_pll_freq(400_000_000),
            Mux4::Sel1 => self.get_audio_pll_freq(AudioPllType::Div1),
            Mux4::Sel2 => self.get_wifi_pll_freq(240_000_000),
            Mux4::Sel3 => self.get_wifi_pll_freq(320_000_000),
        }
    }

    /// Get the selector for the main MCU freq.
    pub fn get_m0_root_sel(&self) -> Mux2 {
        self.chip.hbn.glb().get().mcu_root_sel().get().into()
    }

    /// Get the frequency for M0 root clock.
    pub fn get_m0_root_freq(&self) -> u32 {
        match self.get_m0_root_sel() {
            Mux2::Sel0 => self.get_xclk_freq(),
            Mux2::Sel1 => self.get_m0_pll_freq(),
        }
    }

    /// Get the divider for M0 CPU clock.
    pub fn get_m0_cpu_div(&self) -> u32 {
        self.chip.glb.sys_cfg0().get().hclk_div().get() + 1
    }

    /// Get the frequency for M0 CPU clock.
    pub fn get_m0_cpu_freq(&self) -> u32 {
        let root_freq = self.get_m0_root_freq();
        root_freq / self.get_m0_cpu_div()
    }

    /// Get the divider for M0 secondary clock.
    pub fn get_m0_secondary_div(&self) -> u32 {
        self.chip.glb.sys_cfg0().get().bclk_div().get() + 1
    }

    pub fn get_m0_secondary_freq(&self) -> u32 {
        self.get_m0_cpu_freq() / self.get_m0_secondary_div()
    }

    /// Disable the M0 clock.
    pub fn disable_m0_clock(&self) {
        self.chip.pds.cpu_core_cfg1().modify(|reg| reg.mcu1_clk_en().set(0));
    }

}


/// Methods for MM clocks.
impl ChipClock<'_> {

    /// Get the selector for MM xclock.
    pub fn get_mm_xclk_sel(&self) -> Mux2 {
        self.chip.mm_glb.mm_clk_ctrl_cpu().get().xclk_clk_sel().get().into()
    }

    /// Set the selector for MM xclock.
    pub fn set_mm_xclk_sel(&self, sel: Mux2) {
        self.chip.mm_glb.mm_clk_ctrl_cpu().modify(|reg| reg.xclk_clk_sel().set(sel as _));
    }

    /// Get the frequency for MM xclock.
    pub fn get_mm_xclk_freq(&self) -> u32 {
        match self.get_mm_xclk_sel() {
            Mux2::Sel0 => 32_000_000,
            Mux2::Sel1 => self.get_xtal_freq(),
        }
    }

    /// Get the selector for MM PLL 160 MHz clock.
    pub fn get_mm_pll160_sel(&self) -> Mux2 {
        self.chip.glb.dig_clk_cfg1().get().mm_muxpll_160m_sel().get().into()
    }

    /// Get the frequency for MM PLL 160 MHz clock.
    pub fn get_mm_pll160_freq(&self) -> u32 {
        match self.get_mm_pll160_sel() {
            Mux2::Sel0 => self.get_wifi_pll_freq(160_000_000),
            Mux2::Sel1 => self.get_cpu_pll_freq(160_000_000),
        }
    }

    /// Get the selector for MM PLL 240 MHz clock.
    pub fn get_mm_pll240_sel(&self) -> Mux2 {
        self.chip.glb.dig_clk_cfg1().get().mm_muxpll_240m_sel().get().into()
    }

    /// Get the frequency for MM PLL 240 MHz clock.
    pub fn get_mm_pll240_freq(&self) -> u32 {
        match self.get_mm_pll240_sel() {
            Mux2::Sel0 => self.get_wifi_pll_freq(240_000_000),
            Mux2::Sel1 => self.get_audio_pll_freq(AudioPllType::Div2),
        }
    }

    /// Get the selector for MM PLL 320 MHz clock.
    pub fn get_mm_pll320_sel(&self) -> Mux2 {
        self.chip.glb.dig_clk_cfg1().get().mm_muxpll_320m_sel().get().into()
    }

    /// Get the frequency for MM PLL 320 MHz clock.
    pub fn get_mm_pll320_freq(&self) -> u32 {
        match self.get_mm_pll320_sel() {
            Mux2::Sel0 => self.get_wifi_pll_freq(320_000_000),
            Mux2::Sel1 => self.get_audio_pll_freq(AudioPllType::Div1),
        }
    }

    /// Get the selector for MM bclock.
    pub fn get_mm_bclk_sel(&self) -> Mux4 {
        self.chip.mm_glb.mm_clk_ctrl_cpu().get().bclk1x_sel().get().into()
    }

    /// Get the frequency for MM bclock.
    pub fn get_mm_bclk_freq(&self) -> u32 {
        match self.get_mm_bclk_sel() {
            Mux4::Sel0 | Mux4::Sel1 => self.get_mm_xclk_freq(),
            Mux4::Sel2 => self.get_mm_pll160_freq(),
            Mux4::Sel3 => self.get_mm_pll240_freq()
        }
    }

}


/// Methods for D0 (part of MM) clocks.
impl ChipClock<'_> {

    /// Get the selector for D0 PLL clock.
    pub fn get_d0_pll_sel(&self) -> Mux4 {
        self.chip.mm_glb.mm_clk_ctrl_cpu().get().cpu_clk_sel().get().into()
    }

    /// Get the frequency for D0 PLL clock.
    pub fn get_d0_pll_freq(&self) -> u32 {
        match self.get_d0_pll_sel() {
            Mux4::Sel0 => self.get_mm_pll240_freq(),
            Mux4::Sel1 => self.get_mm_pll320_freq(),
            Mux4::Sel2 | Mux4::Sel3 => self.get_cpu_pll_freq(400_000_000),
        }
    }

    /// Get the selector for D0 root clock.
    pub fn get_d0_root_sel(&self) -> Mux2 {
        self.chip.mm_glb.mm_clk_ctrl_cpu().get().cpu_root_clk_sel().get().into()
    }

    /// Set the selector for D0 root clock.
    pub fn set_d0_root_sel(&self, sel: Mux2) {
        self.chip.mm_glb.mm_clk_ctrl_cpu().modify(|reg| reg.cpu_root_clk_sel().set(sel as _));
    }

    /// Get the frequency for D0 root clock.
    pub fn get_d0_root_freq(&self) -> u32 {
        match self.get_d0_root_sel() {
            Mux2::Sel0 => self.get_mm_xclk_freq(),
            Mux2::Sel1 => self.get_d0_pll_freq(),
        }
    }

    /// Get the divider applied to the frequency for D0 CPU frequency.
    pub fn get_d0_cpu_div(&self) -> u32 {
        self.chip.mm_glb.mm_clk_cpu().get().cpu_clk_div().get() + 1
    }

    /// Get the final D0 CPU frequency.
    pub fn get_d0_cpu_freq(&self) -> u32 {
        let root_freq = self.get_d0_root_freq();
        root_freq / self.get_d0_cpu_div()
    }

    /// Get the divider applied to the frequency for D0 secondary frequency.
    pub fn get_d0_secondary_div(&self) -> u32 {
        self.chip.mm_glb.mm_clk_cpu().get().bclk2x_div().get() + 1
    }

    /// Get the final D0 secondary frequency.
    pub fn get_d0_secondary_freq(&self) -> u32 {
        self.get_d0_cpu_freq() / self.get_d0_secondary_div()
    }

    /// Disable the D0 clock.
    pub fn disable_d0_clock(&self) {
        self.chip.mm_glb.mm_clk_ctrl_cpu().modify(|reg| reg.mmcpu0_clk_en().set(0));
    }

}


/// Methods for LP clocks.
impl ChipClock<'_> {

    /// Get the divider for LP CPU clock.
    pub fn get_lp_cpu_div(&self) -> u32 {
        self.chip.pds.cpu_core_cfg7().get().pico_div().get()
    }

    /// Get the frequency for LP CPU clock.
    pub fn get_lp_cpu_freq(&self) -> u32 {
        self.get_m0_secondary_freq() / self.get_lp_cpu_div()
    }

    /// Disable the LP clock.
    pub fn disable_lp_clock(&self) {
        self.chip.pds.cpu_core_cfg0().modify(|reg| reg.pico_clk_en().set(0));
    }

}


/// Methods for PLL sources.
impl ChipClock<'_> {

    /// Common function for CPU & audio PLL frequency calculation.
    fn get_pll_vco_freq(&self, sdmin: u32) -> u32 {
        const CALC_DIV: u32 = 1 << 11;
        match self.get_xtal_type() {
            CrystalClockType::None     => return 0,
            CrystalClockType::Mhz24    => 24 * 1000 * sdmin / CALC_DIV * (1000 / 2),
            CrystalClockType::Mhz32    => 32 * 1000 * sdmin / CALC_DIV * (1000 / 4),
            CrystalClockType::Mhz38p4  => 384 * 100 * sdmin / CALC_DIV * (1000 / 4),
            CrystalClockType::Mhz40    => 40 * 1000 * sdmin / CALC_DIV * (1000 / 4),
            CrystalClockType::Mhz26    => 26 * 1000 * sdmin / CALC_DIV * (1000 / 2),
            CrystalClockType::Mhz32Rc  => 32 * 1000 * sdmin / CALC_DIV * (1000 / 4),
        }
    }

    pub fn set_pll_sel(&self, typ: PllType) {

        let _cfg1 = match typ {
            PllType::Wifi => self.chip.glb.wifi_pll_cfg1_(),
            PllType::Audio => self.chip.cci.audio_pll_cfg1_(),
            PllType::Cpu => self.chip.cci.cpu_pll_cfg1_(),
            PllType::Mipi => self.chip.glb.mipi_pll_cfg1_(),
            PllType::Uhs => self.chip.glb.uhs_pll_cfg1_(),
        };

        todo!()

    }

    /// Disable the given PLL source clock.
    pub fn disable_pll(&self, typ: PllType) {
        match typ {
            PllType::Wifi => self.chip.glb.wifi_pll_cfg0().modify(|reg| {
                reg.pu_wifipll().set(0);
                reg.pu_wifipll_sfreg().set(0);
            }),
            PllType::Audio => self.chip.cci.audio_pll_cfg0().modify(|reg| {
                reg.pu_aupll().set(0);
                reg.pu_aupll_sfreg().set(0);
            }),
            PllType::Cpu => self.chip.cci.cpu_pll_cfg0().modify(|reg| {
                reg.pu_cpupll().set(0);
                reg.pu_cpupll_sfreg().set(0);
            }),
            PllType::Mipi => self.chip.glb.mipi_pll_cfg0().modify(|reg| {
                reg.pu_mipipll().set(0);
                reg.pu_mipipll_sfreg().set(0);
            }),
            PllType::Uhs => self.chip.glb.uhs_pll_cfg0().modify(|reg| {
                reg.pu_uhspll().set(0);
                reg.pu_uhspll_sfreg().set(0);
            })
        }
    }

    /// Get the CPU PLL frequency.
    pub fn get_cpu_pll_freq(&self, pll_out: u32) -> u32 {
        let sdmin = self.chip.cci.cpu_pll_cfg6().get().cpupll_sdmin().get();
        match self.get_pll_vco_freq(sdmin) {
            475_000_000..=485_000_000 => pll_out / 100 * 120,
            395_000_000..=405_000_000 => pll_out,
            375_000_000..=385_000_000 => pll_out / 100 * 95,
            _ => 0
        }
    }

    /// Get the audio PLL frequency.
    pub fn get_audio_pll_freq(&self, pll_type: AudioPllType) -> u32 {

        let sdmin = self.chip.cci.audio_pll_cfg6().get().aupll_sdmin().get();
        let vco_freq = match self.get_pll_vco_freq(sdmin) {
            451_000_000..=452_000_000 => 451_584_000,
            442_000_000..=443_000_000 => 442_368_000,
            freq => freq,
        };

        match pll_type {
            AudioPllType::Div1 => vco_freq,
            AudioPllType::Div2 => vco_freq / 2,
            AudioPllType::Div2p5 => vco_freq * 2 / 5,
            AudioPllType::Div3 => vco_freq / 3,
            AudioPllType::Div4 => vco_freq / 4,
            AudioPllType::Div5 => vco_freq / 5,
            AudioPllType::Div6 => vco_freq / 6,
            AudioPllType::Div10 => vco_freq / 10,
            AudioPllType::Div15 => vco_freq / 15,
        }

    }

    /// Get the WIFI PLL frequency.
    pub fn get_wifi_pll_freq(&self, pll_out: u32) -> u32 {
        
        const CALC_DIV: u32 = 1 << 19;
        let sdmin = self.chip.glb.wifi_pll_cfg6().get().wifipll_sdmin().get();
        let vco_freq = match self.get_xtal_type() {
            CrystalClockType::None     => return 0,
            CrystalClockType::Mhz24    => sdmin / CALC_DIV * 24_000_000,
            CrystalClockType::Mhz32    => sdmin / CALC_DIV * 32_000_000 / 2,
            CrystalClockType::Mhz38p4  => sdmin / CALC_DIV * 38_400_000 / 2,
            CrystalClockType::Mhz40    => sdmin / CALC_DIV * 40_000_000 / 2,
            CrystalClockType::Mhz26    => 200 * sdmin / CALC_DIV * 26 * 5000,
            CrystalClockType::Mhz32Rc  => sdmin / CALC_DIV * 32_000_000 / 2,
        };

        match vco_freq {
            955000000..=965000000 => pll_out,
            _ => 0
        }

    }

}


/// All crystal clock types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum CrystalClockType {
    None = 0,
    Mhz24 = 1,
    Mhz32 = 2,
    Mhz38p4 = 3,
    Mhz40 = 4,
    Mhz26 = 5,
    Mhz32Rc = 6,
}

impl From<u32> for CrystalClockType {
    fn from(n: u32) -> Self {
        match n {
            1 => Self::Mhz24,
            2 => Self::Mhz32,
            3 => Self::Mhz38p4,
            4 => Self::Mhz40,
            5 => Self::Mhz26,
            6 => Self::Mhz32Rc, // TODO: Check if relevant (because it's only used to select RC32M)
            _ => Self::None
        }
    }
}

/// Selector signals for 2-channel multiplexers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Mux2 {
    Sel0 = 0,
    Sel1 = 1,
}

/// Selector signals for 4-channel multiplexers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Mux4 {
    Sel0 = 0,
    Sel1 = 1,
    Sel2 = 2,
    Sel3 = 3,
}

impl From<u32> for Mux2 {
    fn from(n: u32) -> Self {
        match n {
            0 => Self::Sel0,
            1 => Self::Sel1,
            _ => unreachable!()
        }
    }
}

impl From<u32> for Mux4 {
    fn from(n: u32) -> Self {
        match n {
            0 => Self::Sel0,
            1 => Self::Sel1,
            2 => Self::Sel2,
            3 => Self::Sel3,
            _ => unreachable!()
        }
    }
}


/// Types of audio pll dividers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioPllType {
    Div1,
    Div2,
    Div2p5,
    Div3,
    Div4,
    Div5,
    Div6,
    Div10,
    Div15,
}


/// Types of PLL between WIFI, Audio, CPU, MIPI, UHS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllType {
    Wifi,
    Audio,
    Cpu,
    Mipi,
    Uhs
}
