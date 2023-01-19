//! Atomic utilities for const initialization.

use core::sync::atomic::AtomicBool;
use core::mem::ManuallyDrop;


/// This generic union type is used for atomic initialization, the two
/// generic types must have the same layout, like `bool` and [`AtomicBool`].
/// 
/// SAFETY: This trick is actually valid in a const context because both 
/// and AtomicBool has the same layout as stated in their doc/definition.
union AtomicInit<Raw: Copy, Atomic, const LEN: usize> {
    raw: [Raw; LEN],
    atomic: ManuallyDrop<[Atomic; LEN]>,
}


/// Create a const array of [`AtomicBool`] of a const generic length with 
/// given default value.
pub const fn atomic_bool_array<const LEN: usize>(default: bool) -> [AtomicBool; LEN] {
    ManuallyDrop::into_inner(unsafe { AtomicInit { raw: [default; LEN] }.atomic })
}
