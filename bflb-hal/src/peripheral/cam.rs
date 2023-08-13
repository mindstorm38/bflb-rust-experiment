//! Camera abstraction interface.

use crate::bl808::{CSI, DVP_TSRC0, DVP_TSRC1, DSP2_MISC};
use crate::bl808::dtsrc::Dtsrc;


/// DVP TSRC controller.
pub struct DvpTsrc<const PORT: u8>(pub(crate) ());

impl<const PORT: u8> DvpTsrc<PORT> {

    #[inline]
    fn get_registers() -> Dtsrc {
        match PORT {
            0 => DVP_TSRC0,
            1 => DVP_TSRC1,
            _ => unreachable!()
        }
    }
    
    #[inline]
    pub fn enable(&mut self) {
        Self::get_registers().config().modify(|reg| reg.cr_enable().fill());
    }

    #[inline]
    pub fn disable(&mut self) {
        Self::get_registers().config().modify(|reg| reg.cr_enable().clear());
    }

    /// Configure the sensor input by setting the FIFO threshold for 
    /// each DVP line to start to output. You can optionally invert
    /// the signals of sensor hsync and vsync.
    pub fn set_sensor_input(&mut self, fifo_threshold: u16, hsync_inv: bool, vsync_inv: bool) {

        let registers = Self::get_registers();
        
        registers.config().modify(|reg| {
            reg.cr_snsr_en().fill();
            reg.cr_snsr_hsync_inv().set(hsync_inv as _);
            reg.cr_snsr_vsync_inv().set(vsync_inv as _);
        });

        registers.snsr2dvp_wait_pos().modify(|reg| {
            reg.cr_snsr_fifo_th().set(fifo_threshold as _);
        });

    }

}


/// DSP2.
pub struct Dsp2(pub(crate) ());

impl Dsp2 {

    /// Select DVP TSRC data source.
    pub fn set_dvp_tsrc_source(&mut self, source: DvpTsrcSource) {
        DSP2_MISC.pix_data_ctrl().modify(|reg| {
            reg.dsp2_dtsrc_src().set(source as _);
        });
    }

    /// Apply configuration to the YUV->RGB converter.
    pub fn set_yuv2rgb_config(&mut self, config: &Yuv2RgbParam) {

        DSP2_MISC.y2ra_config_0().modify(|reg| {
            reg.rg_y2ra_pre_0().set(config.pre_offset_0 as _);
            reg.rg_y2ra_pos_0().set(config.post_offset_0 as _);
        });

        DSP2_MISC.y2ra_config_1().modify(|reg| {
            reg.rg_y2ra_pre_1().set(config.pre_offset_1 as _);
            reg.rg_y2ra_pos_1().set(config.post_offset_1 as _);
        });

        DSP2_MISC.y2ra_config_2().modify(|reg| {
            reg.rg_y2ra_pre_2().set(config.pre_offset_2 as _);
            reg.rg_y2ra_pos_2().set(config.post_offset_2 as _);
        });

        DSP2_MISC.y2ra_config_3().modify(|reg| {
            reg.rg_y2ra_mtx_00().set(config.matrix_00 as _);
            reg.rg_y2ra_mtx_01().set(config.matrix_01 as _);
        });

        DSP2_MISC.y2ra_config_4().modify(|reg| {
            reg.rg_y2ra_mtx_02().set(config.matrix_02 as _);
            reg.rg_y2ra_mtx_10().set(config.matrix_10 as _);
        });

        DSP2_MISC.y2ra_config_5().modify(|reg| {
            reg.rg_y2ra_mtx_11().set(config.matrix_11 as _);
            reg.rg_y2ra_mtx_12().set(config.matrix_12 as _);
        });

        DSP2_MISC.y2ra_config_6().modify(|reg| {
            reg.rg_y2ra_mtx_20().set(config.matrix_20 as _);
            reg.rg_y2ra_mtx_21().set(config.matrix_21 as _);
        });

        DSP2_MISC.y2ra_config_7().modify(|reg| {
            reg.rg_y2ra_mtx_22().set(config.matrix_22 as _);
        });

    }

