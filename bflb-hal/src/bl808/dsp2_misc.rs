//! DSP2 misc.

embedded_util::mmio! {
    pub struct Dsp2Misc {
        [0x000] rw config: Dsp2MiscConfig,
        [0x004] rw pix_data_ctrl: Dsp2MiscPixDataCtrl,
        [0x008] rw dvp2bus_src_sel_1: Dsp2MiscDvp2busSrcSel1,
        [0x00C] rw dvp_frame_m_to_n: Dsp2MiscDvpFrameMToN,
        [0x010] rw dsp2_id_bayer: Dsp2MiscDsp2IdBayer,
        [0x014] rw dvp2bus_src_sel_2: Dsp2MiscDvp2busSrcSel2,
        [0x020] rw int_clr: Dsp2MiscIntClr,
        [0x024] rw int_ctrl: Dsp2MiscIntCtrl,
        [0x028] rw dsp2_id_yuv: Dsp2MiscDsp2IdYuv,
        [0x02C] rw dvp_reshape: Dsp2MiscDvpReshape,
        [0x040] rw scalera_i_size: Dsp2MiscScaleraISize,
        [0x044] rw scalera_o_size: Dsp2MiscScaleraOSize,
        [0x048] rw scalerb_i_size: Dsp2MiscScalerbISize,
        [0x04C] rw scalerb_o_size: Dsp2MiscScalerbOSize,
        [0x050] rw scalerc_i_size: Dsp2MiscScalercISize,
        [0x054] rw scalerc_o_size: Dsp2MiscScalercOSize,
        [0x058] rw scalerd_i_size: Dsp2MiscScalerdISize,
        [0x05C] rw scalerd_o_size: Dsp2MiscScalerdOSize,
        [0x060] rw disp_config: Dsp2MiscDispConfig,
        [0x064] rw disp_dpi_config: Dsp2MiscDispDpiConfig,
        [0x068] rw disp_yuv_rgb_config_0: Dsp2MiscDispYuvRgbConfig0,
        [0x06C] rw disp_yuv_rgb_config_1: Dsp2MiscDispYuvRgbConfig1,
        [0x070] rw disp_yuv_rgb_config_2: Dsp2MiscDispYuvRgbConfig2,
        [0x074] rw disp_yuv_rgb_config_3: Dsp2MiscDispYuvRgbConfig3,
        [0x078] rw disp_yuv_rgb_config_4: Dsp2MiscDispYuvRgbConfig4,
        [0x07C] rw disp_yuv_rgb_config_5: Dsp2MiscDispYuvRgbConfig5,
        [0x080] rw dsp2_subsys_bus_ctrl: Dsp2MiscDsp2SubsysBusCtrl,
        [0x0A0] rw osda_i_ctrl: Dsp2MiscOsdaICtrl,
        [0x0A4] rw osdb_i_ctrl: Dsp2MiscOsdbICtrl,
        [0x0A8] rw osdc_i_ctrl: Dsp2MiscOsdcICtrl,
        [0x0AC] rw osdd_i_ctrl: Dsp2MiscOsddICtrl,
        [0x0C0] rw dsp2_subsys_dbg_sel: Dsp2MiscDsp2SubsysDbgSel,
        [0x0FC] rw dummy: Dsp2MiscDummy,
        [0x100] rw adja_ctrl_0: Dsp2MiscAdjaCtrl0,
        [0x104] rw adja_ctrl_1: Dsp2MiscAdjaCtrl1,
        [0x108] rw adja_ctrl_2: Dsp2MiscAdjaCtrl2,
        [0x110] rw adjb_ctrl_0: Dsp2MiscAdjbCtrl0,
        [0x114] rw adjb_ctrl_1: Dsp2MiscAdjbCtrl1,
        [0x118] rw adjb_ctrl_2: Dsp2MiscAdjbCtrl2,
        [0x120] rw adjc_ctrl_0: Dsp2MiscAdjcCtrl0,
        [0x124] rw adjc_ctrl_1: Dsp2MiscAdjcCtrl1,
        [0x128] rw adjc_ctrl_2: Dsp2MiscAdjcCtrl2,
        [0x130] rw adjd_ctrl_0: Dsp2MiscAdjdCtrl0,
        [0x134] rw adjd_ctrl_1: Dsp2MiscAdjdCtrl1,
        [0x138] rw adjd_ctrl_2: Dsp2MiscAdjdCtrl2,
        [0x160] rw y2ra_config_0: Dsp2MiscY2raConfig0,
        [0x164] rw y2ra_config_1: Dsp2MiscY2raConfig1,
        [0x168] rw y2ra_config_2: Dsp2MiscY2raConfig2,
        [0x16C] rw y2ra_config_3: Dsp2MiscY2raConfig3,
        [0x170] rw y2ra_config_4: Dsp2MiscY2raConfig4,
        [0x174] rw y2ra_config_5: Dsp2MiscY2raConfig5,
        [0x178] rw y2ra_config_6: Dsp2MiscY2raConfig6,
        [0x17C] rw y2ra_config_7: Dsp2MiscY2raConfig7,
        [0x180] rw disp_rgb2yuv_config_0: Dsp2MiscDispRgb2yuvConfig0,
        [0x184] rw disp_rgb2yuv_config_1: Dsp2MiscDispRgb2yuvConfig1,
        [0x188] rw disp_rgb2yuv_config_2: Dsp2MiscDispRgb2yuvConfig2,
        [0x18C] rw disp_rgb2yuv_config_3: Dsp2MiscDispRgb2yuvConfig3,
        [0x190] rw disp_rgb2yuv_config_4: Dsp2MiscDispRgb2yuvConfig4,
        [0x194] rw disp_rgb2yuv_config_5: Dsp2MiscDispRgb2yuvConfig5,
        [0x1C0] rw cropa_hsync: Dsp2MiscCropaHsync,
        [0x1C4] rw cropa_vsync: Dsp2MiscCropaVsync,
        [0x1C8] rw cropb_hsync: Dsp2MiscCropbHsync,
        [0x1CC] rw cropb_vsync: Dsp2MiscCropbVsync,
        [0x1D0] rw cropc_hsync: Dsp2MiscCropcHsync,
        [0x1D4] rw cropc_vsync: Dsp2MiscCropcVsync,
        [0x1D8] rw cropd_hsync: Dsp2MiscCropdHsync,
        [0x1DC] rw cropd_vsync: Dsp2MiscCropdVsync,
        [0x1E0] rw crop_enable: Dsp2MiscCropEnable,
        [0x1FC] rw disp_gma_cfg: Dsp2MiscDispGmaCfg,
        [0x200] rw disp_gma_curve_00: Dsp2MiscDispGmaCurve00,
        [0x204] rw disp_gma_curve_04: Dsp2MiscDispGmaCurve04,
        [0x208] rw disp_gma_curve_08: Dsp2MiscDispGmaCurve08,
        [0x20C] rw disp_gma_curve_0c: Dsp2MiscDispGmaCurve0c,
        [0x210] rw disp_gma_curve_10: Dsp2MiscDispGmaCurve10,
        [0x214] rw disp_gma_curve_14: Dsp2MiscDispGmaCurve14,
        [0x218] rw disp_gma_curve_18: Dsp2MiscDispGmaCurve18,
        [0x21C] rw disp_gma_curve_1c: Dsp2MiscDispGmaCurve1c,
        [0x220] rw disp_gma_curve_20: Dsp2MiscDispGmaCurve20,
        [0x224] rw disp_gma_curve_24: Dsp2MiscDispGmaCurve24,
        [0x228] rw disp_gma_curve_28: Dsp2MiscDispGmaCurve28,
        [0x22C] rw disp_gma_curve_2c: Dsp2MiscDispGmaCurve2c,
        [0x230] rw disp_gma_curve_30: Dsp2MiscDispGmaCurve30,
        [0x234] rw disp_gma_curve_34: Dsp2MiscDispGmaCurve34,
        [0x238] rw disp_gma_curve_38: Dsp2MiscDispGmaCurve38,
        [0x23C] rw disp_gma_curve_3c: Dsp2MiscDispGmaCurve3c,
        [0x240] rw disp_gma_curve_40: Dsp2MiscDispGmaCurve40,
        [0x244] rw disp_gma_curve_44: Dsp2MiscDispGmaCurve44,
        [0x248] rw disp_gma_curve_48: Dsp2MiscDispGmaCurve48,
        [0x24C] rw disp_gma_curve_4c: Dsp2MiscDispGmaCurve4c,
        [0x250] rw disp_gma_curve_50: Dsp2MiscDispGmaCurve50,
        [0x254] rw disp_gma_curve_54: Dsp2MiscDispGmaCurve54,
        [0x258] rw disp_gma_curve_58: Dsp2MiscDispGmaCurve58,
        [0x25C] rw disp_gma_curve_5c: Dsp2MiscDispGmaCurve5c,
        [0x260] rw disp_gma_curve_60: Dsp2MiscDispGmaCurve60,
        [0x264] rw disp_gma_curve_64: Dsp2MiscDispGmaCurve64,
        [0x268] rw disp_gma_curve_68: Dsp2MiscDispGmaCurve68,
        [0x26C] rw disp_gma_curve_6c: Dsp2MiscDispGmaCurve6c,
        [0x270] rw disp_gma_curve_70: Dsp2MiscDispGmaCurve70,
        [0x274] rw disp_gma_curve_74: Dsp2MiscDispGmaCurve74,
        [0x278] rw disp_gma_curve_78: Dsp2MiscDispGmaCurve78,
        [0x27C] rw disp_gma_curve_7c: Dsp2MiscDispGmaCurve7c,
        [0x280] rw disp_gma_curve_80: Dsp2MiscDispGmaCurve80,
        [0x284] rw disp_gma_curve_84: Dsp2MiscDispGmaCurve84,
        [0x288] rw disp_gma_curve_88: Dsp2MiscDispGmaCurve88,
        [0x28C] rw disp_gma_curve_8c: Dsp2MiscDispGmaCurve8c,
        [0x290] rw disp_gma_curve_90: Dsp2MiscDispGmaCurve90,
        [0x294] rw disp_gma_curve_94: Dsp2MiscDispGmaCurve94,
        [0x298] rw disp_gma_curve_98: Dsp2MiscDispGmaCurve98,
        [0x29C] rw disp_gma_curve_9c: Dsp2MiscDispGmaCurve9c,
        [0x2A0] rw disp_gma_curve_a0: Dsp2MiscDispGmaCurveA0,
        [0x2A4] rw disp_gma_curve_a4: Dsp2MiscDispGmaCurveA4,
        [0x2A8] rw disp_gma_curve_a8: Dsp2MiscDispGmaCurveA8,
        [0x2AC] rw disp_gma_curve_ac: Dsp2MiscDispGmaCurveAc,
        [0x2B0] rw disp_gma_curve_b0: Dsp2MiscDispGmaCurveB0,
        [0x2B4] rw disp_gma_curve_b4: Dsp2MiscDispGmaCurveB4,
        [0x2B8] rw disp_gma_curve_b8: Dsp2MiscDispGmaCurveB8,
        [0x2BC] rw disp_gma_curve_bc: Dsp2MiscDispGmaCurveBc,
        [0x2C0] rw disp_gma_curve_c0: Dsp2MiscDispGmaCurveC0,
        [0x2C4] rw disp_gma_curve_c4: Dsp2MiscDispGmaCurveC4,
        [0x2C8] rw disp_gma_curve_c8: Dsp2MiscDispGmaCurveC8,
        [0x2CC] rw disp_gma_curve_cc: Dsp2MiscDispGmaCurveCc,
        [0x2D0] rw disp_gma_curve_d0: Dsp2MiscDispGmaCurveD0,
        [0x2D4] rw disp_gma_curve_d4: Dsp2MiscDispGmaCurveD4,
        [0x2D8] rw disp_gma_curve_d8: Dsp2MiscDispGmaCurveD8,
        [0x2DC] rw disp_gma_curve_dc: Dsp2MiscDispGmaCurveDc,
        [0x2E0] rw disp_gma_curve_e0: Dsp2MiscDispGmaCurveE0,
        [0x2E4] rw disp_gma_curve_e4: Dsp2MiscDispGmaCurveE4,
        [0x2E8] rw disp_gma_curve_e8: Dsp2MiscDispGmaCurveE8,
        [0x2EC] rw disp_gma_curve_ec: Dsp2MiscDispGmaCurveEc,
        [0x2F0] rw disp_gma_curve_f0: Dsp2MiscDispGmaCurveF0,
        [0x2F4] rw disp_gma_curve_f4: Dsp2MiscDispGmaCurveF4,
        [0x2F8] rw disp_gma_curve_f8: Dsp2MiscDispGmaCurveF8,
        [0x2FC] rw disp_gma_curve_fc: Dsp2MiscDispGmaCurveFc,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscConfig: u32 {
        [00..01] rg_dvpas_enable,
        [01..02] rg_dvpas_hs_inv,
        [02..03] rg_dvpas_vs_inv,
        [03..04] rg_dvpas_da_order,
        [16..27] rg_dvpas_fifo_th,
        [30..31] cr_dsp2_de_as_hsync,
        [31..32] dsp2_pclk_force_on,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscPixDataCtrl: u32 {
        [00..12] pix_data_ctrl,
        [16..20] pix_data_sht_bit,
        [20..21] pix_data_sht_dir,
        [31..32] dsp2_dtsrc_src,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDvp2busSrcSel1: u32 {
        [00..06] rg_d2b_dvp_sel_a,
        [07..08] rg_d2x_id_sel_a,
        [08..14] rg_d2b_dvp_sel_b,
        [15..16] rg_d2x_id_sel_b,
        [16..22] rg_d2b_dvp_sel_c,
        [23..24] rg_d2x_id_sel_c,
        [24..30] rg_d2b_dvp_sel_d,
        [31..32] rg_d2x_id_sel_d,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDvpFrameMToN: u32 {
        [00..01] cr_frame_m_to_n_en,
        [04..08] cr_frame_interval,
        [08..12] cr_frame_cnt_n,
        [12..16] cr_frame_cnt_m,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDsp2IdBayer: u32 {
        [00..01] bayer_idgen_rst,
        [01..02] bayer_idgen_edge,
        [16..32] bayer_idgen_cnt_incr,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDvp2busSrcSel2: u32 {
        [00..06] rg_d2b_dvp_sel_e,
        [07..08] rg_d2x_id_sel_e,
        [08..14] rg_d2b_dvp_sel_f,
        [15..16] rg_d2x_id_sel_f,
        [16..22] rg_d2b_dvp_sel_g,
        [23..24] rg_d2x_id_sel_g,
        [24..30] rg_d2b_dvp_sel_h,
        [31..32] rg_d2x_id_sel_h,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscIntClr: u32 {
        [00..01] rg_awb_int_clr,
        [01..02] rg_ae_int_clr,
        [02..03] rg_wdr_int_clr,
        [03..04] rg_awb2_int_clr,
        [04..05] rg_osd_pb_int_clr,
        [08..09] rg_seof1_int_clr,
        [09..10] rg_seof2_int_clr,
        [10..11] rg_seof3_int_clr,
        [11..12] rg_ae_hist_int_clr,
        [12..13] rg_awb3_int_clr,
        [13..14] rg_dp_seof0_int_clr,
        [14..15] rg_seof4_int_clr,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscIntCtrl: u32 {
        [00..01] rg_ae_int_mask,
        [01..02] rg_awb_int_mask,
        [02..03] rg_wdr_int_mask,
        [03..04] rg_awb2_int_mask,
        [04..05] rg_osd_pb_int_mask,
        [08..09] rg_seof1_int_mask,
        [09..10] rg_seof1_int_edge,
        [10..12] rg_seof1_int_src,
        [12..13] rg_seof2_int_mask,
        [13..14] rg_seof2_int_edge,
        [14..16] rg_seof2_int_src,
        [16..17] rg_seof3_int_mask,
        [17..18] rg_seof3_int_edge,
        [18..20] rg_seof3_int_src,
        [20..21] rg_ae_hist_int_mask,
        [21..22] rg_awb3_int_mask,
        [22..23] rg_dp_seof0_int_src,
        [23..24] rg_dp_seof0_int_edge,
        [24..25] rg_dp_seof0_int_mask,
        [28..29] rg_seof4_int_mask,
        [29..30] rg_seof4_int_edge,
        [30..32] rg_seof4_int_src,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDsp2IdYuv: u32 {
        [00..01] yuv_idgen_rst,
        [01..02] yuv_idgen_edge,
        [16..32] yuv_idgen_cnt_incr,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDvpReshape: u32 {
        [00..05] rshp_tgl_count,
        [05..06] rshp_hsync_inv,
        [06..07] rshp_vsync_inv,
        [07..08] rshp_clr,
        [08..09] rshp_en,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScaleraISize: u32 {
        [00..11] rg_scalera_i_w,
        [16..27] rg_scalera_i_h,
        [27..28] rg_sclra_sw_sh,
        [28..29] rg_scalera_bypass,
        [29..32] rg_scalera_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScaleraOSize: u32 {
        [00..11] rg_scalera_o_w,
        [16..27] rg_scalera_o_h,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScalerbISize: u32 {
        [00..11] rg_scalerb_i_w,
        [16..27] rg_scalerb_i_h,
        [27..28] rg_sclrb_sw_sh,
        [28..29] rg_scalerb_bypass,
        [29..32] rg_scalerb_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScalerbOSize: u32 {
        [00..11] rg_scalerb_o_w,
        [16..27] rg_scalerb_o_h,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScalercISize: u32 {
        [00..11] rg_scalerc_i_w,
        [16..27] rg_scalerc_i_h,
        [27..28] rg_sclrc_sw_sh,
        [28..29] rg_scalerc_bypass,
        [29..32] rg_scalerc_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScalercOSize: u32 {
        [00..11] rg_scalerc_o_w,
        [16..27] rg_scalerc_o_h,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScalerdISize: u32 {
        [00..11] rg_scalerd_i_w,
        [16..27] rg_scalerd_i_h,
        [27..28] rg_sclrd_sw_sh,
        [28..29] rg_scalerd_bypass,
        [29..32] rg_scalerd_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscScalerdOSize: u32 {
        [00..11] rg_scalerd_o_w,
        [16..27] rg_scalerd_o_h,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispConfig: u32 {
        [04..05] rg_disp_bt656_en,
        [05..06] rg_disp_bt1120_en,
        [06..07] rg_disp_dpi_en,
        [07..08] rg_disp_hdmi_en,
        [08..10] rg_disp_mux_sel,
        [12..14] rg_osddp_sel,
        [31..32] rg_disp_clko_inv,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispDpiConfig: u32 {
        [00..08] rg_disp_dpi_hs_w,
        [08..16] rg_disp_dpi_hfp_w,
        [16..24] rg_disp_dpi_vs_w,
        [24..32] rg_disp_dpi_vfp_w,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispYuvRgbConfig0: u32 {
        [00..09] rg_disp_y2r_pre_0,
        [10..19] rg_disp_y2r_pre_1,
        [20..29] rg_disp_y2r_pre_2,
        [31..32] rg_disp_y2r_en,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispYuvRgbConfig1: u32 {
        [00..09] rg_disp_y2r_pos_0,
        [10..19] rg_disp_y2r_pos_1,
        [20..29] rg_disp_y2r_pos_2,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispYuvRgbConfig2: u32 {
        [00..12] rg_disp_y2r_mtx_00,
        [12..24] rg_disp_y2r_mtx_01,
        [24..32] rg_disp_y2r_mtx_02_l,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispYuvRgbConfig3: u32 {
        [00..04] rg_disp_y2r_mtx_02_u,
        [04..16] rg_disp_y2r_mtx_10,
        [16..28] rg_disp_y2r_mtx_11,
        [28..32] rg_disp_y2r_mtx_12_l,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispYuvRgbConfig4: u32 {
        [00..08] rg_disp_y2r_mtx_12_u,
        [08..20] rg_disp_y2r_mtx_20,
        [20..32] rg_disp_y2r_mtx_21,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispYuvRgbConfig5: u32 {
        [00..12] rg_disp_y2r_mtx_22,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDsp2SubsysBusCtrl: u32 {
        [00..16] rg_dsp2_sub_pclk_force_on,
        [16..32] rg_dsp2_peri_pclk_force_on,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscOsdaICtrl: u32 {
        [00..01] rg_osd_pb_sel,
        [29..32] rg_osda_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscOsdbICtrl: u32 {
        [29..32] rg_osdb_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscOsdcICtrl: u32 {
        [29..32] rg_osdc_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscOsddICtrl: u32 {
        [29..32] rg_osdd_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDsp2SubsysDbgSel: u32 {
        [00..01] dsp2_sub_dbg_en,
        [04..08] dsp2_sub_dbg_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDummy: u32 {
        [00..32] dummy_reg,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjaCtrl0: u32 {
        [00..01] adja_adj_eb,
        [01..10] adja_y_luma,
        [10..21] adja_y_mul0,
        [21..32] adja_y_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjaCtrl1: u32 {
        [00..08] adja_y_min,
        [08..16] adja_y_max,
        [16..24] adja_uv_min,
        [24..32] adja_uv_max,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjaCtrl2: u32 {
        [00..04] rg_adja_sel,
        [10..21] adja_uv_mul0,
        [21..32] adja_uv_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjbCtrl0: u32 {
        [00..01] adjb_adj_eb,
        [01..10] adjb_y_luma,
        [10..21] adjb_y_mul0,
        [21..32] adjb_y_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjbCtrl1: u32 {
        [00..08] adjb_y_min,
        [08..16] adjb_y_max,
        [16..24] adjb_uv_min,
        [24..32] adjb_uv_max,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjbCtrl2: u32 {
        [00..04] rg_adjb_sel,
        [10..21] adjb_uv_mul0,
        [21..32] adjb_uv_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjcCtrl0: u32 {
        [00..01] adjc_adj_eb,
        [01..10] adjc_y_luma,
        [10..21] adjc_y_mul0,
        [21..32] adjc_y_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjcCtrl1: u32 {
        [00..08] adjc_y_min,
        [08..16] adjc_y_max,
        [16..24] adjc_uv_min,
        [24..32] adjc_uv_max,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjcCtrl2: u32 {
        [00..04] rg_adjc_sel,
        [10..21] adjc_uv_mul0,
        [21..32] adjc_uv_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjdCtrl0: u32 {
        [00..01] adjd_adj_eb,
        [01..10] adjd_y_luma,
        [10..21] adjd_y_mul0,
        [21..32] adjd_y_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjdCtrl1: u32 {
        [00..08] adjd_y_min,
        [08..16] adjd_y_max,
        [16..24] adjd_uv_min,
        [24..32] adjd_uv_max,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscAdjdCtrl2: u32 {
        [00..04] rg_adjd_sel,
        [10..21] adjd_uv_mul0,
        [21..32] adjd_uv_mul1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig0: u32 {
        [00..09] rg_y2ra_pre_0,
        [16..25] rg_y2ra_pos_0,
        [27..28] rg_y2ra_en,
        [28..32] rg_y2ra_sel,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig1: u32 {
        [00..09] rg_y2ra_pre_1,
        [16..25] rg_y2ra_pos_1,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig2: u32 {
        [00..09] rg_y2ra_pre_2,
        [16..25] rg_y2ra_pos_2,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig3: u32 {
        [00..12] rg_y2ra_mtx_00,
        [16..28] rg_y2ra_mtx_01,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig4: u32 {
        [00..12] rg_y2ra_mtx_02,
        [16..28] rg_y2ra_mtx_10,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig5: u32 {
        [00..12] rg_y2ra_mtx_11,
        [16..28] rg_y2ra_mtx_12,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig6: u32 {
        [00..12] rg_y2ra_mtx_20,
        [16..28] rg_y2ra_mtx_21,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscY2raConfig7: u32 {
        [00..12] rg_y2ra_mtx_22,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispRgb2yuvConfig0: u32 {
        [00..09] rg_disp_r2y_pre_0,
        [10..19] rg_disp_r2y_pre_1,
        [20..29] rg_disp_r2y_pre_2,
        [31..32] rg_disp_r2y_en,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispRgb2yuvConfig1: u32 {
        [00..09] rg_disp_r2y_pos_0,
        [10..19] rg_disp_r2y_pos_1,
        [20..29] rg_disp_r2y_pos_2,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispRgb2yuvConfig2: u32 {
        [00..12] rg_disp_r2y_mtx_00,
        [12..24] rg_disp_r2y_mtx_01,
        [24..32] rg_disp_r2y_mtx_02_l,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispRgb2yuvConfig3: u32 {
        [00..04] rg_disp_r2y_mtx_02_u,
        [04..16] rg_disp_r2y_mtx_10,
        [16..28] rg_disp_r2y_mtx_11,
        [28..32] rg_disp_r2y_mtx_12_l,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispRgb2yuvConfig4: u32 {
        [00..08] rg_disp_r2y_mtx_12_u,
        [08..20] rg_disp_r2y_mtx_20,
        [20..32] rg_disp_r2y_mtx_21,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispRgb2yuvConfig5: u32 {
        [00..12] rg_disp_r2y_mtx_22,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropaHsync: u32 {
        [00..16] cropa_hsync_start,
        [16..32] cropa_hsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropaVsync: u32 {
        [00..16] cropa_vsync_start,
        [16..32] cropa_vsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropbHsync: u32 {
        [00..16] cropb_hsync_start,
        [16..32] cropb_hsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropbVsync: u32 {
        [00..16] cropb_vsync_start,
        [16..32] cropb_vsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropcHsync: u32 {
        [00..16] cropc_hsync_start,
        [16..32] cropc_hsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropcVsync: u32 {
        [00..16] cropc_vsync_start,
        [16..32] cropc_vsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropdHsync: u32 {
        [00..16] cropd_hsync_start,
        [16..32] cropd_hsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropdVsync: u32 {
        [00..16] cropd_vsync_start,
        [16..32] cropd_vsync_end,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscCropEnable: u32 {
        [00..01] cropa_enable,
        [01..02] cropb_enable,
        [02..03] cropc_enable,
        [03..04] cropd_enable,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCfg: u32 {
        [00..01] dp_gma_ch0_en,
        [01..02] dp_gma_ch1_en,
        [02..03] dp_gma_ch2_en,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve00: u32 {
        [00..08] dp_gma_curve_00,
        [08..16] dp_gma_curve_01,
        [16..24] dp_gma_curve_02,
        [24..32] dp_gma_curve_03,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve04: u32 {
        [00..08] dp_gma_curve_04,
        [08..16] dp_gma_curve_05,
        [16..24] dp_gma_curve_06,
        [24..32] dp_gma_curve_07,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve08: u32 {
        [00..08] dp_gma_curve_08,
        [08..16] dp_gma_curve_09,
        [16..24] dp_gma_curve_0a,
        [24..32] dp_gma_curve_0b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve0c: u32 {
        [00..08] dp_gma_curve_0c,
        [08..16] dp_gma_curve_0d,
        [16..24] dp_gma_curve_0e,
        [24..32] dp_gma_curve_0f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve10: u32 {
        [00..08] dp_gma_curve_10,
        [08..16] dp_gma_curve_11,
        [16..24] dp_gma_curve_12,
        [24..32] dp_gma_curve_13,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve14: u32 {
        [00..08] dp_gma_curve_14,
        [08..16] dp_gma_curve_15,
        [16..24] dp_gma_curve_16,
        [24..32] dp_gma_curve_17,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve18: u32 {
        [00..08] dp_gma_curve_18,
        [08..16] dp_gma_curve_19,
        [16..24] dp_gma_curve_1a,
        [24..32] dp_gma_curve_1b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve1c: u32 {
        [00..08] dp_gma_curve_1c,
        [08..16] dp_gma_curve_1d,
        [16..24] dp_gma_curve_1e,
        [24..32] dp_gma_curve_1f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve20: u32 {
        [00..08] dp_gma_curve_20,
        [08..16] dp_gma_curve_21,
        [16..24] dp_gma_curve_22,
        [24..32] dp_gma_curve_23,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve24: u32 {
        [00..08] dp_gma_curve_24,
        [08..16] dp_gma_curve_25,
        [16..24] dp_gma_curve_26,
        [24..32] dp_gma_curve_27,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve28: u32 {
        [00..08] dp_gma_curve_28,
        [08..16] dp_gma_curve_29,
        [16..24] dp_gma_curve_2a,
        [24..32] dp_gma_curve_2b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve2c: u32 {
        [00..08] dp_gma_curve_2c,
        [08..16] dp_gma_curve_2d,
        [16..24] dp_gma_curve_2e,
        [24..32] dp_gma_curve_2f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve30: u32 {
        [00..08] dp_gma_curve_30,
        [08..16] dp_gma_curve_31,
        [16..24] dp_gma_curve_32,
        [24..32] dp_gma_curve_33,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve34: u32 {
        [00..08] dp_gma_curve_34,
        [08..16] dp_gma_curve_35,
        [16..24] dp_gma_curve_36,
        [24..32] dp_gma_curve_37,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve38: u32 {
        [00..08] dp_gma_curve_38,
        [08..16] dp_gma_curve_39,
        [16..24] dp_gma_curve_3a,
        [24..32] dp_gma_curve_3b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve3c: u32 {
        [00..08] dp_gma_curve_3c,
        [08..16] dp_gma_curve_3d,
        [16..24] dp_gma_curve_3e,
        [24..32] dp_gma_curve_3f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve40: u32 {
        [00..08] dp_gma_curve_40,
        [08..16] dp_gma_curve_41,
        [16..24] dp_gma_curve_42,
        [24..32] dp_gma_curve_43,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve44: u32 {
        [00..08] dp_gma_curve_44,
        [08..16] dp_gma_curve_45,
        [16..24] dp_gma_curve_46,
        [24..32] dp_gma_curve_47,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve48: u32 {
        [00..08] dp_gma_curve_48,
        [08..16] dp_gma_curve_49,
        [16..24] dp_gma_curve_4a,
        [24..32] dp_gma_curve_4b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve4c: u32 {
        [00..08] dp_gma_curve_4c,
        [08..16] dp_gma_curve_4d,
        [16..24] dp_gma_curve_4e,
        [24..32] dp_gma_curve_4f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve50: u32 {
        [00..08] dp_gma_curve_50,
        [08..16] dp_gma_curve_51,
        [16..24] dp_gma_curve_52,
        [24..32] dp_gma_curve_53,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve54: u32 {
        [00..08] dp_gma_curve_54,
        [08..16] dp_gma_curve_55,
        [16..24] dp_gma_curve_56,
        [24..32] dp_gma_curve_57,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve58: u32 {
        [00..08] dp_gma_curve_58,
        [08..16] dp_gma_curve_59,
        [16..24] dp_gma_curve_5a,
        [24..32] dp_gma_curve_5b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve5c: u32 {
        [00..08] dp_gma_curve_5c,
        [08..16] dp_gma_curve_5d,
        [16..24] dp_gma_curve_5e,
        [24..32] dp_gma_curve_5f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve60: u32 {
        [00..08] dp_gma_curve_60,
        [08..16] dp_gma_curve_61,
        [16..24] dp_gma_curve_62,
        [24..32] dp_gma_curve_63,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve64: u32 {
        [00..08] dp_gma_curve_64,
        [08..16] dp_gma_curve_65,
        [16..24] dp_gma_curve_66,
        [24..32] dp_gma_curve_67,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve68: u32 {
        [00..08] dp_gma_curve_68,
        [08..16] dp_gma_curve_69,
        [16..24] dp_gma_curve_6a,
        [24..32] dp_gma_curve_6b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve6c: u32 {
        [00..08] dp_gma_curve_6c,
        [08..16] dp_gma_curve_6d,
        [16..24] dp_gma_curve_6e,
        [24..32] dp_gma_curve_6f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve70: u32 {
        [00..08] dp_gma_curve_70,
        [08..16] dp_gma_curve_71,
        [16..24] dp_gma_curve_72,
        [24..32] dp_gma_curve_73,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve74: u32 {
        [00..08] dp_gma_curve_74,
        [08..16] dp_gma_curve_75,
        [16..24] dp_gma_curve_76,
        [24..32] dp_gma_curve_77,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve78: u32 {
        [00..08] dp_gma_curve_78,
        [08..16] dp_gma_curve_79,
        [16..24] dp_gma_curve_7a,
        [24..32] dp_gma_curve_7b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve7c: u32 {
        [00..08] dp_gma_curve_7c,
        [08..16] dp_gma_curve_7d,
        [16..24] dp_gma_curve_7e,
        [24..32] dp_gma_curve_7f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve80: u32 {
        [00..08] dp_gma_curve_80,
        [08..16] dp_gma_curve_81,
        [16..24] dp_gma_curve_82,
        [24..32] dp_gma_curve_83,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve84: u32 {
        [00..08] dp_gma_curve_84,
        [08..16] dp_gma_curve_85,
        [16..24] dp_gma_curve_86,
        [24..32] dp_gma_curve_87,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve88: u32 {
        [00..08] dp_gma_curve_88,
        [08..16] dp_gma_curve_89,
        [16..24] dp_gma_curve_8a,
        [24..32] dp_gma_curve_8b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve8c: u32 {
        [00..08] dp_gma_curve_8c,
        [08..16] dp_gma_curve_8d,
        [16..24] dp_gma_curve_8e,
        [24..32] dp_gma_curve_8f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve90: u32 {
        [00..08] dp_gma_curve_90,
        [08..16] dp_gma_curve_91,
        [16..24] dp_gma_curve_92,
        [24..32] dp_gma_curve_93,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve94: u32 {
        [00..08] dp_gma_curve_94,
        [08..16] dp_gma_curve_95,
        [16..24] dp_gma_curve_96,
        [24..32] dp_gma_curve_97,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve98: u32 {
        [00..08] dp_gma_curve_98,
        [08..16] dp_gma_curve_99,
        [16..24] dp_gma_curve_9a,
        [24..32] dp_gma_curve_9b,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurve9c: u32 {
        [00..08] dp_gma_curve_9c,
        [08..16] dp_gma_curve_9d,
        [16..24] dp_gma_curve_9e,
        [24..32] dp_gma_curve_9f,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveA0: u32 {
        [00..08] dp_gma_curve_a0,
        [08..16] dp_gma_curve_a1,
        [16..24] dp_gma_curve_a2,
        [24..32] dp_gma_curve_a3,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveA4: u32 {
        [00..08] dp_gma_curve_a4,
        [08..16] dp_gma_curve_a5,
        [16..24] dp_gma_curve_a6,
        [24..32] dp_gma_curve_a7,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveA8: u32 {
        [00..08] dp_gma_curve_a8,
        [08..16] dp_gma_curve_a9,
        [16..24] dp_gma_curve_aa,
        [24..32] dp_gma_curve_ab,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveAc: u32 {
        [00..08] dp_gma_curve_ac,
        [08..16] dp_gma_curve_ad,
        [16..24] dp_gma_curve_ae,
        [24..32] dp_gma_curve_af,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveB0: u32 {
        [00..08] dp_gma_curve_b0,
        [08..16] dp_gma_curve_b1,
        [16..24] dp_gma_curve_b2,
        [24..32] dp_gma_curve_b3,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveB4: u32 {
        [00..08] dp_gma_curve_b4,
        [08..16] dp_gma_curve_b5,
        [16..24] dp_gma_curve_b6,
        [24..32] dp_gma_curve_b7,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveB8: u32 {
        [00..08] dp_gma_curve_b8,
        [08..16] dp_gma_curve_b9,
        [16..24] dp_gma_curve_ba,
        [24..32] dp_gma_curve_bb,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveBc: u32 {
        [00..08] dp_gma_curve_bc,
        [08..16] dp_gma_curve_bd,
        [16..24] dp_gma_curve_be,
        [24..32] dp_gma_curve_bf,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveC0: u32 {
        [00..08] dp_gma_curve_c0,
        [08..16] dp_gma_curve_c1,
        [16..24] dp_gma_curve_c2,
        [24..32] dp_gma_curve_c3,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveC4: u32 {
        [00..08] dp_gma_curve_c4,
        [08..16] dp_gma_curve_c5,
        [16..24] dp_gma_curve_c6,
        [24..32] dp_gma_curve_c7,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveC8: u32 {
        [00..08] dp_gma_curve_c8,
        [08..16] dp_gma_curve_c9,
        [16..24] dp_gma_curve_ca,
        [24..32] dp_gma_curve_cb,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveCc: u32 {
        [00..08] dp_gma_curve_cc,
        [08..16] dp_gma_curve_cd,
        [16..24] dp_gma_curve_ce,
        [24..32] dp_gma_curve_cf,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveD0: u32 {
        [00..08] dp_gma_curve_d0,
        [08..16] dp_gma_curve_d1,
        [16..24] dp_gma_curve_d2,
        [24..32] dp_gma_curve_d3,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveD4: u32 {
        [00..08] dp_gma_curve_d4,
        [08..16] dp_gma_curve_d5,
        [16..24] dp_gma_curve_d6,
        [24..32] dp_gma_curve_d7,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveD8: u32 {
        [00..08] dp_gma_curve_d8,
        [08..16] dp_gma_curve_d9,
        [16..24] dp_gma_curve_da,
        [24..32] dp_gma_curve_db,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveDc: u32 {
        [00..08] dp_gma_curve_dc,
        [08..16] dp_gma_curve_dd,
        [16..24] dp_gma_curve_de,
        [24..32] dp_gma_curve_df,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveE0: u32 {
        [00..08] dp_gma_curve_e0,
        [08..16] dp_gma_curve_e1,
        [16..24] dp_gma_curve_e2,
        [24..32] dp_gma_curve_e3,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveE4: u32 {
        [00..08] dp_gma_curve_e4,
        [08..16] dp_gma_curve_e5,
        [16..24] dp_gma_curve_e6,
        [24..32] dp_gma_curve_e7,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveE8: u32 {
        [00..08] dp_gma_curve_e8,
        [08..16] dp_gma_curve_e9,
        [16..24] dp_gma_curve_ea,
        [24..32] dp_gma_curve_eb,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveEc: u32 {
        [00..08] dp_gma_curve_ec,
        [08..16] dp_gma_curve_ed,
        [16..24] dp_gma_curve_ee,
        [24..32] dp_gma_curve_ef,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveF0: u32 {
        [00..08] dp_gma_curve_f0,
        [08..16] dp_gma_curve_f1,
        [16..24] dp_gma_curve_f2,
        [24..32] dp_gma_curve_f3,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveF4: u32 {
        [00..08] dp_gma_curve_f4,
        [08..16] dp_gma_curve_f5,
        [16..24] dp_gma_curve_f6,
        [24..32] dp_gma_curve_f7,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveF8: u32 {
        [00..08] dp_gma_curve_f8,
        [08..16] dp_gma_curve_f9,
        [16..24] dp_gma_curve_fa,
        [24..32] dp_gma_curve_fb,
    }
}

embedded_util::reg! {
    pub struct Dsp2MiscDispGmaCurveFc: u32 {
        [00..08] dp_gma_curve_fc,
        [08..16] dp_gma_curve_fd,
        [16..24] dp_gma_curve_fe,
        [24..32] dp_gma_curve_ff,
    }
}
