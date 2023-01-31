//! Embedded Runtime

#![no_std]
#![no_main]
#![crate_type = "rlib"]


#[cfg(not(rt_chip_ok))]
compile_error!("no runtime chip selected, use the crate features to select one chip");


// The following chip-specific modules must provides the same set of
// functions.

#[cfg(rt_chip = "bl808_m0")]
mod bl808_m0;
#[cfg(rt_chip = "bl808_m0")]
use crate::bl808_m0 as chip;

#[cfg(rt_chip = "bl808_d0")]
mod bl808_d0;
#[cfg(rt_chip = "bl808_d0")]
use crate::bl808_d0 as chip;


pub mod sym {

    //! Module providing externally linked symbols, defined either 
    //! by assembly or link script.
    
    // Here we define all linker script symbols.
    // Note that we can define all symbols as "u32", because 
    // the linker script forces alignments to 4 bytes.
    extern "C" {

        /// First word of the text section.
        pub static mut _ld_text_start: u32;
        /// First word **after** the text section.
        pub static mut _ld_text_end: u32;

        /// First word of the read-only data section.
        pub static mut _ld_rodata_start: u32;
        /// First word **after** the read-only section.
        pub static mut _ld_rodata_end: u32;

        /// First word of the data section in Flash that
        /// should be copied to RAM.
        pub static mut _ld_data_load_start: u32;
        /// First word of the read/write data section.
        pub static mut _ld_data_start: u32;
        /// First word **after** the read/write data section.
        pub static mut _ld_data_end: u32;

        /// First word of the read/write uninit data section.
        pub static mut _ld_bss_start: u32;
        /// First word **after** the read/write uninit data section.
        pub static mut _ld_bss_end: u32;

        /// First word of the stack.
        pub static mut _ld_stack_origin: u32;
        /// First word **after** the stack.
        pub static mut _ld_stack_top: u32;

        /// The default Machine Trap Generic Handler that is implemented
        /// in assembly and handles context saving and handling via
        /// exception/interrupt handlers that can be registered using
        /// [`register_exception_handler`] and 
        /// [`register_interrupt_handler`].
        /// 
        /// ***It should only be called by hardware on interrupt, only use
        /// this as the symbol.***
        pub fn _mtrap_generic_handler() -> !;

    }

}


// Re-export HAL.
pub use bflb_hal as hal;

// These modules are intentionnaly internal.
mod clic;
mod trap;

/// Re-exports
pub use chip::{get_interrupt_threshold, set_interrupt_threshold};
pub use trap::TrapHandler;

// Internal use.
use trap::TrapHandlers;
use hal::irq::{IrqNum, IRQ_COUNT};
use core::sync::atomic::{AtomicBool, Ordering};


/// All exception (synchronous) handlers.
static EXCEPTION_HANDLERS: TrapHandlers<32> = TrapHandlers::new();
/// All interrupt (asynchronous) handlers.
static INTERRUPT_HANDLERS: TrapHandlers<IRQ_COUNT> = TrapHandlers::new();

/// Internally used for default atomic value for taken arrays.
const TAKEN_DEFAULT: AtomicBool = AtomicBool::new(false);
/// Taken variables for interrupt handles.
static INTERRUPT_TAKEN_ARR: [AtomicBool; IRQ_COUNT] = [TAKEN_DEFAULT; IRQ_COUNT];


/// Use this macro a single time to define your application.
/// This macro is responsible of defining many symbols required
/// for startup.
/// 
/// Example:
/// ```
/// entry!(main)
/// ```
#[macro_export]
macro_rules! entry {
    ($entry:expr) => {

        #[no_mangle]
        extern "C" fn _rust_entry() {
            $entry();
        }

    };
}


/// This function is responsible for loading mutable static variables 
/// and zero-out uninitialized variables, all in RAM. 
/// 
/// *It is called just before entry by the assembly.*
#[no_mangle]
unsafe extern "C" fn _rust_ram_load() {

    // Copy mutable global variables to RAM.
    let src: *mut u32 = &mut sym::_ld_data_load_start;
    let dst: *mut u32 = &mut sym::_ld_data_start;
    let dst_end: *mut u32 = &mut sym::_ld_data_end;
    core::ptr::copy(src, dst, dst_end.offset_from(dst) as _);

    // Zero BSS uninit variables.
    let dst: *mut u32 = &mut sym::_ld_bss_start;
    let dst_end: *mut u32 = &mut sym::_ld_bss_end;
    core::ptr::write_bytes(dst, 0, dst_end.offset_from(dst) as _);

}