    #[inline]
    pub fn set_yuv2rgb_input(&mut self, input: Option<Yuv2rgbInput>) {
        DSP2_MISC.y2ra_config_0().modify(|reg| {
            reg.rg_y2ra_sel().set(input.map(|i| i as u32).unwrap_or(0));
            reg.rg_y2ra_en().set(input.is_some() as _);
        });
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DvpTsrcSource {
    Dvp = 0,
    Csi = 1,
}

#[derive(Debug, Clone)]
pub struct Yuv2RgbParam {
    pub pre_offset_0: i32,
    pub pre_offset_1: i32,
    pub pre_offset_2: i32,
    pub post_offset_0: i32,
    pub post_offset_1: i32,
    pub post_offset_2: i32,
    pub matrix_00: i32,
    pub matrix_01: i32,
    pub matrix_02: i32,
    pub matrix_10: i32,
    pub matrix_11: i32,
    pub matrix_12: i32,
    pub matrix_20: i32,
    pub matrix_21: i32,
    pub matrix_22: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yuv2rgbInput {
    Dsp2Output = 0,
    Dsp2Input = 1,
    OsdA = 2,
    OsdB = 3,
    OsdC = 4,
    OsdD = 5,
    ScalerA = 6,
    ScalerB = 7,
    ScalerC = 8,
    ScalerD = 9,
    Dvp2x = 10,
}


/// CSI controller to use for cameras.
pub struct Csi(pub(crate) ());

impl Csi {

    /// Enable CSI module.
    pub fn enable(&mut self) {
        CSI.mipi_config().modify(|reg| reg.cr_csi_en().fill());
    }

    /// Disable CSI module.
    pub fn disable(&mut self) {
        CSI.mipi_config().modify(|reg| reg.cr_csi_en().clear());
    }

    /// Configure the MIPI CSI module.
    /// 
    /// *Note that* this method doesn't disable CSI and D-PHY prior to 
    /// configuration, so you need to make sure that no weird behavior
    /// happens.
    pub fn config(&mut self, config: &CsiConfig) {

        CSI.mipi_config().modify(|reg| {

            reg.cr_vc_dvp0().set(config.dvp0_virtual_channel);
            // reg.cr_vc_dvp1().set(1);

            reg.cr_unpack_en().set(config.unpack as _);
            reg.cr_sync_sp_en().set(config.sync_short_packet as _);
            reg.cr_data_bit_inv().set(config.data_bit_inverted as _);
            reg.cr_lane_inv().set(config.data_lane_inverted as _);

            reg.cr_lane_num().set(match config.lane_count {
                LaneCount::Rx1Lane => 0,
                LaneCount::Rx2Lane => 1,
            });

        });

    }

    /// Configure the CSI D-PHY physical layer.
    pub fn phy_config(&mut self, config: &CsiPhyConfig) {

        let td_term_en_max = 35 + 4000 / config.data_rate;
        let td_term_en = (td_term_en_max * config.data_rate / 2000) - 1;
        let ths_settle_max = 145 + 10000 / config.data_rate;
        let ths_settle = ((ths_settle_max - td_term_en * 2000 / config.data_rate) * config.data_rate / 2000) - 1;
        let tclk_term_en_max = 38;
        let tclk_term_en = (config.tx_clock_escape * tclk_term_en_max) / 1000;
        let tclk_settle_max = 300;
        let tclk_settle = ((tclk_settle_max - tclk_term_en * 1000 / config.tx_clock_escape) * config.tx_clock_escape / 1000) - 1;

        CSI.dphy_config_1().modify(|reg| {
            reg.time_ck_settle().set(tclk_settle);
            reg.time_ck_term_en().set(tclk_term_en);
            reg.time_hs_settle().set(ths_settle);
            reg.time_hs_term_en().set(td_term_en);
        });

        CSI.dphy_config_2().modify(|reg| {
            reg.ana_term_en().set(0x8);
        });

    }

    /// Enable the CSI D-PHY physical layer.
    pub fn phy_enable(&mut self, lane_count: LaneCount) {

        CSI.dphy_config_0().modify(|reg| {
            reg.cl_enable().fill();
            reg.dl0_enable().fill();
            reg.dl0_forcerxmode().fill();
            if let LaneCount::Rx2Lane = lane_count {
                reg.dl1_enable().fill();
                reg.dl1_forcerxmode().fill();
            }
        });
        
    }

    /// Disable CSI D-PHY physical layer.
    pub fn phy_disable(&mut self) {
        CSI.dphy_config_0().modify(|reg| {
            reg.cl_enable().clear();
            reg.dl0_enable().clear();
            reg.dl0_forcerxmode().clear();
            reg.dl1_enable().clear();
            reg.dl1_forcerxmode().clear();
        });
    }

    pub fn phy_reset(&mut self) {
        CSI.dphy_config_0().modify(|reg| reg.reset_n().clear());
        CSI.dphy_config_0().modify(|reg| reg.reset_n().fill());
    }

}


/// Configuration structure for CSI module.
#[derive(Debug, Clone)]
pub struct CsiConfig {
    /// Number of lanes to use for CSI.
    pub lane_count: LaneCount,
    /// Virtual channel number for DVP0.
    pub dvp0_virtual_channel: u32,
    /// Enable CSI unpacking or not:
    /// - 0 - DVP output is 8-bit valid.
    /// - 1 - DVP output format depends on packet data type (RAW 8/10/12/14).
    pub unpack: bool,
    /// Enable or disable sync short packets(FS/FE/LS/LE) to be received into generic packet buffer.
    pub sync_short_packet: bool,
    /// Enable or disable PPI I/F data byte bit inverse, which should be set to little-endian.
    pub data_bit_inverted: bool,
    /// Enable or disable lane 0 and lane 1 inverse.
    pub data_lane_inverted: bool,
}


/// Configuration structure for CSI physical layer.
#[derive(Debug, Clone)]
pub struct CsiPhyConfig {
    /// Frequency of the TX clock escape (in MHz).
    pub tx_clock_escape: u32,
    /// Frequency of the data (in MHz).
    pub data_rate: u32,
}


/// Restricted number of CSI lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneCount {
    Rx1Lane,
    Rx2Lane,
}




// TODO: Just for prototyping
fn cam_mipi_csi_init() {

    use crate::power::{Power, Vdd15cis};

    const DSP2_USE_CLOCK: u32   = 80000000;
    const PIX_USE_CLOCK: u32    = 66000000;
    const MIPI_WIDTH: u16       = 1920;
    const MIPI_HEIGHT: u16      = 1028;

    let mut power = Power(());
    let mut dsp2 = Dsp2(());
    let mut dtsrc = DvpTsrc::<0>(());
    let mut csi = Csi(());

    // HBN_Aon_Pad_Cfg(DISABLE, HBN_AON_PAD_GPIO40, &aonPadCfg);
    // HBN_Aon_Pad_Cfg(DISABLE, HBN_AON_PAD_GPIO41, &aonPadCfg);
    dtsrc.disable();

    power.set_vdd15_cis(Vdd15cis::Volt1p20);
    // GLB_CSI_Config_MIPIPLL(2, 0x21000);
    // GLB_CSI_Power_Up_MIPIPLL();
    // GLB_Set_DSP_DSP2_CLK(ENABLE, GLB_DSP_DSP2_CLK_MUXPLL_160M, 1);
    let fifo_threshold = (DSP2_USE_CLOCK - PIX_USE_CLOCK) / 1000 * MIPI_WIDTH as u32 / (DSP2_USE_CLOCK / 1000) + 10;
    dtsrc.set_sensor_input(fifo_threshold as u16, false, false);
    dsp2.set_dvp_tsrc_source(DvpTsrcSource::Csi);

    csi.disable();
    csi.phy_disable();
    csi.phy_reset();

    csi.config(&CsiConfig { 
        lane_count: LaneCount::Rx2Lane, 
        dvp0_virtual_channel: 0, 
        unpack: true, 
        sync_short_packet: true, 
        data_bit_inverted: false, 
        data_lane_inverted: false,
    });

    csi.phy_config(&CsiPhyConfig {
        tx_clock_escape: 24,
        data_rate: 520,
    });

    csi.enable();
    csi.phy_enable(LaneCount::Rx2Lane);

    dtsrc.enable();

    // YUV2RGB INIT
    dsp2.set_yuv2rgb_config(&Yuv2RgbParam {
        pre_offset_0: 0,
        pre_offset_1: -128,
        pre_offset_2: -128,
        post_offset_0: 0,
        post_offset_1: 0,
        post_offset_2: 0,
        matrix_00: 512,
        matrix_01: 0,
        matrix_02: 718,
        matrix_10: 512,
        matrix_11: -176,
        matrix_12: -366,
        matrix_20: 512,
        matrix_21: 907,
        matrix_22: 0,
    });
    dsp2.set_yuv2rgb_input(Some(Yuv2rgbInput::Dsp2Input));
    // ------------

    // SENSOR INIT
    // TODO: sipeed_board_pinmux_init
    // TODO: SIPEED_SCCB_Init
    // -----------

}