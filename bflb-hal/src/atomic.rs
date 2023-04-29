//! Trying atomic data structures with interior mutability, usable for
//! storing asynchronous wakers.

use core::sync::atomic::{AtomicUsize, Ordering, AtomicU8, AtomicU32};
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

use core::task::Waker;



/// An atomic numeric ID allocator. It allows allocating and freeing
/// unique numeric IDs. This is backed by an array to store the freed
/// IDs.
pub struct AtomicIdAllocator<const LEN: usize> {
    /// This is a counter of of many times the `alloc` method was 
    /// called, if this is smaller than `LEN` generic parameter,
    /// when greater or equal it indicates that the IDs should be
    /// allocated from the internal queue.
    counter: AtomicUsize,
    /// The internal queue of freed IDs, it's only used once the
    /// count is greater or equal to `LEN`.
    queue: [AtomicUsize; LEN],
    /// The number of indices in the queue.
    queue_len: AtomicUsize,
}

impl<const LEN: usize> AtomicIdAllocator<LEN> {

    pub const fn new() -> Self {
        
        const INIT: AtomicUsize = AtomicUsize::new(0);

        Self {
            queue: [INIT; LEN],
            queue_len: AtomicUsize::new(0),
            counter: AtomicUsize::new(0),
        }

    }

    pub fn alloc(&self) -> Option<usize> {
        let id = self.counter.fetch_add(1, Ordering::AcqRel);
        if id < LEN {
            // The count is smaller than 'LEN', so we directly return
            // the index.
            Some(id)
        } else {
            // If the count is greater or equal to 'LEN', we allocate
            // the ID from the internal queue. We atomically subtract
            // the queue length.
            let prev_len = self.queue_len.fetch_sub(1, Ordering::SeqCst);
            let id = self.queue[prev_len - 1].load(Ordering::SeqCst);
            Some(id)
        }
    }

    pub fn free(&self, index: usize) {
        let prev_len = self.queue_len.fetch_add(1, Ordering::SeqCst);
        self.queue[prev_len].store(index, Ordering::SeqCst);
    }

}



const STATE_UNINIT: u8 = 0;
const STATE_INIT: u8 = 1;


/// A slab backed by an array of the given length. A slab acts like a
/// map indexed by [`usize`] and those indices are automatically
/// allocated by the slab.
pub struct AtomicSlab<T, const LEN: usize> {
    /// Inner data of the slab, stored inline.
    slots: [AtomicSlabSlot<T>; LEN],
    /// The maximum number of slots that have been used.
    len: AtomicUsize,
    /// This contains the next index where a slot is free. When this
    /// is equal to `len`, the length must be incremented just after
    /// insertion.
    next: AtomicUsize,
}

/// Represent a single slot 
struct AtomicSlabSlot<T> {
    /// The data the is written only on once when the slot is written,
    /// after that the stata is set INIT. The data is only invalidated
    /// when the cell is removed.
    value: UnsafeCell<MaybeUninit<T>>,
    /// The current length of the vector.
    state: AtomicU8,
}

impl<T, const LEN: usize> AtomicSlab<T, LEN> {

    /// Create a new slab, this is intended for static variable so 
    /// this can be called in const context.
    pub const fn new() -> Self {
        Self {
            slots: [AtomicSlabSlot::<T>::INIT; LEN],
            len: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    pub fn insert(&self) -> Option<usize> {

        todo!()

    }

}

impl<T> AtomicSlabSlot<T> {

    const INIT: Self = Self {
        value: UnsafeCell::new(MaybeUninit::uninit()),
        state: AtomicU8::new(0),
    };

}








/// A vector backed by a fixed maximum number of elements, and supports
/// interior mutability because the access to this vector is atomic.
/// 
/// This type of vector doesn't provide a way of mutating individual
/// items.
/// 
/// This type of vector is internally used for stored asynchronous
/// wakers for interruptions.
pub struct AtomicVec<T, const LEN: usize> {
    /// Inner data of the vector, stored inline.
    inner: [AtomicVecCell<T>; LEN],
    /// The current length of the vector.
    len: AtomicUsize,
}

unsafe impl<T, const LEN: usize> Sync for AtomicVec<T, LEN> {}

/// Internal cell type.
struct AtomicVecCell<T> {
    /// The data the is written only on once when the cell is written,
    /// after that the valid boolean is set to true. The data is only
    /// invalidated when the cell is removed.
    data: UnsafeCell<MaybeUninit<T>>,
    /// Set after the data can be safely written. This is used as a
    /// boolean after initialization.
    state: AtomicU8,
}

impl<T, const LEN: usize> AtomicVec<T, LEN> {

    #[inline]
    pub const fn new() -> Self {
        
        // SAFETY: It's safe to assume the array initialized because 
        // the cell structure AtomicVecCell should accept all bit
        // patterns to be valid: data is MaybeUninit and valid is 
        // an u8 integer, which is valid for all bit patterns.
        let mut inner: [AtomicVecCell<T>; LEN] = unsafe { MaybeUninit::zeroed().assume_init() };

        Self {
            inner,
            len: AtomicUsize::new(0),
        }

    }

    pub fn push(&self, item: T) {

        // SAFETY: After the length increment, the cell might be
        // accessed from other threads (like interrupt handler),
        // This is safe because the cell initially is in an
        // "uninit" state and its value should not be read, it
        // is considered absent.
        
        // Atomically increment the index and get the previous index
        // at which we should write the element to.
        let index = self.len.fetch_add(1, Ordering::SeqCst);
        let cell = &self.inner[index];

        // SAFETY: As stated in the top SAFETY notice, the cell might
        // be accessed from other threads, but we are the only thread
        // authorized to access data because of its "uninit" state.
        let data = unsafe { &mut *cell.data.get() };
        data.write(item);

        // Once the data has been stored, we can set the state to
        // initialized.
        cell.state.store(STATE_INIT, Ordering::SeqCst);

    }

}
