//! # Core-Local Interrupt Controller.
//! 
//! When CLIC mode is enabled:
//! - CSR *x*tvec has new field definitions:
//!   - \[0..2] mode = 11 to enable CLIC mode
//!   - \[2..6] submode = 0000
//!   - \[6..] base = <aligned to 64 bytes, because 6 bits are forced to 0>
//! - CSRs *x*ie/*x*ip are hard-wired to zero and writes do nothing.
//! - CSR *x*cause has new field definitions:
//!   - \[00..12] exception/interrupt code
//!   - \[16..24] previous interrupt level
//!   - \[27..28] previous interrupt enable (same as mstatus.mpie)
//!   - \[28..30] previous privilege mode (same as mstatus.mpp)
//!   - \[30..31] set by hardware at start of hardware vectoring
//!   - \[31..32] interrupt = 1, exception = 0
//! 
//! Sources:
//! - https://raw.githubusercontent.com/riscv/riscv-fast-interrupt/master/clic.pdf


emhal::mmio_struct! {

    /// Core-Local Interrupt Controller memory registers.
    pub struct Clic {
        [0x0000] rw cfg: ClicCfg,
        [0x0004] r info: ClicInfo,
        [0x0008] rw int_th: u32,
    }

    /// Configuration of a single interrupt.
    pub struct ClicInt {
        /// Interrupt set pending (0 or 1).
        /// 
        /// This correspond to `clicintip[i]` in spec.
        [0x0] rw pending: u8,
        /// Interrupt set enable (0 or 1).
        /// 
        /// This correspond to `clicintie[i]` in spec.
        [0x1] rw enable: u8,
        /// Interrupt set attribute.
        /// 
        /// This correspond to `clicintattr[i]` in spec.
        [0x2] rw attr: ClicIntAttr,
        /// Interrupt control.
        /// 
        /// This correspond to `clicintctl[i]` in spec.
        [0x3] rw control: u8,
    }

}

impl Clic {

    /// Custom function to get a MMIO structure to the given 
    /// interrupt number.
    #[must_use]
    #[inline(always)]
    pub const fn int(self, n: usize) -> ClicInt {
        unsafe { ClicInt::new(self.0.add(0x1000 + n * 4)) }
    }

}

emhal::mmio_reg! {

    pub struct ClicCfg: u32 {
        /// Enable `shv` field in [`ClicIntAttr`].
        [0..1] nvbits,
        /// This field indicates how many upper bits in `clicintctl[i]` are assigned 
        /// to encode the interrupt level.
        [1..5] nlbits,
        /// Specifies how many bits are physically implemented in [`ClicIntAttr`].
        [5..7] nmbits,
    }

    pub struct ClicInfo: u32 {
        /// Actual number of interrupts supported by the implementation.
        [0..13] num_interrupt,
        [13..21] version,
        [21..25] control_bits,
        [25..31] num_trigger,
    }

    pub struct ClicIntAttr: u8 {
        /// Selective Hardware Vectoring.
        /// - When 0, non-vectored jump to *x*tvec.
        /// - When 1, vectored jump to trap-handler function in *x*tvt.
        /// 
        /// *Note that* this bit is only available if `nvbits` is set to 1
        /// in [`ClicCfg`].
        [0..1] shv,
        /// - When 0, level-triggered.
        /// - When 1, edge-triggered.
        [1..2] edge_triggered,
        /// - When 0, positive-edge.
        /// - When 1, negative-edge.
        [2..3] negative_edge,
        /// Privilege mode of this interrupt.
        /// 
        /// With nmbits = 0:
        /// - `xx`: M-mode interrupt
        /// 
        /// With nmbits = 1:
        /// - `0x`: U-mode interrupt
        /// - `1x`: M-mode interrupt
        /// 
        /// With nmbits = 2:
        /// - `00`: U-mode interrupt
        /// - `01`: S-mode interrupt
        /// - `10`: Reserved
        /// - `11`: M-mode interrupt
        /// 
        /// **For security purpose, the mode field can only be set to a privilege 
        /// level that is equal to or lower than the currently running privilege 
        /// level.**
        [6..8] mode,
    }

}
