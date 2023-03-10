//! Common code for controlling USB host.


/// Base trait to implement for USB host controllers (HC).
pub trait HostController {

    /// Initialize the USB controller.
    fn init(&mut self);

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointType {
    Control,
    Bulk,
    Interrupt,
    Isochronous,
}







/// USB Request Block (URB).
#[derive(Debug)]
pub struct RequestBlock<'a> {
    pub transfer_buffer: &'a mut [u8],
    pub actual_len: u32,
    pub timeout: u32,
    pub iso_packets_count: u32,
    pub start_frame: u32,
}


/// Request direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestDirection {
    /// Out request.
    HostToDevice,
    /// In request.
    DeviceToHost,
}

/// Request type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    Standard,
    Class,
    Vendor,
    Reserved,
}

/// Request recipient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestRecipient {
    Device,
    Interface,
    Endpoint,
    Other,
}

pub struct SetupPacket {
    pub request_direction: RequestDirection,
    pub request_type: RequestType,
    pub request_recipient: RequestRecipient,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub len: u16,
}













/// USB host.
pub struct Host<C: HostController> {
    controller: C,
}



/// Abstraction of a host hub.
pub struct UsbhHub {
    /// Device connected or not.
    pub connected: bool,
    pub root: bool,
    pub index: u8,
    pub hub_addr: u8,
    pub children: [UsbhHubPort; 4],
}

/// Abstraction of a port on the host hub.
pub struct UsbhHubPort {
    /// Device connected or not.
    pub connected: bool,
    /// Hub port index.
    pub port: u8,
    /// Device address.
    pub dev_addr: u8,
    /// Device speed.
    pub speed: u8,
}
