//! Serial I/O management on BL808.

use embedded_util::peripheral;

use crate::register::{Uart as UartRegs, GLB, UART0, UART1, UART2};
use crate::register::uart::UartBitPrd;

use super::gpio::{PinPort, PinPull, PinDrive, Uart as UartFunc};
use super::clock::Clocks;

use core::marker::PhantomData;
use core::fmt;


/// Definition of a UART port. This port need to be configured in
/// order to obtain a [`Uart`] structure that is actually usable
/// for TX and/or RX communications.
pub struct UartPort<const PORT: u8> {}
peripheral!(UartPort<PORT>, PORT: u8[0..3]);

impl<const PORT: u8> UartPort<PORT> {

    /// Generic types erasing, this transfer checks to the runtime.
    pub fn erase(self) -> ! {
        todo!()
    }

    /// Configure this UART port for duplex communications.
    pub fn into_duplex<const TX_PIN: u8, const RX_PIN: u8>(self, 
        tx: PinPort<TX_PIN>,
        rx: PinPort<RX_PIN>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> Uart<PORT, PinPort<TX_PIN>, PinPort<RX_PIN>> {

        attach_pin(tx, match PORT {
            0 => UartFunction::Uart0Tx,
            1 => UartFunction::Uart1Tx,
            2 => UartFunction::Uart2Tx,
            _ => unreachable!()
        });

        attach_pin(rx, match PORT {
            0 => UartFunction::Uart0Rx,
            1 => UartFunction::Uart1Rx,
            2 => UartFunction::Uart2Rx,
            _ => unreachable!()
        });

        Uart::init(config, clocks)

    }

    /// Configure this UART port for TX-only communications.
    pub fn into_tx<const TX_PIN: u8>(self, 
        tx: PinPort<TX_PIN>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> Uart<PORT, PinPort<TX_PIN>, Detached> {

        attach_pin(tx, match PORT {
            0 => UartFunction::Uart0Tx,
            1 => UartFunction::Uart1Tx,
            2 => UartFunction::Uart2Tx,
            _ => unreachable!()
        });

        Uart::init(config, clocks)

    }

    /// Configure this UART port for RX-only communications.
    pub fn into_rx<const RX_PIN: u8>(self, 
        rx: PinPort<RX_PIN>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> Uart<PORT, Detached, PinPort<RX_PIN>> {

        attach_pin(rx, match PORT {
            0 => UartFunction::Uart0Rx,
            1 => UartFunction::Uart1Rx,
            2 => UartFunction::Uart2Rx,
            _ => unreachable!()
        });

        Uart::init(config, clocks)

    }

}


/// An I/O capable structure that can be obtained by configuring a
/// [`UartPort`] peripheral.
pub struct Uart<const PORT: u8, Tx: Attachment, Rx: Attachment> {
    _tx: PhantomData<Tx>,
    _rx: PhantomData<Rx>,
}

/// State structure when a UART lane is detached from pin.
pub struct Detached;


impl<const PORT: u8, Tx: Attachment, Rx: Attachment> Uart<PORT, Tx, Rx> {
    
    /// Get back the port associated bith this configured UART.
    /// This can be used to free the peripheral.
    pub fn into_port(self) -> UartPort<PORT> {
        // Note that here the object is dropped, and therefore TX/RX lanes are stopped.
        UartPort {}
    }

    /// Internal function used to initialize the I/O given a configuration and clocks.
    fn init(config: &UartConfig, clocks: &Clocks) -> Self {

        // Calculate the baudrate divisor from UART frequency.
        let uart_freq = clocks.get_uart_freq();
        let div = (uart_freq * 10 / config.baudrate + 5) / 10;

        let regs = Self::get_registers();

        // Disable both TX and RX at start.
        regs.utx_cfg().modify(|reg| reg.en().clear());
        regs.urx_cfg().modify(|reg| reg.en().clear());

        // Set periods.
        let mut bit_prd = UartBitPrd::default();
        bit_prd.utx_period().set(div - 1);
        bit_prd.urx_period().set(div - 1);
        regs.bit_prd().set(bit_prd);

        // Modify both TX and RX registers at once.
        let mut utx_cfg = regs.utx_cfg().get();
        let mut urx_cfg = regs.urx_cfg().get();

        // Set parity.
        match config.parity {
            UartParity::None => {
                utx_cfg.parity_en().clear();
                urx_cfg.parity_en().clear();
            }
            UartParity::Odd => {
                utx_cfg.parity_en().fill();
                utx_cfg.parity_sel().fill();
                urx_cfg.parity_en().fill();
                urx_cfg.parity_sel().fill();
            }
            UartParity::Even => {
                utx_cfg.parity_en().fill();
                utx_cfg.parity_sel().clear();
                urx_cfg.parity_en().fill();
                urx_cfg.parity_sel().clear();
            }
        }

        // Set data bits.
        utx_cfg.bit_count_d().set(config.data_bits as _);
        urx_cfg.bit_count_d().set(config.data_bits as _);

        // Set TX stop bits.
        utx_cfg.bit_count_p().set(config.stop_bits as _);

        // Set TX CTS.
        utx_cfg.cts_en().set(config.flow_control_cts as _);

        // Enable TX free-run mode/
        utx_cfg.frm_en().fill();

        // Disable de-glitch.
        urx_cfg.deg_en().clear();

        // Write back TX/RX config registers.
        regs.utx_cfg().set(utx_cfg);
        regs.urx_cfg().set(urx_cfg);

        // ???
        regs.sw_mode().modify(|reg| reg.urx_rxd_sw_mode().clear());

        // Send LSB-first.
        regs.data_cfg().modify(|reg| reg.bit_inv().clear());

        // Configure FIFO threshold.
        regs.fifo_cfg1().modify(|reg| {
            reg.tx_fifo_th().set(config.tx_fifo_threshold as _);
            reg.rx_fifo_th().set(config.rx_fifo_threshold as _);
        });

        // Clear FIFO.
        regs.fifo_cfg0().modify(|reg| {
            reg.tx_fifo_clear().fill();
            reg.rx_fifo_clear().fill();
            reg.dma_tx_en().clear();
            reg.dma_rx_en().clear();
        });

        regs.int_mask().set(0xFFFFFFFF);

        // Enable TX if a pin is attached.
        if Tx::get_attachment().is_some() {
            regs.utx_cfg().modify(|reg| reg.en().fill());
        }

        // Enable RX if a pin is attached.
        if Rx::get_attachment().is_some() {
            regs.urx_cfg().modify(|reg| reg.en().fill());
        }

        Self {
            _tx: PhantomData,
            _rx: PhantomData,
        }

    }

    /// Get the UART MMIO registers structure associated to the given port.
    #[inline(always)]
    fn get_registers() -> UartRegs {
        match PORT {
            0 => UART0,
            1 => UART1,
            2 => UART2,
            _ => unreachable!()
        }
    }

}


impl<const PORT: u8, const TX_PIN: u8, Rx: Attachment> Uart<PORT, PinPort<TX_PIN>, Rx> {

    /// Simplest function to write a single byte of data to the UART TX.
    pub fn write_byte(&mut self, byte: u8) {
        let regs = Self::get_registers();
        while regs.fifo_cfg1().get().tx_fifo_count().get() == 0 {}
        regs.fifo_wdata().set(byte);
    }
    
}

impl<const PORT: u8, const TX_PIN: u8, Rx: Attachment> fmt::Write for Uart<PORT, PinPort<TX_PIN>, Rx> {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }

}


impl<const PORT: u8, const RX_PIN: u8, Tx: Attachment> Uart<PORT, Tx, PinPort<RX_PIN>> {

    /// Simplest function to read a single byte, if available.
    pub fn read_byte(&mut self) -> Option<u8> {
        let regs = Self::get_registers();
        if regs.fifo_cfg1().get().rx_fifo_count().get() != 0 {
            Some(regs.fifo_rdata().get())
        } else {
            None
        }
    }

}


// Drop implementation that automatically disable lanes.
impl<const PORT: u8, Tx: Attachment, Rx: Attachment> Drop for Uart<PORT, Tx, Rx> {

    fn drop(&mut self) {

        let regs = Self::get_registers();
        regs.utx_cfg().modify(|reg| reg.en().clear());
        regs.urx_cfg().modify(|reg| reg.en().clear());

        if let Some(tx_pin) = Tx::get_attachment() {
            detach_pin(tx_pin);
        }

        if let Some(rx_pin) = Rx::get_attachment() {
            detach_pin(rx_pin);
        }

    }

}


/// Internal function to attach a pin to a specific UART function.
fn attach_pin<const NUM: u8>(pin: PinPort<NUM>, func: UartFunction) {

    debug_assert!(NUM < 12, "uart pin number must be between 0 and 11 included");

    // There are 8 u32 fields per register
    let reg = NUM / 8;
    let field = (NUM % 8) * 4;

    let mut cfg = GLB.uart_cfg1();
    cfg.0 = unsafe { cfg.0.add(reg as usize) };

    cfg.modify(|reg| {
        reg.0 &= !(0xF << field);
        reg.0 |= (func as u32) << field;
    });

    let mut pin = pin.into_alternate::<UartFunc>();
    pin.set_pull(PinPull::Up);
    pin.set_drive(PinDrive::Drive1);
    pin.set_smt(true);
    
}

/// Internal fucntion to detach a pin from this UART.
fn detach_pin(num: u8) {
    
    let reg = num / 8;
    let field = (num % 8) * 4;
    
    let mut cfg = GLB.uart_cfg1();
    cfg.0 = unsafe { cfg.0.add(reg as usize) };

    cfg.modify(|reg| {
        reg.0 &= !(0xF << field);
    });

}


pub trait Attachment {
    fn get_attachment() -> Option<u8>;
}

impl<const NUM: u8> Attachment for PinPort<NUM> {
    #[inline(always)]
    fn get_attachment() -> Option<u8> {
        Some(NUM)
    }
}

impl Attachment for Detached {
    #[inline(always)]
    fn get_attachment() -> Option<u8> {
        None
    }
}


/// Configuration structure for UART initialization.
#[derive(Debug, Clone)]
pub struct UartConfig {
    /// Baudrate of the UART port.
    pub baudrate: u32,
    /// Parity mode of the UART port.
    pub parity: UartParity,
    /// Data bits.
    pub data_bits: UartDataBits,
    /// Stop bits.
    pub stop_bits: UartStopBits,
    /// Enable Request To Send.
    pub flow_control_rts: bool,
    /// Enable Clear To Send.
    pub flow_control_cts: bool,
    /// TX FIFO threshold, in range `0..32`.
    pub tx_fifo_threshold: u8,
    /// RX FIFO threshold, in range `0..32`.
    pub rx_fifo_threshold: u8,
}

impl UartConfig {

    /// Create a new basic config with every option
    /// disabled, data bits to 8 and stop bits to 1.
    pub const fn new(baudrate: u32) -> Self {
        Self { 
            baudrate, 
            parity: UartParity::None, 
            data_bits: UartDataBits::Bits8, 
            stop_bits: UartStopBits::Bits1, 
            flow_control_rts: false, 
            flow_control_cts: false, 
            tx_fifo_threshold: 7, 
            rx_fifo_threshold: 7,
        }
    }

}

/// Parity mode for UART.
#[derive(Debug, Clone, Copy)]
pub enum UartParity {
    None,
    Odd,
    Even,
}

/// Data bits for UART.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum UartDataBits {
    Bits5 = 4,
    Bits6 = 5,
    Bits7 = 6,
    Bits8 = 7,
}

/// Data bits for UART.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum UartStopBits {
    Bits0p5 = 0,
    Bits1 = 1,
    Bits1p5 = 2,
    Bits2 = 3,
}


#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(unused)]
enum UartFunction {
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
    Disabled    = 15,
}
