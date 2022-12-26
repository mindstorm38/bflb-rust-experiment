//! 

emhal::mmio_struct! {
    pub struct Aon {
        [0x800] rw aon: Aon0,
        [0x804] rw common: AonCommon,
        [0x808] rw misc: AonMisc,
        [0x810] rw bg_sys_top: AonBgSysTop,
        [0x814] rw dcdc_top_0: AonDcdcTop0,
        [0x818] rw dcdc_top_1: AonDcdcTop1,
        [0x81C] rw ldo11soc_and_dctest: AonLdo11socAndDctest,
        [0x820] rw dcdc18_top_0: AonDcdc18Top0,
        [0x824] rw dcdc18_top_1: AonDcdc18Top1,
        [0x828] rw dcdc18_top_2: AonDcdc18Top2,
        [0x82C] rw psw_irrcv: AonPswIrrcv,
        [0x880] rw rf_top_aon: AonRfTopAon,
        [0x884] rw xtal_cfg: AonXtalCfg,
        [0x888] rw xtal_cfg2: AonXtalCfg2,
        [0x88C] rw xtal_cfg3: AonXtalCfg3,
        [0x890] rw tsen: AonTsen,
        [0x8C4] rw ldo18io: AonLdo18io,
        [0x900] rw acomp0_ctrl: AonAcomp0Ctrl,
        [0x904] rw acomp1_ctrl: AonAcomp1Ctrl,
        [0x908] rw acomp_ctrl: AonAcompCtrl,
        [0x90C] rw gpadc_reg_cmd: AonGpadcRegCmd,
        [0x910] rw gpadc_reg_config1: AonGpadcRegConfig1,
        [0x914] rw gpadc_reg_config2: AonGpadcRegConfig2,
        [0x918] rw gpadc_reg_scn_pos1: AonGpadcRegScnPos1,
        [0x91C] rw gpadc_reg_scn_pos2: AonGpadcRegScnPos2,
        [0x920] rw gpadc_reg_scn_neg1: AonGpadcRegScnNeg1,
        [0x924] rw gpadc_reg_scn_neg2: AonGpadcRegScnNeg2,
        [0x928] rw gpadc_reg_status: AonGpadcRegStatus,
        [0x92C] rw gpadc_reg_isr: AonGpadcRegIsr,
        [0x930] rw gpadc_reg_result: AonGpadcRegResult,
        [0x934] rw gpadc_reg_raw_result: AonGpadcRegRawResult,
        [0x938] rw gpadc_reg_define: AonGpadcRegDefine,
        [0x93C] rw hbncore_resv0: AonHbncoreResv0,
        [0x940] rw hbncore_resv1: AonHbncoreResv1,
    }
}

emhal::mmio_reg! {
    pub struct Aon0: u32 {
        [0..8] resv,
        [12..13] pu_aon_dc_tbuf,
        [20..21] ldo11_rt_pulldown,
        [21..22] ldo11_rt_pulldown_sel,
        [22..23] sw_pu_ldo11_rt,
    }
}

