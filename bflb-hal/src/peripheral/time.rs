//! Timer management on BL808.

use core::marker::PhantomData;
use core::task::{Context, Poll, Waker};
use core::future::Future;
use core::pin::Pin;

use heapless::Vec;
use spin::Mutex;

use crate::bl808::addr;
use super::clock::Clocks;
    

/// The tick frequency of the core timer.
const FREQ: u32 = 1_000_000;

/// The value to set to time cmp in order to disable it (it's still
/// theoretically enabled but set to the maximum u64 micros, which
/// equals 584'868 years).
const DISABLED_TIME_CMP: u64 = u64::MAX;


/// Providing access to the core's internal RTC timer. This timer is 
/// configured to have a *microsecond* resolution. 
/// 
/// **Note that** you have to be careful not to create this structure 
/// multiple time, even if this is not inherently unsafe.
pub struct Timer(pub(crate) ());

impl Timer {

    /// Initialize the core timer frequency in the given clocks handle.
    /// 
    /// *Note: The core timer needs to be initialized on each core.*
    pub fn init(&self, clocks: &mut Clocks) {
        let divider = clocks.get_mtimer_source_freq() / FREQ;
        clocks.enable_mtimer_clock(divider);
    }

    /// Get the current time in microseconds.
    #[inline]
    pub fn get_time(&self) -> u64 {
        get_time()
    }

    /// Set the time in microseconds.
    #[inline]
    pub fn set_time(&mut self, time: u64) {
        set_time(time)
    }

    /// Synchronized wait, this function will block the current thread
    /// until the given duration has been waited. Prefer the async
    /// variant [`wait`].
    #[inline]
    pub fn wait_block(&self, duration: u64) {
        let start = get_time();
        while get_time() - start < duration {
            core::hint::spin_loop();
        }
    }

    /// Asynchronous wait, this function should be used in an async
    /// context and allows doing other tasks while waiting. Note that
    /// this function take self by shared reference, no exclusive
    /// access is required to wait, and you can wait on multiple 
    /// tasks at once.
    pub fn wait(&self, duration: u64) -> impl Future<Output = ()> + '_ {

        let target_time = get_time() + duration;
        
        // SAFETY: Use a critical section to spin lock.
        critical_section::with(|_| {
            ASYNC_STATE.lock().update_min_time_cmp(target_time);
        });

        WaitFuture {
            _ref: PhantomData,
            target_time
        }

    }

}


/// Future implementing RTC wait.
pub struct WaitFuture<'a> {
    /// The target time which this future will be ready.
    pub target_time: u64,
    /// The lifetime is used to ensure that the timer will not be
    /// accessed outside 
    _ref: PhantomData<&'a ()>,
}

impl<'a> Future for WaitFuture<'a> {

    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        // SAFETY: Use a critical section to spin lock. This is also
        // useful when setting the time cmp register to do all the
        // things atomically and avoid being interrupted before 
        // registering the waker.
        critical_section::with(|_| {

            if get_time() >= self.target_time {
                Poll::Ready(())
            } else {

                let mut state = ASYNC_STATE.lock();
                state.push_waker(cx.waker());
                state.update_min_time_cmp(self.target_time);

                Poll::Pending

            }

        })

    }

}


/// This handler is called when the core time reaches the time cmp
/// register. 
/// 
/// Note that we wake all wakers and this may wake futures that are 
/// still pending, but these ones will push the new waker again.
pub(crate) fn mtimer_handler(_code: usize) {

    // SAFETY: We can spin lock here because no interrupt can be taken
    // in an interrupt handler (this function).
    ASYNC_STATE.lock().call_wakers();

}


/// Internal list of wakers that are used to wake the wait futures.
/// 
/// SAFETY: This is really important to only access this mutex inside
/// critical sections where interrupts are disabled, to avoid dead
/// locking.
static ASYNC_STATE: Mutex<AsyncState> = Mutex::new(AsyncState::new());


/// Internal async state.
struct AsyncState {
    wakers: Vec<Waker, 32>,
    min_time_cmp: u64,
}

impl AsyncState {

    const fn new() -> Self {
        Self {
            wakers: Vec::new(),
            min_time_cmp: DISABLED_TIME_CMP,
        }
    }

    /// Push the given waker to the current async state. It will be
    /// called on the next timer interrupt. It returns true if the
    /// waker has been successfully pushed.
    fn push_waker(&mut self, waker: &Waker) -> bool {
        self.wakers.push(waker.clone()).is_ok()
    }

    /// This function drain all wakers and wake them. It will reset
    /// the min time cmp to the max value (in order to disable the
    /// interrupt). This is usually called in the interrupt handler.
    fn call_wakers(&mut self) {
        for waker in &self.wakers {
            waker.wake_by_ref();
        }
        self.wakers.clear();
        self.min_time_cmp = DISABLED_TIME_CMP;
        set_time_cmp(DISABLED_TIME_CMP);
    }

    /// Update the internal minimum time cmp. If the given time cmp is
    /// smaller than the current one, the current one is set to the
    /// given cmp, and this value is written to the time cmp register.
    fn update_min_time_cmp(&mut self, time_cmp: u64) {
        if time_cmp < self.min_time_cmp {
            self.min_time_cmp = time_cmp;
            set_time_cmp(self.min_time_cmp);
        }
    }

}


#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
const RV32_MTIME: *mut u64 = addr::T_HEAD_RV32_MTIME_BASE as _;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
const RV32_MTIMECMP: *mut u64 = addr::T_HEAD_RV32_MTIMECMP_BASE as _;

#[cfg(feature = "bl808-d0")]
const RV64_MTIMECMP: *mut u64 = addr::T_HEAD_RV64_MTIMECMP0_BASE as _;


/// Get the current time in microseconds.
#[inline]
fn get_time() -> u64 {
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
