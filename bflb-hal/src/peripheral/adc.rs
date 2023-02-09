//! Analog to Digital converter peripheral.

use crate::bl808::{AON, GPIP};
use crate::gpio::{Pin, Alternate, PinFunction, PinDrive, PinConfig};

use embedded_util::peripheral;


/// Exclusive access to ADC peripheral.
pub struct AdcAccess(());

impl AdcAccess {

    peripheral!(simple);

    /// Create a new single channel converter. This can be used later for polling
    /// the channel.
    pub fn into_single<C>(self, config: &AdcConfig, mut channel: C) -> AdcSingle
    where
        C: AdcChannelUntyped
    {
        todo!()
    }

    /// Create a new scan handle for an array of channels. This can be used later
    /// for polling these channels analogic values.
    pub fn into_scan<A>(self, config: &AdcConfig, mut array: A) -> AdcScan<A>
    where
        A: AdcChannelArray,
    {

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

        Self::start_conversion();
        // TODO: Wait 1 ms
        Self::stop_conversion();

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

        // TODO: Disable interrupts and clear status AGAIN.

        AON.gpadc_reg_isr().modify(|reg| {
            reg.gpadc_neg_satur().fill();
            reg.gpadc_pos_satur().fill();
        });

        let mut array_builder = ChannelArrayBuilder::default();
        array.apply(&mut array_builder);
        let array_len = array_builder.index as u8;
        array_builder.init();

        Self::start_conversion();

        AdcScan {
            array,
            array_len,
        }

    }

    /// Start Analog to Digital conversion.
    pub fn start_conversion() {
        Self::stop_conversion();
        // TODO: Wait 100 us
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_conv_start().fill());
    }

    /// Stop Analog to Digital conversion.
    pub fn stop_conversion() {
        // TODO: Rework this function to be defined somewhere else.
        AON.gpadc_reg_cmd().modify(|reg| reg.gpadc_conv_start().clear());
    }

}


/// Represent an ADC channel with a positive and negative reference voltage.
pub struct AdcChannel<P: AdcChannelRef, N: AdcChannelRef> {
    /// Positive channel kind.
    pos: P,
    /// Negative channel kind.
    neg: N,
    /// Raw value.
    raw_value: u32,
}

impl<P: AdcChannelRef, N: AdcChannelRef> AdcChannel<P, N> {

    /// Create a new channel with the given positive and negative reference voltage.
    pub fn new(pos: P, neg: N) -> Self {
        Self {
            pos: pos,
            neg: neg,
            raw_value: 0,
        }
    }

    /// Get the raw value of the channel. Default to 0 when the channel was just created.
    #[inline]
    pub fn raw_value(&self) -> u32 {
        self.raw_value
    }

}

impl<P: AdcChannelRef> AdcChannel<P, Ground> {
    
    /// Create a new channel with the given positive reference voltage and ground for 
    /// negative reference.
    pub fn with_ground(pos: P) -> Self {
        Self::new(pos, Ground)
    }

}


/// A trait to be implemented for possible channels references, such as GPIO pins.
pub trait AdcChannelRef {

    /// Configure the pin for being an ADC channel reference.
    fn configure(&mut self) -> u8;

}

/// Internal macro to implement [`AdcChannelRef`] on [`Pin`].
macro_rules! impl_adc_channel_ref_pin {
    ($channel:literal: $pin:literal) => {
        impl AdcChannelRef for Pin<$pin, Alternate> {
            #[inline]
            fn configure(&mut self) -> u8 {
                let mut cfg = PinConfig::default();
                cfg.set_function(PinFunction::Analog);
                cfg.set_drive(PinDrive::Drive0);
                cfg.set_input_enable(true);
                cfg.set_smt(true);
                self.set_config(cfg);
                $channel
            }
        }
    };
}

/// Implementation of [`AdcChannelRef`] on [`Pin`] for BL808.
#[cfg(any(feature = "bl808_m0", feature = "bl808_d0", feature = "bl808_lp"))]
mod impl_adc_channel_ref_pin {
    impl_adc_channel_ref_pin!(0:  17);
    impl_adc_channel_ref_pin!(3:  11);
    impl_adc_channel_ref_pin!(8:  16);
    impl_adc_channel_ref_pin!(9:  18);
    impl_adc_channel_ref_pin!(10: 19);
}

/// Internal macro to implement [`AdcChannelRef`] a newly defined 
/// ZST structures.
macro_rules! impl_adc_channel_ref_zst {
    ($channel:literal: $zst:ident) => {
        pub struct $zst;
        impl AdcChannelRef for $zst {
            #[inline]
            fn configure(&mut self) -> u8 {
                $channel
            }
        }
    };
}

