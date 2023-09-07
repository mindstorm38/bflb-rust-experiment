//! # GPIO.
//! This module provides an abstract interface to configure and use
//! GPIOs of BouffaloLab chips.
//! 
//! ## Notes on BL808
//! The GPIO pins can be used for specific power domains, AON and PDS.
//! These domains are relevant in HBN and PDS power modes. 
//! In PDS 1/2/3 (not 7) the chip's MISC power domain is still powered
//! so the GPIO can be controller by GLB register. After GLB register
//! is powered off, the AON and PDS domains aquires control of their
//! respective pins
//! 
//! ### AON domain
//! The Always On domain is the domain of pins that remains usable up
//! to HBN1. Comprised of GPIO 9-15 and 40-41.
//! 
//! Note that pins 40/41 are :
//! - GPIO 40 is set by default to the input of XTAL32K.
//! - GPIO 41 is set by default to the output of XTAL32K.
//! 
//! ### PDS domain
//! The Power Down Sleep domain is split in 3 groups depending on
//! physical location of the pin: GPIO 0-8 (left), 16-23 (right) and 
//! 24-39 (top).

use core::marker::PhantomData;

use embedded_util::PtrRw;

use crate::arch::bl808::glb::GlbGpioCfg0;
use crate::arch::bl808::GLB;


/// An exclusive access to a GPIO pin on a particular port.
/// 
/// Available ports: 0 to 45 (included).
pub struct PinAccess<const NUM: u8>(pub(crate) ());

impl<const NUM: u8> PinAccess<NUM> {

    fn new_pin<M: Mode>() -> Pin<NUM, M> {
        Pin { _mode: PhantomData }
    }

    /// Get a new input pin from this port.
    pub fn into_input(self) -> Pin<NUM, Input> {
        let mut pin = Self::new_pin();
        pin.set_config(PinConfig::default());
        pin
    }

    /// Get a new output pin from this port.
    pub fn into_output(self) -> Pin<NUM, Output> {
        let mut pin = Self::new_pin();
        pin.set_config(PinConfig::default());
        pin
    }

    /// Get a new alternate pin from this port.
    pub fn into_alternate(self) -> Pin<NUM, Alternate> {
        let mut pin = Self::new_pin();
        pin.set_config(PinConfig::default());
        pin
    }

}

impl<const NUM: u8> From<PinAccess<NUM>> for Pin<NUM, Input> {
    fn from(pin: PinAccess<NUM>) -> Self {
        pin.into_input()
    }
}

impl<const NUM: u8> From<PinAccess<NUM>> for Pin<NUM, Output> {
    fn from(pin: PinAccess<NUM>) -> Self {
        pin.into_output()
    }
}

impl<const NUM: u8> From<PinAccess<NUM>> for Pin<NUM, Alternate> {
    fn from(pin: PinAccess<NUM>) -> Self {
        pin.into_alternate()
    }
}


/// A configured GPIO pin for a specific mode. This structure can
/// be obtained through the [`PinAccess`] peripheral structure.
pub struct Pin<const NUM: u8, M: Mode> {
    _mode: PhantomData<M>,
}

/// This trait is implemented by all valid modes and constrain the
/// generic type used in [`Pin`] structure.
pub trait Mode {}

/// This trait is implemented by [`Input`] and [`Alternate`] pin
/// modes. It's used for implementations of specific functions.
pub trait InputMode: Mode {}

/// This trait is implemented by [`Output`] and [`Alternate`] pin
/// modes. It's used for implementations of specific functions.
pub trait OutputMode: Mode {}

/// Input pin mode.
pub struct Input(());
impl Mode for Input {}
impl InputMode for Input {}

/// Input pin mode.
pub struct Output(());
impl Mode for Output {}
impl OutputMode for Output {}

/// Alternate function pin mode. This mode basically allow to do
/// anything on the pin because alternate function may need unusual
/// configuration sequences.
pub struct Alternate(());
impl Mode for Alternate {}
impl InputMode for Alternate {}
impl OutputMode for Alternate {}

