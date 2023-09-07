//! Serial I/O management on BL808.

use core::ptr::addr_of;
use core::fmt;

use crate::arch::bl808::{Uart as UartRegs, GLB, UART0, UART1, UART2};
use crate::arch::bl808::uart::UartBitPrd;

use crate::gpio::{Pin, PinPull, PinDrive, PinFunction, Alternate};
use crate::dma::{DmaSrcEndpoint, DmaDstEndpoint, DmaEndpointConfig, 
    DmaPeripheral, DmaDataWidth, DmaBurstSize, DmaIncrement};
use crate::sealed::Sealed;
use crate::clock::Clocks;


/// Abstract definition of a UART port with write access.
pub trait UartTxDev {
    
    /// Synchronously write a single byte of data to the UART TX lane.
    fn write_byte(&mut self, byte: u8);

    /// Synchronously write a bunch a data to the UART TX lane. 
    /// Default implementation just sequentially call [`write_byte`].
    fn write(&mut self, data: &[u8]) {
        for &byte in data {
            self.write_byte(byte)
        }
    }

}

/// Abstract definition of UART port with read access.
pub trait UartRxDev {
    
    /// Synchronously read a single byte of data from the UART RX, if available.
    fn read_byte(&mut self) -> Option<u8>;

}


/// Definition of an exclusive access to a UART port. This port need 
/// to be configured in order to obtain a [`Uart`] structure that is 
/// actually usable for TX and/or RX communications.
/// 
/// Available ports: 0, 1, 2.
pub struct UartAccess<const PORT: u8>(pub(crate) ());

impl<const PORT: u8> UartAccess<PORT> {

