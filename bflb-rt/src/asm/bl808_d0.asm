# Startup entry point for the os.

.attribute arch, "rv64imafcv"

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
    li t0, MSTATUS_MIE | MSTATUS_SIE
    csrc mstatus, t0

    # mapbaddr
    # Clear PLIC ?
    csrr t1, 0xFC1
    li   t2, 0x00200004
    add  t2, t2, t1
    lw   t3, 0(t2)
    sw   t3, 0(t2)
    li   t4, 0x00201004
    add  t2, t4, t1
    lw   t3, 0(t2)
    sw   t3, 0(t2)
 
    # Disable all interrupts and clear pending ones.
    csrw mie, zero
    csrw mip, zero

    # Reset cycles and retired instructions counters.
    csrw mcycle, zero
    csrw minstret, zero

    # Initialize floating point unit.
    li t0, MSTATUS_FS
    csrc mstatus, t0
    li t0, MSTATUS_FS_INITIAL
    csrs mstatus, t0

    # Initialize the trap-vector base address.
    # Use "direct" mode.
    la t0, _mtrap_generic_handler
    ori t0, t0, MTVEC_DIRECT
    csrw mtvec, t0

    # Initialize stack pointer.
    la sp, _ld_stack_top

    # The first function will copy runtime variables to RAM.
    jal _rust_ram_load

    # Init before entry point.
    jal _rust_init

    # Before entering main, we re-enable interrupts.
    # We also enable machine timer/external interrupts.
    csrsi mstatus, MSTATUS_MIE
    li t0, (1 << 7) | (1 << 11)
    csrs mie, t0

    # Enter the entry function.
    jal _rust_entry

    # The processor ends here if the main function returns.
exit:
    wfi
    j exit
