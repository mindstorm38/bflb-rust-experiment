//! Global register, used for clock management.

embedded_util::mmio! {
    pub struct Glb {
        [0x000] rw soc_info0: GlbSocInfo0,
        [0x050] rw core_cfg16: GlbCoreCfg16,
        [0x054] rw core_cfg17: GlbCoreCfg17,
        [0x058] rw core_cfg18: GlbCoreCfg18,
        [0x05C] rw core_cfg19: GlbCoreCfg19,
        [0x060] rw core_cfg20: GlbCoreCfg20,
        [0x064] rw core_cfg21: GlbCoreCfg21,
        [0x068] rw core_cfg22: GlbCoreCfg22,
        [0x06C] rw core_cfg23: GlbCoreCfg23,
        [0x070] rw core_cfg24: GlbCoreCfg24,
        [0x074] rw core_cfg25: GlbCoreCfg25,
        [0x090] rw sys_cfg0: GlbSysCfg0,
        [0x094] rw sys_cfg1: GlbSysCfg1,
        [0x0A0] rw bus_cfg0: GlbBusCfg0,
        [0x0E0] rw emi_cfg0: GlbEmiCfg0,
        [0x0F0] rw rtc_cfg0: GlbRtcCfg0,
        [0x110] rw adc_cfg0: GlbAdcCfg0,
        [0x120] rw dac_cfg0: GlbDacCfg0,
        [0x124] rw dac_cfg1: GlbDacCfg1,
        [0x128] rw dac_cfg2: GlbDacCfg2,
        [0x12C] rw dac_cfg3: GlbDacCfg3,
        [0x130] rw dma_cfg0: GlbDmaCfg0,
        [0x134] rw dma_cfg1: GlbDmaCfg1,
        [0x138] rw dma_cfg2: GlbDmaCfg2,
        [0x140] rw ir_cfg0: GlbIrCfg0,
        [0x144] rw ir_cfg1: GlbIrCfg1,
        [0x150] rw uart_cfg0: GlbUartCfg0,
        [0x154] rw uart_cfg1: GlbUartCfg1,
        [0x158] rw uart_cfg2: GlbUartCfg2,
        [0x170] rw sf_cfg0: GlbSfCfg0,
        [0x180] rw i2c_cfg0: GlbI2cCfg0,
        [0x190] rw i2s_cfg0: GlbI2sCfg0,
        [0x1B0] rw spi_cfg0: GlbSpiCfg0,
        [0x1C0] rw qdec_cfg0: GlbQdecCfg0,
        [0x1D0] rw pwm_cfg0: GlbPwmCfg0,
        [0x1E0] rw pdm_cfg0: GlbPdmCfg0,
        [0x250] rw dig_clk_cfg0: GlbDigClkCfg0,
        [0x254] rw dig_clk_cfg1: GlbDigClkCfg1,
        [0x258] rw dig_clk_cfg2: GlbDigClkCfg2,
        [0x25C] rw dig_clk_cfg3: GlbDigClkCfg3,
        [0x260] rw rf_cfg0: GlbRfCfg0,
        [0x2E0] rw dbg_cfg0: GlbDbgCfg0,
        [0x2E4] rw dbg_cfg1: GlbDbgCfg1,
        [0x2E8] rw dbg_cfg2: GlbDbgCfg2,
        [0x2EC] rw dbg_cfg3: GlbDbgCfg3,
        [0x2F0] rw dbg_cfg4: GlbDbgCfg4,
        [0x300] rw mbist_cfg0: GlbMbistCfg0,
        [0x320] rw bmx_cfg0: GlbBmxCfg0,
        [0x324] rw bmx_cfg1: GlbBmxCfg1,
        [0x328] rw bmx_cfg2: GlbBmxCfg2,
        [0x32C] rw bmx_cfg3: GlbBmxCfg3,
        [0x330] rw bmx_cfg4: GlbBmxCfg4,
        [0x334] rw bmx_cfg5: GlbBmxCfg5,
        [0x338] rw bmx_cfg6: GlbBmxCfg6,
        [0x340] rw audio_cfg0: GlbAudioCfg0,
        [0x344] rw audio_cfg1: GlbAudioCfg1,
        [0x390] rw eth_cfg0: GlbEthCfg0,
        [0x420] rw cam_cfg0: GlbCamCfg0,
        [0x430] rw sdh_cfg0: GlbSdhCfg0,
        [0x490] rw tzc_cfg0: GlbTzcCfg0,
        [0x510] rw parm_cfg0: GlbParmCfg0,
        [0x520] rw debug_cfg0: GlbDebugCfg0,
        [0x524] rw debug_cfg1: GlbDebugCfg1,
        [0x530] rw reset_sts0: GlbResetSts0,
        [0x540] rw swrst_cfg0: GlbSwrstCfg0,
        [0x544] rw swrst_cfg1: GlbSwrstCfg1,
        [0x548] rw swrst_cfg2: GlbSwrstCfg2,
        [0x54C] rw swrst_cfg3: GlbSwrstCfg3,
        [0x580] rw cgen_cfg0: GlbCgenCfg0,
        [0x584] rw cgen_cfg1: GlbCgenCfg1,
        [0x588] rw cgen_cfg2: GlbCgenCfg2,
        [0x58C] rw cgen_cfg3: GlbCgenCfg3,
        [0x5C0] rw hw_rsv0: GlbHwRsv0,
        [0x5C4] rw hw_rsv1: GlbHwRsv1,
        [0x5C8] rw hw_rsv2: GlbHwRsv2,
        [0x5CC] rw hw_rsv3: GlbHwRsv3,
        [0x600] rw sram_cfg0: GlbSramCfg0,
        [0x604] rw sram_cfg1: GlbSramCfg1,
        [0x608] rw sram_cfg2: GlbSramCfg2,
        [0x60C] rw sram_cfg3: GlbSramCfg3,
        [0x610] rw sram_cfg4: GlbSramCfg4,
        [0x620] rw psram_cfg0: GlbPsramCfg0,
        [0x6C0] rw ldo28cis: GlbLdo28cis,
        [0x6C4] rw ldo18io: GlbLdo18io,
        [0x6C8] rw ldo15cis: GlbLdo15cis,
        [0x6CC] rw ldo18flash: GlbLdo18flash,
        [0x6D0] rw ldo12uhs: GlbLdo12uhs,
        [0x6F0] rw proc_mon: GlbProcMon,
        [0x700] rw dll_cfg0: GlbDllCfg0,
        /// Alias for `mipi_pll_cfg0`.
        [0x790] rw mipi_pll_cfg0_: super::PllCfg0,
        [0x790] rw mipi_pll_cfg0: GlbMipiPllCfg0,
        /// Alias for `mipi_pll_cfg1`.
        [0x794] rw mipi_pll_cfg1_: super::PllCfg1,
        [0x794] rw mipi_pll_cfg1: GlbMipiPllCfg1,
        [0x798] rw mipi_pll_cfg2: GlbMipiPllCfg2,
        [0x79C] rw mipi_pll_cfg3: GlbMipiPllCfg3,
        [0x7A0] rw mipi_pll_cfg4: GlbMipiPllCfg4,
        [0x7A4] rw mipi_pll_cfg5: GlbMipiPllCfg5,
        [0x7A8] rw mipi_pll_cfg6: GlbMipiPllCfg6,
        [0x7AC] rw mipi_pll_cfg7: GlbMipiPllCfg7,
        [0x7B0] rw mipi_pll_cfg8: GlbMipiPllCfg8,
        [0x7B4] rw mipi_pll_cfg9: GlbMipiPllCfg9,
        /// Alias for `uhs_pll_cfg0`.
        [0x7D0] rw uhs_pll_cfg0_: super::PllCfg0,
        [0x7D0] rw uhs_pll_cfg0: GlbUhsPllCfg0,
        /// Alias for `uhs_pll_cfg1`.
        [0x7D4] rw uhs_pll_cfg1_: super::PllCfg1,
        [0x7D4] rw uhs_pll_cfg1: GlbUhsPllCfg1,
        [0x7D8] rw uhs_pll_cfg2: GlbUhsPllCfg2,
        [0x7DC] rw uhs_pll_cfg3: GlbUhsPllCfg3,
        [0x7E0] rw uhs_pll_cfg4: GlbUhsPllCfg4,
        [0x7E4] rw uhs_pll_cfg5: GlbUhsPllCfg5,
        [0x7E8] rw uhs_pll_cfg6: GlbUhsPllCfg6,
        [0x7EC] rw uhs_pll_cfg7: GlbUhsPllCfg7,
        [0x7F0] rw uhs_pll_cfg8: GlbUhsPllCfg8,
        [0x7F4] rw uhs_pll_cfg9: GlbUhsPllCfg9,
        /// Alias for `wifi_pll_cfg0`.
        [0x810] rw wifi_pll_cfg0_: super::PllCfg0,
        [0x810] rw wifi_pll_cfg0: GlbWifiPllCfg0,
        /// Alias for `wifi_pll_cfg1`.
        [0x814] rw wifi_pll_cfg1_: super::PllCfg1,
        [0x814] rw wifi_pll_cfg1: GlbWifiPllCfg1,
        [0x818] rw wifi_pll_cfg2: GlbWifiPllCfg2,
        [0x81C] rw wifi_pll_cfg3: GlbWifiPllCfg3,
        [0x820] rw wifi_pll_cfg4: GlbWifiPllCfg4,
        [0x824] rw wifi_pll_cfg5: GlbWifiPllCfg5,
        [0x828] rw wifi_pll_cfg6: GlbWifiPllCfg6,
        [0x82C] rw wifi_pll_cfg7: GlbWifiPllCfg7,
        [0x830] rw wifi_pll_cfg8: GlbWifiPllCfg8,
        [0x834] rw wifi_pll_cfg9: GlbWifiPllCfg9,
        [0x838] rw wifi_pll_cfg10: GlbWifiPllCfg10,
        [0x83C] rw wifi_pll_cfg11: GlbWifiPllCfg11,
        [0x840] rw wifi_pll_cfg12: GlbWifiPllCfg12,
        [0x844] rw wifi_pll_cfg13: GlbWifiPllCfg13,
        [0x8A4] rw gauge: GlbGauge,
        [0x8B8] rw gauge_rx_fifo_ctrl: GlbGaugeRxFifoCtrl,
        [0x8BC] rw gauge_rx_fifo_status: GlbGaugeRxFifoStatus,
        [0x8C0] rw gauge_rx_fifo_data: GlbGaugeRxFifoData,
        [0x8C4] rw gpio_cfg0: GlbGpioCfg0,
        [0x8C8] rw gpio_cfg1: GlbGpioCfg1,
        [0x8CC] rw gpio_cfg2: GlbGpioCfg2,
        [0x8D0] rw gpio_cfg3: GlbGpioCfg3,
        [0x8D4] rw gpio_cfg4: GlbGpioCfg4,
        [0x8D8] rw gpio_cfg5: GlbGpioCfg5,
        [0x8DC] rw gpio_cfg6: GlbGpioCfg6,
        [0x8E0] rw gpio_cfg7: GlbGpioCfg7,
        [0x8E4] rw gpio_cfg8: GlbGpioCfg8,
        [0x8E8] rw gpio_cfg9: GlbGpioCfg9,
        [0x8EC] rw gpio_cfg10: GlbGpioCfg10,
        [0x8F0] rw gpio_cfg11: GlbGpioCfg11,
        [0x8F4] rw gpio_cfg12: GlbGpioCfg12,
        [0x8F8] rw gpio_cfg13: GlbGpioCfg13,
        [0x8FC] rw gpio_cfg14: GlbGpioCfg14,
        [0x900] rw gpio_cfg15: GlbGpioCfg15,
        [0x904] rw gpio_cfg16: GlbGpioCfg16,
        [0x908] rw gpio_cfg17: GlbGpioCfg17,
        [0x90C] rw gpio_cfg18: GlbGpioCfg18,
        [0x910] rw gpio_cfg19: GlbGpioCfg19,
        [0x914] rw gpio_cfg20: GlbGpioCfg20,
        [0x918] rw gpio_cfg21: GlbGpioCfg21,
        [0x91C] rw gpio_cfg22: GlbGpioCfg22,
        [0x920] rw gpio_cfg23: GlbGpioCfg23,
        [0x924] rw gpio_cfg24: GlbGpioCfg24,
        [0x928] rw gpio_cfg25: GlbGpioCfg25,
        [0x92C] rw gpio_cfg26: GlbGpioCfg26,
        [0x930] rw gpio_cfg27: GlbGpioCfg27,
        [0x934] rw gpio_cfg28: GlbGpioCfg28,
        [0x938] rw gpio_cfg29: GlbGpioCfg29,
        [0x93C] rw gpio_cfg30: GlbGpioCfg30,
        [0x940] rw gpio_cfg31: GlbGpioCfg31,
        [0x944] rw gpio_cfg32: GlbGpioCfg32,
        [0x948] rw gpio_cfg33: GlbGpioCfg33,
        [0x94C] rw gpio_cfg34: GlbGpioCfg34,
        [0x950] rw gpio_cfg35: GlbGpioCfg35,
        [0x954] rw gpio_cfg36: GlbGpioCfg36,
        [0x958] rw gpio_cfg37: GlbGpioCfg37,
        [0x95C] rw gpio_cfg38: GlbGpioCfg38,
        [0x960] rw gpio_cfg39: GlbGpioCfg39,
        [0x964] rw gpio_cfg40: GlbGpioCfg40,
        [0x968] rw gpio_cfg41: GlbGpioCfg41,
        [0x96C] rw gpio_cfg42: GlbGpioCfg42,
        [0x970] rw gpio_cfg43: GlbGpioCfg43,
        [0x974] rw gpio_cfg44: GlbGpioCfg44,
        [0x978] rw gpio_cfg45: GlbGpioCfg45,
        [0x97C] rw gpio_cfg46: GlbGpioCfg46,
        [0x980] rw gpio_cfg47: GlbGpioCfg47,
        [0x984] rw gpio_cfg48: GlbGpioCfg48,
        [0x988] rw gpio_cfg49: GlbGpioCfg49,
        [0x98C] rw gpio_cfg50: GlbGpioCfg50,
        [0x990] rw gpio_cfg51: GlbGpioCfg51,
        [0x994] rw gpio_cfg52: GlbGpioCfg52,
        [0x998] rw gpio_cfg53: GlbGpioCfg53,
        [0x99C] rw gpio_cfg54: GlbGpioCfg54,
        [0x9A0] rw gpio_cfg55: GlbGpioCfg55,
        [0x9A4] rw gpio_cfg56: GlbGpioCfg56,
        [0x9A8] rw gpio_cfg57: GlbGpioCfg57,
        [0x9AC] rw gpio_cfg58: GlbGpioCfg58,
        [0x9B0] rw gpio_cfg59: GlbGpioCfg59,
        [0x9B4] rw gpio_cfg60: GlbGpioCfg60,
        [0x9B8] rw gpio_cfg61: GlbGpioCfg61,
        [0x9BC] rw gpio_cfg62: GlbGpioCfg62,
        [0x9C0] rw gpio_cfg63: GlbGpioCfg63,
        [0xAC4] rw gpio_cfg128: GlbGpioCfg128,
        [0xAC8] rw gpio_cfg129: GlbGpioCfg129,
        [0xAE4] rw gpio_cfg136: GlbGpioCfg136,
        [0xAE8] rw gpio_cfg137: GlbGpioCfg137,
        [0xAEC] rw gpio_cfg138: GlbGpioCfg138,
        [0xAF0] rw gpio_cfg139: GlbGpioCfg139,
        [0xAF4] rw gpio_cfg140: GlbGpioCfg140,
        [0xAF8] rw gpio_cfg141: GlbGpioCfg141,
        [0xAFC] rw gpio_cfg142: GlbGpioCfg142,
        [0xB00] rw gpio_cfg143: GlbGpioCfg143,
        [0xB04] rw gpio_cfg144: GlbGpioCfg144,
    }
}

