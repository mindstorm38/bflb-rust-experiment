//! Module for BL808 D0 core runtime.

core::arch::global_asm!(include_str!("asm/common.asm"));
core::arch::global_asm!(include_str!("asm/bl808_d0.asm"));
core::arch::global_asm!(include_str!("asm/rv64imaf_trap.asm"));


pub const HART_COUNT: usize = 1;


/// Chip-specific function to init system before entry point.
pub fn init() {

}
