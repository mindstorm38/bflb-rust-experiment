//! 

emhal::mmio_struct! {
    pub struct Cci {
        [0x000] rw cfg: CciCfg,
        [0x004] rw addr: CciAddr,
        [0x008] rw wdata: CciWdata,
        [0x00C] rw rdata: CciRdata,
        [0x010] rw ctl: CciCtl,
        /// Alias for `audio_pll_cfg0`.
        [0x750] rw audio_pll_cfg0_: super::PllCfg0,
        [0x750] rw audio_pll_cfg0: CciAudioPllCfg0,
        /// Alias for `audio_pll_cfg1`.
        [0x754] rw audio_pll_cfg1_: super::PllCfg1,
        [0x754] rw audio_pll_cfg1: CciAudioPllCfg1,
        [0x758] rw audio_pll_cfg2: CciAudioPllCfg2,
        [0x75C] rw audio_pll_cfg3: CciAudioPllCfg3,
        [0x760] rw audio_pll_cfg4: CciAudioPllCfg4,
        [0x764] rw audio_pll_cfg5: CciAudioPllCfg5,
        [0x768] rw audio_pll_cfg6: CciAudioPllCfg6,
        [0x76C] rw audio_pll_cfg7: CciAudioPllCfg7,
        [0x770] rw audio_pll_cfg8: CciAudioPllCfg8,
        [0x774] rw audio_pll_cfg9: CciAudioPllCfg9,
        [0x778] rw audio_pll_cfg10: CciAudioPllCfg10,
        [0x77C] rw audio_pll_cfg11: CciAudioPllCfg11,
        /// Alias for `cpu_pll_cfg0`.
        [0x7D0] rw cpu_pll_cfg0_: super::PllCfg0,
        [0x7D0] rw cpu_pll_cfg0: CciCpuPllCfg0,
        /// Alias for `cpu_pll_cfg1`.
        [0x7D4] rw cpu_pll_cfg1_: super::PllCfg1,
        [0x7D4] rw cpu_pll_cfg1: CciCpuPllCfg1,
        [0x7D8] rw cpu_pll_cfg2: CciCpuPllCfg2,
        [0x7DC] rw cpu_pll_cfg3: CciCpuPllCfg3,
        [0x7E0] rw cpu_pll_cfg4: CciCpuPllCfg4,
        [0x7E4] rw cpu_pll_cfg5: CciCpuPllCfg5,
        [0x7E8] rw cpu_pll_cfg6: CciCpuPllCfg6,
        [0x7EC] rw cpu_pll_cfg7: CciCpuPllCfg7,
        [0x7F0] rw cpu_pll_cfg8: CciCpuPllCfg8,
        [0x7F4] rw cpu_pll_cfg9: CciCpuPllCfg9,
        [0x7F8] rw cpu_pll_cfg10: CciCpuPllCfg10,
        [0x7FC] rw cpu_pll_cfg11: CciCpuPllCfg11,
    }
}

emhal::mmio_reg! {
    pub struct CciCfg: u32 {
        [0..1] en,
        [1..2] slv_sel_cci2,
        [2..3] mas_sel_cci2,
        [3..4] mas_hw_mode,
        [4..5] m_cci_sclk_en,
        [5..7] div_m_cci_sclk,
        [7..8] cfg_cci1_pre_read,
        [8..9] scci_clk_inv,
        [9..10] mcci_clk_inv,
    }
}

emhal::mmio_reg! {
    pub struct CciAddr: u32 {
        [0..32] apb_cci_addr,
    }
}

emhal::mmio_reg! {
    pub struct CciWdata: u32 {
        [0..32] apb_cci_wdata,
    }
}

emhal::mmio_reg! {
    pub struct CciRdata: u32 {
        [0..32] apb_cci_rdata,
    }
}

