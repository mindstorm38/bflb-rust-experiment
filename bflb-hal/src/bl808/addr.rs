//! Constant addresses definitions.
//! 
//! Source:
//! - https://github.com/bouffalolab/bl_mcu_sdk/blob/master/drivers/lhal/config/bl808/bl808_memorymap.h


pub const CPU_ID_BASE: usize            = 0xF0000000;

// WLSYS
pub const GLB_BASE: usize               = 0x20000000;
pub const MIX_BASE: usize               = 0x20001000;
pub const GPIP_BASE: usize              = 0x20002000;
pub const PHY_BASE: usize               = 0x20002800;
pub const AGC_BASE: usize               = 0x20002c00;
pub const SEC_DBG_BASE: usize           = 0x20003000;
pub const SEC_ENG_BASE: usize           = 0x20004000;
pub const TZ1_BASE: usize               = 0x20005000;
pub const TZC_SEC_BASE: usize           = 0x20005000;
pub const TZ2_BASE: usize               = 0x20006000;
pub const TZC_NSEC_BASE: usize          = 0x20006000;
pub const EFUSE_BASE: usize             = 0x20056000;
pub const EF_DATA_BASE: usize           = 0x20056000;
pub const EF_CTRL_BASE: usize           = 0x20056000;
pub const CCI_BASE: usize               = 0x20008000;
pub const MCU_MISC_BASE: usize          = 0x20009000;
pub const L1C_BASE: usize               = 0x20009000;
pub const UART0_BASE: usize             = 0x2000a000;
pub const UART1_BASE: usize             = 0x2000a100;
pub const SPI0_BASE: usize              = 0x2000a200;
pub const I2C0_BASE: usize              = 0x2000a300;
pub const PWM_BASE: usize               = 0x2000a400;
pub const TIMER0_BASE: usize            = 0x2000a500;
pub const IR_BASE: usize                = 0x2000a600;
pub const CKS_BASE: usize               = 0x2000a700;
pub const IPC0_BASE: usize              = 0x2000a800;
pub const IPC1_BASE: usize              = 0x2000a840;
pub const I2C1_BASE: usize              = 0x2000a900;
pub const UART2_BASE: usize             = 0x2000aa00;
pub const ISO11898_BASE: usize          = 0x2000aa00;
pub const I2S_BASE: usize               = 0x2000ab00;
pub const PDM0_BASE: usize              = 0x2000a000;
pub const LZ4D_BASE: usize              = 0x2000ad00;
pub const QSPI_BASE: usize              = 0x2000b000;
pub const SF_CTRL_BASE: usize           = 0x2000b000;
pub const SF_CTRL_BUF_BASE: usize       = 0x2000b600;
pub const DMA0_BASE: usize              = 0x2000c000;
pub const PDS_BASE: usize               = 0x2000e000;
pub const HBN_BASE: usize               = 0x2000f000;
pub const AON_BASE: usize               = 0x2000f000;
pub const EMI_MISC_BASE: usize          = 0x20050000;
pub const PSRAM_CTRL_BASE: usize        = 0x20052000;
pub const USB_BASE: usize               = 0x20072000;
pub const AUDIO_BASE: usize             = 0x20055000;
pub const SDH_BASE: usize               = 0x20060000;
pub const EMAC_BASE: usize              = 0x20070000;
pub const DMA1_BASE: usize              = 0x20071000;

// MMSYS
pub const MM_MISC_BASE: usize           = 0x30000000;
pub const DMA2_BASE: usize              = 0x30001000;
pub const UART3_BASE: usize             = 0x30002000;
pub const I2C2_BASE: usize              = 0x30003000;
pub const I2C3_BASE: usize              = 0x30004000;
pub const IPC2_BASE: usize              = 0x30005000;
pub const DMA2D_BASE: usize             = 0x30006000;
pub const CLKRST_CTRL_BASE: usize       = 0x30007000;
pub const MM_GLB_BASE: usize            = 0x30007000;
pub const SPI1_BASE: usize              = 0x30008000;
pub const TIMER1_BASE: usize            = 0x30009000;
pub const PSRAM_UHS_BASE: usize         = 0x3000f000;

// ISP_SUBSYS
pub const ISP_MISC_BASE: usize          = 0x30010000;
pub const ISP_BASE: usize               = 0x30011000;
pub const DVP0_BASE: usize              = 0x30012000;
pub const DVP1_BASE: usize              = 0x30012100;
pub const DVP2_BASE: usize              = 0x30012200;
pub const DVP3_BASE: usize              = 0x30012300;
pub const DVP4_BASE: usize              = 0x30012400;
pub const DVP5_BASE: usize              = 0x30012500;
pub const DVP6_BASE: usize              = 0x30012600;
pub const DVP7_BASE: usize              = 0x30012700;
pub const DVP_TSRC0_BASE: usize         = 0x30012800;
pub const DVP_TSRC1_BASE: usize         = 0x30012900;
pub const AXI_CTRL_NR3D_BASE: usize     = 0x30012a00;
pub const OSD_PROBE_BASE: usize         = 0x30012b00;
pub const OSD_A_BASE: usize             = 0x30013000;
pub const OSD_B_BASE: usize             = 0x30014000;
pub const OSD_DP_BASE: usize            = 0x30015000;
pub const OSD_BLEND0_OFFSET: usize      = 0x000;
pub const OSD_BLEND1_OFFSET: usize      = 0x100;
pub const OSD_BLEND2_OFFSET: usize      = 0x200;
pub const OSD_BLEND3_OFFSET: usize      = 0x300;
pub const OSD_DRAW_LOW_OFFSET: usize    = 0x400;
pub const OSD_DRAW_HIGH_OFFSET: usize   = 0x504;
pub const MIPI_BASE: usize              = 0x3001a000;
pub const DBI_BASE: usize               = 0x3001b000;
pub const DSI_BASE: usize               = 0x3001a100;
pub const CSI_BASE: usize               = 0x3001a000;

// CODEC_SUBSYS
pub const CODEC_MISC_BASE: usize        = 0x30020000;
pub const MJPEG_BASE: usize             = 0x30021000;
pub const VIDEO_BASE: usize             = 0x30022000;
pub const MJPEG_DEC_BASE: usize         = 0x30023000;
pub const BL_CNN_BASE: usize            = 0x30024000;

pub const HBN_RAM_BASE: usize           = 0x20010000;

pub const RF_BASE: usize                = 0x20001000;

// FLASH
pub const FLASH1_XIP_BASE: usize        = 0x58000000;
pub const FLASH1_XIP_END: usize         = 0x5C000000;
pub const FLASH2_XIP_BASE: usize        = 0x5C000000;
pub const FLASH2_XIP_END: usize         = 0x60000000;
