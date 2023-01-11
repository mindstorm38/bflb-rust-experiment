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

use super::mmio::{CpuRtc, MCU_MISC, MM_MISC, PDS, HBN, AON, GLB, MM_GLB, CCI};
use super::{CoreId, AsCoreId};


/// Clocks controller for BL808.
pub struct Clocks<C> {
    core_id: C,
}

impl<C: AsCoreId> Clocks<C> {

    /// Create a new clocks controller given a CPU id.
    pub const fn new(core_id: C) -> Self {
        Self {
            core_id,
        }
    }

}


/// High-level mtimer methods.
impl<C: AsCoreId> Clocks<C> {

    /// Get the machine timer RTC register for the current core.
    pub fn get_mtimer_rtc_reg(&self) -> PtrRw<CpuRtc> {
        match self.core_id.as_core_id() {
            CoreId::M0 => MCU_MISC.cpu_mtimer_rtc(),
            CoreId::D0 => MM_MISC.cpu_mtimer_rtc(),
            CoreId::LP => PDS.cpu_mtimer_rtc(),
        }
    }

    /// Enable and configure the machine timer clock.
    pub fn enable_mtimer_clock(&self, div: u32) {
        debug_assert_ne!(div, 0, "divider must be nonzero");
        let rtc = self.get_mtimer_rtc_reg();
        rtc.modify(|reg| reg.enable().set(0));
        rtc.modify(|reg| reg.divider().set(div - 1));
        rtc.modify(|reg| reg.enable().set(1));
    }

    /// Disable the machine timer clock.
    pub fn disable_mtimer_clock(&self) {
        let rtc = self.get_mtimer_rtc_reg();
        rtc.modify(|reg| reg.enable().set(0));
    }

    /// Get machine timer divider.
    pub fn get_mtimer_div(&self) -> u32 {
        self.get_mtimer_rtc_reg().get().divider().get() + 1
    }

    /// Get the source frequency of the machine timer clock, 
    /// without RTC divider.
    /// To get the real frequency of the machine timer, use [`get_mtimer_freq`].
    pub fn get_mtimer_source_freq(&self) -> u32 {
        match self.core_id.as_core_id() {
            CoreId::M0 => self.get_m0_cpu_freq(),
            CoreId::D0 => todo!(),
            CoreId::LP => todo!(),
        }
    }

    /// Get the real frequency of the machine timer clock.
    pub fn get_mtimer_freq(&self) -> u32 {
        self.get_mtimer_source_freq() / self.get_mtimer_div()
    }

}


/// Methods to configure the crystal clock (external clock).
impl<C> Clocks<C> {

