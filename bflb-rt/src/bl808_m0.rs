//! Module for BL808 M0 core runtime.

core::arch::global_asm!(include_str!("asm/common.asm"));
core::arch::global_asm!(include_str!("asm/bl808_m0.asm"));

use bflb_hal::arch::riscv::clic::ClicVectorTable;
use bflb_hal::interrupt::IRQ_COUNT;

/// Machine Trap Vector Table.
#[no_mangle]
#[link_section = ".text.vector"]
static mut _rust_mtrap_tvt: ClicVectorTable<IRQ_COUNT> = ClicVectorTable::new(crate::sym::_mtrap_generic_handler);
