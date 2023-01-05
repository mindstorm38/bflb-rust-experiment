//! Timer management on BL808.

use core::marker::PhantomData;
use core::time::Duration;

use emhal::time::Timer;

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

}

impl<C: AsCoreId> Timer for CoreTimer<C> {

    fn now(&self) -> Duration {
        Duration::from_micros(unsafe { self.mtime.read_volatile() })
    }

}
