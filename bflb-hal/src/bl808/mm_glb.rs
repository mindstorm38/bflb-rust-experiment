//! Multimedia global register.

embedded_util::mmio! {
    pub struct MmGlb {
        [0x000] rw mm_clk_ctrl_cpu: MmGlbMmClkCtrlCpu,
        [0x004] rw mm_clk_cpu: MmGlbMmClkCpu,
        [0x008] rw dp_clk: MmGlbDpClk,
        [0x00C] rw codec_clk: MmGlbCodecClk,
        [0x010] rw mm_clk_ctrl_peri: MmGlbMmClkCtrlPeri,
        [0x018] rw mm_clk_ctrl_peri3: MmGlbMmClkCtrlPeri3,
        [0x040] rw mm_sw_sys_reset: MmGlbMmSwSysReset,
        [0x044] rw sw_reset_mm_peri: MmGlbSwResetMmPeri,
        [0x048] rw sw_reset_sub: MmGlbSwResetSub,
        [0x04C] rw sw_reset_codec_sub: MmGlbSwResetCodecSub,
        [0x050] rw image_sensor_ctrl: MmGlbImageSensorCtrl,
        [0x060] rw tz_mm_clkrst: MmGlbTzMmClkrst,
    }
}

embedded_util::reg! {
    pub struct MmGlbMmClkCtrlCpu: u32 {
        [00..01] pll_en,
        [01..02] cpu_clk_en,
        [02..03] bclk_en,
        [03..04] mm_cpu_clk_en,
        [04..06] uart_clk_sel,
        [06..07] i2c_clk_sel,
        [07..08] spi_clk_sel,
        [08..10] cpu_clk_sel,
        [10..11] xclk_clk_sel,
        [11..12] cpu_root_clk_sel,
        [12..13] mmcpu0_clk_en,
        [13..15] bclk1x_sel,
        [18..19] bclk2x_div_act_pulse,
        [19..20] bclk2x_div_bypass,
        [20..21] sts_bclk2x_prot_done,
        [24..28] bclk2x_sw_done_cnt,
        [28..31] cpu_clk_sw_state,
    }
}

embedded_util::reg! {
    pub struct MmGlbMmClkCpu: u32 {
        [00..08] cpu_clk_div,
        [08..09] cnn_clk_div_en,
        [09..11] cnn_clk_sel,
        [12..15] cnn_clk_div,
        [16..24] bclk2x_div,
        [24..32] bclk1x_div,
    }
}

embedded_util::reg! {
    pub struct MmGlbDpClk: u32 {
        [00..01] clk_div_en,
        [01..03] clk_sel,
        [08..16] clk_div,
        [16..17] dp_clk_div_en,
        [17..18] dp_clk_sel,
        [20..24] dp_clk_div,
    }
}

embedded_util::reg! {
    pub struct MmGlbCodecClk: u32 {
        [08..09] h264_clk_div_en,
        [09..11] h264_clk_sel,
        [12..15] h264_clk_div,
    }
}

embedded_util::reg! {
    pub struct MmGlbMmClkCtrlPeri: u32 {
        [00..08] i2c0_clk_div,
        [08..09] i2c0_clk_div_en,
        [09..10] i2c0_clk_en,
        [16..17] uart0_clk_div_en,
        [17..20] uart0_clk_div,
        [23..24] spi_clk_div_en,
        [24..32] spi_clk_div,
    }
}

embedded_util::reg! {
    pub struct MmGlbMmClkCtrlPeri3: u32 {
        [00..08] i2c1_clk_div,
        [08..09] i2c1_clk_div_en,
        [09..10] i2c1_clk_en,
        [16..17] uart1_clk_div_en,
        [17..20] uart1_clk_div,
    }
}

embedded_util::reg! {
    pub struct MmGlbMmSwSysReset: u32 {
        [00..01] ctrl_sys_reset,
        [02..03] ctrl_pwron_rst,
        [08..09] ctrl_mmcpu0_reset,
    }
}

embedded_util::reg! {
    pub struct MmGlbSwResetMmPeri: u32 {
        [00..01] swrst_mm_misc,
        [01..02] swrst_dma,
        [02..03] swrst_uart0,
        [03..04] swrst_i2c0,
        [04..05] swrst_i2c1,
        [05..06] swrst_ipc,
        [06..07] swrst_dma2d,
        [08..09] swrst_spi,
        [09..10] swrst_timer,
        [10..11] swrst_i2s0,
        [11..12] swrst_i2s1,
        [12..13] swrst_pdm0,
        [13..14] swrst_pdm1,
        [14..15] swrst_uart1,
        [15..16] swrst_puhs,
    }
}

embedded_util::reg! {
    pub struct MmGlbSwResetSub: u32 {
        [00..01] swrst_misc,
        [01..02] swrst_main,
        [02..03] swrst_tsrc,
        [03..04] swrst_dp_tsrc,
        [04..05] swrst_nr3d_ctrl,
        [05..06] swrst_dvp2busa,
        [06..07] swrst_dvp2busb,
        [07..08] swrst_dvp2busc,
        [08..09] swrst_dvp2busd,
        [09..10] swrst_mipi,
        [17..18] swrst_dvp2buse,
        [18..19] swrst_dvp2busf,
        [19..20] swrst_dvp2busg,
        [20..21] swrst_dvp2bush,
    }
}

embedded_util::reg! {
    pub struct MmGlbSwResetCodecSub: u32 {
        [00..01] swrst_codec_misc,
        [01..02] swrst_mjpeg,
        [02..03] swrst_h264,
        [03..04] swrst_mjpeg_dec,
        [04..05] swrst_cnn,
        [16..17] swrst_vram,
    }
}

embedded_util::reg! {
    pub struct MmGlbImageSensorCtrl: u32 {
        [00..01] rg_is_rst_n,
    }
}

embedded_util::reg! {
    pub struct MmGlbTzMmClkrst: u32 {
        [00..01] tzc_mm_swrst_lock,
        [01..02] tzc_mm_sys_reset_lock,
        [02..03] tzc_mm_pwron_rst_lock,
        [03..04] tzc_mm_cpu0_reset_lock,
        [04..05] tzc_mm_clk_lock,
    }
}
