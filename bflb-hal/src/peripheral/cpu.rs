//! CPU interactions for BL808.

use crate::bl808::{GLB, MM_GLB};


/// A peripheral for controlling CPU.
pub struct CpuControl(pub(crate) ());

impl CpuControl {

    pub fn reset_m0(&mut self) {
        GLB.swrst_cfg2().modify(|reg| reg.ctrl_cpu_reset().fill());
    }

    pub fn reset_d0(&mut self) {
        MM_GLB.mm_sw_sys_reset().modify(|reg| reg.ctrl_mmcpu0_reset().fill());
    }

    pub fn reset_lp(&mut self) {
        GLB.swrst_cfg2().modify(|reg| reg.ctrl_pico_reset().fill());
    }

}
