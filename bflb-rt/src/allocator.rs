//! Allocator implementation specific to the runtime, using a 

use core::ptr::NonNull;
use core::sync::atomic::{Ordering, AtomicBool};
use core::cell::UnsafeCell;

use alloc::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::Heap;


/// Custom runtime allocator, that uses a simple interrupt-free spin locking in order to
/// be used in an interrupt context.
pub struct RuntimeAllocator {
    /// An atomic boolean indicating if currently locked.
    lock: AtomicBool,
    /// The allocator, should only be mutated if the current hart has acquired the lock.
    allocator: UnsafeCell<Heap>,
}

/// Our implementation is hart safe.
unsafe impl Sync for RuntimeAllocator {}

impl RuntimeAllocator {

    pub const fn empty() -> Self {
        Self {
            lock: AtomicBool::new(false),
            allocator: UnsafeCell::new(Heap::empty()),
        }
    }

    /// Internal classic spin lock implementation.
    #[inline(always)]
    pub fn with<'a, R, F: FnOnce(&'a mut Heap) -> R>(&'a self, func: F) -> R {

        // Need to execute within critical section to avoid cases where interrupt-enabled
        // context would need to allocate and an interrupt happen before finishing, and
        // then the interrupt context tries to allocate but indefinitely wait...
        critical_section::with(|_| {
            
            while self.lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
                // Intentionally no spin loop hint because it could hang waiting for interrupt.
            }
    
            let ret = {
                // SAFETY: We ensured that we are the only one to access it.
                let allocator = unsafe { &mut *self.allocator.get() };
                func(allocator)
            };

            self.lock.store(false, Ordering::Release);
    
            ret

        })

    }

}

unsafe impl GlobalAlloc for RuntimeAllocator {

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.with(|heap| {
            match heap.allocate_first_fit(layout) {
                Ok(allocation) => allocation.as_ptr(),
                Err(()) => core::ptr::null_mut()
            }
        })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {

        // SAFETY: Our 'alloc' function returns non-null pointers.
        let ptr = unsafe { NonNull::new_unchecked(ptr) };

        self.with(|heap| {
            // SAFETY: The pointer has been allocated with our 'alloc' function.
            unsafe { heap.deallocate(ptr, layout); }
        });

    }

}
