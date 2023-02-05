//! Analog to Digital converter peripheral.

use crate::bl808::{AON, GPIP};
use crate::gpio::PinAccess;

use embedded_util::peripheral;


/// Exclusive access to ADC peripheral.
pub struct AdcAccess(());

impl AdcAccess {

    peripheral!(simple);

    /// Configure ADC for scanning mode and return a builder for configuring
    /// each channel and start scanning.
    pub fn into_scan(self, config: &AdcConfig) -> AdcScan {

        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_global_en().clear());
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_global_en().fill());

        // Reset ADC
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_soft_rst().fill());
        // TODO: Wait 8 NOP
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_soft_rst().clear());

        // Disable interrupts and clear status.
        GPIP.gpadc_config().modify(|reg| {

            reg.gpadc_fifo_underrun_mask().fill();
            reg.gpadc_fifo_overrun_mask().fill();
            reg.gpadc_rdy_mask().fill();

            reg.gpadc_fifo_underrun_clr().fill();
            reg.gpadc_fifo_overrun_clr().fill();
            reg.gpadc_rdy_clr().fill();

            reg.gpadc_fifo_clr().fill();
            reg.gpadc_fifo_thl().clear();
            reg.gpadc_dma_en().clear();

        });

        self.start_conversion();
        // TODO: Wait 1 ms
        self.stop_conversion();

        AON.gpadc_reg_config1().set_with(|reg| {

            reg.gpadc_v18_sel().set(2); // V18 select: 1.82 V
            reg.gpadc_v11_sel().set(1); // V18 select: 1.10 V

            reg.gpadc_clk_div_ratio().set(config.clock_div as _);
            reg.gpadc_res_sel().set(config.resolution as _);

            if config.scan_conv_mode {
                reg.gpadc_scan_en().fill();
                reg.gpadc_clk_ana_inv().fill();
            }

            if config.continuous_conv_mode {
                reg.gpadc_cont_conv_en().fill();
            }

        });

        // TODO: Wait 8 NOP

        AON.gpadc_reg_config2().set_with(|reg| {

            reg.gpadc_dly_sel().set(2);
            reg.gpadc_chop_mode().set(2); // Vref AZ and PGA chop on
            reg.gpadc_pga1_gain().set(1); // Gain 1
            reg.gpadc_pga2_gain().set(1); // Gain 1
            reg.gpadc_pga_en().fill();
            reg.gpadc_pga_os_cal().set(8);
            reg.gpadc_pga_vcm().set(1);   // PGA output common mode control 1.4 V
            reg.gpadc_vref_sel().set(config.vref as _);
            reg.gpadc_diff_mode().set(config.differential_mode as _);

        });

        AON.gpadc_reg_cmd().modify(|reg| {
            reg.gpadc_mic2_diff().fill(); // mic2 diff enable
            reg.gpadc_neg_gnd().set(config.differential_mode as _);
        });

        // Calibration offset.
        AON.gpadc_reg_define().modify(|reg| {
            reg.gpadc_os_cal_data().clear();
        });

        // TODO: Disable interrupts and clear status.

        AON.gpadc_reg_isr().modify(|reg| {
            reg.gpadc_neg_satur().fill();
            reg.gpadc_pos_satur().fill();
        });

        AdcScan {
            index: 0,
            scan_pos: 0,
            scan_neg: 0,
        }

    }

    /// Start Analog to Digital conversion.
    pub fn start_conversion(&mut self) {
        self.stop_conversion();
        // TODO: Wait 100 us
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_conv_start().fill());
    }

    /// Stop Analog to Digital conversion.
    pub fn stop_conversion(&mut self) {
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_conv_start().clear());
    }

}


/// Represent an ADC channel with a positive and negative reference voltages.
pub struct AdcChannel<P, N> {
    /// Positive channel kind.
    pos: P,
    /// Negative channel kind.
    neg: N,
    /// Raw value.
    value: u32,
}

impl<P, N> AdcChannel<P, N>
where
    P: AdcChannelRef,
    N: AdcChannelRef,
{

    /// Create a new channel with the given positive and negative reference voltage.
    pub fn new(pos: P, neg: N) -> Self {
        Self {
            pos,
            neg,
            value: 0,
        }
    }

}

