//! Low-level hart functions like linear hart id (allowing hart local values), it also
//! provides abstractions for low-level.

use core::sync::atomic::{Ordering, AtomicUsize};
use core::cell::{RefCell, Ref, RefMut};
use core::ops::Deref;
use core::arch::asm;

use critical_section::{Mutex, CriticalSection};


/// All supported CPUs (BL808) have one hart.
const HART_COUNT: usize = 1;

/// Statically checks the hart count is valid.
const _: () = assert!(HART_COUNT != 0);

/// The current number of started hart in the execution environment.
/// Initialized to 1 because we already count the ID 0, which is 
/// reserved for hardware hart 0. Unused when only hart 0.
static HART_COUNTER: AtomicUsize = AtomicUsize::new(1);


/// Init the current hart, should be called once per hart.
pub(crate) fn init() {

    let id;

    // Force hart zero to have first ID, easier to debug I guess.
    if hart_zero() {
        id = 0;
    } else {
        id = HART_COUNTER.fetch_add(1, Ordering::Relaxed);
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

    // Only use the mscratch register if there is more than one hart.
    if HART_COUNT > 1 {
        // FIXME: This happened to crash sometimes??
        unsafe {
            asm!("csrw mscratch, {}", in(reg) id);
        }
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
/// 
/// This function doesn't need `init` before being called.
#[inline(always)]
pub fn hart_zero() -> bool {
    // Only if 1 hart maximum is supported, we can optimize.
    if HART_COUNT == 1 {
        true
    } else {
        unsafe {
            let mut id: usize;
            asm!("csrr {}, mhartid", out(reg) id);
            // Return true restore state if previous bit was 1.
            id == 0
        }
    }
}

/// Return the actual number of harts.
#[inline]
pub fn hart_count() -> usize {
    HART_COUNTER.load(Ordering::Relaxed)
}

/// Spin loop hint adapted to low-level, it basically wait for 
/// interrupt on the hart.
#[inline(always)]
pub fn spin_loop() {
    unsafe { crate::arch::riscv::wfi() }
}

/// Data synchronization barrier. This function ensures that all memory accesses
/// are completed when this function returns.
#[inline(always)]
pub fn data_sync() {
    unsafe { crate::arch::riscv::fence() }
}

/// Instruction synchronization barrier. This function will flush the processor's 
/// instruction pipeline so that all instructions following the call are fetched from
/// cache or memory.
#[inline(always)]
pub fn inst_sync() {
    unsafe { crate::arch::riscv::ifence() }
}

/// This internal module is used if the critical section feature is
/// enabled, it provides implementation for the `critical_section` 
/// crate.
#[cfg(feature = "bl-critical-section")]
mod bl_critical_section {

    use critical_section::{RawRestoreState, Impl};
    use core::arch::asm;

    // Internal type to implement the critical section of BfLab.
    struct BlCriticalSection;
    critical_section::set_impl!(BlCriticalSection);

    unsafe impl Impl for BlCriticalSection {

        #[inline(always)]
        unsafe fn acquire() -> RawRestoreState {
            unsafe {
                // Atomically clear the mstatus.mie bit.
                let mut prev: u32;
                asm!("csrrci {}, mstatus, 0b1000", out(reg) prev);
                // Return true restore state if previous bit was 1.
                (prev & 0b1000) != 0
            }
        }

        #[inline(always)]
        unsafe fn release(restore_state: RawRestoreState) {
            // If we need to restore the interrupt to 1.
            if restore_state {
                unsafe {
                    asm!("csrsi mstatus, 0b1000");
                }
            }
        }
        
    }

}

/// Special type that allows defining static hart-local variables. It however does not 
/// provide interior mutability, so only const deref is implemented.
pub struct HartLocal<T> {
    /// Internal array containing instances of the value for each 
    /// hart.
    inner: [T; HART_COUNT],
}

impl<T> HartLocal<T> {

    /// Create a new hart-local variable, initialized to the given value.
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

/// Type alias for making a hart local with interrupt-safe mutex.
pub type HartLocalMutex<T> = HartLocal<Mutex<T>>;

impl<T> HartLocalMutex<T> {

    /// Create a new hart-local with interrupt-safe mutex for accessing underlying value.
    pub const fn new_mutex(value: T) -> Self {
        Self::new(Mutex::new(value))
    }


    #[inline]
    pub fn borrow<'cs>(&'cs self, cs: CriticalSection<'cs>) -> &'cs T {
        (**self).borrow(cs)
    }

}

/// Type alias for making a hart local with an interrupt-safe cell.
pub type HartLocalCell<T> = HartLocalMutex<RefCell<T>>;

impl<T> HartLocalCell<T> {

    /// Create a new hart-local with interrupt-safe cell for accessing underlying value.
    pub const fn new_cell(value: T) -> Self {
        Self::new(Mutex::new(RefCell::new(value)))
    }

    /// Borrow the internal ref cell while guaranteeing that caller is in critical 
    /// section. Being in critical section is required to avoid "dead-locking" (not
    /// really a dead-lock because borrowing will panic).
    #[inline]
    pub fn borrow_ref<'cs>(&'cs self, cs: CriticalSection<'cs>) -> Ref<'cs, T> {
        (**self).borrow_ref(cs)
    }

    /// Borrow the internal ref cell mutably while guaranteeing that caller is in critical 
    /// section. Read [`borrow`] method.
    #[inline]
    pub fn borrow_ref_mut<'cs>(&'cs self, cs: CriticalSection<'cs>) -> RefMut<'cs, T> {
        (**self).borrow_ref_mut(cs)
    }

}
