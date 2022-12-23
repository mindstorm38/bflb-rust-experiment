//! Memory Mapped I/O structures definition.

// Generated files:
pub mod mcu_misc;
pub mod mm_misc;
pub mod mm_glb;
pub mod glb;
pub mod pds;
pub mod hbn;
pub mod cci;
pub mod sf_ctrl;
pub use mcu_misc::McuMisc;
pub use mm_misc::MmMisc;
pub use mm_glb::MmGlb;
pub use glb::Glb;
pub use pds::Pds;
pub use hbn::Hbn;
pub use cci::Cci;
pub use sf_ctrl::SfCtrl;


emhal::mmio_reg! {
    
    /// Common RTC register, used to have a common structure for all RTC configurations.
    pub struct CpuRtc: u32 {
        [0..10] divider,
        [30..31] reset,
        [31..32] enable,
    }

    /// Common `Cfg0` for PLL clock sources.
    pub struct PllCfg0: u32 {
        [0..1] pll_sdm_rstb,
        [2..3] pll_fbdv_rstb,
        [5..6] pu_pll_fbdv,
        [8..9] pu_pll_cp,
        [9..10] pu_pll_sfreg,
        [10..11] pu_pll,
    }

    /// Common `Cfg1` for PLL clock sources.
    pub struct PllCfg1: u32 {
        [0..7] pll_postdiv,
        [8..12] pll_refdiv_ratio,
        [16..18] pll_refclk_sel,
        [20..22] pll_vg11_sel,
        [24..26] pll_vg13_sel,
    }

}
