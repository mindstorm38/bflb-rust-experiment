//! Utility macro for defining peripherals structures with mutex-like logic
//! for protecting concurrent usage of peripherals.


/// Macro for automatic implementation of peripheral functions: `try_take`, `take`,
/// and `free`. These functions uses `unsafe fn new() -> Self` and 
/// `unsafe fn taken() -> &'static AtomicBool`.
/// 
/// This macro is designed for peripheral access structures, such structure cannot
/// be copied/cloned and therefore can only be obtained safely via these functions.
/// 
/// This macro can be used in 3 forms:
/// - `peripheral!()`, with no arguments, only base functions `try_take`, `take` and
///   `free` are implemented.
/// - `peripheral!(single)`, in single mode, the `taken` function is implemented
///   to return a single boolean, but `new` function is missing.
/// - `peripheral!(simple)`, in simple mode, the `new` function returns `Self(())` 
///   and taken return the same variable. The structure must be a tuple struct with
///   one private field of type `()`. This ensures that it can only be constructed
///   externally using the unsafe `new` function.
/// - `peripheral!(array: <var>[<start>..<end>])`, array mode has the same `new` 
///   function as the simple mode, however, the `taken` function returns a different
///   atomic variable depending on the given const generic `<var>` in the given range.
#[macro_export]
macro_rules! peripheral {
    () => {

        /// Try taking ownership of the peripheral, returning `Some` peripheral
        /// is not already taken.
        pub fn try_take() -> Option<Self> {
            use core::sync::atomic::Ordering;
            unsafe { Self::taken() }
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
                .then_some(unsafe { Self::new() })
        }
    
        /// Try taking ownership of the peripheral, this function panics if not
        /// possible.
        pub fn take() -> Self {
            Self::try_take().expect("peripheral was already taken")
        }
    
        /// Free ownership of the peripheral.
        pub fn free(self) {
            drop(self);
            unsafe { Self::taken() }.store(false, core::sync::atomic::Ordering::Release);
        }

    };
    (single) => {

        /// Returns the atomic variable that is used by `try_take` and `take` functions
        /// in order to ensures at runtime that no concurrent ownership of the peripheral
        /// happen.
        /// 
        /// SAFETY: You must ensure that modifying this variable will not lead to multiple
        /// instances of the peripheral to exists at the same time.
        #[inline]
        pub unsafe fn taken() -> &'static core::sync::atomic::AtomicBool {
            static TAKEN: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
            &TAKEN
        }
        
        $crate::peripheral!();

    };
    // For simple peripherals.
    (simple) => {

        /// Return a new instance of the peripheral.
        /// 
        /// SAFETY: You must ensure that no multiple instances of the peripheral to exists 
        /// at the same time.
        #[inline]
        pub unsafe fn new() -> Self {
            Self(())
        }

        $crate::peripheral!(single);

    };
    (array: $var:ident [ $start:literal .. $stop:literal ]) => {
        $crate::peripheral!(array: $var [ ($start) .. ($stop) ]);
    };
    (array: $var:ident [ ($start:expr) .. $stop:literal ]) => {
        $crate::peripheral!(array: $var [ ($start) .. ($stop) ]);
    };
    (array: $var:ident [ $start:literal .. ($stop:expr) ]) => {
        $crate::peripheral!(array: $var [ ($start) .. ($stop) ]);
    };
    // For array peripherals.
    (array: $var:ident [ ($start:expr) .. ($stop:expr) ]) => {

        /// Returns the atomic variable that is used by `try_take` and `take` functions
        /// in order to ensures at runtime that no concurrent ownership of the peripheral
        /// happen.
        /// 
        /// SAFETY: You must ensure that modifying this variable will not lead to multiple
        /// instances of the peripheral to exists at the same time.
        pub unsafe fn taken() -> &'static core::sync::atomic::AtomicBool {
            debug_assert!($var >= $start && $var < $stop, "invalid peripheral port {}", $var);
            const LEN: usize = $stop - $start;
            const TAKEN_DEFAULT: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
            static TAKEN_ARR: [core::sync::atomic::AtomicBool; LEN] = [TAKEN_DEFAULT; LEN];
            &TAKEN_ARR[$var as usize - $start]
        }
 
        /// Return a new instance of the peripheral.
        /// 
        /// SAFETY: You must ensure that no multiple instances of the peripheral to exists 
        /// at the same time.
        #[inline]
        pub unsafe fn new() -> Self {
            Self(())
        }
        
        $crate::peripheral!();

    };
}


