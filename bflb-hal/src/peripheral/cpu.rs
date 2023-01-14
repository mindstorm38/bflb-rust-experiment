//! CPU interactions for BL808.


use super::CpuId;


pub struct ChipCpu<'a> {
    chip: &'a super::Chip,
}

impl super::Chip {
    /// Access the clock management to the CPU management subsystem.
    #[inline(always)]
    pub fn cpu(&self) -> ChipCpu<'_> {
        ChipCpu { chip: self }
    }
}


/// Methods for managing individual CPUs.
impl ChipCpu<'_> {

    /// Halt the M0 CPU.
    pub fn halt_m0(&self) {
        self.chip.clock().disable_m0_clock();
        // TODO: DELAY 1 us
        self.chip.glb.swrst_cfg2().modify(|reg| reg.ctrl_cpu_reset().set(1));
    }

    /// Halt the D0 CPU.
    pub fn halt_d0(&self) {
        self.chip.clock().disable_d0_clock();
        // TODO: DELAY 1 us
        self.chip.mm_glb.mm_sw_sys_reset().modify(|reg| reg.ctrl_mmcpu0_reset().set(1));
    }

    /// Halt the LP CPU.
    pub fn halt_lp(&self) {
        self.chip.clock().disable_lp_clock();
        // TODO: DELAY 1 us
        self.chip.glb.swrst_cfg2().modify(|reg| reg.ctrl_pico_reset().set(1));
    }

    /// Halt the given CPU.
    pub fn halt_cpu(&self, id: CpuId) {
        match id {
            CpuId::M0 => self.halt_m0(),
            CpuId::D0 => self.halt_d0(),
            CpuId::LP => self.halt_lp(),
        }
    }

}
