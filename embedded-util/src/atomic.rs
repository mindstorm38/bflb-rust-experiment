use core::mem::{MaybeUninit, size_of, zeroed};
use core::sync::atomic::{AtomicU8, Ordering};
use core::cell::UnsafeCell;


/// The value is absent.
const ABSENT: u8 = 0;
/// The value is present.
const PRESENT: u8 = 1;
/// The value is being exclusively borrowed.
const BORROWED: u8 = 2;


/// An thread-safe and therefore atomic option type.
/// 
/// This type can be used for a static variable to insert and then take
/// an inner value.
/// 
/// *Note that* this type support ZST and is specifically optimized for
/// such types.
pub struct AtomicOption<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

// SAFETY: We atomically ensure that this type can be shared between threads.
unsafe impl<T> Sync for AtomicOption<T> {} 

impl<T> AtomicOption<T> {

    /// Create a new atomic option with some value.
    pub const fn some(value: T) -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::new(value)),
            state: AtomicU8::new(PRESENT),
        }
    }

    /// Create a new atomic option with no value.
    pub const fn none() -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(ABSENT),
        }
    }

    /// Inner method used to get mutable reference to the uninit wrapper.
    /// 
    /// SAFETY: Caller must ensure that not other mutable reference exits
    /// to the value. This is basically only possible when state is set to 
    /// `BORROWED`.
    #[inline(always)]
    unsafe fn value_mut(&self) -> &mut MaybeUninit<T> {
        &mut *self.value.get()
    }

    /// Take the value from the option, returning `Some` value if it was
    /// previously contained.
    pub fn take(&self) -> Option<T> {

        if size_of::<T>() == 0 {

            // If type is zero-sized, we can avoid reading of memory and the intermediate
            // BORROWED state, replacing it with a zeroed call that should be optimized.
            self.state.compare_exchange(PRESENT, ABSENT, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
                .then_some(unsafe { zeroed() })
            
        } else {

            if self.state.compare_exchange(PRESENT, BORROWED, Ordering::Acquire, Ordering::Relaxed).is_err() {
                return None;
            }

            // SAFETY: We acquire the BORROWED state, so it's safe to mutate value.
            let value = unsafe { self.value_mut().assume_init_read() };
            // Once the value is read, we consider it absent.
            self.state.store(ABSENT, Ordering::Release);

            Some(value)

        }

    }

    /// Insert a value in this option, returning `Ok(())` if the operation
    /// is successful. In case of unsuccessful inserting (insert race), the
    /// value is returned back as an error.
    /// 
    /// *Note that with ZST, this can never return an error because the value
    /// is never actually written so no insert race can happen.*
    pub fn insert(&self, value: T) -> Result<(), T> {

        if size_of::<T>() == 0 {

            // With zero-sized types with dont need to write the value, only
            // the atomic state is actually useful, therefore we can optimize
            // out the BORROWED state.
            match self.state.swap(PRESENT, Ordering::Acquire) {
                // The value was absent, in this case the 
                ABSENT => {}
                PRESENT => {
                    // The value was present, so we can call the drop 
                    // implementation of the type.
                    drop(unsafe { zeroed::<T>() })
                }
                // Other state is not possible with ZSTs. 
                _ => {}
            }
            
        } else {

            match self.state.swap(BORROWED, Ordering::Acquire) {
                // The value is currently being exclusively borrowed (either by
                // take or insert), therefore we can't touch it for now.
                BORROWED => return Err(value),
                PRESENT => {
                    // If the value was present, we simply drop the old value.
                    unsafe { self.value_mut().assume_init_drop(); }
                }
                // If the value was absent, there is nothing to do.
                _ => {}
            }

            // SAFETY: We acquire the BORROWED state, so it's safe to mutate value.
            unsafe { self.value_mut().write(value) };
            // The value is written so it's now considered present.
            self.state.store(PRESENT, Ordering::Release);

        }

        Ok(())

    }
    
}

#[cfg(test)]
mod tests {

    use super::*;

    static ZST_OPT: AtomicOption<()> = AtomicOption::none();
    static ST_OPT: AtomicOption<u8> = AtomicOption::none();

    #[test]
    fn test_zst() {
        
        assert!(ZST_OPT.take().is_none());
        assert!(ZST_OPT.insert(()).is_ok());
        assert_eq!(ZST_OPT.take(), Some(()));
        assert!(ZST_OPT.take().is_none());

    }

    #[test]
    fn test_st() {
        
        assert!(ST_OPT.take().is_none());
        assert!(ST_OPT.insert(95).is_ok());
        assert_eq!(ST_OPT.take(), Some(95));
        assert!(ST_OPT.take().is_none());

    }

}
