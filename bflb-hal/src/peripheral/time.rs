//! Timer management on BL808.

use embedded_util::peripheral;

use crate::bl808::addr;
use super::clock::Clocks;


/// Providing access to the core's internal RTC timer. This timer is configured 
/// to have a *microsecond* resolution. 
/// 
/// **Note that** you have to be careful not  to create this structure multiple
/// time, even if this is not inherently unsafe.
pub struct CoreTimer {}
peripheral!(CoreTimer);

impl CoreTimer {

    /// The tick frequency of the core timer.
    pub const FREQ: u32 = 1_000_000;

    #[cfg(any(feature = "bl808_m0", feature = "bl808_lp"))]
    const RV32_MTIME: *mut u64 = addr::T_HEAD_RV32_MTIME_BASE as _;
    
    #[cfg(any(feature = "bl808_m0", feature = "bl808_lp"))]
    const RV32_MTIMECMP: *mut u64 = addr::T_HEAD_RV32_MTIMECMP_BASE as _;

    #[cfg(feature = "bl808_d0")]
    const RV64_MTIMECMP: *mut u64 = addr::T_HEAD_RV64_MTIMECMP0_BASE as _;

    /// Initialize the core timer frequency in the given clocks handle.
    /// 
    /// *Note: The core timer needs to be initialized on each core.*
    pub fn init(&self, clocks: &mut Clocks) {
        let divider = clocks.get_mtimer_source_freq() / Self::FREQ;
        clocks.enable_mtimer_clock(divider);
    }

    /// Get the current time in microseconds.
    #[inline]
    pub fn get_time(&self) -> u64 {
        #[cfg(feature = "bl808_d0")]
        unsafe {
            let time: u64;
            core::arch::asm!("csrr {}, 0xC01", out(reg) time);
            time
        }
        #[cfg(any(feature = "bl808_m0", feature = "bl808_lp"))]
        unsafe { Self::RV32_MTIME.read_volatile() }
    }

    /// Set the time in microseconds.
    #[inline]
    pub fn set_time(&mut self, time: u64) {
        #[cfg(feature = "bl808_d0")]
        unsafe {
            core::arch::asm!("csrw 0xC01, {}", in(reg) time);
        }
        #[cfg(any(feature = "bl808_m0", feature = "bl808_lp"))]
        unsafe { Self::RV32_MTIME.write_volatile(time) }
    }

    /// Get the time compare in microseconds.
    #[inline]
    pub fn get_time_cmp(&self) -> u64 {
        #[cfg(feature = "bl808_d0")]
        unsafe { Self::RV64_MTIMECMP.read_volatile() }
        #[cfg(any(feature = "bl808_m0", feature = "bl808_lp"))]
        unsafe { Self::RV32_MTIMECMP.read_volatile() }
    }

    /// Set the time compare in microseconds. A machine timer interrupt will
    /// be triggered whenever the time is greater or equal to this value.
    /// 
    /// *Note that you might need to update this value in order to reset the
    /// interrupt pending bit. Only resetting the time will not clear this
    /// bit.*
    #[inline]
    pub fn set_time_cmp(&mut self, cmp: u64) {
        #[cfg(feature = "bl808_d0")]
        unsafe { Self::RV64_MTIMECMP.write_volatile(cmp) }
        #[cfg(any(feature = "bl808_m0", feature = "bl808_lp"))]
        unsafe { Self::RV32_MTIMECMP.write_volatile(cmp) }
    }

}