    /// Get the soc crystal type.
    /// 
    /// *Note that this is only informational, and not used by hardware,
    /// because the crystal clock is physically selected outside chip.*
    /// 
    /// Use [`get_xtal_freq`] to get the real frequency.
    pub fn get_xtal_type(&self) -> XtalType {
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
    pub fn set_xtal_type(&self, typ: XtalType) {
        HBN.rsv3().modify(|reg| {
            reg.xtal_flag().set(0x58);
            reg.xtal_type().set(typ as _);
        });
    }

    /// Get the socket's crystal frequency in Hz.
    pub fn get_xtal_freq(&self) -> u32 {
        match self.get_xtal_type() {
            XtalType::None      => 0,
            XtalType::Mhz24     => 24_000_000,
            XtalType::Mhz32     => 32_000_000,
            XtalType::Mhz38p4   => 38_400_000,
            XtalType::Mhz40     => 40_000_000,
            XtalType::Mhz26     => 26_000_000,
        }
    }

    /// Power on crystal clock and wait for it being enabled.
    pub fn enable_xtal(&self) -> Result<(), ()> {
        
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

}


/// Methods to configure `xclk`.
impl<C> Clocks<C> {

    /// Get the selector for the main xclock freq.
    pub fn get_xclk_sel(&self) -> Mux2 {
        HBN.glb().get().xclk_sel().get().into()
    }

    /// Set the selector for the main xclock freq.
    /// - 0 - RC 32MHz
    /// - 1 - Crystal
    pub fn set_xclk_sel(&self, sel: Mux2) {
        HBN.glb().modify(|reg| reg.xclk_sel().set(sel as _));
    }

    /// Get the main xclock frequency.
    pub fn get_xclk_freq(&self) -> u32 {
        match self.get_xclk_sel() {
            Mux2::Sel0 => 32_000_000,
            Mux2::Sel1 => self.get_xtal_freq(),
        }
    }

}


/// Methods for F32k clock.
impl<C> Clocks<C> {

    /// Get the selector the F32k clock.
    pub fn get_f32k_sel(&self) -> Mux4 {
        HBN.glb().get().f32k_sel().get().into()
    }

    /// Set the selector the F32k clock.
    /// - 0 - RC 32 kHz
    /// - 1 - Crystal 32 kHz
    /// - 2/3 - Crystal divided
    pub fn set_f32k_sel(&self, sel: Mux4) {
        HBN.glb().modify(|reg| reg.f32k_sel().set(sel as _));
    }

    /// Get the frequency for F32K clock, expected to be 32kHz
    /// but can vary if sourced from crystal clock.
    pub fn get_f32k_freq(&self) -> u32 {
        match self.get_f32k_sel() {
            Mux4::Sel0 | Mux4::Sel1 => 32_000,
            Mux4::Sel2 | Mux4::Sel3 => {
                let div = GLB.dig_clk_cfg0().get().dig_32k_div().get() + 1;
                self.get_xtal_freq() / div
            }
        }
    }

}


/// Methods for M0 (part of MCU) clocks.
impl<C> Clocks<C> {

    /// Get the selector for the PLL MCU freq.
    pub fn get_m0_pll_sel(&self) -> Mux4 {
        PDS.cpu_core_cfg1().get().pll_sel().get().into()
    }

    /// Get the frequency output from MCU multiplexer.
    pub fn get_m0_pll_freq(&self) -> u32 {
        match self.get_m0_pll_sel() {
            Mux4::Sel0 => self.get_cpu_pll_freq(400_000_000),
            Mux4::Sel1 => self.get_audio_pll_freq(PllAudioDiv::Div1),
            Mux4::Sel2 => self.get_wifi_pll_freq(240_000_000),
            Mux4::Sel3 => self.get_wifi_pll_freq(320_000_000),
        }
    }

    /// Get the selector for the main MCU freq.
    pub fn get_m0_root_sel(&self) -> Mux2 {
        HBN.glb().get().mcu_root_sel().get().into()
    }

    /// Set the selector for the main MCU freq.
    /// - 0 - Xclock
    /// - 1 - M0 PLL
    pub fn set_m0_root_sel(&self, sel: Mux2) {
        HBN.glb().modify(|reg| reg.mcu_root_sel().set(sel as _));
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
        GLB.sys_cfg0().get().hclk_div().get() + 1
    }

    /// Set the divider for M0 CPU clock.
    pub fn set_m0_cpu_div(&self, div: u32) {
        GLB.sys_cfg0().modify(|reg| reg.hclk_div().set(div - 1));
    }

    /// Get the frequency for M0 CPU clock.
    pub fn get_m0_cpu_freq(&self) -> u32 {
        let root_freq = self.get_m0_root_freq();
        root_freq / self.get_m0_cpu_div()
    }

    /// Get the divider for M0 secondary clock.
    pub fn get_m0_secondary_div(&self) -> u32 {
        GLB.sys_cfg0().get().bclk_div().get() + 1
    }

    /// Get the divider for M0 secondary clock.
    pub fn set_m0_secondary_div(&self, div: u32) {
        GLB.sys_cfg0().modify(|reg| reg.bclk_div().set(div - 1));
    }

    pub fn set_m0_secondary_div_act_pulse(&self, act: bool) {
        GLB.sys_cfg1().modify(|reg| reg.bclk_div_act_pulse().set(act as _));
    }

    pub fn get_m0_secondary_prot_done(&self) -> bool {
        GLB.sys_cfg1().get().sts_bclk_prot_done().get() != 0
    }

    pub fn get_m0_secondary_freq(&self) -> u32 {
        self.get_m0_cpu_freq() / self.get_m0_secondary_div()
    }

    /// Disable the M0 clock.
    pub fn disable_m0_clock(&self) {
        PDS.cpu_core_cfg1().modify(|reg| reg.mcu1_clk_en().set(0));
    }

}


/// Methods for MM clocks.
impl<C> Clocks<C> {

    /// Get the selector for MM xclock.
    pub fn get_mm_xclk_sel(&self) -> Mux2 {
        MM_GLB.mm_clk_ctrl_cpu().get().xclk_clk_sel().get().into()
    }

    /// Set the selector for MM xclock.
    /// - 0 - RC 32 MHz
    /// - 1 - Crystal
    pub fn set_mm_xclk_sel(&self, sel: Mux2) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.xclk_clk_sel().set(sel as _));
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
        GLB.dig_clk_cfg1().get().mm_muxpll_160m_sel().get().into()
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
        GLB.dig_clk_cfg1().get().mm_muxpll_240m_sel().get().into()
    }

