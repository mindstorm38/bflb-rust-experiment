//! Data structure for EHCI.


/// The periodic frame list is a 4K-page aligned array of Frame List Link 
/// pointers. The length of the frame list may be programmable. The 
/// programmability of the periodic frame list is exported to system software 
/// via the HCCPARAMS register. If non-programmable, the length is 1024 elements. 
/// If programmable, the length can be selected by system software as one of 
/// **256, 512, or 1024 elements**. An implementation must support all three 
/// sizes. Programming the size (i.e. the number of elements) is accomplished 
/// by system software writing the appropriate value into Frame List Size field 
/// in the USBCMD register.
#[repr(C, align(4096))]
pub struct PeriodicFrameList<const LEN: usize> {
    pub list: [NextLink; LEN],
}

// COMMON //

embedded_util::reg! {
    /// A link to a descriptor.
    pub struct NextLink: u32 {
        /// This descriptor will only be considered if invalid != 1.
        [00..01] invalid,
        /// The Typ field is used to indicate the exact type of data structure 
        /// being referenced by this pointer. The value encodings are:
        /// - 0 - Isochronous Transfer Descriptor (iTD)
        /// - 1 - Queue Head (QH)
        /// - 2 - Split Transaction Isochronous Transfer Descriptor (siTD)
        /// - 3 - Frame Span Traversal Node (FSTN)
        [01..03] typ,
        /// The 32-bit pointer to the referenced object. The lower 5 bits 
        /// should be discarded (so shifting right by 5 the address), this
        /// implies an alignment of 32 bytes.
        [05..32] ptr,
    }
}

impl NextLink {

    pub fn set_invalid(&mut self) {
        self.invalid().fill();
    }

    pub fn set_valid(&mut self, typ: NextLinkType) {

        self.invalid().clear();

        let (typ, ptr) = match typ {
            NextLinkType::Itd(ptr) => (0, ptr as usize),
            NextLinkType::Qh(ptr) => (1, ptr as usize),
            NextLinkType::Sitd(ptr) => (2, ptr as usize),
        };

        self.typ().set(typ);
        self.ptr().set(ptr as u32 >> 5);

    }

    pub fn is_valid(&mut self) -> bool {
        self.invalid().get() == 0
    }
    
    pub fn get_valid(&mut self) -> Option<NextLinkType> {
        if self.is_valid() {
            let ptr = (self.ptr().get() << 5) as usize;
            Some(match self.typ().get() {
                0 => NextLinkType::Itd(ptr as _),
                1 => NextLinkType::Qh(ptr as _),
                2 => NextLinkType::Sitd(ptr as _),
                3 => todo!(),
                _ => unreachable!()
            })
        } else {
            None
        }
    }

}

#[derive(Debug, Clone)]
pub enum NextLinkType {
    Itd(*mut Itd),
    Qh(*mut Qh),
    Sitd(*mut Sitd),
}

// ISOCHRONOUS //

/// Isochronous (High-Speed) Transfer Descriptor (iTD).
/// 
/// This structure is used only for high-speed isochronous endpoints. All other 
/// transfer types should use queue structures. 
#[repr(C, align(32))]
#[derive(Default)]
pub struct Itd {
    /// Next link pointer.
    pub next_link: NextLink,
    /// Transaction Status and Control list.
    pub transactions: [ItdTransaction; 8],
    pub page_buffer_0: ItdPageBuffer0,
    pub page_buffer_1: ItdPageBuffer1,
    pub page_buffer_2: ItdPageBuffer2,
    pub page_buffer_3: ItdPageBuffer,
    pub page_buffer_4: ItdPageBuffer,
    pub page_buffer_5: ItdPageBuffer,
    pub page_buffer_6: ItdPageBuffer,
}

