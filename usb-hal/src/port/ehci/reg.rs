//! USB EHCI Memory-Mapped registers.

use embedded_util::PtrRw;


/// A structure containing the two EHCI registers: capability and operationnal.
pub struct Ehci {
    /// EHCI Host Controller Capability Registers.
    pub hccr: EhciHccr,
    /// EHCI Host Controller Operational Registers.
    pub hcor: EhciHcor,
}


embedded_util::mmio! {

    /// EHCI Host Controller Capability Registers.
    pub struct EhciHccr {
        /// This register is used as an offset to add to register base 
        /// to find the beginning of the Operational Register Space.
        [0x00] rw capability_len: u8,
        /// This is a two-byte register containing a BCD encoding of the 
        /// EHCI revision number supported by this host controller. The 
        /// most significant byte of this register represents a major 
        /// revision and the least significant byte is the minor revision.
        [0x02] rw hci_version: u16,
        /// This is a set of fields that are structural parameters: Number 
        /// of downstream ports, etc.
        [0x04] rw hcs_params: EhciHcsParams,
        /// Multiple Mode control (time-base bit functionality), addressing 
        /// capability.
        [0x08] rw hcc_params: EhciHccParams,
        /// This optional field is valid only if Port Routing Rules field in 
        /// the HCSPARAMS register is set to a one. The rules for organizing 
        /// companion host controllers and an EHCI host controllers within 
        /// PCI space are described in detail in Section 4.2. This field is 
        /// used to allow a host controller implementation to explicitly
        /// described to which companion host controller each implemented port 
        /// is mapped.
        /// 
        /// This field is a 15-element nibble array (each 4 bits is one array 
        /// element). Each array location corresponds one-to-one with a physical
        /// port provided by the host controller (e.g. PORTROUTE[0] corresponds 
        /// to the first PORTSC port, PORTROUTE[1] to the second PORTSC port,
        /// etc.). The value of each element indicates to which of the companion 
        /// host controllers this port is routed. Only the first N_PORTS elements 
        /// have valid information. A value of zero indicates that the port is 
        /// routed to the lowest numbered function companion host controller. 
        /// 
        /// A value of one indicates that the port is routed to the next lowest 
        /// numbered function companion host controller, and so on.
        [0x0C] rw hcspport_route: [u8; 8],
    }

    /// EHCI Host Controller Operational Registers.
    pub struct EhciHcor {
        /// The Command Register indicates the command to be executed by the 
        /// serial bus host controller. Writing to the register causes a command 
        /// to be executed.
        [0x00] rw usb_cmd: EhciUsbCmd,
        /// This register indicates pending interrupts and various states of the 
        /// Host Controller. The status resulting from a transaction on the 
        /// serial bus is not indicated in this register. Software sets a bit 
        /// to 0 in this register by writing a 1 to it.
        [0x04] rw usb_status: EhciUsbStatus,
        /// This register enables and disables reporting of the corresponding 
        /// interrupt to the software. When a bit is set and the corresponding 
        /// interrupt is active, an interrupt is generated to the host. 
        /// Interrupt sources that are disabled in this register still appear 
        /// in the USBSTS to allow the software to poll for events.
        [0x08] rw usb_int: EhciUsbInt,
        /// This register is used by the host controller to index into the 
        /// periodic frame list. The register updates every 125 microseconds 
        /// (once each micro-frame). Bits [N:3] are used to select a particular 
        /// entry in the Periodic Frame List during periodic schedule execution. 
        /// The number of bits used for the index depends on the size of the frame 
        /// list as set by system software in the Frame List Size field in the 
        /// USBCMD register. 
        /// 
        /// This register must be written as a DWord. Byte writes produce undefined 
        /// results. This register cannot be written unless the Host Controller is 
        /// in the Halted state as indicated by the HCHalted bit. A write to this 
        /// register while the Run/Stop bit is set to a one produces undefined 
        /// results. Writes to this register also affect the SOF value.
        [0x0C] rw frame_index: EhciFrameIndex,
        /// Control Data Structure Segment Register
        /// 
        /// This 32-bit register corresponds to the most significant address bits 
        /// [63:32] for all EHCI data structures. If the 64-bit Addressing Capability 
        /// field in HCCPARAMS is a zero, then this register is not used. Software
        /// cannot write to it and a read from this register will return zeros.
        /// 
        /// If the 64-bit Addressing Capability field in HCCPARAMS is a one, then 
        /// this register is used with the link pointers to construct 64-bit addresses 
        /// to EHCI control data structures. This register is concatenated with the 
        /// link pointer from either the PERIODICLISTBASE, ASYNCLISTADDR, or any 
        /// control data structure link field to construct a 64-bit address.
        [0x10] rw ctrlds_segment: u32,
        /// Periodic Frame List Base Address Register.
        /// 
        /// This 32-bit register contains the beginning address of the Periodic Frame 
        /// List in the system memory. If the host controller is in 64-bit mode (as 
        /// indicated by a one in the 64-bit Addressing Capability field in the 
        /// HCCSPARAMS register), then the most significant 32 bits of every control 
        /// data structure address comes from the CTRLDSSEGMENT register. System 
        /// software loads this register prior to starting the schedule execution 
        /// by the Host Controller. 
        /// 
        /// The memory structure referenced by this physical memory pointer is assumed 
        /// to be **4-Kbyte aligned**. The contents of this register are combined with 
        /// the Frame Index Register (FRINDEX) to enable the Host Controller to step 
        /// through the Periodic Frame List in sequence.
        [0x14] rw periodic_list_base: u32,
        /// Current Asynchronous List Address Register
        /// 
        /// This 32-bit register contains the address of the next asynchronous queue
        /// head to be executed. If the host controller is in 64-bit mode (as indicated 
        /// by a one in 64-bit Addressing Capability field in the HCCPARAMS register), 
        /// then the most significant 32 bits of every control data structure address 
        /// comes from the CTRLDSSEGMENT register. Bits [4:0] of this register cannot 
        /// be modified by system software and will always return a zero when read. 
        /// The memory structure referenced by this physical memory pointer is assumed 
        /// to be **32-byte (cache line) aligned**.
        [0x18] rw async_list_addr: u32,
        /// Configured flag register.
        [0x40] rw config_flag: EhciConfigFlag,
    }

}