    /// Get the frequency for MM PLL 240 MHz clock.
    pub fn get_mm_pll240_freq(&self) -> u32 {
        match self.get_mm_pll240_sel() {
            Mux2::Sel0 => self.get_wifi_pll_freq(240_000_000),
            Mux2::Sel1 => self.get_audio_pll_freq(PllAudioDiv::Div2),
        }
    }

    /// Get the selector for MM PLL 320 MHz clock.
    pub fn get_mm_pll320_sel(&self) -> Mux2 {
        GLB.dig_clk_cfg1().get().mm_muxpll_320m_sel().get().into()
    }

    /// Get the frequency for MM PLL 320 MHz clock.
    pub fn get_mm_pll320_freq(&self) -> u32 {
        match self.get_mm_pll320_sel() {
            Mux2::Sel0 => self.get_wifi_pll_freq(320_000_000),
            Mux2::Sel1 => self.get_audio_pll_freq(PllAudioDiv::Div1),
        }
    }

    /// Get the selector for MM bclock.
    pub fn get_mm_bclk_sel(&self) -> Mux4 {
        MM_GLB.mm_clk_ctrl_cpu().get().bclk1x_sel().get().into()
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
impl<C> Clocks<C> {

    /// Get the selector for D0 PLL clock.
    pub fn get_d0_pll_sel(&self) -> Mux4 {
        MM_GLB.mm_clk_ctrl_cpu().get().cpu_clk_sel().get().into()
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
        MM_GLB.mm_clk_ctrl_cpu().get().cpu_root_clk_sel().get().into()
    }

    /// Set the selector for D0 root clock.
    /// - 0 - MM xclock
    /// - 1 - D0 PLL
    pub fn set_d0_root_sel(&self, sel: Mux2) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.cpu_root_clk_sel().set(sel as _));
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
        MM_GLB.mm_clk_cpu().get().cpu_clk_div().get() + 1
    }

    /// Set the divider applied to the frequency for D0 CPU frequency.
    pub fn set_d0_cpu_div(&self, div: u32) {
        MM_GLB.mm_clk_cpu().modify(|reg| reg.cpu_clk_div().set(div - 1));
    }

    /// Get the final D0 CPU frequency.
    pub fn get_d0_cpu_freq(&self) -> u32 {
        let root_freq = self.get_d0_root_freq();
        root_freq / self.get_d0_cpu_div()
    }

    /// Get the divider applied to the frequency for D0 secondary frequency.
    pub fn get_d0_secondary_div(&self) -> u32 {
        MM_GLB.mm_clk_cpu().get().bclk2x_div().get() + 1
    }

