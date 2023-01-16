//! MCU E907 register.

embedded_util::mmio! {
    pub struct McuMisc {
        [0x000] rw mcu_bus_cfg0: McuMiscMcuBusCfg0,
        [0x004] rw mcu_bus_cfg1: McuMiscMcuBusCfg1,
        /// Alias for `mcu_e907_rtc`.
        [0x014] rw cpu_mtimer_rtc: super::CpuRtc,
        [0x014] rw mcu_e907_rtc: McuMiscMcuE907Rtc,
        [0x100] rw mcu_cfg1: McuMiscMcuCfg1,
        [0x110] rw mcu1_log1: McuMiscMcu1Log1,
        [0x114] rw mcu1_log2: McuMiscMcu1Log2,
        [0x118] rw mcu1_log3: McuMiscMcu1Log3,
        [0x11C] rw mcu1_log4: McuMiscMcu1Log4,
        [0x120] rw mcu1_log5: McuMiscMcu1Log5,
        [0x208] rw irom1_misr_dataout_0: McuMiscIrom1MisrDataout0,
        [0x20C] rw irom1_misr_dataout_1: McuMiscIrom1MisrDataout1,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcuBusCfg0: u32 {
        [00..01] mcu_infra_timeout_en,
        [01..02] mcu_infra_timeout_clr,
        [16..17] sts_mcu_infra_timeout,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcuBusCfg1: u32 {
        [00..01] mcu1_hqos,
        [01..02] mcu1_awqos,
        [02..03] mcu1_arqos,
        [03..04] mcu_x2hs_sp_bypass,
        [07..09] x_wthre_mcu2ext,
        [16..17] mcu_infra_arb_mode,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcuE907Rtc: u32 {
        [00..10] mcu_rtc_div,
        [30..31] mcu_rtc_rst,
        [31..32] mcu_rtc_en,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcuCfg1: u32 {
        [00..01] mcu1_dfs_req,
        [02..03] sts_mcu1_dfs_ack,
        [04..06] mcu1_srst_en,
        [10..12] sts_mcu1_lpmd_b,
        [16..17] mcu1_wfi_force,
        [28..29] mcu1_ndm_rstn_en,
        [29..30] mcu1_hart_rstn_en,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcu1Log1: u32 {
        [00..32] sts_mcu1_mcause,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcu1Log2: u32 {
        [00..32] sts_mcu1_mintstatus,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcu1Log3: u32 {
        [00..32] sts_mcu1_mstatus,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcu1Log4: u32 {
        [00..01] sts_mcu1_sp,
        [01..32] sts_mcu1_pc,
    }
}

embedded_util::reg! {
    pub struct McuMiscMcu1Log5: u32 {
        [24..25] sts_mcu1_lockup,
        [25..26] sts_mcu1_halted,
        [28..29] mcu1_ndm_rstn_req,
        [29..30] mcu1_hart_rstn_req,
    }
}

embedded_util::reg! {
    pub struct McuMiscIrom1MisrDataout0: u32 {
    }
}

embedded_util::reg! {
    pub struct McuMiscIrom1MisrDataout1: u32 {
    }
}
