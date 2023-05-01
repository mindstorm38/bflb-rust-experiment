//! DMA (Direct Memory Access) peripherals.
//! 
//! Interesting post: https://blog.japaric.io/safe-dma/

use core::task::{Poll, Context, Waker};
use core::future::Future;
use core::pin::Pin;

use spin::Mutex;

use crate::bl808::{DMA0, DMA1, DMA2, dma};


// TODO: Guard all access to the port registers to avoid concurrent
// uses.


/// Represent an exclusive access to a DMA channel on a particular DMA
/// port. This can be used to initiate a transfer.
pub struct DmaAccess<const PORT: u8, const CHANNEL: u8>(pub(crate) ());

impl<const PORT: u8, const CHANNEL: u8> DmaAccess<PORT, CHANNEL> {

    /// Execute a new DMA transfer from the given source endpoint to 
    /// the given destination endpoint. Endpoints are generic, see 
    /// implementors of [`DmaEndpoint`] for more information, note 
    /// that supported peripherals depends on the `PORT` used.
    /// 
    /// The returned [`DmaTransfer`] handle can be used to wait for 
    /// result and get back the source and destination endpoint in 
    /// order to reuse them.
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

        // Enable DMA error and terminal count interrupts.
        channel_regs.config().modify(|reg| {
            reg.int_error_mask().clear();
            reg.int_tc_mask().clear();
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


/// Represent a running DMA transfer that is currently running or
/// already finished. Once this transfer is done, it can be used to
/// retrieve the original source and destination endpoints to reuse
/// them.
pub struct DmaTransfer<const PORT: u8, const CHANNEL: u8, Src, Dst> {
    /// Source endpoint of the transfer.
    src: Src,
    /// Destination endpoint of the transfer.
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
        get_port_regs::<PORT>().int_tc_status().get().get(CHANNEL)
    }

    /// Internal function to destruct this transfer to its original
    /// components. This function is unsafe because you must ensure
    /// that the transfer is completed before destructing it. If it's
    /// not the case, the destination endpoint may be aliases by the
    /// DMA controller.
    /// 
    /// This function also release the endpoints and clear the
    /// associated interrupt.
    #[inline]
    unsafe fn destruct(mut self) -> (Src, Dst, DmaAccess<PORT, CHANNEL>) {
        
        get_port_regs::<PORT>().int_tc_clear()
            .set_with(|port| port.set(CHANNEL, true));
        
        self.src.release();
        self.dst.release();

        (self.src, self.dst, DmaAccess(()))

    }

    /// Try destructuring this transfer into its original components.
    /// 
    /// This will only succeed if the DMA transfer is completed ([`completed`]).
    pub fn try_wait(self) -> Result<(Src, Dst, DmaAccess<PORT, CHANNEL>), Self> {
        if self.completed() {
            // SAFETY: This is safe because we know that the transfer
            // is completed, so we can destruct.
            Ok(unsafe { self.destruct() })
        } else {
            Err(self)
        }
    }

    /// Indefinitely wait for completion of this DMA transfer and then 
    /// destruct the transfer into its original components. 
    /// See [`try_destruct`].
    pub fn wait_block(self) -> (Src, Dst, DmaAccess<PORT, CHANNEL>) {
        let mut transfer = self;
        loop {
            transfer = match transfer.try_wait() {
                Ok(fields) => return fields,
                Err(transfer) => transfer,
            };
        }
    }

    /// Wait for completion of this DMA transfer using a future, this
    /// can be awaited in an async context.
    /// 
    /// *This method is only available on the CPU type that supports
    /// interrupts for the current DMA port.*
    /// 
    /// This requires that source and destination endpoint are unpin,
    /// because the 
    pub fn wait(self) -> impl Future<Output = (Src, Dst, DmaAccess<PORT, CHANNEL>)>
    where
        Src: Unpin,
        Dst: Unpin,
        DmaPort<PORT>: DmaAsyncPort,
    {
        DmaTransferFuture {
            inner: Some(self),
        }
    }

}


/// Future type used to await a DMA transfer in an async context.
pub struct DmaTransferFuture<const PORT: u8, const CHANNEL: u8, Src, Dst> {
    /// Inner transfer, used to destruct the transfer, when None it
    /// cannot be polled again.
    inner: Option<DmaTransfer<PORT, CHANNEL, Src, Dst>>,
}

impl<const PORT: u8, const CHANNEL: u8, Src, Dst> Future for DmaTransferFuture<PORT, CHANNEL, Src, Dst>
where
    Src: DmaEndpoint + Unpin,
    Dst: DmaEndpoint + Unpin,
    DmaPort<PORT>: DmaAsyncPort,
{

    type Output = (Src, Dst, DmaAccess<PORT, CHANNEL>);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

        // We can unwrap because calling poll on an already-ready
        // future should not happen.
        if !self.inner.as_ref().unwrap().completed() {

            critical_section::with(|_| {

                // SAFETY: We use a critical section to spin lock the
                // wakers to avoid dead locking in case of interrupts 
                // while locking.
                <DmaPort<PORT> as sealed::DmaAsyncPortWakers>::with_wakers(|wakers| {
                    wakers[CHANNEL as usize] = Some(cx.waker().clone());
                });

                Poll::Pending
                
            })

        } else {
            // If this is completed.
            Poll::Ready(unsafe { self.inner.take().unwrap().destruct() })
        }
        
    }

}


/// Used to initialize the wakers arrays.
const DMA_WAKER_INIT: Option<Waker> = None;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
static DMA0_WAKERS: Mutex<[Option<Waker>; 8]> = Mutex::new([DMA_WAKER_INIT; 8]);

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
static DMA1_WAKERS: Mutex<[Option<Waker>; 4]> = Mutex::new([DMA_WAKER_INIT; 4]);

#[cfg(feature = "bl808-d0")]
static DMA2_WAKERS: Mutex<[Option<Waker>; 8]> = Mutex::new([DMA_WAKER_INIT; 8]);

/// Internal generic handler for DMA ports. This is internally only
/// called from interrupt handlers.
fn dma_handler<const PORT: u8>()
where
    DmaPort<PORT>: DmaAsyncPort,
{

    // Get the status and clear all status.
    let status = get_port_regs::<PORT>().int_tc_status().get();

    // SAFETY: We can spin lock the wakers because we are in an 
    // interrupt and we cannot be deadlocked by another interrupt.
    <DmaPort<PORT> as sealed::DmaAsyncPortWakers>::with_wakers(|wakers| {
        for (i, waker) in wakers.iter_mut().enumerate() {
            if status.get(i as u8) {
                if let Some(waker) = waker.take() {
                    waker.wake();
                }
            }
        }
    });

}

/// Interrupt handler for DMA0 interrupts on M0/LP.
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub(crate) fn dma0_handler(_code: usize) {
    dma_handler::<0>();
}

/// Interrupt handler for DMA1 interrupts on M0/LP.
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub(crate) fn dma1_handler(_code: usize) {
    dma_handler::<1>();
}

/// Interrupt handler for DMA1 interrupts on M0/LP.
#[cfg(feature = "bl808-d0")]
pub(crate) fn dma2_handler(_code: usize) {
    dma_handler::<2>();
}


/// Internal module used for sealed traits.
mod sealed {

