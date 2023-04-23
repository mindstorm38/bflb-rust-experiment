//! DMA (Direct Memory Access) peripherals.
//! 
//! Interesting post: https://blog.japaric.io/safe-dma/

use crate::bl808::{DMA0, DMA1, DMA2, dma};


/// Represent an exclusive access to a DMA channel on a particular port.
pub struct DmaAccess<const PORT: u8, const CHANNEL: u8>(pub(crate) ());

impl<const PORT: u8, const CHANNEL: u8> DmaAccess<PORT, CHANNEL> {

    /// Execute a new DMA transfer from the given source endpoint to the given
    /// destination endpoint. Endpoints are generic, see implementors of 
    /// [`DmaEndpoint`] for more information, note that supported peripherals
    /// depends on the `PORT` used.
    /// 
    /// The returned [`DmaTransfer`] handle can be used to wait for result
    /// and get back the source and destination endpoint in order to reuse 
    /// them.
    #[inline(never)]
    pub fn into_transfer<Src, Dst>(self, 
        mut src: Src,
        mut dst: Dst) -> DmaTransfer<PORT, CHANNEL, Src, Dst>
    where
        Src: DmaSrcEndpoint,
        Dst: DmaDstEndpoint,
    {

        let port_regs = get_port_regs::<PORT>();
        let channel_regs = get_channel_regs::<PORT, CHANNEL>();

        let src_config = src.configure();
        let dst_config = dst.configure();

        let transfer_len;
        let mut src_incr = false;
        let mut dst_incr = false;

        match (src_config.increment, dst_config.increment) {
            (DmaIncrement::Incr(src_len), DmaIncrement::Incr(dst_len)) => {
                assert_eq!(src_len, dst_len, "source and destination length must be equal");
                transfer_len = src_len;
                src_incr = true;
                dst_incr = true;
            }
            (DmaIncrement::Incr(src_len), DmaIncrement::Const) => {
                transfer_len = src_len;
                src_incr = true;
            }
            (DmaIncrement::Const, DmaIncrement::Incr(dst_len)) => {
                transfer_len = dst_len;
                dst_incr = true;
            }
            _ => {
                panic!("both source and destination have undetermined length");
            }
        }

        // TODO: Support for LLI transfers.
        assert!(transfer_len <= 4064, "doing more than 4064 transfers is currently not supported");

        // TODO: Guard this for concurrent calls.
        port_regs.config().modify(|reg| {
            reg.smdma_enable().fill();
        });

        // Temporarily disable the channel.
        channel_regs.config().modify(|reg| {
            reg.enable().clear();
        });

        // Configure control parameters.
        channel_regs.control().modify(|reg| {

            reg.src_increment().set(src_incr as _);
            reg.dst_increment().set(dst_incr as _);
            reg.src_burst_size().set(src_config.burst_size as _);
            reg.dst_burst_size().set(dst_config.burst_size as _);
            reg.src_width().set(src_config.data_width as _);
            reg.dst_width().set(dst_config.data_width as _);

            reg.dst_add_mode().clear();
            reg.dst_minus_mode().clear();

            reg.tc_int_enable().fill();
            reg.transfer_size().set(transfer_len as _);

        });

        channel_regs.config().modify(|reg| {

            if let Some(src) = src_config.peripheral {
                reg.dst_peripheral().set(get_peripheral_id::<PORT>(src));
            } else {
                reg.src_peripheral().clear();
            }

            if let Some(dst) = dst_config.peripheral {
                reg.dst_peripheral().set(get_peripheral_id::<PORT>(dst));
            } else {
                reg.dst_peripheral().clear();
            }

            reg.flow_control().set(match (src_config.peripheral, dst_config.peripheral) {
                (None, None) => 0,
                (None, Some(_)) => 1,
                (Some(_), None) => 2,
                (Some(_), Some(_)) => 3,
            });
            
            reg.lli_counter().clear();

        });

        // Enable DMA error and terminal count interrupt.
        channel_regs.config().modify(|reg| {
            reg.int_error_mask().fill();
            reg.int_tc_mask().fill();
        });

        channel_regs.src_addr().set(unsafe { src.ptr() } as _);
        channel_regs.dst_addr().set(unsafe { dst.ptr() } as _);

        // Clear interrupt related to this channel.
        port_regs.int_tc_clear().set_with(|reg| reg.set(CHANNEL, true));
        port_regs.int_error_clear().set_with(|reg| reg.set(CHANNEL, true));

        channel_regs.config().modify(|reg| {
            reg.enable().fill();
        });

        DmaTransfer {
            src,
            dst,
        }

    }

}


/// Represent a running DMA transfer.
pub struct DmaTransfer<const PORT: u8, const CHANNEL: u8, Src, Dst> {
    src: Src,
    dst: Dst,
}

