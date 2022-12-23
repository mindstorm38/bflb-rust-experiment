//! Power Down Sleep register.

emhal::mmio_struct! {
    pub struct Pds {
        [0x000] rw ctl: PdsCtl,
        [0x004] rw time1: PdsTime1,
        [0x00C] rw int: PdsInt,
        [0x010] rw ctl2: PdsCtl2,
        [0x014] rw ctl3: PdsCtl3,
        [0x018] rw ctl4: PdsCtl4,
        [0x01C] rw stat: PdsStat,
        [0x020] rw ram1: PdsRam1,
        [0x024] rw ctl5: PdsCtl5,
        [0x028] rw ram2: PdsRam2,
        [0x030] rw gpio_i_set: PdsGpioISet,
        [0x034] rw gpio_pd_set: PdsGpioPdSet,
        [0x040] rw gpio_int: PdsGpioInt,
        [0x044] rw gpio_stat: PdsGpioStat,
        [0x110] rw cpu_core_cfg0: PdsCpuCoreCfg0,
        [0x114] rw cpu_core_cfg1: PdsCpuCoreCfg1,
        [0x12C] rw cpu_core_cfg7: PdsCpuCoreCfg7,
        /// Alias for `cpu_core_cfg8`.
        [0x130] rw cpu_mtimer_rtc: super::CpuRtc,
        [0x130] rw cpu_core_cfg8: PdsCpuCoreCfg8,
        [0x134] rw cpu_core_cfg9: PdsCpuCoreCfg9,
        [0x138] rw cpu_core_cfg10: PdsCpuCoreCfg10,
        [0x140] rw cpu_core_cfg12: PdsCpuCoreCfg12,
        [0x144] rw cpu_core_cfg13: PdsCpuCoreCfg13,
        [0x148] rw cpu_core_cfg14: PdsCpuCoreCfg14,
        [0x14C] rw tzc_pds: PdsTzcPds,
        [0x300] rw rc32m_ctrl0: PdsRc32mCtrl0,
        [0x304] rw rc32m_ctrl1: PdsRc32mCtrl1,
        [0x400] rw pu_rst_clkpll: PdsPuRstClkpll,
        [0x500] rw usb_ctl: PdsUsbCtl,
        [0x504] rw usb_phy_ctrl: PdsUsbPhyCtrl,
        [0xA00] rw touch1: PdsTouch1,
        [0xA04] rw touch2: PdsTouch2,
        [0xA08] rw touch3: PdsTouch3,
        [0xA0C] rw touch_sleep_time: PdsTouchSleepTime,
        [0xA10] rw touch_data_hystersis: PdsTouchDataHystersis,
        [0xA14] rw channel_force_data_0: PdsChannelForceData0,
        [0xA18] rw channel_force_data_1: PdsChannelForceData1,
        [0xA1C] rw channel_force_data_2: PdsChannelForceData2,
        [0xA20] rw channel_force_data_3: PdsChannelForceData3,
        [0xA24] rw channel_force_data_4: PdsChannelForceData4,
        [0xA28] rw channel_force_data_5: PdsChannelForceData5,
        [0xA2C] rw channel_vth_data_0: PdsChannelVthData0,
        [0xA30] rw channel_vth_data_1: PdsChannelVthData1,
        [0xA34] rw channel_vth_data_2: PdsChannelVthData2,
        [0xA38] rw channel_raw_data_0: PdsChannelRawData0,
        [0xA3C] rw channel_raw_data_1: PdsChannelRawData1,
        [0xA40] rw channel_raw_data_2: PdsChannelRawData2,
        [0xA44] rw channel_raw_data_3: PdsChannelRawData3,
        [0xA48] rw channel_raw_data_4: PdsChannelRawData4,
        [0xA4C] rw channel_raw_data_5: PdsChannelRawData5,
        [0xA50] rw channel_raw_data_6: PdsChannelRawData6,
        [0xA54] rw channel_raw_data_7: PdsChannelRawData7,
        [0xA58] rw channel_raw_data_8: PdsChannelRawData8,
        [0xA5C] rw channel_raw_data_9: PdsChannelRawData9,
        [0xA60] rw channel_raw_data_10: PdsChannelRawData10,
        [0xA64] rw channel_raw_data_11: PdsChannelRawData11,
        [0xA68] rw channel_lta_data_0: PdsChannelLtaData0,
        [0xA6C] rw channel_lta_data_1: PdsChannelLtaData1,
        [0xA70] rw channel_lta_data_2: PdsChannelLtaData2,
        [0xA74] rw channel_lta_data_3: PdsChannelLtaData3,
        [0xA78] rw channel_lta_data_4: PdsChannelLtaData4,
        [0xA7C] rw channel_lta_data_5: PdsChannelLtaData5,
        [0xA80] rw channel_lta_data_6: PdsChannelLtaData6,
        [0xA84] rw channel_lta_data_7: PdsChannelLtaData7,
        [0xA88] rw channel_lta_data_8: PdsChannelLtaData8,
        [0xA8C] rw channel_lta_data_9: PdsChannelLtaData9,
        [0xA90] rw channel_lta_data_10: PdsChannelLtaData10,
        [0xA94] rw channel_lta_data_11: PdsChannelLtaData11,
        [0xA98] rw channel_flt_data_0: PdsChannelFltData0,
        [0xA9C] rw channel_flt_data_1: PdsChannelFltData1,
        [0xAA0] rw channel_flt_data_2: PdsChannelFltData2,
        [0xAA4] rw channel_flt_data_3: PdsChannelFltData3,
        [0xAA8] rw channel_flt_data_4: PdsChannelFltData4,
        [0xAAC] rw channel_flt_data_5: PdsChannelFltData5,
        [0xAB0] rw channel_flt_data_6: PdsChannelFltData6,
        [0xAB4] rw channel_flt_data_7: PdsChannelFltData7,
        [0xAB8] rw channel_flt_data_8: PdsChannelFltData8,
        [0xABC] rw channel_flt_data_9: PdsChannelFltData9,
        [0xAC0] rw channel_flt_data_10: PdsChannelFltData10,
        [0xAC4] rw channel_flt_data_11: PdsChannelFltData11,
        [0xAC8] rw touch_rsvd: PdsTouchRsvd,
        [0xACC] rw touch_int_setting: PdsTouchIntSetting,
        [0xAD0] rw touch_int_status: PdsTouchIntStatus,
    }
}

