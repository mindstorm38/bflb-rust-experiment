//! EHCI abstraction layer for USB.

use crate::host::{HostController, RequestBlock, EndpointConfig};

pub mod reg;
pub mod data;

use reg::Ehci;
use data::{Qh, PeriodicFrameList};

use self::data::NextLinkType;


pub const PERIODIC_FRAME_COUNT: usize = 1024;
pub const PERIODIC_QH_COUNT: usize = 11;


/// Host controller implementation for EHCI platforms.
pub struct EhciHostController<'a> {
    /// EHCI memory-mapped registers.
    registers: Ehci,
    /// The Asynchronous Transfer List (based at the ASYNCLISTADDR register), 
    /// is where all the control and bulk transfers are managed. Host controllers 
    /// use this list only when it reaches the end of the periodic list, the 
    /// periodic list is disabled, or the periodic list is empty.
    async_queue_head: &'a mut Qh,
    /// TODO
    periodic_queue_head: &'a mut [Qh; PERIODIC_QH_COUNT],
    /// This schedule is for all periodic transfers (isochronous and interrupt).
    /// The periodic schedule is referenced from the operational registers space 
    /// using the PERIODICLISTBASE address register and the FRINDEX register. 
    periodic_frame_list: &'a mut PeriodicFrameList<PERIODIC_FRAME_COUNT>,
}

impl<'a> EhciHostController<'a> {

    pub fn new(
        registers: Ehci, 
        async_queue_head: &'a mut Qh, 
        periodic_queue_head: &'a mut [Qh; PERIODIC_QH_COUNT],
        periodic_frame_list: &'a mut PeriodicFrameList<PERIODIC_FRAME_COUNT>,
    ) -> Self {
        Self {
            registers,
            async_queue_head,
            periodic_queue_head,
            periodic_frame_list,
        }
    }

}

impl HostController for EhciHostController<'_> {

    fn init(&mut self) {
        
        let async_qh = &mut *self.async_queue_head;
        *async_qh = Qh::default();

        let async_qh_link_type = NextLinkType::Qh(async_qh);
        async_qh.horizontal_link.set_valid(async_qh_link_type);
        async_qh.endpoint_characteristics.head_of_reclamation_list_flag().fill();
        async_qh.overlay.next_link.set_invalid();
        async_qh.overlay.atl_next_link.set_invalid();
        async_qh.overlay.token.halted().fill();

        for link in &mut self.periodic_frame_list.list {
            link.0 = 0; // We use the zero value as a marker.
        }

        for (i, qh_head) in self.periodic_queue_head.iter_mut().enumerate().rev() {
            
            *qh_head = Qh::default();
            qh_head.horizontal_link.set_invalid();
            qh_head.endpoint_capabilities.int_schedule_mask().set(1);
            qh_head.overlay.next_link.set_invalid();
            qh_head.overlay.atl_next_link.set_invalid();
            qh_head.overlay.token.halted().fill();

            let interval = 1 << i;
            for j in (interval - 1)..PERIODIC_FRAME_COUNT {

                let link = &mut self.periodic_frame_list.list[j];

                if link.0 == 0 {
                    link.set_valid(NextLinkType::Qh(qh_head));
                } else if let Some(NextLinkType::Qh(qh)) = link.get_valid() {

                    let mut qh = unsafe { &mut *qh };

                    // TODO: Rework this horrible mutable pointers unsafety.
                    while !std::ptr::eq(qh, qh_head) {
                        if let Some(NextLinkType::Qh(next_qh)) = qh.horizontal_link.get_valid() {
                            qh = unsafe { &mut *next_qh };
                        } else {
                            qh.horizontal_link.set_valid(NextLinkType::Qh(qh_head));
                            break;
                        }
                    }

                }

            }

        }

        let hcor = self.registers.hcor;

        // Reset and wait for hardware.
        hcor.usb_cmd().modify(|reg| reg.host_controller_reset().fill());
        while hcor.usb_cmd().get().host_controller_reset().get() != 0 {}

        // Disable all interuptions for now.
        hcor.usb_int().modify(|reg| reg.0 = 0);

        // Addresses are on 32 bit, so segment address (most significant 32 bits) is 0.
        hcor.ctrlds_segment().set(0);
        hcor.async_list_addr().set(self.async_queue_head as *mut _ as usize as u32);
        hcor.periodic_list_base().set(self.periodic_frame_list as *mut _ as usize as u32);

        // Initialize parameters.
        hcor.usb_cmd().modify(|reg| {
            reg.frame_list_size().set(0);       // Type 0 = 1024 frames
            reg.int_threshold_control().set(1); // 1 micro-frame
            reg.async_schedule_enable().fill();
            reg.periodic_schedule_enable().fill();
            reg.run_stop().fill();
        });

        // Wait for EHCI to run.
        while hcor.usb_status().get().host_controller_halted().get() != 0 {}

        // Enable EHCI interrupts.
        hcor.usb_int().modify(|reg| {
            reg.usb_int_enable().fill();
            reg.usb_error_int_enable().fill();
            reg.port_change_int_enable().fill();
            reg.host_system_error_enable().fill();
            reg.async_advance_int_enable().fill();
        });

    }

    fn get_frame_number(&self) -> u16 {
        todo!()
    }

    fn alloc_pipe(&mut self, config: &EndpointConfig) -> usize {
        todo!()
    }

    fn submit_urb(&mut self, pipe: usize, urb: &RequestBlock) {

    }

}

impl EhciHostController<'_> {

}


struct EhciPipe {

}
