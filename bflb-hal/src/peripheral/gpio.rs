//! GPIO management on BL808.

use core::marker::PhantomData;

use embedded_util::peripheral;
use emhal::mmio::PtrRw;

use crate::register::GLB;
use crate::register::glb::GlbGpioCfg0;


/// Represent an unconfigured GPIO pin peripheral
pub struct PinPort<const NUM: u8> {}
peripheral!(PinPort<NUM>, NUM: u8[0..46]);

impl<const NUM: u8> PinPort<NUM> {

    /// Generic types erasing, this transfer checks to the runtime.
    pub fn erase(self) -> ! {
        todo!()
    }

    fn new_pin<M: Mode>() -> Pin<NUM, M> {
        Pin { _mode: PhantomData }
    }

    /// Get a new input pin from this port.
    pub fn into_input(self) -> Pin<NUM, Input> {
        let pin = Self::new_pin();
        pin.get_cfg().modify(|reg| {
            reg.gpio_0_ie().fill();
            reg.gpio_0_func_sel().set(11);
        });
        pin
    }

    /// Get a new output pin from this port.
    pub fn into_output(self) -> Pin<NUM, Output> {
        let pin = Self::new_pin();
        pin.get_cfg().modify(|reg| {
            reg.gpio_0_oe().fill();
            reg.gpio_0_func_sel().set(11);
            reg.gpio_0_mode().set(1); // Toggle output mode.
            reg.gpio_0_drv().set(0);
        });
        pin
    }

    /// Get a new alternate pin from this port.
    pub fn into_alternate<F: Function>(self) -> Pin<NUM, Alternate<F>> {
        let pin = Self::new_pin();
        pin.get_cfg().modify(|reg| {
            reg.gpio_0_func_sel().set(F::id());
        });
        pin
    }

}


/// A configured GPIO pin for a specific mode. This structure can
/// be obtained through the [`PinPort`] peripheral structure.
pub struct Pin<const NUM: u8, M: Mode> {
    _mode: PhantomData<M>,
}

/// This trait is implemented by all valid modes and constrain the
/// generic type used in [`Pin`] structure.
pub trait Mode {}

/// This trait is implemented by [`Input`] and [`Alternate`] pin
/// modes. It's used for implementations of specific functions.
pub trait InputLike: Mode {}

/// This trait is implemented by [`Output`] and [`Alternate`] pin
/// modes. It's used for implementations of specific functions.
pub trait OutputLike: Mode {}

/// Input pin mode.
pub struct Input {}
impl Mode for Input {}
impl InputLike for Input {}

/// Input pin mode.
pub struct Output {}
impl Mode for Output {}
impl OutputLike for Output {}

/// Alternate function pin mode.
pub struct Alternate<F: Function> {
    _func: PhantomData<F>,
}
impl<F: Function> Mode for Alternate<F> {}
impl<F: Function> InputLike for Alternate<F> {}
impl<F: Function> OutputLike for Alternate<F> {}

/// Alternate function trait.
pub trait Function {
    fn id() -> u32;
}

/// Internal macro used to implement all structures that can
/// be used to define alternate functions.
macro_rules! functions {
    ($($func:ident = $val:literal),+ $(,)?) => {
        $(
        pub struct $func {}
        impl Function for $func {
            #[inline]
            fn id() -> u32 { $val }
        }
        )+
    };
}

functions! {
    Sdh         = 0,
    Spi0        = 1,
    I2s         = 3,
    Pdm         = 4,
    I2c0        = 5,
    I2c1        = 6,
    Uart        = 7,
    Emac        = 8,
    Cam         = 9,
    Analog      = 10,
    Digital     = 11,
    Sdu         = 12,
    Pwm0        = 16,
    Pwm1        = 17,
    Spi1        = 18,
    I2c2        = 19,
    I2c3        = 20,
    DbiB        = 22,
    DbiC        = 23,
    JtagLP      = 25,
    JtagM0      = 26,
    JtagD0      = 27,
    ClockOut    = 31,
    Unused      = 0xFF,
}


impl<const NUM: u8, M: Mode> Pin<NUM, M> {
    
    /// Get back the port associated bith this pin.
    /// This can be used to free the peripheral.
    pub fn into_port(self) -> PinPort<NUM> {
        PinPort {}
    }

