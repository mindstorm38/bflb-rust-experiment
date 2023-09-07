//! Hibernate register.

embedded_util::mmio! {
    pub struct Hbn {
        [0x000] rw ctl: HbnCtl,
        [0x004] rw time_l: HbnTimeL,
        [0x008] rw time_h: HbnTimeH,
        [0x00C] rw rtc_time_l: HbnRtcTimeL,
        [0x010] rw rtc_time_h: HbnRtcTimeH,
        [0x014] rw irq_mode: HbnIrqMode,
        [0x018] rw irq_stat: HbnIrqStat,
        [0x01C] rw irq_clr: HbnIrqClr,
        [0x020] rw pir_cfg: HbnPirCfg,
        [0x024] rw pir_vth: HbnPirVth,
        [0x028] rw pir_interval: HbnPirInterval,
        [0x02C] rw bor_cfg: HbnBorCfg,
        [0x030] rw glb: HbnGlb,
        [0x034] rw sram: HbnSram,
        [0x038] rw pad_ctrl_0: HbnPadCtrl0,
        [0x03C] rw pad_ctrl_1: HbnPadCtrl1,
        [0x100] rw rsv0: HbnRsv0,
        [0x104] rw rsv1: HbnRsv1,
        [0x108] rw rsv2: HbnRsv2,
        [0x10C] rw rsv3: HbnRsv3,
        [0x200] rw rc32k_ctrl0: HbnRc32kCtrl0,
        [0x204] rw xtal32k: HbnXtal32k,
        [0x208] rw rtc_rst_ctrl: HbnRtcRstCtrl,
        [0x20C] rw rtc_rst_ctrl2: HbnRtcRstCtrl2,
    }
}

embedded_util::reg! {
    pub struct HbnCtl: u32 {
        [00..04] rtc_ctl,
        [04..05] rtc_dly_option,
        [05..06] pu_ldo18io_aon,
        [07..08] mode,
        [08..09] trap_mode,
        [09..10] pwrdn_hbn_core,
        [12..13] sw_rst,
        [13..14] dis_pwr_off_ldo11,
        [14..15] dis_pwr_off_ldo11_rt,
        [15..19] ldo11_rt_vout_sel,
        [19..23] ldo11_aon_vout_sel,
        [23..24] pu_dcdc_aon,
        [24..25] pu_dcdc18_aon,
        [25..26] pwr_on_option,
        [26..27] sram_slp_option,
        [27..28] sram_slp,
        [28..32] state,
    }
}

embedded_util::reg! {
    pub struct HbnTimeL: u32 {
        [00..32] time_l,
    }
}

embedded_util::reg! {
    pub struct HbnTimeH: u32 {
        [00..08] time_h,
    }
}

embedded_util::reg! {
    pub struct HbnRtcTimeL: u32 {
        [00..32] rtc_time_latch_l,
    }
}

embedded_util::reg! {
    pub struct HbnRtcTimeH: u32 {
        [00..08] rtc_time_latch_h,
        [31..32] rtc_time_latch,
    }
}

embedded_util::reg! {
    pub struct HbnIrqMode: u32 {
        [00..04] pin_wakeup_mode,
        [04..13] pin_wakeup_mask,
        [16..17] en_hw_pu_pd,
        [18..19] irq_bor_en,
        [20..22] irq_acomp0_en,
        [22..24] irq_acomp1_en,
        [24..27] pin_wakeup_sel,
        [27..28] pin_wakeup_en,
    }
}

embedded_util::reg! {
    pub struct HbnIrqStat: u32 {
        [00..32] irq_stat,
    }
}

embedded_util::reg! {
    pub struct HbnIrqClr: u32 {
        [00..32] irq_clr,
    }
}

embedded_util::reg! {
    pub struct HbnPirCfg: u32 {
        [00..02] pir_hpf_sel,
        [02..03] pir_lpf_sel,
        [04..06] pir_dis,
        [07..08] pir_en,
        [08..09] gpadc_cs,
    }
}

embedded_util::reg! {
    pub struct HbnPirVth: u32 {
        [00..14] pir_vth,
    }
}

