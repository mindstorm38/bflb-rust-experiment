//! MCU E907 register.

emhal::mmio_struct! {
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

emhal::mmio_reg! {
    pub struct McuMiscMcuBusCfg0: u32 {
        [0..1] mcu_infra_timeout_en,
        [1..2] mcu_infra_timeout_clr,
        [16..17] sts_mcu_infra_timeout,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcuBusCfg1: u32 {
        [0..1] mcu1_hqos,
        [1..2] mcu1_awqos,
        [2..3] mcu1_arqos,
        [3..4] mcu_x2hs_sp_bypass,
        [7..9] x_wthre_mcu2ext,
        [16..17] mcu_infra_arb_mode,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcuE907Rtc: u32 {
        [0..10] mcu_rtc_div,
        [30..31] mcu_rtc_rst,
        [31..32] mcu_rtc_en,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcuCfg1: u32 {
        [0..1] mcu1_dfs_req,
        [2..3] sts_mcu1_dfs_ack,
        [4..6] mcu1_srst_en,
        [10..12] sts_mcu1_lpmd_b,
        [16..17] mcu1_wfi_force,
        [28..29] mcu1_ndm_rstn_en,
        [29..30] mcu1_hart_rstn_en,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcu1Log1: u32 {
        [0..32] sts_mcu1_mcause,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcu1Log2: u32 {
        [0..32] sts_mcu1_mintstatus,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcu1Log3: u32 {
        [0..32] sts_mcu1_mstatus,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcu1Log4: u32 {
        [0..1] sts_mcu1_sp,
        [1..32] sts_mcu1_pc,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscMcu1Log5: u32 {
        [24..25] sts_mcu1_lockup,
        [25..26] sts_mcu1_halted,
        [28..29] mcu1_ndm_rstn_req,
        [29..30] mcu1_hart_rstn_req,
    }
}

emhal::mmio_reg! {
    pub struct McuMiscIrom1MisrDataout0: u32 {
    }
}

emhal::mmio_reg! {
    pub struct McuMiscIrom1MisrDataout1: u32 {
    }
}