    /// Internal function to get a read/write pointer to the 
    /// configuration register of this pin.
    #[inline]
    fn get_cfg(&self) -> PtrRw<GlbGpioCfg0> {
        let mut gpio_cfg = GLB.gpio_cfg0();
        gpio_cfg.0 = unsafe { gpio_cfg.0.add(NUM as usize) };
        gpio_cfg
    }

    /// Set the pull up/down/float mode for this pin.
    #[inline]
    pub fn set_pull(&mut self, pull: PinPull) {
        self.get_cfg().modify(|reg| {
            match pull {
                PinPull::Float => {
                    reg.gpio_0_pu().clear();
                    reg.gpio_0_pd().clear();
                }
                PinPull::Up => {
                    reg.gpio_0_pu().fill();
                    reg.gpio_0_pd().clear();
                }
                PinPull::Down => {
                    reg.gpio_0_pd().fill();
                    reg.gpio_0_pu().clear();
                }
            }
        });
    }

}

impl<const NUM: u8, M: InputLike> Pin<NUM, M> {

    /// Enable of disable Shmitt trigger mode for an input pin.
    #[inline]
    pub fn set_smt(&mut self, smt: bool) {
        self.get_cfg().modify(|reg| reg.gpio_0_smt().set(smt as _));
    }

    #[inline]
    pub fn is_smt(&self) -> bool {
        self.get_cfg().get().gpio_0_smt().get() != 0
    }

}

impl<const NUM: u8, M: OutputLike> Pin<NUM, M> {

    /// Set the drive mode for an output pin.
    #[inline]
    pub fn set_drive(&mut self, drive: PinDrive) {
        self.get_cfg().modify(|reg| reg.gpio_0_drv().set(drive as _));
    }

    #[inline]
    pub fn set_high(&mut self) {

        let reg = NUM / 32;
        let bit = NUM % 32;
        
        let mut cfg = GLB.gpio_cfg138();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };
        cfg.modify(|reg| {
            reg.0 |= 1 << bit;
        });

    }

    #[inline]
    pub fn set_low(&mut self) {

        let reg = NUM / 32;
        let bit = NUM % 32;

        let mut cfg = GLB.gpio_cfg140();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };
        cfg.modify(|reg| {
            reg.0 |= 1 << bit;
        });

    }

}


// /// Configuration mode of a pin.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum PinMode {
//     /// GPIO input.
//     Input,
//     /// GPIO output.
//     Output,
//     /// TBD
//     Analog,
//     /// Alternative custom function.
//     Alternate(PinFunction)
// }

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

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// pub enum PinFunction {
//     Sdh      = 0,
//     Spi0     = 1,
//     I2s      = 3,
//     Pdm      = 4,
//     I2c0     = 5,
//     I2c1     = 6,
//     Uart     = 7,
//     Emac     = 8,
//     Cam      = 9,
//     Analog   = 10,
//     Digital  = 11,
//     Sdu      = 12,
//     Pwm0     = 16,
//     Pwm1     = 17,
//     Spi1     = 18,
//     I2c2     = 19,
//     I2c3     = 20,
//     DbiB     = 22,
//     DbiC     = 23,
//     JtagLP   = 25,
//     JtagM0   = 26,
//     JtagD0   = 27,
//     ClockOut = 31,
// }

// impl PinFunction {

//     pub fn from_number(num: u8) -> Option<Self> {
//         Some(match num {
//             0 => Self::Sdh,
//             1 => Self::Spi0,
//             3 => Self::I2s,
//             4 => Self::Pdm,
//             5 => Self::I2c0,
//             6 => Self::I2c1,
//             7 => Self::Uart,
//             8 => Self::Emac,
//             9 => Self::Cam,
//             10 => Self::Analog,
//             11 => Self::Digital,
//             12 => Self::Sdu,
//             16 => Self::Pwm0,
//             17 => Self::Pwm1,
//             18 => Self::Spi1,
//             19 => Self::I2c2,
//             20 => Self::I2c3,
//             22 => Self::DbiB,
//             23 => Self::DbiC,
//             25 => Self::JtagLP,
//             26 => Self::JtagM0,
//             27 => Self::JtagD0,
//             31 => Self::ClockOut,
//             _ => return None,
//         })
//     }

// }
