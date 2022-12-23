//! MM C906 register.

emhal::mmio_struct! {
    pub struct MmMisc {
        [0x000] rw cpu0_boot: MmMiscCpu0Boot,
        [0x008] rw cpu_cfg: MmMiscCpuCfg,
        [0x00C] rw cpu_sts1: MmMiscCpuSts1,
        [0x010] rw cpu_sts2: MmMiscCpuSts2,
        /// Alias for `cpu_rtc`.
        [0x018] rw cpu_mtimer_rtc: super::CpuRtc,
        [0x018] rw cpu_rtc: MmMiscCpuRtc,
        [0x01C] rw tzc_mmsys_misc: MmMiscTzcMmsysMisc,
        [0x020] rw peri_apb_ctrl: MmMiscPeriApbCtrl,
        [0x02C] rw mm_infra_qos_ctrl: MmMiscMmInfraQosCtrl,
        [0x040] rw dma_clk_ctrl: MmMiscDmaClkCtrl,
        [0x050] rw vram_ctrl: MmMiscVramCtrl,
        [0x060] rw sram_parm: MmMiscSramParm,
        [0x0A0] rw mm_int_sta0: MmMiscMmIntSta0,
        [0x0A4] rw mm_int_mask0: MmMiscMmIntMask0,
        [0x0A8] rw mm_int_clr_0: MmMiscMmIntClr0,
        [0x0AC] rw mm_int_sta1: MmMiscMmIntSta1,
        [0x0B0] rw mm_int_mask1: MmMiscMmIntMask1,
        [0x0B4] rw mm_int_clr_1: MmMiscMmIntClr1,
        [0x0F0] rw mmsys_debug_sel: MmMiscMmsysDebugSel,
        [0x0FC] rw mmsys_misc_dummy: MmMiscMmsysMiscDummy,
        [0x100] rw ddr_debug: MmMiscDdrDebug,
        [0x140] rw mm_berr_cfg0: MmMiscMmBerrCfg0,
        [0x144] rw mm_berr_cfg1: MmMiscMmBerrCfg1,
        [0x148] rw mm_berr_cfg2: MmMiscMmBerrCfg2,
        [0x14C] rw mm_berr_cfg3: MmMiscMmBerrCfg3,
        [0x150] rw mm_berr_cfg4: MmMiscMmBerrCfg4,
        [0x154] rw mm_berr_cfg5: MmMiscMmBerrCfg5,
        [0x158] rw mm_berr_cfg6: MmMiscMmBerrCfg6,
        [0x15C] rw mm_berr_cfg7: MmMiscMmBerrCfg7,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscCpu0Boot: u32 {
        [0..32] cpu0_rvba,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscCpuCfg: u32 {
        [0..13] cpu0_apb_base,
        [28..29] cpu0_ndm_rstn_en,
        [29..30] cpu0_hart_rstn_en,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscCpuSts1: u32 {
        [4..6] cpu0_lpmd_b,
        [16..24] cpu0_retire_pc_39_32,
        [24..25] cpu0_retire,
        [25..26] cpu0_pad_halted,
        [28..29] cpu0_ndm_rstn_req,
        [29..30] cpu0_hart_rstn_req,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscCpuSts2: u32 {
        [0..32] cpu0_retire_pc_31_0,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscCpuRtc: u32 {
        [0..10] c906_rtc_div,
        [30..31] c906_rtc_rst,
        [31..32] c906_rtc_en,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscTzcMmsysMisc: u32 {
        [0..1] tzc_mm_cpu0_lock,
        [2..3] tzc_mm_sram_lock,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscPeriApbCtrl: u32 {
        [0..1] mminfra_berr_int_en,
        [1..2] berr_int_en,
        [2..3] codec_berr_int_en,
        [3..4] mmcpu_berr_int_en,
        [8..9] mm_x2hs_sp_bypass,
        [16..32] rg_pclk_force_on,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmInfraQosCtrl: u32 {
        [2..3] mmcpu0_awqos,
        [3..4] mmcpu0_arqos,
        [16..18] h_wthre_mm2conn,
        [18..20] h_wthre_conn2mm,
        [20..22] x_wthre_mmhw2pa,
        [22..24] x_wthre_mmhw2ext,
        [24..26] x_wthre_puhs,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscDmaClkCtrl: u32 {
        [0..8] dma_clk_en,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscVramCtrl: u32 {
        [0..1] sysram_set,
        [1..3] h2pf_sram_rel,
        [4..5] vram_sram_rel,
        [6..7] sub_sram_rel,
        [7..8] blai_sram_rel,
        [8..11] h2pf_sram_sel,
        [12..13] vram_sram_sel,
        [14..15] sub_sram_sel,
        [15..16] blai_sram_sel,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscSramParm: u32 {
        [0..4] sram_cpu_ram_dvs,
        [4..5] sram_cpu_ram_dvse,
        [5..6] sram_cpu_ram_nap,
        [8..12] sram_l2ram_dvs,
        [12..13] sram_l2ram_dvse,
        [13..14] sram_l2ram_nap,
        [16..20] sram_cdc_ram_dvs,
        [20..21] sram_cdc_ram_dvse,
        [21..22] sram_cdc_ram_nap,
        [24..28] sram_sub_ram_dvs,
        [28..29] sram_sub_ram_dvse,
        [29..30] sram_sub_ram_nap,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmIntSta0: u32 {
        [0..32] mm_int_sta0,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmIntMask0: u32 {
        [0..32] mm_int_mask0,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmIntClr0: u32 {
        [0..32] mm_int_clr0,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmIntSta1: u32 {
        [0..32] mm_int_sta1,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmIntMask1: u32 {
        [0..32] mm_int_mask1,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmIntClr1: u32 {
        [0..32] mm_int_clr1,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmsysDebugSel: u32 {
        [0..4] mmsys_debug_sel,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmsysMiscDummy: u32 {
        [0..1] pir_ctrl_o,
        [1..2] light_sensor_ctrl_o,
        [2..3] ir_cut_ctrl_o,
        [3..4] dvp_sensor_pwdn,
        [4..32] dummy_reg,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscDdrDebug: u32 {
        [0..1] ddr_calib_done,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg0: u32 {
        [0..3] berr_en,
        [8..11] codec_berr_en,
        [16..17] mmcpu_berr_en,
        [24..29] mminfra_berr_en,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg1: u32 {
        [0..1] berr_clr,
        [1..2] codec_berr_clr,
        [2..3] mmcpu_berr_clr,
        [3..4] mminfra_berr_clr,
        [8..9] berr_last,
        [9..10] codec_berr_last,
        [10..11] mmcpu_berr_last,
        [11..12] mminfra_berr_last,
        [16..17] sts_berr,
        [17..18] sts_codec_berr,
        [18..19] sts_mmcpu_berr,
        [19..20] sts_mminfra_berr,
        [24..25] sts_berr_write,
        [25..26] sts_codec_berr_write,
        [26..27] sts_mmcpu_berr_write,
        [27..28] sts_mminfra_berr_write,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg2: u32 {
        [0..3] sts_berr_src,
        [8..12] sts_berr_id,
        [16..19] sts_codec_berr_src,
        [24..25] sts_codec_berr_id,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg3: u32 {
        [0..1] sts_mmcpu_berr_src,
        [8..12] sts_mmcpu_berr_id,
        [16..21] sts_mminfra_berr_src,
        [24..30] sts_mminfra_berr_id,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg4: u32 {
        [0..32] sts_berr_addr,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg5: u32 {
        [0..32] sts_codec_berr_addr,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg6: u32 {
        [0..32] sts_mmcpu_berr_addr,
    }
}

emhal::mmio_reg! {
    pub struct MmMiscMmBerrCfg7: u32 {
        [0..32] sts_mminfra_berr_addr,
    }
}
