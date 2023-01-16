//! 

embedded_util::mmio! {
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

embedded_util::reg! {
    pub struct CciCfg: u32 {
        [00..01] en,
        [01..02] slv_sel_cci2,
        [02..03] mas_sel_cci2,
        [03..04] mas_hw_mode,
        [04..05] m_cci_sclk_en,
        [05..07] div_m_cci_sclk,
        [07..08] cfg_cci1_pre_read,
        [08..09] scci_clk_inv,
        [09..10] mcci_clk_inv,
    }
}

embedded_util::reg! {
    pub struct CciAddr: u32 {
        [00..32] apb_cci_addr,
    }
}

embedded_util::reg! {
    pub struct CciWdata: u32 {
        [00..32] apb_cci_wdata,
    }
}

embedded_util::reg! {
    pub struct CciRdata: u32 {
        [00..32] apb_cci_rdata,
    }
}

embedded_util::reg! {
    pub struct CciCtl: u32 {
        [00..01] write_flag,
        [01..02] read_flag,
        [02..04] ahb_state,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg0: u32 {
        [00..01] aupll_sdm_rstb,
        [01..02] aupll_postdiv_rstb,
        [02..03] aupll_fbdv_rstb,
        [03..04] aupll_refdiv_rstb,
        [04..05] pu_aupll_postdiv,
        [05..06] pu_aupll_fbdv,
        [06..07] pu_aupll_clamp_op,
        [07..08] pu_aupll_pfd,
        [08..09] pu_aupll_cp,
        [09..10] pu_aupll_sfreg,
        [10..11] pu_aupll,
        [11..12] pu_aupll_clktree,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg1: u32 {
        [00..07] aupll_postdiv,
        [08..12] aupll_refdiv_ratio,
        [16..18] aupll_refclk_sel,
        [20..22] aupll_vg11_sel,
        [24..26] aupll_vg13_sel,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg2: u32 {
        [00..01] aupll_sel_cp_bias,
        [04..06] aupll_icp_5u,
        [06..08] aupll_icp_1u,
        [08..09] aupll_int_frac_sw,
        [09..10] aupll_cp_startup_en,
        [10..11] aupll_cp_opamp_en,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg3: u32 {
        [00..01] aupll_c4_en,
        [04..06] aupll_r4,
        [08..09] aupll_r4_short,
        [12..14] aupll_c3,
        [14..16] aupll_cz,
        [16..19] aupll_rz,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg4: u32 {
        [00..02] aupll_sel_sample_clk,
        [04..06] aupll_sel_fb_clk,
        [08..09] aupll_sdmclk_sel,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg5: u32 {
        [00..03] aupll_vco_speed,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg6: u32 {
        [00..19] aupll_sdmin,
        [24..25] aupll_sdm_bypass,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg7: u32 {
        [00..01] aupll_sdm_order_sel,
        [16..18] aupll_sdm_sig_dith_sel,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg8: u32 {
        [00..01] aupll_en_div1,
        [01..02] aupll_en_div2,
        [02..03] aupll_en_div2p5,
        [03..04] aupll_en_div3,
        [04..05] aupll_en_div4,
        [05..06] aupll_en_div5,
        [06..07] aupll_en_div6,
        [07..08] aupll_en_div10,
        [08..09] aupll_en_div15,
        [09..10] aupll_sel_div1_div2,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg9: u32 {
        [00..01] aupll_dc_tp_out_en,
        [01..02] ten_aupll,
        [02..03] ten_aupll_sfreg,
        [04..05] dten_aupll_fin,
        [05..06] dten_aupll_fref,
        [06..07] dten_aupll_fsdm,
        [07..08] dten_aupll_div15,
        [08..09] dten_aupll_div5,
        [09..10] dten_aupll_postdiv_clk,
        [10..11] dtest_aupll_pulldown,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg10: u32 {
        [00..01] aupll_ssc_en,
        [04..12] aupll_ssc_cnt,
        [12..15] aupll_ssc_gain,
        [16..17] aupll_ssc_start_gate_en,
        [20..21] aupll_ssc_start,
    }
}

embedded_util::reg! {
    pub struct CciAudioPllCfg11: u32 {
        [00..16] aupll_resv,
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

embedded_util::reg! {
    pub struct CciCpuPllCfg0: u32 {
        [00..01] cpupll_sdm_rstb,
        [01..02] cpupll_postdiv_rstb,
        [02..03] cpupll_fbdv_rstb,
        [03..04] cpupll_refdiv_rstb,
        [04..05] pu_cpupll_postdiv,
        [05..06] pu_cpupll_fbdv,
        [06..07] pu_cpupll_clamp_op,
        [07..08] pu_cpupll_pfd,
        [08..09] pu_cpupll_cp,
        [09..10] pu_cpupll_sfreg,
        [10..11] pu_cpupll,
        [11..12] pu_cpupll_clktree,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg1: u32 {
        [00..07] cpupll_postdiv,
        [08..12] cpupll_refdiv_ratio,
        [16..18] cpupll_refclk_sel,
        [20..22] cpupll_vg11_sel,
        [24..26] cpupll_vg13_sel,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg2: u32 {
        [00..01] cpupll_sel_cp_bias,
        [04..06] cpupll_icp_5u,
        [06..08] cpupll_icp_1u,
        [08..09] cpupll_int_frac_sw,
        [09..10] cpupll_cp_startup_en,
        [10..11] cpupll_cp_opamp_en,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg3: u32 {
        [00..01] cpupll_c4_en,
        [04..06] cpupll_r4,
        [08..09] cpupll_r4_short,
        [12..14] cpupll_c3,
        [14..16] cpupll_cz,
        [16..19] cpupll_rz,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg4: u32 {
        [00..02] cpupll_sel_sample_clk,
        [04..06] cpupll_sel_fb_clk,
        [08..09] cpupll_sdmclk_sel,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg5: u32 {
        [00..03] cpupll_vco_speed,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg6: u32 {
        [00..19] cpupll_sdmin,
        [24..25] cpupll_sdm_bypass,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg7: u32 {
        [00..01] cpupll_sdm_order_sel,
        [16..18] cpupll_sdm_sig_dith_sel,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg8: u32 {
        [00..01] cpupll_en_div1,
        [01..02] cpupll_en_div2,
        [02..03] cpupll_en_div2p5,
        [03..04] cpupll_en_div3,
        [04..05] cpupll_en_div4,
        [05..06] cpupll_en_div5,
        [06..07] cpupll_en_div6,
        [07..08] cpupll_en_div10,
        [08..09] cpupll_en_div15,
        [09..10] cpupll_sel_div1_div2,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg9: u32 {
        [00..01] cpupll_dc_tp_out_en,
        [01..02] ten_cpupll,
        [02..03] ten_cpupll_sfreg,
        [04..05] dten_cpupll_fin,
        [05..06] dten_cpupll_fref,
        [06..07] dten_cpupll_fsdm,
        [07..08] dten_cpupll_div15,
        [08..09] dten_cpupll_div5,
        [09..10] dten_cpupll_postdiv_clk,
        [10..11] dtest_cpupll_pulldown,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg10: u32 {
        [00..01] cpupll_ssc_en,
        [04..12] cpupll_ssc_cnt,
        [12..15] cpupll_ssc_gain,
        [16..17] cpupll_ssc_start_gate_en,
        [20..21] cpupll_ssc_start,
    }
}

embedded_util::reg! {
    pub struct CciCpuPllCfg11: u32 {
        [00..16] cpupll_resv,
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
