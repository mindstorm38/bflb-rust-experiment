# Common startup code and constants, used for all chips.

.equ MSTATUS_MIE,           1 << 3
.equ MSTATUS_SIE,           1 << 1

.equ MSTATUS_FS,            3 << 13
.equ MSTATUS_FS_OFF,        0 << 13
.equ MSTATUS_FS_INITIAL,    1 << 13
.equ MSTATUS_FS_CLEAN,      2 << 13
.equ MSTATUS_FS_DIRTY,      3 << 13

.equ MTVEC_DIRECT,          0
.equ MTVEC_VECTORED,        1
.equ MTVEC_CLIC,            3

.equ mtvt,          0x307
.equ mnxti,         0x345
.equ mintstatus,    0xFB1
.equ mintthresh,    0x347
.equ mscratchcsw,   0x348
.equ mscratchcswl,  0x349


.section .text

.global _start_common
_start_common:

    # # Initialize stack pointer.
    # la sp, _ld_stack_top

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
