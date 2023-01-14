//! Multimedia global register.

emhal::mmio_struct! {
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

emhal::mmio_reg! {
    pub struct MmGlbMmClkCtrlCpu: u32 {
        [0..1] pll_en,
        [1..2] cpu_clk_en,
        [2..3] bclk_en,
        [3..4] mm_cpu_clk_en,
        [4..6] uart_clk_sel,
        [6..7] i2c_clk_sel,
        [7..8] spi_clk_sel,
        [8..10] cpu_clk_sel,
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

emhal::mmio_reg! {
    pub struct MmGlbMmClkCpu: u32 {
        [0..8] cpu_clk_div,
        [8..9] cnn_clk_div_en,
        [9..11] cnn_clk_sel,
        [12..15] cnn_clk_div,
        [16..24] bclk2x_div,
        [24..32] bclk1x_div,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbDpClk: u32 {
        [0..1] clk_div_en,
        [1..3] clk_sel,
        [8..16] clk_div,
        [16..17] dp_clk_div_en,
        [17..18] dp_clk_sel,
        [20..24] dp_clk_div,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbCodecClk: u32 {
        [8..9] h264_clk_div_en,
        [9..11] h264_clk_sel,
        [12..15] h264_clk_div,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbMmClkCtrlPeri: u32 {
        [0..8] i2c0_clk_div,
        [8..9] i2c0_clk_div_en,
        [9..10] i2c0_clk_en,
        [16..17] uart0_clk_div_en,
        [17..20] uart0_clk_div,
        [23..24] spi_clk_div_en,
        [24..32] spi_clk_div,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbMmClkCtrlPeri3: u32 {
        [0..8] i2c1_clk_div,
        [8..9] i2c1_clk_div_en,
        [9..10] i2c1_clk_en,
        [16..17] uart1_clk_div_en,
        [17..20] uart1_clk_div,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbMmSwSysReset: u32 {
        [0..1] ctrl_sys_reset,
        [2..3] ctrl_pwron_rst,
        [8..9] ctrl_mmcpu0_reset,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbSwResetMmPeri: u32 {
        [0..1] swrst_mm_misc,
        [1..2] swrst_dma,
        [2..3] swrst_uart0,
        [3..4] swrst_i2c0,
        [4..5] swrst_i2c1,
        [5..6] swrst_ipc,
        [6..7] swrst_dma2d,
        [8..9] swrst_spi,
        [9..10] swrst_timer,
        [10..11] swrst_i2s0,
        [11..12] swrst_i2s1,
        [12..13] swrst_pdm0,
        [13..14] swrst_pdm1,
        [14..15] swrst_uart1,
        [15..16] swrst_puhs,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbSwResetSub: u32 {
        [0..1] swrst_misc,
        [1..2] swrst_main,
        [2..3] swrst_tsrc,
        [3..4] swrst_dp_tsrc,
        [4..5] swrst_nr3d_ctrl,
        [5..6] swrst_dvp2busa,
        [6..7] swrst_dvp2busb,
        [7..8] swrst_dvp2busc,
        [8..9] swrst_dvp2busd,
        [9..10] swrst_mipi,
        [17..18] swrst_dvp2buse,
        [18..19] swrst_dvp2busf,
        [19..20] swrst_dvp2busg,
        [20..21] swrst_dvp2bush,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbSwResetCodecSub: u32 {
        [0..1] swrst_codec_misc,
        [1..2] swrst_mjpeg,
        [2..3] swrst_h264,
        [3..4] swrst_mjpeg_dec,
        [4..5] swrst_cnn,
        [16..17] swrst_vram,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbImageSensorCtrl: u32 {
        [0..1] rg_is_rst_n,
    }
}

emhal::mmio_reg! {
    pub struct MmGlbTzMmClkrst: u32 {
        [0..1] tzc_mm_swrst_lock,
        [1..2] tzc_mm_sys_reset_lock,
        [2..3] tzc_mm_pwron_rst_lock,
        [3..4] tzc_mm_cpu0_reset_lock,
        [4..5] tzc_mm_clk_lock,
    }
}
