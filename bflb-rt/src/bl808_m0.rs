//! Module for BL808 M0 core runtime.

#[cfg(not(target_arch = "riscv32"))]
compile_error!("bl808_m0 chip requires riscv32 target architecture");

core::arch::global_asm!(include_str!("asm/bl808_m0.asm"));

use riscv_hal::clic::{Clic, set_mintthresh, get_mintthresh};

use crate::clic::ClicVectorTable;
use crate::InterruptTrigger;
use crate::IrqNum;
use crate::IRQ_COUNT;


/// On the M0/LP core, we use the CLIC for interruption handling.
const CLIC: Clic = Clic(0xE0800000u32 as _);


/// Machine Trap Vector Table.
#[no_mangle]
#[link_section = ".text.vector"]
static mut _rust_mtrap_tvt: ClicVectorTable<IRQ_COUNT> = ClicVectorTable::new(crate::sym::_mtrap_generic_handler);


pub fn init() {

    // We use all bits for interrupt level, no priority bit.
    CLIC.cfg().modify(|reg| reg.nlbits().set(8));

    for irq_num in 0..IRQ_COUNT {
        let int = CLIC.int(irq_num);
        int.enable().set(0);
        int.pending().set(0);
        int.attr().modify(|reg| reg.vectored().clear());
        int.control().set(255);
    }

    

}

#[inline(always)]
pub fn is_interrupt_enabled(num: IrqNum) -> bool {
    CLIC.int(num as _).enable().get() != 0
}

#[inline(always)]
pub fn set_interrupt_enabled(num: IrqNum, enabled: bool) {
    CLIC.int(num as _).enable().set(enabled as _);
}

#[inline(always)]
pub fn is_interrupt_pending(num: IrqNum) -> bool {
    CLIC.int(num as _).pending().get() != 0
}

#[inline(always)]
pub fn set_interrupt_pending(num: IrqNum, pending: bool) {
    // NB: Look at Read-only or Read/Write in "pending" doc.
    CLIC.int(num as _).pending().set(pending as _);
}

#[inline(always)]
pub fn get_interrupt_level(num: IrqNum) -> u8 {
    CLIC.int(num as _).control().get()
}

#[inline(always)]
pub fn set_interrupt_level(num: IrqNum, level: u8) {
    // NB: Read doc of "control" to understand that no all level are valid bit patterns.
    CLIC.int(num as _).control().set(level);
}

#[inline(always)]
pub fn get_interrupt_trigger(num: IrqNum) -> InterruptTrigger {
    let mut tmp = CLIC.int(num as _).attr().get();
    match (tmp.edge_triggered().get(), tmp.negative_edge().get()) {
        (0, 0) => InterruptTrigger::PositiveLevel,
        (0, 1) => InterruptTrigger::NegativeLevel,
        (1, 0) => InterruptTrigger::PositiveEdge,
        (1, 1) => InterruptTrigger::NegativeEdge,
        // Unreachable and should be optimized-out because only these patterns
        // are valid because of the fields' range.
        _ => unreachable!()
    }
}

#[inline(always)]
pub fn set_interrupt_trigger(num: IrqNum, trigger: InterruptTrigger) {
    CLIC.int(num as _).attr().modify(|reg| {

        let (edge, neg) = match trigger {
            InterruptTrigger::PositiveLevel => (0, 0),
            InterruptTrigger::NegativeLevel => (0, 1),
            InterruptTrigger::PositiveEdge => (1, 0),
            InterruptTrigger::NegativeEdge => (1, 1),
        };

        reg.edge_triggered().set(edge);
        reg.negative_edge().set(neg);

    });
}

#[inline(always)]
pub fn get_interrupt_threshold() -> u8 {
    get_mintthresh()
}

#[inline(always)]
pub fn set_interrupt_threshold(level: u8) {
    set_mintthresh(level)
}
