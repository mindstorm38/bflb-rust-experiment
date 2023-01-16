//! Camera  management registers.
//! 
//! This module is sourced from SDK source code and RM:
//! - mjpeg_reg.h


embedded_util::mmio! {

    pub struct Mjpeg {
        [0x000] rw mjpeg_control_1: MjpegControl1,
        [0x004] rw mjpeg_control_2: MjpegControl2,
        [0x008] rw mjpeg_yy_frame_addr: u32,
        [0x00C] rw mjpeg_uv_frame_addr: u32,
        [0x010] rw mjpeg_yuv_mem: MjpegYuvMem,
        [0x014] rw jpeg_frame_addr: u32,
        /// Also named w_burst_count
        [0x018] rw jpeg_store_memory: u32,
        [0x01C] rw mjpeg_control_3: MjpegControl3,
        [0x020] rw mjpeg_frame_fifo_pop: MjpegFrameFifoPop,
        [0x024] rw mjpeg_frame_size: MjpegFrameSize,
        [0x028] rw mjpeg_header_byte: MjpegHeaderByte,
        [0x030] rw mjpeg_swap_mode: MjpegSwapMode,
        [0x034] rw mjpeg_swap_bit_cnt: u32,
        [0x038] rw mjpeg_yuv_mem_sw: u32,
        [0x040] rw mjpeg_y_frame_read_status_1: u32,
        [0x044] rw mjpeg_y_frame_read_status_2: u32,
        [0x048] rw mjpeg_y_frame_write_status: u32,
        [0x04C] rw mjpeg_uv_frame_read_status_1: u32,
        [0x050] rw mjpeg_uv_frame_read_status_2: u32,
        [0x054] rw mjpeg_uv_frame_write_status: u32,
        [0x058] rw mjpeg_frame_w_hblk_status: u32,
        [0x080] rw mjpeg_start_addr0: u32,
        [0x084] rw mjpeg_bit_cnt0: u32,
        [0x088] rw mjpeg_start_addr1: u32,
        [0x08C] rw mjpeg_bit_cnt1: u32,
        [0x090] rw mjpeg_start_addr2: u32,
        [0x094] rw mjpeg_bit_cnt2: u32,
        [0x098] rw mjpeg_start_addr3: u32,
        [0x09C] rw mjpeg_bit_cnt3: u32,
        [0x100] rw mjpeg_q_enc: MjpegQEnc,
        [0x110] rw mjpeg_frame_id_10: u32,
        [0x114] rw mjpeg_frame_id_32: u32,
        [0x1F0] rw mjpeg_debug: u32,
        [0x1FC] rw mjpeg_dummy_reg: u32,
        [0x400] rw mjpeg_q_param_00: [u16; 64],
        [0x480] rw mjpeg_q_param_40: [u16; 64],
    }

}


embedded_util::reg! {

    pub struct MjpegControl1: u32 {
        [00..01] mjpeg_enable,
        [01..02] mjpeg_bit_order,
        [02..03] order_u_even,
        [03..04] hw_mode_swen,
        [04..05] last_hf_wblk_dmy,
        [05..06] last_hf_hblk_dmy,
        [06..07] reflect_dmy,
        [07..08] read_fwrap,
        /// Burst length setting:
        /// - 0 - INCR1
        /// - 1 - INCR4
        /// - 2 - INCR8
        /// - 3 - INCR16
        /// - 5 - INCR32
        /// - 6 - INCR64
        [08..11] burst_len,
        [12..14] yuv_mode,
        [24..30] mjpeg_hw_frame,
    }

    pub struct MjpegControl2: u32 {
        [00..05] sw_frame,
        /// Write-only command bit to trigger a compression.
        [06..07] sw_kick,
        /// Enable the kick mode.
        [07..08] sw_kick_mode, 
        /// Enable the sofware mode.
        [08..09] mjpeg_sw_mode,
        [09..10] mjpeg_sw_run,
        [10..13] yy_dvp2axi_sel,
        [13..16] uv_dvp2axi_sel,
        [16..32] mjpeg_wait_cycle,
    }

    pub struct MjpegYuvMem: u32 {
        [00..13] yy_mem_hblock,
        [16..29] uv_mem_hblock,
    }

    pub struct MjpegControl3: u32 {
        [00..01] int_normal_en,
        [01..02] int_cam_en,
        [02..03] int_mem_en,
        [03..04] int_frame_en,
        [04..05] sts_normal_int,
        [05..06] sts_cam_int,
        [06..07] sts_mem_int,
        [07..08] sts_frame_int,
        [08..09] idle,
        [09..10] func,
        [10..11] wait,
        [11..12] flush,
        [12..13] mans,
        [13..14] manf,
        [14..15] axi_read_idle,
        [15..16] axi_write_idle,
        [16..21] frame_count_trigger_int,
        [21..22] int_idle_en,
        [22..23] sts_idle_int,
        [24..29] frame_valid_count,
        [29..30] int_swap_en,
        [30..31] sts_swap_int,
    }

    pub struct MjpegFrameFifoPop: u32 {
        [00..01] rfifo_pop,
        [01..02] w_swap_clear,
        [08..09] int_normal_clear,
        [09..10] int_cam_clear,
        [10..11] int_mem_clear,
        [11..12] int_frame_clear,
        [12..13] int_idle_clear,
        [13..14] int_swap_clear,
    }

    pub struct MjpegFrameSize: u32 {
        [00..12] frame_wblock,
        [16..28] frame_hblock,
    }

    pub struct MjpegHeaderByte: u32 {
        [00..12] head_byte,
        [16..17] tail_exp,
        [24..26] y0_order,
        [26..28] u0_order,
        [28..30] y1_order,
        [30..32] v0_order,
    }

    pub struct MjpegSwapMode: u32 {
        [00..01] w_swap_mode,
        [08..09] sts_swap0_full,
        [09..10] sts_swap1_full,
        [10..11] sts_read_swap_index,
        [11..12] sts_swap_fstart,
        [12..13] sts_swap_fend,
    }

    pub struct MjpegQEnc: u32 {
        [00..01] frame_q_sram_0,
        [01..02] frame_q_sram_1,
        [02..03] frame_q_sram_2,
        [03..04] frame_q_sram_3,
        [24..25] q_sram_sw,
        [25..26] q_sram_enc,
    }

}
