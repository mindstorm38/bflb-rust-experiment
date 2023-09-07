//! Camera  management registers.
//! 
//! This module is sourced from SDK source code:
//! - https://github.com/bouffalolab/bouffalo_sdk/blob/master/drivers/lhal/include/hardware/cam_reg.h
//! - https://github.com/bouffalolab/bouffalo_sdk/blob/master/drivers/lhal/include/hardware/cam_front_reg.h


embedded_util::mmio! {

    pub struct Cam {
        /// Global module configuration.
        [0x000] rw dvp2axi_cfg: CamDvp2AxiCfg,
        /// AXI start address.
        [0x004] rw dvp2axi_addr_start: u32,
        /// AXI burst count before wrap to `addr_start`.
        [0x008] rw dvp2axi_mem_size: u32,
        /// Interrupt status and errors.
        [0x00C] rw dvp_status_and_error: CamDvpStatusAndError,
        /// Single frame byte count.
        [0x010] rw dvp2axi_frame_size: u32,
        /// FIFO interrupts control.
        [0x014] rw dvp_frame_fifo_pop: CamDvpFrameFifoPop,
        /// Bitwise frame valid in period.
        [0x018] rw dvp2axi_frame_valid: u32,
        [0x01C] rw dvp2axi_frame_period: u32,
        [0x020] rw dvp2axi_misc: CamDvp2AxiMisc,
        /// HSync crop configuration.
        [0x030] rw dvp2axi_hsync_crop: CamDvp2AxiCrop,
        /// VSync crop configuration.
        [0x034] rw dvp2axi_vsync_crop: CamDvp2AxiCrop,
        /// X/Y resolutions.
        [0x038] rw dvp2axi_fram_exm: CamDvp2AxiFramExm,
        /// DVP2BUS PIC 0 Start address.
        [0x040] rw frame_start_addr0: u32,
        /// DVP2BUS PIC 1 Start address.
        [0x048] rw frame_start_addr1: u32,
        /// DVP2BUS PIC 2 Start address.
        [0x050] rw frame_start_addr2: u32,
        /// DVP2BUS PIC 3 Start address.
        [0x058] rw frame_start_addr3: u32,
        /// DVP2BUS PIC 0 & 1 IDs.
        [0x060] rw frame_id_sts01: CamFrameIdSts01,
        /// DVP2BUS PIC 2 & 3 IDs.
        [0x064] rw frame_id_sts23: CamFrameIdSts23,
        [0x0F0] rw dvp_debug: u32,
        [0x0FC] rw dvp_dummy_reg: u32,
    }

    pub struct CamFront {
        [0x000] rw cfg: CamFrontCfg,
        [0x004] rw pix_data_ctrl: CamFrontPixDataCtrl,
        [0x008] rw dvp2bus_src_sel_1: CamFrontDvp2BusSrcSel1,
        [0x014] rw dvp2bus_src_sel_2: CamFrontDvp2BusSrcSel2,
        [0x028] rw isp_id_yuv: u32,
        [0x108] rw adj_a_ctrl_2: u32, 
        [0x160] rw y2ra_cfg0: CamFrontY2raCfg0,
        [0x164] rw y2ra_cfg1: CamFrontY2raCfg1,
        [0x168] rw y2ra_cfg2: CamFrontY2raCfg2,
        [0x16C] rw y2ra_cfg3: CamFrontY2raCfg3,
        [0x170] rw y2ra_cfg4: CamFrontY2raCfg4,
        [0x174] rw y2ra_cfg5: CamFrontY2raCfg5,
        [0x178] rw y2ra_cfg6: CamFrontY2raCfg6,
        [0x17C] rw y2ra_cfg7: CamFrontY2raCfg7,
    }

}