    /// Get the divider applied to the frequency for D0 secondary frequency.
    pub fn set_d0_secondary_div(&self, div: u32) {
        MM_GLB.mm_clk_cpu().modify(|reg| reg.bclk2x_div().set(div - 1));
    }

    pub fn set_d0_secondary_div_act_pulse(&self, act: bool) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.bclk2x_div_act_pulse().set(act as _));
    }

    pub fn get_d0_secondary_prot_done(&self) -> bool {
        MM_GLB.mm_clk_ctrl_cpu().get().sts_bclk2x_prot_done().get() != 0
    }

    /// Get the final D0 secondary frequency.
    pub fn get_d0_secondary_freq(&self) -> u32 {
        self.get_d0_cpu_freq() / self.get_d0_secondary_div()
    }

    /// Disable the D0 clock.
    pub fn disable_d0_clock(&self) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.mmcpu0_clk_en().set(0));
    }

}


/// Methods for LP clocks.
impl<C> Clocks<C> {

    /// Get the divider for LP CPU clock.
    pub fn get_lp_cpu_div(&self) -> u32 {
        PDS.cpu_core_cfg7().get().pico_div().get()
    }

    /// Set the divider for LP CPU clock.
    pub fn set_lp_cpu_div(&self, div: u32) {
        PDS.cpu_core_cfg7().modify(|reg| reg.pico_div().set(div));
    }

    pub fn set_lp_cpu_div_act_pulse(&self, act: bool) {
        GLB.sys_cfg1().modify(|reg| reg.pico_clk_div_act_pulse().set(act as _));
    }

    pub fn get_lp_cpu_prot_done(&self) -> bool {
        GLB.sys_cfg1().get().sts_pico_clk_prot_done().get() != 0
    }

    /// Get the frequency for LP CPU clock.
    pub fn get_lp_cpu_freq(&self) -> u32 {
        self.get_m0_secondary_freq() / self.get_lp_cpu_div()
    }

    /// Disable the LP clock.
    pub fn disable_lp_clock(&self) {
        PDS.cpu_core_cfg0().modify(|reg| reg.pico_clk_en().set(0));
    }

}


/// Methods for UART peripherals.
impl<C> Clocks<C> {

    pub fn set_uart0_enable(&self, enable: bool) {
        GLB.cgen_cfg1().modify(|reg| reg.cgen_s1a_uart0().set(enable as _));
    }

    pub fn set_uart1_enable(&self, enable: bool) {
        GLB.cgen_cfg1().modify(|reg| reg.cgen_s1a_uart1().set(enable as _));
    }

    /// Enable global UART clock.
    pub fn set_uart_enable(&self, enable: bool) {
        GLB.uart_cfg0().modify(|reg| reg.uart_clk_en().set(enable as _));
    }

    /// Get the UART clock selector.
    pub fn get_uart_sel(&self) -> UartSel {
        let mut reg = HBN.glb().get();
        let sel_raw = (reg.uart_clk_sel2().get() << 1) | reg.uart_clk_sel().get();
        match sel_raw {
            0 => UartSel::SecondaryM0,
            1 => UartSel::Pll160,
            2 => UartSel::Xclock,
            _ => unreachable!("invalid uart clock selector")
        }
    }

    /// Set the UART clock selector.
    pub fn set_uart_sel(&self, ref_clock: UartSel) {
        HBN.glb().modify(|reg| {
            let val = ref_clock as u32;
            reg.uart_clk_sel2().set((val >> 1) & 1);
            reg.uart_clk_sel().set(val & 1);
        });
    }

    /// Get the divisor for UART clock.
    pub fn get_uart_div(&self) -> u32 {
        GLB.uart_cfg0().get().uart_clk_div().get() + 1
    }

    /// Set the divisor for UART clock.
    pub fn set_uart_div(&self, div: u32) {
        GLB.uart_cfg0().modify(|reg| reg.uart_clk_div().set(div - 1));
    }

