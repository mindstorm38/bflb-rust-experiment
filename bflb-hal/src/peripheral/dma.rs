//! DMA (Direct Memory Access) peripherals.

use core::sync::atomic::AtomicBool;

use crate::bl808::{DMA0, DMA1, DMA2, dma};

use embedded_util::{Peripheral, atomic};


/// Represent an exclusive access to a DMA channel on a particular port.
pub struct DmaAccess<const PORT: u8, const CHANNEL: u8>(());

impl<const PORT: u8, const CHANNEL: u8> Peripheral for DmaAccess<PORT, CHANNEL> {
    
    unsafe fn taken() -> &'static AtomicBool {

        debug_assert!(PORT < 3, "invalid dma port {PORT}");
        debug_assert!(CHANNEL < (if PORT == 1 { 4 } else { 8 }), "invalid dma channel {CHANNEL} for port {PORT}");

        static TAKEN_ARR: [AtomicBool; 20] = atomic::atomic_bool_array(false);

        match PORT {
            0 => &TAKEN_ARR[0  + CHANNEL as usize],
            2 => &TAKEN_ARR[8  + CHANNEL as usize],
            1 => &TAKEN_ARR[16 + CHANNEL as usize],
            _ => unreachable!()
        }

    }

    unsafe fn new() -> Self {
        Self(())
    }

}


impl<const PORT: u8, const CHANNEL: u8> DmaAccess<PORT, CHANNEL> {

    /// Configure and get a channel from this DMA access.
    pub fn into_channel(self, config: &DmaConfig) -> DmaChannel<PORT, CHANNEL> {
        
        let channel = DmaChannel(());

        let port_regs = channel.get_port_regs();
        let channel_regs = channel.get_channel_regs();

        // TODO: Guard this call from being called.
        port_regs.config().modify(|reg| {
            reg.smdma_enable().fill();
        });

        // Temporarily disable the channel.
        channel_regs.config().modify(|reg| {
            reg.enable().clear();
        });

        // Configure control parameters.
        channel_regs.control().modify(|reg| {
            reg.src_increment().set(config.src.incr as _);
            reg.dst_increment().set(config.dst.incr as _);
            reg.src_burst_size().set(config.src.burst_size as _);
            reg.dst_burst_size().set(config.dst.burst_size as _);
            reg.src_width().set(config.src.data_width as _);
            reg.dst_width().set(config.dst.data_width as _);
        });

        channel_regs.config().modify(|reg| {

            match config.direction {
                DmaDirection::MemoryToMemory => {
                    reg.src_peripheral().clear();
                    reg.dst_peripheral().clear();
                    reg.flow_control().set(0);
                }
                DmaDirection::MemoryToPeripheral(dst) => {
                    reg.src_peripheral().clear();
                    reg.dst_peripheral().set(get_peripheral_id::<PORT>(dst));
                    reg.flow_control().set(1);
                }
                DmaDirection::PeripheralToMemory(src) => {
                    reg.src_peripheral().set(get_peripheral_id::<PORT>(src));
                    reg.dst_peripheral().clear();
                    reg.flow_control().set(2);
                }
                DmaDirection::PeripheralToPeripheral(src, dst) => {
                    reg.src_peripheral().set(get_peripheral_id::<PORT>(src));
                    reg.dst_peripheral().set(get_peripheral_id::<PORT>(dst));
                    reg.flow_control().set(3);
                }
            }

            reg.lli_counter().clear();

        });

        // Enable DMA error and terminal count interrupt.
        channel_regs.config().modify(|reg| {
            reg.int_error_mask().fill();
            reg.int_tc_mask().fill();
        });

        channel_regs.src_addr().set(config.src.addr);
        channel_regs.dst_addr().set(config.dst.addr);

        channel_regs.control().modify(|reg| {
            reg.tc_int_enable().clear();
        });

        port_regs.int_tc_clear().set_with(|reg| reg.set(CHANNEL, true));
        port_regs.int_error_clear().set_with(|reg| reg.set(CHANNEL, true));

        // TODO: IRQ

        channel

    }

}


/// A configured DMA channel.
pub struct DmaChannel<const PORT: u8, const CHANNEL: u8>(());

impl<const PORT: u8, const CHANNEL: u8> DmaChannel<PORT, CHANNEL> {

    #[inline]
    fn get_port_regs(&self) -> dma::Dma {
        match PORT {
            0 => DMA0,
            1 => DMA1,
            2 => DMA2,
            _ => unreachable!()
        }
    }

