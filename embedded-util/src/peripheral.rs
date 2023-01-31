//! Utility macro for defining peripherals structures with mutex-like logic
//! for protecting concurrent usage of peripherals.

use core::ops::{Deref, DerefMut};


/// Macro for easy implementation of the trait [`Peripheral`]. This trait
/// provides an atomically checked singleton state for the peripheral struct.
/// 
/// Example:
/// ```
/// use embedded_hal::peripheral;
/// 
/// peripheral! {
///     MY_PERIPHERAL0: MyPeripheral<0>;
///     MY_PERIPHERAL1: MyPeripheral<1>;
/// }
/// ```
/// 
/// This allows using the peripheral structure to be used as follow:
/// ```
/// use embedded_hal::{Peripheral, PeripheralGuard};
/// 
/// fn example() {
/// 
///     let periph: MyPeripheral<0> = MyPeripheral::take();
///     // ...
///     MyPeripheral::free(periph);
/// 
///     let periph_guard: PeripheralGuard<MyPeripheral<1>> = MyPeripheral::borrow();
///     // ...
///     // automatically freed
/// 
/// }
/// ```
#[macro_export]
macro_rules! peripheral {
    // For simple peripherals.
    ($type:ty) => {

        impl $crate::Peripheral for $type {
            
            #[inline]
            unsafe fn taken() -> &'static core::sync::atomic::AtomicBool {
                static TAKEN: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
                &TAKEN
            }

            #[inline]
            unsafe fn new() -> Self {
                Self(())
            }

        }

    };
    // For array peripherals.
    ($type:ty, $var:ident: $var_type:ty [ $start:literal .. $stop:literal ]) => {

        impl<const $var: $var_type> $crate::Peripheral for $type {

            unsafe fn taken() -> &'static core::sync::atomic::AtomicBool {
                debug_assert!($var >= $start && $var < $stop, "invalid peripheral port {}", $var);
                const LEN: usize = $stop - $start;
                const TAKEN_DEFAULT: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
                static TAKEN_ARR: [core::sync::atomic::AtomicBool; LEN] = [TAKEN_DEFAULT; LEN];
                &TAKEN_ARR[$var as usize - $start]
            }

            #[inline]
            unsafe fn new() -> Self {
                Self(())
            }

        }

    }
}


pub trait Peripheral: Sized {

    /// This function returns the unique static reference to the atomic boolean
    /// variable indicating if this peripheral is available (false) or taken (true).
    /// 
    /// This function is to be used internally and if marked unsafe because modifying
    /// it outside of this trait's functions might result in invalid states.
    unsafe fn taken() -> &'static core::sync::atomic::AtomicBool;

    /// Create an instance of this peripheral out of nowhere, this is unsafe because
    /// you have to ensure that at most **one** instance of this peripheral exists
    /// for its entire lifetime.
    /// 
    /// It's much better to use [`take`] or [`borrow`] to ensure that the peripheral
    /// isn't owned multiple times.
    unsafe fn new() -> Self;

    /// Take the ownership of this peripheral.
    /// 
    /// This function is thread-safe and atomically checks for the availability of
    /// this peripheral, a bit like a mutex but here you get complete ownership of
    /// the instance. These instances are usually near zero-size but cannot be 
    /// copied.
    /// 
    /// Use [`free`] to manually free the peripheral when you finished using it.
    /// 
    /// Use [`borrow`] 
    fn take() -> Self {
        unsafe { 
            Self::taken().compare_exchange(false, true, 
                core::sync::atomic::Ordering::Acquire, 
                core::sync::atomic::Ordering::Relaxed
            ).expect("peripheral is already owned and cannot be borrowed");
            Self::new() 
        }
    }

    /// Borrom the ownership of this peripheral. Read the documentation of [`take`].
    /// The term borrow hasn't the same meaning has in the language itself, because
    /// here you get full ownership of the peripheral, but when the its lifetime 
    /// ends, it is automatically freed.
    fn borrow() -> PeripheralGuard<Self> {
        PeripheralGuard(Self::take())
    }

    /// Free a peripheral that was previsouly borrowed by [`take`] or leaked from
    /// its guard.
    fn free(peripheral: Self) {
        // We just drop the peripheral. It might seem that the signature
        // of this structure is pointless but it actually forces the owner
        // to give back ownership. This *obviously* requires that only one
        // instance of the structure exists.
        let _ = peripheral;
        unsafe {
            Self::taken().store(false, core::sync::atomic::Ordering::Release);
        }
    }

}


/// A lifetime-guarded wrapper for a particular peripheral type. When this guard
/// reached end of scope, the peripheral is automatically freed.
pub struct PeripheralGuard<P: Peripheral>(P);

impl<P: Peripheral> Drop for PeripheralGuard<P> {
    fn drop(&mut self) {
        unsafe {
            P::taken().store(false, core::sync::atomic::Ordering::Release);
        }
    }
}

impl<P: Peripheral> Deref for PeripheralGuard<P> {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Peripheral> DerefMut for PeripheralGuard<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
