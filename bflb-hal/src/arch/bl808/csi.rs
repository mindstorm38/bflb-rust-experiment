//! MIPI CSI registers.

embedded_util::mmio! {
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
        [0x090] rw dphy_config_4: CsiDphyConfig4,
        [0x094] rw dphy_config_5: CsiDphyConfig5,
        [0x0FC] rw dummy_reg: CsiDummyReg,
    }
}

embedded_util::reg! {
    pub struct CsiMipiConfig: u32 {
        [00..01] cr_csi_en,
        [01..02] cr_lane_num,
        [03..04] cr_lane_inv,
        [04..05] cr_data_bit_inv,
        [05..06] cr_sync_sp_en,
        [06..07] cr_unpack_en,
        [12..14] cr_vc_dvp0,
        [14..16] cr_vc_dvp1,
    }
}

embedded_util::reg! {
    pub struct CsiIntStatus: u32 {
        [00..04] int_status,
    }
}

embedded_util::reg! {
    pub struct CsiIntMask: u32 {
        [00..04] int_mask,
    }
}

embedded_util::reg! {
    pub struct CsiIntClear: u32 {
        [00..04] int_clear,
    }
}

embedded_util::reg! {
    pub struct CsiIntEnable: u32 {
        [00..04] int_enable,
    }
}

embedded_util::reg! {
    pub struct CsiGnrBufStatus: u32 {
        [00..04] st_gnr_fifo_cnt,
    }
}

embedded_util::reg! {
    pub struct CsiGnrBufRdata: u32 {
        [00..32] gnr_buf_rdata,
    }
}

embedded_util::reg! {
    pub struct CsiDphyConfig0: u32 {
        /// Enable lane number 0.
        [00..01] dl0_enable,
        /// Enable lane number 1.
        [01..02] dl1_enable,
        /// Enable clock lane.
        [02..03] cl_enable,
        [04..05] dl0_stopstate,
        [05..06] dl1_stopstate,
        [06..07] cl_stopstate,
        [08..09] dl0_ulpsactivenot,
        [09..10] dl1_ulpsactivenot,
        [10..11] cl_ulpsactivenot,
        [12..13] dl0_forcerxmode,
        [13..14] dl1_forcerxmode,
        [14..15] cl_rxclkactivehs,
        [15..16] cl_rxulpsclknot,
        [31..32] reset_n,
    }
}

embedded_util::reg! {
    pub struct CsiDphyConfig1: u32 {
        /// Time interval during which the HS receiver shall ignore any clock lane HS transitions.
        [00..08] time_ck_settle,
        /// Time for the clock lane receiver to enable the HS line termination.
        [08..16] time_ck_term_en,
        /// Time interval during which the HS receiver shall ignore any data lane HS transitions.
        [16..24] time_hs_settle,
        [24..32] time_hs_term_en,
    }
}

embedded_util::reg! {
    pub struct CsiDphyConfig2: u32 {
        [00..01] ana_lprxen_clk,
        [01..02] ana_hsrxen_clk,
        [02..04] ana_hsrx_stop_state,
        [04..06] ana_hsrx_sync_en,
        [06..08] ana_lprxen,
        [08..10] ana_hsrxen,
        [10..15] ana_term_en,
        [15..16] ana_test_en,
        [16..20] pt_lock_counter,
        [20..21] pt_prbs_or_jitt,
        [21..22] pt_lp_mode,
        [22..23] pt_en,
    }
}

embedded_util::reg! {
    pub struct CsiDphyConfig3: u32 {
        [00..16] csi_ana_1,
        [16..32] csi_ana_0,
    }
}

embedded_util::reg! {
    pub struct CsiDphyConfig4: u32 {

    }
}

embedded_util::reg! {
    pub struct CsiDphyConfig5: u32 {
        
    }
}

embedded_util::reg! {
    pub struct CsiDummyReg: u32 {
        [00..32] dummy_reg,
    }
}
