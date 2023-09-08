//! DMA (Direct Memory Access) peripherals.
//! 
//! Interesting post: https://blog.japaric.io/safe-dma/
//!
//!  TODO: Guard all access to the port registers to avoid concurrent
//! uses.

use core::cell::RefCell;
use alloc::boxed::Box;

use critical_section::{Mutex, CriticalSection};

use crate::arch::bl808::{DMA0, DMA1, DMA2, dma};
use crate::cache::{invalidate_data_range, clean_data_range};


/// This peripheral structure wraps all DMA ports available.
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

/// This peripheral structure wrap channels of DMA port 2.
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

impl Dma {

    pub(crate) const fn new() -> Self {
        Self {
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
            }
        }
    }

}

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

        let src_config = unsafe { src.configure() };
        let dst_config = unsafe { dst.configure() };

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

        let port_regs = get_port_regs::<PORT>();
        let channel_regs = get_channel_regs::<PORT, CHANNEL>();

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

            reg.transfer_size().set(transfer_len as _);

            reg.tc_int_enable().fill();

        });

        channel_regs.config().modify(|reg| {

            if let Some(src) = src_config.peripheral {
                reg.src_peripheral().set(get_peripheral_id::<PORT>(src));
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

        // Interrupts are masked by default because we are in manual waiting mode.
        // This will be modified if callback-based waiting is later used.
        channel_regs.config().modify(|reg| {
            reg.int_error_mask().fill();
            reg.int_tc_mask().fill();
        });

        channel_regs.src_addr().set(src_config.address as _);
        channel_regs.dst_addr().set(dst_config.address as _);

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
    Src: DmaSrcEndpoint,
    Dst: DmaDstEndpoint
{

    /// Return true if the transfer is completed and can be destructured.
    #[inline]
    pub fn completed(&self) -> bool {
        // We know that the channel is interrupt-masked, so we must use raw register.
        get_port_regs::<PORT>().raw_int_tc_status().get().get(CHANNEL)
    }

    /// Internal function to destruct this transfer to its original
    /// components. This function is unsafe because you must ensure
    /// that the transfer is completed before destructing it. If it's
    /// not the case, the destination endpoint may be aliases by the
    /// DMA controller.
    /// 
    /// This function also release the endpoints and disabled the channel.
    /// 
    /// SAFETY: Caller must ensure that the source and destination endpoint will no longer
    /// be accessed in any way by the DMA controller.
    #[inline]
    unsafe fn destruct(mut self) -> (Src, Dst, DmaAccess<PORT, CHANNEL>) {

        // Disabling the channel ofc...
        get_channel_regs::<PORT, CHANNEL>()
            .config().modify(|reg| {
                reg.enable().clear();
            });

        self.src.close();
        self.dst.close();

        (self.src, self.dst, DmaAccess(()))

    }

    /// Try destructuring this transfer into its original components.
    /// 
    /// This will only succeed if the DMA transfer is completed ([`completed`]).
    pub fn try_wait(self) -> Result<(Src, Dst, DmaAccess<PORT, CHANNEL>), Self> {
        if self.completed() {
            // We know that this channel is interrupt-masked, so it should not generate
            // interrupts. So we have to manually clear the terminal count bit here.
            get_port_regs::<PORT>().int_tc_clear()
                .set_with(|port| port.set(CHANNEL, true));
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
    pub fn wait(self) -> (Src, Dst, DmaAccess<PORT, CHANNEL>) {
        let mut transfer = self;
        loop {
            transfer = match transfer.try_wait() {
                Ok(fields) => return fields,
                Err(transfer) => transfer,
            };
        }
    }

    /// Wait for completion of this DMA transfer, calling the given callback when 
    /// completed.
    /// 
    /// Additional constraints are put on source and destination: they need to be [`Send`]
    /// with a static lifetime because they need to be saved appart in a static location.
    /// 
    /// *This method is only available on the CPU type that supports
    /// interrupts for the current DMA port.*
    pub fn wait_callback<F>(self, callback: F)
    where
        DmaAccess<PORT, CHANNEL>: DmaInterruptSupport,
        F: FnOnce(Src, Dst, DmaAccess<PORT, CHANNEL>) + Send + 'static,
        Self: Send + 'static,
    {

        // The closure here are a bit tricky, because both 'self' and 'callback' are 
        // destructured by this closure, so this closure is forced to be an FnOnce.
        // PS: This closure will be called upon interrupt.
        let mut once_wrapper = Some(move || {
            let (src, dst, access) = unsafe { self.destruct() };
            callback(src, dst, access)
        });

        // The problem is that we need this to be an FnMut in order to be abstracted in
        // our static callbacks array, so we put it in an option so we can take it.
        let wrapper: DmaCallback = Box::new(move || {
            if let Some(once_wrapper) = once_wrapper.take() {
                once_wrapper();
            }
        });

        critical_section::with(|cs| {

            <DmaAccess<PORT, CHANNEL> as DmaInterruptSupport>
                ::with_callback(move |slot| {
                    *slot = Some(wrapper);
                }, cs);
            
            // Unmask interrupt for this channel so it will now generate interrupts.
            get_channel_regs::<PORT, CHANNEL>().config()
                .modify(|config| config.int_tc_mask().clear());

        });
        
    }

}


/// Type alias for a boxed closure used as a DMA transfer callback.
type DmaCallback = Box<dyn FnMut() + Send>;
/// Internal type alias for a callbacks array.
type DmaCallbacks<const CHANNELS: usize> = [Mutex<RefCell<Option<DmaCallback>>>; CHANNELS];
/// Default value: no callback.
const NO_CALLBACK: Mutex<RefCell<Option<DmaCallback>>> = Mutex::new(RefCell::new(None));
/// Callbacks for DMA port 0.
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
static DMA0_CALLBACKS: DmaCallbacks<8> = [NO_CALLBACK; 8];
/// Callbacks for DMA port 1.
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
static DMA1_CALLBACKS: DmaCallbacks<4> = [NO_CALLBACK; 4];
/// Callbacks for DMA port 2.
#[cfg(feature = "bl808-d0")]
static DMA2_CALLBACKS: DmaCallbacks<8> = [NO_CALLBACK; 8];


/// Trait implemented on DMA ports that support interrupts on the current chip.
pub trait DmaInterruptSupport {
    /// SAFETY: Caller must ensure that this function is called in an interrupt-free 
    /// context and is the only possible owner of the DMA channel.
    fn with_callback(func: impl FnOnce(&mut Option<DmaCallback>), cs: CriticalSection);
}

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
impl<const CHANNEL: u8> DmaInterruptSupport for DmaAccess<0, CHANNEL> {
    fn with_callback(func: impl FnOnce(&mut Option<DmaCallback>), cs: CriticalSection) {
        func(&mut DMA0_CALLBACKS[CHANNEL as usize].borrow_ref_mut(cs));
    }
}

#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
impl<const CHANNEL: u8> DmaInterruptSupport for DmaAccess<1, CHANNEL> {
    fn with_callback(func: impl FnOnce(&mut Option<DmaCallback>), cs: CriticalSection) {
        func(&mut DMA1_CALLBACKS[CHANNEL as usize].borrow_ref_mut(cs));
    }
}

#[cfg(feature = "bl808-d0")]
impl<const CHANNEL: u8> DmaInterruptSupport for DmaAccess<2, CHANNEL> {
    fn with_callback(func: impl FnOnce(&mut Option<DmaCallback>), cs: CriticalSection) {
        func(&mut DMA2_CALLBACKS[CHANNEL as usize].borrow_ref_mut(cs));
    }
}


/// Internal generic handler for DMA ports. This handler should be called only for DMA
/// channels on which `wait_callback` has been called (so with unmasked interrupt).
#[inline(never)]
fn dma_handler(port_regs: dma::Dma, callbacks: &[Mutex<RefCell<Option<DmaCallback>>>], cs: CriticalSection) {

    // Get the status and clear all status.
    let status = port_regs.int_tc_status().get();
    port_regs.int_tc_clear().set(status);

    // Iterate over all callbacks and check if the corresponding interrupt bit has been
    // set, then we remove the callback and call it.
    for (i, callback) in callbacks.iter().enumerate() {
        if status.get(i as u8) {
            if let Some(mut callback) = callback.borrow_ref_mut(cs).take() {
                // The callback will destruct and close the DMA channel, it's safe because
                // we previously cleared the terminal count interrupt so it should not
                // spin interrupt.
                callback();
            }
        }
    }

}

/// Interrupt handler for DMA0 interrupts on M0/LP.
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub(crate) fn dma0_handler(_code: usize, cs: CriticalSection) {
    dma_handler(get_port_regs::<0>(), &DMA0_CALLBACKS[..], cs);
}

/// Interrupt handler for DMA1 interrupts on M0/LP.
#[cfg(any(feature = "bl808-m0", feature = "bl808-lp"))]
pub(crate) fn dma1_handler(_code: usize, cs: CriticalSection) {
    dma_handler(get_port_regs::<1>(), &DMA1_CALLBACKS[..], cs);
}

/// Interrupt handler for DMA1 interrupts on M0/LP.
#[cfg(feature = "bl808-d0")]
pub(crate) fn dma2_handler(_code: usize, cs: CriticalSection) {
    dma_handler(get_port_regs::<2>(), &DMA2_CALLBACKS[..], cs);
}


/// Structure describing how an endpoint should be configured.
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
    /// Number of bytes transferred at once while the memory is owned by 
    /// the DMA transfer.
    pub burst_size: DmaBurstSize,
    /// When `Const` is returned, the length is undetermined and the DMA
    /// increment will be disabled. If the two endpoints have undetermined
    /// length, the transfer setup will panic.
    /// 
    /// When `Incr` is returned, the DMA increment will be enabled and the 
    /// length must be equal to the opposite endpoint, or the opposite must 
    /// return `Const` length. The given length tell how many transfers of 
    /// the given `data_width` to do.
    pub increment: DmaIncrement,
    /// Address of this DMA endpoint.
    pub address: *const (),
}

/// A source endpoint for a DMA transfer.
pub trait DmaSrcEndpoint {

    /// Configure the endpoint, it provides informations about the start address, the
    /// optional increment, data width, burst size and if the it's a peripheral.
    /// 
    /// SAFETY: The implementor must ensure that the returned endpoint's address leads
    /// to valid data regarding the configuration. This data can be **const aliased**
    /// during the DMA transfer.
    unsafe fn configure(&mut self) -> DmaEndpointConfig;

    /// Close this endpoint from a DMA transfer, this is called when the DMA transfer is 
    /// stopped and destructured.
    fn close(&mut self) {}

}

/// A destination endpoint for a DMA transfer.
pub trait DmaDstEndpoint {

    /// Configure the endpoint, it provides informations about the start address, the
    /// optional increment, data width, burst size and if the it's a peripheral.
    /// 
    /// SAFETY: The implementor must ensure that the returned endpoint's address leads
    /// to valid data regarding the configuration. This data **cannot be aliased**
    /// during the DMA transfer.
    unsafe fn configure(&mut self) -> DmaEndpointConfig;

    /// Close this endpoint from a DMA transfer, this is called when the DMA transfer is 
    /// stopped and destructured.
    fn close(&mut self) {}

}

/// Implementation for string slices.
impl DmaSrcEndpoint for &'static str {

    unsafe fn configure(&mut self) -> DmaEndpointConfig {
        DmaEndpointConfig {
            peripheral: None,
            data_width: DmaDataWidth::Byte,
            burst_size: DmaBurstSize::for_size(self.len()),
            increment: DmaIncrement::Incr(self.len()),
            address: self.as_ptr() as _
        }
    }

}

/// Implementation for boxed types, the access is exclusive and the data is leaked if the
/// box is forgotten, this behavior allows us to make sure that the data cannot be aliased
/// after leaking, because the DMA controller might alias it at any time.
impl<T: Copy> DmaSrcEndpoint for Box<T> {

    unsafe fn configure(&mut self) -> DmaEndpointConfig {

        let size = core::mem::size_of::<T>();
        assert_ne!(size, 0, "zero sized types cannot be transferred through DMA");

        // We want to be sure that the data that has been written to the box have been
        // written back to the memory, so that DMA controller can see the actual data.
        unsafe { clean_data_range(&**self as *const T as usize, size) }

        DmaEndpointConfig {
            peripheral: None,
            data_width: DmaDataWidth::Byte,
            burst_size: DmaBurstSize::for_size(size),
            increment: DmaIncrement::Incr(size),
            address: &**self as *const T as *const ()
        }

    }
    
}

/// Same as for source endpoint on box.
impl<T: Copy> DmaDstEndpoint for Box<T> {

    unsafe fn configure(&mut self) -> DmaEndpointConfig {
        // SAFETY: Same as source endpoint.
        unsafe { <Self as DmaSrcEndpoint>::configure(self) }
    }

    fn close(&mut self) {

        // FIXME: Invalidating the box memory can overflow on neighbor bytes that are not
        // stored inside of the box and may belong to other memory location that should 
        // not be touched without notice.

        // When DMA has finished transfer to the box, that we know is located in RAM and
        // therefore we need to invalidate cache lines that have been bypassed by the DMA.
        // It's only needed for destination endpoint because data is written in it.
        let addr = &**self as *const T as usize;
        let size = core::mem::size_of::<T>();
        unsafe { invalidate_data_range(addr, size); }

    }

}

/// Implementation for static referenced types, the access is valid because the data is
/// guaranteed to always be available in the future.
impl<T: Copy> DmaSrcEndpoint for &'static T {

    unsafe fn configure(&mut self) -> DmaEndpointConfig {

        let size = core::mem::size_of::<T>();
        assert_ne!(size, 0, "zero sized types cannot be transferred through DMA");

        DmaEndpointConfig {
            peripheral: None,
            data_width: DmaDataWidth::Byte,
            burst_size: DmaBurstSize::for_size(size),
            increment: DmaIncrement::Incr(size),
            address: &**self as *const T as *const ()
        }

    }

}

/// Trait internally used and implemented by primitive integer types,
/// used for generic implementations of [`DmaEndpoint`].
pub trait DmaPrimitiveType {

    /// The data width to transfer the primitive type.
    fn data_width() -> DmaDataWidth;

    /// The burst size to transfer the primitive type.
    fn burst_size() -> DmaBurstSize;

}

/// Implementation for array slices with runtime length.
impl<T: DmaPrimitiveType> DmaSrcEndpoint for &'static [T] {

    unsafe fn configure(&mut self) -> DmaEndpointConfig {
        DmaEndpointConfig {
            peripheral: None,
            data_width: T::data_width(),
            burst_size: T::burst_size(),
            increment: DmaIncrement::Incr(self.len()),
            address: self.as_ptr() as _
        }
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
/// be transferred before releasing the memory bus. The values
/// are expressed in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaBurstSize {
    Incr1 = 0,
    Incr2 = 1,
    Incr8 = 2,
    Incr16 = 3,
}

impl DmaBurstSize {

    /// Find the optimal burst size for the given size (in bytes) to transfer.
    pub const fn for_size(size: usize) -> Self {
        match size {
            0..=1 => DmaBurstSize::Incr1,
            2..=7 => DmaBurstSize::Incr2,
            8..=15 => DmaBurstSize::Incr8,
            _ => DmaBurstSize::Incr16,
        }
    }
    
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

/// Internal function to get the given port registers array.
#[inline(always)]
const fn get_port_regs<const PORT: u8>() -> dma::Dma {
    match PORT {
        0 => DMA0,
        1 => DMA1,
        2 => DMA2,
        _ => unreachable!()
    }
}

/// Internal function to get the registers array of the given channel on given port.
#[inline(always)]
const fn get_channel_regs<const PORT: u8, const CHANNEL: u8>() -> dma::DmaChannel {
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
