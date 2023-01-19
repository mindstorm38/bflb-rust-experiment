//! DMA peripheral.
//! 
//! This module is sourced from the Reference Manual.


embedded_util::mmio! {

    pub struct Dma {
        /// Status of the DMA interrupt after masking.
        [0x000] ro int_status: DmaIntStatus,
        /// Interrupt terminal count request status.
        [0x004] ro int_tc_status: DmaIntTcStatus,
        /// Terminal count request clear.
        [0x008] wo int_tc_clear: DmaIntTcClear,
        /// Interrupt error status.
        [0x00C] ro int_error_status: DmaIntErrorStatus,
        /// Interrupt error clear.
        [0x010] wo int_error_clear: DmaIntErrorClear,
        /// Status of the terminal count interrupt prior to masking.
        [0x014] ro raw_int_tc_status: DmaRawIntTcStatus,
        /// Status of the error interrupt prior to masking.
        [0x018] ro raw_int_error_status: DmaRawIntErrorStatus,
        /// Channel enable status.
        [0x01C] ro channel_enable_status: DmaChannelEnableStatus,
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

    pub struct DmaIntStatus: u32 {
        /// Status of the DMA interrupt after masking.
        [00..08] int_status,
    }

    pub struct DmaIntTcStatus: u32 {
        /// Interrupt terminal count request status.
        [00..08] int_tc_status,
    }

    pub struct DmaIntTcClear: u32 {
        /// Terminal count request clear.
        [00..08] int_tc_clear,
    }

    pub struct DmaIntErrorStatus: u32 {
        /// Interrupt error status.
        [00..08] int_error_status,
    }

    pub struct DmaIntErrorClear: u32 {
        /// Interrupt error clear.
        [00..08] int_error_clear,
    }

    pub struct DmaRawIntTcStatus: u32 {
        /// Status of the terminal count interrupt prior to masking.
        [00..08] raw_int_tc_status,
    }

    pub struct DmaRawIntErrorStatus: u32 {
        /// Status of the error interrupt prior to masking..
        [00..08] raw_int_error_status,
    }

    pub struct DmaChannelEnableStatus: u32 {
        /// Channel enable status.
        [00..08] channel_enable_status,
    }

    pub struct DmaConfig: u32 {
        /// SMDMA Enable.
        [00..01] smd_dma,
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
