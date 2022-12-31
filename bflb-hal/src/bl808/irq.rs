//! Interrupt management and numbers for BL808.



pub mod m0_lp {

    /// Base number for custom IRQ.
    const NUM_BASE: usize = 16;

    // DMA
    pub const DMA0_ALL: usize   = NUM_BASE + 15;
    pub const DMA1_ALL: usize   = NUM_BASE + 16;
    // IR
    pub const IRTX: usize       = NUM_BASE + 19;
    pub const IRRX: usize       = NUM_BASE + 20;
    // USB
    pub const USB: usize        = NUM_BASE + 21;
    // EMAC
    pub const EMAC: usize       = NUM_BASE + 24;
    // ADC
    pub const GPADC_DMA: usize  = NUM_BASE + 25;
    // SPI
    pub const SPI0: usize       = NUM_BASE + 27;
    // UART
    pub const UART0: usize      = NUM_BASE + 28;
    pub const UART1: usize      = NUM_BASE + 29;
    pub const UART2: usize      = NUM_BASE + 30;
    // GPIO
    pub const GPIO_DMA: usize   = NUM_BASE + 31;
    pub const GPIO_INT0: usize  = NUM_BASE + 44;
    // I2C
    pub const I2C0: usize       = NUM_BASE + 32;
    pub const I2C1: usize       = NUM_BASE + 39;
    // PWM
    pub const PWM: usize        = NUM_BASE + 33;
    // TIMER0
    pub const TIMER0_CH0: usize = NUM_BASE + 36; 
    pub const TIMER0_CH1: usize = NUM_BASE + 37; 
    pub const TIMER0_WDT: usize = NUM_BASE + 38; 
    // I2S
    pub const I2S: usize        = NUM_BASE + 40;
    // PDS
    pub const PDS_WAKEUP: usize = NUM_BASE + 50;
    // HBN
    pub const HBN_OUT0: usize   = NUM_BASE + 51;
    pub const HBN_OUT1: usize   = NUM_BASE + 52;

    /// Maximum IRQ number (included).
    pub const MAX: usize        = HBN_OUT1;

}

pub mod d0 {
    // TODO:
}


/// Interrupt Request controller abstraction structure.
/// 
/// Use this structure to easily manage interrupts
pub struct ClicIrq {
    
}