embedded_util::reg! {

    pub struct CamDvp2AxiCfg: u32 {
        /// Module enable.
        [00..01] enable,
        /// DVP2BUS SW manual mode (ignored if `swap_mode` enabled).
        [01..02] sw_mode,
        /// Image sensor frame valid polarity:
        /// - 0 - Active low
        /// - 1 - Active high
        [02..03] frame_valid_polarity,
        /// Image sensor line valid polarity:
        /// - 0 - Active low
        /// - 1 - Active high
        [03..04] line_valid_polarity,
        /// Burst length setting:
        /// - 0 - INCR1
        /// - 1 - INCR4
        /// - 2 - INCR8
        /// - 3 - INCR16
        /// - 5 - INCR32
        /// - 6 - INCR64
        [04..07] burst_len,
        /// Image sensor mode selection:
        /// - 0 - VSync & HSync
        /// - 1 - VSync | HSync
        /// - 2 - VSync
        /// - 3 - HSync
        [08..11] dvp_mode,
        /// DVP2BUS HW mode with frame start address wrap to `addr_start`.
        [11..12] hx_mode_fwrap,
        /// Drop mode enable.
        [12..13] drop_enable,
        /// When drop is enabled:
        /// - 0 - Drop all odd bytes
        /// - 1 - Drop all even bytes
        [13..14] drop_even,
        /// AXI QoS software mode enable.
        [14..15] qos_sw_mode,
        /// AXI QoS software mode value.
        [15..16] qos_sw,
        /// DVP 8-bit mode enable:
        /// - 0 - DVP `pix_data` is 16-bit wide.
        /// - 1 - DVP `pix_data` is 24-bit mode.
        /// - 2 - DVP `pix_data` is 24-comp-16-bit mode.
        /// - 3 - DVP `pix_data` is 24-exp-32-bit mode.
        /// - 4 - DVP `pix_data` is 8-bit wide.
        [16..19] dvp_data_mode,
        /// Byte select signal for DVP 8-bit mode:
        /// - 0 - Select the lower byte of `pix_data`.
        /// - 1 - Select the upper byte of `pix_data`.
        [19..20] dvp_data_bsel,
        /// DVP2BUS vertical sub-sampling enable.
        [22..23] vertical_subsample_enable,
        /// DVP2BUS vertical sub-sampling polarity:
        /// - 0 - Odd lines are masked.
        /// - 1 - Even lines are masked.
        [23..24] vertical_subsample_polarity,
        /// Cycles in FSM Wait mode.
        [24..32] dvp_wait_cycle,
    }

    pub struct CamDvpStatusAndError: u32 {
        /// Frame to issue interrupt at software mode.
        [00..05] frame_count_trigger_int,
        /// HSync valid pixel count match interrupt enable.
        [06..07] int_hcount_en,
        /// VSync valid pixel count match interrupt enable.
        [07..08] int_vcount_en,
        /// Normal Write interrupt enable.
        [08..09] int_normal_en,
        /// Memory OverWrite interrupt enable.
        [09..10] int_mem_en,
        /// Frame OverWrite interrupt enable.
        [10..11] int_frame_en,
        /// FIFO OverWrite interrupt enable.
        [11..12] int_fifo_en,
        /// Normal Write interrupt status (read-only).
        [12..13] sts_normal_int,
        /// Memory OverWrite interrupt status (read-only).
        [13..14] sts_mem_int,
        /// Frame OverWrite interrupt status (read-only).
        [14..15] sts_frame_int,
        /// FIFO OverWrite interrupt status (read-only).
        [15..16] sts_fifo_int,
        /// Frame counts in memory before read out in SW mode (read-only).
        [16..21] frame_valid_count,
        /// Hsync valid pixel count non-match interrupt status (read-only).
        [21..22] sts_hcount_int,
        /// Vsync valid line count non-match interrupt status (read-only).
        [22..23] sts_vcount_int,
        /// DVP in idle state (read-only).
        [24..25] st_bus_idle,
        /// DVP in functional state (read-only).
        [25..26] st_bus_func,
        /// DVP in wait state (read-only).
        [26..27] st_bus_wait,
        /// DVP in flush state (read-only).
        [27..28] st_bus_flush,
        /// DVP2BUS AHB idle status (read-only).
        [28..29] axi_idle,
        /// DVP2BUS asynchronous fifo idle status (read-only).
        [29..30] st_dvp_idle,
    }

    pub struct CamDvpFrameFifoPop: u32 {
        /// Write-only command bit to trigger a FIFO pop.
        [00..01] rfifo_pop,
        /// Interrupt clear.
        [04..05] int_normal_clr,
        /// Interrupt clear.
        [05..06] int_mem_clr,
        /// Interrupt clear.
        [06..07] int_frame_clr,
        /// Interrupt clear.
        [07..08] int_fifo_clr,
        /// Interrupt clear.
        [08..09] int_hcount_clr,
        /// Interrupt clear.
        [09..10] int_vcount_clr,
    }

    pub struct CamDvp2AxiMisc: u32 {
        /// Only work when `dvp_data_mode = 3` is 24-exp-32-bit mode.
        [00..08] alpha,
        /// Only work when `dvp_data_mode = 2` is 24-comp-16-bit mode.
        /// - 0 - B2(5)B1(6)B0(5)
        /// - 1 - B1(5)B2(6)B0(5)
        /// - 2 - B2(5)B0(6)B1(5)
        /// - 3 - B0(5)B2(6)B1(5)
        /// - 4 - B1(5)B0(6)B2(5)
        /// - 5 - B0(5)B1(6)B2(5)
        [08..11] format_565,
    }

    pub struct CamDvp2AxiCrop: u32 {
        [00..16] crop_end,
        [16..32] crop_start,
    }

    pub struct CamDvp2AxiFramExm: u32 {
        [00..16] resolution_x,
        [16..32] resolution_y,
    }
    
    pub struct CamFrameIdSts01: u32 {
        /// DVP2BUS PIC 0 ID.
        [00..16] frame_id_0,
        /// DVP2BUS PIC 1 ID.
        [16..32] frame_id_1,
    }
    
    pub struct CamFrameIdSts23: u32 {
        /// DVP2BUS PIC 2 ID.
        [00..16] frame_id_2,
        /// DVP2BUS PIC 3 ID.
        [16..32] frame_id_3,
    }

}