impl_adc_channel_ref_zst!(12: DacA);
impl_adc_channel_ref_zst!(13: DacB);
impl_adc_channel_ref_zst!(14: TsenP);
impl_adc_channel_ref_zst!(15: TsenN);
impl_adc_channel_ref_zst!(16: Vref);
impl_adc_channel_ref_zst!(18: VbatHalf);
impl_adc_channel_ref_zst!(23: Ground);


/// A trait used internally to untype the [`AdcChannel`] structure
/// for adding it to [`ChannelArrayBuilder`].
pub trait AdcChannelUntyped {

    fn apply(&mut self, builder: &mut ChannelArrayBuilder);

    fn set_raw_value(&mut self, value: u32);

}

impl<P, N> AdcChannelUntyped for AdcChannel<P, N>
where
    P: AdcChannelRef,
    N: AdcChannelRef,
{

    fn apply(&mut self, builder: &mut ChannelArrayBuilder) {
        builder.add_channel(self.pos.configure(), self.neg.configure())
    }

    #[inline]
    fn set_raw_value(&mut self, raw_value: u32) {
        self.raw_value = raw_value;
    }
    

}


/// A trait implemented for arrays of ADC channels.
pub trait AdcChannelArray {
    
    /// Apply the channels to the given builder.
    fn apply(&mut self, builder: &mut ChannelArrayBuilder);

    fn set_raw_values(&mut self, values: &[u32]);

}

macro_rules! impl_channel_array_tuple {
    ($($index:tt: $ty_name:ident),+) => {
        impl<$($ty_name,)+> AdcChannelArray for ($($ty_name,)+)
        where
            $($ty_name: AdcChannelUntyped,)+
        {

            fn apply(&mut self, builder: &mut ChannelArrayBuilder) {
                $(self.$index.apply(&mut *builder);)+
            }

            fn set_raw_values(&mut self, values: &[u32]) {
                $(self.$index.set_raw_value(values[$index]);)+
            }

        }
    };
}

impl_channel_array_tuple!(0: I0);
impl_channel_array_tuple!(0: I0, 1: I1);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5, 6: I6);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5, 6: I6, 7: I7);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5, 6: I6, 7: I7, 8: I8);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5, 6: I6, 7: I7, 8: I8, 9: I9);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5, 6: I6, 7: I7, 8: I8, 9: I9, 10: I10);
impl_channel_array_tuple!(0: I0, 1: I1, 2: I2, 3: I3, 4: I4, 5: I5, 6: I6, 7: I7, 8: I8, 9: I9, 10: I10, 11: I11);


/// A builder for registering channels (up to 12 channels).
#[derive(Default)]
pub struct ChannelArrayBuilder {
    index: usize,
    scan_pos: u64,
    scan_neg: u64,
}

impl ChannelArrayBuilder {

    /// Add a channel with the given positive and negative reference voltages.
    pub fn add_channel(&mut self, pos: u8, neg: u8) {

        let offset = self.index * 5;
        self.scan_pos |= (pos as u64) << offset;
        self.scan_neg |= (neg as u64) << offset;

        self.index += 1;

    }

    fn init(self) {

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


/// A guard structure for keeping channel array owned until the scan is
/// completed.
pub struct AdcScan<A: AdcChannelArray> {
    /// The array of channels.
    array: A,
    /// The number of channels in the array.
    array_len: u8,
}

impl<A: AdcChannelArray> AdcScan<A> {

    /// Wait for scan to be completed and read all values to channels.
    pub fn poll(&mut self) -> &A {

        while GPIP.gpadc_config().get().gpadc_fifo_data_count().get() < self.array_len as u32 {
            // TODO: Wait
        }

        let mut values_buf = [0; 16];
        let values = &mut values_buf[..self.array_len as usize];

        for value in &mut values[..] {
            *value = GPIP.gpadc_dma_rdata().get().gpadc_dma_rdata().get();
        }

        self.array.set_raw_values(values);

        &self.array

    }

    pub fn finish(self) -> A {
        AdcAccess::stop_conversion();
        self.array
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

impl Default for AdcConfig {
    fn default() -> Self {
        Self { 
            clock_div: AdcClockDiv::Div32, 
            scan_conv_mode: true, 
            continuous_conv_mode: false, 
            differential_mode: false, 
            resolution: AdcResolution::Resolution16, 
            vref: AdcVref::V3p2 
        }
    }
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
