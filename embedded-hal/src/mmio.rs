//! Utility for defining memory mapped structures and registers.


/// This macro can be used to generate MMIO structures. These structures are
/// special because they are actually a single pointer in size, but provides
/// many methods to access in a volatile way the internal registers that are
/// defined with the macro.
/// 
/// Note that field's type is expected to be copy, but is not checked by 
/// this macro. Read the documentation of [`core::ptr::read_volatile`] for
/// more information about the constraints applied to the type.
/// 
/// Example:
/// ```
/// mmio_struct! {
/// 
///     pub struct MmioPeripheral {
///         [0x00] r version: u32,          // A simple field at 0x00
///         [0x04] r state: u32,            // Another field at 0x04
///         [0x04] w set_state: u32,        // Write access to field at 0x04
///         [0x04] r state_ext: u64,        // Read access to field at 0x04 but as an u64
///         [0x08] s sub0: MmioSub,         // You can also define sub structures
///         [0x0A] s sub1: MmioSub,         // ..multiple time.
///         [0x0C] r bits: MyReg,
///     }
/// 
///     pub struct MmioSub {
///         [0x00] r a: bool,
///         [0x01] r b: bool,
///     }
/// 
/// }
/// 
/// mmio_reg! {
///     pub struct MyReg: u32 { ... }
/// }
/// ```
#[macro_export]
macro_rules! mmio_struct {
    (

        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                [$field_index:literal] $field_mode:ident $field_name:ident : $field_type:ty
            ),*
            $(,)?
        }

        $($t:tt)*

    ) => {

        $(#[$struct_meta])*
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        $vis struct $name(pub *mut u8);
        impl $name {

            #[inline]
            pub const fn new(ptr: *mut u8) -> Self {
                Self(ptr)
            }

            $(
                $crate::__mmio_struct_field!(
                    $field_mode, 
                    $field_index, 
                    $field_name, 
                    $field_type,
                    $($field_meta),*
                );
            )*

        }

        $crate::mmio_struct! {
            $($t)*
        }

    };
    () => {};
}

#[macro_export]
macro_rules! __mmio_struct_field {
    (rw, $field_index:literal, $field_name:ident, $field_type:ty, $($field_meta:meta),*) => {
        $(#[$field_meta])*
        #[must_use]
        #[inline(always)]
        pub fn $field_name(self) -> $crate::mmio::PtrRw<$field_type> {
            unsafe { $crate::mmio::PtrRw(self.0.add($field_index) as _) }
        }
    };
    (r, $field_index:literal, $field_name:ident, $field_type:ty, $($field_meta:meta),*) => {
        $(#[$field_meta])*
        #[must_use]
        #[inline(always)]
        pub fn $field_name(self) -> $crate::mmio::PtrR<$field_type> {
            unsafe { $crate::mmio::PtrR(self.0.add($field_index) as _) }
        }
    };
    (w, $field_index:literal, $field_name:ident, $field_type:ty, $($field_meta:meta),*) => {
        $(#[$field_meta])*
        #[must_use]
        #[inline(always)]
        pub fn $field_name(self) -> $crate::mmio::PtrW<$field_type> {
            unsafe { $crate::mmio::PtrW(self.0.add($field_index) as _) }
        }
    };
    (sub, $field_index:literal, $field_name:ident, $field_type:ty, $($field_meta:meta),*) => {
        $(#[$field_meta])*
        #[must_use]
        #[inline(always)]
        pub const fn $field_name(self) -> $field_type {
            unsafe { <$field_type>::new(self.0.add($field_index)) }
        }
    };
}


/// This macro can be used to define bit registers.
/// 
/// Example:
/// ```
/// mmio_reg! {
///     pub struct MyReg: u32 {
///         [0..10] field0: u16,
///         [10..11] field1: u8,
///     }
/// }
/// ```
#[macro_export]
macro_rules! mmio_reg {
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
                pub fn $field_name(&mut self) -> $crate::mmio::PtrField<'_, Self, $field_start, $field_end> {
                    $crate::mmio::PtrField(self)
                }
            )*

        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    $( 
                    .field(stringify!($field_name), &$crate::mmio::Reg::get::<$field_start, $field_end>(self)) 
                    )*
                    .finish()
            }
        }

        impl $crate::mmio::Reg for $name {

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

        $crate::mmio_reg! {
            $($t)*
        }

    };
    () => {};
}


/// A read-only pointer to some value in an MMIO struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtrR<T>(pub *const T);

impl<T> PtrR<T> {

    /// Get the value referenced by the pointer.
    #[inline(always)]
    pub fn get(self) -> T {
        unsafe { self.0.read_volatile() }
    }

}


/// A write-only pointer to some value in an MMIO struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtrW<T>(pub *mut T);

impl<T> PtrW<T> {

    /// Set the value referenced by the pointer.
    #[inline(always)]
    pub fn set(self, val: T) {
        unsafe { self.0.write_volatile(val) }
    }

}


/// A read/write pointer to some value in an MMIO struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtrRw<T>(pub *mut T);

impl<T> PtrRw<T> {

    /// Get the value referenced by the pointer.
    #[inline(always)]
    pub fn get(self) -> T {
        unsafe { self.0.read_volatile() }
    }

    /// Set the value referenced by the pointer.
    #[inline(always)]
    pub fn set(self, val: T) {
        unsafe { self.0.write_volatile(val) }
    }

    /// Set the value by modifying the default value using a function.
    #[inline(always)]
    pub fn set_with<F: FnOnce(&mut T)>(self, func: F)
    where
        T: Default
    {
        let mut val = T::default();
        func(&mut val);
        self.set(val);
    }

    /// Modify the value referenced by the pointer using a function.
    /// 
    /// You can use this together with registers in order to modify
    /// some bit fields in it and then apply changes. For example:
    /// ```
    /// let reg_ptr: PtrRw<MyReg>;
    /// reg_ptr.modify(|reg| reg.myfield().set(123));
    /// ```
    /// 
    /// *Synchronization is obviously not guarenteed and race 
    /// modifications can happen depending on your case.*
    #[inline(always)]
    pub fn modify<F: FnOnce(&mut T)>(self, func: F)
    where 
        Self: Copy 
    {
        let mut val = self.get();
        func(&mut val);
        self.set(val);
    }

}


/// A read/write "artificial" pointer to some register's field.
/// This pointer is artificial meaning that it is backed by the
/// register's pointer but holds `START` and `END` offsets to
/// understand where to get/set the value.
pub struct PtrField<'a, R: Reg, const START: u8, const END: u8>(pub &'a mut R);

impl<'a, R: Reg, const START: u8, const END: u8> PtrField<'a, R, START, END> {

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