emhal::mmio_reg! {
    pub struct PdsCtl: u32 {
        [0..1] start_ps,
        [1..2] cr_sleep_forever,
        [2..3] cr_xtal_force_off,
        [3..4] cr_pds_wifi_save_state,
        [4..5] cr_pds_pd_dcdc11,
        [5..6] cr_pds_pd_bg_sys,
        [6..7] cr_pds_ctrl_gpio_ie_pu_pd,
        [7..8] cr_pds_pd_dcdc18,
        [8..9] cr_pds_gate_clk,
        [9..10] cr_pds_mem_stby,
        [10..11] cr_pds_glb_reg_reset_protect,
        [11..12] cr_pds_iso_en,
        [12..13] cr_pds_wait_xtal_rdy,
        [13..14] cr_pds_pwr_off,
        [14..15] cr_pds_pd_xtal,
        [15..16] cr_pds_ctrl_soc_enb,
        [16..17] cr_pds_rst_soc,
        [17..18] cr_pds_rc32m_off_dis,
        [18..19] cr_pds_dcdc11_vsel_en,
        [19..20] cr_pds_ctrl_usbpll_pd,
        [20..21] cr_pds_ctrl_aupll_pd,
        [21..22] cr_pds_ctrl_cpupll_pd,
        [22..23] cr_pds_ctrl_wifipll_pd,
        [23..28] cr_pds_dcdc11_vol,
        [28..30] cr_pds_ctrl_rf,
        [30..31] cr_pds_start_use_tbtt_sleep,
        [31..32] cr_pds_gpio_iso_mode,
    }
}

