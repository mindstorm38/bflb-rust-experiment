//! BouffaloLab Hardware Abstraction Layers.
//! 
//! This library provides many modules for each peripheral or utility available of the
//! selected chip.

#![no_std]

extern crate alloc;


#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
#[cfg(not(target_arch = "riscv32"))]
compile_error!("bl808 m0 chip requires 'riscv32' target architecture");

#[cfg(feature = "bl808-d0")]
#[cfg(not(target_arch = "riscv64"))]
compile_error!("bl808 d0 chip requires 'riscv64' target architecture");


#[cfg(any(feature = "bl808-m0", feature = "bl808-d0", feature = "bl808-lp"))]
pub mod bl808;

mod sealed;

pub mod hart;
pub mod interrupt;
pub mod clock;
pub mod power;
pub mod time;
pub mod cpu;
pub mod dma;

pub mod gpio;
pub mod uart;
pub mod i2c;
pub mod adc;

// pub mod cam;

use interrupt::Interrupts;
use clock::Clocks;
use time::Timer;
use cpu::CpuControl;

use gpio::PinAccess;
use uart::UartAccess;
use adc::AdcAccess;
use dma::Dma;

use core::sync::atomic::{AtomicBool, Ordering};

/// Re-export of the RISC-V HAL library.
pub use riscv_hal as riscv;


/// We want the peripherals to be a ZST.
const _: () = assert!(core::mem::size_of::<Peripherals>() == 0);

/// Internal variable to know if peripherals structure is available.
static TAKEN: AtomicBool = AtomicBool::new(false);


/// Peripherals specific to the BuffaloLab chips, and specifically to
/// the selected chip. The different peripherals are made public in
/// order to be moved out of the structure, and maybe moved/shared to
/// other threads and functions.
/// 
/// Note that some fields in this structure are used to group many
/// "sub" peripherals, like GPIO, UART or DMA which contains multiple
/// ports/channels. These ports/channels can be owned and managed 
/// individually.
pub struct Peripherals {
    /// Handle to the interrupts manager.
    pub interrupts: Interrupts,
    /// The chip's clocks.
    pub clocks: Clocks,
    /// The core RTC timer.
    pub timer: Timer,
    /// The core's CPU control 
    pub cpu_control: CpuControl,
    /// GPIO pins access.
    pub gpio: Gpio,
    /// UART ports access.
    pub uart: Uart,
    /// DMA ports access.
    pub dma: Dma,
    /// ADC peripheral access.
    pub adc: AdcAccess,
}

impl Peripherals {

    /// Try taking ownership of the peripheral, returning `Some` peripheral
    /// is not already taken.
    pub fn try_take() -> Option<Self> {
        TAKEN
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
            .then_some(Self::new())
    }

    /// Try taking ownership of the peripheral, this function panics if not
    /// possible.
    pub fn take() -> Self {
        Self::try_take().expect("peripheral was already taken")
    }

    /// Free ownership of the peripheral.
    pub fn free(self) {
        drop(self);
        TAKEN.store(false, core::sync::atomic::Ordering::Release);
    }

    fn new() -> Self {
        Self {
            interrupts: Interrupts(()),
            clocks: Clocks(()),
            timer: Timer(()),
            cpu_control: CpuControl(()),
            gpio: Gpio {
                p0: PinAccess(()),
                p1: PinAccess(()),
                p2: PinAccess(()),
                p3: PinAccess(()),
                p4: PinAccess(()),
                p5: PinAccess(()),
                p6: PinAccess(()),
                p7: PinAccess(()),
                p8: PinAccess(()),
                p9: PinAccess(()),
                p10: PinAccess(()),
                p11: PinAccess(()),
                p12: PinAccess(()),
                p13: PinAccess(()),
                p14: PinAccess(()),
                p15: PinAccess(()),
                p16: PinAccess(()),
                p17: PinAccess(()),
                p18: PinAccess(()),
                p19: PinAccess(()),
                p20: PinAccess(()),
                p21: PinAccess(()),
                p22: PinAccess(()),
                p23: PinAccess(()),
                p24: PinAccess(()),
                p25: PinAccess(()),
                p26: PinAccess(()),
                p27: PinAccess(()),
                p28: PinAccess(()),
                p29: PinAccess(()),
                p30: PinAccess(()),
                p31: PinAccess(()),
                p32: PinAccess(()),
                p33: PinAccess(()),
                p34: PinAccess(()),
                p35: PinAccess(()),
                p36: PinAccess(()),
                p37: PinAccess(()),
                p38: PinAccess(()),
                p39: PinAccess(()),
                p40: PinAccess(()),
                p41: PinAccess(()),
                p42: PinAccess(()),
                p43: PinAccess(()),
                p44: PinAccess(()),
                p45: PinAccess(()),
            },
            uart: Uart {
                p0: UartAccess(()),
                p1: UartAccess(()),
                p2: UartAccess(()),
            },
            dma: Dma::new(),
            adc: AdcAccess(()),
        }
    }



}


/// This peripheral structure wraps all GPIO pin available.
pub struct Gpio {
    pub p0: PinAccess<0>,
    pub p1: PinAccess<1>,
    pub p2: PinAccess<2>,
    pub p3: PinAccess<3>,
    pub p4: PinAccess<4>,
    pub p5: PinAccess<5>,
    pub p6: PinAccess<6>,
    pub p7: PinAccess<7>,
    pub p8: PinAccess<8>,
    pub p9: PinAccess<9>,
    pub p10: PinAccess<10>,
    pub p11: PinAccess<11>,
    pub p12: PinAccess<12>,
    pub p13: PinAccess<13>,
    pub p14: PinAccess<14>,
    pub p15: PinAccess<15>,
    pub p16: PinAccess<16>,
    pub p17: PinAccess<17>,
    pub p18: PinAccess<18>,
    pub p19: PinAccess<19>,
    pub p20: PinAccess<20>,
    pub p21: PinAccess<21>,
    pub p22: PinAccess<22>,
    pub p23: PinAccess<23>,
    pub p24: PinAccess<24>,
    pub p25: PinAccess<25>,
    pub p26: PinAccess<26>,
    pub p27: PinAccess<27>,
    pub p28: PinAccess<28>,
    pub p29: PinAccess<29>,
    pub p30: PinAccess<30>,
    pub p31: PinAccess<31>,
    pub p32: PinAccess<32>,
    pub p33: PinAccess<33>,
    pub p34: PinAccess<34>,
    pub p35: PinAccess<35>,
    pub p36: PinAccess<36>,
    pub p37: PinAccess<37>,
    pub p38: PinAccess<38>,
    pub p39: PinAccess<39>,
    pub p40: PinAccess<40>,
    pub p41: PinAccess<41>,
    pub p42: PinAccess<42>,
    pub p43: PinAccess<43>,
    pub p44: PinAccess<44>,
    pub p45: PinAccess<45>,
}


/// This peripheral structure wrap ports of UART controller.
pub struct Uart {
    pub p0: UartAccess<0>,
    pub p1: UartAccess<1>,
    pub p2: UartAccess<2>,
}
