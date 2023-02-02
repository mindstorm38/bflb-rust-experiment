//! Utility macro for defining pratical hardware registers.


/// This macro can be used to define bit registers.
/// 
/// Example:
/// ```
/// embedded_util::reg! {
///     pub struct MyReg: u32 {
///         [0..10] field0,
///         [10..11] field1,
///     }
/// }
/// ```
#[macro_export]
macro_rules! reg {
    (

        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident: $regtype:ty {
            $(
                $(#[$field_meta:meta])*
                [$field_start:literal .. $field_end:literal] $field_name:ident
            ),*
            $(,)?
        }

        $($t:tt)*

    ) => {

        $(#[$struct_meta])*
        #[derive(Clone, Copy, Eq, PartialEq, Default)]
        #[repr(transparent)]
        $vis struct $name(pub $regtype);
        impl $name {

            #[inline]
            pub const fn new(val: $regtype) -> Self {
                Self(val)
            }

            $(
                $(#[$field_meta])*
                #[must_use]
                #[inline]
                pub fn $field_name(&mut self) -> $crate::RegPtr<'_, Self, $field_start, $field_end> {
                    $crate::RegPtr(self)
                }
            )*

        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    $( 
                    .field(stringify!($field_name), &$crate::Reg::get::<$field_start, $field_end>(self)) 
                    )*
                    .finish()
            }
        }

        impl $crate::Reg for $name {

            type Type = $regtype;

            #[inline(always)]
            fn get<const START: u8, const END: u8>(&self) -> Self::Type {
                let mask = (1 << (END - START)) - 1;
                (self.0 >> START) & mask
            }

            #[inline(always)]
            fn set<const START: u8, const END: u8>(&mut self, val: Self::Type) {
                let mask = (1 << (END - START)) - 1;
                self.0 &= !(mask << START);
                self.0 |= (val & mask) << START;
            }

            #[inline(always)]
            fn fill<const START: u8, const END: u8>(&mut self) {
                let mask = (1 << (END - START)) - 1;
                self.0 |= mask << START;
            }
        
            #[inline(always)]
            fn clear<const START: u8, const END: u8>(&mut self) {
                let mask = (1 << (END - START)) - 1;
                self.0 &= !(mask << START);
            }

        }

        $crate::reg! {
            $($t)*
        }

    };
    () => {};
}


/// A read/write "artificial" pointer to some register's field.
/// This pointer is artificial meaning that it is backed by the
/// register's pointer but holds `START` and `END` offsets to
/// understand where to get/set the value.
pub struct RegPtr<'a, R: Reg, const START: u8, const END: u8>(pub &'a mut R);

impl<'a, R: Reg, const START: u8, const END: u8> RegPtr<'a, R, START, END> {

    /// Get the value of this register's field.
    #[inline(always)]
    pub fn get(self) -> R::Type {
        self.0.get::<START, END>()
    }

    /// Set the value of this register's field.
    #[inline(always)]
    pub fn set(self, val: R::Type) {
        self.0.set::<START, END>(val);
    }

    /// Set all bits to 1.
    #[inline(always)]
    pub fn fill(self) {
        self.0.fill::<START, END>();
    }

    /// Set all bits to 0.
    #[inline(always)]
    pub fn clear(self) {
        self.0.clear::<START, END>();
    }

}

/// Base trait implemented automatically by all defined registers.
/// The structure implementing this should be copy (because registers
/// are inherently integers).
pub trait Reg: Copy {

    type Type;

    /// Get the inner sub value in the given range.
    fn get<const START: u8, const END: u8>(&self) -> Self::Type;

    /// Set the inner sub value in the given range with the given value.
    fn set<const START: u8, const END: u8>(&mut self, val: Self::Type);

    /// Set all bits to 1.
    fn fill<const START: u8, const END: u8>(&mut self);

    /// Set all bits to 0.
    fn clear<const START: u8, const END: u8>(&mut self);

}