    /// Get the final UART frequency.
    pub fn get_uart_freq(&self) -> u32 {
        let freq = match self.get_uart_sel() {
            UartSel::SecondaryM0 => self.get_m0_secondary_freq(),
            UartSel::Pll160 => todo!("pll160 is not implemented"),
            UartSel::Xclock => self.get_xclk_freq(),
        };
        freq / self.get_uart_div()
    }

    // pub fn set_uart_clock(&self, enable: bool, ref_clock: UartRefClock, div: u32) {

    //     GLB.uart_cfg0().modify(|reg| reg.uart_clk_en().set(0));
    //     GLB.uart_cfg0().modify(|reg| reg.uart_clk_div().set(div - 1));

    //     HBN.glb().modify(|reg| {
    //         let val = ref_clock as u32;
    //         reg.uart_clk_sel2().set((val >> 1) & 1);
    //         reg.uart_clk_sel().set(val & 1);
    //     });

    //     GLB.uart_cfg0().modify(|reg| reg.uart_clk_en().set(enable as _));

    // }

}


/// Methods for PLL sources.
impl<C> Clocks<C> {

    /// Set selector of the given PLL clock source.
    pub fn set_pll_sel(&self, typ: PllType, ref_clock: PllRefClock) {

        let cfg1 = match typ {
            PllType::Wifi => GLB.wifi_pll_cfg1_(),
            PllType::Audio => CCI.audio_pll_cfg1_(),
            PllType::Cpu => CCI.cpu_pll_cfg1_(),
            PllType::Mipi => GLB.mipi_pll_cfg1_(),
            PllType::Uhs => GLB.uhs_pll_cfg1_(),
        };

        let ref_val = match (typ, ref_clock) {
            (PllType::Wifi, PllRefClock::Xtal) => 1,
            (_, PllRefClock::Xtal) => 0,
            (_, PllRefClock::Rc32m) => 3,
        };

        cfg1.modify(|reg| {
            reg.pll_refclk_sel().set(ref_val);
        });

    }

    /// Disable the given PLL source clock.
    pub fn disable_pll(&self, typ: PllType) {

        let cfg0 = match typ {
            PllType::Wifi => GLB.wifi_pll_cfg0_(),
            PllType::Audio => CCI.audio_pll_cfg0_(),
            PllType::Cpu => CCI.cpu_pll_cfg0_(),
            PllType::Mipi => GLB.mipi_pll_cfg0_(),
            PllType::Uhs => GLB.uhs_pll_cfg0_(),
        };

        cfg0.modify(|reg| {
            reg.pu_pll().set(0);
            reg.pu_pll_sfreg().set(0);
        });

    }

    /// Enable the Wifi PLL with a specific configuration.
    pub fn enable_wifi_pll(&self, config: &PllWacConfig) {

        GLB.wifi_pll_cfg1().modify(|reg| {
            reg.wifipll_refdiv_ratio().set(config.basic.refdiv_ratio as _);
        });

        GLB.wifi_pll_cfg2().modify(|reg| {
            reg.wifipll_int_frac_sw().set(config.basic.int_frac_sw as _);
            reg.wifipll_icp_1u().set(config.basic.icp_1u as _);
            reg.wifipll_icp_5u().set(config.basic.icp_5u as _);
        });

        GLB.wifi_pll_cfg3().modify(|reg| {
            reg.wifipll_rz().set(config.basic.rz as _);
            reg.wifipll_cz().set(config.basic.cz as _);
            reg.wifipll_c3().set(config.basic.c3 as _);
            reg.wifipll_r4_short().set(config.basic.r4_short as _);
            reg.wifipll_c4_en().set(config.basic.c4_en as _);
        });

        GLB.wifi_pll_cfg4().modify(|reg| {
            reg.wifipll_sel_sample_clk().set(config.basic.sel_sample_clk as _);
        });

        GLB.wifi_pll_cfg5().modify(|reg| {
            reg.wifipll_vco_speed().set(config.basic.vco_speed as _);
        });

        GLB.wifi_pll_cfg6().modify(|reg| {
            reg.wifipll_sdm_ctrl_hw().set(config.basic.sdm_ctrl_hw as _);
            reg.wifipll_sdm_bypass().set(config.basic.sdm_bypass as _);
            reg.wifipll_sdmin().set(config.sdmin as _);
        });

        let cfg0 = GLB.wifi_pll_cfg0();

        cfg0.modify(|reg| reg.pu_wifipll_sfreg().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(3)).unwrap();
        cfg0.modify(|reg| reg.pu_wifipll().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(3)).unwrap();

        cfg0.modify(|reg| reg.wifipll_sdm_rstb().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.wifipll_sdm_rstb().set(0));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.wifipll_sdm_rstb().set(1));

