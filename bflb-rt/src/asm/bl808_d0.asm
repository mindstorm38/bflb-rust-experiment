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
    csrci mstatus, MSTATUS_MIE | MSTATUS_SIE
 
    # Initialize floating point unit.
    li t0, MSTATUS_FS
    csrc mstatus, t0
    li t0, MSTATUS_FS_INITIAL
    csrs mstatus, t0

    # mapbaddr (APB bus base addr)
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

    # Initialize the trap-vector base address.
    # Use "direct" mode.
    la t0, _mtrap_generic_handler
    ori t0, t0, MTVEC_DIRECT
    csrw mtvec, t0

    # Initialize stack pointer.
    la sp, _ld_stack_end

    # The first function will copy runtime variables to RAM.
    # This will also copy .ramtext sections that contains the 
    # interrupt handler, this is mostly why interrupts are disable
    # while manipulating this.
    jal _rust_mem_init
    
    # Init before entry point.
    jal _rust_init

    # Re-enable interrupts after startup.
    csrsi mstatus, MSTATUS_MIE

    # Enter the entry function.
    jal _rust_entry


# Aligned to 4 bytes because of 'mtvec'.
.align 4
.global _mtrap_generic_handler
_mtrap_generic_handler:

gp_save:

    # Space for 29 qword registers (232 bytes).
    addi sp, sp, -232
    sd x1,    0(sp)   # ra
                      # sp/x2 is not saved, because we use it
                      # gp/x3 is not saved, because it is stable accross the program
    sd x4,    8(sp)   # tp
    sd x5,   16(sp)   # t0
    sd x6,   24(sp)   # t1
    sd x7,   32(sp)   # t2
    sd x8,   40(sp)   # s0
    sd x9,   48(sp)   # s1
    sd x10,  56(sp)   # a0
    sd x11,  64(sp)   # a1
    sd x12,  72(sp)   # a2
    sd x13,  80(sp)   # a3
    sd x14,  88(sp)   # a4
    sd x15,  96(sp)   # a5
    sd x16, 104(sp)   # a6
    sd x17, 112(sp)   # a7
    sd x18, 120(sp)   # s2
    sd x19, 128(sp)   # s3
    sd x20, 136(sp)   # s4
    sd x21, 144(sp)   # s5
    sd x22, 152(sp)   # s6
    sd x23, 160(sp)   # s7
    sd x24, 168(sp)   # s8
    sd x25, 176(sp)   # s9
    sd x26, 184(sp)   # s10
    sd x27, 192(sp)   # s11
    sd x28, 200(sp)   # t3
    sd x29, 208(sp)   # t4
    sd x30, 216(sp)   # t5
    sd x31, 224(sp)   # t6

    # Check status of the floating point unit.
    csrr t0, mstatus
    li t1, MSTATUS_FS
    and t0, t0, t1

    # If FS is not dirty, do not save status.
    li t1, MSTATUS_FS_DIRTY
    bne t0, t1, fp_no_save

fp_save:

    # Save floating point registers.
    addi sp, sp, -128
    fsw f0,    0(sp)
    fsw f1,    4(sp)
    fsw f2,    8(sp)
    fsw f3,   12(sp)
    fsw f4,   16(sp)
    fsw f5,   20(sp)
    fsw f6,   24(sp)
    fsw f7,   28(sp)
    fsw f8,   32(sp)
    fsw f9,   36(sp)
    fsw f10,  40(sp)
    fsw f11,  44(sp)
    fsw f12,  48(sp)
    fsw f13,  52(sp)
    fsw f14,  56(sp)
    fsw f15,  60(sp)
    fsw f16,  64(sp)
    fsw f17,  68(sp)
    fsw f18,  72(sp)
    fsw f19,  76(sp)
    fsw f20,  80(sp)
    fsw f21,  84(sp)
    fsw f22,  88(sp)
    fsw f23,  92(sp)
    fsw f24,  96(sp)
    fsw f25, 100(sp)
    fsw f26, 104(sp)
    fsw f27, 108(sp)
    fsw f28, 112(sp)
    fsw f29, 116(sp)
    fsw f30, 120(sp)
    fsw f31, 124(sp)

    # Add 'true' on top of the stack to indicate that floating point 
    # registers are saved.
    addi sp, sp, -4
    li t0, 1
    sd t0, 0(sp)

    # Set FS to clean
    li t0, MSTATUS_FS
    csrc mstatus, t0
    li t0, MSTATUS_FS_CLEAN
    csrs mstatus, t0

    j handler

fp_no_save:

    # Add 'false' on top of the stack to indicate that floating point 
    # registers are not saved.
    addi sp, sp, -4
    sd zero, 0(sp)

handler:

    # Call the trap handler in Rust code.
    csrr a0, mcause

    # Intentionnaly use a register because we are unsure about how far
    # this function can be placed.
    la t0, _rust_mtrap_handler
    jalr t0

fp_restore:

    # Read the boolean indicating if floating point register were saved.
    ld t0, 0(sp)
    addi sp, sp, 4
    beq t0, zero, gp_restore

    flw f0,    0(sp)
    flw f1,    4(sp)
    flw f2,    8(sp)
    flw f3,   12(sp)
    flw f4,   16(sp)
    flw f5,   20(sp)
    flw f6,   24(sp)
    flw f7,   28(sp)
    flw f8,   32(sp)
    flw f9,   36(sp)
    flw f10,  40(sp)
    flw f11,  44(sp)
    flw f12,  48(sp)
    flw f13,  52(sp)
    flw f14,  56(sp)
    flw f15,  60(sp)
    flw f16,  64(sp)
    flw f17,  68(sp)
    flw f18,  72(sp)
    flw f19,  76(sp)
    flw f20,  80(sp)
    flw f21,  84(sp)
    flw f22,  88(sp)
    flw f23,  92(sp)
    flw f24,  96(sp)
    flw f25, 100(sp)
    flw f26, 104(sp)
    flw f27, 108(sp)
    flw f28, 112(sp)
    flw f29, 116(sp)
    flw f30, 120(sp)
    flw f31, 124(sp)
    addi sp, sp, 128

gp_restore:

    ld x1,    0(sp)
    ld x4,    8(sp)
    ld x5,   16(sp)
    ld x6,   24(sp)
    ld x7,   32(sp)
    ld x8,   40(sp)
    ld x9,   48(sp)
    ld x10,  56(sp)
    ld x11,  64(sp)
    ld x12,  72(sp)
    ld x13,  80(sp)
    ld x14,  88(sp)
    ld x15,  96(sp)
    ld x16, 104(sp)
    ld x17, 112(sp)
    ld x18, 120(sp)
    ld x19, 128(sp)
    ld x20, 136(sp)
    ld x21, 144(sp)
    ld x22, 152(sp)
    ld x23, 160(sp)
    ld x24, 168(sp)
    ld x25, 176(sp)
    ld x26, 184(sp)
    ld x27, 192(sp)
    ld x28, 200(sp)
    ld x29, 208(sp)
    ld x30, 216(sp)
    ld x31, 224(sp)
    addi sp, sp, 232

    mret
