//! Timer management on BL808.

use core::sync::atomic::{AtomicBool, Ordering};

use alloc::collections::VecDeque;
use alloc::boxed::Box;

use critical_section::CriticalSection;
use smallvec::SmallVec;

use crate::hart::{HartLocalCell, HartLocal};
use crate::arch::bl808::addr;
use crate::clock::Clocks;
use crate::interrupt::MACHINE_TIMER;


/// The tick frequency of the core timer.
const FREQ: u32 = 1_000_000;

/// The value to set to time cmp in order to disable it (it's still
/// theoretically enabled but set to the maximum u64 micros, which
/// equals 584'868 years).
const DISABLED_TIME_CMP: u64 = u64::MAX;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
const RV32_MTIME: *mut u64 = addr::T_HEAD_RV32_MTIME_BASE as _;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
const RV32_MTIMECMP: *mut u64 = addr::T_HEAD_RV32_MTIMECMP_BASE as _;

#[cfg(feature = "bl808-d0")]
const RV64_MTIMECMP: *mut u64 = addr::T_HEAD_RV64_MTIMECMP0_BASE as _;


/// Providing exclusive access to the core's internal RTC timer configuration. This timer
/// is configured to have a *microsecond* resolution.
pub struct Timer(pub(crate) ());

impl Timer {

    /// Initialize the core timer frequency in the given clocks handle.
    /// 
    /// *Note: The core timer needs to be initialized on each core on hart 0.*
    pub fn init(&self, clocks: &mut Clocks) {
        let divider = clocks.get_mtimer_source_freq() / FREQ;
        clocks.enable_mtimer_clock(divider);
    }

    /// Set the time in microseconds.
    #[inline]
    pub fn set_time(&mut self, time: u64) {
        set_time(time)
    }

}


/// Get the current time in microseconds.
#[inline]
pub fn get_time() -> u64 {
    #[cfg(feature = "bl808-d0")]
    unsafe {
        let time: u64;
        core::arch::asm!("csrr {}, 0xC01", out(reg) time);
        time
    }
    #[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
    unsafe { RV32_MTIME.read_volatile() }
}

/// Set the time in microseconds.
#[inline]
fn set_time(time: u64) {
    #[cfg(feature = "bl808-d0")]
    unsafe {
        core::arch::asm!("csrw 0xC01, {}", in(reg) time);
    }
    #[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
    unsafe { RV32_MTIME.write_volatile(time) }
}

/// Get the time compare in microseconds.
#[inline]
#[allow(unused)]
fn get_time_cmp() -> u64 {
    #[cfg(feature = "bl808-d0")]
    unsafe { RV64_MTIMECMP.read_volatile() }
    #[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
    unsafe { RV32_MTIMECMP.read_volatile() }
}

/// Set the time compare in microseconds. A machine timer interrupt 
/// will be triggered whenever the time is greater or equal to this 
/// value.
/// 
/// *Note that you might need to update this value in order to 
/// reset the interrupt pending bit. Only resetting the time will 
/// not clear this bit.*
#[inline]
fn set_time_cmp(cmp: u64) {
    #[cfg(feature = "bl808-d0")]
    unsafe { RV64_MTIMECMP.write_volatile(cmp) }
    #[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
    unsafe { RV32_MTIMECMP.write_volatile(cmp) }
}


/// State of the callback after being called or not.
#[derive(Debug, Clone, Copy)]
enum TimerCallbackState {
    /// The callback has been consumed and should be removed from the queue.
    Consumed,
    /// The callback has been updated so it should be reordered in the queue.
    Updated,
    /// The callback is still pending for target time.
    Waiting,
}

/// Timer callback abstraction.
trait TimerCallback: Send {
    
    /// Return the target time.
    fn target_time(&self) -> u64;

    /// Try to call it, returning its (new) state.
    fn call(&mut self, time: u64) -> TimerCallbackState;

}

/// Descriptor for a callback and when to call it.
struct TimerCallbackImpl<F> {
    /// The target time for calling this callback.
    target_time: u64,
    /// The actual callback to call.
    callback: F,
}

