//! Hibernate register.

emhal::mmio_struct! {
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

emhal::mmio_reg! {
    pub struct HbnCtl: u32 {
        [0..4] rtc_ctl,
        [4..5] rtc_dly_option,
        [5..6] pu_ldo18io_aon,
        [7..8] mode,
        [8..9] trap_mode,
        [9..10] pwrdn_hbn_core,
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

emhal::mmio_reg! {
    pub struct HbnTimeL: u32 {
        [0..32] time_l,
    }
}

emhal::mmio_reg! {
    pub struct HbnTimeH: u32 {
        [0..8] time_h,
    }
}

emhal::mmio_reg! {
    pub struct HbnRtcTimeL: u32 {
        [0..32] rtc_time_latch_l,
    }
}

emhal::mmio_reg! {
    pub struct HbnRtcTimeH: u32 {
        [0..8] rtc_time_latch_h,
        [31..32] rtc_time_latch,
    }
}

emhal::mmio_reg! {
    pub struct HbnIrqMode: u32 {
        [0..4] pin_wakeup_mode,
        [4..13] pin_wakeup_mask,
        [16..17] en_hw_pu_pd,
        [18..19] irq_bor_en,
        [20..22] irq_acomp0_en,
        [22..24] irq_acomp1_en,
        [24..27] pin_wakeup_sel,
        [27..28] pin_wakeup_en,
    }
}

emhal::mmio_reg! {
    pub struct HbnIrqStat: u32 {
        [0..32] irq_stat,
    }
}

emhal::mmio_reg! {
    pub struct HbnIrqClr: u32 {
        [0..32] irq_clr,
    }
}

emhal::mmio_reg! {
    pub struct HbnPirCfg: u32 {
        [0..2] pir_hpf_sel,
        [2..3] pir_lpf_sel,
        [4..6] pir_dis,
        [7..8] pir_en,
        [8..9] gpadc_cs,
    }
}

emhal::mmio_reg! {
    pub struct HbnPirVth: u32 {
        [0..14] pir_vth,
    }
}

emhal::mmio_reg! {
    pub struct HbnPirInterval: u32 {
        [0..12] pir_interval,
    }
}

emhal::mmio_reg! {
    pub struct HbnBorCfg: u32 {
        [0..1] bod_sel,
        [1..4] bod_vth,
        [4..5] pu_bod,
        [5..6] r_bod_out,
    }
}

emhal::mmio_reg! {
    pub struct HbnGlb: u32 {
        /// Alias for `root_clk_sel & 1`.
        [0..1] xclk_sel,
        /// Alias for `(root_clk_sel >> 1) & 1`.
        [1..2] mcu_root_sel,
        [0..2] root_clk_sel,
        [2..3] uart_clk_sel,
        [3..5] f32k_sel,
        [7..13] reset_event,
        [13..14] clr_reset_event,
        [15..16] uart_clk_sel2,
        [16..20] sw_ldo11soc_vout_sel_aon,
        [24..28] sw_ldo11_rt_vout_sel,
        [28..32] sw_ldo11_aon_vout_sel,
    }
}

emhal::mmio_reg! {
    pub struct HbnSram: u32 {
        [6..7] retram_ret,
        [7..8] retram_slp,
    }
}

emhal::mmio_reg! {
    pub struct HbnPadCtrl0: u32 {
        [0..9] aon_pad_ie_smt,
        [10..19] aon_led_sel,
        [20..29] en_aon_ctrl_gpio,
        [31..32] aon_gpio_iso_mode,
    }
}

emhal::mmio_reg! {
    pub struct HbnPadCtrl1: u32 {
        [0..9] aon_pad_oe,
        [10..19] aon_pad_pd,
        [20..29] aon_pad_pu,
    }
}

emhal::mmio_reg! {
    pub struct HbnRsv0: u32 {
        [0..32] rsv0,
    }
}

emhal::mmio_reg! {
    pub struct HbnRsv1: u32 {
        [0..32] rsv1,
    }
}

emhal::mmio_reg! {
    pub struct HbnRsv2: u32 {
        [0..32] rsv2,
    }
}

emhal::mmio_reg! {
    pub struct HbnRsv3: u32 {
        /// Alias for `rsv3 & 0xFF`.
        [0..8] xtal_type,
        /// Alias for `(rsv3 >> 8) & 0xFF`.
        [8..16] xtal_flag,
        [0..32] rsv3,
    }
}

emhal::mmio_reg! {
    pub struct HbnRc32kCtrl0: u32 {
        [0..1] rc32k_cal_done,
        [1..2] rc32k_rdy,
        [2..3] rc32k_cal_inprogress,
        [3..5] rc32k_cal_div,
        [5..6] rc32k_cal_precharge,
        [6..16] rc32k_dig_code_fr_cal,
        [16..18] rc32k_vref_dly,
        [18..19] rc32k_allow_cal,
        [19..20] rc32k_ext_code_en,
        [20..21] rc32k_cal_en,
        [21..22] pu_rc32k,
        [22..32] rc32k_code_fr_ext,
    }
}

emhal::mmio_reg! {
    pub struct HbnXtal32k: u32 {
        [2..3] xtal32k_ext_sel,
        [3..5] xtal32k_amp_ctrl,
        [5..7] xtal32k_reg,
        [7..8] xtal32k_outbuf_stre,
        [8..9] xtal32k_otf_short,
        [9..11] xtal32k_inv_stre,
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

emhal::mmio_reg! {
    pub struct HbnRtcRstCtrl: u32 {
        [0..16] rtc_rst_wait_cnt_rtc,
        [16..19] rtc_rst_refdiv_rtc,
        [19..32] rtc_rst_ctrl_misc,
    }
}

emhal::mmio_reg! {
    pub struct HbnRtcRstCtrl2: u32 {
        [0..8] rtc_resv,
        [8..9] en_hw_pu_rc32k,
    }
}
