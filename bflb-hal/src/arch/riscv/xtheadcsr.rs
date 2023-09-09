//! T-Head extension CSR.
//! 
//! Credit: This file is imported from https://github.com/rustsbi/xuantie
//! No official documentation mentions these...


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

super::impl_csr_rw!(Mhcr, 0x7C1);