    /// Configure this UART port for duplex communications.
    pub fn init_duplex<const TX_PIN: u8, const RX_PIN: u8>(self, 
        tx: impl Into<Pin<TX_PIN, Alternate>>,
        rx: impl Into<Pin<RX_PIN, Alternate>>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> Uart<PORT, Pin<TX_PIN, Alternate>, Pin<RX_PIN, Alternate>> {
        let mut tx = tx.into();
        let mut rx = rx.into();
        attach_pin(&mut tx, Self::port_tx_function());
        attach_pin(&mut rx, Self::port_rx_function());
        init::<PORT>(config, clocks, true, true);
        Uart { tx, rx }
    }

    /// Configure this UART port for TX-only communications.
    pub fn init_simplex_transmit<const TX_PIN: u8>(self, 
        tx: impl Into<Pin<TX_PIN, Alternate>>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> Uart<PORT, Pin<TX_PIN, Alternate>, ()> {
        let mut tx = tx.into();
        attach_pin(&mut tx, Self::port_tx_function());
        init::<PORT>(config, clocks, true, false);
        Uart { tx, rx: () }
    }

    /// Configure this UART port for RX-only communications.
    pub fn init_simplex_receive<const RX_PIN: u8>(self, 
        rx: impl Into<Pin<RX_PIN, Alternate>>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> Uart<PORT, (), Pin<RX_PIN, Alternate>> {
        let mut rx = rx.into();
        attach_pin(&mut rx, Self::port_rx_function());
        init::<PORT>(config, clocks, false, true);
        Uart { tx: (), rx }
    }

    fn port_tx_function() -> UartFunction {
        match PORT {
            0 => UartFunction::Uart0Tx,
            1 => UartFunction::Uart1Tx,
            2 => UartFunction::Uart2Tx,
            _ => unreachable!()
        }
    }

    fn port_rx_function() -> UartFunction {
        match PORT {
            0 => UartFunction::Uart0Rx,
            1 => UartFunction::Uart1Rx,
            2 => UartFunction::Uart2Rx,
            _ => unreachable!()
        }
    }

}


/// Internal trait used to dynamically know if a pin is present or not.
pub trait UartPin: Sealed {
    /// Return the pin number, if present.
    fn pin() -> Option<u8>;
}

/// Internal marker trait used to statically know if a pin is present.
pub trait UartPresentPin: UartPin {}

impl UartPin for () {
    #[inline]
    fn pin() -> Option<u8> {
        None
    }
}

impl<const NUM: u8> UartPin for Pin<NUM, Alternate> {
    #[inline]
    fn pin() -> Option<u8> {
        Some(NUM)
    }
}

impl<const NUM: u8> UartPresentPin for Pin<NUM, Alternate> {}


/// An initialized access to a UART port.
pub struct Uart<const PORT: u8, Tx: UartPin, Rx: UartPin> {
    tx: Tx,
    rx: Rx,
}

impl<const PORT: u8, Tx: UartPin, Rx: UartPin> Uart<PORT, Tx, Rx> {

    /// Downgrade this UART port and disable it. 
    pub fn downgrade(self) -> (UartAccess<PORT>, Tx, Rx) {
        // Drop will be called at the end, effectively closing the port.
        unsafe { (UartAccess(()), addr_of!(self.tx).read(), addr_of!(self.rx).read()) }
    }

}

impl<const PORT: u8, Tx: UartPin, Rx: UartPin> Drop for Uart<PORT, Tx, Rx> {

    fn drop(&mut self) {
        let regs = get_registers::<PORT>();
        regs.utx_cfg().modify(|reg| reg.en().clear());
        regs.urx_cfg().modify(|reg| reg.en().clear());
        Tx::pin().map(detach_pin);
        Rx::pin().map(detach_pin);
    }

}

impl<const PORT: u8, Tx: UartPresentPin, Rx: UartPin> UartTxDev for Uart<PORT, Tx, Rx> {

    fn write_byte(&mut self, byte: u8) {
        let regs = get_registers::<PORT>();
        while regs.fifo_cfg1().get().tx_fifo_count().get() == 0 {}
        regs.fifo_wdata().set(byte);
    }

}

/// Default implementation of write.
impl<const PORT: u8, Tx: UartPresentPin, Rx: UartPin> fmt::Write for Uart<PORT, Tx, Rx> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

impl<const PORT: u8, Tx: UartPin, Rx: UartPresentPin> UartRxDev for Uart<PORT, Tx, Rx> {

    fn read_byte(&mut self) -> Option<u8> {
        let regs = get_registers::<PORT>();
        if regs.fifo_cfg1().get().rx_fifo_count().get() != 0 {
            Some(regs.fifo_rdata().get())
        } else {
            None
        }
    }

}

impl<const PORT: u8, Tx: UartPin, Rx: UartPresentPin> DmaSrcEndpoint for Uart<PORT, Tx, Rx> {
    
    unsafe fn configure(&mut self) -> DmaEndpointConfig {
        
        // We configure the port to enable DMA for TX.
        let regs = get_registers::<PORT>();
        regs.fifo_cfg0().modify(|reg| reg.dma_rx_en().fill());

        DmaEndpointConfig {
            peripheral: Some(match PORT {
                0 => DmaPeripheral::Uart0Rx,
                1 => DmaPeripheral::Uart1Rx,
                2 => DmaPeripheral::Uart2Rx,
                _ => unreachable!()
            }),
            data_width: DmaDataWidth::Byte,
            burst_size: DmaBurstSize::Incr1,
            increment: DmaIncrement::Const,
            address: regs.fifo_rdata().0 as _
        }

    }

    fn close(&mut self) {
        get_registers::<PORT>().fifo_cfg0().modify(|reg| reg.dma_rx_en().clear());
    }

}

impl<const PORT: u8, Tx: UartPresentPin, Rx: UartPin> DmaDstEndpoint for Uart<PORT, Tx, Rx> {
    
    unsafe fn configure(&mut self) -> DmaEndpointConfig {
        
        // We configure the port to enable DMA for RX.
        let regs = get_registers::<PORT>();
        regs.fifo_cfg0().modify(|reg| reg.dma_tx_en().fill());

        DmaEndpointConfig {
            peripheral: Some(match PORT {
                0 => DmaPeripheral::Uart0Tx,
                1 => DmaPeripheral::Uart1Tx,
                2 => DmaPeripheral::Uart2Tx,
                _ => unreachable!()
            }),
            data_width: DmaDataWidth::Byte,
            burst_size: DmaBurstSize::Incr1,
            increment: DmaIncrement::Const,
            address: regs.fifo_wdata().0 as _
        }

    }

    fn close(&mut self) {
        get_registers::<PORT>().fifo_cfg0().modify(|reg| reg.dma_tx_en().clear());
    }

}

/// Return the UART registers for the given port.
#[inline]
fn get_registers<const PORT: u8>() -> UartRegs {
    match PORT {
        0 => UART0,
        1 => UART1,
        2 => UART2,
        _ => unreachable!()
    }
}

/// Get the UART MMIO registers structure associated to the given port.
fn init<const PORT: u8>(config: &UartConfig, clocks: &Clocks, enable_tx: bool, enable_rx: bool) {
    init_internal(get_registers::<PORT>(), config, clocks, enable_tx, enable_rx);
}

/// Internal function used to initialize the I/O given a configuration and clocks.
#[inline(never)]
fn init_internal(regs: UartRegs, config: &UartConfig, clocks: &Clocks, enable_tx: bool, enable_rx: bool) {

    // Calculate the baudrate divisor from UART frequency.
    let uart_freq = clocks.get_mcu_uart_freq();
    let div = (uart_freq * 10 / config.baudrate + 5) / 10;

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

    regs.int_mask().set(0xFFF);

    // Enable TX if a pin is attached.
    if enable_tx {
        regs.utx_cfg().modify(|reg| reg.en().fill());
    }

    // Enable RX if a pin is attached.
    if enable_rx {
        regs.urx_cfg().modify(|reg| reg.en().fill());
    }

}

/// Internal function to attach a pin to a specific UART function.
fn attach_pin<const NUM: u8>(pin: &mut Pin<NUM, Alternate>, func: UartFunction) {

    // There are 8 u32 fields per register
    let sig = NUM % 12;
    let reg = sig / 8;
    let field = (sig % 8) * 4;

    let mut cfg = GLB.uart_cfg1();
    cfg.0 = unsafe { cfg.0.add(reg as usize) };

    cfg.modify(|reg| {
        reg.0 &= !(0xF << field);
        reg.0 |= (func as u32) << field;
    });

    pin.modify_config(|cfg| {
        cfg.set_function(PinFunction::Uart);
        cfg.set_pull(PinPull::Up);
        cfg.set_drive(PinDrive::Drive1);
        cfg.set_input_enable(true);
        cfg.set_smt(true);
    });

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