    #[inline]
    fn get_channel_regs(&self) -> dma::DmaChannel {
        self.get_port_regs().channel(CHANNEL as usize)
    }

    pub fn start(&mut self) {
        self.get_channel_regs().config().modify(|reg| {
            reg.enable().fill();
        });
    }

    pub fn stop(&mut self) {
        self.get_channel_regs().config().modify(|reg| {
            reg.enable().clear();
        });
    }

    pub fn busy(&self) -> bool {
        self.get_channel_regs().config().get().enable().get() != 0
    }

}


/// Internal function to get a peripheral numeric identifier corresponding
/// to the given peripheral and port. Not all peripheral are available for
/// each port.
fn get_peripheral_id<const PORT: u8>(peripheral: DmaPeripheral) -> u32 {
    use DmaPeripheral::*;
    if PORT == 0 || PORT == 1 {
        match peripheral {
            Uart0Rx => 0,
            Uart0Tx => 1,
            Uart1Rx => 2,
            Uart1Tx => 3,
            Uart2Rx => 4,
            Uart2Tx => 5,
            I2c0Rx => 6,
            I2c0Tx => 7,
            IrTx => 8,
            GpioTx => 9,
            Spi0Rx => 10,
            Spi0Tx => 11,
            AudioRx => 12,
            AudioTx => 13,
            I2c1Rx => 14,
            I2c1Tx => 15,
            I2sRx => 16,
            I2sTx => 17,
            Pdm => 18,
            AdcRx => 22,
            AdcTx => 23,
            _ => panic!("invalid peripheral for port {PORT}")
        }
    } else if PORT == 2 {
        match peripheral {
            Uart3Rx => 0,
            Uart3Tx => 1,
            Spi1Rx => 2,
            Spi1Tx => 3,
            I2c2Rx => 6,
            I2c2Tx => 7,
            I2c3Rx => 8,
            I2c3Tx => 9,
            DsiRx => 10,
            DsiTx => 11,
            DbiTx => 22,
            _ => panic!("invalid peripheral for port {PORT}")
        }
    } else {
        panic!("invalid port")
    }
}


/// Configuration structure for DMA channel initialization.
#[derive(Debug, Clone)]
pub struct DmaConfig {
    /// The direction of the channel.
    pub direction: DmaDirection,
    /// Source endpoint configuration.
    pub src: DmaEndpointConfig,
    /// Destination endpoint configuration.
    pub dst: DmaEndpointConfig,
    /// Length of the transfer, between 0 and 4095.
    pub size: u16,
}

/// Configuration structure for source or destination endpoint
/// of a DMA channel.
#[derive(Debug, Clone)]
pub struct DmaEndpointConfig {
    /// The address of the endpoint.
    pub addr: u32,
    /// Enable increment of the address after each transfer.
    pub incr: bool,
    /// Burst size for this endpoint.
    pub burst_size: DmaBurstSize,
    /// Data with for transfers.
    pub data_width: DmaDataWidth,
}

/// DMA direction of transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDirection {
    MemoryToMemory,
    MemoryToPeripheral(DmaPeripheral),
    PeripheralToMemory(DmaPeripheral),
    PeripheralToPeripheral(DmaPeripheral, DmaPeripheral),
}

/// DMA peripheral available for configuration of a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaPeripheral {
    Uart0Rx,
    Uart0Tx,
    Uart1Rx,
    Uart1Tx,
    Uart2Rx,
    Uart2Tx,
    Uart3Rx,
    Uart3Tx,
    I2c0Rx,
    I2c0Tx,
    I2c1Rx,
    I2c1Tx,
    I2c2Rx,
    I2c2Tx,
    I2c3Rx,
    I2c3Tx,
    IrTx,
    GpioTx,
    Spi0Rx,
    Spi0Tx,
    Spi1Rx,
    Spi1Tx,
    AudioRx,
    AudioTx,
    I2sRx,
    I2sTx,
    Pdm,
    AdcRx,
    AdcTx,
    DsiRx,
    DsiTx,
    DbiTx,
}

/// DMA data width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaDataWidth {
    /// 8 bits.
    Byte = 0,
    /// 16 bits.
    Hword = 1,
    /// 32 bits.
    Word = 2,
    /// 64 bits.
    Dword = 3,
}

/// DMA burst count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaBurstSize {
    Incr1 = 0,
    Incr2 = 1,
    Incr8 = 2,
    Incr16 = 3,
}