impl<const PORT: u8, const CHANNEL: u8, Src, Dst> DmaTransfer<PORT, CHANNEL, Src, Dst>
where
    Src: DmaEndpoint,
    Dst: DmaEndpoint
{

    /// Return true if the transfer is completed and can be destructured
    #[inline]
    pub fn completed(&self) -> bool {
        get_port_regs::<PORT>().raw_int_tc_status().get().get(CHANNEL)
    }

    /// Try destructuring this transfer into its original components.
    /// 
    /// This will only succeed if the DMA transfer is completed ([`completed`]).
    pub fn try_destruct(self) -> Result<(Src, Dst, DmaAccess<PORT, CHANNEL>), Self> {
        if self.completed() {

            // The structure is droppped here, it's safe to return back the ownership 
            // of source and destination because we checked that the transfer has 
            // completed.
            let DmaTransfer { mut src, mut dst } = self;

            // Some endpoints need to be unconfigured after DMA transfer is complete.
            src.unconfigure();
            dst.unconfigure();

            // It's safe to create a DmaAccess because it has been consumed in order
            // to create the transfer which ensured exclusive access to DMA channel.
            Ok((src, dst, DmaAccess(())))

        } else {
            Err(self)
        }
    }

    /// Indefinitly wait for completion of this DMA transfer and then destruct the
    /// transfer into its original components. See [`try_destruct`].
    pub fn wait_destruct(self) -> (Src, Dst, DmaAccess<PORT, CHANNEL>) {
        let mut transfer = self;
        loop {
            transfer = match transfer.try_destruct() {
                Ok(fields) => return fields,
                Err(transfer) => transfer,
            };
        }
    }

}


/// Structure describing how an endpoint should be configured.
/// 
/// *Note that* this does not include the source and destination address.
#[derive(Debug, Clone)]
pub struct DmaEndpointConfig {
    /// Set to `None` if this endpoint is not a peripheral, but some 
    /// peripheral enumeration if this is one.
    /// 
    /// *Note that each DMA controller has its own subset of supported
    /// peripherals.*
    pub peripheral: Option<DmaPeripheral>,
    /// Data width of individual transfers of this endpoint.
    pub data_width: DmaDataWidth,
    /// Number of bytes transfered at once while the memory is owned by 
    /// the DMA transfer.
    pub burst_size: DmaBurstSize,
    /// When `Const` is returned, the length is undetermined and the DMA
    /// increment will be disabled. If the two endpoints have undetermined
    /// length, the transfer setup will panic.
    /// 
    /// When `Incr` is returned, the DMA increment will be enabled and the 
    /// length must be equal to the opposit endpoint, or the opposit must 
    /// return `Const` length. The given length tell how many transfers of 
    /// the given `data_width` to do.
    pub increment: DmaIncrement,
}

/// An abstract endpoint (source or destination) for DMA transfers.
/// 
/// The implementors have to define functions that provides the
/// address of the endpoint, increment enable, data width and burst
/// size. If the endpoint is a peripheral, it must also be given.
/// 
/// When DMA transfers data from a source to a destination. It first
/// copy data from source to an internal buffer and then copy back from
/// this buffer to the destination. This is why each endpoint defines
/// data width and burst size.
pub trait DmaEndpoint {

    /// This is called when the endpoint is about to be configured, it 
    /// also need to return the configuration to apply to the endpoint.
    /// 
    /// *Note that* this take an exclusive self reference, it's intended
    /// for use by some implementors to apply modifications to the
    /// endpoint instance before configuration.
    fn configure(&mut self) -> DmaEndpointConfig;

    /// Unconfigure this endpoint, this is called when the DMA transfer
    /// is deconstructed and endpoints are released.
    fn unconfigure(&mut self) {}

}

/// Specialized trait for source-capable DMA endpoints.
/// 
/// SAFETY: This trait is unsafe because **you must** ensure that the 
/// returned pointer lead to valid data regarding the configuration
/// returned by [`DmaEndpoint::configure`]. The pointed data is allowed
/// to be const referenced while the endpoint is configured.
pub unsafe trait DmaSrcEndpoint: DmaEndpoint {

    /// Get the pointer to the constant data of this endpoint.
    unsafe fn ptr(&self) -> *const ();

}

/// Specialized trait for destination-capable DMA endpoints.
/// 
/// SAFETY: This trait is unsafe because **you must** ensure that the 
/// returned pointer lead to valid data regarding the configuration
/// returned by [`DmaEndpoint::configure`]. The pointed data should
/// not be shared while the endpoint is configured to avoid concurrent
/// accesses and undefined behaviors.
pub unsafe trait DmaDstEndpoint: DmaEndpoint {

    /// Get the pointer to the mutable data of this endpoint.
    unsafe fn ptr(&self) -> *mut ();

}

/// Trait internally used and implemented by primitive integer types,
/// used for generic implementations of [`DmaEndpoint`].
pub trait DmaPrimitiveType {

    /// The data width to transfer the primitive type.
    fn data_width() -> DmaDataWidth;

    /// The burst size to transfer the primitive type.
    fn burst_size() -> DmaBurstSize;

}

