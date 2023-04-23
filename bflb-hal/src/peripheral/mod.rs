//! Actual HAL modules, most are common to all chips and some are 
//! feature-gated to some specific chips. The initial access to the
//! peripheral is provided by 

pub mod irq;
pub mod clock;
pub mod time;
pub mod cpu;

pub mod gpio;
pub mod uart;
pub mod dma;
pub mod adc;


use irq::Interrupts;
use clock::Clocks;
use time::CoreTimer;
use cpu::CpuControl;

use gpio::PinAccess;
use uart::UartAccess;
use dma::DmaAccess;
use adc::AdcAccess;


/// Peripherals specific to the BuffaloLab chips, and specifically to
/// the selected chip. The different peripherals are made public in
/// order to be moved out of the structure, and maybe moved/shared to
/// other threads or functions.
pub struct Peripherals {
    /// Handle to the interrupts manager.
    pub interrupts: Interrupts,
    /// The chip's clocks.
    pub clocks: Clocks,
    /// The core real time clock.
    pub core_timer: CoreTimer,
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

    embedded_util::peripheral!(single);

    pub unsafe fn new() -> Self {
        Self {
            interrupts: Interrupts(()),
            clocks: Clocks(()),
            core_timer: CoreTimer(()),
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
            dma: Dma {
                p0: Dma0 {
                    c0: DmaAccess(()),
                    c1: DmaAccess(()),
                    c2: DmaAccess(()),
                    c3: DmaAccess(()),
                    c4: DmaAccess(()),
                    c5: DmaAccess(()),
                    c6: DmaAccess(()),
                    c7: DmaAccess(()),
                },
                p1: Dma1 {
                    c0: DmaAccess(()),
                    c1: DmaAccess(()),
                    c2: DmaAccess(()),
                    c3: DmaAccess(()),
                },
                p2: Dma2 {
                    c0: DmaAccess(()),
                    c1: DmaAccess(()),
                    c2: DmaAccess(()),
                    c3: DmaAccess(()),
                    c4: DmaAccess(()),
                    c5: DmaAccess(()),
                    c6: DmaAccess(()),
                    c7: DmaAccess(()),
                },
            },
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


/// This peripheral structure wraps all DMA channels available.
pub struct Dma {
    /// DMA port 0.
    pub p0: Dma0,
    /// DMA port 1.
    pub p1: Dma1,
    /// DMA port 2.
    pub p2: Dma2,
}

/// This peripheral structure wrap channels of DMA port 0.
pub struct Dma0 {
    pub c0: DmaAccess<0, 0>,
    pub c1: DmaAccess<0, 1>,
    pub c2: DmaAccess<0, 2>,
    pub c3: DmaAccess<0, 3>,
    pub c4: DmaAccess<0, 4>,
    pub c5: DmaAccess<0, 5>,
    pub c6: DmaAccess<0, 6>,
    pub c7: DmaAccess<0, 7>,
}

/// This peripheral structure wrap channels of DMA port 1.
pub struct Dma1 {
    pub c0: DmaAccess<1, 0>,
    pub c1: DmaAccess<1, 1>,
    pub c2: DmaAccess<1, 2>,
    pub c3: DmaAccess<1, 3>,
}

/// This peripheral structure wrap channels of DMA port 0.
pub struct Dma2 {
    pub c0: DmaAccess<2, 0>,
    pub c1: DmaAccess<2, 1>,
    pub c2: DmaAccess<2, 2>,
    pub c3: DmaAccess<2, 3>,
    pub c4: DmaAccess<2, 4>,
    pub c5: DmaAccess<2, 5>,
    pub c6: DmaAccess<2, 6>,
    pub c7: DmaAccess<2, 7>,
}


/// This peripheral structure wrap ports of UART controller.
pub struct Uart {
    pub p0: UartAccess<0>,
    pub p1: UartAccess<1>,
    pub p2: UartAccess<2>,
}
