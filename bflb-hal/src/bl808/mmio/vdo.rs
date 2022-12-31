//! Video/H264 registers.

emhal::mmio_struct! {
    pub struct Vdo {
        [0x000] rw h264_encoder_ctrl: VdoH264EncoderCtrl,
        [0x004] rw h264_frame_size: VdoH264FrameSize,
        [0x008] rw h264_frame_base_addr_y: VdoH264FrameBaseAddrY,
        [0x00C] rw h264_frame_base_addr_uv: VdoH264FrameBaseAddrUv,
        [0x010] rw h264_bs_base_addr: VdoH264BsBaseAddr,
        [0x014] rw h264_lb_base_addr: VdoH264LbBaseAddr,
        [0x018] rw h264_bs_size: VdoH264BsSize,
        [0x01C] rw h264_bs_full_thr: VdoH264BsFullThr,
        [0x020] rw h264_src_bf_height: VdoH264SrcBfHeight,
        [0x024] rw core_reg0: VdoCoreReg0,
        [0x028] rw core_reg1: VdoCoreReg1,
        [0x02C] rw core_reg2: VdoCoreReg2,
        [0x030] rw core_reg3: VdoCoreReg3,
        [0x034] rw core_reg4: VdoCoreReg4,
        [0x038] rw core_reg5: VdoCoreReg5,
        [0x03C] rw core_reg6: VdoCoreReg6,
        [0x040] rw core_reg7: VdoCoreReg7,
        [0x044] rw core_reg8: VdoCoreReg8,
        [0x048] rw core_reg9: VdoCoreReg9,
        [0x04C] rw core_reg10: VdoCoreReg10,
        [0x050] rw core_reg11: VdoCoreReg11,
        [0x054] rw core_reg12: VdoCoreReg12,
        [0x058] rw core_reg13: VdoCoreReg13,
        [0x05C] rw core_reg14: VdoCoreReg14,
        [0x064] rw core_reg16: VdoCoreReg16,
        [0x068] rw core_reg17: VdoCoreReg17,
        [0x06C] rw core_reg18: VdoCoreReg18,
        [0x070] rw core_reg19: VdoCoreReg19,
        [0x074] rw core_reg20: VdoCoreReg20,
        [0x078] rw core_reg21: VdoCoreReg21,
        [0x07C] rw core_reg22: VdoCoreReg22,
        [0x080] rw core_reg23: VdoCoreReg23,
        [0x084] rw core_reg24: VdoCoreReg24,
        [0x088] rw core_reg25: VdoCoreReg25,
        [0x094] rw core_reg28: VdoCoreReg28,
        [0x09C] rw core_reg30: VdoCoreReg30,
        [0x0A0] rw core_reg31: VdoCoreReg31,
        [0x0A4] rw core_reg32: VdoCoreReg32,
        [0x0A8] rw core_reg33: VdoCoreReg33,
        [0x0AC] rw core_reg34: VdoCoreReg34,
        [0x0B0] rw core_reg35: VdoCoreReg35,
        [0x0B4] rw core_reg36: VdoCoreReg36,
        [0x0B8] rw core_reg37: VdoCoreReg37,
        [0x0BC] rw core_reg38: VdoCoreReg38,
        [0x0C0] rw src_dvp_sel: VdoSrcDvpSel,
        [0x0C4] rw padding_ctrl: VdoPaddingCtrl,
        [0x0D8] rw frame_id_clr: VdoFrameIdClr,
        [0x100] rw int: VdoInt,
        [0x104] rw int_clr: VdoIntClr,
        [0x108] rw int_mask: VdoIntMask,
        [0x10C] rw total_bytecount: VdoTotalBytecount,
        [0x110] rw frame_bytecount: VdoFrameBytecount,
        [0x114] rw src_r_dbg: VdoSrcRDbg,
        [0x118] rw core_dbg: VdoCoreDbg,
        [0x11C] rw ctrl_dbg: VdoCtrlDbg,
        [0x120] rw checksum_0: VdoChecksum0,
        [0x124] rw checksum_1: VdoChecksum1,
        [0x128] rw checksum_2: VdoChecksum2,
        [0x12C] rw checksum_3: VdoChecksum3,
        [0x130] rw checksum_4: VdoChecksum4,
        [0x134] rw checksum_5: VdoChecksum5,
        [0x138] rw checksum_6: VdoChecksum6,
        [0x13C] rw checksum_7: VdoChecksum7,
        [0x140] rw reserved: VdoReserved,
        [0x144] rw h264_y_frame_mem_size: VdoH264YFrameMemSize,
        [0x148] rw h264_uv_frame_mem_size: VdoH264UvFrameMemSize,
        [0x14C] rw h264_s_frame_size: VdoH264SFrameSize,
        [0x150] rw h264_s_frame_base_addr_y: VdoH264SFrameBaseAddrY,
        [0x154] rw h264_s_frame_base_addr_uv: VdoH264SFrameBaseAddrUv,
        [0x158] rw h264_s_y_frame_mem_size: VdoH264SYFrameMemSize,
        [0x15C] rw h264_s_uv_frame_mem_size: VdoH264SUvFrameMemSize,
        [0x160] rw h264_s_bs_base_addr: VdoH264SBsBaseAddr,
        [0x164] rw reserved2: VdoReserved2,
        [0x168] rw h264_s_bs_size: VdoH264SBsSize,
        [0x16C] rw h264_s_bs_full_thr: VdoH264SBsFullThr,
        [0x170] rw h264_s_src_bf_height: VdoH264SSrcBfHeight,
        [0x174] rw s_padding_ctrl: VdoSPaddingCtrl,
        [0x178] rw s_total_bytecount: VdoSTotalBytecount,
        [0x17C] rw s_frame_bytecount: VdoSFrameBytecount,
        [0x180] rw s_src_r_dbg: VdoSSrcRDbg,
        [0x184] rw src_sta_clr: VdoSrcStaClr,
        [0x188] rw src_ctrl: VdoSrcCtrl,
        [0x18C] rw h264_refy0_base_addr: VdoH264Refy0BaseAddr,
        [0x190] rw h264_refu0_base_addr: VdoH264Refu0BaseAddr,
        [0x194] rw h264_refv0_base_addr: VdoH264Refv0BaseAddr,
        [0x198] rw h264_refy1_base_addr: VdoH264Refy1BaseAddr,
        [0x19C] rw h264_refu1_base_addr: VdoH264Refu1BaseAddr,
        [0x1A0] rw h264_refv1_base_addr: VdoH264Refv1BaseAddr,
        [0x1A4] rw h264_ext_base_addr: VdoH264ExtBaseAddr,
        [0x1A8] rw h264_s_refy0_base_addr: VdoH264SRefy0BaseAddr,
        [0x1AC] rw h264_s_refu0_base_addr: VdoH264SRefu0BaseAddr,
        [0x1B0] rw h264_s_refv0_base_addr: VdoH264SRefv0BaseAddr,
        [0x1B4] rw h264_s_refy1_base_addr: VdoH264SRefy1BaseAddr,
        [0x1B8] rw h264_s_refu1_base_addr: VdoH264SRefu1BaseAddr,
        [0x1BC] rw h264_s_refv1_base_addr: VdoH264SRefv1BaseAddr,
        [0x1C0] rw h264_s_ext_base_addr: VdoH264SExtBaseAddr,
        [0x1C4] rw core_dbg2: VdoCoreDbg2,
        [0x1C8] rw s_core_dbg2: VdoSCoreDbg2,
        [0x1CC] rw h264_y_srcbf_size: VdoH264YSrcbfSize,
        [0x1D0] rw h264_uv_srcbf_size: VdoH264UvSrcbfSize,
        [0x1D4] rw h264_s_y_srcbf_size: VdoH264SYSrcbfSize,
        [0x1D8] rw h264_s_uv_srcbf_size: VdoH264SUvSrcbfSize,
        [0x1DC] rw h264_src_dbg: VdoH264SrcDbg,
        [0x1E0] rw h264_yy_d2b_addr: VdoH264YyD2bAddr,
        [0x1E4] rw h264_uv_d2b_addr: VdoH264UvD2bAddr,
        [0x1E8] rw h264_yy_ofst_addr: VdoH264YyOfstAddr,
        [0x1EC] rw h264_uv_ofst_addr: VdoH264UvOfstAddr,
        [0x1F0] rw h264_d2b_ml: VdoH264D2bMl,
        [0x1F4] rw h264_load_mbidx_dbg: VdoH264LoadMbidxDbg,
        [0x1F8] rw h264_yc_mb_err: VdoH264YcMbErr,
        [0x1FC] rw h264_s_yy_d2b_addr: VdoH264SYyD2bAddr,
        [0x200] rw h264_s_uv_d2b_addr: VdoH264SUvD2bAddr,
        [0x204] rw h264_s_yy_ofst_addr: VdoH264SYyOfstAddr,
        [0x208] rw h264_s_uv_ofst_addr: VdoH264SUvOfstAddr,
        [0x20C] rw h264_s_d2b_ml: VdoH264SD2bMl,
        [0x210] rw h264_s_load_mbidx_dbg: VdoH264SLoadMbidxDbg,
        [0x214] rw h264_s_yc_mb_err: VdoH264SYcMbErr,
        [0x228] rw h264_pfch_ctrl: VdoH264PfchCtrl,
        [0x22C] rw h264_pfch_base: VdoH264PfchBase,
        [0x230] rw h264_pfchv_base: VdoH264PfchvBase,
        [0x234] rw h264_s_pfch_ctrl: VdoH264SPfchCtrl,
        [0x238] rw h264_s_pfch_base: VdoH264SPfchBase,
        [0x23C] rw h264_s_pfchv_base: VdoH264SPfchvBase,
        [0x240] rw h264_hwback_mode: VdoH264HwbackMode,
        [0x244] rw h264_roi_mode: VdoH264RoiMode,
        [0x248] rw h264_roi_bit: VdoH264RoiBit,
        [0x24C] rw h264_roi_qptune: VdoH264RoiQptune,
        [0x250] rw h264_roi_bittune: VdoH264RoiBittune,
        [0x254] rw h264_roi0: VdoH264Roi0,
        [0x258] rw h264_roi1: VdoH264Roi1,
        [0x25C] rw h264_roi2: VdoH264Roi2,
        [0x260] rw h264_roi3: VdoH264Roi3,
        [0x264] rw h264_roi4: VdoH264Roi4,
        [0x268] rw h264_roi5: VdoH264Roi5,
        [0x26C] rw h264_roi6: VdoH264Roi6,
        [0x270] rw h264_roi7: VdoH264Roi7,
        [0x274] rw h264_s_roi_mode: VdoH264SRoiMode,
        [0x278] rw h264_s_roi_bit: VdoH264SRoiBit,
        [0x27C] rw h264_s_roi_qptune: VdoH264SRoiQptune,
        [0x280] rw h264_s_roi_bittune: VdoH264SRoiBittune,
        [0x284] rw h264_s_roi0: VdoH264SRoi0,
        [0x288] rw h264_s_roi1: VdoH264SRoi1,
        [0x28C] rw h264_s_roi2: VdoH264SRoi2,
        [0x290] rw h264_s_roi3: VdoH264SRoi3,
        [0x294] rw h264_s_roi4: VdoH264SRoi4,
        [0x298] rw h264_s_roi5: VdoH264SRoi5,
        [0x29C] rw h264_s_roi6: VdoH264SRoi6,
        [0x300] rw h264_s_roi7: VdoH264SRoi7,
        [0x304] rw h264_osd_en: VdoH264OsdEn,
        [0x308] rw h264_osd0: VdoH264Osd0,
        [0x30C] rw h264_osd1: VdoH264Osd1,
        [0x310] rw h264_osd2: VdoH264Osd2,
        [0x314] rw h264_osd3: VdoH264Osd3,
        [0x318] rw h264_osd4: VdoH264Osd4,
        [0x31C] rw h264_osd5: VdoH264Osd5,
        [0x320] rw h264_osd6: VdoH264Osd6,
        [0x324] rw h264_osd7: VdoH264Osd7,
        [0x328] rw h264_osd8: VdoH264Osd8,
        [0x32C] rw h264_osd9: VdoH264Osd9,
        [0x330] rw h264_osd10: VdoH264Osd10,
        [0x334] rw h264_osd11: VdoH264Osd11,
        [0x338] rw h264_osd12: VdoH264Osd12,
        [0x33C] rw h264_osd13: VdoH264Osd13,
        [0x340] rw h264_osd14: VdoH264Osd14,
        [0x344] rw h264_osd15: VdoH264Osd15,
        [0x348] rw h264_mv0_en: VdoH264Mv0En,
        [0x34C] rw pfch_dbg_reqcnt: VdoPfchDbgReqcnt,
        [0x350] rw pfch_dbg_rcmd: VdoPfchDbgRcmd,
        [0x354] rw pfch_dbg_wcmd: VdoPfchDbgWcmd,
        [0x358] rw pfch_dbg_wdata: VdoPfchDbgWdata,
        [0x35C] rw pfch_dbg_pfchsta: VdoPfchDbgPfchsta,
        [0x360] rw pfchv_dbg_reqcnt: VdoPfchvDbgReqcnt,
        [0x364] rw pfchv_dbg_rcmd: VdoPfchvDbgRcmd,
        [0x368] rw pfchv_dbg_wcmd: VdoPfchvDbgWcmd,
        [0x36C] rw pfchv_dbg_wdata: VdoPfchvDbgWdata,
        [0x370] rw pfchv_dbg_pfchsta: VdoPfchvDbgPfchsta,
        [0x374] rw pfch_s_dbg_reqcnt: VdoPfchSDbgReqcnt,
        [0x378] rw pfch_s_dbg_rcmd: VdoPfchSDbgRcmd,
        [0x37C] rw pfch_s_dbg_wcmd: VdoPfchSDbgWcmd,
        [0x380] rw pfch_s_dbg_wdata: VdoPfchSDbgWdata,
        [0x384] rw pfch_s_dbg_pfchsta: VdoPfchSDbgPfchsta,
        [0x388] rw pfchv_s_dbg_reqcnt: VdoPfchvSDbgReqcnt,
        [0x38C] rw pfchv_s_dbg_rcmd: VdoPfchvSDbgRcmd,
        [0x390] rw pfchv_s_dbg_wcmd: VdoPfchvSDbgWcmd,
        [0x394] rw pfchv_s_dbg_wdata: VdoPfchvSDbgWdata,
        [0x398] rw pfchv_s_dbg_pfchsta: VdoPfchvSDbgPfchsta,
        [0x39C] rw vdonr: Vdonr,
        [0x3A0] rw vdonr_mvctrl: VdonrMvctrl,
        [0x3A4] rw vdonr_sta_ctrl: VdonrStaCtrl,
        [0x3A8] rw vdonr_absmv_sum: VdonrAbsmvSum,
        [0x3AC] rw vdonr_mvlevel: VdonrMvlevel,
        [0x3B0] rw vdonr_pmbsad: VdonrPmbsad,
        [0x3B4] rw vdonr_imbcnt: VdonrImbcnt,
        [0x3B8] rw vdonr_s_mvctrl: VdonrSMvctrl,
        [0x3BC] rw vdonr_s_sta_ctrl: VdonrSStaCtrl,
        [0x3C0] rw vdonr_s_absmv_sum: VdonrSAbsmvSum,
        [0x3C4] rw vdonr_s_mvlevel: VdonrSMvlevel,
        [0x3C8] rw vdonr_s_pmbsad: VdonrSPmbsad,
        [0x3CC] rw vdonr_s_imbcnt: VdonrSImbcnt,
        [0x3D0] rw stuf_dbg: VdoStufDbg,
        [0x3D4] rw nal_dbg1: VdoNalDbg1,
        [0x3D8] rw nal_dbg2: VdoNalDbg2,
        [0x3DC] rw nal_dbg3: VdoNalDbg3,
        [0x3E0] rw roi_dbg: VdoRoiDbg,
        [0x3E4] rw rc_dbg: VdoRcDbg,
        [0x3E8] rw s_stuf_dbg: VdoSStufDbg,
        [0x3EC] rw s_nal_dbg1: VdoSNalDbg1,
        [0x3F0] rw s_nal_dbg2: VdoSNalDbg2,
        [0x3F4] rw s_nal_dbg3: VdoSNalDbg3,
        [0x3F8] rw s_roi_dbg: VdoSRoiDbg,
        [0x3FC] rw s_rc_dbg: VdoSRcDbg,
        [0x400] rw refaxi_dbg: VdoRefaxiDbg,
        [0x404] rw pfchaxi_dbg: VdoPfchaxiDbg,
        [0x408] rw bsaxi_dbg: VdoBsaxiDbg,
        [0x40C] rw frame_id: VdoFrameId,
        [0x410] rw s_vdo_frame_id: VdoSVdoFrameId,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264EncoderCtrl: u32 {
        [0..7] cfg_enc_ctrl_mby_idx,
        [8..9] cfg_core_clk_en,
        [9..10] cfg_zstuf_dis,
        [10..11] cfg_qr_upd,
        [11..12] cfg_s_qr_upd,
        [12..13] cfg_mem_clk_en,
        [13..14] cfg_rc_gop_upd,
        [14..15] cfg_s_rc_gop_upd,
        [15..16] cfg_duo_bs_en,
        [16..17] cfg_buf_clr,
        [17..18] cfg_s_buf_clr,
        [18..19] cfg_vdonr_en,
        [19..20] cfg_s_vdonr_en,
        [20..21] cfg_bs_status_init,
        [21..22] cfg_s_bs_status_init,
        [22..23] cfg_src_line_clr,
        [23..24] cfg_s_src_line_clr,
        /// - 0 - Auto
        /// - 2 - Manual
        [24..26] cfg_ctrl_mode,
        [26..27] cfg_cref_en,
        [27..28] cfg_s_cref_en,
        [28..29] cfg_frame_start,
        [29..30] cfg_enc_seq_en,
        [30..31] cfg_s_frame_start,
        [31..32] cfg_s_enc_seq_en,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264FrameSize: u32 {
        [0..11] cfg_frame_width,
        [16..27] cfg_frame_height,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264FrameBaseAddrY: u32 {
        [0..32] cfg_frame_base_addr_y,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264FrameBaseAddrUv: u32 {
        [0..32] cfg_frame_base_addr_uv,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264BsBaseAddr: u32 {
        [0..32] cfg_bs_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264LbBaseAddr: u32 {
        [0..32] cfg_lb_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264BsSize: u32 {
        [0..32] cfg_bs_size,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264BsFullThr: u32 {
        [0..32] cfg_bs_full_thr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SrcBfHeight: u32 {
        [0..15] cfg_c_src_bf_height,
        [16..31] cfg_y_src_bf_height,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg0: u32 {
        [0..1] enc_en,
        [12..13] vui_aspect_info,
        [13..14] vui_fixed_f_rate,
        [14..15] vui_timing_info,
        [15..16] idr_enable,
        [16..17] s_enc_en,
        [28..29] s_vui_aspect_info,
        [29..30] s_vui_fixed_f_rate,
        [30..31] s_vui_timing_info,
        [31..32] s_idr_enable,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg1: u32 {
        [0..11] img_width,
        [16..27] s_img_width,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg2: u32 {
        [0..11] img_height,
        [16..27] s_img_height,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg3: u32 {
        [0..6] target_i_qp,
        [16..22] s_target_i_qp,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg4: u32 {
        [0..5] cqp_offset,
        [16..21] s_cqp_offset,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg5: u32 {
        [0..6] target_p_qp,
        [16..22] s_target_p_qp,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg6: u32 {
        [0..7] mb_lines,
        [16..23] s_mb_lines,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg7: u32 {
        [0..4] num_slices,
        [16..20] s_num_slices,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg8: u32 {
        [0..10] num_pframes,
        [16..26] s_num_pframes,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg9: u32 {
        [0..1] dis_filter,
        [16..17] s_dis_filter,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg10: u32 {
        [0..4] alpha_off,
        [16..20] s_alpha_off,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg11: u32 {
        [0..4] beta_off,
        [16..20] s_beta_off,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg12: u32 {
        [0..1] profile_idc,
        [16..17] s_profile_idc,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg13: u32 {
        [0..8] level_idc,
        [16..24] s_level_idc,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg14: u32 {
        [0..3] luma8x8_cost,
        [3..6] luma16x16_cost,
        [6..9] chroma8x8_cost,
        [16..19] s_luma8x8_cost,
        [19..22] s_luma16x16_cost,
        [22..25] s_chroma8x8_cost,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg16: u32 {
        [0..14] frame0,
        [16..30] s_frame0,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg17: u32 {
        [0..14] frame1,
        [16..30] s_frame1,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg18: u32 {
        [0..14] ext_data,
        [16..30] s_ext_data,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg19: u32 {
        [0..6] ext_mem_config0,
        [16..22] s_ext_mem_config0,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg20: u32 {
        [0..5] ext_mem_config1,
        [16..21] s_ext_mem_config1,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg21: u32 {
        [0..12] ext_mem_config2,
        [16..28] s_ext_mem_config2,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg22: u32 {
        [0..10] ext_mem_config3,
        [16..26] s_ext_mem_config3,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg23: u32 {
        [0..12] num_imb_bits,
        [13..14] nal_align,
        [14..15] cons_ipred,
        [16..28] s_num_imb_bits,
        [29..30] s_nal_align,
        [30..31] s_cons_ipred,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg24: u32 {
        [0..12] num_pmb_bits,
        [16..28] s_num_pmb_bits,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg25: u32 {
        [0..1] interlace_mode,
        [1..2] bottom_field_first,
        [2..3] prev_field_ref,
        [16..17] s_interlace_mode,
        [17..18] s_bottom_field_first,
        [18..19] s_prev_field_ref,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg28: u32 {
        [0..6] pframe_min_qp,
        [8..14] pframe_max_qp,
        [16..22] s_pframe_min_qp,
        [24..30] s_pframe_max_qp,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg30: u32 {
        [15..16] reset_i_state,
        [31..32] s_reset_i_state,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg31: u32 {
        [0..6] iframe_min_qp,
        [8..14] iframe_max_qp,
        [16..22] s_iframe_min_qp,
        [24..30] s_iframe_max_qp,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg32: u32 {
        [0..16] vui_time_scale_l,
        [16..32] s_vui_time_scale_l,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg33: u32 {
        [0..16] vui_time_scale_h,
        [16..32] s_vui_time_scale_h,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg34: u32 {
        [0..16] vui_num_u_tick_l,
        [16..32] s_vui_num_u_tick_l,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg35: u32 {
        [0..16] vui_num_u_tick_h,
        [16..32] s_vui_num_u_tick_h,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg36: u32 {
        [0..8] vui_aspect_idc,
        [16..24] s_vui_aspect_idc,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg37: u32 {
        [0..16] vui_aspect_width,
        [16..32] s_vui_aspect_width,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreReg38: u32 {
        [0..16] vui_aspect_height,
        [16..32] s_vui_aspect_height,
    }
}

emhal::mmio_reg! {
    pub struct VdoSrcDvpSel: u32 {
        [0..3] uv_dvp2axi_sel,
        [4..7] y_dvp2axi_sel,
        [16..19] s_uv_dvp2axi_sel,
        [20..23] s_y_dvp2axi_sel,
    }
}

emhal::mmio_reg! {
    pub struct VdoPaddingCtrl: u32 {
        [0..8] y_padding_pixel,
        [8..16] u_padding_pixel,
        [16..24] v_padding_pixel,
        [24..29] y_line_pad,
    }
}

emhal::mmio_reg! {
    pub struct VdoFrameIdClr: u32 {
        [0..1] frame_id_clr,
        [1..2] s_frame_id_clr,
    }
}

emhal::mmio_reg! {
    pub struct VdoInt: u32 {
        [0..1] frm_done_int,
        [1..2] s_frm_done_int,
        [4..5] seq_done_int,
        [5..6] s_seq_done_int,
        [8..9] bs_full_int,
        [9..10] s_bs_full_int,
    }
}

emhal::mmio_reg! {
    pub struct VdoIntClr: u32 {
        [0..1] frm_done_int_clr,
        [1..2] s_frm_done_int_clr,
        [4..5] seq_done_int_clr,
        [5..6] s_seq_done_int_clr,
        [8..9] bs_full_int_clr,
        [9..10] s_bs_full_int_clr,
    }
}

emhal::mmio_reg! {
    pub struct VdoIntMask: u32 {
        [0..1] frm_done_int_mask,
        [1..2] s_frm_done_int_mask,
        [4..5] seq_done_int_mask,
        [5..6] s_seq_done_int_mask,
        [8..9] bs_full_int_mask,
        [9..10] s_bs_full_int_mask,
    }
}

emhal::mmio_reg! {
    pub struct VdoTotalBytecount: u32 {
        [0..32] bs_tot_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoFrameBytecount: u32 {
        [0..32] bs_frm_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoSrcRDbg: u32 {
        [0..13] y_read_mbx,
        [16..29] uv_read_mbx,
        [30..31] axi_r_idle,
        [31..32] src_wr_ov_rd,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreDbg: u32 {
        [0..7] core_mbx,
        [8..15] core_mby,
        [16..17] m_core_idle,
        [17..18] s_core_idle,
        [20..25] se_fsm,
        [29..30] oclk_en,
        [30..31] timeout_state,
        [31..32] mem_ctrl_idle,
    }
}

emhal::mmio_reg! {
    pub struct VdoCtrlDbg: u32 {
        [0..3] ctrl_state,
        [3..4] h264_proc,
        [4..7] s_ctrl_state,
        [7..8] s_h264_proc,
        [8..9] ctrl_ce,
        [12..13] stream_idx,
        [16..24] ctrler_frm_cnt,
        [24..32] s_ctrler_frm_cnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum0: u32 {
        [0..32] vdo0_axi_rdata_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum1: u32 {
        [0..32] vdo0_axi_wdata_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum2: u32 {
        [0..32] vdo1_axi_rdata_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum3: u32 {
        [0..32] vdo1_axi_wdata_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum4: u32 {
        [0..32] ld_yblk_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum5: u32 {
        [0..32] ld_cblk_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum6: u32 {
        [0..32] mem_din_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoChecksum7: u32 {
        [0..32] mem_dout_chksum,
    }
}

emhal::mmio_reg! {
    pub struct VdoReserved: u32 {
        [0..32] reserved,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264YFrameMemSize: u32 {
        [0..32] cfg_mem_bcnt_y,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264UvFrameMemSize: u32 {
        [0..32] cfg_mem_bcnt_uv,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SFrameSize: u32 {
        [0..11] cfg_s_frame_width,
        [16..27] cfg_s_frame_height,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SFrameBaseAddrY: u32 {
        [0..32] cfg_s_frame_base_addr_y,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SFrameBaseAddrUv: u32 {
        [0..32] cfg_s_frame_base_addr_uv,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SYFrameMemSize: u32 {
        [0..32] cfg_s_mem_bcnt_y,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SUvFrameMemSize: u32 {
        [0..32] cfg_s_mem_bcnt_uv,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SBsBaseAddr: u32 {
        [0..32] cfg_s_bs_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoReserved2: u32 {
        [0..32] reserved2,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SBsSize: u32 {
        [0..32] cfg_s_bs_size,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SBsFullThr: u32 {
        [0..32] cfg_s_bs_full_thr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SSrcBfHeight: u32 {
        [0..15] cfg_s_c_src_bf_height,
        [16..31] cfg_s_y_src_bf_height,
    }
}

emhal::mmio_reg! {
    pub struct VdoSPaddingCtrl: u32 {
        [0..8] s_y_padding_pixel,
        [8..16] s_u_padding_pixel,
        [16..24] s_v_padding_pixel,
        [24..29] s_y_line_pad,
    }
}

emhal::mmio_reg! {
    pub struct VdoSTotalBytecount: u32 {
        [0..32] s_bs_tot_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoSFrameBytecount: u32 {
        [0..32] s_bs_frm_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoSSrcRDbg: u32 {
        [0..13] s_y_read_mbx,
        [16..29] s_uv_read_mbx,
        [30..31] s_axi_r_idle,
        [31..32] s_src_wr_ov_rd,
    }
}

emhal::mmio_reg! {
    pub struct VdoSrcStaClr: u32 {
        [0..1] src_uv_wovr_clr,
        [4..5] src_y_wovr_clr,
        [8..9] s_src_uv_wovr_clr,
        [12..13] s_src_y_wovr_clr,
    }
}

emhal::mmio_reg! {
    pub struct VdoSrcCtrl: u32 {
        [0..4] frm_buf_num,
        [4..5] mfrm_buf_en,
        [5..15] req_period,
        [15..16] no_wait_1st_vsync,
        [16..20] s_frm_buf_num,
        [20..21] s_mfrm_buf_en,
        [21..31] s_req_period,
        [31..32] s_no_wait_1st_vsync,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Refy0BaseAddr: u32 {
        [0..32] cfg_refy0_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Refu0BaseAddr: u32 {
        [0..32] cfg_refu0_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Refv0BaseAddr: u32 {
        [0..32] cfg_refv0_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Refy1BaseAddr: u32 {
        [0..32] cfg_refy1_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Refu1BaseAddr: u32 {
        [0..32] cfg_refu1_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Refv1BaseAddr: u32 {
        [0..32] cfg_refv1_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264ExtBaseAddr: u32 {
        [0..32] cfg_ext_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRefy0BaseAddr: u32 {
        [0..32] cfg_s_refy0_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRefu0BaseAddr: u32 {
        [0..32] cfg_s_refu0_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRefv0BaseAddr: u32 {
        [0..32] cfg_s_refv0_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRefy1BaseAddr: u32 {
        [0..32] cfg_s_refy1_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRefu1BaseAddr: u32 {
        [0..32] cfg_s_refu1_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRefv1BaseAddr: u32 {
        [0..32] cfg_s_refv1_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SExtBaseAddr: u32 {
        [0..32] cfg_s_ext_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoCoreDbg2: u32 {
        [0..1] pframe,
        [4..14] pframe_cnt,
        [16..26] tclock,
        [26..27] r_yref_wait,
        [27..28] r_vref_wait,
        [28..29] w_yrec_wait,
        [29..30] pfch_vbuf_wait,
        [30..31] cabac_wait,
        [31..32] enc_busy,
    }
}

emhal::mmio_reg! {
    pub struct VdoSCoreDbg2: u32 {
        [0..1] s_pframe,
        [4..14] s_pframe_cnt,
        [16..26] s_tclock,
        [26..27] s_r_yref_wait,
        [27..28] s_r_vref_wait,
        [28..29] s_w_yrec_wait,
        [29..30] s_pfch_vbuf_wait,
        [30..31] s_cabac_wait,
        [31..32] s_enc_busy,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264YSrcbfSize: u32 {
        [0..32] cfg_y_srcbf_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264UvSrcbfSize: u32 {
        [0..32] cfg_uv_srcbf_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SYSrcbfSize: u32 {
        [0..32] cfg_s_y_srcbf_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SUvSrcbfSize: u32 {
        [0..32] cfg_s_uv_srcbf_bycnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SrcDbg: u32 {
        [0..1] d2b_sync_en,
        [4..9] old_read_mode,
        [16..17] s_d2b_sync_en,
        [20..25] s_old_read_mode,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264YyD2bAddr: u32 {
        [0..32] yy_d2b_addr_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264UvD2bAddr: u32 {
        [0..32] uv_d2b_addr_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264YyOfstAddr: u32 {
        [0..32] yy_mfrm_ofst_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264UvOfstAddr: u32 {
        [0..32] uv_mfrm_ofst_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264D2bMl: u32 {
        [0..16] yy_d2b_ml_dbg,
        [16..32] uv_d2b_ml_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264LoadMbidxDbg: u32 {
        [0..32] load_mbidx_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264YcMbErr: u32 {
        [0..3] yc_mb_err,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SYyD2bAddr: u32 {
        [0..32] s_yy_d2b_addr_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SUvD2bAddr: u32 {
        [0..32] s_uv_d2b_addr_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SYyOfstAddr: u32 {
        [0..32] s_yy_mfrm_ofst_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SUvOfstAddr: u32 {
        [0..32] s_uv_mfrm_ofst_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SD2bMl: u32 {
        [0..16] s_yy_d2b_ml_dbg,
        [16..32] s_uv_d2b_ml_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SLoadMbidxDbg: u32 {
        [0..32] s_load_mbidx_dbg,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SYcMbErr: u32 {
        [0..3] s_yc_mb_err,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264PfchCtrl: u32 {
        [0..7] cfg_pfch_mbx_s,
        [8..15] cfg_pfch_mby_s,
        [16..17] cfg_pfch_en,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264PfchBase: u32 {
        [0..32] cfg_pfch_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264PfchvBase: u32 {
        [0..32] cfg_pfchv_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SPfchCtrl: u32 {
        [0..7] cfg_s_pfch_mbx_s,
        [8..15] cfg_s_pfch_mby_s,
        [16..17] cfg_s_pfch_en,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SPfchBase: u32 {
        [0..32] cfg_s_pfch_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SPfchvBase: u32 {
        [0..32] cfg_s_pfchv_base_addr,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264HwbackMode: u32 {
        [0..1] cfg_sramck_mode,
        [1..2] cfg_nal_mp_mode,
        [2..3] cfg_s_nal_mp_mode,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264RoiMode: u32 {
        [0..8] cfg_roi_en,
        [8..9] cfg_roi_upd,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264RoiBit: u32 {
        [0..13] cfg_pbit_ratio,
        [13..26] cfg_ibit_ratio,
        [26..32] cfg_ithre,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264RoiQptune: u32 {
        [0..6] cfg_pqp_decr,
        [8..14] cfg_iqp_decr,
        [16..22] cfg_n2r_qp_decr,
        [24..30] cfg_fixqp,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264RoiBittune: u32 {
        [0..5] cfg_roi_pbit_ofst,
        [8..16] cfg_roi_ibit_ofst,
        [16..21] cfg_nroi_pbit_ofst,
        [24..32] cfg_nroi_ibit_ofst,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi0: u32 {
        [0..7] cfg_roi0_mbx_s,
        [8..15] cfg_roi0_mbx_e,
        [16..23] cfg_roi0_mby_s,
        [24..31] cfg_roi0_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi1: u32 {
        [0..7] cfg_roi1_mbx_s,
        [8..15] cfg_roi1_mbx_e,
        [16..23] cfg_roi1_mby_s,
        [24..31] cfg_roi1_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi2: u32 {
        [0..7] cfg_roi2_mbx_s,
        [8..15] cfg_roi2_mbx_e,
        [16..23] cfg_roi2_mby_s,
        [24..31] cfg_roi2_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi3: u32 {
        [0..7] cfg_roi3_mbx_s,
        [8..15] cfg_roi3_mbx_e,
        [16..23] cfg_roi3_mby_s,
        [24..31] cfg_roi3_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi4: u32 {
        [0..7] cfg_roi4_mbx_s,
        [8..15] cfg_roi4_mbx_e,
        [16..23] cfg_roi4_mby_s,
        [24..31] cfg_roi4_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi5: u32 {
        [0..7] cfg_roi5_mbx_s,
        [8..15] cfg_roi5_mbx_e,
        [16..23] cfg_roi5_mby_s,
        [24..31] cfg_roi5_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi6: u32 {
        [0..7] cfg_roi6_mbx_s,
        [8..15] cfg_roi6_mbx_e,
        [16..23] cfg_roi6_mby_s,
        [24..31] cfg_roi6_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Roi7: u32 {
        [0..7] cfg_roi7_mbx_s,
        [8..15] cfg_roi7_mbx_e,
        [16..23] cfg_roi7_mby_s,
        [24..31] cfg_roi7_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoiMode: u32 {
        [0..8] cfg_s_roi_en,
        [8..9] cfg_s_roi_upd,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoiBit: u32 {
        [0..13] cfg_s_pbit_ratio,
        [13..26] cfg_s_ibit_ratio,
        [26..32] cfg_s_ithre,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoiQptune: u32 {
        [0..6] cfg_s_pqp_decr,
        [8..14] cfg_s_iqp_decr,
        [16..22] cfg_s_n2r_qp_decr,
        [24..30] cfg_s_fixqp,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoiBittune: u32 {
        [0..5] cfg_s_roi_pbit_ofst,
        [8..16] cfg_s_roi_ibit_ofst,
        [16..21] cfg_s_nroi_pbit_ofst,
        [24..32] cfg_s_nroi_ibit_ofst,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi0: u32 {
        [0..7] cfg_s_roi0_mbx_s,
        [8..15] cfg_s_roi0_mbx_e,
        [16..23] cfg_s_roi0_mby_s,
        [24..31] cfg_s_roi0_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi1: u32 {
        [0..7] cfg_s_roi1_mbx_s,
        [8..15] cfg_s_roi1_mbx_e,
        [16..23] cfg_s_roi1_mby_s,
        [24..31] cfg_s_roi1_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi2: u32 {
        [0..7] cfg_s_roi2_mbx_s,
        [8..15] cfg_s_roi2_mbx_e,
        [16..23] cfg_s_roi2_mby_s,
        [24..31] cfg_s_roi2_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi3: u32 {
        [0..7] cfg_s_roi3_mbx_s,
        [8..15] cfg_s_roi3_mbx_e,
        [16..23] cfg_s_roi3_mby_s,
        [24..31] cfg_s_roi3_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi4: u32 {
        [0..7] cfg_s_roi4_mbx_s,
        [8..15] cfg_s_roi4_mbx_e,
        [16..23] cfg_s_roi4_mby_s,
        [24..31] cfg_s_roi4_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi5: u32 {
        [0..7] cfg_s_roi5_mbx_s,
        [8..15] cfg_s_roi5_mbx_e,
        [16..23] cfg_s_roi5_mby_s,
        [24..31] cfg_s_roi5_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi6: u32 {
        [0..7] cfg_s_roi6_mbx_s,
        [8..15] cfg_s_roi6_mbx_e,
        [16..23] cfg_s_roi6_mby_s,
        [24..31] cfg_s_roi6_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264SRoi7: u32 {
        [0..7] cfg_s_roi7_mbx_s,
        [8..15] cfg_s_roi7_mbx_e,
        [16..23] cfg_s_roi7_mby_s,
        [24..31] cfg_s_roi7_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264OsdEn: u32 {
        [0..16] cfg_osd_en,
        [16..32] cfg_osd_mssel,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd0: u32 {
        [0..7] cfg_osd0_mbx_s,
        [8..15] cfg_osd0_mbx_e,
        [16..23] cfg_osd0_mby_s,
        [24..31] cfg_osd0_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd1: u32 {
        [0..7] cfg_osd1_mbx_s,
        [8..15] cfg_osd1_mbx_e,
        [16..23] cfg_osd1_mby_s,
        [24..31] cfg_osd1_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd2: u32 {
        [0..7] cfg_osd2_mbx_s,
        [8..15] cfg_osd2_mbx_e,
        [16..23] cfg_osd2_mby_s,
        [24..31] cfg_osd2_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd3: u32 {
        [0..7] cfg_osd3_mbx_s,
        [8..15] cfg_osd3_mbx_e,
        [16..23] cfg_osd3_mby_s,
        [24..31] cfg_osd3_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd4: u32 {
        [0..7] cfg_osd4_mbx_s,
        [8..15] cfg_osd4_mbx_e,
        [16..23] cfg_osd4_mby_s,
        [24..31] cfg_osd4_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd5: u32 {
        [0..7] cfg_osd5_mbx_s,
        [8..15] cfg_osd5_mbx_e,
        [16..23] cfg_osd5_mby_s,
        [24..31] cfg_osd5_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd6: u32 {
        [0..7] cfg_osd6_mbx_s,
        [8..15] cfg_osd6_mbx_e,
        [16..23] cfg_osd6_mby_s,
        [24..31] cfg_osd6_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd7: u32 {
        [0..7] cfg_osd7_mbx_s,
        [8..15] cfg_osd7_mbx_e,
        [16..23] cfg_osd7_mby_s,
        [24..31] cfg_osd7_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd8: u32 {
        [0..7] cfg_osd8_mbx_s,
        [8..15] cfg_osd8_mbx_e,
        [16..23] cfg_osd8_mby_s,
        [24..31] cfg_osd8_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd9: u32 {
        [0..7] cfg_osd9_mbx_s,
        [8..15] cfg_osd9_mbx_e,
        [16..23] cfg_osd9_mby_s,
        [24..31] cfg_osd9_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd10: u32 {
        [0..7] cfg_osd10_mbx_s,
        [8..15] cfg_osd10_mbx_e,
        [16..23] cfg_osd10_mby_s,
        [24..31] cfg_osd10_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd11: u32 {
        [0..7] cfg_osd11_mbx_s,
        [8..15] cfg_osd11_mbx_e,
        [16..23] cfg_osd11_mby_s,
        [24..31] cfg_osd11_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd12: u32 {
        [0..7] cfg_osd12_mbx_s,
        [8..15] cfg_osd12_mbx_e,
        [16..23] cfg_osd12_mby_s,
        [24..31] cfg_osd12_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd13: u32 {
        [0..7] cfg_osd13_mbx_s,
        [8..15] cfg_osd13_mbx_e,
        [16..23] cfg_osd13_mby_s,
        [24..31] cfg_osd13_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd14: u32 {
        [0..7] cfg_osd14_mbx_s,
        [8..15] cfg_osd14_mbx_e,
        [16..23] cfg_osd14_mby_s,
        [24..31] cfg_osd14_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Osd15: u32 {
        [0..7] cfg_osd15_mbx_s,
        [8..15] cfg_osd15_mbx_e,
        [16..23] cfg_osd15_mby_s,
        [24..31] cfg_osd15_mby_e,
    }
}

emhal::mmio_reg! {
    pub struct VdoH264Mv0En: u32 {
        [0..1] cfg_mv0_en,
        [1..2] cfg_s_mv0_en,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchDbgReqcnt: u32 {
        [0..32] dbg_reqcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchDbgRcmd: u32 {
        [0..32] dbg_rcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchDbgWcmd: u32 {
        [0..32] dbg_wcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchDbgWdata: u32 {
        [0..32] dbg_wdata,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchDbgPfchsta: u32 {
        [0..32] dbg_pfchsta,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvDbgReqcnt: u32 {
        [0..32] dbgv_reqcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvDbgRcmd: u32 {
        [0..32] dbgv_rcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvDbgWcmd: u32 {
        [0..32] dbgv_wcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvDbgWdata: u32 {
        [0..32] dbgv_wdata,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvDbgPfchsta: u32 {
        [0..32] dbgv_pfchsta,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchSDbgReqcnt: u32 {
        [0..32] s_dbg_reqcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchSDbgRcmd: u32 {
        [0..32] s_dbg_rcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchSDbgWcmd: u32 {
        [0..32] s_dbg_wcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchSDbgWdata: u32 {
        [0..32] s_dbg_wdata,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchSDbgPfchsta: u32 {
        [0..32] s_dbg_pfchsta,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvSDbgReqcnt: u32 {
        [0..32] s_dbgv_reqcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvSDbgRcmd: u32 {
        [0..32] s_dbgv_rcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvSDbgWcmd: u32 {
        [0..32] s_dbgv_wcmd,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvSDbgWdata: u32 {
        [0..32] s_dbgv_wdata,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchvSDbgPfchsta: u32 {
        [0..32] s_dbgv_pfchsta,
    }
}

emhal::mmio_reg! {
    pub struct Vdonr: u32 {
        [0..6] noiselevel,
        [8..12] temperalfilterstrength,
        [16..22] s_noiselevel,
        [24..28] s_temperalfilterstrength,
    }
}

emhal::mmio_reg! {
    pub struct VdonrMvctrl: u32 {
        [0..17] zmv_pflevel,
    }
}

emhal::mmio_reg! {
    pub struct VdonrStaCtrl: u32 {
        [0..16] ysmlr_thr,
    }
}

emhal::mmio_reg! {
    pub struct VdonrAbsmvSum: u32 {
        [0..22] absmv_sum,
    }
}

emhal::mmio_reg! {
    pub struct VdonrMvlevel: u32 {
        [0..14] nzmv_mbcnt,
        [16..30] ysmlr_mbcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdonrPmbsad: u32 {
        [0..30] pmb_sad,
    }
}

emhal::mmio_reg! {
    pub struct VdonrImbcnt: u32 {
        [0..14] intra_mbcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdonrSMvctrl: u32 {
        [0..17] s_zmv_pflevel,
    }
}

emhal::mmio_reg! {
    pub struct VdonrSStaCtrl: u32 {
        [0..16] s_ysmlr_thr,
    }
}

emhal::mmio_reg! {
    pub struct VdonrSAbsmvSum: u32 {
        [0..22] s_absmv_sum,
    }
}

emhal::mmio_reg! {
    pub struct VdonrSMvlevel: u32 {
        [0..14] s_nzmv_mbcnt,
        [16..30] s_ysmlr_mbcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdonrSPmbsad: u32 {
        [0..30] s_pmb_sad,
    }
}

emhal::mmio_reg! {
    pub struct VdonrSImbcnt: u32 {
        [0..14] s_intra_mbcnt,
    }
}

emhal::mmio_reg! {
    pub struct VdoStufDbg: u32 {
        [0..26] pic_bycnt,
        [26..27] pic_bycnt_ovf,
        [27..28] stuf_ovf,
        [28..29] bincnt_ovf,
    }
}

emhal::mmio_reg! {
    pub struct VdoNalDbg1: u32 {
        [0..26] stuf_cnt,
        [26..32] sh_state,
    }
}

emhal::mmio_reg! {
    pub struct VdoNalDbg2: u32 {
        [0..30] bincntx3,
    }
}

emhal::mmio_reg! {
    pub struct VdoNalDbg3: u32 {
        [0..7] mp_mbx,
        [8..15] mp_mby,
    }
}

emhal::mmio_reg! {
    pub struct VdoRoiDbg: u32 {
        [0..12] roi_pmbbits,
        [16..28] nroi_pmbbits,
    }
}

emhal::mmio_reg! {
    pub struct VdoRcDbg: u32 {
        [0..6] rc_min_qp,
        [8..14] rc_qp,
    }
}

emhal::mmio_reg! {
    pub struct VdoSStufDbg: u32 {
        [0..26] s_pic_bycnt,
        [26..27] s_pic_bycnt_ovf,
        [27..28] s_stuf_ovf,
        [28..29] s_bincnt_ovf,
    }
}

emhal::mmio_reg! {
    pub struct VdoSNalDbg1: u32 {
        [0..26] s_stuf_cnt,
        [26..32] s_sh_state,
    }
}

emhal::mmio_reg! {
    pub struct VdoSNalDbg2: u32 {
        [0..30] s_bincntx3,
    }
}

emhal::mmio_reg! {
    pub struct VdoSNalDbg3: u32 {
        [0..7] s_mp_mbx,
        [8..15] s_mp_mby,
    }
}

emhal::mmio_reg! {
    pub struct VdoSRoiDbg: u32 {
        [0..12] s_roi_pmbbits,
        [16..28] s_nroi_pmbbits,
    }
}

emhal::mmio_reg! {
    pub struct VdoSRcDbg: u32 {
        [0..6] s_rc_min_qp,
        [8..14] s_rc_qp,
    }
}

emhal::mmio_reg! {
    pub struct VdoRefaxiDbg: u32 {
        [0..1] ref2axi_busy,
        [4..6] ref_rfifo_error,
        [6..8] ref_wfifo_error,
        [8..11] ref_rfifo_empty,
        [12..17] ref_wfifo_empty,
    }
}

emhal::mmio_reg! {
    pub struct VdoPfchaxiDbg: u32 {
        [0..1] pfch_rfifo_busy,
        [4..6] pfch_rfifo_error,
        [8..11] pfch_rfifo_empty,
        [12..13] pfchv_rfifo_busy,
        [16..18] pfchv_rfifo_error,
        [20..23] pfchv_rfifo_empty,
    }
}

emhal::mmio_reg! {
    pub struct VdoBsaxiDbg: u32 {
        [0..1] bs_wfifo_busy,
        [4..6] bs_wfifo_error,
        [8..13] bs_wfifo_empty,
        [16..18] bs_st,
    }
}

emhal::mmio_reg! {
    pub struct VdoFrameId: u32 {
        [0..32] frame_id,
    }
}

emhal::mmio_reg! {
    pub struct VdoSVdoFrameId: u32 {
        [0..32] s_frame_id,
    }
}
