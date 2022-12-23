//! HAL/PAC for BL808
//! 
//! This chip contains three CPUs:
//! - M0: Main CPU (32 bits)
//! - D0: Multimedia-oriented CPU (64 bits)
//! - LP: Low power CPU (32 bits embedded)
//! 
//! This chip has many different types of memories:
//! - Flash:    Application address space
//! - OCRAM:    Mainly for M0
//! - WRAM:     Mainly for wireless network data
//! - XRAM:     Shared RAM for communication between cores.
//! - DRAM:     Multimedia RAM used by D0 or modules like H264/NPU
//! - VRAM:     Multimedia RAM used by D0 or modules like H264/NPU

pub mod addr;
pub mod cpu;
pub mod mmio;
pub mod clock;
// pub mod flash;


/// BL808 Chip.
pub struct Chip {
    /// The CPU identifier register.
    pub cpu_id: *const u32,
    /// The MCU misc registers.
    pub mcu_misc: mmio::McuMisc,
    /// The MM misc registers.
    pub mm_misc: mmio::MmMisc,
    /// The MM global registers.
    pub mm_glb: mmio::MmGlb,
    /// The global registers.
    pub glb: mmio::Glb,
    /// The Power Down Sleep registers.
    pub pds: mmio::Pds,
    /// The Hibernate registers.
    pub hbn: mmio::Hbn,
    /// The ??? registers.
    pub cci: mmio::Cci,
    /// The Serial Flash Control registers.
    pub sf_ctrl: mmio::SfCtrl,
}

impl Chip {

    /// Build a const chip structure (better to use as a constant)
    /// with default addresses.
    pub const fn new() -> Self {
        Self { 
            cpu_id: addr::CORE_ID as _,
            mcu_misc: mmio::McuMisc(addr::MCU_MISC_BASE as _), 
            mm_misc: mmio::MmMisc(addr::MM_MISC_BASE as _), 
            mm_glb: mmio::MmGlb(addr::MM_GLB_BASE as _), 
            glb: mmio::Glb(addr::GLB_BASE as _), 
            pds: mmio::Pds(addr::PDS_BASE as _), 
            hbn: mmio::Hbn(addr::HBN_BASE as _), 
            cci: mmio::Cci(addr::CCI_BASE as _), 
            sf_ctrl: mmio::SfCtrl(addr::SF_CTRL_BASE as _),
        }
    }

    /// Get the CPU id where the current program is running on.
    pub fn get_cpu_id(&self) -> Result<CpuId, ()> {
        Ok(match unsafe { self.cpu_id.read_volatile() } {
            0xE9070000 => CpuId::M0,
            0xDEAD5500 => CpuId::D0,
            0xDEADE902 => CpuId::LP,
            _ => return Err(()),
        })
    }

}


/// The three possible core identifiers in the BL808.
#[derive(Debug, PartialEq, Eq)]
pub enum CpuId {
    /// T-head E907, 32-bit, RV32IMAFCP.
    /// 
    /// *Also called MCU in SDK.*
    M0,
    /// T-head C906, 64-bit, RV64IMAFCV.
    /// 
    /// *Also called DSP in SDK.*
    D0,
    /// T-head 902, 32-bit, RV32EMC.
    LP,
}


/// The two possible CPU groups.
#[derive(Debug, Clone, Copy,PartialEq, Eq)]
pub enum CpuGroup {
    Group0,
    Group1,
}
