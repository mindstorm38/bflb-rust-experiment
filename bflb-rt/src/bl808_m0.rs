//! Module for BL808 M0 core runtime.

core::arch::global_asm!(include_str!("asm/common.asm"));
core::arch::global_asm!(include_str!("asm/bl808_m0.asm"));
core::arch::global_asm!(include_str!("asm/rv32imaf_trap.asm"));


use bflb_hal::bl808::{GLB, CLIC};

use crate::clic::ClicVectorTable;
use crate::IRQ_COUNT;


/// Machine Trap Vector Table.
#[no_mangle]
#[link_section = ".text.vector"]
static mut _rust_mtrap_tvt: ClicVectorTable<IRQ_COUNT> = ClicVectorTable::new(crate::sym::_mtrap_generic_handler);


pub(crate) fn init() {

    // We use all bits for interrupt level, no priority bit.
    CLIC.cfg().modify(|reg| reg.nlbits().set(8));

    for irq_num in 0..IRQ_COUNT {
        let int = CLIC.int(irq_num);
        int.enable().set(0);
        int.pending().set(0);
        int.attr().modify(|reg| reg.vectored().clear());
        int.control().set(255);
    }

    // Disable UART sig swap for all pin groups.
    GLB.parm_cfg0().modify(|reg| reg.uart_swap_set().clear());

    // These registers are not properly initialized by default.
    GLB.uart_cfg1().set_with(|reg| reg.0 = 0xFFFFFFFF);
    GLB.uart_cfg2().set_with(|reg| reg.0 = 0x0000FFFF);

}
