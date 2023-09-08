//! T-Head Sync (Multi-core synchronization instructions).
//! 
//! Credit: This file is imported from https://github.com/rustsbi/xuantie
//! Official doc: https://github.com/T-head-Semi/thead-extension-spec/blob/master/xtheadsync.adoc

#![allow(unsafe_op_in_unsafe_fn)]

use core::arch::asm;


/// SYNC, Synchronize instruction
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
///
/// # Permissions
///
/// Can run on M, U mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, E907 and E906 cores.
#[inline]
pub unsafe fn sync() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x018")
}

/// SYNC.S, Synchronize and broadcast instruction
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
/// This request will be broadcast to all other harts.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 core.
#[inline]
pub unsafe fn sync_s() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x019")
}

/// SYNC.I, Synchronize and clean instruction
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
/// The pipeline is emptied when this instruction retires.
///
/// # Permissions
///
/// Can run on M, U mode, or S mode if applicable.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910, C906, E907 and E906 cores.
#[inline]
pub unsafe fn sync_i() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x01A")
}

/// SYNC.IS, Synchronize, clean and broadcast instruction
///
/// Ensures that all instructions before retire earlier than this instruction,
/// and all instructions after retire later than this instruction.
/// The pipeline is emptied when this instruction retires.
/// This request will be broadcast to all other harts.
///
/// # Permissions
///
/// Can run on M, S or U mode.
///
/// # Exceptions
///
/// Raises illegal instruction exception when `mxstatus.theadisaee = 0`, or
/// when `mxstatus.theadisaee = 1` but run on U mode.
///
/// # Platform support
///
/// This instruction is supported on Xuantie C910 core.
#[inline]
pub unsafe fn sync_is() {
    asm!(".insn i 0x0B, 0, x0, x0, 0x01B")
}