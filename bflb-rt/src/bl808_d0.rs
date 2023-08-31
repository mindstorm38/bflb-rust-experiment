//! Module for BL808 D0 core runtime.
    
core::arch::global_asm!(include_str!("asm/common.asm"));
core::arch::global_asm!(include_str!("asm/bl808_d0.asm"));


/// Chip-specific function to init system before entry point.
pub(crate) fn init() {

}
