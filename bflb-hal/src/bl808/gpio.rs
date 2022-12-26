//! GPIO management on BL808.

use emhal::mmio::PtrRw;

use super::mmio::glb::GlbGpioCfg0;
use super::Bl808;


/// GPIO controller for BL808.
pub struct Gpio {}

impl Gpio {

    pub const fn new() -> Self {
        Self {}
    }

    fn get_gpio_cfg(&self, pin: u8) -> PtrRw<GlbGpioCfg0> {

        // The GPIO config registers go from 0 to 45 included.
        // So we check this here, only in debug mode.
        debug_assert!(pin <= 45, "gpio pin number must be between 0 and 45 included, got {pin}");

        let mut gpio_cfg = Bl808::GLB.gpio_cfg0();
        gpio_cfg.0 = unsafe { gpio_cfg.0.add(pin as usize) };
        gpio_cfg

    }

    /// Initialize a GPIO pin.
    #[inline(never)]
    pub fn init(&self, pin: u8, config: &GpioConfig) {
        self.get_gpio_cfg(pin).modify(|reg| {

            // Clear the whole register.
            reg.0 = 0;

            reg.gpio_0_int_mask().fill();

            match config.mode {
                GpioMode::Input { smt } => {
                    reg.gpio_0_ie().set(1);
                    reg.gpio_0_func_sel().set(11);
                    reg.gpio_0_smt().set(smt as _);
                }
                GpioMode::Output { mode, drive } => {
                    reg.gpio_0_oe().set(1);
                    reg.gpio_0_func_sel().set(11);
                    reg.gpio_0_mode().set(match mode {
                        GpioOutputMode::Normal => 0,
                        GpioOutputMode::Toggle => 1,
                    });
                    reg.gpio_0_drv().set(drive as _);
                }
                GpioMode::Analog => {}
                GpioMode::Alternate { function } => {
                    reg.gpio_0_ie().set(1);
                    reg.gpio_0_func_sel().set(function as _);
                }
            }

            match config.pull {
                GpioPull::Float => (),
                GpioPull::Up => reg.gpio_0_pu().set(1),
                GpioPull::Down => reg.gpio_0_pd().set(1),
            }

        });
    }

    /// Deinitialize a GPIO pin.
    pub fn deinit(&self, pin: u8) {
        const DEINIT_CONFIG: GpioConfig = GpioConfig::new();
        self.init(pin, &DEINIT_CONFIG);
    }

    /// Set the output value for a pin configured as normal output.
    #[inline(never)]
    pub fn set_normal(&self, pin: u8, on: bool) {
        self.get_gpio_cfg(pin).modify(|reg| {
            reg.gpio_0_o().set(on as _);
        });
    }

    #[inline(never)]
    pub fn set_toggle(&self, pin: u8) {

        let reg = pin / 32;
        let bit = pin % 32;

        let mut cfg = Bl808::GLB.gpio_cfg138();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };
        cfg.modify(|reg| {
            reg.0 |= 1 << bit;
        });

    }

    #[inline(never)]
    pub fn clear_toggle(&self, pin: u8) {

        let reg = pin / 32;
        let bit = pin % 32;

        let mut cfg = Bl808::GLB.gpio_cfg140();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };
        cfg.modify(|reg| {
            reg.0 |= 1 << bit;
        });

    }

    // pub fn init_uart(&self, pin: u8, func: GpioUartFunction) {

    //     let func_val = func as u8 as u32;

    //     let sig = pin % 12;

    //     if sig < 8 {

    //         let sig_pos = sig << 2;
    //         self.chip.glb.uart_cfg1().get().0;

    //     } else {

    //     }

    // }

}


#[derive(Debug, Clone)]
pub struct GpioConfig {
    pub mode: GpioMode,
    pub pull: GpioPull,
}

impl GpioConfig {

    /// Create a default gpio config that is:
    /// - Input
    /// - Floating (no PUPD)
    /// - Drive 0
    /// - Function SDH (= 0)
    /// - SMT is false
    pub const fn new() -> Self {
        Self {
            mode: GpioMode::Input {
                smt: false,
            },
            pull: GpioPull::Float,
        }
    }

    pub const fn with_normal_output() -> Self {
        Self {
            mode: GpioMode::Output { 
                mode: GpioOutputMode::Normal, 
                drive: GpioDrive::Drive0 
            },
            pull: GpioPull::Float,
        }
    }

    pub const fn with_toggle_output() -> Self {
        Self {
            mode: GpioMode::Output { 
                mode: GpioOutputMode::Toggle, 
                drive: GpioDrive::Drive0 
            },
            pull: GpioPull::Float,
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioPull {
    /// No pull-[up,down].
    Float,
    /// Pull-up.
    Up,
    /// Pull-down.
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioMode {
    /// GPIO input.
    Input {
        /// Enable/disable Shmitt trigger.
        smt: bool,
    },
    /// GPIO output.
    Output {
        /// Output mode.
        mode: GpioOutputMode,
        /// Output drive.
        drive: GpioDrive,
    },
    /// TBD
    Analog,
    /// Alternative mode, with custom configuration not defined
    /// by this API.
    Alternate {
        function: GpioFunction,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioOutputMode {
    /// Normal output mode, a single bit defines the output.
    Normal,
    /// Toggle output mode, use two set/clear bits.
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioDrive {
    Drive0 = 0,
    Drive1 = 1,
    Drive2 = 2,
    Drive3 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioFunction {
    Sdh      = 0,
    Spi0     = 1,
    I2s      = 3,
    Pdm      = 4,
    I2c0     = 5,
    I2c1     = 6,
    Emac     = 8,
    Cam      = 9,
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpioUartFunction {
    Uart0Rts    = 0,
    Uart0Cts    = 1,
    Uart0Tx     = 2,
    Uart0Rx     = 3,
    Uart1Rts    = 4,
    Uart1Cts    = 5,
    Uart1Tx     = 6,
    Uart1Rx     = 7,
    Uart2Rts    = 8,
    Uart2Cts    = 9,
    Uart2Tx     = 10,
    Uart2Rx     = 11,
}