impl<F> TimerCallback for TimerCallbackImpl<F> 
where
    F: FnMut() -> Option<u64>,
    F: Send + 'static
{
    
    #[inline]
    fn target_time(&self) -> u64 {
        self.target_time
    }

    #[inline]
    fn call(&mut self, time: u64) -> TimerCallbackState {
        if self.target_time <= time {
            if let Some(duration) = (self.callback)() {
                self.target_time = time + duration;
                TimerCallbackState::Updated
            } else {
                TimerCallbackState::Consumed
            }
        } else {
            TimerCallbackState::Waiting
        }
    }

}

/// The ordered queue of callbacks. It's hart local because timer interrupts are.
static CALLBACK_QUEUE: HartLocalCell<VecDeque<Box<dyn TimerCallback>>> = HartLocalCell::new_cell(VecDeque::new());
/// For each hart, tell if the timer interrupt has been enable on the current hart.
static INTERRUPT_ENABLED: HartLocal<AtomicBool> = HartLocal::new(AtomicBool::new(false));

/// Internal function to insert the given callback into the queue. **This function 
/// requires to be executed in an interrupt-free context to avoid deadlocking.**
fn insert_callback(queue: &mut VecDeque<Box<dyn TimerCallback>>, callback: Box<dyn TimerCallback>) {
    
    let target_time = callback.target_time();
    
    // Search for the index to insert at.
    let insert_idx = match queue.binary_search_by(|desc| {
        desc.target_time().cmp(&target_time)
    }) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };

    // The first queue element is the next one to be called.
    if insert_idx == 0 {
        set_time_cmp(target_time);
    }

    queue.insert(insert_idx, callback);

}

/// Synchronized wait, this function will block the current thread until the given 
/// duration (micros) has been waited. Prefer [`wait_callback`] or [`repeat_callback`] 
/// to use an interrupt-driven callback.
#[inline]
pub fn wait(duration: u64) {
    let start = get_time();
    while get_time() - start < duration { }
}

/// Wait for the given duration and then call the given callback. Note that it will be
/// called in an interrupt-free context. The callback can return `None` to just be 
/// consumed and removed from the queue, but it can also return `Some` new duration to
/// be called again in the future (returning 0 means that it will be called on next int).
pub fn wait_callback<F>(duration: u64, callback: F)
where
    F: FnMut() -> Option<u64>,
    F: Send + 'static
{
    critical_section::with(|cs| {

        // TODO: Check if relevant.
        if INTERRUPT_ENABLED.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            // SAFETY: We guard this call from 
            unsafe { MACHINE_TIMER.set_enabled(true); }
        }

        let mut queue = CALLBACK_QUEUE.borrow_ref_mut(cs);
        insert_callback(&mut queue, Box::new(TimerCallbackImpl {
            target_time: get_time() + duration,
            callback,
        }))

    });
}


/// This handler is called when the core time reaches the time cmp register.
pub(crate) fn mtimer_handler(_code: usize, cs: CriticalSection) {

    let mut queue = CALLBACK_QUEUE.borrow_ref_mut(cs);
    let time = get_time();

    // Temporary array of updated callbacks.
    let mut updated_callbacks = SmallVec::<[_; 4]>::new();

    // Call every callback that reached their target time and take the index of 
    // the last consumed one.
    while let Some(callback) = queue.front_mut() {
        match callback.call(time) {
            TimerCallbackState::Consumed => { queue.pop_front(); }
            TimerCallbackState::Updated => {
                updated_callbacks.push(queue.pop_front().unwrap());
            }
            TimerCallbackState::Waiting => {
                // Callbacks are ordered in the queue so we can just break here.
                break
            }
        }
    }

    // Consume the update callbacks vector and insert them again.
    for update_callback in updated_callbacks {
        insert_callback(&mut queue, update_callback);
    }

    // Then we get the front to update time compare register to the next callback,
    // or just disable it by setting it to the maximum value.
    if let Some(front) = queue.front() {
        set_time_cmp(front.target_time());
    } else {
        set_time_cmp(DISABLED_TIME_CMP);
    }

}