impl EhciHcor {

    /// Get a read/write pointer to a [`EhciPortStatusControl`] for a given port.
    /// 
    /// A host controller must implement one or more port registers. The number of 
    /// port registers implemented by a particular instantiation of a host controller 
    /// is documented in the HCSPARAMs register. Software uses this information as an 
    /// input parameter to determine how many ports need to be serviced. All ports have 
    /// the structure defined below.
    #[inline(always)]
    pub fn port_status_control(&self, port: usize) -> PtrRw<EhciPortStatusControl> {
        PtrRw((0x44 + port * 4) as *mut _)
    }

}


embedded_util::reg! {

    /// Host Controller Structural Parameters.
    pub struct EhciHcsParams: u32 {
        /// This field specifies the number of physical downstream ports 
        /// implemented on this host controller. The value of this field 
        /// determines how many port registers are addressable in the 
        /// Operational Register Space (see Table 2-8). Valid values are 
        /// in the range of 1H to FH. A zero in this field is undefined.
        [00..04] num_ports,
        /// This field indicates whether the host controller implementation 
        /// includes port power control. A one in this bit indicates the 
        /// ports have port power switches. A zero in this bit indicates 
        /// the port do not have port power switches. The value of this 
        /// field affects the functionality of the Port Power field in 
        /// each port status and control register (see Section 2.3.8).
        [04..05] port_power_control,
        /// This field indicates the method used by this implementation 
        /// for how all ports are mapped to companion controllers. The 
        /// value of this field has the following interpretation:
        /// - 0 - The first N_PCC ports are routed to the lowest numbered 
        ///   function companion host controller, the next N_PCC port are 
        ///   routed to the next lowest function companion controller, 
        ///   and so on.
        /// - 1 - The port routing is explicitly enumerated by the first 
        ///   N_PORTS elements of the HCSP-PORTROUTE array.
        [07..08] port_routing_rules,
        /// This field indicates the number of ports supported per companion 
        /// host controller. It is used to indicate the port routing 
        /// configuration to system software.
        /// 
        /// For example, if N_PORTS has a value of 6 and N_CC has a value 
        /// of 2 then N_PCC could have a value of 3. The convention is that 
        /// the first N_PCC ports are assumed to be routed to companion 
        /// controller 1, the next N_PCC ports to companion controller 2, etc.
        /// 
        /// In the previous example, the N_PCC could have been 4, where 
        /// the first 4 are routed to companion controller 1 and the last 
        /// two are routed to companion controller 2.
        /// 
        /// The number in this field must be consistent with N_PORTS and N_CC.
        [08..12] ports_per_companion_controller,
        /// This field indicates the number of companion controllers 
        /// associated with this USB 2.0 host controller.
        /// 
        /// A zero in this field indicates there are no companion host 
        /// controllers. Port-ownership hand-off is not supported. Only 
        /// high-speed devices are supported on the host controller root 
        /// ports.
        /// 
        /// A value larger than zero in this field indicates there are 
        /// companion USB 1.1 host controller(s). Port-ownership hand-offs 
        /// are supported. High, Full- and Low-speed devices are supported
        /// on the host controller root ports.
        [12..16] num_companion_controller,
        /// This bit indicates whether the ports support port indicator 
        /// control. When this bit is a one, the port status and control 
        /// registers include a read/writeable field for controlling the 
        /// state of the port indicator. 
        [16..17] port_indicators,
        /// *Optional.* This register identifies which of the host controller 
        /// ports is the debug port. The value is the port number (one-based) 
        /// of the debug port. A non-zero value in this field indicates the 
        /// presence of a debug port. The value in this register must not be 
        /// greater than N_PORTS (see below).
        [20..24] debug_port_number,
    }

    /// Host Controller Capability Parameters.
    pub struct EhciHccParams: u32 {
        /// This field documents the addressing range capability of this 
        /// implementation. The value of this field determines whether 
        /// software should use the data structures defined in Section 3 
        /// (32-bit) or those defined in Appendix B (64-bit).
        /// Values for this field have the following interpretation:
        /// - 0 - data structures using 32-bit address memory pointers.
        /// - 1 - data structures using 64-bit address memory pointers.
        [00..01] addr_64bit,
        /// Default = Implementation dependent. If this bit is set to a 
        /// zero, then system software must use a frame list length of 
        /// 1024 elements with this host controller. The USBCMD register 
        /// Frame List Size field is a read-only register and should be 
        /// set to zero.
        /// 
        /// If set to a one, then system software can specify and use a 
        /// smaller frame list and configure the host controller via the 
        /// USBCMD register Frame List Size field. The frame list must 
        /// always be aligned on a 4K page boundary. This requirement 
        /// ensures that the frame list is always physically contiguous.
        [01..02] prog_frame_list_flag,
        /// Default = Implementation dependent. If this bit is set to a 
        /// one, then the host controller supports the park feature for 
        /// high-speed queue heads in the Asynchronous Schedule. The 
        /// feature can be disabled or enabled and set to a specific level 
        /// by using the Asynchronous Schedule Park Mode Enable and 
        /// Asynchronous Schedule Park Mode Count fields in the USBCMD 
        /// register.
        [02..03] async_schedule_park_cap,
        /// Default = implementation dependent. This field indicates, 
        /// relative to the current position of the executing host 
        /// controller, where software can reliably update the isochronous 
        /// schedule. When bit [7] is zero, the value of the least 
        /// significant 3 bits indicates the number of micro-frames a host 
        /// controller can hold a set of isochronous data structures (one 
        /// or more) before  flushing the state. When bit [7] is a one, 
        /// then host software assumes the host controller may cache an 
        /// isochronous data structure for an entire frame. 
        [04..08] isochronous_scheduling_threshold,
        /// Default = Implementation Dependent. This optional field 
        /// indicates the existence of a capabilities list. A value of 
        /// 00h indicates no extended capabilities are implemented. A 
        /// non-zero value in this register indicates the offset in PCI 
        /// configuration space of the first EHCI extended capability. 
        /// The pointer value must be 40h or greater if implemented to 
        /// maintain the consistency of the PCI header defined for this 
        /// class of device.
        [08..16] extended_cap_ptr,
    }

    /// USB Command Register.
    pub struct EhciUsbCmd: u32 {
        /// Default 0. 1=Run. 0=Stop. When set to a 1, the Host Controller
        /// proceeds with execution of the schedule. The Host Controller 
        /// continues execution as long as this bit is set to a 1. When this 
        /// bit is set to 0, the Host Controller completes the current and 
        /// any actively pipelined transactions on the USB and then halts. 
        /// 
        /// The Host Controller must halt within 16 micro-frames after 
        /// software clears the Run bit. The HC Halted bit in the status 
        /// register indicates when the Host Controller has finished its 
        /// pending pipelined  transactions and has entered the stopped 
        /// state. Software must not write a one to this field unless the 
        /// host controller is in the Halted state (i.e. HCHalted in the 
        /// USBSTS register is a one). Doing so will yield undefined results.
        /// 
        /// Read/Write.
        [00..01] run_stop,
        /// This control bit is used by software to reset the host controller. 
        /// The effects of this on Root Hub registers are similar to a Chip
        /// Hardware Reset.
        /// 
        /// When software writes a one to this bit, the Host Controller resets 
        /// its internal pipelines, timers, counters, state machines, etc. to 
        /// their initial value. Any transaction currently in progress on USB 
        /// is immediately terminated. A USB reset is not driven on downstream
        /// ports.
        /// 
        /// Read/Write.
        [01..02] host_controller_reset,
        /// Default 0. This field is R/W only if Programmable Frame List Flag 
        /// in the HCCPARAMS registers is set to a one. This field specifies 
        /// the size of the frame list. The size the frame list controls which 
        /// bits in the Frame Index Register should be used for the Frame List 
        /// Current index. Values mean:
        /// - 0 - 1024 elements (4096 bytes)
        /// - 1 - 512 elements (2048 bytes)
        /// - 2 - 256 elements (1024 bytes)
        [02..04] frame_list_size,
        /// Default 0. This bit controls whether the host controller skips 
        /// processing the Periodic Schedule. Values mean:
        /// - 0 - Do not process the Periodic Schedule
        /// - 1 - Use the PERIODICLISTBASE register to access Periodic Schedule.
        /// 
        /// Read/Write.
        [04..05] periodic_schedule_enable,
        /// Default 0. This bit controls whether the host controller skips 
        /// processing the Asynchronous Schedule. Values mean:
        /// - 0 - Do not process the Asynchronous Schedule
        /// - 1 - Use the ASYNCLISTADDR register to access the Asynchronous Schedule
        /// 
        /// Read/Write
        [05..06] async_schedule_enable,
        /// This bit is used as a doorbell by software to tell the host controller 
        /// to issue an interrupt the next time it advances asynchronous schedule. 
        /// Software must write a 1 to this bit to ring the doorbell.
        /// 
        /// Read/Write
        [06..07] int_on_async_adv_doorbell,
        /// This control bit is not required. If implemented, it allows the driver 
        /// to reset the EHCI controller without affecting the state of the ports 
        /// or the relationship to the companion host controllers. For example, the
        /// PORSTC registers should not be reset to their default values and the 
        /// CF bit setting should not go to zero (retaining port ownership 
        /// relationships).
        /// 
        /// A host software read of this bit as zero indicates the Light Host 
        /// Controller Reset has completed and it is safe for host software to 
        /// re-initialize the host controller. A host software read of this bit 
        /// as a one indicates the Light Host Controller Reset has not yet
        /// completed.
        /// 
        /// If not implemented a read of this field will always return a zero.
        /// 
        /// Read/Write
        [07..08] light_host_controller_reset,
        /// If the Asynchronous Park Capability bit in the HCCPARAMS register is 
        /// a one, then this field defaults to 3h and is R/W. Otherwise it defaults 
        /// to zero and is RO. It contains a count of the number of successive 
        /// transactions the host controller is allowed to execute from a high-speed 
        /// queue head on the Asynchronous schedule before continuing traversal of 
        /// the Asynchronous schedule. See Section 4.10.3.2 for full operational 
        /// details. Valid values are 1h to 3h. Software must not write a zero to 
        /// this bit when Park Mode Enable is a one as this will result in 
        /// undefined behavior.
        [08..10] async_schedule_park_mode_count, 
        /// If the Asynchronous Park Capability bit in the HCCPARAMS register is a 
        /// one, then this bit defaults to a 1h and is R/W. Otherwise the bit must 
        /// be a zero and is RO. Software uses this bit to enable or disable Park 
        /// mode. When this bit is one, Park mode is enabled. When this bit is a 
        /// zero, Park mode is disabled.
        [11..12] async_schedule_park_mode_enable,
        /// Default 8. This field is used by system software to select the maximum 
        /// rate at which the host controller will issue interrupts. The only valid 
        /// values are defined below. If software writes an invalid value to this 
        /// register, the results are undefined.
        /// - 1 - 1 micro-frame
        /// - 2 - 2 micro-frames
        /// - 4 - 4 micro-frames
        /// - 8 - 8 micro-frames (default, 1 ms)
        /// - 16 - 16 micro-frames (2 ms)
        /// - 32 - 32 micro-frames (4 ms)
        /// - 64 - 64 micro-frames (8 ms)
        /// 
        /// Read/Write
        [16..24] int_threshold_control,
    }

    /// USBSTS USB Status Register.
    pub struct EhciUsbStatus: u32 {
        /// The Host Controller sets this bit to 1 on the completion of a USB 
        /// transaction, which results in the retirement of a Transfer Descriptor
        /// that had its IOC bit set.
        /// 
        /// The Host Controller also sets this bit to 1 when a short packet is 
        /// detected (actual number of bytes received was less than the expected 
        /// number of bytes).
        [00..01] usb_int,
        /// The Host Controller sets this bit to 1 when completion of a USB 
        /// transaction results in an error condition (e.g., error counter 
        /// underflow). If the TD on which the error interrupt occurred also had 
        /// its IOC bit set, both this bit and USBINT bit are set. 
        [01..02] usb_error_int,
        /// The Host Controller sets this bit to a one when any port for which 
        /// the Port Owner bit is set to zero has a change bit transition from 
        /// a zero to a one or a Force Port Resume bit transition from a zero 
        /// to a one as a result of a J-K transition detected on a suspended port. 
        /// This bit will also be set as a result of the Connect Status Change 
        /// being set to a one after system software has relinquished ownership 
        /// of a connected port by writing a one to a port's Port Owner bit.
        /// 
        /// This bit is allowed to be maintained in the Auxiliary power well. 
        /// Alternatively, it is also acceptable that on a D3 to D0 transition 
        /// of the EHCI HC device, this bit is loaded with the OR of all of the 
        /// PORTSC change bits (including: Force port resume, over-current
        /// change, enable/disable change and connect status change).
        [02..03] port_change_detect,
        /// The Host Controller sets this bit to a one when the Frame List Index 
        /// rolls over from its maximum value to zero. The exact value at which 
        /// the rollover occurs depends on the frame list size. For example, if
        /// the frame list size (as programmed in the Frame List Size field of 
        /// the USBCMD register) is 1024, the Frame Index Register rolls over 
        /// every time FRINDEX[13] toggles. Similarly, if the size is 512, the 
        /// Host Controller sets this bit to a one every time FRINDEX[12] toggles.
        [03..04] frame_list_rollover,
        /// The Host Controller sets this bit to 1 when a serious error occurs 
        /// during a host system access involving the Host Controller module. 
        /// In a PCI system, conditions that set this bit to 1 include PCI Parity 
        /// error, PCI Master Abort, and PCI Target Abort. When this error occurs, 
        /// the Host Controller clears the Run/Stop bit in the Command register 
        /// to prevent further execution of the scheduled TDs.
        [04..05] host_system_error,
        /// Default 0. System software can force the host controller to issue an 
        /// interrupt the next time the host controller advances the asynchronous 
        /// schedule by writing a one to the Interrupt on Async Advance Doorbell 
        /// bit in the USBCMD register. This status bit indicates the assertion 
        /// of that interrupt source.
        [05..06] int_on_async_advance,
        /// Default 1. This bit is a zero whenever the Run/Stop bit is a one. The
        /// Host Controller sets this bit to one after it has stopped executing 
        /// as a result of the Run/Stop bit being set to 0, either by software 
        /// or by the Host Controller hardware (e.g. internal error).
        /// 
        /// Read only.
        [12..13] host_controller_halted,
        /// This is a read-only status bit, which is used to detect an empty 
        /// asynchronous schedule.
        ///  
        /// Read only.
        [13..14] reclamation,
        /// Default 0. The bit reports the current real status of the Periodic 
        /// Schedule. If this bit is a zero then the status of the Periodic 
        /// Schedule is disabled. If this bit is a one then the status of the 
        /// Periodic Schedule is enabled. The Host Controller is not required 
        /// to immediately disable or enable the Periodic Schedule when software 
        /// transitions the Periodic Schedule Enable bit in the USBCMD register.
        /// 
        /// When this bit and the Periodic Schedule Enable bit are the same value, 
        /// the Periodic Schedule is either enabled (1) or disabled (0).
        /// 
        /// Read only.
        [14..15] periodic_schedule_status,
        /// Default 0. The bit reports the current real status of the Asynchronous 
        /// Schedule. If this bit is a zero then the status of the Asynchronous 
        /// Schedule is disabled. If this bit is a one then the status of the 
        /// Asynchronous Schedule is enabled. The Host Controller is not required 
        /// to immediately disable or enable the Asynchronous Schedule when 
        /// software transitions the Asynchronous Schedule Enable bit in the USBCMD 
        /// register. When this bit and the Asynchronous Schedule Enable bit are 
        /// the same value, the Asynchronous Schedule is either enabled (1) or 
        /// disabled (0).
        /// 
        /// Read only.
        [15..16] async_schedule_status,
    }

    /// USB Interrupt Enable Register.
    pub struct EhciUsbInt: u32 {
        /// When this bit is a one, and the USBINT bit in the USBSTS register is 
        /// a one, the host controller will issue an interrupt at the next interrupt 
        /// threshold. The interrupt is acknowledged by software clearing the USBINT 
        /// bit.
        [00..01] usb_int_enable,
        /// When this bit is a one, and the USBERRINT bit in the USBSTS register is 
        /// a one, the host controller will issue an interrupt at the next interrupt 
        /// threshold. The interrupt is acknowledged by software clearing the 
        /// USBERRINT bit.
        [01..02] usb_error_int_enable,
        /// When this bit is a one, and the Port Change Detect bit in the USBSTS 
        /// register is a one, the host controller will issue an interrupt. The 
        /// interrupt is acknowledged by software clearing the Port Change Detect 
        /// bit.
        [02..03] port_change_int_enable,
        /// When this bit is a one, and the Frame List Rollover bit in the USBSTS 
        /// register is a one, the host controller will issue an interrupt. The 
        /// interrupt is acknowledged by software clearing the Frame List Rollover 
        /// bit.
        [03..04] frame_list_rollover_enable,
        /// When this bit is a one, and the Host System Error Status bit in the 
        /// USBSTS register is a one, the host controller will issue an interrupt. 
        /// The interrupt is acknowledged by software clearing the Host System Error 
        /// bit.
        [04..05] host_system_error_enable,
        /// When this bit is a one, and the Interrupt on Async Advance bit in the 
        /// USBSTS register is a one, the host controller will issue an interrupt 
        /// at the next interrupt threshold. The interrupt is acknowledged by software
        /// clearing the Interrupt on Async Advance bit.
        [05..06] async_advance_int_enable,
    }

    /// Frame Index Register.
    pub struct EhciFrameIndex: u32 {
        /// The value in this register increments at the end of each time frame 
        /// (e.g. micro-frame). Bits [N:3] are used for the Frame List current 
        /// index. This means that each location of the frame list is accessed 
        /// 8 times (frames or micro-frames) before moving to the next index.
        [00..14] frame_index,
    }

    pub struct EhciConfigFlag: u32 {
        /// Default 0. Host software sets this bit as the last action in its 
        /// process of configuring the Host Controller. This bit controls the 
        /// default port-routing control logic. Bit values and side-effects 
        /// are listed below:
        /// - 0 - Port routing control logic default-routes each port to an 
        ///   implementation dependent classic host controller.
        /// - 1 - Port routing control logic default-routes all ports to this 
        ///   host controller.
        /// 
        /// Read/Write.
        [0..1] configure_flag,
    }

    pub struct EhciPortStatusControl: u32 {
        /// Default 0. 1=Device is present on port. 0=No device is present.
        /// 
        /// This value reflects the current state of the port, and may not 
        /// correspond directly to the event that caused the Connect Status 
        /// Change bit (Bit 1) to be set.
        /// 
        /// Read only.
        [00..01] current_connect_status,
        /// Default 0. 1=Change in Current Connect Status. 0=No change.
        /// 
        /// Indicates a change has occurred in the port’s Current Connect 
        /// Status. The host controller sets this bit for all changes to 
        /// the port device connect status, even if system software has not 
        /// cleared an existing connect status change. For example, the
        /// insertion status changes twice before system software has 
        /// cleared the changed condition, hub hardware will be “setting” an 
        /// already-set bit (i.e., the bit will remain set).
        /// 
        /// Software sets this bit to 0 by writing a 1 to it.
        [01..02] connect_status_change,
        /// Default 0. 1=Enable. 0=Disable.
        /// 
        /// Ports can only be enabled by the host controller as a part of the 
        /// reset and enable. Software cannot enable a port by writing a one 
        /// to this field. The host controller will only set this bit to a one 
        /// when the reset sequence determines that the attached device is a 
        /// high-speed device.
        /// 
        /// Ports can be disabled by either a fault condition (disconnect event 
        /// or other fault condition) or by host software. Note that the bit 
        /// status does not change until the port state actually changes. There 
        /// may be a delay in disabling or enabling a port due to other host 
        /// controller and bus events.
        /// 
        /// When the port is disabled (0b) downstream propagation of data is 
        /// blocked on this port, except for reset. 
        /// 
        /// This field is zero if Port Power is zero.
        /// 
        /// Read/Write
        [02..03] enabled,
        /// Default 0. 1=Port enabled/disabled status has changed. 0=No change.
        /// 
        /// For the root hub, this bit gets set to a one only when a port is 
        /// disabled due to the appropriate conditions existing at the EOF2 
        /// point. Software clears this bit by writing a 1 to it.
        /// 
        /// This field is zero if Port Power is zero.
        [03..04] enabled_change,
        /// Default 0. 1=This port currently has an over-current condition. 
        /// 0=This port does not have an over-current condition.
        /// 
        /// This bit will automatically transition from a one to a zero when 
        /// the over current condition is removed.
        /// 
        /// Read only.
        [04..05] overcurrent_active,
        /// Default 0. 1=This bit gets set to a one when there is a change to 
        /// Over-current Active. 
        /// 
        /// Software clears this bit by writing a one to this bit position.
        [05..06] overcurrent_change,
        /// Default 0. 1= Resume detected/driven on port. 0=No resume (K-state) 
        /// detected/driven on port.
        /// 
        /// This functionality defined for manipulating this bit depends on the 
        /// value of the Suspend bit. For example, if the port is not suspended 
        /// (*Suspend* and *Enabled* bits are a one) and software transitions 
        /// this bit to a one, then the effects on the bus are undefined.
        ///
        /// Software sets this bit to a 1 to drive resume signaling. The Host 
        /// Controller sets this bit to a 1 if a J-to-K transition is detected 
        /// while the port is in the Suspend state. When this bit transitions 
        /// to a one because a J-to-K transition is detected, the *Port Change 
        /// Detect* bit in the USBSTS register is also set to a one. If software 
        /// sets this bit to a one, the host controller must not set the *Port 
        /// Change Detect* bit.
        /// 
        /// Read/Write
        [06..07] force_port_resume,
        /// Default 0. 1=Port in suspend state. 0=Port not in suspend state.
        /// 
        /// Port Enabled Bit and Suspend bit of this register define the port
        /// states as follows:
        /// - enabled=0, suspend=? - Disable
        /// - enabled=1, suspend=0 - Enable
        /// - enabled=1, suspend=1 - Suspend
        /// 
        /// When in suspend state, downstream propagation of data is blocked on
        ///  this port, except for port reset. The blocking occurs at the end of 
        /// the current transaction, if a transaction was in progress when this 
        /// bit was written to 1. In the suspend state, the port is sensitive
        /// to resume detection. Note that the bit status does not change until 
        /// the port is suspended and that there may be a delay in suspending a 
        /// port if there is a transaction currently in progress on the USB.
        /// 
        /// Read/Write
        [07..08] suspend,
        /// Default 0. 1=Port is in Reset. 0=Port is not in Reset.
        /// 
        /// When software writes a one to this bit (from a zero), the bus reset 
        /// sequence as defined in the USB Specification Revision 2.0 is started. 
        /// Software writes a zero to this bit to terminate the bus reset sequence. 
        /// Software must keep this bit at a one long enough to ensure the reset 
        /// sequence, as specified in the USB Specification Revision 2.0, completes. 
        /// 
        /// Note: when software writes this bit to a one, it must also write a zero 
        /// to the Port Enable bit.
        /// 
        /// Read/Write
        [08..09] reset,
        /// These bits reflect the current logical levels of the D+ (bit 11) and D- 
        /// (bit 10) signal lines. These bits are used for detection of low-speed 
        /// USB devices prior to the port reset and enable sequence. This field is 
        /// valid only when the port enable bit is zero and the current connect 
        /// status bit is set to a one.
        /// 
        /// The encoding of the bits are:
        /// - 0 - SE0 - Not Low-speed device, perform EHCI reset
        /// - 1 - J-state - Not Low-speed device, perform EHCI reset
        /// - 2 - K-state - Low-speed device, release ownership of port
        /// - 3 - Undefined - Not Low-speed device, perform EHCI reset.
        /// 
        /// This value of this field is undefined if Port Power is zero.
        /// 
        /// Read only.
        [10..12] line_status,
        /// The function of this bit depends on the value of the Port Power Control 
        /// (PPC) field in the HCSPARAMS register. The behavior is as follows:
        /// - ppc=0, pp=1 - Read-only, Host controller does not have port power 
        ///   control switches. Each port is hard-wired to power
        /// - ppc=1, pp=? - Read/Write, Host controller has port power control 
        ///   switches. This bit represents the current setting of the switch 
        ///   (0 = off, 1 = on). When power is not available on a port (i.e. PP 
        ///   equals a 0), the port is non-functional and will not report attaches, 
        ///   detaches, etc.
        /// 
        /// When an over-current condition is detected on a powered port and PPC is 
        /// a one, the PP bit in each affected port may be transitioned by the host 
        /// controller from a 1 to 0 (removing power from the port).
        /// 
        /// Read/Write (conditionnaly).
        [12..13] port_power,
        /// Default 1.
        /// 
        /// This bit unconditionally goes to a 0b when the Configured bit in the 
        /// CONFIGFLAG register makes a 0b to 1b transition. This bit unconditionally 
        /// goes to 1b whenever the Configured bit is zero.
        /// 
        /// System software uses this field to release ownership of the port to a 
        /// selected host controller (in the event that the attached device is not 
        /// a high-speed device). Software writes a one to this bit when the attached 
        /// device is not a high-speed device. A one in this bit means that a companion
        /// host controller owns and controls the port. 
        /// 
        /// Read/Write.
        [13..14] port_owner,
        /// Default 0.
        /// 
        /// Writing to these bits has no effect if the P_INDICATOR bit in the HCSPARAMS 
        /// register is a zero. If P_INDICATOR bit is a one, then the bit encodings are:
        /// - 0 - Port indicators are off
        /// - 1 - Amber
        /// - 2 - Green
        /// 
        /// Refer to the USB Specification Revision 2.0 for a description on how these 
        /// bits are to be used.
        /// 
        /// This field is zero if Port Power is zero.
        [14..16] port_indicator_control,
        /// Default 0.
        /// 
        /// When this field is zero, the port is NOT operating in a test mode. A non-zero 
        /// value indicates that it is operating in test mode and the specific test mode 
        /// is indicated by the specific value. The encoding of the test mode bits are 
        /// (0110b - 1111b are reserved):
        /// - 0000b - Test mode not enabled
        /// - 0001b - Test J-state
        /// - 0010b - Test K-state
        /// - 0011b - Test SE0 state
        /// - 0100b - Test packet
        /// - 0101b - Test force enable
        /// 
        /// Read/Write.
        [16..20] port_test_control,
        /// Default 0.
        /// 
        /// Writing this bit to a one enables the port to be sensitive to device connects 
        /// as wake-up events.
        /// 
        /// This field is zero if Port Power is zero.
        /// 
        /// Read/Write
        [20..21] wake_on_connect_enable,
        /// Default 0.
        /// 
        /// Writing this bit to a one enables the port to be sensitive to device disconnects 
        /// as wake-up events. 
        /// 
        /// This field is zero if Port Power is zero.
        /// 
        /// Read/Write
        [21..22] wake_on_disconnect_enable,
        /// Default 0.
        /// 
        /// Writing this bit to a one enables the port to be sensitive to over-current 
        /// conditions as wake-up events.
        /// 
        /// This field is zero if Port Power is zero.
        /// 
        /// Read/Write
        [22..23] wake_on_overcurrent_enable,
    }

}