    use super::Waker;

    /// Internal trait that allows modifying wakers of a particular port.
    pub trait DmaAsyncPortWakers {
        /// Spin lock the wakers for this port and run a function with the
        /// wakers array, **you must** ensure that the spin lock is called
        /// in a safe manner regarding interrupts.
        fn with_wakers<T, F: FnOnce(&mut [Option<Waker>]) -> T>(func: F) -> T;
    }

}

/// A trait internally used to constrain the possible DMA ports
/// available for the currently selected chip.
pub trait DmaAsyncPort: sealed::DmaAsyncPortWakers {}
pub struct DmaPort<const PORT: u8>;

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
impl sealed::DmaAsyncPortWakers for DmaPort<0> {
    fn with_wakers<T, F: FnOnce(&mut [Option<Waker>]) -> T>(func: F) -> T {
        func(&mut DMA0_WAKERS.lock()[..])
    }
}
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
impl sealed::DmaAsyncPortWakers for DmaPort<1> {
    fn with_wakers<T, F: FnOnce(&mut [Option<Waker>]) -> T>(func: F) -> T {
        func(&mut DMA1_WAKERS.lock()[..])
    }
}
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
impl DmaAsyncPort for DmaPort<0> {}
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
impl DmaAsyncPort for DmaPort<1> {}

#[cfg(feature = "bl808-d0")]
impl sealed::DmaAsyncPortWakers for DmaPort<2> {
    fn with_wakers<T, F: FnOnce(&mut [Option<Waker>]) -> T>(func: F) -> T {
        func(&mut DMA2_WAKERS.lock()[..])
    }
}
#[cfg(feature = "bl808-d0")]
impl DmaAsyncPort for DmaPort<2> {}


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

    /// Release this endpoint from a DMA transfer, this is called when
    /// the DMA transfer is deconstructed and both endpoints are 
    /// released.
    fn release(&mut self) {}

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


/// Implementation for array slices with compile-time length.
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

/// Implementation for array slices with runtime length.
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

/// Implementation for string slices.
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


/// Internal macro used to define DMA primitive integer types, these
/// are used to implement [`DmaEndpoint`] on generic arrays and slices
/// for example.
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
/// be transferred before released the memory bus. The values are
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
    /// is used but for an unknown count. If an endpoint returns a
    /// constant address, the opposite endpoint should specify a know
    /// increment size.
    Const,
    /// The DMA controller will increment the address of the endpoint
    /// to run the given number of transfers. Each transfer has the
    /// size of the configured [`DmaDataWidth`].
    Incr(usize),
}

/// DMA peripheral available for configuration of a channel. Some
/// peripherals are not possible for some DMA ports.
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