        cfg0.modify(|reg| reg.wifipll_fbdv_rstb().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.wifipll_fbdv_rstb().set(0));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.wifipll_fbdv_rstb().set(1));

        GLB.wifi_pll_cfg5().modify(|reg| {
            reg.wifipll_vco_div3_en().set(1);
        });

        GLB.wifi_pll_cfg8().modify(|reg| {
            reg.wifipll_en_ctrl_hw().set(1);
            reg.wifipll_en_div4().set(1);
            reg.wifipll_en_div5().set(1);
            reg.wifipll_en_div6().set(1);
            reg.wifipll_en_div8().set(1);
            reg.wifipll_en_div10().set(1);
        });

    }

    /// Enable the Audio PLL with a specific configuration.
    pub fn enable_audio_pll(&self, config: &PllWacConfig) {

        CCI.audio_pll_cfg1().modify(|reg| {
            reg.aupll_refdiv_ratio().set(config.basic.refdiv_ratio as _);
        });

        CCI.audio_pll_cfg2().modify(|reg| {
            reg.aupll_int_frac_sw().set(config.basic.int_frac_sw as _);
            reg.aupll_icp_1u().set(config.basic.icp_1u as _);
            reg.aupll_icp_5u().set(config.basic.icp_5u as _);
        });

        CCI.audio_pll_cfg3().modify(|reg| {
            reg.aupll_rz().set(config.basic.rz as _);
            reg.aupll_cz().set(config.basic.cz as _);
            reg.aupll_c3().set(config.basic.c3 as _);
            reg.aupll_r4_short().set(config.basic.r4_short as _);
            reg.aupll_c4_en().set(config.basic.c4_en as _);
        });

        CCI.audio_pll_cfg4().modify(|reg| {
            reg.aupll_sel_sample_clk().set(config.basic.sel_sample_clk as _);
        });

        CCI.audio_pll_cfg5().modify(|reg| {
            reg.aupll_vco_speed().set(config.basic.vco_speed as _);
        });

        CCI.audio_pll_cfg6().modify(|reg| {
            reg.aupll_sdm_bypass().set(config.basic.sdm_bypass as _);
            reg.aupll_sdmin().set(config.sdmin as _);
        });

        let cfg0 = CCI.audio_pll_cfg0();

        cfg0.modify(|reg| reg.pu_aupll_sfreg().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(3)).unwrap();
        cfg0.modify(|reg| reg.pu_aupll().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(3)).unwrap();

        cfg0.modify(|reg| reg.aupll_sdm_rstb().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.aupll_sdm_rstb().set(0));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.aupll_sdm_rstb().set(1));

        cfg0.modify(|reg| reg.aupll_fbdv_rstb().set(1));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.aupll_fbdv_rstb().set(0));
        // self.chip.timer().sleep_arch(Duration::from_micros(2)).unwrap();
        cfg0.modify(|reg| reg.aupll_fbdv_rstb().set(1));

        CCI.audio_pll_cfg1().modify(|reg| {
            if let 0x12D0E | 0x1C395 | 0x17851 | 0x16944 | 0x115E5 = config.sdmin {
                reg.aupll_postdiv().set(0x14);
            } else {
                reg.aupll_postdiv().set(0x12);
            }
        });

        CCI.audio_pll_cfg8().modify(|reg| {
            reg.aupll_en_div1().set(1);
            reg.aupll_en_div2().set(1);
            reg.aupll_en_div2p5().set(1);
            reg.aupll_en_div5().set(1);
            reg.aupll_en_div6().set(1);
        });

    }

    // TODO: enable_cpu_pll
    // TODO: enable_mipi_pll
    // TODO: enable_uhs_pll

    /// Common function for CPU & audio PLL frequency calculation.
    fn get_pll_vco_freq(&self, sdmin: u32) -> u32 {
        const CALC_DIV: u32 = 1 << 11;
        match self.get_xtal_type() {
            XtalType::None     => return 0,
            XtalType::Mhz24    => 24 * 1000 * sdmin / CALC_DIV * (1000 / 2),
            XtalType::Mhz32    => 32 * 1000 * sdmin / CALC_DIV * (1000 / 4),
            XtalType::Mhz38p4  => 384 * 100 * sdmin / CALC_DIV * (1000 / 4),
            XtalType::Mhz40    => 40 * 1000 * sdmin / CALC_DIV * (1000 / 4),
            XtalType::Mhz26    => 26 * 1000 * sdmin / CALC_DIV * (1000 / 2),
            // CrystalClockType::Mhz32Rc  => 32 * 1000 * sdmin / CALC_DIV * (1000 / 4),
        }
    }

    /// Get the CPU PLL frequency.
    pub fn get_cpu_pll_freq(&self, pll_out: u32) -> u32 {
        let sdmin = CCI.cpu_pll_cfg6().get().cpupll_sdmin().get();
        match self.get_pll_vco_freq(sdmin) {
            475_000_000..=485_000_000 => pll_out / 100 * 120,
            395_000_000..=405_000_000 => pll_out,
            375_000_000..=385_000_000 => pll_out / 100 * 95,
            _ => 0
        }
    }

    /// Get the audio PLL frequency.
    pub fn get_audio_pll_freq(&self, div: PllAudioDiv) -> u32 {

        let sdmin = CCI.audio_pll_cfg6().get().aupll_sdmin().get();
        let vco_freq = match self.get_pll_vco_freq(sdmin) {
            451_000_000..=452_000_000 => 451_584_000,
            442_000_000..=443_000_000 => 442_368_000,
            freq => freq,
        };

        match div {
            PllAudioDiv::Div1 => vco_freq,
            PllAudioDiv::Div2 => vco_freq / 2,
            PllAudioDiv::Div2p5 => vco_freq * 2 / 5,
            PllAudioDiv::Div3 => vco_freq / 3,
            PllAudioDiv::Div4 => vco_freq / 4,
            PllAudioDiv::Div5 => vco_freq / 5,
            PllAudioDiv::Div6 => vco_freq / 6,
            PllAudioDiv::Div10 => vco_freq / 10,
            PllAudioDiv::Div15 => vco_freq / 15,
        }

    }

    /// Get the WIFI PLL frequency.
    pub fn get_wifi_pll_freq(&self, pll_out: u32) -> u32 {
        
        const CALC_DIV: u32 = 1 << 19;
        let sdmin = GLB.wifi_pll_cfg6().get().wifipll_sdmin().get();
        let vco_freq = match self.get_xtal_type() {
            XtalType::None     => return 0,
            XtalType::Mhz24    => sdmin / CALC_DIV * 24_000_000,
            XtalType::Mhz32    => sdmin / CALC_DIV * 32_000_000 / 2,
            XtalType::Mhz38p4  => sdmin / CALC_DIV * 38_400_000 / 2,
            XtalType::Mhz40    => sdmin / CALC_DIV * 40_000_000 / 2,
            XtalType::Mhz26    => 200 * sdmin / CALC_DIV * 26 * 5000,
            // CrystalClockType::Mhz32Rc  => sdmin / CALC_DIV * 32_000_000 / 2,
        };

        match vco_freq {
            955000000..=965000000 => pll_out,
            _ => 0
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


/// Selector for UART clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum UartSel {
    /// Source from secondary M0 clock.
    SecondaryM0 = 0,
    /// From mux PLL 160 MHz.
    Pll160 = 1,
    /// From xclock.
    Xclock = 2,
}


/// Types of audio pll dividers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllAudioDiv {
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

/// Types of PLL between Wifi, Audio, CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllWacType {
    Wifi,
    Audio,
    Cpu,
}

