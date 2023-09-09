//! Module for configuring the various PLL (Phase-Locked Loop) clocks.

use crate::arch::bl808::{GLB, CCI};

use super::{XtalType, get_xtal_type};


/// Set selector of the given PLL clock source.
pub unsafe fn set_pll_sel(typ: PllType, ref_clock: PllRefClock) {

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
pub unsafe fn disable_pll(typ: PllType) {

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
pub unsafe fn enable_wifi_pll(config: &PllWacConfig) {

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
pub unsafe fn enable_audio_pll(config: &PllWacConfig) {

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
fn get_pll_vco_freq(sdmin: u32) -> u32 {
    const CALC_DIV: u32 = 1 << 11;
    match get_xtal_type() {
        XtalType::None     => 0,
        XtalType::Mhz24    => 24 * 1000 * sdmin / CALC_DIV * (1000 / 2),
        XtalType::Mhz32    => 32 * 1000 * sdmin / CALC_DIV * (1000 / 4),
        XtalType::Mhz38p4  => 384 * 100 * sdmin / CALC_DIV * (1000 / 4),
        XtalType::Mhz40    => 40 * 1000 * sdmin / CALC_DIV * (1000 / 4),
        XtalType::Mhz26    => 26 * 1000 * sdmin / CALC_DIV * (1000 / 2),
    }
}

/// Get the CPU PLL frequency.
pub fn get_cpu_pll_freq(pll_out: u32) -> u32 {
    let sdmin = CCI.cpu_pll_cfg6().get().cpupll_sdmin().get();
    match get_pll_vco_freq(sdmin) {
        475_000_000..=485_000_000 => pll_out / 100 * 120,
        395_000_000..=405_000_000 => pll_out,
        375_000_000..=385_000_000 => pll_out / 100 * 95,
        _ => 0
    }
}

/// Get the audio PLL frequency.
pub fn get_audio_pll_freq(div: PllAudioDiv) -> u32 {

    let sdmin = CCI.audio_pll_cfg6().get().aupll_sdmin().get();
    let vco_freq = match get_pll_vco_freq(sdmin) {
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
pub fn get_wifi_pll_freq(pll_out: u32) -> u32 {
    
    const CALC_DIV: u32 = 1 << 19;
    let sdmin = GLB.wifi_pll_cfg6().get().wifipll_sdmin().get();
    let vco_freq = match get_xtal_type() {
        XtalType::None     => return 0,
        XtalType::Mhz24    => sdmin / CALC_DIV * 24_000_000,
        XtalType::Mhz32    => sdmin / CALC_DIV * 32_000_000 / 2,
        XtalType::Mhz38p4  => sdmin / CALC_DIV * 38_400_000 / 2,
        XtalType::Mhz40    => sdmin / CALC_DIV * 40_000_000 / 2,
        XtalType::Mhz26    => 200 * sdmin / CALC_DIV * 26 * 5000,
    };

    match vco_freq {
        955000000..=965000000 => pll_out,
        _ => 0
    }

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
