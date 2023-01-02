//! MIPI CSI registers.

emhal::mmio_struct! {
    pub struct Csi {
        [0x000] rw mipi_config: CsiMipiConfig,
        [0x010] rw int_status: CsiIntStatus,
        [0x014] rw int_mask: CsiIntMask,
        [0x018] rw int_clear: CsiIntClear,
        [0x01C] rw int_enable: CsiIntEnable,
        [0x020] rw gnr_buf_status: CsiGnrBufStatus,
        [0x024] rw gnr_buf_rdata: CsiGnrBufRdata,
        [0x080] rw dphy_config_0: CsiDphyConfig0,
        [0x084] rw dphy_config_1: CsiDphyConfig1,
        [0x088] rw dphy_config_2: CsiDphyConfig2,
        [0x08C] rw dphy_config_3: CsiDphyConfig3,
        [0x0FC] rw dummy_reg: CsiDummyReg,
    }
}

emhal::mmio_reg! {
    pub struct CsiMipiConfig: u32 {
        [0..1] cr_csi_en,
        [1..2] cr_lane_num,
        [3..4] cr_lane_inv,
        [4..5] cr_data_bit_inv,
        [5..6] cr_sync_sp_en,
        [6..7] cr_unpack_en,
        [12..14] cr_vc_dvp0,
        [14..16] cr_vc_dvp1,
    }
}

emhal::mmio_reg! {
    pub struct CsiIntStatus: u32 {
        [0..4] int_status,
    }
}

emhal::mmio_reg! {
    pub struct CsiIntMask: u32 {
        [0..4] int_mask,
    }
}

emhal::mmio_reg! {
    pub struct CsiIntClear: u32 {
        [0..4] int_clear,
    }
}

emhal::mmio_reg! {
    pub struct CsiIntEnable: u32 {
        [0..4] int_enable,
    }
}

emhal::mmio_reg! {
    pub struct CsiGnrBufStatus: u32 {
        [0..4] st_gnr_fifo_cnt,
    }
}

emhal::mmio_reg! {
    pub struct CsiGnrBufRdata: u32 {
        [0..32] gnr_buf_rdata,
    }
}

emhal::mmio_reg! {
    pub struct CsiDphyConfig0: u32 {
        [0..1] dl0_enable,
        [1..2] dl1_enable,
        [2..3] cl_enable,
        [4..5] dl0_stopstate,
        [5..6] dl1_stopstate,
        [6..7] cl_stopstate,
        [8..9] dl0_ulpsactivenot,
        [9..10] dl1_ulpsactivenot,
        [10..11] cl_ulpsactivenot,
        [12..13] dl0_forcerxmode,
        [13..14] dl1_forcerxmode,
        [14..15] cl_rxclkactivehs,
        [15..16] cl_rxulpsclknot,
        [31..32] reset_n,
    }
}

emhal::mmio_reg! {
    pub struct CsiDphyConfig1: u32 {
        [0..8] time_ck_settle,
        [8..16] time_ck_term_en,
        [16..24] time_hs_settle,
        [24..32] time_hs_term_en,
    }
}

emhal::mmio_reg! {
    pub struct CsiDphyConfig2: u32 {
        [0..1] ana_lprxen_clk,
        [1..2] ana_hsrxen_clk,
        [2..4] ana_hsrx_stop_state,
        [4..6] ana_hsrx_sync_en,
        [6..8] ana_lprxen,
        [8..10] ana_hsrxen,
        [10..15] ana_term_en,
        [15..16] ana_test_en,
        [16..20] pt_lock_counter,
        [20..21] pt_prbs_or_jitt,
        [21..22] pt_lp_mode,
        [22..23] pt_en,
    }
}

emhal::mmio_reg! {
    pub struct CsiDphyConfig3: u32 {
        [0..16] csi_ana_1,
        [16..32] csi_ana_0,
    }
}

emhal::mmio_reg! {
    pub struct CsiDummyReg: u32 {
        [0..32] dummy_reg,
    }
}
