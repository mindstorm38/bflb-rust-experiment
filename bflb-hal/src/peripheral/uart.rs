//! Serial I/O management on BL808.

use core::marker::PhantomData;
use core::fmt;
use core::ptr::addr_of;

use embedded_util::peripheral;

use crate::bl808::{Uart as UartRegs, GLB, UART0, UART1, UART2};
use crate::bl808::uart::UartBitPrd;

use crate::gpio::{Pin, PinPull, PinDrive, PinFunction, Alternate};
use crate::dma::{DmaEndpoint, DmaSrcEndpoint, DmaDstEndpoint, 
    DmaEndpointConfig, DmaPeripheral, DmaDataWidth, DmaBurstSize, 
    DmaIncrement};
use crate::clock::Clocks;


/// Definition of an exclusive access to a UART port. This port need 
/// to be configured in order to obtain a [`Uart`] structure that is 
/// actually usable for TX and/or RX communications.
/// 
/// Available ports: 0, 1, 2.
pub struct UartAccess<const PORT: u8>(());

impl<const PORT: u8> UartAccess<PORT> {

    peripheral!(array: PORT[0..3]);

    /// Generic types erasing, this transfer checks to the runtime.
    pub fn erase(self) -> ! {
        todo!()
    }

    /// Configure this UART port for duplex communications.
    pub fn into_duplex<const TX_PIN: u8, const RX_PIN: u8>(self, 
        mut tx: Pin<TX_PIN, Alternate>,
        mut rx: Pin<RX_PIN, Alternate>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> (UartTx<PORT, TX_PIN, Duplex>, UartRx<PORT, RX_PIN, Duplex>) {

        attach_pin(&mut tx, match PORT {
            0 => UartFunction::Uart0Tx,
            1 => UartFunction::Uart1Tx,
            2 => UartFunction::Uart2Tx,
            _ => unreachable!()
        });

        attach_pin(&mut rx, match PORT {
            0 => UartFunction::Uart0Rx,
            1 => UartFunction::Uart1Rx,
            2 => UartFunction::Uart2Rx,
            _ => unreachable!()
        });

        init::<PORT>(config, clocks, true, true);

        (
            UartTx { pin: tx, _origin: PhantomData }, 
            UartRx { pin: rx, _origin: PhantomData },
        )

    }

    /// Configure this UART port for TX-only communications.
    pub fn into_tx<const TX_PIN: u8>(self, 
        mut tx: Pin<TX_PIN, Alternate>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> UartTx<PORT, TX_PIN, SingleTx> {

        attach_pin(&mut tx, match PORT {
            0 => UartFunction::Uart0Tx,
            1 => UartFunction::Uart1Tx,
            2 => UartFunction::Uart2Tx,
            _ => unreachable!()
        });

        init::<PORT>(config, clocks, true, false);
        UartTx { pin: tx, _origin: PhantomData }

    }

    /// Configure this UART port for RX-only communications.
    pub fn into_rx<const RX_PIN: u8>(self, 
        mut rx: Pin<RX_PIN, Alternate>,
        config: &UartConfig, 
        clocks: &Clocks
    ) -> UartRx<PORT, RX_PIN, SingleRx> {

        attach_pin(&mut rx, match PORT {
            0 => UartFunction::Uart0Rx,
            1 => UartFunction::Uart1Rx,
            2 => UartFunction::Uart2Rx,
            _ => unreachable!()
        });

        init::<PORT>(config, clocks, false, true);
        UartRx { pin: rx, _origin: PhantomData }

    }

    /// Reconstruct a UART access from a single TX lane.
    pub fn from_tx<const TX_PIN: u8>(
        tx: UartTx<PORT, TX_PIN, SingleTx>
    ) -> (Self, Pin<TX_PIN, Alternate>) {

        // FIXME: These fixes using read are actually valid since the Pin structure is not
        // read in the drop implementation of UartTx (also ZST), so we kind of move it 
        // before drop. I really need to come with a clearer way of doing this, without
        // runtime overhead.
        (Self(()), unsafe { addr_of!(tx.pin).read() })

    }

    /// Reconstruct a UART access from a single RX lane.
    pub fn from_rx<const RX_PIN: u8>(
        rx: UartRx<PORT, RX_PIN, SingleRx>
    ) -> (Self, Pin<RX_PIN, Alternate>) {
        (Self(()), unsafe { addr_of!(rx.pin).read() })
    }

    /// Reconstruct a UART access from duplex lanes.
    pub fn from_duplex<const TX_PIN: u8, const RX_PIN: u8>(
        tx: UartTx<PORT, TX_PIN, Duplex>,
        rx: UartRx<PORT, RX_PIN, Duplex>,
    ) -> (Self, Pin<TX_PIN, Alternate>, Pin<RX_PIN, Alternate>) {
        (Self(()), unsafe { addr_of!(tx.pin).read() }, unsafe { addr_of!(rx.pin).read() })
    }

}


/// Define the UART Tx, transmission lane. It typically provides write methods
/// and implement DMA destination endpoint.
pub struct UartTx<const PORT: u8, const PIN: u8, O: UartOrigin> {
    pin: Pin<PIN, Alternate>,
    _origin: PhantomData<O>
}

/// Define the UART Rx, receiving lane. It typically provides read methods and
/// implement DMA source endpoint.
pub struct UartRx<const PORT: u8, const PIN: u8, O: UartOrigin> {
    pin: Pin<PIN, Alternate>,
    _origin: PhantomData<O>
}


/// Marker trait specifying origin of UART lanes, used when reconstructing.
pub trait UartOrigin {}

pub struct Duplex;
pub struct SingleTx;
pub struct SingleRx;
impl UartOrigin for Duplex {}
impl UartOrigin for SingleTx {}
impl UartOrigin for SingleRx {}


impl<const PORT: u8, const PIN: u8, O: UartOrigin> UartTx<PORT, PIN, O> {

    /// Simplest function to write a single byte of data to the UART TX.
    #[inline(never)]
    pub fn write_byte(&mut self, byte: u8) {
        let regs = get_registers::<PORT>();
        while regs.fifo_cfg1().get().tx_fifo_count().get() == 0 {}
        regs.fifo_wdata().set(byte);
    }
    
}

impl<const PORT: u8, const PIN: u8, O: UartOrigin> fmt::Write for UartTx<PORT, PIN, O> {

    #[inline(never)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }

}


impl<const PORT: u8, const PIN: u8, O: UartOrigin> UartRx<PORT, PIN, O> {

    /// Simplest function to read a single byte, if available.
    #[inline(never)]
    pub fn read_byte(&mut self) -> Option<u8> {
        let regs = get_registers::<PORT>();
        if regs.fifo_cfg1().get().rx_fifo_count().get() != 0 {
            Some(regs.fifo_rdata().get())
        } else {
            None
        }
    }

}

impl<const PORT: u8, const PIN: u8, O: UartOrigin> Drop for UartTx<PORT, PIN, O> {
    fn drop(&mut self) {
        let regs = get_registers::<PORT>();
        regs.utx_cfg().modify(|reg| reg.en().clear());
        detach_pin(PIN);
    }
}

impl<const PORT: u8, const PIN: u8, O: UartOrigin> Drop for UartRx<PORT, PIN, O> {
    fn drop(&mut self) {
        let regs = get_registers::<PORT>();
        regs.urx_cfg().modify(|reg| reg.en().clear());
        detach_pin(PIN);
    }
}


// For DMA support

impl<const PORT: u8, const PIN: u8, O: UartOrigin> DmaEndpoint for UartTx<PORT, PIN, O> {

    fn configure(&mut self) -> DmaEndpointConfig {

        // We configure the port to enable DMA for TX.
        get_registers::<PORT>().fifo_cfg0().modify(|reg| reg.dma_tx_en().fill());

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
        }

    }

    fn unconfigure(&mut self) {
        get_registers::<PORT>().fifo_cfg0().modify(|reg| reg.dma_tx_en().clear());
    }

}

unsafe impl<const PORT: u8, const PIN: u8, O: UartOrigin> DmaDstEndpoint for UartTx<PORT, PIN, O> {
    unsafe fn ptr(&self) -> *mut () {
        get_registers::<PORT>().fifo_wdata().0 as _
    }
}

impl<const PORT: u8, const PIN: u8, O: UartOrigin> DmaEndpoint for UartRx<PORT, PIN, O> {

