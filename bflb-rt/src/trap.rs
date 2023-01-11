//! Utilities for traps.

use core::sync::atomic::{Ordering, AtomicUsize};
use core::mem::ManuallyDrop;


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
    /// number can however be null.
    handlers: [AtomicUsize; LEN],
}

/// An internal type for const-initialization of the handlers array.
/// Note that the raw array has the same layout as the atomic one,
/// because usize and AtomicUsize have the same layout, and therefore
/// the array has the same layout.
union TrapHandlersInit<const LEN: usize> {
    raw: [usize; LEN],
    atomic: ManuallyDrop<[AtomicUsize; LEN]>,
}

impl<const LEN: usize> TrapHandlers<LEN> {

    /// Create a new trap handlers structure.
    pub const fn new() -> Self {
        unsafe { 
            Self {
                handlers: ManuallyDrop::into_inner(TrapHandlersInit { raw: [0; LEN] }.atomic),
            }
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