embedded_util::reg! {
    pub struct GlbSocInfo0: u32 {
        [27..28] chip_rdy,
        [28..32] id,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg16: u32 {
        [00..32] np_int_sta0,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg17: u32 {
        [00..32] np_int_sta1,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg18: u32 {
        [00..32] np_int_mask0,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg19: u32 {
        [00..32] np_int_mask1,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg20: u32 {
        [00..32] np_int_clr0,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg21: u32 {
        [00..32] np_int_clr1,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg22: u32 {
        [00..32] e902_int_en0,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg23: u32 {
        [00..32] e902_int_en1,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg24: u32 {
        [00..32] sts_e902_int_bus_0,
    }
}

embedded_util::reg! {
    pub struct GlbCoreCfg25: u32 {
        [00..32] sts_e902_int_bus_1,
    }
}

embedded_util::reg! {
    pub struct GlbSysCfg0: u32 {
        [00..01] pll_en,
        [01..02] fclk_en,
        [02..03] hclk_en,
        [03..04] bclk_en,
        [06..08] hbn_root_clk_sel,
        [08..16] hclk_div,
        [16..24] bclk_div,
    }
}

embedded_util::reg! {
    pub struct GlbSysCfg1: u32 {
        [00..01] bclk_div_act_pulse,
        [01..02] bclk_div_bypass,
        [02..03] sts_bclk_prot_done,
        [04..08] bclk_sw_done_cnt,
        [16..17] pico_clk_div_act_pulse,
        [17..18] pico_clk_div_bypass,
        [18..19] sts_pico_clk_prot_done,
        [20..24] pico_clk_sw_done_cnt,
        [24..27] fclk_sw_state,
    }
}

embedded_util::reg! {
    pub struct GlbBusCfg0: u32 {
        [00..16] rg_apb2_pck_force,
    }
}

embedded_util::reg! {
    pub struct GlbEmiCfg0: u32 {
        [09..10] emi_clk_en,
        [14..17] emi_clk_sel,
        [22..24] emi_clk_div,
    }
}

embedded_util::reg! {
    pub struct GlbRtcCfg0: u32 {
        [00..17] cpu_rtc_div,
        [18..19] cpu_rtc_en,
        [19..20] cpu_rtc_sel,
    }
}

embedded_util::reg! {
    pub struct GlbAdcCfg0: u32 {
        [00..06] gpadc_32m_clk_div,
        [07..08] gpadc_32m_clk_sel,
        [08..09] gpadc_32m_div_en,
    }
}

embedded_util::reg! {
    pub struct GlbDacCfg0: u32 {
        [00..01] gpdaca_rstn_ana,
        [01..02] gpdacb_rstn_ana,
        [07..08] gpdac_test_en,
        [08..09] gpdac_ref_sel,
        [09..12] gpdac_test_sel,
        [24..32] gpdac_reserved,
    }
}

embedded_util::reg! {
    pub struct GlbDacCfg1: u32 {
        [00..01] gpdac_a_en,
        [01..02] gpdac_ioa_en,
        [18..20] gpdac_a_rng,
        [20..23] gpdac_a_outmux,
    }
}

embedded_util::reg! {
    pub struct GlbDacCfg2: u32 {
        [00..01] gpdac_b_en,
        [01..02] gpdac_iob_en,
        [18..20] gpdac_b_rng,
        [20..23] gpdac_b_outmux,
    }
}

embedded_util::reg! {
    pub struct GlbDacCfg3: u32 {
        [00..10] gpdac_b_data,
        [16..26] gpdac_a_data,
    }
}

embedded_util::reg! {
    pub struct GlbDmaCfg0: u32 {
        [24..32] dma_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbDmaCfg1: u32 {
        [24..32] dma2_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbDmaCfg2: u32 {
        [00..32] dma_cn_sel,
    }
}

embedded_util::reg! {
    pub struct GlbIrCfg0: u32 {
        [16..22] ir_clk_div,
        [23..24] ir_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbIrCfg1: u32 {
        [00..01] led_din_reg,
        [01..02] led_din_sel,
        [02..03] led_din_polarity_sel,
        [04..08] leddrv_ibias,
        [08..12] ir_rx_gpio_sel,
        [31..32] pu_leddrv,
    }
}

embedded_util::reg! {
    pub struct GlbUartCfg0: u32 {
        [00..03] uart_clk_div,
        [04..05] uart_clk_en,
        [07..08] hbn_uart_clk_sel,
        [22..23] hbn_uart_clk_sel2,
        [24..25] uart2_io_sel,
    }
}

embedded_util::reg! {
    pub struct GlbUartCfg1: u32 {
        [00..04] uart_sig_0_sel,
        [04..08] uart_sig_1_sel,
        [08..12] uart_sig_2_sel,
        [12..16] uart_sig_3_sel,
        [16..20] uart_sig_4_sel,
        [20..24] uart_sig_5_sel,
        [24..28] uart_sig_6_sel,
        [28..32] uart_sig_7_sel,
    }
}

embedded_util::reg! {
    pub struct GlbUartCfg2: u32 {
        [00..04] uart_sig_8_sel,
        [04..08] uart_sig_9_sel,
        [08..12] uart_sig_10_sel,
        [12..16] uart_sig_11_sel,
    }
}

embedded_util::reg! {
    pub struct GlbSfCfg0: u32 {
        [08..11] sf_clk_div,
        [11..12] sf_clk_en,
        [12..14] sf_clk_sel,
        [14..16] sf_clk_sel2,
    }
}

embedded_util::reg! {
    pub struct GlbI2cCfg0: u32 {
        [16..24] i2c_clk_div,
        [24..25] i2c_clk_en,
        [25..26] i2c_clk_sel,
    }
}

embedded_util::reg! {
    pub struct GlbI2sCfg0: u32 {
        [00..06] i2s_ref_clk_div,
        [06..07] i2s_di_ref_clk_sel,
        [07..08] i2s_ref_clk_en,
        [08..09] i2s_do_ref_clk_sel,
    }
}

embedded_util::reg! {
    pub struct GlbSpiCfg0: u32 {
        [00..05] spi_clk_div,
        [08..09] spi_clk_en,
        [09..10] spi_clk_sel,
        [16..20] spi_swap_set,
    }
}

embedded_util::reg! {
    pub struct GlbQdecCfg0: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbPwmCfg0: u32 {
        [00..01] pwm1_io_sel,
        [01..02] pwm2_io_sel,
    }
}

embedded_util::reg! {
    pub struct GlbPdmCfg0: u32 {
        [00..01] pdm_io_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDigClkCfg0: u32 {
        [00..11] dig_32k_div,
        [12..13] dig_32k_en,
        [13..14] dig_32k_comp,
        [16..23] dig_512k_div,
        [24..25] dig_512k_en,
        [25..26] dig_512k_comp,
        [28..30] dig_clk_src_sel,
        [31..32] en_platform_wakeup,
    }
}

embedded_util::reg! {
    pub struct GlbDigClkCfg1: u32 {
        [00..01] mm_muxpll_160m_sel,
        [01..02] mm_muxpll_240m_sel,
        [02..03] mm_muxpll_320m_sel,
        [08..10] top_muxpll_80m_sel,
        [10..12] top_muxpll_160m_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDigClkCfg2: u32 {
        [00..02] chip_clk_out_0_sel,
        [02..04] chip_clk_out_1_sel,
        [04..06] chip_clk_out_2_sel,
        [06..08] chip_clk_out_3_sel,
        [08..09] chip_clk_out_0_en,
        [09..10] chip_clk_out_1_en,
        [10..11] chip_clk_out_2_en,
        [11..12] chip_clk_out_3_en,
        [12..14] gpio_tmr_clk_sel,
        [14..16] gpio_mm_tmr_clk_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDigClkCfg3: u32 {
        [00..01] dsi_txclkesc_sel,
        [01..02] csi_txclkesc_sel,
    }
}

embedded_util::reg! {
    pub struct GlbRfCfg0: u32 {
        [09..10] cfg_inv_rf2_test_clk_o,
    }
}

embedded_util::reg! {
    pub struct GlbDbgCfg0: u32 {
        [00..30] dbg_ll_ctrl,
        [30..32] dbg_ll_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDbgCfg1: u32 {
        [00..30] dbg_lh_ctrl,
        [30..32] dbg_lh_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDbgCfg2: u32 {
        [00..30] dbg_hl_ctrl,
        [30..32] dbg_hl_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDbgCfg3: u32 {
        [00..30] dbg_hh_ctrl,
        [30..32] dbg_hh_sel,
    }
}

embedded_util::reg! {
    pub struct GlbDbgCfg4: u32 {
        [00..01] debug_oe,
        [01..32] debug_i,
    }
}

embedded_util::reg! {
    pub struct GlbMbistCfg0: u32 {
        [00..01] mbist_mode,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg0: u32 {
        [00..05] bmx_timeout_en,
        [05..06] bmx_arb_mode,
        [06..07] bmx_timeout_clr,
        [07..09] h_wthre_hw2ext,
        [09..10] bmx_busy_option_dis,
        [10..11] bmx_gating_dis,
        [11..16] sts_bmx_timeout_sts,
        [16..24] pds_apb_cfg,
        [24..32] hbn_apb_cfg,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg1: u32 {
        [00..01] bmx_berr_int_en,
        [01..02] mcu_berr_int_en,
        [16..17] bmx_qos_cpu,
        [17..18] bmx_qos_sdu,
        [18..19] bmx_qos_sec0,
        [19..20] bmx_qos_sec1,
        [20..21] bmx_qos_sec2,
        [21..22] bmx_qos_dma,
        [22..23] bmx_qos_cci,
        [23..24] bmx_qos_pldma,
        [24..25] bmx_qos_blem,
        [25..26] bmx_qos_emaca,
        [26..27] bmx_qos_dma2,
        [27..28] bmx_qos_sdhm,
        [28..32] bmx_dbg_sel,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg2: u32 {
        [00..14] bmx_berr_en,
        [16..17] mcu_berr_en,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg3: u32 {
        [00..01] bmx_berr_clr,
        [01..02] bmx_berr_last,
        [08..09] mcu_berr_clr,
        [09..10] mcu_berr_last,
        [16..17] sts_bmx_berr,
        [17..18] sts_mcu_berr,
        [24..25] sts_bmx_berr_write,
        [25..26] sts_mcu_berr_write,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg4: u32 {
        [00..14] sts_bmx_berr_src,
        [16..17] sts_mcu_berr_src,
        [24..32] sts_mcu_berr_id,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg5: u32 {
        [00..32] sts_bmx_berr_addr,
    }
}

embedded_util::reg! {
    pub struct GlbBmxCfg6: u32 {
        [00..32] sts_mcu_berr_addr,
    }
}

embedded_util::reg! {
    pub struct GlbAudioCfg0: u32 {
        [00..06] audio_pdm_clk_div,
        [07..08] audio_pdm_clk_en,
        [08..14] audio_adc_clk_div,
        [15..16] audio_adc_clk_en,
        [16..22] audio_dac_clk_div,
        [23..24] audio_dac_clk_en,
        [31..32] audio_auto_div_en,
    }
}

embedded_util::reg! {
    pub struct GlbAudioCfg1: u32 {
        [00..10] padc_clk_div,
        [10..11] padc_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbEthCfg0: u32 {
        [05..06] cfg_sel_eth_ref_clk_o,
        [06..07] cfg_inv_eth_ref_clk_o,
        [07..08] cfg_inv_eth_tx_clk,
        [10..11] cfg_inv_eth_rx_clk,
    }
}

embedded_util::reg! {
    pub struct GlbCamCfg0: u32 {
        [27..28] cam_ref_clk_en,
        [28..30] cam_ref_clk_src_sel,
        [30..32] cam_ref_clk_div,
    }
}

embedded_util::reg! {
    pub struct GlbSdhCfg0: u32 {
        [09..12] sdh_clk_div,
        [12..13] sdh_clk_sel,
        [13..14] sdh_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbTzcCfg0: u32 {
        [12..13] tzc_glb_pwron_rst_lock,
        [13..14] tzc_glb_cpu_reset_lock,
        [14..15] tzc_glb_sys_reset_lock,
        [15..16] tzc_glb_cpu2_reset_lock,
        [21..22] tzc_glb_pwr_lock,
        [22..23] tzc_glb_int_lock,
        [24..25] tzc_glb_cpupll_lock,
        [25..26] tzc_glb_misc_lock,
        [26..27] tzc_glb_sram_lock,
        [27..28] tzc_glb_swrst_lock,
        [28..29] tzc_glb_bmx_lock,
        [29..30] tzc_glb_dbg_lock,
        [30..31] tzc_glb_mbist_lock,
        [31..32] tzc_glb_clk_lock,
    }
}

embedded_util::reg! {
    pub struct GlbParmCfg0: u32 {
        [00..01] bd_en,
        [02..06] uart_swap_set,
        [08..09] swap_sflash_io_3_io_0,
        [09..10] sel_embedded_sflash,
        [11..12] sel_psram0_x16,
        [12..13] spi_0_master_mode,
        [13..14] spi_0_swap,
        [14..15] sel_dbi_type_c,
        [15..16] ant_switch_sel,
        [17..18] p1_adc_test_with_cci,
        [18..19] p2_dac_test_with_cci,
        [19..20] p3_cci_use_io_2_5,
        [20..21] p4_adc_test_with_jtag,
        [21..22] p5_dac_test_with_jtag,
        [22..23] p6_sdh_use_io_0_5,
        [23..24] p7_jtag_use_io_2_5,
        [25..27] rf1_test_mode,
        [27..28] mm_spi_master_mode,
        [28..29] mm_spi_swap,
        [29..30] audio_test_mode,
        [30..32] sel_rf_audio_test,
    }
}

embedded_util::reg! {
    pub struct GlbDebugCfg0: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbDebugCfg1: u32 {
        [20..21] debug_ndreset_gate,
    }
}

embedded_util::reg! {
    pub struct GlbResetSts0: u32 {
        [00..07] top_reset_recorder,
        [07..08] clr_top_reset_recorder,
    }
}

embedded_util::reg! {
    pub struct GlbSwrstCfg0: u32 {
        [00..01] swrst_s00,
        [01..02] swrst_s01,
        [04..05] swrst_s20,
        [08..09] swrst_s30,
        [09..10] swrst_s31,
        [10..11] swrst_s32,
        [11..12] swrst_s33,
        [16..17] swrst_s1_ext_emi_misc,
        [17..18] swrst_s1_ext_psram0_ctrl,
        [18..19] swrst_s1_ext_psram1_ctrl,
        [19..20] swrst_s1_ext_usb,
        [20..21] swrst_s1_ext_mix2,
        [21..22] swrst_s1_ext_audio,
        [22..23] swrst_s1_ext_sdh,
        [23..24] swrst_s1_ext_emac,
        [24..25] swrst_s1_ext_dma2,
    }
}

embedded_util::reg! {
    pub struct GlbSwrstCfg1: u32 {
        [00..01] swrst_s10,
        [01..02] swrst_s11,
        [02..03] swrst_s12,
        [03..04] swrst_s13,
        [04..05] swrst_s14,
        [05..06] swrst_s15,
        [06..07] swrst_s16,
        [07..08] swrst_s17,
        [08..09] swrst_s18,
        [09..10] swrst_s19,
        [10..11] swrst_s1a,
        [11..12] swrst_s1b,
        [12..13] swrst_s1c,
        [13..14] swrst_s1d,
        [14..15] swrst_s1e,
        [15..16] swrst_s1f,
        [16..17] swrst_s1a0,
        [17..18] swrst_s1a1,
        [18..19] swrst_s1a2,
        [19..20] swrst_s1a3,
        [20..21] swrst_s1a4,
        [21..22] swrst_s1a5,
        [22..23] swrst_s1a6,
        [23..24] swrst_s1a7,
        [24..25] swrst_s1a8,
        [25..26] swrst_s1a9,
        [26..27] swrst_s1aa,
        [27..28] swrst_s1ab,
        [28..29] swrst_s1ac,
        [29..30] swrst_s1ad,
        [30..31] swrst_s1ae,
        [31..32] swrst_s1af,
    }
}

embedded_util::reg! {
    pub struct GlbSwrstCfg2: u32 {
        [00..01] ctrl_pwron_rst,
        [01..02] ctrl_cpu_reset,
        [02..03] ctrl_sys_reset,
        [03..04] ctrl_pico_reset,
        [04..05] ctrl_cpu2_reset,
        [05..06] ctrl_chip_reset,
        [06..07] wl_wdt_reset_mm_en,
        [07..08] mmwdt2wl_rst_msk,
        [24..25] pka_clk_sel,
        [28..32] ctrl_reset_dummy,
    }
}

embedded_util::reg! {
    pub struct GlbSwrstCfg3: u32 {
        [02..03] disrst_s12,
        [04..05] disrst_s14,
        [08..09] disrst_s18,
        [11..12] disrst_s1b,
        [16..17] disrst_s1a0,
        [17..18] disrst_s1a1,
        [18..19] disrst_s1a2,
        [19..20] disrst_s1a3,
        [20..21] disrst_s1a4,
        [21..22] disrst_s1a5,
        [22..23] disrst_s1a6,
        [23..24] disrst_s1a7,
        [24..25] disrst_s1a8,
        [25..26] disrst_s1a9,
        [26..27] disrst_s1aa,
    }
}

embedded_util::reg! {
    pub struct GlbCgenCfg0: u32 {
        [00..01] cgen_m_cpu,
        [01..02] cgen_m_sdu,
        [02..03] cgen_m_sec,
        [03..04] cgen_m_dma,
        [04..05] cgen_m_cci,
    }
}

embedded_util::reg! {
    pub struct GlbCgenCfg1: u32 {
        [00..01] cgen_s1_rsvd0,
        [02..03] cgen_s1_gpip,
        [03..04] cgen_s1_sec_dbg,
        [04..05] cgen_s1_sec_eng,
        [05..06] cgen_s1_tz,
        [06..07] cgen_s1_rsvd6,
        [07..08] cgen_s1_ef_ctrl,
        [08..09] cgen_s1_rsvd8,
        [09..10] cgen_s1_rsvd9,
        [10..11] cgen_s1_rsvd10,
        [11..12] cgen_s1_sf_ctrl,
        [12..13] cgen_s1_dma,
        [13..14] cgen_s1_rsvd13,
        [14..15] cgen_s1_rsvd14,
        [15..16] cgen_s1_rsvd15,
        [16..17] cgen_s1a_uart0,
        [17..18] cgen_s1a_uart1,
        [18..19] cgen_s1a_spi,
        [19..20] cgen_s1a_i2c,
        [20..21] cgen_s1a_pwm,
        [21..22] cgen_s1a_timer,
        [22..23] cgen_s1a_ir,
        [23..24] cgen_s1a_cks,
        [24..25] cgen_s1a_rsvd8,
        [25..26] cgen_s1a_i2c1,
        [26..27] cgen_s1a_uart2,
        [27..28] cgen_s1a_rsvd11,
        [28..29] cgen_s1a_rsvd12,
        [29..30] cgen_s1a_rsvd13,
        [30..31] cgen_s1a_rsvd14,
        [31..32] cgen_s1a_rsvd15,
    }
}

embedded_util::reg! {
    pub struct GlbCgenCfg2: u32 {
        [00..01] cgen_s0,
        [04..05] cgen_s2_wifi,
        [10..11] cgen_s3_bt_ble2,
        [11..12] cgen_s3_m1542,
        [16..17] cgen_s1_ext_emi_misc,
        [17..18] cgen_s1_ext_psram0_ctrl,
        [18..19] cgen_s1_ext_psram_ctrl,
        [19..20] cgen_s1_ext_usb,
        [20..21] cgen_s1_ext_mix2,
        [21..22] cgen_s1_ext_audio,
        [22..23] cgen_s1_ext_sdh,
        [23..24] cgen_s1_ext_emac,
        [24..25] cgen_s1_ext_dma2,
        [25..26] cgen_s1_ext_rsvd9,
        [26..27] cgen_s1_ext_rsvd10,
        [27..28] cgen_s1_ext_rsvd11,
    }
}

embedded_util::reg! {
    pub struct GlbCgenCfg3: u32 {
        [00..01] cgen_mm_wifipll_160m,
        [01..02] cgen_mm_wifipll_240m,
        [02..03] cgen_mm_wifipll_320m,
        [03..04] cgen_mm_aupll_div1,
        [04..05] cgen_mm_aupll_div2,
        [05..06] cgen_emi_cpupll_400m,
        [06..07] cgen_emi_cpupll_200m,
        [07..08] cgen_emi_wifipll_320m,
        [08..09] cgen_emi_aupll_div1,
        [09..10] cgen_top_cpupll_80m,
        [10..11] cgen_top_cpupll_100m,
        [11..12] cgen_top_cpupll_160m,
        [12..13] cgen_top_cpupll_400m,
        [13..14] cgen_top_wifipll_240m,
        [14..15] cgen_top_wifipll_320m,
        [15..16] cgen_top_aupll_div2,
        [16..17] cgen_top_aupll_div1,
    }
}

embedded_util::reg! {
    pub struct GlbHwRsv0: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbHwRsv1: u32 {
        /// Numeric identifier of the flash.
        [00..31] flash_id,
        /// Indicate if the stored flash identifier is valid.
        [31..32] flash_id_valid,
    }
}

embedded_util::reg! {
    pub struct GlbHwRsv2: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbHwRsv3: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbSramCfg0: u32 {
        [00..02] cr_mcu_cache_ret,
        [02..06] cr_mcu_hsram_ret,
        [08..09] cr_wb_ram_ret,
        [09..11] cr_misc_ram_ret,
    }
}

embedded_util::reg! {
    pub struct GlbSramCfg1: u32 {
        [00..02] cr_mcu_cache_slp,
        [02..06] cr_mcu_hsram_slp,
        [06..08] cr_mcu_rom_slp,
        [08..09] cr_wb_ram_slp,
        [09..11] cr_misc_ram_slp,
    }
}

embedded_util::reg! {
    pub struct GlbSramCfg2: u32 {
        [00..01] cr_mcu_cache_dvse,
        [01..02] cr_mcu_hsram_dvse,
        [02..03] cr_mcu_rom_dvse,
        [03..04] cr_wb_ram_dvse,
        [04..05] cr_misc_ram_dvse,
        [05..06] cr_ocram_dvse,
        [06..07] cr_wram_dvse,
        [08..09] cr_mcu_cache_nap,
        [09..10] cr_mcu_hsram_nap,
        [11..12] cr_wb_ram_nap,
        [12..13] cr_misc_ram_nap,
        [13..14] cr_ocram_nap,
        [14..15] cr_wram_nap,
    }
}

embedded_util::reg! {
    pub struct GlbSramCfg3: u32 {
        [00..08] em_sel,
        [28..30] vram_sel,
    }
}

embedded_util::reg! {
    pub struct GlbSramCfg4: u32 {
        [00..04] cr_mcu_cache_dvs,
        [04..08] cr_mcu_hsram_dvs,
        [08..12] cr_mcu_rom_dvs,
        [12..16] cr_wb_ram_dvs,
        [16..20] cr_misc_ram_dvs,
        [20..24] cr_ocram_dvs,
        [24..28] cr_wram_dvs,
    }
}

embedded_util::reg! {
    pub struct GlbPsramCfg0: u32 {
        [27..28] psramb_clk_en,
        [28..30] psramb_clk_sel,
        [30..32] psramb_clk_div,
    }
}

embedded_util::reg! {
    pub struct GlbLdo28cis: u32 {
        [00..01] pu_ldo28cis,
        [01..02] ldo28cis_bypass,
        [02..03] ldo28cis_pulldown,
        [03..04] ldo28cis_pulldown_sel,
        [04..07] ldo28cis_bm,
        [08..11] ldo28cis_cc,
        [11..12] ldo28cis_ocp_out,
        [12..15] ldo28cis_ocp_th,
        [15..16] ldo28cis_ocp_en,
        [16..19] ldo28cis_sstart_delay,
        [19..20] ldo28cis_sstart_en,
        [20..24] ldo28cis_vout_sel,
        [24..28] ldo28cis_vout_trim,
    }
}

embedded_util::reg! {
    pub struct GlbLdo18io: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbLdo15cis: u32 {
        [00..01] pu_ldo15cis,
        [01..02] ldo15cis_bypass,
        [02..03] ldo15cis_pulldown,
        [03..04] ldo15cis_pulldown_sel,
        [04..07] ldo15cis_bm,
        [08..11] ldo15cis_cc,
        [11..12] ldo15cis_ocp_out,
        [12..15] ldo15cis_ocp_th,
        [15..16] ldo15cis_ocp_en,
        [16..19] ldo15cis_sstart_delay,
        [19..20] ldo15cis_sstart_en,
        [20..24] ldo15cis_vout_sel,
        [24..28] ldo15cis_vout_trim,
    }
}

embedded_util::reg! {
    pub struct GlbLdo18flash: u32 {
        [00..01] pu_ldo18flash,
        [01..02] ldo18flash_bypass,
        [02..03] ldo18flash_pulldown,
        [03..04] ldo18flash_pulldown_sel,
        [04..07] ldo18flash_bm,
        [08..11] ldo18flash_cc,
        [11..12] ldo18flash_ocp_out,
        [12..15] ldo18flash_ocp_th,
        [15..16] ldo18flash_ocp_en,
        [16..19] ldo18flash_sstart_delay,
        [19..20] ldo18flash_sstart_en,
        [20..24] ldo18flash_vout_sel,
        [24..28] ldo18flash_vout_trim,
    }
}

embedded_util::reg! {
    pub struct GlbLdo12uhs: u32 {
        [00..01] pu_ldo12uhs,
        [01..02] ldo12uhs_bypass,
        [02..03] ldo12uhs_pulldown,
        [03..04] ldo12uhs_pulldown_sel,
        [04..07] ldo12uhs_bm,
        [08..11] ldo12uhs_cc,
        [11..12] ldo12uhs_ocp_out,
        [12..15] ldo12uhs_ocp_th,
        [15..16] ldo12uhs_ocp_en,
        [16..19] ldo12uhs_sstart_delay,
        [19..20] ldo12uhs_sstart_en,
        [20..24] ldo12uhs_vout_sel,
        [24..28] ldo12uhs_vout_trim,
    }
}

embedded_util::reg! {
    pub struct GlbProcMon: u32 {
        [00..01] pu_proc_mon,
        [01..02] osc_en_rvt,
        [02..03] osc_en_lvt,
        [03..04] osc_sel,
        [04..05] rstn_ringcount,
        [05..06] rstn_refcount,
        [08..12] refcount_div_onehot,
        [12..28] ring_freq,
        [28..29] ring_freq_rdy,
    }
}

embedded_util::reg! {
    pub struct GlbDllCfg0: u32 {
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg0: u32 {
        [00..01] mipipll_sdm_rstb,
        [02..03] mipipll_fbdv_rstb,
        [05..06] pu_mipipll_fbdv,
        [08..09] pu_mipipll_cp,
        [09..10] pu_mipipll_sfreg,
        [10..11] pu_mipipll,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg1: u32 {
        [00..07] mipipll_even_div_ratio,
        [07..08] mipipll_even_div_en,
        [08..12] mipipll_refdiv_ratio,
        [16..18] mipipll_refclk_sel,
        [20..22] mipipll_vg11_sel,
        [24..26] mipipll_vg13_sel,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg2: u32 {
        [00..01] mipipll_sel_cp_bias,
        [04..06] mipipll_icp_5u,
        [06..08] mipipll_icp_1u,
        [08..09] mipipll_int_frac_sw,
        [09..10] mipipll_cp_startup_en,
        [10..11] mipipll_cp_opamp_en,
        [11..12] mipipll_cp_ota_en,
        [12..13] mipipll_pfd_en,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg3: u32 {
        [00..01] mipipll_c4_en,
        [04..06] mipipll_r4,
        [08..09] mipipll_r4_short,
        [12..14] mipipll_c3,
        [14..16] mipipll_cz,
        [16..19] mipipll_rz,
        [19..20] mipipll_lf_test_en,
        [20..21] mipipll_fast_lock_en,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg4: u32 {
        [00..02] mipipll_sel_sample_clk,
        [04..06] mipipll_sel_fb_clk,
        [08..09] mipipll_lock_det_en,
        [09..11] mipipll_lock_clk_sel,
        [12..13] mipipll_lock_clk_inv_en,
        [15..17] mipipll_lock_win_sel,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg5: u32 {
        [00..03] mipipll_vco_speed,
        [03..05] mipipll_vco_vdd_ctrl,
        [05..06] mipipll_vco_vdd_ctrl_extra,
        [07..10] mipipll_vco_postdiv_sel,
        [10..11] mipipll_vco_postdiv_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg6: u32 {
        [00..19] mipipll_sdmin,
        [24..25] mipipll_sdm_bypass,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg7: u32 {
        [00..01] mipipll_sdm_order_sel,
        [01..03] mipipll_sdm_dith_sel,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg8: u32 {
        [00..01] mipipll_dc_tp_out_en,
        [01..02] mipipll_ten,
        [02..03] mipipll_ten_sfreg,
        [04..05] mipipll_dten_ckin,
        [05..06] mipipll_dten_fref,
        [06..07] mipipll_dten_fsdm,
        [07..08] mipipll_dten_pupll,
        [08..09] mipipll_dten_pll_locked,
        [10..11] mipipll_dtest_pull_down,
    }
}

embedded_util::reg! {
    pub struct GlbMipiPllCfg9: u32 {
        [00..01] mipipll_ssc_en,
        [04..12] mipipll_ssc_cnt,
        [12..15] mipipll_ssc_gain,
        [16..17] mipipll_ssc_start_gate_en,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg0: u32 {
        [00..01] uhspll_sdm_rstb,
        [02..03] uhspll_fbdv_rstb,
        [05..06] pu_uhspll_fbdv,
        [08..09] pu_uhspll_cp,
        [09..10] pu_uhspll_sfreg,
        [10..11] pu_uhspll,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg1: u32 {
        [00..07] uhspll_even_div_ratio,
        [07..08] uhspll_even_div_en,
        [08..12] uhspll_refdiv_ratio,
        [16..18] uhspll_refclk_sel,
        [20..22] uhspll_vg11_sel,
        [24..26] uhspll_vg13_sel,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg2: u32 {
        [00..01] uhspll_sel_cp_bias,
        [04..06] uhspll_icp_5u,
        [06..08] uhspll_icp_1u,
        [08..09] uhspll_int_frac_sw,
        [09..10] uhspll_cp_startup_en,
        [10..11] uhspll_cp_opamp_en,
        [11..12] uhspll_cp_ota_en,
        [12..13] uhspll_pfd_en,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg3: u32 {
        [00..01] uhspll_c4_en,
        [04..06] uhspll_r4,
        [08..09] uhspll_r4_short,
        [12..14] uhspll_c3,
        [14..16] uhspll_cz,
        [16..19] uhspll_rz,
        [19..20] uhspll_lf_test_en,
        [20..21] uhspll_fast_lock_en,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg4: u32 {
        [00..02] uhspll_sel_sample_clk,
        [04..06] uhspll_sel_fb_clk,
        [08..09] uhspll_lock_det_en,
        [09..11] uhspll_lock_clk_sel,
        [12..13] uhspll_lock_clk_inv_en,
        [15..17] uhspll_lock_win_sel,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg5: u32 {
        [00..03] uhspll_vco_speed,
        [03..05] uhspll_vco_vdd_ctrl,
        [05..06] uhspll_vco_vdd_ctrl_extra,
        [07..10] uhspll_vco_postdiv_sel,
        [10..11] uhspll_vco_postdiv_clk_en,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg6: u32 {
        [00..19] uhspll_sdmin,
        [24..25] uhspll_sdm_bypass,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg7: u32 {
        [00..01] uhspll_sdm_order_sel,
        [01..03] uhspll_sdm_dith_sel,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg8: u32 {
        [00..01] uhspll_dc_tp_out_en,
        [01..02] uhspll_ten,
        [02..03] uhspll_ten_sfreg,
        [04..05] uhspll_dten_ckin,
        [05..06] uhspll_dten_fref,
        [06..07] uhspll_dten_fsdm,
        [07..08] uhspll_dten_pupll,
        [08..09] uhspll_dten_pll_locked,
        [10..11] uhspll_dtest_pull_down,
    }
}

embedded_util::reg! {
    pub struct GlbUhsPllCfg9: u32 {
        [00..01] uhspll_ssc_en,
        [04..12] uhspll_ssc_cnt,
        [12..15] uhspll_ssc_gain,
        [16..17] uhspll_ssc_start_gate_en,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg0: u32 {
        [00..01] wifipll_sdm_rstb,
        [01..02] wifipll_postdiv_rstb,
        [02..03] wifipll_fbdv_rstb,
        [03..04] wifipll_refdiv_rstb,
        [04..05] pu_wifipll_postdiv,
        [05..06] pu_wifipll_fbdv,
        [06..07] pu_wifipll_clamp_op,
        [07..08] pu_wifipll_pfd,
        [08..09] pu_wifipll_cp,
        [09..10] pu_wifipll_sfreg,
        [10..11] pu_wifipll,
        [11..12] pu_wifipll_clktree,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg1: u32 {
        [00..07] wifipll_postdiv,
        [08..12] wifipll_refdiv_ratio,
        [16..18] wifipll_refclk_sel,
        [20..22] wifipll_vg11_sel,
        [24..26] wifipll_vg13_sel,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg2: u32 {
        [00..01] wifipll_sel_cp_bias,
        [04..06] wifipll_icp_5u,
        [06..08] wifipll_icp_1u,
        [08..09] wifipll_int_frac_sw,
        [09..10] wifipll_cp_startup_en,
        [10..11] wifipll_cp_opamp_en,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg3: u32 {
        [00..01] wifipll_c4_en,
        [04..06] wifipll_r4,
        [08..09] wifipll_r4_short,
        [12..14] wifipll_c3,
        [14..16] wifipll_cz,
        [16..19] wifipll_rz,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg4: u32 {
        [00..02] wifipll_sel_sample_clk,
        [04..06] wifipll_sel_fb_clk,
        [08..09] wifipll_sdmclk_sel,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg5: u32 {
        [00..03] wifipll_vco_speed,
        [04..05] wifipll_vco_div1_en,
        [08..09] wifipll_vco_div2_en,
        [12..13] wifipll_vco_div3_en,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg6: u32 {
        [00..26] wifipll_sdmin,
        [28..29] wifipll_sdm_bypass,
        [29..30] wifipll_sdm_bypass_hw,
        [31..32] wifipll_sdm_ctrl_hw,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg7: u32 {
        [00..02] wifipll_sdm_order_sel,
        [04..06] wifipll_sdm_noi_prbs_sel,
        [08..09] wifipll_sdm_noi_prbs_en,
        [12..14] wifipll_sdm_sig_prbs_sel,
        [16..18] wifipll_sdm_sig_dith_sel,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg8: u32 {
        [00..01] wifipll_en_div2,
        [01..02] wifipll_en_div4,
        [02..03] wifipll_en_div5,
        [03..04] wifipll_en_div6,
        [04..05] wifipll_en_div8,
        [05..06] wifipll_en_div10,
        [06..07] wifipll_en_div12,
        [07..08] wifipll_en_div20,
        [08..09] wifipll_en_div30,
        [09..10] wifipll_sel_div2_div4,
        [10..11] en_wifipll_div30_bz_adc,
        [12..13] wifipll_en_div2_hw,
        [31..32] wifipll_en_ctrl_hw,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg9: u32 {
        [00..01] wifipll_dc_tp_out_en,
        [01..02] ten_wifipll,
        [02..03] ten_wifipll_sfreg,
        [04..05] dten_wifipll_fin,
        [05..06] dten_wifipll_fref,
        [06..07] dten_wifipll_fsdm,
        [07..08] dten_wifipll_div30,
        [08..09] dten_wifipll_div10,
        [09..10] dten_wifipll_postdiv_clk,
        [10..11] usbpll_dtest_pclk_en,
        [11..12] usbpll_dtest_clkout_en,
        [12..13] dtest_wifipll_pulldown,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg10: u32 {
        [02..03] usbpll_ssc_start,
        [03..04] usbpll_ssc_start_gate_en,
        [04..07] usbpll_ssc_gain,
        [08..09] usbpll_ssc_en,
        [09..10] usbpll_sdm_bypass,
        [10..11] usbpll_sdm_order_sel,
        [16..18] usbpll_sdm_sig_dith_sel,
        [20..21] usbpll_div2_en,
        [21..22] usbpll_clkout_en,
        [24..26] usbpll_sel_sample_clk,
        [28..29] usbpll_rstb,
        [29..30] pu_usbpll_mmdiv,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg11: u32 {
        [00..19] usbpll_sdmin,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg12: u32 {
        [00..09] usbpll_ssc_cnt,
    }
}

embedded_util::reg! {
    pub struct GlbWifiPllCfg13: u32 {
        [00..16] wifipll_resv,
        [21..22] usbpll_dl_ctrl,
        [22..23] wifipll_dl_ctrl_30_bz_adc,
        [23..24] wifipll_dl_ctrl_30,
        [24..25] wifipll_dl_ctrl_20,
        [25..26] wifipll_dl_ctrl_12,
        [26..27] wifipll_dl_ctrl_10,
        [27..28] wifipll_dl_ctrl_8,
        [28..29] wifipll_dl_ctrl_6,
        [29..30] wifipll_dl_ctrl_5,
        [30..31] wifipll_dl_ctrl_4,
        [31..32] wifipll_dl_ctrl_2,
    }
}

embedded_util::reg! {
    pub struct GlbGauge: u32 {
        [00..03] gauge_reserve,
        [03..05] gauge_ictrl_adc,
        [05..06] gauge_dem_en,
        [06..07] gauge_ckb_en,
        [07..08] gauge_chop_phas,
        [08..11] gauge_chop_freq,
        [11..12] gauge_chop_en,
        [12..13] gauge_sel_edge,
        [13..15] gauge_quan_gain,
        [15..16] gauge_sdm_pu,
        [16..17] gauge_channel_sel,
        [17..18] gauge_channel_en,
        [18..19] gauge_lp_mode,
        [20..23] tmux_gauge_power,
        [23..24] ten_gauge_power,
        [24..28] ntc_bias_sel,
        [28..29] ntc_bias_en,
        [29..30] gauge_ldo_pu,
        [30..31] gauge_vcm_pu,
        [31..32] gauge_bg_pu,
    }
}

embedded_util::reg! {
    pub struct GlbGaugeRxFifoCtrl: u32 {
        [00..01] gauge_rx_fifo_flush,
        [01..02] gauge_rxo_int_en,
        [02..03] gauge_rxu_int_en,
        [03..04] gauge_rxa_int_en,
        [04..05] gauge_rx_drq_en,
        [05..06] gauge_rx_data_res,
        [08..09] gauge_rx_ch_en,
        [14..16] gauge_rx_drq_cnt,
        [16..19] gauge_rx_trg_level,
        [24..26] gauge_rx_data_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGaugeRxFifoStatus: u32 {
        [01..02] gauge_rxo_int,
        [02..03] gauge_rxu_int,
        [04..05] gauge_rxa_int,
        [16..19] gauge_rxa_cnt,
        [24..25] gauge_rxa,
    }
}

embedded_util::reg! {
    pub struct GlbGaugeRxFifoData: u32 {
        [00..32] gauge_rx_data,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg0: u32 {
        [00..01] gpio_0_ie,
        [01..02] gpio_0_smt,
        [02..04] gpio_0_drv,
        [04..05] gpio_0_pu,
        [05..06] gpio_0_pd,
        [06..07] gpio_0_oe,
        [08..13] gpio_0_func_sel,
        [16..20] gpio_0_int_mode_set,
        [20..21] gpio_0_int_clr,
        [21..22] gpio_0_int_stat,
        [22..23] gpio_0_int_mask,
        [24..25] gpio_0_o,
        [25..26] gpio_0_set,
        [26..27] gpio_0_clr,
        [28..29] gpio_0_i,
        [30..32] gpio_0_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg1: u32 {
        [00..01] gpio_1_ie,
        [01..02] gpio_1_smt,
        [02..04] gpio_1_drv,
        [04..05] gpio_1_pu,
        [05..06] gpio_1_pd,
        [06..07] gpio_1_oe,
        [08..13] gpio_1_func_sel,
        [16..20] gpio_1_int_mode_set,
        [20..21] gpio_1_int_clr,
        [21..22] gpio_1_int_stat,
        [22..23] gpio_1_int_mask,
        [24..25] gpio_1_o,
        [25..26] gpio_1_set,
        [26..27] gpio_1_clr,
        [28..29] gpio_1_i,
        [30..32] gpio_1_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg2: u32 {
        [00..01] gpio_2_ie,
        [01..02] gpio_2_smt,
        [02..04] gpio_2_drv,
        [04..05] gpio_2_pu,
        [05..06] gpio_2_pd,
        [06..07] gpio_2_oe,
        [08..13] gpio_2_func_sel,
        [16..20] gpio_2_int_mode_set,
        [20..21] gpio_2_int_clr,
        [21..22] gpio_2_int_stat,
        [22..23] gpio_2_int_mask,
        [24..25] gpio_2_o,
        [25..26] gpio_2_set,
        [26..27] gpio_2_clr,
        [28..29] gpio_2_i,
        [30..32] gpio_2_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg3: u32 {
        [00..01] gpio_3_ie,
        [01..02] gpio_3_smt,
        [02..04] gpio_3_drv,
        [04..05] gpio_3_pu,
        [05..06] gpio_3_pd,
        [06..07] gpio_3_oe,
        [08..13] gpio_3_func_sel,
        [16..20] gpio_3_int_mode_set,
        [20..21] gpio_3_int_clr,
        [21..22] gpio_3_int_stat,
        [22..23] gpio_3_int_mask,
        [24..25] gpio_3_o,
        [25..26] gpio_3_set,
        [26..27] gpio_3_clr,
        [28..29] gpio_3_i,
        [30..32] gpio_3_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg4: u32 {
        [00..01] gpio_4_ie,
        [01..02] gpio_4_smt,
        [02..04] gpio_4_drv,
        [04..05] gpio_4_pu,
        [05..06] gpio_4_pd,
        [06..07] gpio_4_oe,
        [08..13] gpio_4_func_sel,
        [16..20] gpio_4_int_mode_set,
        [20..21] gpio_4_int_clr,
        [21..22] gpio_4_int_stat,
        [22..23] gpio_4_int_mask,
        [24..25] gpio_4_o,
        [25..26] gpio_4_set,
        [26..27] gpio_4_clr,
        [28..29] gpio_4_i,
        [30..32] gpio_4_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg5: u32 {
        [00..01] gpio_5_ie,
        [01..02] gpio_5_smt,
        [02..04] gpio_5_drv,
        [04..05] gpio_5_pu,
        [05..06] gpio_5_pd,
        [06..07] gpio_5_oe,
        [08..13] gpio_5_func_sel,
        [16..20] gpio_5_int_mode_set,
        [20..21] gpio_5_int_clr,
        [21..22] gpio_5_int_stat,
        [22..23] gpio_5_int_mask,
        [24..25] gpio_5_o,
        [25..26] gpio_5_set,
        [26..27] gpio_5_clr,
        [28..29] gpio_5_i,
        [30..32] gpio_5_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg6: u32 {
        [00..01] gpio_6_ie,
        [01..02] gpio_6_smt,
        [02..04] gpio_6_drv,
        [04..05] gpio_6_pu,
        [05..06] gpio_6_pd,
        [06..07] gpio_6_oe,
        [08..13] gpio_6_func_sel,
        [16..20] gpio_6_int_mode_set,
        [20..21] gpio_6_int_clr,
        [21..22] gpio_6_int_stat,
        [22..23] gpio_6_int_mask,
        [24..25] gpio_6_o,
        [25..26] gpio_6_set,
        [26..27] gpio_6_clr,
        [28..29] gpio_6_i,
        [30..32] gpio_6_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg7: u32 {
        [00..01] gpio_7_ie,
        [01..02] gpio_7_smt,
        [02..04] gpio_7_drv,
        [04..05] gpio_7_pu,
        [05..06] gpio_7_pd,
        [06..07] gpio_7_oe,
        [08..13] gpio_7_func_sel,
        [16..20] gpio_7_int_mode_set,
        [20..21] gpio_7_int_clr,
        [21..22] gpio_7_int_stat,
        [22..23] gpio_7_int_mask,
        [24..25] gpio_7_o,
        [25..26] gpio_7_set,
        [26..27] gpio_7_clr,
        [28..29] gpio_7_i,
        [30..32] gpio_7_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg8: u32 {
        [00..01] gpio_8_ie,
        [01..02] gpio_8_smt,
        [02..04] gpio_8_drv,
        [04..05] gpio_8_pu,
        [05..06] gpio_8_pd,
        [06..07] gpio_8_oe,
        [08..13] gpio_8_func_sel,
        [16..20] gpio_8_int_mode_set,
        [20..21] gpio_8_int_clr,
        [21..22] gpio_8_int_stat,
        [22..23] gpio_8_int_mask,
        [24..25] gpio_8_o,
        [25..26] gpio_8_set,
        [26..27] gpio_8_clr,
        [28..29] gpio_8_i,
        [30..32] gpio_8_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg9: u32 {
        [00..01] gpio_9_ie,
        [01..02] gpio_9_smt,
        [02..04] gpio_9_drv,
        [04..05] gpio_9_pu,
        [05..06] gpio_9_pd,
        [06..07] gpio_9_oe,
        [08..13] gpio_9_func_sel,
        [16..20] gpio_9_int_mode_set,
        [20..21] gpio_9_int_clr,
        [21..22] gpio_9_int_stat,
        [22..23] gpio_9_int_mask,
        [24..25] gpio_9_o,
        [25..26] gpio_9_set,
        [26..27] gpio_9_clr,
        [28..29] gpio_9_i,
        [30..32] gpio_9_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg10: u32 {
        [00..01] gpio_10_ie,
        [01..02] gpio_10_smt,
        [02..04] gpio_10_drv,
        [04..05] gpio_10_pu,
        [05..06] gpio_10_pd,
        [06..07] gpio_10_oe,
        [08..13] gpio_10_func_sel,
        [16..20] gpio_10_int_mode_set,
        [20..21] gpio_10_int_clr,
        [21..22] gpio_10_int_stat,
        [22..23] gpio_10_int_mask,
        [24..25] gpio_10_o,
        [25..26] gpio_10_set,
        [26..27] gpio_10_clr,
        [28..29] gpio_10_i,
        [30..32] gpio_10_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg11: u32 {
        [00..01] gpio_11_ie,
        [01..02] gpio_11_smt,
        [02..04] gpio_11_drv,
        [04..05] gpio_11_pu,
        [05..06] gpio_11_pd,
        [06..07] gpio_11_oe,
        [08..13] gpio_11_func_sel,
        [16..20] gpio_11_int_mode_set,
        [20..21] gpio_11_int_clr,
        [21..22] gpio_11_int_stat,
        [22..23] gpio_11_int_mask,
        [24..25] gpio_11_o,
        [25..26] gpio_11_set,
        [26..27] gpio_11_clr,
        [28..29] gpio_11_i,
        [30..32] gpio_11_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg12: u32 {
        [00..01] gpio_12_ie,
        [01..02] gpio_12_smt,
        [02..04] gpio_12_drv,
        [04..05] gpio_12_pu,
        [05..06] gpio_12_pd,
        [06..07] gpio_12_oe,
        [08..13] gpio_12_func_sel,
        [16..20] gpio_12_int_mode_set,
        [20..21] gpio_12_int_clr,
        [21..22] gpio_12_int_stat,
        [22..23] gpio_12_int_mask,
        [24..25] gpio_12_o,
        [25..26] gpio_12_set,
        [26..27] gpio_12_clr,
        [28..29] gpio_12_i,
        [30..32] gpio_12_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg13: u32 {
        [00..01] gpio_13_ie,
        [01..02] gpio_13_smt,
        [02..04] gpio_13_drv,
        [04..05] gpio_13_pu,
        [05..06] gpio_13_pd,
        [06..07] gpio_13_oe,
        [08..13] gpio_13_func_sel,
        [16..20] gpio_13_int_mode_set,
        [20..21] gpio_13_int_clr,
        [21..22] gpio_13_int_stat,
        [22..23] gpio_13_int_mask,
        [24..25] gpio_13_o,
        [25..26] gpio_13_set,
        [26..27] gpio_13_clr,
        [28..29] gpio_13_i,
        [30..32] gpio_13_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg14: u32 {
        [00..01] gpio_14_ie,
        [01..02] gpio_14_smt,
        [02..04] gpio_14_drv,
        [04..05] gpio_14_pu,
        [05..06] gpio_14_pd,
        [06..07] gpio_14_oe,
        [08..13] gpio_14_func_sel,
        [16..20] gpio_14_int_mode_set,
        [20..21] gpio_14_int_clr,
        [21..22] gpio_14_int_stat,
        [22..23] gpio_14_int_mask,
        [24..25] gpio_14_o,
        [25..26] gpio_14_set,
        [26..27] gpio_14_clr,
        [28..29] gpio_14_i,
        [30..32] gpio_14_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg15: u32 {
        [00..01] gpio_15_ie,
        [01..02] gpio_15_smt,
        [02..04] gpio_15_drv,
        [04..05] gpio_15_pu,
        [05..06] gpio_15_pd,
        [06..07] gpio_15_oe,
        [08..13] gpio_15_func_sel,
        [16..20] gpio_15_int_mode_set,
        [20..21] gpio_15_int_clr,
        [21..22] gpio_15_int_stat,
        [22..23] gpio_15_int_mask,
        [24..25] gpio_15_o,
        [25..26] gpio_15_set,
        [26..27] gpio_15_clr,
        [28..29] gpio_15_i,
        [30..32] gpio_15_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg16: u32 {
        [00..01] gpio_16_ie,
        [01..02] gpio_16_smt,
        [02..04] gpio_16_drv,
        [04..05] gpio_16_pu,
        [05..06] gpio_16_pd,
        [06..07] gpio_16_oe,
        [08..13] gpio_16_func_sel,
        [16..20] gpio_16_int_mode_set,
        [20..21] gpio_16_int_clr,
        [21..22] gpio_16_int_stat,
        [22..23] gpio_16_int_mask,
        [24..25] gpio_16_o,
        [25..26] gpio_16_set,
        [26..27] gpio_16_clr,
        [28..29] gpio_16_i,
        [30..32] gpio_16_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg17: u32 {
        [00..01] gpio_17_ie,
        [01..02] gpio_17_smt,
        [02..04] gpio_17_drv,
        [04..05] gpio_17_pu,
        [05..06] gpio_17_pd,
        [06..07] gpio_17_oe,
        [08..13] gpio_17_func_sel,
        [16..20] gpio_17_int_mode_set,
        [20..21] gpio_17_int_clr,
        [21..22] gpio_17_int_stat,
        [22..23] gpio_17_int_mask,
        [24..25] gpio_17_o,
        [25..26] gpio_17_set,
        [26..27] gpio_17_clr,
        [28..29] gpio_17_i,
        [30..32] gpio_17_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg18: u32 {
        [00..01] gpio_18_ie,
        [01..02] gpio_18_smt,
        [02..04] gpio_18_drv,
        [04..05] gpio_18_pu,
        [05..06] gpio_18_pd,
        [06..07] gpio_18_oe,
        [08..13] gpio_18_func_sel,
        [16..20] gpio_18_int_mode_set,
        [20..21] gpio_18_int_clr,
        [21..22] gpio_18_int_stat,
        [22..23] gpio_18_int_mask,
        [24..25] gpio_18_o,
        [25..26] gpio_18_set,
        [26..27] gpio_18_clr,
        [28..29] gpio_18_i,
        [30..32] gpio_18_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg19: u32 {
        [00..01] gpio_19_ie,
        [01..02] gpio_19_smt,
        [02..04] gpio_19_drv,
        [04..05] gpio_19_pu,
        [05..06] gpio_19_pd,
        [06..07] gpio_19_oe,
        [08..13] gpio_19_func_sel,
        [16..20] gpio_19_int_mode_set,
        [20..21] gpio_19_int_clr,
        [21..22] gpio_19_int_stat,
        [22..23] gpio_19_int_mask,
        [24..25] gpio_19_o,
        [25..26] gpio_19_set,
        [26..27] gpio_19_clr,
        [28..29] gpio_19_i,
        [30..32] gpio_19_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg20: u32 {
        [00..01] gpio_20_ie,
        [01..02] gpio_20_smt,
        [02..04] gpio_20_drv,
        [04..05] gpio_20_pu,
        [05..06] gpio_20_pd,
        [06..07] gpio_20_oe,
        [08..13] gpio_20_func_sel,
        [16..20] gpio_20_int_mode_set,
        [20..21] gpio_20_int_clr,
        [21..22] gpio_20_int_stat,
        [22..23] gpio_20_int_mask,
        [24..25] gpio_20_o,
        [25..26] gpio_20_set,
        [26..27] gpio_20_clr,
        [28..29] gpio_20_i,
        [30..32] gpio_20_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg21: u32 {
        [00..01] gpio_21_ie,
        [01..02] gpio_21_smt,
        [02..04] gpio_21_drv,
        [04..05] gpio_21_pu,
        [05..06] gpio_21_pd,
        [06..07] gpio_21_oe,
        [08..13] gpio_21_func_sel,
        [16..20] gpio_21_int_mode_set,
        [20..21] gpio_21_int_clr,
        [21..22] gpio_21_int_stat,
        [22..23] gpio_21_int_mask,
        [24..25] gpio_21_o,
        [25..26] gpio_21_set,
        [26..27] gpio_21_clr,
        [28..29] gpio_21_i,
        [30..32] gpio_21_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg22: u32 {
        [00..01] gpio_22_ie,
        [01..02] gpio_22_smt,
        [02..04] gpio_22_drv,
        [04..05] gpio_22_pu,
        [05..06] gpio_22_pd,
        [06..07] gpio_22_oe,
        [08..13] gpio_22_func_sel,
        [16..20] gpio_22_int_mode_set,
        [20..21] gpio_22_int_clr,
        [21..22] gpio_22_int_stat,
        [22..23] gpio_22_int_mask,
        [24..25] gpio_22_o,
        [25..26] gpio_22_set,
        [26..27] gpio_22_clr,
        [28..29] gpio_22_i,
        [30..32] gpio_22_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg23: u32 {
        [00..01] gpio_23_ie,
        [01..02] gpio_23_smt,
        [02..04] gpio_23_drv,
        [04..05] gpio_23_pu,
        [05..06] gpio_23_pd,
        [06..07] gpio_23_oe,
        [08..13] gpio_23_func_sel,
        [16..20] gpio_23_int_mode_set,
        [20..21] gpio_23_int_clr,
        [21..22] gpio_23_int_stat,
        [22..23] gpio_23_int_mask,
        [24..25] gpio_23_o,
        [25..26] gpio_23_set,
        [26..27] gpio_23_clr,
        [28..29] gpio_23_i,
        [30..32] gpio_23_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg24: u32 {
        [00..01] gpio_24_ie,
        [01..02] gpio_24_smt,
        [02..04] gpio_24_drv,
        [04..05] gpio_24_pu,
        [05..06] gpio_24_pd,
        [06..07] gpio_24_oe,
        [08..13] gpio_24_func_sel,
        [16..20] gpio_24_int_mode_set,
        [20..21] gpio_24_int_clr,
        [21..22] gpio_24_int_stat,
        [22..23] gpio_24_int_mask,
        [24..25] gpio_24_o,
        [25..26] gpio_24_set,
        [26..27] gpio_24_clr,
        [28..29] gpio_24_i,
        [30..32] gpio_24_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg25: u32 {
        [00..01] gpio_25_ie,
        [01..02] gpio_25_smt,
        [02..04] gpio_25_drv,
        [04..05] gpio_25_pu,
        [05..06] gpio_25_pd,
        [06..07] gpio_25_oe,
        [08..13] gpio_25_func_sel,
        [16..20] gpio_25_int_mode_set,
        [20..21] gpio_25_int_clr,
        [21..22] gpio_25_int_stat,
        [22..23] gpio_25_int_mask,
        [24..25] gpio_25_o,
        [25..26] gpio_25_set,
        [26..27] gpio_25_clr,
        [28..29] gpio_25_i,
        [30..32] gpio_25_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg26: u32 {
        [00..01] gpio_26_ie,
        [01..02] gpio_26_smt,
        [02..04] gpio_26_drv,
        [04..05] gpio_26_pu,
        [05..06] gpio_26_pd,
        [06..07] gpio_26_oe,
        [08..13] gpio_26_func_sel,
        [16..20] gpio_26_int_mode_set,
        [20..21] gpio_26_int_clr,
        [21..22] gpio_26_int_stat,
        [22..23] gpio_26_int_mask,
        [24..25] gpio_26_o,
        [25..26] gpio_26_set,
        [26..27] gpio_26_clr,
        [28..29] gpio_26_i,
        [30..32] gpio_26_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg27: u32 {
        [00..01] gpio_27_ie,
        [01..02] gpio_27_smt,
        [02..04] gpio_27_drv,
        [04..05] gpio_27_pu,
        [05..06] gpio_27_pd,
        [06..07] gpio_27_oe,
        [08..13] gpio_27_func_sel,
        [16..20] gpio_27_int_mode_set,
        [20..21] gpio_27_int_clr,
        [21..22] gpio_27_int_stat,
        [22..23] gpio_27_int_mask,
        [24..25] gpio_27_o,
        [25..26] gpio_27_set,
        [26..27] gpio_27_clr,
        [28..29] gpio_27_i,
        [30..32] gpio_27_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg28: u32 {
        [00..01] gpio_28_ie,
        [01..02] gpio_28_smt,
        [02..04] gpio_28_drv,
        [04..05] gpio_28_pu,
        [05..06] gpio_28_pd,
        [06..07] gpio_28_oe,
        [08..13] gpio_28_func_sel,
        [16..20] gpio_28_int_mode_set,
        [20..21] gpio_28_int_clr,
        [21..22] gpio_28_int_stat,
        [22..23] gpio_28_int_mask,
        [24..25] gpio_28_o,
        [25..26] gpio_28_set,
        [26..27] gpio_28_clr,
        [28..29] gpio_28_i,
        [30..32] gpio_28_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg29: u32 {
        [00..01] gpio_29_ie,
        [01..02] gpio_29_smt,
        [02..04] gpio_29_drv,
        [04..05] gpio_29_pu,
        [05..06] gpio_29_pd,
        [06..07] gpio_29_oe,
        [08..13] gpio_29_func_sel,
        [16..20] gpio_29_int_mode_set,
        [20..21] gpio_29_int_clr,
        [21..22] gpio_29_int_stat,
        [22..23] gpio_29_int_mask,
        [24..25] gpio_29_o,
        [25..26] gpio_29_set,
        [26..27] gpio_29_clr,
        [28..29] gpio_29_i,
        [30..32] gpio_29_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg30: u32 {
        [00..01] gpio_30_ie,
        [01..02] gpio_30_smt,
        [02..04] gpio_30_drv,
        [04..05] gpio_30_pu,
        [05..06] gpio_30_pd,
        [06..07] gpio_30_oe,
        [08..13] gpio_30_func_sel,
        [16..20] gpio_30_int_mode_set,
        [20..21] gpio_30_int_clr,
        [21..22] gpio_30_int_stat,
        [22..23] gpio_30_int_mask,
        [24..25] gpio_30_o,
        [25..26] gpio_30_set,
        [26..27] gpio_30_clr,
        [28..29] gpio_30_i,
        [30..32] gpio_30_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg31: u32 {
        [00..01] gpio_31_ie,
        [01..02] gpio_31_smt,
        [02..04] gpio_31_drv,
        [04..05] gpio_31_pu,
        [05..06] gpio_31_pd,
        [06..07] gpio_31_oe,
        [08..13] gpio_31_func_sel,
        [16..20] gpio_31_int_mode_set,
        [20..21] gpio_31_int_clr,
        [21..22] gpio_31_int_stat,
        [22..23] gpio_31_int_mask,
        [24..25] gpio_31_o,
        [25..26] gpio_31_set,
        [26..27] gpio_31_clr,
        [28..29] gpio_31_i,
        [30..32] gpio_31_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg32: u32 {
        [00..01] gpio_32_ie,
        [01..02] gpio_32_smt,
        [02..04] gpio_32_drv,
        [04..05] gpio_32_pu,
        [05..06] gpio_32_pd,
        [06..07] gpio_32_oe,
        [08..13] gpio_32_func_sel,
        [16..20] gpio_32_int_mode_set,
        [20..21] gpio_32_int_clr,
        [21..22] gpio_32_int_stat,
        [22..23] gpio_32_int_mask,
        [24..25] gpio_32_o,
        [25..26] gpio_32_set,
        [26..27] gpio_32_clr,
        [28..29] gpio_32_i,
        [30..32] gpio_32_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg33: u32 {
        [00..01] gpio_33_ie,
        [01..02] gpio_33_smt,
        [02..04] gpio_33_drv,
        [04..05] gpio_33_pu,
        [05..06] gpio_33_pd,
        [06..07] gpio_33_oe,
        [08..13] gpio_33_func_sel,
        [16..20] gpio_33_int_mode_set,
        [20..21] gpio_33_int_clr,
        [21..22] gpio_33_int_stat,
        [22..23] gpio_33_int_mask,
        [24..25] gpio_33_o,
        [25..26] gpio_33_set,
        [26..27] gpio_33_clr,
        [28..29] gpio_33_i,
        [30..32] gpio_33_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg34: u32 {
        [00..01] gpio_34_ie,
        [01..02] gpio_34_smt,
        [02..04] gpio_34_drv,
        [04..05] gpio_34_pu,
        [05..06] gpio_34_pd,
        [06..07] gpio_34_oe,
        [08..13] gpio_34_func_sel,
        [16..20] gpio_34_int_mode_set,
        [20..21] gpio_34_int_clr,
        [21..22] gpio_34_int_stat,
        [22..23] gpio_34_int_mask,
        [24..25] gpio_34_o,
        [25..26] gpio_34_set,
        [26..27] gpio_34_clr,
        [28..29] gpio_34_i,
        [30..32] gpio_34_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg35: u32 {
        [00..01] gpio_35_ie,
        [01..02] gpio_35_smt,
        [02..04] gpio_35_drv,
        [04..05] gpio_35_pu,
        [05..06] gpio_35_pd,
        [06..07] gpio_35_oe,
        [08..13] gpio_35_func_sel,
        [16..20] gpio_35_int_mode_set,
        [20..21] gpio_35_int_clr,
        [21..22] gpio_35_int_stat,
        [22..23] gpio_35_int_mask,
        [24..25] gpio_35_o,
        [25..26] gpio_35_set,
        [26..27] gpio_35_clr,
        [28..29] gpio_35_i,
        [30..32] gpio_35_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg36: u32 {
        [00..01] gpio_36_ie,
        [01..02] gpio_36_smt,
        [02..04] gpio_36_drv,
        [04..05] gpio_36_pu,
        [05..06] gpio_36_pd,
        [06..07] gpio_36_oe,
        [08..13] gpio_36_func_sel,
        [16..20] gpio_36_int_mode_set,
        [20..21] gpio_36_int_clr,
        [21..22] gpio_36_int_stat,
        [22..23] gpio_36_int_mask,
        [24..25] gpio_36_o,
        [25..26] gpio_36_set,
        [26..27] gpio_36_clr,
        [28..29] gpio_36_i,
        [30..32] gpio_36_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg37: u32 {
        [00..01] gpio_37_ie,
        [01..02] gpio_37_smt,
        [02..04] gpio_37_drv,
        [04..05] gpio_37_pu,
        [05..06] gpio_37_pd,
        [06..07] gpio_37_oe,
        [08..13] gpio_37_func_sel,
        [16..20] gpio_37_int_mode_set,
        [20..21] gpio_37_int_clr,
        [21..22] gpio_37_int_stat,
        [22..23] gpio_37_int_mask,
        [24..25] gpio_37_o,
        [25..26] gpio_37_set,
        [26..27] gpio_37_clr,
        [28..29] gpio_37_i,
        [30..32] gpio_37_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg38: u32 {
        [00..01] gpio_38_ie,
        [01..02] gpio_38_smt,
        [02..04] gpio_38_drv,
        [04..05] gpio_38_pu,
        [05..06] gpio_38_pd,
        [06..07] gpio_38_oe,
        [08..13] gpio_38_func_sel,
        [16..20] gpio_38_int_mode_set,
        [20..21] gpio_38_int_clr,
        [21..22] gpio_38_int_stat,
        [22..23] gpio_38_int_mask,
        [24..25] gpio_38_o,
        [25..26] gpio_38_set,
        [26..27] gpio_38_clr,
        [28..29] gpio_38_i,
        [30..32] gpio_38_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg39: u32 {
        [00..01] gpio_39_ie,
        [01..02] gpio_39_smt,
        [02..04] gpio_39_drv,
        [04..05] gpio_39_pu,
        [05..06] gpio_39_pd,
        [06..07] gpio_39_oe,
        [08..13] gpio_39_func_sel,
        [16..20] gpio_39_int_mode_set,
        [20..21] gpio_39_int_clr,
        [21..22] gpio_39_int_stat,
        [22..23] gpio_39_int_mask,
        [24..25] gpio_39_o,
        [25..26] gpio_39_set,
        [26..27] gpio_39_clr,
        [28..29] gpio_39_i,
        [30..32] gpio_39_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg40: u32 {
        [00..01] gpio_40_ie,
        [01..02] gpio_40_smt,
        [02..04] gpio_40_drv,
        [04..05] gpio_40_pu,
        [05..06] gpio_40_pd,
        [06..07] gpio_40_oe,
        [08..13] gpio_40_func_sel,
        [16..20] gpio_40_int_mode_set,
        [20..21] gpio_40_int_clr,
        [21..22] gpio_40_int_stat,
        [22..23] gpio_40_int_mask,
        [24..25] gpio_40_o,
        [25..26] gpio_40_set,
        [26..27] gpio_40_clr,
        [28..29] gpio_40_i,
        [30..32] gpio_40_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg41: u32 {
        [00..01] gpio_41_ie,
        [01..02] gpio_41_smt,
        [02..04] gpio_41_drv,
        [04..05] gpio_41_pu,
        [05..06] gpio_41_pd,
        [06..07] gpio_41_oe,
        [08..13] gpio_41_func_sel,
        [16..20] gpio_41_int_mode_set,
        [20..21] gpio_41_int_clr,
        [21..22] gpio_41_int_stat,
        [22..23] gpio_41_int_mask,
        [24..25] gpio_41_o,
        [25..26] gpio_41_set,
        [26..27] gpio_41_clr,
        [28..29] gpio_41_i,
        [30..32] gpio_41_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg42: u32 {
        [00..01] gpio_42_ie,
        [01..02] gpio_42_smt,
        [02..04] gpio_42_drv,
        [04..05] gpio_42_pu,
        [05..06] gpio_42_pd,
        [06..07] gpio_42_oe,
        [08..13] gpio_42_func_sel,
        [16..20] gpio_42_int_mode_set,
        [20..21] gpio_42_int_clr,
        [21..22] gpio_42_int_stat,
        [22..23] gpio_42_int_mask,
        [24..25] gpio_42_o,
        [25..26] gpio_42_set,
        [26..27] gpio_42_clr,
        [28..29] gpio_42_i,
        [30..32] gpio_42_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg43: u32 {
        [00..01] gpio_43_ie,
        [01..02] gpio_43_smt,
        [02..04] gpio_43_drv,
        [04..05] gpio_43_pu,
        [05..06] gpio_43_pd,
        [06..07] gpio_43_oe,
        [08..13] gpio_43_func_sel,
        [16..20] gpio_43_int_mode_set,
        [20..21] gpio_43_int_clr,
        [21..22] gpio_43_int_stat,
        [22..23] gpio_43_int_mask,
        [24..25] gpio_43_o,
        [25..26] gpio_43_set,
        [26..27] gpio_43_clr,
        [28..29] gpio_43_i,
        [30..32] gpio_43_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg44: u32 {
        [00..01] gpio_44_ie,
        [01..02] gpio_44_smt,
        [02..04] gpio_44_drv,
        [04..05] gpio_44_pu,
        [05..06] gpio_44_pd,
        [06..07] gpio_44_oe,
        [08..13] gpio_44_func_sel,
        [16..20] gpio_44_int_mode_set,
        [20..21] gpio_44_int_clr,
        [21..22] gpio_44_int_stat,
        [22..23] gpio_44_int_mask,
        [24..25] gpio_44_o,
        [25..26] gpio_44_set,
        [26..27] gpio_44_clr,
        [28..29] gpio_44_i,
        [30..32] gpio_44_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg45: u32 {
        [00..01] gpio_45_ie,
        [01..02] gpio_45_smt,
        [02..04] gpio_45_drv,
        [04..05] gpio_45_pu,
        [05..06] gpio_45_pd,
        [06..07] gpio_45_oe,
        [08..13] gpio_45_func_sel,
        [16..20] gpio_45_int_mode_set,
        [20..21] gpio_45_int_clr,
        [21..22] gpio_45_int_stat,
        [22..23] gpio_45_int_mask,
        [24..25] gpio_45_o,
        [25..26] gpio_45_set,
        [26..27] gpio_45_clr,
        [28..29] gpio_45_i,
        [30..32] gpio_45_mode,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg46: u32 {
        [00..01] gpio_46_ie,
        [01..02] gpio_46_smt,
        [02..04] gpio_46_drv,
        [04..05] gpio_46_pu,
        [05..06] gpio_46_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg47: u32 {
        [00..01] gpio_47_ie,
        [01..02] gpio_47_smt,
        [02..04] gpio_47_drv,
        [04..05] gpio_47_pu,
        [05..06] gpio_47_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg48: u32 {
        [00..01] gpio_48_ie,
        [01..02] gpio_48_smt,
        [02..04] gpio_48_drv,
        [04..05] gpio_48_pu,
        [05..06] gpio_48_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg49: u32 {
        [00..01] gpio_49_ie,
        [01..02] gpio_49_smt,
        [02..04] gpio_49_drv,
        [04..05] gpio_49_pu,
        [05..06] gpio_49_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg50: u32 {
        [00..01] gpio_50_ie,
        [01..02] gpio_50_smt,
        [02..04] gpio_50_drv,
        [04..05] gpio_50_pu,
        [05..06] gpio_50_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg51: u32 {
        [00..01] gpio_51_ie,
        [01..02] gpio_51_smt,
        [02..04] gpio_51_drv,
        [04..05] gpio_51_pu,
        [05..06] gpio_51_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg52: u32 {
        [00..01] gpio_52_ie,
        [01..02] gpio_52_smt,
        [02..04] gpio_52_drv,
        [04..05] gpio_52_pu,
        [05..06] gpio_52_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg53: u32 {
        [00..01] gpio_53_ie,
        [01..02] gpio_53_smt,
        [02..04] gpio_53_drv,
        [04..05] gpio_53_pu,
        [05..06] gpio_53_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg54: u32 {
        [00..01] gpio_54_ie,
        [01..02] gpio_54_smt,
        [02..04] gpio_54_drv,
        [04..05] gpio_54_pu,
        [05..06] gpio_54_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg55: u32 {
        [00..01] gpio_55_ie,
        [01..02] gpio_55_smt,
        [02..04] gpio_55_drv,
        [04..05] gpio_55_pu,
        [05..06] gpio_55_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg56: u32 {
        [00..01] gpio_56_ie,
        [01..02] gpio_56_smt,
        [02..04] gpio_56_drv,
        [04..05] gpio_56_pu,
        [05..06] gpio_56_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg57: u32 {
        [00..01] gpio_57_ie,
        [01..02] gpio_57_smt,
        [02..04] gpio_57_drv,
        [04..05] gpio_57_pu,
        [05..06] gpio_57_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg58: u32 {
        [00..01] gpio_58_ie,
        [01..02] gpio_58_smt,
        [02..04] gpio_58_drv,
        [04..05] gpio_58_pu,
        [05..06] gpio_58_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg59: u32 {
        [00..01] gpio_59_ie,
        [01..02] gpio_59_smt,
        [02..04] gpio_59_drv,
        [04..05] gpio_59_pu,
        [05..06] gpio_59_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg60: u32 {
        [00..01] gpio_60_ie,
        [01..02] gpio_60_smt,
        [02..04] gpio_60_drv,
        [04..05] gpio_60_pu,
        [05..06] gpio_60_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg61: u32 {
        [00..01] gpio_61_ie,
        [01..02] gpio_61_smt,
        [02..04] gpio_61_drv,
        [04..05] gpio_61_pu,
        [05..06] gpio_61_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg62: u32 {
        [00..01] gpio_62_ie,
        [01..02] gpio_62_smt,
        [02..04] gpio_62_drv,
        [04..05] gpio_62_pu,
        [05..06] gpio_62_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg63: u32 {
        [00..01] gpio_63_ie,
        [01..02] gpio_63_smt,
        [02..04] gpio_63_drv,
        [04..05] gpio_63_pu,
        [05..06] gpio_63_pd,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg128: u32 {
        [00..01] gpio_0_i,
        [01..02] gpio_1_i,
        [02..03] gpio_2_i,
        [03..04] gpio_3_i,
        [04..05] gpio_4_i,
        [05..06] gpio_5_i,
        [06..07] gpio_6_i,
        [07..08] gpio_7_i,
        [08..09] gpio_8_i,
        [09..10] gpio_9_i,
        [10..11] gpio_10_i,
        [11..12] gpio_11_i,
        [12..13] gpio_12_i,
        [13..14] gpio_13_i,
        [14..15] gpio_14_i,
        [15..16] gpio_15_i,
        [16..17] gpio_16_i,
        [17..18] gpio_17_i,
        [18..19] gpio_18_i,
        [19..20] gpio_19_i,
        [20..21] gpio_20_i,
        [21..22] gpio_21_i,
        [22..23] gpio_22_i,
        [23..24] gpio_23_i,
        [24..25] gpio_24_i,
        [25..26] gpio_25_i,
        [26..27] gpio_26_i,
        [27..28] gpio_27_i,
        [28..29] gpio_28_i,
        [29..30] gpio_29_i,
        [30..31] gpio_30_i,
        [31..32] gpio_31_i,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg129: u32 {
        [00..01] gpio_32_i,
        [01..02] gpio_33_i,
        [02..03] gpio_34_i,
        [03..04] gpio_35_i,
        [04..05] gpio_36_i,
        [05..06] gpio_37_i,
        [06..07] gpio_38_i,
        [07..08] gpio_39_i,
        [08..09] gpio_40_i,
        [09..10] gpio_41_i,
        [10..11] gpio_42_i,
        [11..12] gpio_43_i,
        [12..13] gpio_44_i,
        [13..14] gpio_45_i,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg136: u32 {
        [00..01] gpio_0_o,
        [01..02] gpio_1_o,
        [02..03] gpio_2_o,
        [03..04] gpio_3_o,
        [04..05] gpio_4_o,
        [05..06] gpio_5_o,
        [06..07] gpio_6_o,
        [07..08] gpio_7_o,
        [08..09] gpio_8_o,
        [09..10] gpio_9_o,
        [10..11] gpio_10_o,
        [11..12] gpio_11_o,
        [12..13] gpio_12_o,
        [13..14] gpio_13_o,
        [14..15] gpio_14_o,
        [15..16] gpio_15_o,
        [16..17] gpio_16_o,
        [17..18] gpio_17_o,
        [18..19] gpio_18_o,
        [19..20] gpio_19_o,
        [20..21] gpio_20_o,
        [21..22] gpio_21_o,
        [22..23] gpio_22_o,
        [23..24] gpio_23_o,
        [24..25] gpio_24_o,
        [25..26] gpio_25_o,
        [26..27] gpio_26_o,
        [27..28] gpio_27_o,
        [28..29] gpio_28_o,
        [29..30] gpio_29_o,
        [30..31] gpio_30_o,
        [31..32] gpio_31_o,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg137: u32 {
        [00..01] gpio_32_o,
        [01..02] gpio_33_o,
        [02..03] gpio_34_o,
        [03..04] gpio_35_o,
        [04..05] gpio_36_o,
        [05..06] gpio_37_o,
        [06..07] gpio_38_o,
        [07..08] gpio_39_o,
        [08..09] gpio_40_o,
        [09..10] gpio_41_o,
        [10..11] gpio_42_o,
        [11..12] gpio_43_o,
        [12..13] gpio_44_o,
        [13..14] gpio_45_o,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg138: u32 {
        [00..01] gpio_0_set,
        [01..02] gpio_1_set,
        [02..03] gpio_2_set,
        [03..04] gpio_3_set,
        [04..05] gpio_4_set,
        [05..06] gpio_5_set,
        [06..07] gpio_6_set,
        [07..08] gpio_7_set,
        [08..09] gpio_8_set,
        [09..10] gpio_9_set,
        [10..11] gpio_10_set,
        [11..12] gpio_11_set,
        [12..13] gpio_12_set,
        [13..14] gpio_13_set,
        [14..15] gpio_14_set,
        [15..16] gpio_15_set,
        [16..17] gpio_16_set,
        [17..18] gpio_17_set,
        [18..19] gpio_18_set,
        [19..20] gpio_19_set,
        [20..21] gpio_20_set,
        [21..22] gpio_21_set,
        [22..23] gpio_22_set,
        [23..24] gpio_23_set,
        [24..25] gpio_24_set,
        [25..26] gpio_25_set,
        [26..27] gpio_26_set,
        [27..28] gpio_27_set,
        [28..29] gpio_28_set,
        [29..30] gpio_29_set,
        [30..31] gpio_30_set,
        [31..32] gpio_31_set,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg139: u32 {
        [00..01] gpio_32_set,
        [01..02] gpio_33_set,
        [02..03] gpio_34_set,
        [03..04] gpio_35_set,
        [04..05] gpio_36_set,
        [05..06] gpio_37_set,
        [06..07] gpio_38_set,
        [07..08] gpio_39_set,
        [08..09] gpio_40_set,
        [09..10] gpio_41_set,
        [10..11] gpio_42_set,
        [11..12] gpio_43_set,
        [12..13] gpio_44_set,
        [13..14] gpio_45_set,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg140: u32 {
        [00..01] gpio_0_clr,
        [01..02] gpio_1_clr,
        [02..03] gpio_2_clr,
        [03..04] gpio_3_clr,
        [04..05] gpio_4_clr,
        [05..06] gpio_5_clr,
        [06..07] gpio_6_clr,
        [07..08] gpio_7_clr,
        [08..09] gpio_8_clr,
        [09..10] gpio_9_clr,
        [10..11] gpio_10_clr,
        [11..12] gpio_11_clr,
        [12..13] gpio_12_clr,
        [13..14] gpio_13_clr,
        [14..15] gpio_14_clr,
        [15..16] gpio_15_clr,
        [16..17] gpio_16_clr,
        [17..18] gpio_17_clr,
        [18..19] gpio_18_clr,
        [19..20] gpio_19_clr,
        [20..21] gpio_20_clr,
        [21..22] gpio_21_clr,
        [22..23] gpio_22_clr,
        [23..24] gpio_23_clr,
        [24..25] gpio_24_clr,
        [25..26] gpio_25_clr,
        [26..27] gpio_26_clr,
        [27..28] gpio_27_clr,
        [28..29] gpio_28_clr,
        [29..30] gpio_29_clr,
        [30..31] gpio_30_clr,
        [31..32] gpio_31_clr,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg141: u32 {
        [00..01] gpio_32_clr,
        [01..02] gpio_33_clr,
        [02..03] gpio_34_clr,
        [03..04] gpio_35_clr,
        [04..05] gpio_36_clr,
        [05..06] gpio_37_clr,
        [06..07] gpio_38_clr,
        [07..08] gpio_39_clr,
        [08..09] gpio_40_clr,
        [09..10] gpio_41_clr,
        [10..11] gpio_42_clr,
        [11..12] gpio_43_clr,
        [12..13] gpio_44_clr,
        [13..14] gpio_45_clr,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg142: u32 {
        [00..01] cr_gpio_tx_en,
        [01..02] cr_invert_code0_high,
        [02..03] cr_invert_code1_high,
        [07..16] cr_code_total_time,
        [16..24] cr_code0_high_time,
        [24..32] cr_code1_high_time,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg143: u32 {
        [00..01] cr_gpio_dma_tx_en,
        [01..02] cr_gpio_dma_out_sel_latch,
        [02..03] gpio_tx_fifo_clr,
        [03..04] gpio_tx_end_clr,
        [04..05] gpio_tx_fifo_overflow,
        [05..06] gpio_tx_fifo_underflow,
        [07..08] cr_gpio_dma_park_value,
        [08..16] gpio_tx_fifo_cnt,
        [16..23] cr_gpio_tx_fifo_th,
        [23..24] cr_gpio_tx_end_mask,
        [24..25] cr_gpio_tx_fifo_mask,
        [25..26] cr_gpio_tx_fer_mask,
        [26..27] r_gpio_tx_end_int,
        [27..28] r_gpio_tx_fifo_int,
        [28..29] r_gpio_tx_fer_int,
        [29..30] cr_gpio_tx_end_en,
        [30..31] cr_gpio_tx_fifo_en,
        [31..32] cr_gpio_tx_fer_en,
    }
}

embedded_util::reg! {
    pub struct GlbGpioCfg144: u32 {
        [00..16] gpio_tx_data_to_fifo,
    }
}
