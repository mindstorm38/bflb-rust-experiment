//! Serial Flash.

embedded_util::mmio! {
    pub struct SfCtrl {
        [0x000] rw _0: SfCtrl0,
        [0x000] rw if_sahb_0: SfCtrlIfSahb0,
        [0x000] rw io_dly_0: SfCtrlIoDly0,
        [0x000] rw sf_aes_key_0: SfCtrlSfAesKey0,
        [0x004] rw _1: SfCtrl1,
        [0x004] rw if_sahb_1: SfCtrlIfSahb1,
        [0x004] rw io_dly_1: SfCtrlIoDly1,
        [0x004] rw sf_aes_key_1: SfCtrlSfAesKey1,
        [0x008] rw sf_if_sahb_0: SfCtrlSfIfSahb0,
        [0x008] rw if_sahb_2: SfCtrlIfSahb2,
        [0x008] rw if1_sahb: SfCtrlIf1Sahb,
        [0x008] rw io_dly_2: SfCtrlIoDly2,
        [0x008] rw sf_aes_key_2: SfCtrlSfAesKey2,
        [0x00C] rw sf_if_sahb_1: SfCtrlSfIfSahb1,
        [0x00C] rw io_dly_3: SfCtrlIoDly3,
        [0x00C] rw sf_aes_key_3: SfCtrlSfAesKey3,
        [0x010] rw sf_if_sahb_2: SfCtrlSfIfSahb2,
        [0x010] rw io_dly_4: SfCtrlIoDly4,
        [0x010] rw sf_aes_key_4: SfCtrlSfAesKey4,
        [0x014] rw sf_if_iahb_0: SfCtrlSfIfIahb0,
        [0x014] rw sf_aes_key_5: SfCtrlSfAesKey5,
        [0x018] rw sf_if_iahb_1: SfCtrlSfIfIahb1,
        [0x018] rw sf_aes_key_6: SfCtrlSfAesKey6,
        [0x01C] rw sf_if_iahb_2: SfCtrlSfIfIahb2,
        [0x01C] rw sf_aes_key_7: SfCtrlSfAesKey7,
        [0x020] rw sf_if_status_0: SfCtrlSfIfStatus0,
        [0x020] rw sf_aes_iv_w0: SfCtrlSfAesIvW0,
        [0x024] rw sf_if_status_1: SfCtrlSfIfStatus1,
        [0x024] rw sf_aes_iv_w1: SfCtrlSfAesIvW1,
        [0x028] rw sf_aes: SfCtrlSfAes,
        [0x028] rw sf_aes_iv_w2: SfCtrlSfAesIvW2,
        [0x02C] rw sf_ahb2sif_status: SfCtrlSfAhb2sifStatus,
        [0x02C] rw sf_aes_iv_w3: SfCtrlSfAesIvW3,
        [0x030] rw sf_if_io_dly_0: SfCtrlSfIfIoDly0,
        [0x030] rw if_io_dly_1: SfCtrlIfIoDly1,
        [0x030] rw sf_aes_start: SfCtrlSfAesStart,
        [0x034] rw sf_if_io_dly_1: SfCtrlSfIfIoDly1,
        [0x034] rw sf_aes_end: SfCtrlSfAesEnd,
        [0x038] rw sf_if_io_dly_2: SfCtrlSfIfIoDly2,
        [0x03C] rw sf_if_io_dly_3: SfCtrlSfIfIoDly3,
        [0x040] rw sf_if_io_dly_4: SfCtrlSfIfIoDly4,
        [0x044] rw sf_reserved: SfCtrlSfReserved,
        [0x048] rw sf2_if_io_dly_0: SfCtrlSf2IfIoDly0,
        [0x048] rw if_io_dly_2: SfCtrlIfIoDly2,
        [0x04C] rw sf2_if_io_dly_1: SfCtrlSf2IfIoDly1,
        [0x050] rw sf2_if_io_dly_2: SfCtrlSf2IfIoDly2,
        [0x054] rw sf2_if_io_dly_3: SfCtrlSf2IfIoDly3,
        [0x058] rw sf2_if_io_dly_4: SfCtrlSf2IfIoDly4,
        [0x05C] rw sf3_if_io_dly_0: SfCtrlSf3IfIoDly0,
        [0x05C] rw if_io_dly_3: SfCtrlIfIoDly3,
        [0x060] rw sf3_if_io_dly_1: SfCtrlSf3IfIoDly1,
        [0x064] rw sf3_if_io_dly_2: SfCtrlSf3IfIoDly2,
        [0x068] rw sf3_if_io_dly_3: SfCtrlSf3IfIoDly3,
        [0x06C] rw sf3_if_io_dly_4: SfCtrlSf3IfIoDly4,
        [0x070] rw _2: SfCtrl2,
        [0x074] rw _3: SfCtrl3,
        [0x078] rw sf_if_iahb_3: SfCtrlSfIfIahb3,
        [0x07C] rw sf_if_iahb_4: SfCtrlSfIfIahb4,
        [0x080] rw sf_if_iahb_5: SfCtrlSfIfIahb5,
        [0x084] rw sf_if_iahb_6: SfCtrlSfIfIahb6,
        [0x088] rw sf_if_iahb_7: SfCtrlSfIfIahb7,
        [0x08C] rw sf_if_iahb_8: SfCtrlSfIfIahb8,
        [0x090] rw sf_if_iahb_9: SfCtrlSfIfIahb9,
        [0x094] rw sf_if_iahb_10: SfCtrlSfIfIahb10,
        [0x098] rw sf_if_iahb_11: SfCtrlSfIfIahb11,
        [0x09C] rw sf_if_iahb_12: SfCtrlSfIfIahb12,
        [0x0A0] rw sf_id0_offset: SfCtrlSfId0Offset,
        [0x0A4] rw sf_id1_offset: SfCtrlSfId1Offset,
        [0x0A8] rw sf_bk2_id0_offset: SfCtrlSfBk2Id0Offset,
        [0x0AC] rw sf_bk2_id1_offset: SfCtrlSfBk2Id1Offset,
        [0x0B0] rw sf_dbg: SfCtrlSfDbg,
        [0x0C0] rw sf_if2_ctrl_0: SfCtrlSfIf2Ctrl0,
        [0x0C4] rw sf_if2_ctrl_1: SfCtrlSfIf2Ctrl1,
        [0x0C8] rw sf_if2_sahb_0: SfCtrlSfIf2Sahb0,
        [0x0C8] rw if2_sahb: SfCtrlIf2Sahb,
        [0x0CC] rw sf_if2_sahb_1: SfCtrlSfIf2Sahb1,
        [0x0D0] rw sf_if2_sahb_2: SfCtrlSfIf2Sahb2,
        [0x100] rw prot_en_rd: SfCtrlProtEnRd,
        [0x104] rw prot_en: SfCtrlProtEn,
        [0x200] rw sf_aes_key_r0_0: SfCtrlSfAesKeyR00,
        [0x200] rw aes_region: SfCtrlAesRegion,
        [0x204] rw sf_aes_key_r0_1: SfCtrlSfAesKeyR01,
        [0x208] rw sf_aes_key_r0_2: SfCtrlSfAesKeyR02,
        [0x20C] rw sf_aes_key_r0_3: SfCtrlSfAesKeyR03,
        [0x210] rw sf_aes_key_r0_4: SfCtrlSfAesKeyR04,
        [0x214] rw sf_aes_key_r0_5: SfCtrlSfAesKeyR05,
        [0x218] rw sf_aes_key_r0_6: SfCtrlSfAesKeyR06,
        [0x21C] rw sf_aes_key_r0_7: SfCtrlSfAesKeyR07,
        [0x220] rw sf_aes_iv_r0_w0: SfCtrlSfAesIvR0W0,
        [0x224] rw sf_aes_iv_r0_w1: SfCtrlSfAesIvR0W1,
        [0x228] rw sf_aes_iv_r0_w2: SfCtrlSfAesIvR0W2,
        [0x22C] rw sf_aes_iv_r0_w3: SfCtrlSfAesIvR0W3,
        [0x230] rw sf_aes_r0_start: SfCtrlSfAesR0Start,
        [0x234] rw sf_aes_r0_end: SfCtrlSfAesR0End,
        [0x280] rw sf_aes_key_r1_0: SfCtrlSfAesKeyR10,
        [0x284] rw sf_aes_key_r1_1: SfCtrlSfAesKeyR11,
        [0x288] rw sf_aes_key_r1_2: SfCtrlSfAesKeyR12,
        [0x28C] rw sf_aes_key_r1_3: SfCtrlSfAesKeyR13,
        [0x290] rw sf_aes_key_r1_4: SfCtrlSfAesKeyR14,
        [0x294] rw sf_aes_key_r1_5: SfCtrlSfAesKeyR15,
        [0x298] rw sf_aes_key_r1_6: SfCtrlSfAesKeyR16,
        [0x29C] rw sf_aes_key_r1_7: SfCtrlSfAesKeyR17,
        [0x2A0] rw sf_aes_iv_r1_w0: SfCtrlSfAesIvR1W0,
        [0x2A4] rw sf_aes_iv_r1_w1: SfCtrlSfAesIvR1W1,
        [0x2A8] rw sf_aes_iv_r1_w2: SfCtrlSfAesIvR1W2,
        [0x2AC] rw sf_aes_iv_r1_w3: SfCtrlSfAesIvR1W3,
        [0x2B0] rw sf_aes_r1_start: SfCtrlSfAesR1Start,
        [0x2B4] rw sf_aes_r1_end: SfCtrlSfAesR1End,
        [0x300] rw sf_aes_key_r2_0: SfCtrlSfAesKeyR20,
        [0x304] rw sf_aes_key_r2_1: SfCtrlSfAesKeyR21,
        [0x308] rw sf_aes_key_r2_2: SfCtrlSfAesKeyR22,
        [0x30C] rw sf_aes_key_r2_3: SfCtrlSfAesKeyR23,
        [0x310] rw sf_aes_key_r2_4: SfCtrlSfAesKeyR24,
        [0x314] rw sf_aes_key_r2_5: SfCtrlSfAesKeyR25,
        [0x318] rw sf_aes_key_r2_6: SfCtrlSfAesKeyR26,
        [0x31C] rw sf_aes_key_r2_7: SfCtrlSfAesKeyR27,
        [0x320] rw sf_aes_iv_r2_w0: SfCtrlSfAesIvR2W0,
        [0x324] rw sf_aes_iv_r2_w1: SfCtrlSfAesIvR2W1,
        [0x328] rw sf_aes_iv_r2_w2: SfCtrlSfAesIvR2W2,
        [0x32C] rw sf_aes_iv_r2_w3: SfCtrlSfAesIvR2W3,
        [0x330] rw sf_aes_r2_start: SfCtrlSfAesR2Start,
        [0x334] rw sf_aes_r2_end: SfCtrlSfAesR2End,
    }
}