// /// Macro for easy implementation of the trait [`Peripheral`]. This trait
// /// provides an atomically checked singleton state for the peripheral struct.
// /// 
// /// Example:
// /// ```
// /// use embedded_hal::peripheral;
// /// 
// /// peripheral! {
// ///     MY_PERIPHERAL0: MyPeripheral<0>;
// ///     MY_PERIPHERAL1: MyPeripheral<1>;
// /// }
// /// ```
// /// 
// /// This allows using the peripheral structure to be used as follow:
// /// ```
// /// use embedded_hal::{Peripheral, PeripheralGuard};
// /// 
// /// fn example() {
// /// 
// ///     let periph: MyPeripheral<0> = MyPeripheral::take();
// ///     // ...
// ///     MyPeripheral::free(periph);
// /// 
// ///     let periph_guard: PeripheralGuard<MyPeripheral<1>> = MyPeripheral::borrow();
// ///     // ...
// ///     // automatically freed
// /// 
// /// }
// /// ```
// #[macro_export]
// macro_rules! peripheral {
//     // For simple peripherals.
//     ($type:ty) => {

//         impl $crate::Peripheral for $type {
            
//             #[inline]
//             unsafe fn taken() -> &'static core::sync::atomic::AtomicBool {
//                 static TAKEN: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
//                 &TAKEN
//             }

//             #[inline]
//             unsafe fn new() -> Self {
//                 Self(())
//             }

//         }

//     };
//     // For array peripherals.
//     ($type:ty, $var:ident: $var_type:ty [ $start:literal .. $stop:literal ]) => {

//         impl<const $var: $var_type> $crate::Peripheral for $type {

//             unsafe fn taken() -> &'static core::sync::atomic::AtomicBool {
//                 debug_assert!($var >= $start && $var < $stop, "invalid peripheral port {}", $var);
//                 const LEN: usize = $stop - $start;
//                 const TAKEN_DEFAULT: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
//                 static TAKEN_ARR: [core::sync::atomic::AtomicBool; LEN] = [TAKEN_DEFAULT; LEN];
//                 &TAKEN_ARR[$var as usize - $start]
//             }

//             #[inline]
//             unsafe fn new() -> Self {
//                 Self(())
//             }

//         }

//     }
// }


// /// A trait for peripheral checked access.
// pub trait Peripheral: Sized {

//     /// This function returns the unique static reference to the atomic boolean
//     /// variable indicating if this peripheral is available (false) or taken (true).
//     /// 
//     /// This function is to be used internally and if marked unsafe because modifying
//     /// it outside of this trait's functions might result in invalid states.
//     unsafe fn taken() -> &'static AtomicBool;

//     /// Create an instance of this peripheral out of nowhere, this is unsafe because
//     /// you have to ensure that at most **one** instance of this peripheral exists
//     /// for its entire lifetime.
//     /// 
//     /// It's much better to use [`take`] or [`borrow`] to ensure that the peripheral
//     /// isn't owned multiple times.
//     unsafe fn new() -> Self;

//     /// Take the ownership of this peripheral.
//     /// 
//     /// This function is thread-safe and atomically checks for the availability of
//     /// this peripheral, a bit like a mutex but here you get complete ownership of
//     /// the instance. These instances are usually near zero-size but cannot be 
//     /// copied.
//     /// 
//     /// Use [`free`] to manually free the peripheral when you finished using it.
//     /// 
//     /// Use [`borrow`] 
//     fn take() -> Self {
//         unsafe { 
//             Self::taken().compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
//                 .expect("peripheral is already owned and cannot be borrowed");
//             Self::new() 
//         }
//     }

//     /// Free a peripheral that was previsouly borrowed by [`take`] or leaked from
//     /// its guard.
//     fn free(peripheral: Self) {
//         // We just drop the peripheral. It might seem that the signature
//         // of this structure is pointless but it actually forces the owner
//         // to give back ownership. This *obviously* requires that only one
//         // instance of the structure exists.
//         drop(peripheral);
//         unsafe {
//             Self::taken().store(false, Ordering::Release);
//         }
//     }

//     /// Borrom the ownership of this peripheral. Read the documentation of [`take`].
//     /// The term borrow hasn't the same meaning has in the language itself, because
//     /// here you get full ownership of the peripheral, but when the its lifetime 
//     /// ends, it is automatically freed.
//     fn borrow() -> PeripheralGuard<Self> {
//         PeripheralGuard(Self::take())
//     }

// }


// /// A lifetime-guarded wrapper for a particular peripheral type. When this guard
// /// reached end of scope, the peripheral is automatically freed.
// pub struct PeripheralGuard<P: Peripheral>(P);

// impl<P: Peripheral> Drop for PeripheralGuard<P> {
//     fn drop(&mut self) {
//         unsafe {
//             P::taken().store(false, Ordering::Release);
//         }
//     }
// }

// impl<P: Peripheral> Deref for PeripheralGuard<P> {
//     type Target = P;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<P: Peripheral> DerefMut for PeripheralGuard<P> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
