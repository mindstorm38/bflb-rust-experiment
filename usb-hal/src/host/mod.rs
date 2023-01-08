//! Common code for controlling USB host.


/// Base trait to implement for USB host controllers (HC).
pub trait HostController {

    /// Initialize the USB controller.
    fn init(&mut self);

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