impl<const NUM: u8, M: Mode> Pin<NUM, M> {
    
    /// Get back the port associated bit this pin.
    /// This can be used to free the peripheral.
    pub fn downgrade(self) -> PinAccess<NUM> {
        PinAccess(())
    }

    /// Internal function to get a read/write pointer to the 
    /// configuration register of this pin.
    #[inline]
    fn get_cfg(&self) -> PtrRw<GlbGpioCfg0> {
        let mut gpio_cfg = GLB.gpio_cfg0();
        gpio_cfg.0 = unsafe { gpio_cfg.0.add(NUM as usize) };
        gpio_cfg
    }

    /// Obtain a copy of the internal pin's configuration, one owned
    /// this configuration can be modified to be applied back to the
    /// pin with [`set_config`].
    /// 
    /// *Note: if you want to modify only one parameter, there are 
    /// regular functions on this `Pin` structure.*
    #[inline]
    pub fn config(&self) -> PinConfig<M> {
        PinConfig {
            raw: self.get_cfg().get(),
            _mode: PhantomData
        }
    }

    /// Set the configuration of the pin.
    #[inline]
    pub fn set_config(&mut self, config: PinConfig<M>) {
        self.get_cfg().set(config.raw);
    }

    /// A single method for modification of the pin configuration 
    /// through a closure. Basically [`config`], closure and then
    /// [`set_config`].
    #[inline]
    pub fn modify_config<F>(&mut self, func: F)
    where
        F: FnOnce(&mut PinConfig<M>),
    {
        let mut config = self.config();
        func(&mut config);
        self.set_config(config);
    }

    /// Get the current pull up/down/float mode for this pin.
    #[inline]
    pub fn pull(&self) -> PinPull {
        self.config().pull()
    }

    /// Set the pull up/down/float mode for this pin.
    #[inline]
    pub fn set_pull(&mut self, pull: PinPull) {
        self.modify_config(|cfg| cfg.set_pull(pull));
    }

}

impl<const NUM: u8, M: InputMode> Pin<NUM, M> {

    /// Return true of Shmitt trigger mode is enabled for this pin.
    #[inline]
    pub fn smt(&self) -> bool {
        self.config().smt()
    }

    /// Enable of disable Shmitt trigger mode for an input pin.
    #[inline]
    pub fn set_smt(&mut self, smt: bool) {
        self.modify_config(|cfg| cfg.set_smt(smt));
    }

}

impl<const NUM: u8, M: OutputMode> Pin<NUM, M> {

    /// Get the drive mode for an output pin.
    #[inline]
    pub fn drive(&self) -> PinDrive {
        self.config().drive()
    }

    /// Set the drive mode for an output pin.
    #[inline]
    pub fn set_drive(&mut self, drive: PinDrive) {
        self.modify_config(|cfg| cfg.set_drive(drive));
    }

}

impl<const NUM: u8> Pin<NUM, Output> {

    /// Set this output pin state to high.
    #[inline]
    pub fn set_high(&mut self) {
        
        self.get_cfg().modify(|reg| {
            reg.gpio_0_ie().clear();
            reg.gpio_0_oe().fill();
        });

        let reg = NUM / 32;
        let bit = NUM % 32;
        
        let mut cfg = GLB.gpio_cfg138();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };
        cfg.modify(|reg| {
            reg.0 |= 1 << bit;
        });

    }

    /// Set this output pin state to low.
    #[inline]
    pub fn set_low(&mut self) {
        
        self.get_cfg().modify(|reg| {
            reg.gpio_0_ie().clear();
            reg.gpio_0_oe().fill();
        });

        let reg = NUM / 32;
        let bit = NUM % 32;
        
        let mut cfg = GLB.gpio_cfg140();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };
        cfg.modify(|reg| {
            reg.0 |= 1 << bit;
        });

    }

    /// Set this output pin state to floating.
    #[inline]
    pub fn set_open(&mut self) {
        
        self.get_cfg().modify(|reg| {
            reg.gpio_0_ie().fill();
            reg.gpio_0_oe().clear();
        });

    }

    /// Set the boolean value of this output pin.
    pub fn set_value(&mut self, val: bool) {
        if val {
            self.set_high();
        } else {
            self.set_low();
        }
    }

}