/// This function is responsible for initializing the system before
/// calling the entry point.
#[no_mangle]
extern "C" fn _rust_init() {
    chip::init();
}


/// It is called on any trap after the context has been saved in
/// the most efficient way (when possible, only dirty registers
/// should be saved).
#[no_mangle]
extern "C" fn _rust_mtrap_handler(cause: usize) {
    
    // Interrupt bit is at XLEN - 1
    const INTERRUPT_MASK: usize = 1 << (usize::BITS - 1);
    // Only use the lower 12 bits in order to be compliant with
    // CLIC mode where only those are used for the code.
    const CODE_MASK: usize = 0xFFF;

    let code = cause & CODE_MASK;
    let interrupt = cause & INTERRUPT_MASK != 0;

    let handler = if interrupt {
        INTERRUPT_HANDLERS.get(code, default_trap_handler)
    } else {
        EXCEPTION_HANDLERS.get(code, default_trap_handler)
    };

    (handler)(code);

}


/// The default trap handler, do nothing.
pub fn default_trap_handler(code: usize) {
    let _ = code;
}


/// Acquire control of a particular interrupt, if not possible this function 
/// will panic, indicating a logical programming error.
/// 
/// In case of success, a guard is returned that will release the interrupt
/// when dropped and goes out of scope.
pub fn take_interrupt(num: IrqNum) -> InterruptGuard {

    INTERRUPT_TAKEN_ARR[num as usize].compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .expect("interrupt is already owned and cannot be borrowed");

    InterruptGuard { num }

}

/// Use this structure to manage a particular interrupt.
pub struct InterruptGuard {
    num: IrqNum,
}

impl InterruptGuard {

    /// Get the current trap handler for this interrupt.
    pub fn get_handler(&self) -> TrapHandler {
        INTERRUPT_HANDLERS.get(self.num as _, default_trap_handler)
    }

    /// Set the trap handler for this interrupt.
    /// 
    /// The given function will be called when an interrupt request is
    /// processed while the interrupt is enabled and has a sufficient 
    /// level compared to the global threshold.
    pub fn set_handler(&mut self, handler: TrapHandler) {
        INTERRUPT_HANDLERS.set(self.num as _, handler);
    }

    /// Get the enabled status of this interrupt.
    pub fn is_enabled(&self) -> bool {
        chip::is_interrupt_enabled(self.num as _)
    }

    /// Enable or not this interrupt. Need to be true for the trap
    /// handler to be called.
    pub fn set_enable(&mut self, enable: bool) {
        chip::set_interrupt_enabled(self.num as _, enable);
    }

    /// Return true if this interrupt is pending and the trap handler
    /// must be called.
    pub fn is_pending(&self) -> bool {
        chip::is_interrupt_pending(self.num as _)
    }

    /// Set the pending status of this interrupt.
    /// 
    /// **Note that** this might have no effect depending of the trigger
    /// configured with `set_trigger`.
    pub fn set_pending(&mut self, pending: bool) {
        chip::set_interrupt_pending(self.num as _, pending);
    }

    /// Get the interrupt's level.
    pub fn get_level(&self) -> u8 {
        chip::get_interrupt_level(self.num as _)
    }

    /// Set the interrupt level. This interrupt need to have a higher level
    /// than the global threshold in order to be processed.
    pub fn set_level(&mut self, level: u8) {
        chip::set_interrupt_level(self.num as _, level);
    }

    /// Get the type of trigger for this interrupt.
    pub fn get_trigger(&self) -> InterruptTrigger {
        chip::get_interrupt_trigger(self.num as _)
    }

    /// Set the type of trigger for this interrupt. Note that this can affect
    /// how `set_pending` can be read/write by software.
    pub fn set_trigger(&mut self, trigger: InterruptTrigger) {
        chip::set_interrupt_trigger(self.num as _, trigger);
    }

}

impl Drop for InterruptGuard {
    fn drop(&mut self) {
        INTERRUPT_TAKEN_ARR[self.num as usize].store(false, Ordering::Release);
    }
}


/// Trigger mode that can be configured for a particular interrupt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptTrigger {
    PositiveLevel,
    NegativeLevel,
    PositiveEdge,
    NegativeEdge,
}


/// This implementation of the panic handler will simply abort without any message.
#[panic_handler]
#[cfg(feature = "panic_abort")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: Instead of spin looping indefinitly, it might be possible to 
    // close clock gates and stop the core.
    loop { core::hint::spin_loop() }
}