embedded_util::reg! {
    pub struct HbnPirInterval: u32 {
        [00..12] pir_interval,
    }
}

embedded_util::reg! {
    pub struct HbnBorCfg: u32 {
        [00..01] bod_sel,
        [01..04] bod_vth,
        [04..05] pu_bod,
        [05..06] r_bod_out,
    }
}

embedded_util::reg! {
    pub struct HbnGlb: u32 {
        /// Alias for `root_clk_sel & 1`.
        [00..01] xclk_sel,
        /// Alias for `(root_clk_sel >> 1) & 1`.
        [01..02] mcu_root_sel,
        [00..02] root_clk_sel,
        [02..03] uart_clk_sel,
        [03..05] f32k_sel,
        [07..13] reset_event,
        [13..14] clr_reset_event,
        [15..16] uart_clk_sel2,
        [16..20] sw_ldo11soc_vout_sel_aon,
        [24..28] sw_ldo11_rt_vout_sel,
        [28..32] sw_ldo11_aon_vout_sel,
    }
}

embedded_util::reg! {
    pub struct HbnSram: u32 {
        [06..07] retram_ret,
        [07..08] retram_slp,
    }
}

embedded_util::reg! {
    pub struct HbnPadCtrl0: u32 {
        [00..09] aon_pad_ie_smt,
        [10..19] aon_led_sel,
        [20..29] en_aon_ctrl_gpio,
        [31..32] aon_gpio_iso_mode,
    }
}

embedded_util::reg! {
    pub struct HbnPadCtrl1: u32 {
        [00..09] aon_pad_oe,
        [10..19] aon_pad_pd,
        [20..29] aon_pad_pu,
    }
}

embedded_util::reg! {
    pub struct HbnRsv0: u32 {
        [00..32] rsv0,
    }
}

embedded_util::reg! {
    pub struct HbnRsv1: u32 {
        [00..32] rsv1,
    }
}

embedded_util::reg! {
    pub struct HbnRsv2: u32 {
        [00..32] rsv2,
    }
}

embedded_util::reg! {
    pub struct HbnRsv3: u32 {
        /// Alias for `rsv3 & 0xFF`.
        [00..08] xtal_type,
        /// Alias for `(rsv3 >> 8) & 0xFF`.
        [08..16] xtal_flag,
        [00..32] rsv3,
    }
}

embedded_util::reg! {
    pub struct HbnRc32kCtrl0: u32 {
        [00..01] rc32k_cal_done,
        [01..02] rc32k_rdy,
        [02..03] rc32k_cal_inprogress,
        [03..05] rc32k_cal_div,
        [05..06] rc32k_cal_precharge,
        [06..16] rc32k_dig_code_fr_cal,
        [16..18] rc32k_vref_dly,
        [18..19] rc32k_allow_cal,
        [19..20] rc32k_ext_code_en,
        [20..21] rc32k_cal_en,
        [21..22] pu_rc32k,
        [22..32] rc32k_code_fr_ext,
    }
}

embedded_util::reg! {
    pub struct HbnXtal32k: u32 {
        [02..03] xtal32k_ext_sel,
        [03..05] xtal32k_amp_ctrl,
        [05..07] xtal32k_reg,
        [07..08] xtal32k_outbuf_stre,
        [08..09] xtal32k_otf_short,
        [09..11] xtal32k_inv_stre,
        [11..17] xtal32k_capbank,
        [17..18] xtal32k_ac_cap_short,
        [18..19] pu_xtal32k_buf,
        [19..20] pu_xtal32k,
        [20..21] xtal32k_hiz_en,
        [22..23] dten_xtal32k,
        [23..24] ten_xtal32k,
        [24..25] f32k_sel_rtc,
    }
}

embedded_util::reg! {
    pub struct HbnRtcRstCtrl: u32 {
        [00..16] rtc_rst_wait_cnt_rtc,
        [16..19] rtc_rst_refdiv_rtc,
        [19..32] rtc_rst_ctrl_misc,
    }
}

embedded_util::reg! {
    pub struct HbnRtcRstCtrl2: u32 {
        [00..08] rtc_resv,
        [08..09] en_hw_pu_rc32k,
    }
}
