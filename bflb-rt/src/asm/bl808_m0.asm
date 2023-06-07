# Startup entry point for the os.

.attribute arch, "rv32imafc"
.option norvc

.section .text.init

# The startup function, the first called on boot.
.align 2
.global _start
_start:

    # Disable linker instruction relaxation for the `la` instruction below.
    # This disallows the assembler from assuming that `gp` is already initialized.
    # This causes the value stored in `gp` to be calculated from `pc`.
    # The job of the global pointer is to give the linker the ability to address
    # memory relative to GP instead of as an absolute address.
.option push
.option norelax
    la gp, _ld_global_pointer
.option pop

    # Disable interruptions for startup.
    csrci mstatus, MSTATUS_MIE | MSTATUS_SIE
 
    # Initialize floating point unit.
    li t0, MSTATUS_FS
    csrc mstatus, t0
    li t0, MSTATUS_FS_INITIAL
    csrs mstatus, t0

    # Initialize the trap-vector base address.
    # Use CLIC mode.
    la t0, _mtrap_generic_handler
    ori t0, t0, MTVEC_CLIC
    csrw mtvec, t0

    # Intentionnaly not using mtvt because we'll
    # only allow unvectored interrupts.
    la t0, _rust_mtrap_tvt
    csrw 0x307, t0

    j _start_common
