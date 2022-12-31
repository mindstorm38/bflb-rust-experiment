//! Timer management on BL808.

use core::marker::PhantomData;
use core::time::Duration;

use super::{AsCoreId, CoreId};
use super::clock::Clocks;
use super::addr;


/// Basic CoreTimer register controller.
pub struct CoreTimer<C> {
    core_id: PhantomData<C>,
    mtime: *const u64,
}

impl<C: AsCoreId> CoreTimer<C> {

    /// The tick frequency of the core timer.
    pub const FREQ: u32 = 1_000_000;

    pub fn new(core_id: C) -> Self {
        Self {
            core_id: PhantomData,
            mtime: match core_id.as_core_id() {
                CoreId::M0 => addr::T_HEAD_RV32_MTIME_BASE,
                CoreId::D0 => todo!(),
                CoreId::LP => addr::T_HEAD_RV32_MTIME_BASE,
            } as _
        }
    }

    /// Initialize the core timer frequency and other registers.
    /// 
    /// The core timer needs to be initialized on each core.
    pub fn init(&self, clocks: &mut Clocks<C>) {
        let divider = clocks.get_mtimer_source_freq() / Self::FREQ;
        clocks.enable_mtimer_clock(divider);
    }

    /// Get the current time. It's returned as a duration since startup
    /// of the clock.
    pub fn time(&self) -> Duration {
        Duration::from_micros(unsafe { self.mtime.read_volatile() })
    }

    /// Sleep for a specific duration.
    pub fn sleep(&self, duration: Duration) {
        let start = self.time();
        while self.time() - start < duration {}
    }

}
