//! Timer management on BL808.

use core::time::Duration;

use emhal::time::Timer;

use super::{AsCoreId, CoreId};
use super::clock::Clocks;
use super::addr;


/// Providing access to the core's internal RTC timer. This timer is configured 
/// to have a *microsecond* resolution. 
/// 
/// **Note that** you have to be careful not  to create this structure multiple
/// time, even if this is not inherently unsafe.
pub struct CoreTimer<C> {
    /// The core id.
    core_id: C,
}

impl<C: AsCoreId> CoreTimer<C> {

    /// The tick frequency of the core timer.
    pub const FREQ: u32 = 1_000_000;

    const RV32_MTIME: *mut u64 = addr::T_HEAD_RV32_MTIME_BASE as _;
    const RV32_MTIMECMP: *mut u64 = addr::T_HEAD_RV32_MTIMECMP_BASE as _;
    const RV64_MTIMECMP: *mut u64 = addr::T_HEAD_RV64_MTIMECMP0_BASE as _;

    pub const fn new(core_id: C) -> Self {
        Self {
            core_id
        }
    }

    /// Initialize the core timer frequency in the given clocks handle.
    /// 
    /// *Note: The core timer needs to be initialized on each core.*
    pub fn init(&self, clocks: &Clocks<C>) {
        let divider = clocks.get_mtimer_source_freq() / Self::FREQ;
        clocks.enable_mtimer_clock(divider);
    }

    #[inline(always)]
    fn is_rv64(&self) -> bool {
        matches!(self.core_id.as_core_id(), CoreId::D0)
    }

    /// Get the current time in microseconds.
    pub fn get_time(&self) -> u64 {
        unsafe { 
            if self.is_rv64() {
                #[cfg(target_pointer_width = "64")]
                {
                    let time: usize;
                    core::arch::asm!("csrr {}, 0xC01", out(reg) time);
                    time as u64
                }
                #[cfg(not(target_pointer_width = "64"))]
                {
                    panic!("incoherent pointer width");
                }
            } else {
                Self::RV32_MTIME.read_volatile()
            }
        }
    }

    /// Set the time in microseconds.
    pub fn set_time(&self, time: u64) {
        unsafe {
            if self.is_rv64() {
                #[cfg(target_pointer_width = "64")]
                {
                    core::arch::asm!("csrw 0xC01, {}", in(reg) time)
                }
                #[cfg(not(target_pointer_width = "64"))]
                { 
                    panic!("incoherent pointer width");
                }
            } else {
                Self::RV32_MTIME.write_volatile(time)
            }
        }
    }

    /// Get the time compare in microseconds.
    pub fn get_time_cmp(&self) -> u64 {
        unsafe {
            if self.is_rv64() {
                Self::RV64_MTIMECMP.read_volatile()
            } else {
                Self::RV32_MTIMECMP.read_volatile()
            }
        }
    }

    /// Set the time compare in microseconds. A machine timer interrupt will
    /// be triggered whenever the time is greater or equal to this value.
    /// 
    /// *Note that you might need to update this value in order to reset the
    /// interrupt pending bit. Only resetting the time will not clear this
    /// bit.*
    pub fn set_time_cmp(&self, cmp: u64) {
        unsafe {
            if self.is_rv64() {
                Self::RV64_MTIMECMP.write_volatile(cmp)
            } else {
                Self::RV32_MTIMECMP.write_volatile(cmp)
            }
        }
    }

}

impl<C: AsCoreId> Timer for CoreTimer<C> {

    fn now(&self) -> Duration {
        Duration::from_micros(self.get_time())
    }

}