emhal::mmio_reg! {
    pub struct PdsTime1: u32 {
        [0..32] cr_sleep_duration,
    }
}

emhal::mmio_reg! {
    pub struct PdsInt: u32 {
        [0..1] ro_pds_wake_int,
        [1..2] ro_pds_rf_done_int,
        [2..3] ro_pds_wifi_tbtt_sleep_irq,
        [3..4] ro_pds_wifi_tbtt_wakeup_irq,
        [4..5] cr_pds_wake_int_mask,
        [5..6] cr_pds_rf_done_int_mask,
        [6..7] cr_pds_wifi_tbtt_sleep_irq_mask,
        [7..8] cr_pds_wifi_tbtt_wakeup_irq_mask,
        [8..9] cr_pds_int_clr,
        [10..21] cr_pds_wakeup_src_en,
        [21..32] ro_pds_wakeup_event,
    }
}

emhal::mmio_reg! {
    pub struct PdsCtl2: u32 {
        [1..2] cr_pds_force_mm_pwr_off,
        [3..4] cr_pds_force_usb_pwr_off,
        [5..6] cr_pds_force_mm_iso_en,
        [7..8] cr_pds_force_usb_iso_en,
        [8..9] cr_pds_force_np_pds_rst,
        [9..10] cr_pds_force_mm_pds_rst,
        [10..11] cr_pds_force_wb_pds_rst,
        [11..12] cr_pds_force_usb_pds_rst,
        [12..13] cr_pds_force_np_mem_stby,
        [13..14] cr_pds_force_mm_mem_stby,
        [14..15] cr_pds_force_wb_mem_stby,
        [15..16] cr_pds_force_usb_mem_stby,
        [16..17] cr_pds_force_np_gate_clk,
        [17..18] cr_pds_force_mm_gate_clk,
        [18..19] cr_pds_force_wb_gate_clk,
        [19..20] cr_pds_force_usb_gate_clk,
    }
}

emhal::mmio_reg! {
    pub struct PdsCtl3: u32 {
        [1..2] cr_pds_force_misc_pwr_off,
        [4..5] cr_pds_force_misc_iso_en,
        [7..8] cr_pds_force_misc_pds_rst,
        [10..11] cr_pds_force_misc_mem_stby,
        [13..14] cr_pds_force_misc_gate_clk,
        [26..27] cr_pds_mm_iso_en,
        [29..30] cr_pds_usb_iso_en,
        [30..31] cr_pds_misc_iso_en,
    }
}

emhal::mmio_reg! {
    pub struct PdsCtl4: u32 {
        [1..2] cr_pds_np_reset,
        [2..3] cr_pds_np_mem_stby,
        [3..4] cr_pds_np_gate_clk,
        [8..9] cr_pds_mm_pwr_off,
        [9..10] cr_pds_mm_reset,
        [10..11] cr_pds_mm_mem_stby,
        [11..12] cr_pds_mm_gate_clk,
        [13..14] cr_pds_wb_reset,
        [14..15] cr_pds_wb_mem_stby,
        [15..16] cr_pds_wb_gate_clk,
        [20..21] cr_pds_usb_pwr_off,
        [21..22] cr_pds_usb_reset,
        [22..23] cr_pds_usb_mem_stby,
        [23..24] cr_pds_usb_gate_clk,
        [24..25] cr_pds_misc_pwr_off,
        [25..26] cr_pds_misc_reset,
        [26..27] cr_pds_misc_mem_stby,
        [27..28] cr_pds_misc_gate_clk,
    }
}

emhal::mmio_reg! {
    pub struct PdsStat: u32 {
        [0..5] ro_pds_state,
        [8..13] ro_pds_rf_state,
        [24..27] reset_event,
        [31..32] clr_reset_event,
    }
}