impl<const NUM: u8> Pin<NUM, Alternate> {

    /// Get the function of this alternate pin.
    #[inline]
    pub fn function(&self) -> PinFunction {
        self.config().function()
    }

    /// Set the function for this alternate pin.
    #[inline]
    pub fn set_function(&mut self, func: PinFunction) {
        self.modify_config(|cfg| cfg.set_function(func));
    }

    /// Return true if this alternate pin has input enable.
    #[inline]
    pub fn input_enable(&self) -> bool {
        self.config().input_enable()
    }

    /// Set input enable of this alternate pin.
    #[inline]
    pub fn set_input_enable(&mut self, enable: bool) {
        self.modify_config(|cfg| cfg.set_input_enable(enable));
    }

    /// Return true if this alternate pin has output enable.
    #[inline]
    pub fn output_enable(&self) -> bool {
        self.config().output_enable()
    }

    /// Set output enable of this alternate pin.
    #[inline]
    pub fn set_output_enable(&mut self, enable: bool) {
        self.modify_config(|cfg| cfg.set_output_enable(enable));
    }

}


/// Represent the configuration of a pin at some point in time.
/// This can be used to modify multiple pin's parameters at once.
/// 
/// *Note that* this structure is small enough to implement [`Copy`],
/// this is why getters take `self` by value and setters by mutable
/// reference.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PinConfig<M: Mode> {
    raw: GlbGpioCfg0,
    _mode: PhantomData<M>,
}

impl Default for PinConfig<Input> {
    #[inline]
    fn default() -> Self {
        let mut raw = GlbGpioCfg0::default();
        raw.gpio_0_ie().fill();
        raw.gpio_0_func_sel().set(PinFunction::Digital as _);
        Self { raw, _mode: PhantomData }
    }
}

impl Default for PinConfig<Output> {
    #[inline]
    fn default() -> Self {
        let mut raw = GlbGpioCfg0::default();
        raw.gpio_0_oe().fill();
        raw.gpio_0_func_sel().set(PinFunction::Digital as _);
        raw.gpio_0_mode().set(1); // Toggle output mode.
        Self { raw, _mode: PhantomData }
    }
}

impl Default for PinConfig<Alternate> {
    #[inline]
    fn default() -> Self {
        Self { raw: GlbGpioCfg0::default(), _mode: PhantomData }
    }
}

impl<M: Mode> PinConfig<M> {

    /// Get the current pull up/down/float mode for this pin.
    pub fn pull(mut self) -> PinPull {
        match (self.raw.gpio_0_pu().get(), self.raw.gpio_0_pd().get()) {
            (1, 0) => PinPull::Up,
            (0, 1) => PinPull::Down,
            _ => PinPull::Float,
        }
    }

    /// Set the pull up/down/float mode for this pin.
    #[inline]
    pub fn set_pull(&mut self, pull: PinPull) {
        match pull {
            PinPull::Float => {
                self.raw.gpio_0_pu().clear();
                self.raw.gpio_0_pd().clear();
            }
            PinPull::Up => {
                self.raw.gpio_0_pu().fill();
                self.raw.gpio_0_pd().clear();
            }
            PinPull::Down => {
                self.raw.gpio_0_pd().fill();
                self.raw.gpio_0_pu().clear();
            }
        }
    }

}

impl<M: InputMode> PinConfig<M> {

    /// Return true of Shmitt trigger mode is enabled for this pin.
    #[inline]
    pub fn smt(mut self) -> bool {
        self.raw.gpio_0_smt().get() != 0
    }