/// Implementation for arrays.
impl<T: DmaPrimitiveType, const LEN: usize> DmaEndpoint for &'static [T; LEN] {

    fn configure(&mut self) -> DmaEndpointConfig {
        DmaEndpointConfig {
            peripheral: None,
            data_width: T::data_width(),
            burst_size: T::burst_size(),
            increment: DmaIncrement::Incr(LEN),
        }
    }

}

unsafe impl<T: DmaPrimitiveType, const LEN: usize> DmaSrcEndpoint for &'static [T; LEN] {
    unsafe fn ptr(&self) -> *const () {
        self.as_ptr() as _
    }
}

/// Implementation for slices.
impl<T: DmaPrimitiveType> DmaEndpoint for &'static [T] {

    fn configure(&mut self) -> DmaEndpointConfig {
        DmaEndpointConfig {
            peripheral: None,
            data_width: T::data_width(),
            burst_size: T::burst_size(),
            increment: DmaIncrement::Incr(self.len()),
        }
    }

}

unsafe impl<T: DmaPrimitiveType> DmaSrcEndpoint for &'static [T] {
    unsafe fn ptr(&self) -> *const () {
        self.as_ptr() as _
    }
}

/// Implementation for str slices.
impl DmaEndpoint for &'static str {

    fn configure(&mut self) -> DmaEndpointConfig {
        DmaEndpointConfig {
            peripheral: None,
            data_width: DmaDataWidth::Byte,
            burst_size: match self.len() {
                0..=1 => DmaBurstSize::Incr1,
                2..=7 => DmaBurstSize::Incr2,
                8..=15 => DmaBurstSize::Incr8,
                _ => DmaBurstSize::Incr16,
            },
            increment: DmaIncrement::Incr(self.len()),
        }
    }

}

unsafe impl DmaSrcEndpoint for &'static str {
    unsafe fn ptr(&self) -> *const () {
        self.as_ptr() as _
    }
}


// impl<T: DmaPrimitiveType> DmaEndpoint for Box<[T]> {

//     #[inline]
//     fn peripheral(&self) -> Option<DmaPeripheral> {
//         None // Not a peripheral
//     }

//     #[inline]
//     fn data_width(&self) -> DmaDataWidth {
//         T::data_width()
//     }

//     #[inline]
//     fn burst_size(&self) -> DmaBurstSize {
//         T::burst_size()
//     }

//     #[inline]
//     fn len(&self) -> DmaIncrement {
//         DmaIncrement::Incr((&**self).len())
//     }

// }

// impl<T: DmaPrimitiveType, const LEN: usize> DmaEndpoint for Box<[T; LEN]> {

//     #[inline]
//     fn peripheral(&self) -> Option<DmaPeripheral> {
//         None // Not a peripheral
//     }

//     #[inline]
//     fn data_width(&self) -> DmaDataWidth {
//         T::data_width()
//     }

//     #[inline]
//     fn burst_size(&self) -> DmaBurstSize {
//         T::burst_size()
//     }

//     #[inline]
//     fn len(&self) -> DmaIncrement {
//         DmaIncrement::Incr(LEN)
//     }

// }    


/// Define primitive implementations of `DmaEndpoint`.
macro_rules! impl_primitive_type {
    ($($ty:ty),+ = $data_width:ident, $burst_size:ident) => {
        
        $(impl DmaPrimitiveType for $ty {
        
            #[inline]
            fn data_width() -> DmaDataWidth {
                DmaDataWidth::$data_width
            }
        
            #[inline]
            fn burst_size() -> DmaBurstSize {
                DmaBurstSize::$burst_size
            }
        
        })+

    };

}

impl_primitive_type!(u8,  i8  = Byte,  Incr1);
impl_primitive_type!(u16, i16 = Hword, Incr2);
impl_primitive_type!(u32, i32 = Word,  Incr2);
impl_primitive_type!(u64, i64 = Dword, Incr8);


/// DMA data width. This represent how many bytes are copied on
/// each transfer. 
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

/// DMA burst count. This represent the amount of data the can
/// be transfered before released the memory bus. The values are
/// expressed in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaBurstSize {
    Incr1 = 0,
    Incr2 = 1,
    Incr8 = 2,
    Incr16 = 3,
}

/// DMA increment for transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaIncrement {
    /// The address doesn't change between transfers, the same address
    /// is used but for an unknown count. The count must be determined
    Const,
    /// The address is incremented between the given number of transfers.
    Incr(usize),
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


#[inline(always)]
fn get_port_regs<const PORT: u8>() -> dma::Dma {
    match PORT {
        0 => DMA0,
        1 => DMA1,
        2 => DMA2,
        _ => unreachable!()
    }
}


#[inline(always)]
fn get_channel_regs<const PORT: u8, const CHANNEL: u8>() -> dma::DmaChannel {
    get_port_regs::<PORT>().channel(CHANNEL as usize)
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