embedded_util::reg! {
    pub struct SfCtrl0: u32 {
        [02..03] sf_clk_sf_rx_inv_sel,
        [03..04] sf_clk_out_gate_en,
        [04..05] sf_clk_out_inv_sel,
        [08..11] sf_if_read_dly_n,
        [11..12] sf_if_read_dly_en,
        [16..17] sf_if_int,
        [17..18] sf_if_int_clr,
        [18..19] sf_if_int_set,
        [19..20] sf_if_32b_adr_en,
        [20..21] sf_aes_dout_endian,
        [21..22] sf_aes_din_endian,
        [22..23] sf_aes_key_endian,
        [23..24] sf_aes_iv_endian,
        [24..32] sf_id,
    }
}

embedded_util::reg! {
    pub struct SfCtrl1: u32 {
        [00..08] sf_if_sr_pat_mask,
        [08..16] sf_if_sr_pat,
        [16..17] sf_if_sr_int,
        [17..18] sf_if_sr_int_en,
        [18..19] sf_if_sr_int_set,
        [20..23] sf_if_0_ack_lat,
        [23..24] sf_ahb2sif_diswrap,
        [24..25] sf_if_reg_hold,
        [25..26] sf_if_reg_wp,
        [26..27] sf_ahb2sif_stopped,
        [27..28] sf_ahb2sif_stop,
        [28..29] sf_if_fn_sel,
        [29..30] sf_if_en,
        [30..31] sf_ahb2sif_en,
        [31..32] sf_ahb2sram_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfSahb0: u32 {
        [00..01] sf_if_busy,
        [01..02] sf_if_0_trig,
        [02..12] sf_if_0_dat_byte,
        [12..17] sf_if_0_dmy_byte,
        [17..20] sf_if_0_adr_byte,
        [20..23] sf_if_0_cmd_byte,
        [23..24] sf_if_0_dat_rw,
        [24..25] sf_if_0_dat_en,
        [25..26] sf_if_0_dmy_en,
        [26..27] sf_if_0_adr_en,
        [27..28] sf_if_0_cmd_en,
        [28..31] sf_if_0_spi_mode,
        [31..32] sf_if_0_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfSahb1: u32 {
        [00..32] sf_if_0_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfSahb2: u32 {
        [00..32] sf_if_0_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb0: u32 {
        [12..17] sf_if_1_dmy_byte,
        [17..20] sf_if_1_adr_byte,
        [20..23] sf_if_1_cmd_byte,
        [23..24] sf_if_1_dat_rw,
        [24..25] sf_if_1_dat_en,
        [25..26] sf_if_1_dmy_en,
        [26..27] sf_if_1_adr_en,
        [27..28] sf_if_1_cmd_en,
        [28..31] sf_if_1_spi_mode,
        [31..32] sf_if_1_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb1: u32 {
        [00..32] sf_if_1_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb2: u32 {
        [00..32] sf_if_1_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfStatus0: u32 {
        [00..32] sf_if_status_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfStatus1: u32 {
        [00..32] sf_if_status_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAes: u32 {
        [00..01] sf_aes_en,
        [01..03] sf_aes_mode,
        [03..04] sf_aes_blk_mode,
        [04..05] sf_aes_xts_key_opt,
        [05..32] sf_aes_status,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAhb2sifStatus: u32 {
        [00..32] sf_ahb2sif_status,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIoDly0: u32 {
        [00..02] sf_cs_dly_sel,
        [02..04] sf_cs2_dly_sel,
        [08..10] sf_clk_out_dly_sel,
        [26..28] sf_dqs_oe_dly_sel,
        [28..30] sf_dqs_di_dly_sel,
        [30..32] sf_dqs_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIoDly1: u32 {
        [00..02] sf_io_0_oe_dly_sel,
        [08..10] sf_io_0_di_dly_sel,
        [16..18] sf_io_0_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIoDly2: u32 {
        [00..02] sf_io_1_oe_dly_sel,
        [08..10] sf_io_1_di_dly_sel,
        [16..18] sf_io_1_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIoDly3: u32 {
        [00..02] sf_io_2_oe_dly_sel,
        [08..10] sf_io_2_di_dly_sel,
        [16..18] sf_io_2_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIoDly4: u32 {
        [00..02] sf_io_3_oe_dly_sel,
        [08..10] sf_io_3_di_dly_sel,
        [16..18] sf_io_3_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfReserved: u32 {
        [00..32] sf_reserved,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf2IfIoDly0: u32 {
        [00..02] sf2_cs_dly_sel,
        [02..04] sf2_cs2_dly_sel,
        [08..10] sf2_clk_out_dly_sel,
        [26..28] sf2_dqs_oe_dly_sel,
        [28..30] sf2_dqs_di_dly_sel,
        [30..32] sf2_dqs_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf2IfIoDly1: u32 {
        [00..02] sf2_io_0_oe_dly_sel,
        [08..10] sf2_io_0_di_dly_sel,
        [16..18] sf2_io_0_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf2IfIoDly2: u32 {
        [00..02] sf2_io_1_oe_dly_sel,
        [08..10] sf2_io_1_di_dly_sel,
        [16..18] sf2_io_1_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf2IfIoDly3: u32 {
        [00..02] sf2_io_2_oe_dly_sel,
        [08..10] sf2_io_2_di_dly_sel,
        [16..18] sf2_io_2_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf2IfIoDly4: u32 {
        [00..02] sf2_io_3_oe_dly_sel,
        [08..10] sf2_io_3_di_dly_sel,
        [16..18] sf2_io_3_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf3IfIoDly0: u32 {
        [00..02] sf3_cs_dly_sel,
        [02..04] sf3_cs2_dly_sel,
        [08..10] sf3_clk_out_dly_sel,
        [26..28] sf3_dqs_oe_dly_sel,
        [28..30] sf3_dqs_di_dly_sel,
        [30..32] sf3_dqs_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf3IfIoDly1: u32 {
        [00..02] sf3_io_0_oe_dly_sel,
        [08..10] sf3_io_0_di_dly_sel,
        [16..18] sf3_io_0_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf3IfIoDly2: u32 {
        [00..02] sf3_io_1_oe_dly_sel,
        [08..10] sf3_io_1_di_dly_sel,
        [16..18] sf3_io_1_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf3IfIoDly3: u32 {
        [00..02] sf3_io_2_oe_dly_sel,
        [08..10] sf3_io_2_di_dly_sel,
        [16..18] sf3_io_2_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSf3IfIoDly4: u32 {
        [00..02] sf3_io_3_oe_dly_sel,
        [08..10] sf3_io_3_di_dly_sel,
        [16..18] sf3_io_3_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrl2: u32 {
        [00..02] sf_if_pad_sel,
        [03..04] sf_if_pad_sel_lock,
        [04..05] sf_if_dtr_en,
        [05..06] sf_if_dqs_en,
        [06..07] sf_if_trig_wr_prot,
        [07..08] sf_id_offset_lock,
        [25..26] sf_ahb2sif_remap_lock,
        [26..28] sf_ahb2sif_remap,
        [28..29] sf_if_bk_swap,
        [29..30] sf_if_bk2_mode,
        [30..31] sf_if_bk2_en,
        [31..32] sf_if_0_bk_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrl3: u32 {
        [00..04] sf_cmds_2_wrap_len,
        [04..05] sf_cmds_2_en,
        [05..08] sf_cmds_2_bt_dly,
        [08..09] sf_cmds_2_bt_en,
        [09..10] sf_cmds_2_wrap_q_ini,
        [10..12] sf_cmds_2_wrap_mode,
        [12..13] sf_cmds_2_wrap_q,
        [13..17] sf_cmds_1_wrap_len,
        [17..18] sf_cmds_1_en,
        [18..20] sf_cmds_1_wrap_mode,
        [20..21] sf_cmds_core_en,
        [29..32] sf_if_1_ack_lat,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb3: u32 {
        [12..17] sf_if_2_dmy_byte,
        [17..20] sf_if_2_adr_byte,
        [20..23] sf_if_2_cmd_byte,
        [23..24] sf_if_2_dat_rw,
        [24..25] sf_if_2_dat_en,
        [25..26] sf_if_2_dmy_en,
        [26..27] sf_if_2_adr_en,
        [27..28] sf_if_2_cmd_en,
        [28..31] sf_if_2_spi_mode,
        [31..32] sf_if_2_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb4: u32 {
        [00..32] sf_if_2_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb5: u32 {
        [00..32] sf_if_2_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb6: u32 {
        [17..20] sf_if_3_adr_byte,
        [20..23] sf_if_3_cmd_byte,
        [26..27] sf_if_3_adr_en,
        [27..28] sf_if_3_cmd_en,
        [28..31] sf_if_3_spi_mode,
        [31..32] sf_if_3_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb7: u32 {
        [00..32] sf_if_3_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb8: u32 {
        [00..32] sf_if_3_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb9: u32 {
        [12..17] sf_if_4_dmy_byte,
        [17..20] sf_if_4_adr_byte,
        [20..23] sf_if_4_cmd_byte,
        [23..24] sf_if_4_dat_rw,
        [24..25] sf_if_4_dat_en,
        [25..26] sf_if_4_dmy_en,
        [26..27] sf_if_4_adr_en,
        [27..28] sf_if_4_cmd_en,
        [28..31] sf_if_4_spi_mode,
        [31..32] sf_if_4_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb10: u32 {
        [00..32] sf_if_4_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb11: u32 {
        [00..32] sf_if_4_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIfIahb12: u32 {
        [02..03] sf2_clk_sf_rx_inv_sel,
        [03..04] sf2_clk_sf_rx_inv_src,
        [04..05] sf2_clk_out_inv_sel,
        [05..06] sf3_clk_out_inv_sel,
        [08..11] sf2_if_read_dly_n,
        [11..12] sf2_if_read_dly_en,
        [12..13] sf2_if_read_dly_src,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfId0Offset: u32 {
        [00..28] sf_id0_offset,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfId1Offset: u32 {
        [00..28] sf_id1_offset,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfBk2Id0Offset: u32 {
        [00..28] sf_bk2_id0_offset,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfBk2Id1Offset: u32 {
        [00..28] sf_bk2_id1_offset,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfDbg: u32 {
        [00..05] sf_autoload_st,
        [05..06] sf_autoload_st_done,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIf2Ctrl0: u32 {
        [02..03] sf_clk_sf_if2_rx_inv_sel,
        [08..11] sf_if2_read_dly_n,
        [11..12] sf_if2_read_dly_en,
        [16..17] sf_if2_int,
        [17..18] sf_if2_int_clr,
        [18..19] sf_if2_int_set,
        [23..24] sf_if2_replace_sf1,
        [24..25] sf_if2_replace_sf2,
        [25..26] sf_if2_replace_sf3,
        [26..28] sf_if2_pad_sel,
        [28..29] sf_if2_bk_swap,
        [29..30] sf_if2_bk2_mode,
        [30..31] sf_if2_bk2_en,
        [31..32] sf_if2_bk_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIf2Ctrl1: u32 {
        [00..08] sf_if2_sr_pat_mask,
        [08..16] sf_if2_sr_pat,
        [16..17] sf_if2_sr_int,
        [17..18] sf_if2_sr_int_en,
        [18..19] sf_if2_sr_int_set,
        [20..23] sf_if2_ack_lat,
        [24..25] sf_if2_reg_hold,
        [25..26] sf_if2_reg_wp,
        [28..29] sf_if2_fn_sel,
        [29..30] sf_if2_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIf2Sahb0: u32 {
        [00..01] sf_if2_busy,
        [01..02] sf_if2_0_trig,
        [02..12] sf_if2_0_dat_byte,
        [12..17] sf_if2_0_dmy_byte,
        [17..20] sf_if2_0_adr_byte,
        [20..23] sf_if2_0_cmd_byte,
        [23..24] sf_if2_0_dat_rw,
        [24..25] sf_if2_0_dat_en,
        [25..26] sf_if2_0_dmy_en,
        [26..27] sf_if2_0_adr_en,
        [27..28] sf_if2_0_cmd_en,
        [28..31] sf_if2_0_spi_mode,
        [31..32] sf_if2_0_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIf2Sahb1: u32 {
        [00..32] sf_if2_0_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfIf2Sahb2: u32 {
        [00..32] sf_if2_0_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlProtEnRd: u32 {
        [01..02] id0_en_rd,
        [02..03] id1_en_rd,
        [28..29] sf_sec_tzsid_lock,
        [29..30] sf_if2_0_trig_wr_lock,
        [30..31] sf_if_0_trig_wr_lock,
        [31..32] sf_dbg_dis,
    }
}

embedded_util::reg! {
    pub struct SfCtrlProtEn: u32 {
        [01..02] id0_en,
        [02..03] id1_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR00: u32 {
        [00..32] sf_aes_key_r0_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR01: u32 {
        [00..32] sf_aes_key_r0_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR02: u32 {
        [00..32] sf_aes_key_r0_2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR03: u32 {
        [00..32] sf_aes_key_r0_3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR04: u32 {
        [00..32] sf_aes_key_r0_4,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR05: u32 {
        [00..32] sf_aes_key_r0_5,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR06: u32 {
        [00..32] sf_aes_key_r0_6,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR07: u32 {
        [00..32] sf_aes_key_r0_7,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR0W0: u32 {
        [00..32] sf_aes_iv_r0_w0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR0W1: u32 {
        [00..32] sf_aes_iv_r0_w1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR0W2: u32 {
        [00..32] sf_aes_iv_r0_w2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR0W3: u32 {
        [00..32] sf_aes_iv_r0_w3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesR0Start: u32 {
        [00..19] sf_aes_region_r0_start,
        [29..30] sf_aes_region_r0_hw_key_en,
        [30..31] sf_aes_region_r0_en,
        [31..32] sf_aes_region_r0_lock,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesR0End: u32 {
        [00..19] sf_aes_region_r0_end,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR10: u32 {
        [00..32] sf_aes_key_r1_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR11: u32 {
        [00..32] sf_aes_key_r1_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR12: u32 {
        [00..32] sf_aes_key_r1_2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR13: u32 {
        [00..32] sf_aes_key_r1_3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR14: u32 {
        [00..32] sf_aes_key_r1_4,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR15: u32 {
        [00..32] sf_aes_key_r1_5,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR16: u32 {
        [00..32] sf_aes_key_r1_6,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR17: u32 {
        [00..32] sf_aes_key_r1_7,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR1W0: u32 {
        [00..32] sf_aes_iv_r1_w0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR1W1: u32 {
        [00..32] sf_aes_iv_r1_w1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR1W2: u32 {
        [00..32] sf_aes_iv_r1_w2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR1W3: u32 {
        [00..32] sf_aes_iv_r1_w3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesR1Start: u32 {
        [00..19] sf_aes_r1_start,
        [29..30] sf_aes_r1_hw_key_en,
        [30..31] sf_aes_r1_en,
        [31..32] sf_aes_r1_lock,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesR1End: u32 {
        [00..19] sf_aes_r1_end,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR20: u32 {
        [00..32] sf_aes_key_r2_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR21: u32 {
        [00..32] sf_aes_key_r2_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR22: u32 {
        [00..32] sf_aes_key_r2_2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR23: u32 {
        [00..32] sf_aes_key_r2_3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR24: u32 {
        [00..32] sf_aes_key_r2_4,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR25: u32 {
        [00..32] sf_aes_key_r2_5,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR26: u32 {
        [00..32] sf_aes_key_r2_6,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKeyR27: u32 {
        [00..32] sf_aes_key_r2_7,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR2W0: u32 {
        [00..32] sf_aes_iv_r2_w0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR2W1: u32 {
        [00..32] sf_aes_iv_r2_w1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR2W2: u32 {
        [00..32] sf_aes_iv_r2_w2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvR2W3: u32 {
        [00..32] sf_aes_iv_r2_w3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesR2Start: u32 {
        [00..19] sf_aes_r2_start,
        [29..30] sf_aes_r2_hw_key_en,
        [30..31] sf_aes_r2_en,
        [31..32] sf_aes_r2_lock,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesR2End: u32 {
        [00..19] sf_aes_r2_end,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIfSahb0: u32 {
        [00..01] if_busy,
        [01..02] if_0_trig,
        [02..12] if_0_dat_byte,
        [12..17] if_0_dmy_byte,
        [17..20] if_0_adr_byte,
        [20..23] if_0_cmd_byte,
        [23..24] if_0_dat_rw,
        [24..25] if_0_dat_en,
        [25..26] if_0_dmy_en,
        [26..27] if_0_adr_en,
        [27..28] if_0_cmd_en,
        [28..31] if_0_spi_mode,
        [31..32] if_0_qpi_mode_en,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIfSahb1: u32 {
        [00..32] if_0_cmd_buf_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIfSahb2: u32 {
        [00..32] if_0_cmd_buf_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIf1Sahb: u32 {
    }
}

embedded_util::reg! {
    pub struct SfCtrlIf2Sahb: u32 {
    }
}

embedded_util::reg! {
    pub struct SfCtrlIoDly0: u32 {
        [00..02] cs_dly_sel,
        [02..04] cs2_dly_sel,
        [08..10] clk_out_dly_sel,
        [26..28] dqs_oe_dly_sel,
        [28..30] dqs_di_dly_sel,
        [30..32] dqs_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIoDly1: u32 {
        [00..02] io_0_oe_dly_sel,
        [08..10] io_0_di_dly_sel,
        [16..18] io_0_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIoDly2: u32 {
        [00..02] io_1_oe_dly_sel,
        [08..10] io_1_di_dly_sel,
        [16..18] io_1_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIoDly3: u32 {
        [00..02] io_2_oe_dly_sel,
        [08..10] io_2_di_dly_sel,
        [16..18] io_2_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIoDly4: u32 {
        [00..02] io_3_oe_dly_sel,
        [08..10] io_3_di_dly_sel,
        [16..18] io_3_do_dly_sel,
    }
}

embedded_util::reg! {
    pub struct SfCtrlIfIoDly1: u32 {
    }
}

embedded_util::reg! {
    pub struct SfCtrlIfIoDly2: u32 {
    }
}

embedded_util::reg! {
    pub struct SfCtrlIfIoDly3: u32 {
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey0: u32 {
        [00..32] sf_aes_key_0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey1: u32 {
        [00..32] sf_aes_key_1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey2: u32 {
        [00..32] sf_aes_key_2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey3: u32 {
        [00..32] sf_aes_key_3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey4: u32 {
        [00..32] sf_aes_key_4,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey5: u32 {
        [00..32] sf_aes_key_5,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey6: u32 {
        [00..32] sf_aes_key_6,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesKey7: u32 {
        [00..32] sf_aes_key_7,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvW0: u32 {
        [00..32] sf_aes_iv_w0,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvW1: u32 {
        [00..32] sf_aes_iv_w1,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvW2: u32 {
        [00..32] sf_aes_iv_w2,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesIvW3: u32 {
        [00..32] sf_aes_iv_w3,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesStart: u32 {
        [00..19] sf_aes_region_start,
        [29..30] sf_aes_region_hw_key_en,
        [30..31] sf_aes_region_en,
        [31..32] sf_aes_region_lock,
    }
}

embedded_util::reg! {
    pub struct SfCtrlSfAesEnd: u32 {
        [00..19] sf_aes_region_end,
    }
}

embedded_util::reg! {
    pub struct SfCtrlAesRegion: u32 {
    }
}