    /// Enable of disable Shmitt trigger mode for an input pin.
    #[inline]
    pub fn set_smt(&mut self, smt: bool) {
        self.raw.gpio_0_smt().set(smt as _);
    }

}

impl<M: OutputMode> PinConfig<M> {

    /// Get the drive mode for an output pin.
    #[inline]
    pub fn drive(mut self) -> PinDrive {
        match self.raw.gpio_0_drv().get() {
            0 => PinDrive::Drive0,
            1 => PinDrive::Drive1,
            2 => PinDrive::Drive2,
            3 => PinDrive::Drive3,
            _ => unreachable!("should be unreachable and optimized-out")
        }
    }

    /// Set the drive mode for an output pin.
    #[inline]
    pub fn set_drive(&mut self, drive: PinDrive) {
        self.raw.gpio_0_drv().set(drive as _);
    }

}

impl PinConfig<Alternate> {

    /// Get the function of this alternate pin.
    #[inline]
    pub fn function(mut self) -> PinFunction {
        PinFunction::from_number(self.raw.gpio_0_func_sel().get() as _).unwrap()
    }

    /// Set the function for this alternate pin.
    #[inline]
    pub fn set_function(&mut self, func: PinFunction) {
        self.raw.gpio_0_func_sel().set(func as _);
    }

    /// Return true if this alternate pin has input enable.
    #[inline]
    pub fn input_enable(mut self) -> bool {
        self.raw.gpio_0_ie().get() != 0
    }

    /// Set input enable of this alternate pin.
    #[inline]
    pub fn set_input_enable(&mut self, enable: bool) {
        self.raw.gpio_0_ie().set(enable as _);
    }

    /// Return true if this alternate pin has output enable.
    #[inline]
    pub fn output_enable(mut self) -> bool {
        self.raw.gpio_0_oe().get() != 0
    }

    /// Set output enable of this alternate pin.
    #[inline]
    pub fn set_output_enable(&mut self, enable: bool) {
        self.raw.gpio_0_oe().set(enable as _);
    }

}


/// Pull mode for a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinPull {
    /// No pull-[up,down].
    Float,
    /// Pull-up.
    Up,
    /// Pull-down.
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinDrive {
    Drive0 = 0,
    Drive1 = 1,
    Drive2 = 2,
    Drive3 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinFunction {
    Sdh      = 0,
    Spi0     = 1,
    I2s      = 3,
    Pdm      = 4,
    I2c0     = 5,
    I2c1     = 6,
    Uart     = 7,
    Emac     = 8,
    Cam      = 9,
    Analog   = 10,
    Digital  = 11,
    Sdu      = 12,
    Pwm0     = 16,
    Pwm1     = 17,
    Spi1     = 18,
    I2c2     = 19,
    I2c3     = 20,
    DbiB     = 22,
    DbiC     = 23,
    JtagLP   = 25,
    JtagM0   = 26,
    JtagD0   = 27,
    ClockOut = 31,
}

impl PinFunction {

    pub fn from_number(num: u8) -> Option<Self> {
        Some(match num {
            0 => Self::Sdh,
            1 => Self::Spi0,
            3 => Self::I2s,
            4 => Self::Pdm,
            5 => Self::I2c0,
            6 => Self::I2c1,
            7 => Self::Uart,
            8 => Self::Emac,
            9 => Self::Cam,
            10 => Self::Analog,
            11 => Self::Digital,
            12 => Self::Sdu,
            16 => Self::Pwm0,
            17 => Self::Pwm1,
            18 => Self::Spi1,
            19 => Self::I2c2,
            20 => Self::I2c3,
            22 => Self::DbiB,
            23 => Self::DbiC,
            25 => Self::JtagLP,
            26 => Self::JtagM0,
            27 => Self::JtagD0,
            31 => Self::ClockOut,
            _ => return None,
        })
    }

}
