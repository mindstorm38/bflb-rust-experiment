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
// pub mod cpu;
pub mod mmio;
pub mod clock;
// pub mod timer;

pub mod gpio;
pub mod uart;


/// Get the CPU id where the current program is running on.
pub fn get_cpu_id() -> Result<CpuId, ()> {
    Ok(match mmio::CPU_ID.cpu_id().get() {
        0xE9070000 => CpuId::M0,
        0xDEAD5500 => CpuId::D0,
        0xDEADE902 => CpuId::LP,
        _ => return Err(()),
    })
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