emhal::mmio_reg! {
    pub struct AonCommon: u32 {
        [0..3] tmux_aon,
        [3..4] pmip_dc_tp_out_en_aon,
        [4..5] ten_bg_sys_aon,
        [5..6] ten_dcdc11_0_aon,
        [6..7] ten_dcdc11_1_aon,
        [7..8] ten_dcdc18_0_aon,
        [8..9] ten_dcdc18_1_aon,
        [9..10] ten_ldo12uhs,
        [10..11] ten_ldo18flash,
        [11..12] ten_ldo15cis,
        [12..13] ten_ldo18io_aon,
        [13..14] ten_ldo28cis,
        [14..15] ten_rc32m,
        [16..17] ten_ldo15rf_aon,
        [17..18] ten_xtal_aon,
        [18..19] dten_xtal_aon,
        [19..20] ten_mbg_aon,
        [20..21] ten_cip_misc_aon,
        [21..22] ten_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonMisc: u32 {
        [0..1] sw_soc_en_aon,
        [1..2] sw_wb_en_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonBgSysTop: u32 {
        [0..1] pu_bg_sys_aon,
        [1..2] istart_ctrl_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonDcdcTop0: u32 {
        [0..2] dcdc11_sstart_time_aon,
        [4..7] dcdc11_stby_lp_cur_aon,
        [8..11] dcdc11_vc_clamp_vth_aon,
        [11..16] dcdc11_vout_sel_aon,
        [16..20] dcdc11_vout_trim_aon,
        [20..24] dcdc11_vpfm_aon,
        [24..27] dcdc11_zvs_td_opt_aon,
        [28..30] dcdc11_vstby_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonDcdcTop1: u32 {
        [0..5] dcdc11_nonoverlap_td_aon,
        [5..6] dcdc11_ocp_out_aon,
        [6..7] dcdc11_ocp_rst_aon,
        [8..11] dcdc11_ocp_vth_aon,
        [11..12] dcdc11_osc_2m_mode_aon,
        [12..16] dcdc11_osc_freq_trim_aon,
        [16..17] dcdc11_pulldown_aon,
        [20..24] dcdc11_rc_sel_aon,
        [24..25] dcdc11_rdy_aon,
        [26..31] dcdc11_slope_curr_sel_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonLdo11socAndDctest: u32 {
        [4..8] dcdc11_cfb_sel_aon,
        [8..12] dcdc11_chf_sel_aon,
        [12..15] dcdc11_comp_gm_sel_aon,
        [16..19] dcdc11_cs_delay_aon,
        [20..22] dcdc11_drv_sr_aon,
        [22..23] dcdc11_en_antiring_aon,
        [23..24] dcdc11_en_osc_inhibit_t2_aon,
        [24..25] dcdc11_en_slow_osc_aon,
        [25..26] dcdc11_en_stby_lp_aon,
        [26..27] dcdc11_en_stop_osc_aon,
        [27..28] dcdc11_force_en_cs_zvs_aon,
        [28..31] dcdc11_isense_trim_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonDcdc18Top0: u32 {
        [0..2] dcdc18_sstart_time_aon,
        [4..7] dcdc18_stby_lp_cur_aon,
        [8..11] dcdc18_vc_clamp_vth_aon,
        [11..16] dcdc18_vout_sel_aon,
        [16..20] dcdc18_vout_trim_aon,
        [20..24] dcdc18_vpfm_aon,
        [24..27] dcdc18_zvs_td_opt_aon,
        [28..30] dcdc18_vstby_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonDcdc18Top1: u32 {
        [0..5] dcdc18_nonoverlap_td_aon,
        [5..6] dcdc18_ocp_out_aon,
        [6..7] dcdc18_ocp_rst_aon,
        [8..11] dcdc18_ocp_vth_aon,
        [11..12] dcdc18_osc_2m_mode_aon,
        [12..16] dcdc18_osc_freq_trim_aon,
        [16..17] dcdc18_pulldown_aon,
        [20..24] dcdc18_rc_sel_aon,
        [24..25] dcdc18_rdy_aon,
        [26..31] dcdc18_slope_curr_sel_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonDcdc18Top2: u32 {
        [4..8] dcdc18_cfb_sel_aon,
        [8..12] dcdc18_chf_sel_aon,
        [12..15] dcdc18_comp_gm_sel_aon,
        [16..19] dcdc18_cs_delay_aon,
        [20..22] dcdc18_drv_sr_aon,
        [22..23] dcdc18_en_antiring_aon,
        [23..24] dcdc18_en_osc_inhibit_t2_aon,
        [24..25] dcdc18_en_slow_osc_aon,
        [25..26] dcdc18_en_stby_lp_aon,
        [26..27] dcdc18_en_stop_osc_aon,
        [27..28] dcdc18_force_en_cs_zvs_aon,
        [28..31] dcdc18_isense_trim_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonPswIrrcv: u32 {
        [0..1] pu_psw_irrcv_aon,
        [19..20] usb20_rref_ext_en_aon,
        [20..21] en_por33_aon,
        [21..22] usb20_rref_hiz_aon,
        [24..30] usb20_rcal_code_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonRfTopAon: u32 {
        [0..1] pu_mbg_aon,
        [1..2] pu_ldo15rf_aon,
        [2..3] pu_sfreg_aon,
        [4..5] pu_xtal_buf_aon,
        [5..6] pu_xtal_aon,
        [8..9] ldo15rf_sstart_sel_aon,
        [9..11] ldo15rf_sstart_delay_aon,
        [12..13] ldo15rf_pulldown_aon,
        [13..14] ldo15rf_pulldown_sel_aon,
        [16..19] ldo15rf_vout_sel_aon,
        [24..26] ldo15rf_cc_aon,
        [28..29] ldo15rf_bypass_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonXtalCfg: u32 {
        [0..2] xtal_bk_aon,
        [2..3] xtal_capcode_extra_aon,
        [3..4] xtal_ext_sel_aon,
        [4..8] xtal_buf_en_aon,
        [8..12] xtal_buf_hp_aon,
        [12..13] xtal_fast_startup_aon,
        [13..14] xtal_sleep_aon,
        [14..16] xtal_amp_ctrl_aon,
        [16..22] xtal_capcode_out_aon,
        [22..28] xtal_capcode_in_aon,
        [28..30] xtal_gm_boost_aon,
        [30..32] xtal_rdy_sel_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonXtalCfg2: u32 {
        [0..1] wifi_xtal_ldo33_bypass_aon,
        [1..4] wifi_xtal_ldo33_sel_aon,
        [4..6] wifi_xtal_ldo18_sel_aon,
        [6..7] wifi_xtal_ldo33_pu_aon,
        [7..8] wifi_xtal_ldo18_pu_aon,
        [10..14] wifi_xtal_reserve,
        [16..17] wifi_xtal_ldo18_short_filter_aon,
        [30..32] xtal_buf_drv_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonXtalCfg3: u32 {
        [12..13] wifi_xtal_clk_inv_en_aon,
        [16..17] wifi_xtal_cml_en_aon,
        [17..19] wifi_xtal_cml_r_sel_aon,
        [20..21] wifi_xtal_clk_en_aon,
        [30..32] wifi_xtal_buf_drv_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonTsen: u32 {
        [0..12] tsen_refcode_corner,
        [16..28] tsen_refcode_rfcal,
        [28..29] xtal_rdy,
        [29..30] xtal_inn_cfg_en_aon,
        [30..32] xtal_rdy_int_sel_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonLdo18io: u32 {
        [1..2] ldo18io_bypass_iso_aon,
        [2..3] ldo18io_pulldown_aon,
        [3..4] ldo18io_pulldown_sel_aon,
        [4..7] ldo18io_bm_aon,
        [8..11] ldo18io_cc_aon,
        [11..12] ldo18io_ocp_out_aon,
        [12..15] ldo18io_ocp_th_aon,
        [15..16] ldo18io_ocp_en_aon,
        [16..19] ldo18io_sstart_delay_aon,
        [19..20] ldo18io_sstart_en_aon,
        [20..24] ldo18io_vout_sel_aon,
        [24..28] ldo18io_vout_trim_aon,
    }
}

emhal::mmio_reg! {
    pub struct AonAcomp0Ctrl: u32 {
        [0..1] acomp0_en,
        [4..7] acomp0_hyst_seln,
        [7..10] acomp0_hyst_selp,
        [10..12] acomp0_bias_prog,
        [12..18] acomp0_level_sel,
        [18..22] acomp0_neg_sel,
        [22..26] acomp0_pos_sel,
        [26..27] acomp0_muxen,
    }
}

emhal::mmio_reg! {
    pub struct AonAcomp1Ctrl: u32 {
        [0..1] acomp1_en,
        [4..7] acomp1_hyst_seln,
        [7..10] acomp1_hyst_selp,
        [10..12] acomp1_bias_prog,
        [12..18] acomp1_level_sel,
        [18..22] acomp1_neg_sel,
        [22..26] acomp1_pos_sel,
        [26..27] acomp1_muxen,
    }
}

emhal::mmio_reg! {
    pub struct AonAcompCtrl: u32 {
        [0..1] acomp1_rstn_ana,
        [1..2] acomp0_rstn_ana,
        [8..9] acomp1_test_en,
        [9..10] acomp0_test_en,
        [10..12] acomp1_test_sel,
        [12..14] acomp0_test_sel,
        [17..18] acomp1_out_raw,
        [19..20] acomp0_out_raw,
        [24..30] acomp_vref_sel,
        [30..32] acomp_reserved,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegCmd: u32 {
        [0..1] gpadc_global_en,
        [1..2] gpadc_conv_start,
        [2..3] gpadc_soft_rst,
        [3..8] gpadc_neg_sel,
        [8..13] gpadc_pos_sel,
        [13..14] gpadc_neg_gnd,
        [14..15] gpadc_micbias_en,
        [15..16] gpadc_micpga_en,
        [16..17] gpadc_byp_micboost,
        [17..18] gpadc_rcal_en,
        [18..19] gpadc_dwa_en,
        [19..20] gpadc_mic2_diff,
        [20..21] gpadc_mic1_diff,
        [21..23] gpadc_mic_pga2_gain,
        [23..24] gpadc_micboost_32db_en,
        [27..28] gpadc_chip_sen_pu,
        [28..31] gpadc_sen_sel,
        [31..32] gpadc_sen_test_en,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegConfig1: u32 {
        [0..1] gpadc_cal_os_en,
        [1..2] gpadc_cont_conv_en,
        [2..5] gpadc_res_sel,
        [8..9] gpadc_vcm_sel_en,
        [9..10] gpadc_vcm_hyst_sel,
        [10..11] gpadc_lowv_det_en,
        [11..12] gpadc_pwm_trg_en,
        [12..16] gpadc_clk_ana_dly,
        [16..17] gpadc_clk_ana_dly_en,
        [17..18] gpadc_clk_ana_inv,
        [18..21] gpadc_clk_div_ratio,
        [21..25] gpadc_scan_length,
        [25..26] gpadc_scan_en,
        [26..27] gpadc_dither_en,
        [27..29] gpadc_v11_sel,
        [29..31] gpadc_v18_sel,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegConfig2: u32 {
        [2..3] gpadc_diff_mode,
        [3..4] gpadc_vref_sel,
        [4..5] gpadc_vbat_en,
        [5..6] gpadc_tsext_sel,
        [6..7] gpadc_ts_en,
        [7..9] gpadc_pga_vcm,
        [9..13] gpadc_pga_os_cal,
        [13..14] gpadc_pga_en,
        [14..15] gpadc_pga_vcmi_en,
        [15..17] gpadc_chop_mode,
        [17..18] gpadc_bias_sel,
        [18..19] gpadc_test_en,
        [19..22] gpadc_test_sel,
        [22..25] gpadc_pga2_gain,
        [25..28] gpadc_pga1_gain,
        [28..31] gpadc_dly_sel,
        [31..32] gpadc_tsvbe_low,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegScnPos1: u32 {
        [0..5] gpadc_scan_pos_0,
        [5..10] gpadc_scan_pos_1,
        [10..15] gpadc_scan_pos_2,
        [15..20] gpadc_scan_pos_3,
        [20..25] gpadc_scan_pos_4,
        [25..30] gpadc_scan_pos_5,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegScnPos2: u32 {
        [0..5] gpadc_scan_pos_6,
        [5..10] gpadc_scan_pos_7,
        [10..15] gpadc_scan_pos_8,
        [15..20] gpadc_scan_pos_9,
        [20..25] gpadc_scan_pos_10,
        [25..30] gpadc_scan_pos_11,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegScnNeg1: u32 {
        [0..5] gpadc_scan_neg_0,
        [5..10] gpadc_scan_neg_1,
        [10..15] gpadc_scan_neg_2,
        [15..20] gpadc_scan_neg_3,
        [20..25] gpadc_scan_neg_4,
        [25..30] gpadc_scan_neg_5,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegScnNeg2: u32 {
        [0..5] gpadc_scan_neg_6,
        [5..10] gpadc_scan_neg_7,
        [10..15] gpadc_scan_neg_8,
        [15..20] gpadc_scan_neg_9,
        [20..25] gpadc_scan_neg_10,
        [25..30] gpadc_scan_neg_11,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegStatus: u32 {
        [0..1] gpadc_data_rdy,
        [16..32] gpadc_reserved,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegIsr: u32 {
        [0..1] gpadc_neg_satur,
        [1..2] gpadc_pos_satur,
        [4..5] gpadc_neg_satur_clr,
        [5..6] gpadc_pos_satur_clr,
        [8..9] gpadc_neg_satur_mask,
        [9..10] gpadc_pos_satur_mask,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegResult: u32 {
        [0..26] gpadc_data_out,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegRawResult: u32 {
        [0..12] gpadc_raw_data,
    }
}

emhal::mmio_reg! {
    pub struct AonGpadcRegDefine: u32 {
        [0..16] gpadc_os_cal_data,
    }
}

emhal::mmio_reg! {
    pub struct AonHbncoreResv0: u32 {
        [0..32] hbncore_resv0_data,
    }
}

emhal::mmio_reg! {
    pub struct AonHbncoreResv1: u32 {
        [0..32] hbncore_resv1_data,
    }
}