/// Types of PLL between MIPI, UHS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllMuType {
    Mipi,
    Uhs
}

/// The reference clock for PLL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PllRefClock {
    /// Sources from soc crystal.
    Xtal,
    /// Sources from RC 32 MHz
    Rc32m,
}

/// Basic configuration for WAC PLL clocks.
#[derive(Debug, Clone)]
pub struct PllWacBasicConfig {
    pub refdiv_ratio: u8,
    pub int_frac_sw: u8,
    pub icp_1u: u8,
    pub icp_5u: u8,
    pub rz: u8,
    pub cz: u8,
    pub c3: u8,
    pub r4_short: u8,
    pub c4_en: bool,
    pub sel_sample_clk: u8,
    pub vco_speed: u8,
    pub sdm_ctrl_hw: u8,
    pub sdm_bypass: bool,
}

/// Configuration for WAC PLL clocks.
#[derive(Debug, Clone)]
pub struct PllWacConfig<'a> {
    pub basic: &'a PllWacBasicConfig,
    pub sdmin: u32,
}

/// Basic configuration for WAC PLL clocks.
#[derive(Debug, Clone)]
pub struct PllMuBasicConfig {
    pub refdiv_ratio: u8,
    pub sel_sample: u8,
    pub even_div_en: bool,
    pub even_div_ratio: u8,
}

