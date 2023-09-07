//! DVP TSRC.

embedded_util::mmio! {
    pub struct Dtsrc {
        [0x000] rw config: DtsrcConfig,
        [0x004] rw frame_size_h: DtsrcFrameSizeH,
        [0x008] rw frame_size_v: DtsrcFrameSizeV,
        [0x00C] rw frame_size_cea_861: DtsrcFrameSizeCea861,
        [0x010] rw pix_data_range: DtsrcPixDataRange,
        [0x014] rw pix_data_step: DtsrcPixDataStep,
        [0x020] rw axi2dvp_setting: DtsrcAxi2dvpSetting,
        [0x024] rw axi2dvp_start_addr_by: DtsrcAxi2dvpStartAddrBy,
        [0x028] rw axi2dvp_burst_cnt: DtsrcAxi2dvpBurstCnt,
        [0x02C] rw axi2dvp_status: DtsrcAxi2dvpStatus,
        [0x030] rw axi2dvp_swap_addr_by: DtsrcAxi2dvpSwapAddrBy,
        [0x034] rw axi2dvp_prefetch: DtsrcAxi2dvpPrefetch,
        [0x038] rw snsr2dvp_wait_pos: DtsrcSnsr2dvpWaitPos,
        [0x040] rw axi2dvp_start_addr_uv: DtsrcAxi2dvpStartAddrUv,
        [0x044] rw axi2dvp_swap_addr_uv: DtsrcAxi2dvpSwapAddrUv,
    }
}

embedded_util::reg! {
    pub struct DtsrcConfig: u32 {
        [00..01] cr_enable,
        [01..02] cr_axi_en,
        [02..03] cr_mode_cea_861,
        [03..04] cr_snsr_en,
        [04..05] cr_snsr_hsync_inv,
        [05..06] cr_snsr_vsync_inv,
        [07..08] cr_axi_swap_mode,
        [08..11] cr_axi_swap_idx_sel,
        [12..13] cr_axi_swap_idx_swm,
        [13..14] cr_axi_swap_idx_swv,
        [16..19] cr_axi_dvp_data_mode,
        [20..22] cr_axi_b0_sel,
        [22..24] cr_axi_b1_sel,
        [24..26] cr_axi_b2_sel,
    }
}

embedded_util::reg! {
    pub struct DtsrcFrameSizeH: u32 {
        [00..12] cr_total_h,
        [16..28] cr_blank_h,
    }
}

embedded_util::reg! {
    pub struct DtsrcFrameSizeV: u32 {
        [00..12] cr_total_v,
        [16..28] cr_blank_v,
    }
}

embedded_util::reg! {
    pub struct DtsrcFrameSizeCea861: u32 {
        [00..08] cr_h_duration,
        [08..16] cr_h_placement,
        [16..24] cr_v_duration,
        [24..32] cr_v_placement,
    }
}

embedded_util::reg! {
    pub struct DtsrcPixDataRange: u32 {
        [00..16] cr_data_min,
        [16..32] cr_data_max,
    }
}

embedded_util::reg! {
    pub struct DtsrcPixDataStep: u32 {
        [00..08] cr_data_step,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpSetting: u32 {
        [00..03] cr_axi_xlen,
        [04..05] cr_axi_drain_err_clr,
        [08..09] cr_axi_420_mode,
        [09..10] cr_axi_420_ud_sel,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpStartAddrBy: u32 {
        [00..32] cr_axi_addr_start_by,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpBurstCnt: u32 {
        [00..32] cr_axi_frame_bc,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpStatus: u32 {
        [00..07] st_axi_fifo_cnt_by,
        [07..08] st_axi_drain_error_by,
        [08..09] st_axi_state_idle_by,
        [09..10] st_axi_state_func_by,
        [10..11] st_axi_state_flsh_by,
        [16..23] st_axi_fifo_cnt_uv,
        [23..24] st_axi_drain_error_uv,
        [24..25] st_axi_state_idle_uv,
        [25..26] st_axi_state_func_uv,
        [26..27] st_axi_state_flsh_uv,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpSwapAddrBy: u32 {
        [00..32] cr_axi_addr_swap_by,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpPrefetch: u32 {
        [00..12] cr_prefetch_v,
    }
}

embedded_util::reg! {
    pub struct DtsrcSnsr2dvpWaitPos: u32 {
        [00..12] cr_snsr_fifo_th,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpStartAddrUv: u32 {
        [00..32] cr_axi_addr_start_uv,
    }
}

embedded_util::reg! {
    pub struct DtsrcAxi2dvpSwapAddrUv: u32 {
        [00..32] cr_axi_addr_swap_uv,
    }
}
