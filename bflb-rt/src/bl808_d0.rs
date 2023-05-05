//! Module for BL808 D0 core runtime.
    
compile_error!("todo: rework bl808_d0 chip assembly");


// Include global startup assembly.
core::arch::global_asm!(include_str!("asm/bl808_d0.asm"));


/// Chip-specific function to init system before entry point.
pub(crate) fn init() {

}