/// Configuration for WAC PLL clocks.
#[derive(Debug, Clone)]
pub struct PllMuConfig<'a> {
    pub basic: &'a PllMuBasicConfig,
    pub sdmin: u32,
}


/// Wifi PLL config with 24 MHz.
pub const PLL_CFG_WIFI_24: PllWacBasicConfig = PllWacBasicConfig {
    refdiv_ratio: 1,
    int_frac_sw: 0,
    icp_1u: 0,
    icp_5u: 2,
    rz: 3,
    cz: 1,
    c3: 2,
    r4_short: 1,
    c4_en: false,
    sel_sample_clk: 1,
    vco_speed: 5,
    sdm_ctrl_hw: 1,
    sdm_bypass: true,
};

/// Wifi PLL config with 32, 38.4 or 40 MHz.
pub const PLL_CFG_WIFI_32_38P4_40: PllWacBasicConfig = PllWacBasicConfig {
    refdiv_ratio: 2,
    ..PLL_CFG_WIFI_24
};

/// Wifi PLL config with 26 MHz.
pub const PLL_CFG_WIFI_26: PllWacBasicConfig = PllWacBasicConfig {
    refdiv_ratio: 1,
    int_frac_sw: 1,
    icp_1u: 1,
    icp_5u: 0,
    rz: 5,
    cz: 2,
    c3: 2,
    r4_short: 0,
    c4_en: true,
    sel_sample_clk: 1,
    vco_speed: 5,
    sdm_ctrl_hw: 0,
    sdm_bypass: false,
};

/// Wifi PLL configs for 960 MHz.
pub const PLL_CFG_WIFI_960: [PllWacConfig; 5] = [
    PllWacConfig { basic: &PLL_CFG_WIFI_24, sdmin: 0x1400000 },
    PllWacConfig { basic: &PLL_CFG_WIFI_32_38P4_40, sdmin: 0x1E00000 },
    PllWacConfig { basic: &PLL_CFG_WIFI_32_38P4_40, sdmin: 0x1900000 },
    PllWacConfig { basic: &PLL_CFG_WIFI_32_38P4_40, sdmin: 0x1800000 },
    PllWacConfig { basic: &PLL_CFG_WIFI_26, sdmin: 0x1276276 },
];
