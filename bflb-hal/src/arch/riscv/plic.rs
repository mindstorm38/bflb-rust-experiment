//! # Platform-Level Interrupt Controller (standard extension).
//! 
//! Sources:
//! - https://raw.githubusercontent.com/riscv/riscv-plic-spec/master/riscv-plic-1.0.0_rc6.pdf

use embedded_util::PtrRw;


embedded_util::mmio! {
    
    /// Platform-Level Interrupt Controller memory registers.
    pub struct Plic {}
    
}

impl Plic {

    /// Custom function to get a RW pointer to the priority
    /// register for the given interrupt source number.
    #[must_use]
    #[inline(always)]
    pub const fn priority(self, source: usize) -> PtrRw<u32> {
        debug_assert!(source != 0 && source <= 1023, "invalid interrupt source");
        unsafe { PtrRw(self.0.add(0x0000 + source * 4) as _) }
    }

    /// Custom function to get a RW pointer to the pending
    /// bits register for the given interrupt source number.
    /// 
    /// *Note that* this register contains 32 pending bits, 
    /// so just take the modulo of the given source number
    /// to know which bit to modify.
    #[must_use]
    #[inline(always)]
    pub const fn pending(self, source: usize) -> PtrRw<u32> {
        debug_assert!(source != 0 && source <= 1023, "invalid interrupt source");
        unsafe { PtrRw(self.0.add(0x1000 + (source / 32 * 4)) as _) }
    }

    /// Custom function to get a RW pointer to the enable
    /// bits register for the given interrupt source number.
    /// 
    /// *Note that* this register contains 32 enable bits, 
    /// so just take the modulo of the given source number
    /// to know which bit to modify.
    #[must_use]
    #[inline(always)]
    pub const fn enable(self, context: usize, source: usize) -> PtrRw<u32> {
        debug_assert!(context <= 15871, "invalid context");
        debug_assert!(source != 0 && source <= 1023, "invalid interrupt source");
        unsafe { PtrRw(self.0.add(0x2000 + (context * 0x80) + (source / 32 * 4)) as _) }
    }

    /// Custom function to get a RW pointer to the priority
    /// threshold of a given context.
    #[must_use]
    #[inline(always)]
    pub const fn priority_threshold(self, context: usize) -> PtrRw<u32> {
        debug_assert!(context <= 15871, "invalid context");
        unsafe { PtrRw(self.0.add(0x200000 + context * 0x1000) as _) }
    }

    /// Custom function to get a RW pointer to the claim/complete
    /// register of a given context.
    #[must_use]
    #[inline(always)]
    pub const fn claim_complete(self, context: usize) -> PtrRw<u32> {
        debug_assert!(context <= 15871, "invalid context");
        unsafe { PtrRw(self.0.add(0x200004 + context * 0x1000) as _) }
    }

}