impl<P> AdcChannel<P, AdcGnd>
where 
    P: AdcChannelRef,
{

    /// Create a new channel with the given positive reference voltage and ground for 
    /// negative reference.
    pub fn with_gnd(pos: P) -> Self {
        Self::new(pos, AdcGnd)
    }

}


/// A trait to be implemented for possible channels references, such as GPIO pins.
pub trait AdcChannelRef {

    /// Configure the pin for being an ADC channel reference and then return the
    /// numeric ID of the reference.
    fn configure(&mut self) -> u8;

}

pub struct AdcGnd;
impl AdcChannelRef for AdcGnd {
    #[inline]
    fn configure(&mut self) -> u8 { 23 }
}

impl AdcChannelRef for PinAccess<17> {
    #[inline]
    fn configure(&mut self) -> u8 {
        0
    }
}

impl AdcChannelRef for PinAccess<5> {
    #[inline]
    fn configure(&mut self) -> u8 { 1 }
}


/// A trait implemented for arrays of ADC channels.
pub trait AdcChannelArray {
    
    /// Apply the channels to the given builder.
    fn apply(&self, builder: &mut AdcBuilder);

}



/// A builder for registering channels (up to 12 channels).
/// This structure is given to 
pub struct AdcBuilder {
    index: usize,
    scan_pos: u64,
    scan_neg: u64,
}

impl AdcBuilder {

    /// Add a channel with the given positive and negative reference voltages.
    pub fn add_channel(&mut self, pos: AdcChannelKind, neg: AdcChannelKind) -> AdcChannel {

        let offset = self.index * 5;
        self.scan_pos |= (pos as u64) << offset;
        self.scan_neg |= (neg as u64) << offset;

        self.index += 1;
        self

    }

    pub fn init(self) {

        AON.gpadc_reg_scn_pos1().set_with(|reg| {
            reg.0 = self.scan_pos as u32
        });

        AON.gpadc_reg_scn_pos2().set_with(|reg| {
            reg.0 = (self.scan_pos >> 32) as u32
        });

        AON.gpadc_reg_scn_neg1().set_with(|reg| {
            reg.0 = self.scan_neg as u32;
        });

        AON.gpadc_reg_scn_neg2().set_with(|reg| {
            reg.0 = (self.scan_neg >> 32) as u32
        });

    }

}


#[derive(Debug, Clone)]
pub struct AdcConfig {
    pub clock_div: AdcClockDiv,
    pub scan_conv_mode: bool,
    pub continuous_conv_mode: bool,
    pub differential_mode: bool,
    pub resolution: AdcResolution,
    pub vref: AdcVref,
}

/// Clock division definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcClockDiv {
    Div4 = 1,
    Div8 = 2,
    Div12 = 3,
    Div16 = 4,
    Div20 = 5,
    Div24 = 6,
    Div32 = 7,
}

/// ADC resolution definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcResolution {
    Resolution12 = 0,
    Resolution14 = 2,
    Resolution16 = 4,
}

/// ADC voltage reference definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcVref {
    V3p2 = 0,
    V2p0 = 1,
}

// /// All kind of ADC channels.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// pub enum AdcChannelKind {
//     /// GPIO pin 17.
//     Channel0 = 0,
//     /// GPIO pin 5.
//     Channel1 = 1,
//     /// GPIO pin 4.
//     Channel2 = 2,
//     /// GPIO pin 11.
//     Channel3 = 3,
//     /// GPIO pin 6.
//     Channel4 = 4,
//     /// GPIO pin 40.
//     Channel5 = 5,
//     /// GPIO pin 12.
//     Channel6 = 6,
//     /// GPIO pin 13.
//     Channel7 = 7,
//     /// GPIO pin 16.
//     Channel8 = 8,
//     /// GPIO pin 18.
//     Channel9 = 9,
//     /// GPIO pin 19.
//     Channel10 = 10,
//     /// GPIO pin 34.
//     Channel11 = 11,
//     DacA = 12,
//     DacB = 13,
//     TsenP = 14,
//     TsenN = 15,
//     Vref = 16,
//     VabtHalf = 18,
//     Gnd = 23,
// }