embedded_util::reg! {

    pub struct CamFrontCfg: u32 {
        [0..1] dvpas_enable,
        [1..2] dvpas_hs_inv,
        [2..3] dvpas_vs_inv,
        [3..4] dvpas_da_order,
        [16..27] dvpas_fifo_th,
    }
    
    pub struct CamFrontPixDataCtrl: u32 {
        [0..12] pix_data_ctrl,
        [16..20] pix_data_sht_bit,
        [20..21] pix_data_sht_dir,
        [31..32] isp_dtsrc_src,
    }

    pub struct CamFrontDvp2BusSrcSel1: u32 {
        [00..06] d2b_dvp_sel_a,
        [07..08] d2x_id_sel_a,
        [08..14] d2b_dvp_sel_b,
        [15..16] d2x_id_sel_b,
        [16..22] d2b_dvp_sel_c,
        [23..24] d2x_id_sel_c,
        [24..30] d2b_dvp_sel_d,
        [31..32] d2x_id_sel_d,
    }

    pub struct CamFrontDvp2BusSrcSel2: u32 {
        [00..06] d2b_dvp_sel_e,
        [07..08] d2x_id_sel_e,
        [08..14] d2b_dvp_sel_f,
        [15..16] d2x_id_sel_f,
        [16..22] d2b_dvp_sel_g,
        [23..24] d2x_id_sel_g,
        [24..30] d2b_dvp_sel_h,
        [31..32] d2x_id_sel_h,
    }

    pub struct CamFrontY2raCfg0: u32 {
        [00..09] y2ra_pre_0,
        [16..25] y2ra_pos_0,
        [27..28] y2ra_en,
        [28..32] y2ra_sel,
    }
    
    pub struct CamFrontY2raCfg1: u32 {
        [0..9]   y2ra_pre_1,
        [16..25] y2ra_pos_1
    }
    
    pub struct CamFrontY2raCfg2: u32 {
        [0..9]   y2ra_pre_2,
        [16..25] y2ra_pos_2
    }
    
    pub struct CamFrontY2raCfg3: u32 {
        [0..12]  y2ra_mtx_00,
        [16..28] y2ra_mtx_01,
    }
    
    pub struct CamFrontY2raCfg4: u32 {
        [0..12]  y2ra_mtx_02,
        [16..28] y2ra_mtx_10,
    }
    
    pub struct CamFrontY2raCfg5: u32 {
        [0..12]  y2ra_mtx_11,
        [16..28] y2ra_mtx_12,
    }
    
    pub struct CamFrontY2raCfg6: u32 {
        [0..12]  y2ra_mtx_20,
        [16..28] y2ra_mtx_21,
    }
    
    pub struct CamFrontY2raCfg7: u32 {
        [0..12]  y2ra_mtx_22,
    }

}
