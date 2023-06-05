# Generic RV32F machine trap handler.
# - 32 bit GP registers.
# - 32 bit FP registers.

.attribute arch, "rv32imaf"

# Aligned to 64 bytes because of 'mtvec' with CLIC mode.
.align 6
.section .text
.global _mtrap_generic_handler
_mtrap_generic_handler:

.gp_save:

    # Space for 29 qword registers (232 bytes).
    addi sp, sp, -232
    sw x1,    0(sp)   # ra
                      # sp/x2 is not saved, because we use it
                      # gp/x3 is not saved, because it is stable accross the program
    sw x4,    8(sp)   # tp
    sw x5,   16(sp)   # t0
    sw x6,   24(sp)   # t1
    sw x7,   32(sp)   # t2
    sw x8,   40(sp)   # s0
    sw x9,   48(sp)   # s1
    sw x10,  56(sp)   # a0
    sw x11,  64(sp)   # a1
    sw x12,  72(sp)   # a2
    sw x13,  80(sp)   # a3
    sw x14,  88(sp)   # a4
    sw x15,  96(sp)   # a5
    sw x16, 104(sp)   # a6
    sw x17, 112(sp)   # a7
    sw x18, 120(sp)   # s2
    sw x19, 128(sp)   # s3
    sw x20, 136(sp)   # s4
    sw x21, 144(sp)   # s5
    sw x22, 152(sp)   # s6
    sw x23, 160(sp)   # s7
    sw x24, 168(sp)   # s8
    sw x25, 176(sp)   # s9
    sw x26, 184(sp)   # s10
    sw x27, 192(sp)   # s11
    sw x28, 200(sp)   # t3
    sw x29, 208(sp)   # t4
    sw x30, 216(sp)   # t5
    sw x31, 224(sp)   # t6

    # Check status of the floating point unit.
    csrr t0, mstatus
    li t1, MSTATUS_FS
    and t0, t0, t1

    # If FS is not dirty, do not save status.
    li t1, MSTATUS_FS_DIRTY
    bne t0, t1, .fp_no_save

.fp_save:

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
    sw t0, 0(sp)

    # Set FS to clean
    li t0, MSTATUS_FS
    csrc mstatus, t0
    li t0, MSTATUS_FS_CLEAN
    csrs mstatus, t0

    j .handler

.fp_no_save:

    # Add 'false' on top of the stack to indicate that floating point 
    # registers are not saved.
    addi sp, sp, -4
    sw zero, 0(sp)

.handler:

    # Call the trap handler in Rust code.
    csrr a0, mcause

    # Intentionnaly use a register because we are unsure about how far
    # this function can be placed.
    la t0, _rust_mtrap_handler
    jalr t0

.fp_restore:

    # Read the boolean indicating if floating point register were saved.
    lw t0, 0(sp)
    addi sp, sp, 4
    beq t0, zero, .gp_restore

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

.gp_restore:

    lw x1,    0(sp)
    lw x4,    8(sp)
    lw x5,   16(sp)
    lw x6,   24(sp)
    lw x7,   32(sp)
    lw x8,   40(sp)
    lw x9,   48(sp)
    lw x10,  56(sp)
    lw x11,  64(sp)
    lw x12,  72(sp)
    lw x13,  80(sp)
    lw x14,  88(sp)
    lw x15,  96(sp)
    lw x16, 104(sp)
    lw x17, 112(sp)
    lw x18, 120(sp)
    lw x19, 128(sp)
    lw x20, 136(sp)
    lw x21, 144(sp)
    lw x22, 152(sp)
    lw x23, 160(sp)
    lw x24, 168(sp)
    lw x25, 176(sp)
    lw x26, 184(sp)
    lw x27, 192(sp)
    lw x28, 200(sp)
    lw x29, 208(sp)
    lw x30, 216(sp)
    lw x31, 224(sp)
    addi sp, sp, 232

    mret
