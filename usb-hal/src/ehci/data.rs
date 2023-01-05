//! Data structure for EHCI.


emhal::mmio_reg! {
    
    /// Frame List Link pointers direct the host controller to the first 
    /// work item in the frame’s periodic schedule for the current micro-frame. 
    pub struct FrameListLink: u32 {
        /// The T-bit, when this bit is set to a one, the host controller 
        /// will never use the value of the frame list pointer as a physical 
        /// memory pointer. 
        [00..01] t,
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
    pub list: [FrameListLink; LEN],
}

/// The Asynchronous Transfer List (based at the ASYNCLISTADDR register), is 
/// where all the control and bulk transfers are managed. Host controllers use 
/// this list only when it reaches the end of the periodic list, the periodic 
/// list is disabled, or the periodic list is empty.
pub struct AsyncListQueue {
    pub 
}



/// EHCI Isochronous (High-Speed) Transfer Descriptor.
#[repr(C, align(32))]
pub struct IsochronousTransferDescriptor {
    /// Next link pointer.
    pub next_lp: u32,
    /// Transaction Status and Control list.
    pub transaction_scl: [u32; 8],
    /// Buffer Page Pointer List.
    pub buffer_pl: [u32; 7],
}


/// EHCI Queue Element Transfer Descriptor.
#[repr(C)]
pub struct EhciQtd {
    /// Next qTD Pointer.
    pub next_qtd: u32,
    /// Alternate Next qTD Pointer.
    pub alt_next_qtd: u32,
    /// qTD Token.
    pub token: u32,
    /// Buffer Page Pointer List.
    pub buffer_lp: [u32; 5],
}


/// EHCI Queue Head.
#[repr(C)]
pub struct EhciQh {
    /// Queue Head Horizontal Link Pointer.
    pub head_lp: u32,
    /// Endpoint Characteristics.
    pub ep_char: u32,
    /// Endpoint Capabilities.
    pub ep_cap: u32,
    /// Current qTD Pointer.
    pub curr_qtd: u32,
    /// Transfer overlay.
    pub overlay: EhciQtd,
}


/// Split Transaction Isochronous Transfer Descriptor.
#[repr(C)]
pub struct EhciSitd {
    /// Next link pointer.
    pub next_lp: u32,
    /// Endpoint and Transaction Translator Characteristics.
    pub ep_char: u32,
    /// Micro-frame Schedule Control.
    pub uframe_sc: u32,
    /// Transfer Status and Control.
    pub transfer_sc: u32,
    /// Buffer Pointer List.
    pub buffer_pl: [u32; 2],
    /// Back link pointer.
    pub back_lp: u32,
}


pub struct EhciPipe {
    pub dev_addr: u8,
    pub ep_addr: u8,
    pub ep_type: u8,
    pub ep_interval: u8,
    pub speed: u8,
    pub ep_mps: u16,
    pub toggle: bool,
    pub in_use: bool,
    pub xfrd: u32,
    pub waiter: bool,
    pub wait_sem: *const (),
}


pub struct EhciQhHw {
    pub hw: EhciQh,
    pub first_qtd: u32,
    pub pipe: &'static EhciPipe,
}

pub struct EhciQtdHw {
    pub hw: EhciQtd,
}

pub struct EhciItdHw {
    pub hw: IsochronousTransferDescriptor,
    // pub iso_packet: ,
    pub start_frame: u32,
    // pub list: ,
}
