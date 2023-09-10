//! BouffaloLab Hardware Abstraction Layers.
//! 
//! This library provides many modules for each peripheral or utility available of the
//! selected chip.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

extern crate alloc;


#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
#[cfg(not(target_arch = "riscv32"))]
compile_error!("bl808 m0 chip requires 'riscv32' target architecture");

#[cfg(feature = "bl808-d0")]
#[cfg(not(target_arch = "riscv64"))]
compile_error!("bl808 d0 chip requires 'riscv64' target architecture");


// Low-level module.
pub mod arch;

// Internal module for sealing traits.
mod sealed;

// Low-level abstractions.
pub mod hart;
pub mod cache;
pub mod interrupt;

// Peripheral abstractions.
pub mod clock;
pub mod power;
pub mod time;
pub mod cpu;
pub mod dma;

// I/O abstractions.
pub mod gpio;
pub mod uart;
pub mod i2c;
pub mod adc;

// Internal reuses.
use cpu::CpuControl;

use gpio::PinAccess;
use uart::UartAccess;
use adc::AdcAccess;
use dma::Dma;

use core::sync::atomic::{AtomicBool, Ordering};


/// Initialize the current hart of the currently selected chip, this function needs to be
/// called before actually using this abstraction layer, because some assumptions made by
/// needs prior initialization by this function.
/// 
/// This function is unsafe because you must ensure that this function is called exactly
/// once per hart, not calling it or calling it multiple time is undefined behavior.
pub unsafe fn init() {
    hart::init();
    init_impl();
    clock::init(); // Need before initializing timer clock.
    time::init();
}

/// Init function specific to BL808 M0 core.
#[cfg(feature = "bl808-m0")]
fn init_impl() {

    use arch::bl808::{CLIC, GLB};
    use interrupt::COUNT;

    // We use all bits for interrupt level, no priority bit.
    CLIC.cfg().modify(|reg| reg.nlbits().set(8));

    for irq_num in 0..COUNT {
        let int = CLIC.int(irq_num);
        int.enable().set(0);
        int.pending().set(0);
        int.attr().modify(|reg| reg.vectored().clear());
        int.control().set(255);
    }

    // Disable UART sig swap for all pin groups.
    GLB.parm_cfg0().modify(|reg| reg.uart_swap_set().clear());

    // These registers are not properly initialized by default.
    GLB.uart_cfg1().set_with(|reg| reg.0 = 0xFFFFFFFF);
    GLB.uart_cfg2().set_with(|reg| reg.0 = 0x0000FFFF);
    
}


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
            .then_some(unsafe { Self::new() })
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

    /// Create a new peripheral structure, containing accesses to all abstracted 
    /// peripherals that can be individually owned. This function is unsafe because the
    /// caller must ensure that no other instance of these peripheral exists, because 
    /// using two instances targetting the same peripheral will ultimately lead to 
    /// unstable software.
    /// 
    /// **Therefore you should use `try_take` or `take` if you want to be sure that no
    /// other one own this structure at the same time.** 
    /// 
    /// *This function is internally used by runtime in order to quickly setup a working
    /// communication in case of panics.*
    pub unsafe fn new() -> Self {
        Self {
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
