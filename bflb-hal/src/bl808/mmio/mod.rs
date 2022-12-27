//! Memory Mapped I/O structures definitions.
//! 
//! The goal is to provides good APIs to manipulate both memory mapped 
//! structures with read or/and write fields and registers field by 
//! field.
//! 
//! Many modules are generated from C headers by 'tools/parse_reg.py'.

pub mod mcu_misc;
pub mod mm_misc;
pub mod mm_glb;
pub mod glb;
pub mod pds;
pub mod hbn;
pub mod cci;
pub mod sf_ctrl;
pub mod aon;
pub use mcu_misc::McuMisc;
pub use mm_misc::MmMisc;
pub use mm_glb::MmGlb;
pub use glb::Glb;
pub use pds::Pds;
pub use hbn::Hbn;
pub use cci::Cci;
pub use sf_ctrl::SfCtrl;
pub use aon::Aon;

// Manually implemented.
pub mod uart;
pub use uart::Uart;


use super::addr;


/// The register that stores the CPU identifier.
pub const CPU_ID: CpuId     = CpuId(addr::CPU_ID_BASE as _);
/// The MCU misc registers.
pub const MCU_MISC: McuMisc = McuMisc(addr::MCU_MISC_BASE as _);
/// The MM misc registers.
pub const MM_MISC: MmMisc   = MmMisc(addr::MM_MISC_BASE as _);
/// The MM global registers.
pub const MM_GLB: MmGlb     = MmGlb(addr::MM_GLB_BASE as _);
/// The global registers.
pub const GLB: Glb          = Glb(addr::GLB_BASE as _);
/// The Power Down Sleep registers.
pub const PDS: Pds          = Pds(addr::PDS_BASE as _);
/// The Hibernate registers.
pub const HBN: Hbn          = Hbn(addr::HBN_BASE as _);
/// The ??? registers.
pub const AON: Aon          = Aon(addr::AON_BASE as _);
/// The ??? registers.
pub const CCI: Cci          = Cci(addr::CCI_BASE as _);
/// The Serial Flash Control registers.
pub const SF_CTRL: SfCtrl   = SfCtrl(addr::SF_CTRL_BASE as _);

pub const UART0: Uart       = Uart(addr::UART0_BASE as _);
pub const UART1: Uart       = Uart(addr::UART1_BASE as _);
pub const UART2: Uart       = Uart(addr::UART2_BASE as _);


emhal::mmio_struct! {

    /// The CPU identification structure, contains only one read-only
    /// field containing the numeric ID.
    pub struct CpuId {
        [0] r cpu_id: u32,
    }

}


emhal::mmio_reg! {
    
    /// Common RTC register, used to have a common structure for all RTC configurations.
    pub struct CpuRtc: u32 {
        [0..10] divider,
        [30..31] reset,
        [31..32] enable,
    }

    /// Common `Cfg0` for PLL clock sources.
    /// 
    /// Note that some fields are reserved for Wifi, Audio, CPU (WAC).
    pub struct PllCfg0: u32 {
        /// Common to all PLLs.
        [0..1] pll_sdm_rstb,
        /// **Reserved to WAC PLLs.**
        [1..2] pll_postdiv_rstb,
        /// Common to all PLLs.
        [2..3] pll_fbdv_rstb,
        /// **Reserved to WAC PLLs.**
        [3..4] pll_refdiv_rstb,
        /// **Reserved to WAC PLLs.**
        [4..5] pu_pll_postdiv,
        /// Common to all PLLs.
        [5..6] pu_pll_fbdv,
        /// **Reserved to WAC PLLs.**
        [6..7] pu_pll_clamp_op,
        /// **Reserved to WAC PLLs.**
        [7..8] pu_pll_pfd,
        /// Common to all PLLs.
        [8..9] pu_pll_cp,
        /// Common to all PLLs.
        [9..10] pu_pll_sfreg,
        /// Common to all PLLs.
        [10..11] pu_pll,
        /// **Reserved to WAC PLLs.**
        [11..12] pu_pll_clktree,
    }

    /// Common `Cfg1` for PLL clock sources.
    /// 
    /// Note that some fields are reserved for Wifi, Audio, CPU (WAC) or
    /// MIPI, UHS (MU).
    pub struct PllCfg1: u32 {
        /// **Reserved to MU PLLs.**
        [0..7] pll_even_div_ratio,
        /// **Reserved to MU PLLs.**
        [7..8] pll_even_div_en,
        /// **Reserved to WAC PLLs.**
        [0..7] pll_postdiv,
        /// Common to all PLLs.
        [8..12] pll_refdiv_ratio,
        /// Common to all PLLs.
        [16..18] pll_refclk_sel,
        /// Common to all PLLs.
        [20..22] pll_vg11_sel,
        /// Common to all PLLs.
        [24..26] pll_vg13_sel,
    }

}
