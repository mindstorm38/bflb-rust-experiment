//! T-Head extension CSR.
//! 
//! Credit: This file is imported from https://github.com/rustsbi/xuantie
//! No official documentation mentions these...

#![allow(unsafe_op_in_unsafe_fn)]


embedded_util::reg! {

    pub struct Mhcr: usize {
        /// I-cache enable.
        ///
        /// # Platform support
        ///
        /// This bit is supported on Xuantie C910, C906, E907, E906 and E902 cores.
        [0..1] ie,
        /// D-cache enable.
        ///
        /// # Platform support
        ///
        /// This bit is supported on Xuantie C910, C906, E907 and E906 cores.
        [1..2] de,
        /// Cache write allocate configuration enable.
        ///
        /// # Platform support
        ///
        /// This bit is supported on Xuantie C910, C906, E907 and E906 cores.
        [2..3] wa,
        /// Write back enable; true for write back, false for write through
        ///
        /// # Platform support.
        ///
        /// This bit is supported on Xuantie C910, C906, E907 and E906 cores.
        [3..4] wb,
        /// Return stack enable.
        ///
        /// # Platform support
        ///
        /// This bit is supported on Xuantie C910, C906, E907 and E906 cores.
        [4..5] rs,
        /// Branch predict enable.
        ///
        /// # Platform support
        ///
        /// This bit is supported on Xuantie C910, C906, E907 and E906 cores.
        [5..6] bpe,
        /// Branch target buffer enable.
        ///
        /// # Platform support
        ///
        /// This bit is supported on Xuantie C910, C906, E907 and E906 cores.
        [6..7] btb,
        /// Write bulk transfer enable.
        [8..9] wbr,
    }

}


#[inline(always)]
pub unsafe fn set_mhcr(mhcr: Mhcr) {
    core::arch::asm!("csrw 0x7C1, {}", in(reg) mhcr.0)
}

#[inline(always)]
pub unsafe fn get_mhcr() -> Mhcr {
    let mut mhcr = Mhcr(0);
    core::arch::asm!("csrr {}, 0x7C1", out(reg) mhcr.0);
    mhcr
}

#[inline(always)]
pub unsafe fn modify_mhcr(func: impl FnOnce(&mut Mhcr)) {
    let mut mhcr = get_mhcr();
    func(&mut mhcr);
    set_mhcr(mhcr);
}
