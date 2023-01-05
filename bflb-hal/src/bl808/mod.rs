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

pub mod mmio;
pub mod addr;

pub mod irq;

pub mod clock;
pub mod time;

pub mod gpio;
pub mod uart;
pub mod usb;

pub mod camera;
pub mod mjpeg;


/// Get the core id where the current program is running on.
pub fn get_core_id() -> Option<CoreId> {
    Some(match mmio::CORE_ID.core_id().get() {
        0xE9070000 => CoreId::M0,
        0xDEAD5500 => CoreId::D0,
        0xDEADE902 => CoreId::LP,
        _ => return None,
    })
}


/// The three possible core identifiers in the BL808.
/// 
/// This enumeration is used where execution depends
/// on the actual core type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreId {
    /// T-head E907, 32-bit, RV32IMAFCP.
    /// 
    /// *Also called MCU in SDK.*
    M0,
    /// T-head C906, 64-bit, RV64IMAFCV.
    /// 
    /// *Also called DSP in SDK.*
    D0,
    /// T-head E902, 32-bit, RV32EMC.
    LP,
}

/// Statically-known `CpuId` for M0.
/// 
/// This structure can be used as generic parameter in
/// order to change code behaviours at compile-time.
#[derive(Debug, Clone, Copy)]
pub struct CoreM0;

/// Statically-known `CpuId` for D0.
/// 
/// This structure can be used as generic parameter in
/// order to change code behaviours at compile-time.
#[derive(Debug, Clone, Copy)]
pub struct CoreD0;

/// Statically-known `CpuId` for LP.
/// 
/// This structure can be used as generic parameter in
/// order to change code behaviours at compile-time.
#[derive(Debug, Clone, Copy)]
pub struct CoreLP;

/// A trait to cheaply convert a CPU structure ([`CoreM0`, 
/// `CoreD0`, `CoreLP`] or [`CoreId`]) into a `CoreId`.
/// 
/// When using particular core structures, this convertion
/// can be resolved and optimized at compile time. This
/// can provides slightly shorter machine codes.
pub trait AsCoreId: Copy {

    /// Convert to the [`CoreId`] enumeration.
    /// 
    /// The implementation of this function is marked 
    /// `inline(always)` on concrete core structures.
    fn as_core_id(self) -> CoreId;
    
}

impl AsCoreId for CoreM0 {
    #[inline(always)]
    fn as_core_id(self) -> CoreId {
        CoreId::M0
    }
}

impl AsCoreId for CoreD0 {
    #[inline(always)]
    fn as_core_id(self) -> CoreId {
        CoreId::D0
    }
}

impl AsCoreId for CoreLP {
    #[inline(always)]
    fn as_core_id(self) -> CoreId {
        CoreId::LP
    }
}


/// The two possible CPU groups.
#[derive(Debug, Clone, Copy,PartialEq, Eq)]
pub enum CpuGroup {
    Group0,
    Group1,
}