emhal::mmio_reg! {
    pub struct CciCtl: u32 {
        [0..1] write_flag,
        [1..2] read_flag,
        [2..4] ahb_state,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg0: u32 {
        [0..1] aupll_sdm_rstb,
        [1..2] aupll_postdiv_rstb,
        [2..3] aupll_fbdv_rstb,
        [3..4] aupll_refdiv_rstb,
        [4..5] pu_aupll_postdiv,
        [5..6] pu_aupll_fbdv,
        [6..7] pu_aupll_clamp_op,
        [7..8] pu_aupll_pfd,
        [8..9] pu_aupll_cp,
        [9..10] pu_aupll_sfreg,
        [10..11] pu_aupll,
        [11..12] pu_aupll_clktree,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg1: u32 {
        [0..7] aupll_postdiv,
        [8..12] aupll_refdiv_ratio,
        [16..18] aupll_refclk_sel,
        [20..22] aupll_vg11_sel,
        [24..26] aupll_vg13_sel,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg2: u32 {
        [0..1] aupll_sel_cp_bias,
        [4..6] aupll_icp_5u,
        [6..8] aupll_icp_1u,
        [8..9] aupll_int_frac_sw,
        [9..10] aupll_cp_startup_en,
        [10..11] aupll_cp_opamp_en,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg3: u32 {
        [0..1] aupll_c4_en,
        [4..6] aupll_r4,
        [8..9] aupll_r4_short,
        [12..14] aupll_c3,
        [14..16] aupll_cz,
        [16..19] aupll_rz,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg4: u32 {
        [0..2] aupll_sel_sample_clk,
        [4..6] aupll_sel_fb_clk,
        [8..9] aupll_sdmclk_sel,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg5: u32 {
        [0..3] aupll_vco_speed,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg6: u32 {
        [0..19] aupll_sdmin,
        [24..25] aupll_sdm_bypass,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg7: u32 {
        [0..1] aupll_sdm_order_sel,
        [16..18] aupll_sdm_sig_dith_sel,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg8: u32 {
        [0..1] aupll_en_div1,
        [1..2] aupll_en_div2,
        [2..3] aupll_en_div2p5,
        [3..4] aupll_en_div3,
        [4..5] aupll_en_div4,
        [5..6] aupll_en_div5,
        [6..7] aupll_en_div6,
        [7..8] aupll_en_div10,
        [8..9] aupll_en_div15,
        [9..10] aupll_sel_div1_div2,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg9: u32 {
        [0..1] aupll_dc_tp_out_en,
        [1..2] ten_aupll,
        [2..3] ten_aupll_sfreg,
        [4..5] dten_aupll_fin,
        [5..6] dten_aupll_fref,
        [6..7] dten_aupll_fsdm,
        [7..8] dten_aupll_div15,
        [8..9] dten_aupll_div5,
        [9..10] dten_aupll_postdiv_clk,
        [10..11] dtest_aupll_pulldown,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg10: u32 {
        [0..1] aupll_ssc_en,
        [4..12] aupll_ssc_cnt,
        [12..15] aupll_ssc_gain,
        [16..17] aupll_ssc_start_gate_en,
        [20..21] aupll_ssc_start,
    }
}

emhal::mmio_reg! {
    pub struct CciAudioPllCfg11: u32 {
        [0..16] aupll_resv,
        [23..24] aupll_dl_ctrl_15,
        [24..25] aupll_dl_ctrl_10,
        [25..26] aupll_dl_ctrl_6,
        [26..27] aupll_dl_ctrl_5,
        [27..28] aupll_dl_ctrl_4,
        [28..29] aupll_dl_ctrl_3,
        [29..30] aupll_dl_ctrl_2p5,
        [30..31] aupll_dl_ctrl_2,
        [31..32] aupll_dl_ctrl_1,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg0: u32 {
        [0..1] cpupll_sdm_rstb,
        [1..2] cpupll_postdiv_rstb,
        [2..3] cpupll_fbdv_rstb,
        [3..4] cpupll_refdiv_rstb,
        [4..5] pu_cpupll_postdiv,
        [5..6] pu_cpupll_fbdv,
        [6..7] pu_cpupll_clamp_op,
        [7..8] pu_cpupll_pfd,
        [8..9] pu_cpupll_cp,
        [9..10] pu_cpupll_sfreg,
        [10..11] pu_cpupll,
        [11..12] pu_cpupll_clktree,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg1: u32 {
        [0..7] cpupll_postdiv,
        [8..12] cpupll_refdiv_ratio,
        [16..18] cpupll_refclk_sel,
        [20..22] cpupll_vg11_sel,
        [24..26] cpupll_vg13_sel,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg2: u32 {
        [0..1] cpupll_sel_cp_bias,
        [4..6] cpupll_icp_5u,
        [6..8] cpupll_icp_1u,
        [8..9] cpupll_int_frac_sw,
        [9..10] cpupll_cp_startup_en,
        [10..11] cpupll_cp_opamp_en,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg3: u32 {
        [0..1] cpupll_c4_en,
        [4..6] cpupll_r4,
        [8..9] cpupll_r4_short,
        [12..14] cpupll_c3,
        [14..16] cpupll_cz,
        [16..19] cpupll_rz,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg4: u32 {
        [0..2] cpupll_sel_sample_clk,
        [4..6] cpupll_sel_fb_clk,
        [8..9] cpupll_sdmclk_sel,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg5: u32 {
        [0..3] cpupll_vco_speed,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg6: u32 {
        [0..19] cpupll_sdmin,
        [24..25] cpupll_sdm_bypass,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg7: u32 {
        [0..1] cpupll_sdm_order_sel,
        [16..18] cpupll_sdm_sig_dith_sel,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg8: u32 {
        [0..1] cpupll_en_div1,
        [1..2] cpupll_en_div2,
        [2..3] cpupll_en_div2p5,
        [3..4] cpupll_en_div3,
        [4..5] cpupll_en_div4,
        [5..6] cpupll_en_div5,
        [6..7] cpupll_en_div6,
        [7..8] cpupll_en_div10,
        [8..9] cpupll_en_div15,
        [9..10] cpupll_sel_div1_div2,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg9: u32 {
        [0..1] cpupll_dc_tp_out_en,
        [1..2] ten_cpupll,
        [2..3] ten_cpupll_sfreg,
        [4..5] dten_cpupll_fin,
        [5..6] dten_cpupll_fref,
        [6..7] dten_cpupll_fsdm,
        [7..8] dten_cpupll_div15,
        [8..9] dten_cpupll_div5,
        [9..10] dten_cpupll_postdiv_clk,
        [10..11] dtest_cpupll_pulldown,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg10: u32 {
        [0..1] cpupll_ssc_en,
        [4..12] cpupll_ssc_cnt,
        [12..15] cpupll_ssc_gain,
        [16..17] cpupll_ssc_start_gate_en,
        [20..21] cpupll_ssc_start,
    }
}

emhal::mmio_reg! {
    pub struct CciCpuPllCfg11: u32 {
        [0..16] cpupll_resv,
        [23..24] cpupll_dl_ctrl_15,
        [24..25] cpupll_dl_ctrl_10,
        [25..26] cpupll_dl_ctrl_6,
        [26..27] cpupll_dl_ctrl_5,
        [27..28] cpupll_dl_ctrl_4,
        [28..29] cpupll_dl_ctrl_3,
        [29..30] cpupll_dl_ctrl_2p5,
        [30..31] cpupll_dl_ctrl_2,
        [31..32] cpupll_dl_ctrl_1,
    }
}
