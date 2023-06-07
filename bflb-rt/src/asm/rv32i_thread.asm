# Generic context switch function

.attribute arch, "rv32i"

.section .ramtext

# pub fn _thread_switch(into: *mut Context, from: *const Context);
.global _thread_switch
_thread_switch:

    # Skip the following section if there is no context to save into.
    beq a0, zero, .restore

    # We save the return address (ra) to pc for future restore.
    # The current function will ultimately return through this 
    sw ra,   0(a0)  # into->pc
    sw sp,   4(a0)  # into->sp
    sw s0,   8(a0)  # into->s0
    sw s1,  12(a0)  # into->s1
    sw s2,  16(a0)  # into->s2
    sw s3,  20(a0)  # into->s3
    sw s4,  24(a0)  # into->s4
    sw s5,  28(a0)  # into->s5
    sw s6,  32(a0)  # into->s6
    sw s7,  36(a0)  # into->s7
    sw s8,  40(a0)  # into->s8
    sw s9,  44(a0)  # into->s9
    sw s10, 48(a0)  # into->s10
    sw s11, 52(a0)  # into->s11

.restore:
    # Restore context.
    lw ra,   0(a1)  # from->pc (used as temporary register).
    lw sp,   4(a1)  # from->sp
    lw s0,   8(a1)  # from->s0
    lw s1,  12(a1)  # from->s1
    lw s2,  16(a1)  # from->s2
    lw s3,  20(a1)  # from->s3
    lw s4,  24(a1)  # from->s4
    lw s5,  28(a1)  # from->s5
    lw s6,  32(a1)  # from->s6
    lw s7,  36(a1)  # from->s7
    lw s8,  40(a1)  # from->s8
    lw s9,  44(a1)  # from->s9
    lw s10, 48(a1)  # from->s10
    lw s11, 52(a1)  # from->s11
    jr ra

    # Important note: We don't ret from this function, it's 
    # intentionnal because return will happen when the context
    # is restored.
