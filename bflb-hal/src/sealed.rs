//! Sealed trait for all modules.


/// This trait is public but in this crate-only module, so this trait cannot be 
/// implemented outside of this crate.
pub trait Sealed { }

impl Sealed for () {}
impl<const NUM: u8> Sealed for crate::gpio::Pin<NUM, crate::gpio::Alternate> {}
