//! Memory Mapped I/O structures definitions for BL808.
//! 
//! HAL/PAC for BL808
//! 
//! This chip contains three CPUs:
//! - M0: Main CPU (T-head E907, 32-bit, RV32IMAFCP)
//! - D0: Multimedia-oriented CPU (T-head C906, 64-bit, RV64IMAFCV)
//! - LP: Low power CPU (T-head E902, 32-bit, RV32EMC)
//! 
//! This chip has many different types of memories:
//! - Flash:    Application address space
//! - OCRAM:    Mainly for M0
//! - WRAM:     Mainly for wireless network data
//! - XRAM:     Shared RAM for communication between cores.
//! - DRAM:     Multimedia RAM used by D0 or modules like H264/NPU
//! - VRAM:     Multimedia RAM used by D0 or modules like H264/NPU
//! 
//! Many modules are generated from C headers by 'tools/parse_reg.py'.


// Providing register addresses.
pub mod addr;

// Automatically implemented (see tools/parse_reg.py).
pub mod mcu_misc;
pub mod mm_misc;
pub mod mm_glb;
pub mod glb;
pub mod pds;
pub mod hbn;
pub mod cci;
pub mod sf_ctrl;
pub mod aon;
pub mod gpip;
pub use mcu_misc::McuMisc;
pub use mm_misc::MmMisc;
pub use mm_glb::MmGlb;
pub use glb::Glb;
pub use pds::Pds;
pub use hbn::Hbn;
pub use cci::Cci;
pub use sf_ctrl::SfCtrl;
pub use aon::Aon;
pub use gpip::Gpip;

// Manually implemented.
pub mod uart;
pub use uart::Uart;
pub mod dma;
pub use dma::Dma;
pub mod cam;
pub use cam::{Cam, CamFront};
pub mod mjpeg;
pub use mjpeg::Mjpeg;
pub mod csi;
pub use csi::Csi;

// Externally implemented.
use riscv_hal::clic::Clic;


/// The register that stores the CPU identifier.
pub const CORE_ID: CoreId       = CoreId(addr::CPU_ID_BASE as _);
/// The MCU misc registers.
pub const MCU_MISC: McuMisc     = McuMisc(addr::MCU_MISC_BASE as _);
/// The MM misc registers.
pub const MM_MISC: MmMisc       = MmMisc(addr::MM_MISC_BASE as _);
/// The MM global registers.
pub const MM_GLB: MmGlb         = MmGlb(addr::MM_GLB_BASE as _);
/// The global registers.
pub const GLB: Glb              = Glb(addr::GLB_BASE as _);
/// The Power Down Sleep registers.
pub const PDS: Pds              = Pds(addr::PDS_BASE as _);
/// The Hibernate registers.
pub const HBN: Hbn              = Hbn(addr::HBN_BASE as _);
/// The ??? registers.
pub const AON: Aon              = Aon(addr::AON_BASE as _);
/// The ??? registers.
pub const CCI: Cci              = Cci(addr::CCI_BASE as _);
/// The Serial Flash Control registers.
pub const SF_CTRL: SfCtrl       = SfCtrl(addr::SF_CTRL_BASE as _);
/// General Purpose ??
pub const GPIP: Gpip            = Gpip(addr::GPIP_BASE as _);

// DMA
pub const DMA0: Dma             = Dma(addr::DMA0_BASE as _);
pub const DMA1: Dma             = Dma(addr::DMA1_BASE as _);
pub const DMA2: Dma             = Dma(addr::DMA2_BASE as _);

// UART
pub const UART0: Uart           = Uart(addr::UART0_BASE as _);
pub const UART1: Uart           = Uart(addr::UART1_BASE as _);
pub const UART2: Uart           = Uart(addr::UART2_BASE as _);

// CAMERA
pub const CAM_FRONT: CamFront   = CamFront(addr::ISP_MISC_BASE as _);
pub const CAM0: Cam             = Cam(addr::DVP0_BASE as _);
pub const CAM1: Cam             = Cam(addr::DVP1_BASE as _);
pub const CAM2: Cam             = Cam(addr::DVP2_BASE as _);
pub const CAM3: Cam             = Cam(addr::DVP3_BASE as _);
pub const CAM4: Cam             = Cam(addr::DVP4_BASE as _);
pub const CAM5: Cam             = Cam(addr::DVP5_BASE as _);
pub const CAM6: Cam             = Cam(addr::DVP6_BASE as _);
pub const CAM7: Cam             = Cam(addr::DVP7_BASE as _);

// MIPI
pub const CSI: Csi              = Csi(addr::CSI_BASE as _);

// VIDEO
pub const MJPEG: Mjpeg          = Mjpeg(addr::MJPEG_DEC_BASE as _);

/// Core-Local Interrupt Controller registers.
/// 
/// Each core has particular CLIC configuration:
/// - M0: 96 interrupts, 4 bits priority
/// - D0: No CLIC (TODO: Check that)
/// - LP: 32 interrupts
pub const CLIC: Clic            = Clic(addr::T_HEAD_RV32_CLIC_BASE as _);

embedded_util::mmio! {

    /// The CPU identification structure, contains only one read-only
    /// field containing the numeric ID.
    pub struct CoreId {
        [0] ro core_id: u32,
    }

}


embedded_util::reg! {
    
    /// Common RTC register, used to have a common structure for all RTC configurations.
    pub struct CpuRtc: u32 {
        [00..10] divider,
        [30..31] reset,
        [31..32] enable,
    }

    /// Common `Cfg0` for PLL clock sources.
    /// 
    /// Note that some fields are reserved for Wifi, Audio, CPU (WAC).
    pub struct PllCfg0: u32 {
        /// Common to all PLLs.
        [00..01] pll_sdm_rstb,
        /// **Reserved to WAC PLLs.**
        [01..02] pll_postdiv_rstb,
        /// Common to all PLLs.
        [02..03] pll_fbdv_rstb,
        /// **Reserved to WAC PLLs.**
        [03..04] pll_refdiv_rstb,
        /// **Reserved to WAC PLLs.**
        [04..05] pu_pll_postdiv,
        /// Common to all PLLs.
        [05..06] pu_pll_fbdv,
        /// **Reserved to WAC PLLs.**
        [06..07] pu_pll_clamp_op,
        /// **Reserved to WAC PLLs.**
        [07..08] pu_pll_pfd,
        /// Common to all PLLs.
        [08..09] pu_pll_cp,
        /// Common to all PLLs.
        [09..10] pu_pll_sfreg,
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
        [00..07] pll_even_div_ratio,
        /// **Reserved to MU PLLs.**
        [07..08] pll_even_div_en,
        /// **Reserved to WAC PLLs.**
        [00..07] pll_postdiv,
        /// Common to all PLLs.
        [08..12] pll_refdiv_ratio,
        /// Common to all PLLs.
        [16..18] pll_refclk_sel,
        /// Common to all PLLs.
        [20..22] pll_vg11_sel,
        /// Common to all PLLs.
        [24..26] pll_vg13_sel,
    }

}
