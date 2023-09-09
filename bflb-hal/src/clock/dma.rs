//! DMA (Direct Memory Access) related clocks.

use crate::arch::bl808::GLB;


/// Enable clock gate for DMA controllers.
pub unsafe fn set_dma_enable(enable: bool) {
    GLB.cgen_cfg1().modify(|reg| reg.cgen_s1_dma().set(enable as _));
}
