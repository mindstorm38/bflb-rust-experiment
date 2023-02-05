//! Utilities for traps.

use core::sync::atomic::{Ordering, AtomicUsize};


/// A trap handler type is a pointer to a function that take the 
pub type TrapHandler = fn(usize);


/// A structure for storing data about trap handlers (exceptions or interrupts).
/// 
/// This structure is intentionnaly made to be static and immutable.
pub struct TrapHandlers<const LEN: usize> {
    /// Pointer to a particular trap handler functions.
    /// 
    /// We are using usize atomics because it's guaranteed to have the same layout
    /// as usize and it's practical for const initialization of the array. This
    /// number can however be null. It's more like a `Option<fn()>`.
    handlers: [AtomicUsize; LEN],
}

impl<const LEN: usize> TrapHandlers<LEN> {

    /// Create a new trap handlers structure.
    pub const fn new() -> Self {
        const HANDLER_DEFAULT: AtomicUsize = AtomicUsize::new(0);
        Self {
            handlers: [HANDLER_DEFAULT; LEN],
        }
    }

    /// Set the trap handler function of a given trap number.
    /// This operation is atomic and therefore safe to call.
    #[inline(always)]
    pub fn set(&self, num: usize, handler: TrapHandler) {
        // We intentionnaly use relaxed ordering because we don't care 
        // of its ordering.
        self.handlers[num].store(handler as *const () as _, Ordering::Relaxed);
    }

    /// Get the trap handler of a given trap number.
    /// This operation is atomic and therefore safe to call.
    #[inline(always)]
    pub fn get(&self, num: usize, default: TrapHandler) -> TrapHandler {
        match self.handlers[num].load(Ordering::Relaxed) {
            0 => default,
            // SAFETY: We ensure that the atomic pointer is never null,
            // and because usize, pointers and function pointers have the
            // same layout.
            addr => unsafe { core::mem::transmute::<_, TrapHandler>(addr) }
        }
    }

}