emhal::mmio_reg! {
    pub struct PdsRam1: u32 {
        [0..4] cr_ocram_slp,
        [4..8] cr_ocram_ret,
        [8..14] cr_pds_ram_clk_cnt,
        [16..22] cr_pds_ram_clk2_cnt,
        [24..25] cr_pds_ctrl_np_ram_clk,
        [25..26] cr_pds_ctrl_mm_ram_clk,
        [26..27] cr_pds_ctrl_wb_ram_clk,
        [27..28] cr_pds_ctrl_usb_ram_clk,
        [28..29] cr_pds_ctrl_misc_ram_clk,
        [30..31] cr_pds_ctrl_ram_clk2,
        [31..32] cr_pds_ctrl_ram_clk,
    }
}

emhal::mmio_reg! {
    pub struct PdsCtl5: u32 {
        [0..1] cr_np_wfi_mask,
        [2..3] cr_mm_wfi_mask,
        [4..5] cr_pico_wfi_mask,
        [8..9] cr_pds_ctrl_usb33,
        [9..10] cr_pds_pd_ldo18io,
        [16..19] cr_pds_gpio_keep_en,
    }
}

emhal::mmio_reg! {
    pub struct PdsRam2: u32 {
        [0..10] cr_wram_slp,
        [10..20] cr_wram_ret,
    }
}

emhal::mmio_reg! {
    pub struct PdsGpioISet: u32 {
        [0..3] cr_pds_gpio_ie_set,
        [3..6] cr_pds_gpio_pd_set,
        [6..9] cr_pds_gpio_pu_set,
    }
}

emhal::mmio_reg! {
    pub struct PdsGpioPdSet: u32 {
        [0..32] cr_pds_gpio_set_int_mask,
    }
}

emhal::mmio_reg! {
    pub struct PdsGpioInt: u32 {
        [2..3] gpio_set1_int_clr,
        [4..8] gpio_set1_int_mode,
        [10..11] gpio_set2_int_clr,
        [12..16] gpio_set2_int_mode,
        [18..19] gpio_set3_int_clr,
        [20..24] gpio_set3_int_mode,
        [26..27] gpio_set4_int_clr,
        [28..32] gpio_set4_int_mode,
    }
}

