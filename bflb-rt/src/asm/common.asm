# Common constants.

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

# Other CSRs
.equ mtvt,          0x307
.equ mnxti,         0x345
.equ mintstatus,    0xFB1
.equ mintthresh,    0x347
.equ mscratchcsw,   0x348
.equ mscratchcswl,  0x349