    fn configure(&mut self) -> DmaEndpointConfig {

        // We configure the port to enable DMA for RX.
        get_registers::<PORT>().fifo_cfg0().modify(|reg| reg.dma_rx_en().fill());

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
        }

    }

    fn unconfigure(&mut self) {
        get_registers::<PORT>().fifo_cfg0().modify(|reg| reg.dma_rx_en().clear());
    }

}

unsafe impl<const PORT: u8, const PIN: u8, O: UartOrigin> DmaSrcEndpoint for UartRx<PORT, PIN, O> {
    unsafe fn ptr(&self) -> *const () {
        get_registers::<PORT>().fifo_rdata().0 as _
    }
}

// Utils

fn get_registers<const PORT: u8>() -> UartRegs {
    match PORT {
        0 => UART0,
        1 => UART1,
        2 => UART2,
        _ => unreachable!()
    }
}

/// Get the UART MMIO registers structure associated to the given port.
fn init<const PORT: u8>(config: &UartConfig, clocks: &Clocks, tx: bool, rx: bool) {
    init_internal(get_registers::<PORT>(), config, clocks, tx, rx);
}

/// Internal function used to initialize the I/O given a configuration and clocks.
#[inline(never)]
fn init_internal(regs: UartRegs, config: &UartConfig, clocks: &Clocks, tx: bool, rx: bool) {

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
    if tx {
        regs.utx_cfg().modify(|reg| reg.en().fill());
    }

    // Enable RX if a pin is attached.
    if rx {
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