emhal::mmio_reg! {
    pub struct PdsGpioStat: u32 {
        [0..32] gpio_int_stat,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg0: u32 {
        [28..29] pico_clk_en,
        [29..30] e902_dfs_req,
        [30..31] e902_dfs_ack,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg1: u32 {
        [4..6] pll_sel,
        [8..9] mcu1_clk_en,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg7: u32 {
        [0..8] pico_div,
        [28..30] e902_lpmd_b,
        [31..32] pico_rst_mask,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg8: u32 {
        [0..10] e902_rtc_div,
        [30..31] e902_rtc_rst,
        [31..32] e902_rtc_en,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg9: u32 {
        [0..32] pico_rtc_cnt_l,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg10: u32 {
        [0..32] pico_rtc_cnt_h,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg12: u32 {
        [0..12] e902_iahbl_base,
        [16..28] e902_iahbl_mask,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg13: u32 {
        [0..32] e902_rst_addr,
    }
}

emhal::mmio_reg! {
    pub struct PdsCpuCoreCfg14: u32 {
        [0..32] e906_rst_addr,
    }
}

emhal::mmio_reg! {
    pub struct PdsTzcPds: u32 {
        [0..1] cr_e902_cfg_wr_lock,
        [1..2] cr_e906_cfg_wr_lock,
    }
}

emhal::mmio_reg! {
    pub struct PdsRc32mCtrl0: u32 {
        [0..1] rc32m_cal_done,
        [1..2] rc32m_rdy,
        [2..3] rc32m_cal_inprogress,
        [3..5] rc32m_cal_div,
        [5..6] rc32m_cal_precharge,
        [6..14] rc32m_dig_code_fr_cal,
        [17..18] rc32m_allow_cal,
        [18..19] rc32m_refclk_half,
        [19..20] rc32m_ext_code_en,
        [20..21] rc32m_cal_en,
        [21..22] rc32m_pd,
        [22..30] rc32m_code_fr_ext,
    }
}

emhal::mmio_reg! {
    pub struct PdsRc32mCtrl1: u32 {
        [0..1] rc32m_test_en,
        [1..2] rc32m_soft_rst,
        [2..3] rc32m_clk_soft_rst,
        [3..4] rc32m_clk_inv,
        [4..5] rc32m_clk_force_on,
        [24..32] rc32m_reserved,
    }
}

emhal::mmio_reg! {
    pub struct PdsPuRstClkpll: u32 {
        [9..10] cr_pds_pu_clkpll_sfreg,
        [10..11] cr_pds_pu_clkpll,
    }
}

emhal::mmio_reg! {
    pub struct PdsUsbCtl: u32 {
        [0..1] usb_sw_rst_n,
        [1..2] usb_ext_susp_n,
        [2..3] usb_wakeup,
        [3..4] usb_l1_wakeup,
        [4..5] usb_drvbus_pol,
        [5..6] usb_iddig,
    }
}

emhal::mmio_reg! {
    pub struct PdsUsbPhyCtrl: u32 {
        [0..1] usb_phy_ponrst,
        [1..2] usb_phy_oscouten,
        [2..4] usb_phy_xtlsel,
        [4..5] usb_phy_outclksel,
        [5..6] usb_phy_pllaliv,
        [6..7] pu_usb20_psw,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouch1: u32 {
        [0..3] touch_vref_sel,
        [3..6] touch_vldo_sel,
        [6..7] touch_comp_hys_sel,
        [7..8] touch_current_sel,
        [16..17] touch_clk_sel,
        [17..20] touch_clk_div_ratio,
        [20..23] touch_pcharge_high,
        [23..26] touch_pcharge_low,
        [26..27] touch_cont_en,
        [27..28] touch_cycle_en,
        [28..29] touch_ulp_en,
        [30..31] pu_touch,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouch2: u32 {
        [0..4] touch_channel_sel,
        [4..5] touch_channel0_highz_en,
        [5..6] touch_channel1_highz_en,
        [6..7] touch_channel2_highz_en,
        [7..8] touch_channel3_highz_en,
        [8..9] touch_channel4_highz_en,
        [9..10] touch_channel5_highz_en,
        [10..11] touch_channel6_highz_en,
        [11..12] touch_channel7_highz_en,
        [12..13] touch_channel8_highz_en,
        [13..14] touch_channel9_highz_en,
        [14..15] touch_channel10_highz_en,
        [15..16] touch_channel11_highz_en,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouch3: u32 {
        [0..1] touch_channel_cal_en,
        [1..2] touch_force_value_en,
        [2..3] touch_data_hys_en,
        [4..5] touch_lta_en,
        [5..8] touch_lta_order,
        [8..9] touch_flt_en,
        [9..12] touch_flt_order,
        [12..13] touch_self_mutual_sel,
        [13..15] touch_vldo_ccsel,
        [18..19] ten_touch,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouchSleepTime: u32 {
        [0..23] touch_sleep_cycle,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouchDataHystersis: u32 {
        [0..9] touch_data_hys,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelForceData0: u32 {
        [0..16] touch_force_data_ch0,
        [16..32] touch_force_data_ch1,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelForceData1: u32 {
        [0..16] touch_force_data_ch2,
        [16..32] touch_force_data_ch3,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelForceData2: u32 {
        [0..16] touch_force_data_ch4,
        [16..32] touch_force_data_ch5,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelForceData3: u32 {
        [0..16] touch_force_data_ch6,
        [16..32] touch_force_data_ch7,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelForceData4: u32 {
        [0..16] touch_force_data_ch8,
        [16..32] touch_force_data_ch9,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelForceData5: u32 {
        [0..16] touch_force_data_ch10,
        [16..32] touch_force_data_ch11,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelVthData0: u32 {
        [0..8] touch_vth_data_ch0,
        [8..16] touch_vth_data_ch1,
        [16..24] touch_vth_data_ch2,
        [24..32] touch_vth_data_ch3,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelVthData1: u32 {
        [0..8] touch_vth_data_ch4,
        [8..16] touch_vth_data_ch5,
        [16..24] touch_vth_data_ch6,
        [24..32] touch_vth_data_ch7,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelVthData2: u32 {
        [0..8] touch_vth_data_ch8,
        [8..16] touch_vth_data_ch9,
        [16..24] touch_vth_data_ch10,
        [24..32] touch_vth_data_ch11,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData0: u32 {
        [0..16] touch_raw_data_ch0,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData1: u32 {
        [0..16] touch_raw_data_ch1,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData2: u32 {
        [0..16] touch_raw_data_ch2,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData3: u32 {
        [0..16] touch_raw_data_ch3,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData4: u32 {
        [0..16] touch_raw_data_ch4,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData5: u32 {
        [0..16] touch_raw_data_ch5,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData6: u32 {
        [0..16] touch_raw_data_ch6,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData7: u32 {
        [0..16] touch_raw_data_ch7,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData8: u32 {
        [0..16] touch_raw_data_ch8,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData9: u32 {
        [0..16] touch_raw_data_ch9,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData10: u32 {
        [0..16] touch_raw_data_ch10,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelRawData11: u32 {
        [0..16] touch_raw_data_ch11,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData0: u32 {
        [0..16] touch_lta_data_ch0,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData1: u32 {
        [0..16] touch_lta_data_ch1,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData2: u32 {
        [0..16] touch_lta_data_ch2,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData3: u32 {
        [0..16] touch_lta_data_ch3,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData4: u32 {
        [0..16] touch_lta_data_ch4,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData5: u32 {
        [0..16] touch_lta_data_ch5,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData6: u32 {
        [0..16] touch_lta_data_ch6,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData7: u32 {
        [0..16] touch_lta_data_ch7,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData8: u32 {
        [0..16] touch_lta_data_ch8,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData9: u32 {
        [0..16] touch_lta_data_ch9,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData10: u32 {
        [0..16] touch_lta_data_ch10,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelLtaData11: u32 {
        [0..16] touch_lta_data_ch11,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData0: u32 {
        [0..16] touch_flt_data_ch0,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData1: u32 {
        [0..16] touch_flt_data_ch1,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData2: u32 {
        [0..16] touch_flt_data_ch2,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData3: u32 {
        [0..16] touch_flt_data_ch3,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData4: u32 {
        [0..16] touch_flt_data_ch4,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData5: u32 {
        [0..16] touch_flt_data_ch5,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData6: u32 {
        [0..16] touch_flt_data_ch6,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData7: u32 {
        [0..16] touch_flt_data_ch7,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData8: u32 {
        [0..16] touch_flt_data_ch8,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData9: u32 {
        [0..16] touch_flt_data_ch9,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData10: u32 {
        [0..16] touch_flt_data_ch10,
    }
}

emhal::mmio_reg! {
    pub struct PdsChannelFltData11: u32 {
        [0..16] touch_flt_data_ch11,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouchRsvd: u32 {
        [0..8] touch_reserved,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouchIntSetting: u32 {
        [0..12] touch_int_clr,
        [16..28] touch_int_mask,
        [31..32] touch_int_en,
    }
}

emhal::mmio_reg! {
    pub struct PdsTouchIntStatus: u32 {
        [0..12] touch_int_status,
        [12..13] touch_end_flag,
    }
}