embedded_util::reg! {
    
    /// The host controller uses the information in each transaction description 
    /// plus the endpoint information contained in the first three dwords of the 
    /// Buffer Page Pointer list, to execute a transaction on the USB.
    pub struct ItdTransaction: u32 {
        /// This field is a value that is an offset, expressed in bytes, from the 
        /// beginning of a buffer. This field is concatenated onto the buffer page 
        /// pointer indicated in the adjacent `page_select` field to produce the 
        /// starting buffer address for this transaction.
        [00..12] offset,
        /// These bits are set by software to indicate which of the buffer page
        /// pointers the offset field in this slot should be concatenated to 
        /// produce the starting memory address for this transaction. The valid 
        /// range of values for this field is 0 to 6.
        [12..15] page_select,
        /// If this bit is set to a one, it specifies that when this transaction 
        /// completes, the Host Controller should issue an interrupt at the next
        /// interrupt threshold.
        [15..16] int_on_complete,
        /// **For an OUT**, this field is the number of data bytes the host 
        /// controller will send during the transaction. The host controller is 
        /// not required to update this field to reflect the actual number of 
        /// bytes transferred during the transfer.
        /// 
        /// **For an IN**, the initial value of the field is the number of bytes 
        /// the host expects the endpoint to deliver. During the status update, 
        /// the host controller writes back the number of bytes successfully 
        /// received.
        /// 
        /// The maximum value this field may contain is 0xC00 (3072).
        [16..28] length,
        /// Set to a one by the Host Controller during status update in the case 
        /// where the host did not receive a valid response from the device 
        /// (Timeout, CRC, Bad PID, etc.). This bit may only be set for 
        /// isochronous IN transactions.
        [28..29] transaction_error,
        /// Set to a 1 by the Host Controller during status update when a “babble” 
        /// is detected during the transaction generated by this descriptor.
        [29..30] babble_detected,
        /// Set to a 1 by the Host Controller during status update to indicate 
        /// that the Host Controller is unable to keep up with the reception of 
        /// incoming data (overrun) or is unable to supply data fast enough during 
        /// transmission (underrun). 
        [30..31] data_buffer_error,
        /// Set to 1 by software to enable the execution of an isochronous 
        /// transaction by the Host Controller. When the transaction associated 
        /// with this descriptor is completed, the Host Controller sets this bit 
        /// to 0 indicating that a transaction for this element should not be 
        /// executed when it is next encountered in the schedule.
        [31..32] active,
    }

    /// This data structure requires the associated data buffer to be contiguous
    /// (relative to virtual memory), but allows the physical memory pages to be 
    /// non-contiguous. Seven page pointers are provided to support the expression 
    /// of 8 isochronous transfers. 
    /// 
    /// Since each pointer is a 4K aligned page pointer, the least significant 12 
    /// bits in several of the page pointers are used for other purposes:
    /// - Page 0: [`ItdPageBuffer0`]
    pub struct ItdPageBuffer: u32 {
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

    /// See documentation of [`ItdPageBuffer`].
    pub struct ItdPageBuffer0: u32 {
        /// This field selects the specific device serving as the data source or sink.
        [00..07] device_addr,
        /// This 4-bit field selects the particular endpoint number on the device 
        /// serving as the data source or sink.
        [08..12] endpoint_num,
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

    /// See documentation of [`ItdPageBuffer`].
    pub struct ItdPageBuffer1: u32 {
        /// This directly corresponds to the maximum packet size of the associated 
        /// endpoint (wMaxPacketSize). This field is used for high-bandwidth 
        /// endpoints where more than one transaction is issued per transaction 
        /// description (.e.g. per micro-frame). This field is used with the Multi 
        /// field to support high-bandwidth pipes. This field is also used for all 
        /// IN transfers to detect packet babble.
        /// 
        /// Software should not set a value larger than 1024 (400h). Any value larger 
        /// yields undefined results.
        [00..10] max_packet_len,
        /// 0 = OUT; 1 = IN. This field encodes whether the high-speed transaction 
        /// should use an IN or OUT PID.
        [11..12] direction,
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

    /// See documentation of [`ItdPageBuffer`].
    pub struct ItdPageBuffer2: u32 {
        /// This field is used to indicate to the host controller the number of 
        /// transactions that should be executed per transaction description (e.g. 
        /// per micro-frame). The valid values are:
        /// - 1 - One transaction to be issued for this endpoint per micro-frame.
        /// - 2 - Two transactions to be issued for this endpoint per micro-frame.
        /// - 3 - Three transactions to be issued for this endpoint per micro-frame.
        [00..02] multi,
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

}

// SPLIT ISOCHRONOUS //

/// Split-transaction Isochronous Transaction Descriptor.
/// 
/// All Full-speed isochronous transfers through Transaction Translators are 
/// managed using the siTD data structure. This data structure satisfies the 
/// operational requirements for managing the split transaction protocol.
#[repr(C, align(32))]
#[derive(Default)]
pub struct Sitd {
    /// Next link pointer.
    pub next_link: NextLink,
    pub endpoint0: SitdEndpoint0,
    pub endpoint1: SitdEndpoint1,
    pub transfer_state: SitdTransferState,
    pub page_buffer_0: SitdPageBuffer0,
    pub page_buffer_1: SitdPageBuffer1,
    pub back_link: SitdBackLink,
}

embedded_util::reg! {

    /// Specify static information about the full-speed endpoint, the addressing of 
    /// the parent transaction translator and micro-frame scheduling control.
    pub struct SitdEndpoint0: u32 {
        /// This field selects the specific device serving as the data source or sink.
        [00..07] device_addr,
        /// This 4-bit field selects the particular endpoint number on the device 
        /// serving as the data source or sink.
        [08..12] endpoint_num,
        /// This field holds the device address of the transaction translators’ hub.
        [16..23] hub_addr,
        /// This field is the port number of the recipient transaction translator.
        [24..31] port_num,
        /// 0 = OUT; 1 = IN. This field encodes whether the high-speed transaction 
        /// should use an IN or OUT PID.
        [31..32] direction,
    }

    /// Specify static information about the full-speed endpoint, the addressing of 
    /// the parent transaction translator and micro-frame scheduling control.
    pub struct SitdEndpoint1: u32 {
        /// This field (along with the Active and SplitX-state fields in the Status 
        /// byte) are used to determine during which micro-frames the host controller 
        /// should execute start-split transactions.
        ///
        /// The host controller uses the value of the three low-order bits of the 
        /// FRINDEX register to index into this bit field. If the FRINDEX register 
        /// value indexes to a position where the μFrame S-mask field is a one, then 
        /// this siTD is a candidate for transaction execution.
        /// 
        /// An all zeros value in this field, in combination with existing in the 
        /// periodic frame list has undefined results.
        [00..08] split_start_mask,
        /// This field (along with the Active and SplitX-state fields in the Status 
        /// byte) are used to determine during which micro-frames the host controller 
        /// should execute complete-split transactions. When the criteria for using 
        /// this field are met, an all zeros value in this field has undefined behavior.
        /// 
        /// The host controller uses the value of the three low-order bits of the FRINDEX 
        /// register to index into this bit field. If the FRINDEX register value indexes 
        /// to a position where the μFrame C-Mask field is a one, then this siTD is a 
        /// candidate for transaction execution.
        /// 
        /// There may be more than one bit in this mask set.
        [08..16] split_completion_mask,
    }

    /// Used to manage the state of the transfer.
    pub struct SitdTransferState: u32 {
        /// Values:
        /// - 0 - **Do Start Split.** This value directs the host controller to issue a 
        ///   Start split transaction to the endpoint when a match is encountered in the 
        ///   S-mask.
        /// - 1 - **Do Complete Split.** This value directs the host controller to issue
        ///   a Complete split transaction to the endpoint when a match is encountered 
        ///   in the C-mask.
        [01..02] split_transaction_state,
        /// The host controller detected that a host-induced hold-off caused the host 
        /// controller to miss a required complete-split transaction.
        [02..03] missed_micro_frame,
        /// Set to a 1 by the Host Controller during status update in the case where 
        /// the host did not receive a valid response from the device (Timeout, CRC, 
        /// Bad PID, etc.). This bit will only be set for IN transactions.
        [03..04] transaction_error,
        /// Set to a 1 by the Host Controller during status update when a “babble” 
        /// is detected during the transaction generated by this descriptor.
        [04..05] babble_detected,
        /// Set to a 1 by the Host Controller during status update to indicate 
        /// that the Host Controller is unable to keep up with the reception of 
        /// incoming data (overrun) or is unable to supply data fast enough during 
        /// transmission (underrun). 
        [05..06] data_buffer_error,
        /// Set to a 1 by the Host Controller when an ERR response is received from 
        /// the transaction translator.
        [06..07] err,
        /// Set to 1 by software to enable the execution of an isochronous split
        /// transaction by the Host Controller. 
        [07..08] active,
        /// This field is used by the host controller to record which split-completes 
        /// have been executed. 
        [08..16] micro_frame_complete_progress_mask,
        /// This field is initialized by software to the total number of bytes expected 
        /// in this transfer. Maximum value is 1023 (3FFh).
        [16..26] bytes_to_transfer,
        /// Used to indicate which data page pointer should be concatenated with the 
        /// CurrentOffset field to construct a data buffer pointer (0 selects Page 0 
        /// pointer and 1 selects Page 1). The host controller is not required to 
        /// write this field back when the siTD is retired (Active bit transitioned 
        /// from a one to a zero).
        [30..31] page_select, 
        /// 0 = Do not interrupt when transaction is complete. 1 = Do interrupt when 
        /// transaction is complete. When the host controller determines that the split
        /// transaction has completed it will assert a hardware interrupt at the next 
        /// interrupt threshold.
        [31..32] int_on_complete,
    }

    pub struct SitdPageBuffer0: u32 {
        /// The 12 least significant bits of the Page 0 pointer is the current byte
        /// offset for the current page pointer 
        /// 
        /// The host controller is not required to write this field back when the siTD 
        /// is retired (Active bit transitioned from a one to a zero).
        [00..12] current_offset,
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

    pub struct SitdPageBuffer1: u32 {
        /// Software initializes this field with the number of OUT start-splits this 
        /// transfer requires. Any value larger than 6 is undefined.
        [00..03] transaction_count,
        /// This field is used with T-count to determine whether to send all, first, 
        /// middle, or last with each outbound transaction payload. System software 
        /// must initialize this field with the appropriate starting value. The host 
        /// controller must correctly manage this state during the lifetime of the 
        /// transfer. The bit encodings are:
        /// - 0 - **All.** The entire full-speed transaction data payload is in this
        ///   transaction (i.e. less than or equal to 188 bytes).
        /// - 1 - **Begin.** This is the first data payload for a full-speed transaction 
        ///   that is greater than 188 bytes.
        /// - 2 - **Mid.** This the middle payload for a full-speed OUT transaction that 
        ///   is larger than 188 bytes.
        /// - 3 - **End.** This is the last payload for a full-speed OUT transaction that
        ///   was larger than 188 bytes.
        [03..05] transaction_pos,
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

    pub struct SitdBackLink: u32 {
        /// This descriptor will only be considered if invalid != 1.
        [00..01] invalid,
        /// This field is a physical memory pointer to an siTD.
        [05..32] ptr,
    }

}

impl SitdBackLink {

    pub fn set_invalid(&mut self) {
        self.invalid().fill();
    }

    pub fn set_valid(&mut self, ptr: *const Sitd) {
        self.invalid().clear();
        self.ptr().set(ptr as usize as u32 >> 5);
    }

}

// QUEUE TRANSFER DESCRIPTOR //

/// Queue Element Transfer Descriptor (qTD)
/// 
/// This data structure is only used with a queue head (see Section 3.6). This data 
/// structure is used for one or more USB transactions. See Section 4.10 for a 
/// complete description of the behavioral model. This data structure is used to 
/// transfer up to 20480 (5*4096) bytes. The structure contains two structure pointers 
/// used for queue advancement, a Dword of transfer state and a five-element array of 
/// data buffer pointers. This structure is 32 bytes (or one 32-byte cache line). 
/// This data structure must be physically contiguous.
/// 
/// The buffer associated with this transfer must be virtually contiguous. The buffer
/// may start on any byte boundary. A separate buffer pointer list element must be 
/// used for each physical page in the buffer, regardless of whether the buffer is 
/// physically contiguous.
#[repr(C)]
#[derive(Default)]
pub struct Qtd {
    /// Link to the next qTD pointer.
    pub next_link: QtdNextLink,
    /// Used to support hardware-only advance of the data stream to the next client 
    /// buffer on short packet. To be more explicit the host controller will always 
    /// use this pointer when the current qTD is retired due to short packet.
    pub atl_next_link: QtdNextLink,
    /// Contains most of the information the host controller requires to execute a 
    /// USB transaction (the remaining endpoint-addressing information is specified
    /// in the queue head).
    pub token: QtdToken,
    pub page_buffer_0: QtdPageBuffer0,
    pub page_buffer_1: QtdPageBuffer,
    pub page_buffer_2: QtdPageBuffer,
    pub page_buffer_3: QtdPageBuffer,
    pub page_buffer_4: QtdPageBuffer,
}

embedded_util::reg! {

    pub struct QtdNextLink: u32 {
        /// This descriptor will only be considered if invalid != 1.
        [00..01] invalid,
        /// This field contains the physical memory address of the next qTD to be 
        /// processed. 
        /// 
        /// The lower 5 bits should be discarded (so shifting right by  5 the address), 
        /// this implies an alignment of 32 bytes.
        [05..32] ptr,
    }

    pub struct QtdToken: u32 {
        /// If the QH.EPS field indicates a High-speed device and the PID_Code 
        /// indicates an OUT endpoint, then this is the state bit for the Ping 
        /// protocol. See Section 4.11 for operational details. 
        /// 
        /// The bit encodings are:
        /// - 0 - **Do OUT.** This value directs the host controller to issue 
        ///   an OUT PID to the endpoint.
        /// - 1 - **Do Ping.** This value directs the host controller to issue 
        ///   a PING PID to the endpoint.
        [00..01] ping_state,
        /// This bit is ignored by the host controller unless the QH.EPS field 
        /// indicates a full- or low-speed endpoint. When a Full- or Low-speed 
        /// device, the host controller uses this bit to track the state of 
        /// the split-transaction. The functional requirements of the host 
        /// controller for managing this state bit and the split transaction 
        /// protocol depends on whether the endpoint is in the periodic or 
        /// asynchronous schedule.
        /// 
        /// The bit encodings are:
        /// - 0 - **Do Start Split.** This value directs the host controller 
        ///   to issue a Start split transaction to the endpoint.
        /// - 1 - **Do Complete Split.** This value directs the host controller 
        ///   to issue a Complete split transaction to the endpoint.
        [01..02] split_transaction_state,
        /// The host controller detected that a host-induced hold-off caused the host 
        /// controller to miss a required complete-split transaction.
        [02..03] missed_micro_frame,
        /// Set to a 1 by the Host Controller during status update in the case where 
        /// the host did not receive a valid response from the device (Timeout, CRC, 
        /// Bad PID, etc.). This bit will only be set for IN transactions.
        [03..04] transaction_error,
        /// Set to a 1 by the Host Controller during status update when a “babble” 
        /// is detected during the transaction generated by this descriptor.
        [04..05] babble_detected,
        /// Set to a 1 by the Host Controller during status update to indicate 
        /// that the Host Controller is unable to keep up with the reception of 
        /// incoming data (overrun) or is unable to supply data fast enough during 
        /// transmission (underrun). 
        [05..06] data_buffer_error,
        /// Set to a 1 by the Host Controller during status updates to indicate that
        /// a serious error has occurred at the device/endpoint addressed by this qTD.
        /// This can be caused by babble, the error counter counting down to zero, or
        /// reception of the STALL handshake from the device during a transaction. Any
        /// time that a transaction results in the Halted bit being set to a one, the 
        /// Active bit is also set to 0.
        [06..07] halted,
        /// Set to 1 by software to enable the execution of transactions by the 
        /// Host Controller. 
        [07..08] active,
        /// This field is an encoding of the token which should be used for transactions
        /// associated with this transfer descriptor. Encodings are:
        /// - 0 - OUT token, generates token (E1H)
        /// - 1 - IN token, generates token (69H)
        /// - 2 - SETUP token, generates token (2DH), undefined if endpoint is an 
        ///   Interrupt transfer type, e.g. micro frame S-mask field in the queue 
        ///   head is non-zero.
        [08..10] pid_code,
        /// This field is a 2-bit down counter that keeps track of the number of 
        /// consecutive Errors detected while executing this qTD. If this field is
        /// programmed with a non zero value during setup, the Host Controller 
        /// decrements the count and writes it back to the qTD if the transaction 
        /// fails. 
        [10..12] error_counter,
        /// This field is used as an index into the qTD buffer pointer list. Valid 
        /// values are in the range 0H to 4H. The host controller is not required 
        /// to write this field back when the qTD is retired.
        [12..15] current_page,
        /// If this bit is set to a one, it specifies that when this transaction 
        /// completes, the Host Controller should issue an interrupt at the next
        /// interrupt threshold.
        [15..16] int_on_complete,
        /// This field specifies the total number of bytes to be moved with this 
        /// transfer descriptor. This field is decremented by the number of bytes 
        /// actually moved during the transaction, only on the successful completion 
        /// of the transaction.
        /// 
        /// The maximum value software may store in this field is 5 * 4K (5000H). 
        /// This is the maximum number of bytes 5 page pointers can access.
        /// 
        /// If the value of this field is zero when the host controller fetches 
        /// this transfer descriptor (and the active bit is set), the host controller 
        /// executes a zero-length transaction and retires the transfer descriptor.
        [16..31] bytes_to_transfer,
        /// This is the data toggle sequence bit. The use of this bit depends on the
        /// setting of the Data Toggle Control bit in the queue head.
        [31..32] data_toggle,
    }

    pub struct QtdPageBuffer: u32 {
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
    }

    pub struct QtdPageBuffer0: u32 {
        /// This is a 4K aligned pointer to physical memory (with the 12 LSBits
        /// discarded).
        [12..32] ptr,
        /// This field is the byte offset into the current page (as selected by 
        /// C_Page).
        [00..12] current_offset,
    }

}

impl QtdNextLink {

    pub fn set_invalid(&mut self) {
        self.invalid().fill();
    }

    pub fn set_valid(&mut self, ptr: *const Qtd) {
        self.invalid().clear();
        self.ptr().set(ptr as usize as u32 >> 5);
    }

}


/// Queue Head
#[repr(C)]
#[derive(Default)]
pub struct Qh {
    /// link pointer to the next data object to be processed after any required 
    /// processing in this queue has been completed, as well as the control bits 
    /// defined below.
    pub horizontal_link: NextLink,
    /// These are the USB endpoint characteristics including addressing, maximum
    /// packet size, and endpoint speed.
    pub endpoint_characteristics: QhEndpointCharacteristics,
    /// These are adjustable parameters of the endpoint. They effect how the 
    /// endpoint data stream is managed by the host controller. 
    pub endpoint_capabilities: QhEndpointCapabilities,
    /// contains a pointer to the source qTD currently associated with the overlay.
    /// The host controller uses this pointer to write back the overlay area into
    /// the source qTD after the transfer is complete.
    pub current_qtd_link: QhCurrentQtdLink,
    /// Transfer overlay.
    pub overlay: Qtd,
}

embedded_util::reg! {

    pub struct QhEndpointCharacteristics: u32 {
        /// This field selects the specific device serving as the data source or sink.
        [00..07] device_addr,
        /// This bit is used by system software to request that the host controller 
        /// set the Active bit to zero. 
        [07..08] inactivate_on_next_transaction,
        /// This 4-bit field selects the particular endpoint number on the device 
        /// serving as the data source or sink.
        [08..12] endpoint_num,
        /// This is the speed of the associated endpoint. Bit combinations are:
        /// - 0 - Full-Speed (12 Mb/s)
        /// - 1 - Low-Speed (1.5 Mb/s)
        /// - 2 - High-Speed (480 Mb/s)
        [12..14] endpoint_speed,
        /// This bit specifies where the host controller should get the initial data 
        /// toggle on an overlay transition.
        /// - 0 - Ignore DT bit from incoming qTD. Host controller preserves DT bit 
        ///   in the queue head.
        /// - 1 - Initial data toggle comes from incoming qTD DT bit. Host controller 
        ///   replaces DT bit in the queue head from the DT bit in the qTD.
        [14..15] data_toggle_control,
        /// This bit is set by System Software to mark a queue head as being the head 
        /// of the reclamation list.
        [15..16] head_of_reclamation_list_flag,
        /// This directly corresponds to the maximum packet size of the associated 
        /// endpoint (wMaxPacketSize).
        /// 
        /// The maximum value this field may contain is 0x400 (1024).
        [16..27] max_packet_len,
        /// If the QH.EPS field indicates the endpoint is not a high-speed device, 
        /// and the endpoint is an control endpoint, then software must set this 
        /// bit to a one.
        /// 
        /// Otherwise it should always set this bit to a zero.
        [27..28] control_endpoint_flag,
        /// This field contains a value, which is used by the host controller to 
        /// reload the Nak Counter field.
        [28..32] nak_count_reload,
    }

    pub struct QhEndpointCapabilities: u32 {
        /// This field is used for all endpoint speeds. Software should set this 
        /// field to a zero when the queue head is on the asynchronous schedule. 
        /// A non-zero value in this field indicates an interrupt endpoint.
        [00..08] int_schedule_mask,
        /// This field is ignored by the host controller unless the EPS field 
        /// indicates this device is a low- or full-speed device and this queue
        /// head is in the periodic list. This field (along with the Active and 
        /// SplitX-state fields) is used to determine during which micro-frames 
        /// the host controller should execute a complete-split transaction. 
        /// When the criteria for using this field are met, a zero value in this 
        /// field has undefined behavior.
        [08..16] split_completion_mask,
        /// This field is ignored by the host controller unless the EPS field 
        /// indicates a full- or low-speed device. The value is the USB device 
        /// address of the USB 2.0 Hub below which the full- or low-speed device 
        /// associated with this endpoint is attached. This field is used in the 
        /// split-transaction protocol. 
        [16..23] hub_addr,
        /// This field is ignored by the host controller unless the EPS field 
        /// indicates a full- or low-speed device. The value is the port number 
        /// identifier on the USB 2.0 Hub (for hub at device address Hub Addr 
        /// below), below which the full- or low-speed device associated with 
        /// this endpoint is attached. This information is used in the 
        /// split-transaction protocol. 
        [23..30] port_num,
        /// This field is a multiplier used to key the host controller as the 
        /// number of successive packets the host controller may submit to the
        /// endpoint in the current execution. The host controller makes the 
        /// simplifying assumption that software properly initializes this field 
        /// (regardless of location of queue head in the schedules or other run 
        /// time parameters). 
        [30..32] high_bandwidth_pipe_mult,
    }

    pub struct QhCurrentQtdLink: u32 {
        /// The 32-bit pointer to the referenced object. The lower 5 bits 
        /// should be discarded (so shifting right by 5 the address), this
        /// implies an alignment of 32 bytes.
        [05..32] ptr,
    }

}
