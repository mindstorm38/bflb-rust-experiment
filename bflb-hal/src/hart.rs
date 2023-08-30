//! This module provides hart initialization code that allows a major
//! feature that is hart-local variables. The runtime is based on the
//! value stored in `mscratch` which is a unique hart ID. We are not
//! relying on `mhartid` because IDs are not guaranteed to be 
//! contiguous.

use core::sync::atomic::{Ordering, AtomicUsize};
use core::ops::Deref;
use core::arch::asm;


/// All supported CPUs (BL808) have one hart.
const HART_COUNT: usize = 1;

/// Statically checks the hart count is valid.
const _: () = assert!(HART_COUNT != 0);

/// The current number of started hart in the execution environment.
/// Initialized to 1 because we already count the ID 0, which is 
/// reserved for hardware hart 0. Unused when only hart 0.
static HART_COUNTER: AtomicUsize = AtomicUsize::new(1);


/// Init the current hart, should be called once on hart 
/// initialization, just before chip specific initializer.
pub fn init() {

    let id;

    // Force hart zero to have first ID, easier to debug I guess.
    if hart_zero() {
        id = 0;
    } else {
        id = HART_COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    // If the id is greater or equal to architectural maximum defined
    // in the chip module, the id is unexpected and we panic to avoid
    // running in code that may encounter undefined behaviors.
    if id >= HART_COUNT {
        // For now we spin loop, maybe panic in the future but we need
        // to ensure that the panic handler will not access a 
        // HartLocal variable.
        loop { spin_loop() }
    }
    
    unsafe {
        asm!("csrw mscratch, {}", in(reg) id);
    }

}

/// Internal function that returns the unique identifier of the
/// currently running hart.
#[inline(always)]
pub fn hart() -> usize {
    // Only if 1 hart maximum is supported, we know that only 0 can
    // be returned. This will allow inlining of the 0 constant which
    // will optimize all HartLocal accesses.
    if HART_COUNT == 1 {
        0
    } else {
        let id;
        unsafe {
            asm!("csrr {}, mscratch", out(reg) id);
        }
        id
    }
}

/// This function returns true if the hart is the hart 0, which is
/// responsible of most startup tasks. The specification states that
/// exactly one hart must have this ID. This runtime also ensures that
/// this hart 0 will have the id 0 returned by `hart()` function.
#[inline(always)]
pub fn hart_zero() -> bool {
    // Only if 1 hart maximum is supported, we can optimize.
    if HART_COUNT == 1 {
        true
    } else {
        unsafe {
            // Atomically clear the mstatus.mie bit.
            let mut id: usize;
            asm!("csrr {}, mhartid", out(reg) id);
            // Return true restore state if previous bit was 1.
            id == 0
        }
    }
}

/// Spin loop hint adapted to low-level, it basically wait for 
/// interrupt on the hart.
#[inline(always)]
pub fn spin_loop() {
    unsafe {
        asm!("wfi");
    }
}

/// Disable interrupts on the current hart if needed, and returns 
/// true if the interrupt was enabled.
#[inline(always)]
pub fn acquire_interrupt() -> bool {
    unsafe {
        // Atomically clear the mstatus.mie bit.
        let mut prev: u32;
        asm!("csrrci {}, mstatus, 0b1000", out(reg) prev);
        // Return true restore state if previous bit was 1.
        (prev & 0b1000) != 0
    }
}

/// Enable interrupts if it was previously the case, depending on
/// the restore argument.
#[inline(always)]
pub fn release_interrupt(restore: bool) {
    // If we need to restore the interrupt to 1.
    if restore {
        unsafe {
            asm!("csrsi mstatus, 0b1000");
        }
    }
}

/// Execute the given closure in an interrupt-free context.
#[inline(always)]
pub fn without_interrupt<T>(func: impl FnOnce() -> T) -> T {
    let restore = acquire_interrupt();
    let ret = func();
    release_interrupt(restore);
    ret
}


/// Special type that allows defining static hart-local variables. It
/// however does not provide interior mutability, so only const deref
/// is implemented.
pub struct HartLocal<T> {
    /// Internal array containing instances of the value for each 
    /// hart.
    inner: [T; HART_COUNT],
}

impl<T> HartLocal<T> {

    /// Create a new hart-local variable, initialized to the given
    /// value.
    pub const fn new(value: T) -> Self {
        Self {
            inner: [value; HART_COUNT]
        }
    }

}

/// The hart-local container is safe to sync, in order to be put in
/// a static context.
unsafe impl<T> Sync for HartLocal<T> {}

impl<T> Deref for HartLocal<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: We rely on the fact that the `init` function checks
        // for the hart id to be lower than `HART_COUNT`, if it's not
        // the case the hart spin loops, so they can't get here 
        // because no hart local variable is accessed.
        unsafe { self.inner.get_unchecked(hart()) }
    }

}
