//! UART management on BL808.

use super::gpio::{Pin, PinMode, PinFunction, PinPull, PinDrive};
use super::clock::Clocks;

use super::mmio::{self, GLB, UART0, UART1, UART2};
use super::mmio::uart::UartBitPrd;

use core::fmt;


/// Definition of a UART port.
/// 
/// The UART port is stopped and pins are detached automatically
/// when this object is dropped.
pub struct Uart {
    port: UartPort,
    tx_pin: Option<Pin>,
    rx_pin: Option<Pin>,
}

impl Uart {

    /// Create an uninitialized UART port.
    /// 
    /// **You must** ensure that there are no other instances of
    /// this structure uses this UART port.
    pub fn new(port: UartPort) -> Self {
        Self {
            port,
            tx_pin: None,
            rx_pin: None,
        }
    }

    /// Internal function to attach a pin to a specific UART function.
    fn attach_pin(&mut self, pin: &mut Pin, func: UartFunction) {

        debug_assert!(pin.number() < 12, "uart pin number must be between 0 and 11 included");

        // There are 8 u32 fields per register
        let reg = pin.number() / 8;
        let field = (pin.number() % 8) * 4;

        let mut cfg = GLB.uart_cfg1();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };

        cfg.modify(|reg| {
            reg.0 &= !(0xF << field);
            reg.0 |= (func as u32) << field;
        });

        pin.set_mode(PinMode::Alternate(PinFunction::Uart));
        pin.set_pull(PinPull::Up);
        pin.set_drive(PinDrive::Drive1);
        pin.set_smt(true);
        
    }

    /// Internal fucntion to detach a pin from this UART.
    fn detach_pin(&mut self, pin: &mut Pin) {
        
        let reg = pin.number() / 8;
        let field = (pin.number() % 8) * 4;
        
        let mut cfg = GLB.uart_cfg1();
        cfg.0 = unsafe { cfg.0.add(reg as usize) };

        cfg.modify(|reg| {
            reg.0 &= !(0xF << field);
        });

    }

    /// Attach a GPIO pin as UART TX on this port.
    pub fn attach_tx(&mut self, mut pin: Pin) {
        self.attach_pin(&mut pin, match self.port {
            UartPort::Port0 => UartFunction::Uart0Tx,
            UartPort::Port1 => UartFunction::Uart1Tx,
            UartPort::Port2 => UartFunction::Uart2Tx,
        });
        self.tx_pin = Some(pin);
    }

    /// Attach a GPIO pin as UART RX on this port.
    pub fn attach_rx(&mut self, mut pin: Pin) {
        self.attach_pin(&mut pin, match self.port {
            UartPort::Port0 => UartFunction::Uart0Rx,
            UartPort::Port1 => UartFunction::Uart1Rx,
            UartPort::Port2 => UartFunction::Uart2Rx,
        });
        self.rx_pin = Some(pin);
    }

    /// Detach the GPIO pin. Returning None if no pin was attached.
    pub fn detach_tx(&mut self) -> Option<Pin> {
        let mut pin = self.tx_pin.take()?;
        self.detach_pin(&mut pin);
        Some(pin)
    }

    /// Detach the GPIO pin. Returning None if no pin was attached.
    pub fn detach_rx(&mut self) -> Option<Pin> {
        let mut pin = self.rx_pin.take()?;
        self.detach_pin(&mut pin);
        Some(pin)
    }

    /// Internal method to get the right UART MMIO structure depending 
    /// on the configured port for this UART.
    fn get_mmio(&self) -> mmio::Uart {
        match self.port {
            UartPort::Port0 => UART0,
            UartPort::Port1 => UART1,
            UartPort::Port2 => UART2,
        }
    }

    /// Configure this UART port using the given configuration.
    /// 
    /// This can be called multiple time if the pin attachment is changed.
    /// 
    /// *Note that* you must give the clocks handle that will be used
    /// to query the real UART frequency, in order to properly configure
    /// the baudrate.
    pub fn init<C>(&mut self, config: &UartConfig, clocks: &Clocks<C>) {

        // Calculate the baudrate divisor from UART frequency.
        let uart_freq = clocks.get_uart_freq();
        let div = (uart_freq * 10 / config.baudrate + 5) / 10;

        let mmio = self.get_mmio();

        // Disable both TX and RX at start.
        mmio.utx_cfg().modify(|reg| reg.en().clear());
        mmio.urx_cfg().modify(|reg| reg.en().clear());

        // Set periods.
        let mut bit_prd = UartBitPrd::default();
        bit_prd.utx_period().set(div - 1);
        bit_prd.urx_period().set(div - 1);
        mmio.bit_prd().set(bit_prd);

        // Modify both TX and RX registers at once.
        let mut utx_cfg = mmio.utx_cfg().get();
        let mut urx_cfg = mmio.urx_cfg().get();

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
        mmio.utx_cfg().set(utx_cfg);
        mmio.urx_cfg().set(urx_cfg);

        // ???
        mmio.sw_mode().modify(|reg| reg.urx_rxd_sw_mode().clear());

        // Send LSB-first.
        mmio.data_cfg().modify(|reg| reg.bit_inv().clear());

        // Configure FIFO threshold.
        mmio.fifo_cfg1().modify(|reg| {
            reg.tx_fifo_th().set(config.tx_fifo_threshold as _);
            reg.rx_fifo_th().set(config.rx_fifo_threshold as _);
        });

        // Clear FIFO.
        mmio.fifo_cfg0().modify(|reg| {
            reg.tx_fifo_clear().fill();
            reg.rx_fifo_clear().fill();
            reg.dma_tx_en().clear();
            reg.dma_rx_en().clear();
        });

        mmio.int_mask().set(0xFFFFFFFF);

    }

    /// Start this UART port. It should have been previously configured
    /// using [``].
    pub fn start(&mut self) {
        
        let mmio = self.get_mmio();

        // Enable TX if a pin is attached.
        if self.tx_pin.is_some() {
            mmio.utx_cfg().modify(|reg| reg.en().fill());
        }

        // Enable RX if a pin is attached.
        if self.rx_pin.is_some() {
            mmio.urx_cfg().modify(|reg| reg.en().fill());
        }

    }

    /// Stop this UART port.
    pub fn stop(&mut self) {
        let mmio = self.get_mmio();
        mmio.utx_cfg().modify(|reg| reg.en().clear());
        mmio.urx_cfg().modify(|reg| reg.en().clear());
    }

    /// Simplest function to read a single byte, if available.
    pub fn read_byte(&mut self) -> Option<u8> {
        let mmio = self.get_mmio();
        if mmio.fifo_cfg1().get().rx_fifo_count().get() != 0 {
            Some(mmio.fifo_rdata().get())
        } else {
            None
        }
    }

    /// Simplest function to write a single byte of data to the UART TX.
    pub fn write_byte(&mut self, byte: u8) {
        let mmio = self.get_mmio();
        while mmio.fifo_cfg1().get().tx_fifo_count().get() == 0 {}
        mmio.fifo_wdata().set(byte);
    }

}

impl Drop for Uart {
    fn drop(&mut self) {
        self.stop();
        self.detach_tx();
        self.detach_rx();
    }
}


#[derive(Debug, Clone, Copy)]
pub enum UartPort {
    Port0,
    Port1,
    Port2,
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


impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
