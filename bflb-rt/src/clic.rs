//! Common internal code for CLIC-based chips.


/// Low-level Trap Vector Handler referenced by the Trap Vector Table.
/// 
/// This function type is intentionnaly never returning and unsafe
/// because it will return from the function using *x*ret instruction.
pub type ClicVectorHandler = unsafe extern "C" fn() -> !;


/// Internal structure used to align the trap vector table.
#[repr(C, align(64))]
pub struct ClicVectorTable<const LEN: usize>([ClicVectorHandler; LEN]);

impl<const LEN: usize> ClicVectorTable<LEN> {
    pub const fn new(default: ClicVectorHandler) -> Self {
        Self([default; LEN])
    }
}
