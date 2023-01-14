//! GPIO management on BL808.

use embedded_util::peripheral;
use emhal::mmio::PtrRw;

use crate::register::GLB;
use crate::register::glb::GlbGpioCfg0;


/// Represent a configurable GPIO pin. You must ensure that
/// no other instance of this structure exists with the same
/// pin number for the lifetime of this structure.
pub struct Pin<const NUM: u8> {}
peripheral!(Pin<NUM>, NUM: u8[0..46]);

impl<const NUM: u8> Pin<NUM> {

    /// Get the pointer to the GPIO configuration for the current pin.
    #[inline]
    fn get_cfg(&self) -> PtrRw<GlbGpioCfg0> {
        let mut gpio_cfg = GLB.gpio_cfg0();
        gpio_cfg.0 = unsafe { gpio_cfg.0.add(NUM as usize) };
        gpio_cfg
    }

    /// Set the mode of this pin.
    /// 
    /// **Note that** this will reset any other parameter to the
    /// default one for the mode.
    #[inline]
    pub fn set_mode(&mut self, mode: PinMode) {
        self.get_cfg().modify(|reg| {
            reg.0 = 0;
            reg.gpio_0_int_mask().fill();
            match mode {
                PinMode::Input => {
                    reg.gpio_0_ie().fill();
                    reg.gpio_0_func_sel().set(11);
                }
                PinMode::Output => {
                    reg.gpio_0_oe().fill();
                    reg.gpio_0_func_sel().set(11);
                    reg.gpio_0_mode().set(1); // Toggle output mode.
                    reg.gpio_0_drv().set(0);
                }
                PinMode::Analog => {
                    reg.gpio_0_func_sel().set(10);
                }
                PinMode::Alternate(func) => {
                    reg.gpio_0_func_sel().set(func as _);
                }
            }
        });
    }

    pub fn get_mode(&mut self) -> PinMode {
        let mut reg = self.get_cfg().get();
        match reg.gpio_0_func_sel().get() as u8 {
            10 => PinMode::Analog,
            11 if reg.gpio_0_ie().get() != 0 => PinMode::Input,
            11 if reg.gpio_0_oe().get() != 0 => PinMode::Output,
            func => PinMode::Alternate(PinFunction::from_number(func).unwrap()),
        }
    }

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

    /// Enable of disable Shmitt trigger mode for an input pin.
    #[inline]
    pub fn set_smt(&mut self, smt: bool) {
        self.get_cfg().modify(|reg| reg.gpio_0_smt().set(smt as _));
    }

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


/// Configuration mode of a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinMode {
    /// GPIO input.
    Input,
    /// GPIO output.
    Output,
    /// TBD
    Analog,
    /// Alternative custom function.
    Alternate(PinFunction)
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
