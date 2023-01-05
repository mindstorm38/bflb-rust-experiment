//! Time related abstrations.

use core::time::Duration;


/// Implement this trait on any timers types.
pub trait Timer {

    /// Return the current time of this timer. You can choose the origin
    /// of your choice for the timer, T0 is not necesarilly unix epoch.
    /// 
    /// **You must** ensure that any call never return a time
    /// prior to a previously call.
    fn now(&self) -> Duration;

    /// Sleep for a specific duration.
    fn sleep(&self, duration: Duration) {
        let start = self.now();
        while self.now() - start < duration {}
    }

}


/// Implement this trait on all [`Timer`] implementations that are guaranteed
/// to return a duration with [`now`] method with T0 being Unix epoch.
pub trait DateTimer: Timer {}
