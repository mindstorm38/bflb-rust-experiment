//! DMA peripheral.
//! 
//! This module is sourced from the Reference Manual.


embedded_util::mmio! {

    pub struct Dma {
        /// Status of the DMA interrupt after masking.
        [0x000] ro int_status: DmaBitField,
        /// Interrupt terminal count request status.
        [0x004] ro int_tc_status: DmaBitField,
        /// Terminal count request clear.
        [0x008] wo int_tc_clear: DmaBitField,
        /// Interrupt error status.
        [0x00C] ro int_error_status: DmaBitField,
        /// Interrupt error clear.
        [0x010] wo int_error_clear: DmaBitField,
        /// Status of the terminal count interrupt prior to masking.
        [0x014] ro raw_int_tc_status: DmaBitField,
        /// Status of the error interrupt prior to masking.
        [0x018] ro raw_int_error_status: DmaBitField,
        /// Channel enable status.
        [0x01C] ro channel_enable_status: DmaBitField,
        /// Software burst request (SoftBReq).
        [0x020] rw software_burst_request: u32,
        /// Software single request (SoftSReq).
        [0x024] rw software_single_request: u32,
        /// Software last burst request (SoftLBReq).
        [0x028] rw software_last_burst_request: u32,
        /// Software last single request (SoftLSReq).
        [0x02C] rw software_last_single_request: u32,
        /// DMA configuration register.
        [0x030] rw config: DmaConfig,
        /// DMA synchronization logic for DMA request signals:
        /// - 0 - Enable
        /// - 1 - Disable
        [0x034] rw sync: u32,
    }

    pub struct DmaChannel {
        /// DMA source address.
        [0x00] rw src_addr: u32,
        /// DMA Destination address.
        [0x04] rw dst_addr: u32,
        /// LLI, First linked list item. Bits 0..2 must be 0 (aligned to 4 bytes).
        [0x08] rw lli: u32,
        /// Control register for DMA channel.
        [0x0C] rw control: DmaChannelControl,
        /// Configuration register for DMA channel.
        [0x10] rw config: DmaChannelConfig,
        /// Reserved register containing unknown parameters.
        [0x1C] rw reserved: DmaChannelReserved,
    }

}

impl Dma {

    /// Custom function to get the registers for configuring a specific
    /// DMA channel. Maximum channel number may varry depending on the
    /// DMA controller.
    #[must_use]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> DmaChannel {
        unsafe { DmaChannel::new(self.0.add(0x100 + n * 0x100)) }
    }

}


embedded_util::reg! {

    pub struct DmaConfig: u32 {
        /// SMDMA Enable.
        [00..01] smdma_enable,
        /// AHB master endianness:
        /// - 0 - Little
        /// - 1 - Big
        [01..02] ahb_master_endianness,
    }

    pub struct DmaChannelControl: u32 {
        /// Transfer size: 0-4095. Number of data transfers left to complete when 
        /// the SMDMA is the flow controller.
        [00..12] transfer_size,
        /// Source burst size:
        /// - 0 - INCR1
        /// - 1 - INCR4
        /// - 2 - INCR8
        /// - 3 - INCR16
        [12..14] src_burst_size,
        /// Minus mode: Not issue all destination traffic.
        [14..15] dst_minus_mode,
        /// Destination burst size:
        /// - 0 - INCR1
        /// - 1 - INCR4
        /// - 2 - INCR8
        /// - 3 - INCR16
        [15..17] dst_burst_size,
        /// Add mode: issue remain destination traffic.
        [17..18] dst_add_mode,
        /// Source transfer width:
        /// - 0 - Byte
        /// - 1 - Half-Word
        /// - 2 - Word
        /// - 3 - Double-Word
        [18..20] src_width,
        /// Destination transfer width:
        /// - 0 - Byte
        /// - 1 - Half-Word
        /// - 2 - Word
        /// - 3 - Double-Word
        [21..23] dst_width,
        /// Only effect when dst_min_mode = 1.
        [23..26] fix_count,
        /// Source increment (SI). When set, the source address is incremented after 
        /// each transfer. Default to 1.
        [26..27] src_increment,
        /// Destination increment (DI). When set, the destination address is incremented 
        /// after each transfer. Default to 1.
        [27..28] dst_increment,
        /// Currently unused.
        [28..31] prot,
        /// Terminal count interrupt enable bit. It controls whether the current 
        /// LLI is expected to trigger the terminal count interrupt.
        [31..32] tc_int_enable,
    }

    pub struct DmaChannelConfig: u32 {
        /// Channel enable.
        [00..01] enable,
        /// Source peripheral.
        [01..06] src_peripheral,
        /// Destination peripheral.
        [06..11] dst_peripheral,
        /// - 000 - Memory-to-memory (DMA)
        /// - 001 - Memory-to-peripheral (DMA)
        /// - 010 - Peripheral-to-memory (DMA)
        /// - 011 - Source peripheral-to-Destination peripheral (DMA)
        /// - 100 - Source peripheral-to-Destination peripheral (Destination peripheral)
        /// - 101 - Memory-to-peripheral (peripheral)
        /// - 110 - Peripheral-to-memory (peripheral)
        /// - 111 - Source peripheral-to-Destination peripheral (Source peripheral)
        [11..14] flow_control,
        /// Interrupt error mask.
        [14..15] int_error_mask,
        /// Terminal count interrupt mask.
        [15..16] int_tc_mask,
        /// Lock.
        [16..17] lock,
        /// - 0 - No data in FIFO of the channel
        /// - 1 - FIFO of the channel has data
        [17..18] active,
        /// - 0 - Enable DMA requests
        /// - 1 - Ignore subsequent source DMA requests
        [18..19] halt,
        /// LLI counter. Increased 1 each LLI run. Cleared 0 when config Control.
        [20..30] lli_counter,
    }

    pub struct DmaChannelReserved: u32 {
        /// Destination remain single issue mode.
        [03..04] dst_remain_single,
        /// Source remain single issue mode.
        [04..05] src_remain_single,
    }

}


/// A bit fields where each bit is associated to a channel of a DMA controller.
#[derive(Clone, Copy, Eq, PartialEq, Default)]
#[repr(transparent)]
pub struct DmaBitField(pub u32);

impl DmaBitField {

    /// Set the bit for the given channel.
    #[inline]
    pub fn set(&mut self, channel: u8, bit: bool) {
        if bit {
            self.0 |= 1 << channel;
        } else {
            self.0 &= !(1 << channel);
        }
    }


    #[inline]
    pub fn get(&self, channel: u8) -> bool {
        (self.0 & (1 << channel)) != 0
    }

}


/// Represent a DMA Linked-List-Item. This is equivalent to the first 4 words
/// of the [`DmaChannel`] MMIO structure.
#[repr(C)]
pub struct DmaChannelLli {
    /// Source address of the transfer.
    pub src_addr: u32,
    /// Destination address of the transfer.
    pub dst_addr: u32,
    /// Adddress of the next DMA LLI.
    pub next_lli_addr: u32,
    /// Control register, it manages configuration of source and destination 
    /// and the terminal count interrupt.
    pub control: DmaChannelControl,
}


