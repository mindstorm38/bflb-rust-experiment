//! Embedded Runtime

#![no_std]
#![no_main]
#![crate_type = "rlib"]


#[cfg(all(not(rt_chip_ok), not(feature = "__dev")))]
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

// These modules are intentionnaly internal.
mod clic;
mod trap;

/// Re-export the IRQ type enumeration.
pub use chip::{IrqNum, get_interrupt_threshold, set_interrupt_threshold};
pub use trap::TrapHandler;

// Internal use.
use trap::TrapHandlers;

/// All exception (synchronous) handlers.
static EXCEPTION_HANDLERS: TrapHandlers<32> = TrapHandlers::new();
/// All interrupt (asynchronous) handlers.
static INTERRUPT_HANDLERS: TrapHandlers<{chip::IRQ_COUNT}> = TrapHandlers::new();


/// The global writer to use to print panicking information when happening.
#[cfg(feature = "panic")]
static mut PANIC_WRITER: Option<&'static mut dyn core::fmt::Write> = None;


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


/// Get a handle to this particular interrupt, this handle allows you to
/// manage the interrupt handler and enable status.
pub fn get_interrupt(num: IrqNum) -> InterruptHandle {
    InterruptHandle { num }
}


/// Use this structure to manage a particular interrupt.
pub struct InterruptHandle {
    num: IrqNum,
}

impl InterruptHandle {

    /// Get the current trap handler for this interrupt.
    pub fn get_handler(&self) -> TrapHandler {
        INTERRUPT_HANDLERS.get(self.num as _, default_trap_handler)
    }

    /// Set the trap handler for this interrupt.
    /// 
    /// The given function will be called when an interrupt request is
    /// processed while the interrupt is enabled and has a sufficient 
    /// level compared to the global threshold.
    pub fn set_handler(&self, handler: TrapHandler) {
        INTERRUPT_HANDLERS.set(self.num as _, handler);
    }

    /// Get the enabled status of this interrupt.
    pub fn is_enabled(&self) -> bool {
        chip::is_interrupt_enabled(self.num as _)
    }

    /// Enable or not this interrupt. Need to be true for the trap
    /// handler to be called.
    pub fn set_enable(&self, enable: bool) {
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
    pub fn set_pending(&self, pending: bool) {
        chip::set_interrupt_pending(self.num as _, pending);
    }

    /// Get the interrupt's level.
    pub fn get_level(&self) -> u8 {
        chip::get_interrupt_level(self.num as _)
    }

    /// Set the interrupt level. This interrupt need to have a higher level
    /// than the global threshold in order to be processed.
    pub fn set_level(&self, level: u8) {
        chip::set_interrupt_level(self.num as _, level);
    }

    /// Get the type of trigger for this interrupt.
    pub fn get_trigger(&self) -> InterruptTrigger {
        chip::get_interrupt_trigger(self.num as _)
    }

    /// Set the type of trigger for this interrupt. Note that this can affect
    /// how `set_pending` can be read/write by software.
    pub fn set_trigger(&self, trigger: InterruptTrigger) {
        chip::set_interrupt_trigger(self.num as _, trigger);
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


/// Panic handler will only abort.
#[cfg(feature = "panic")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {

    use core::fmt::Write;

    // If some panic writer is available, use it.
    if let Some(write) = unsafe { PANIC_WRITER.as_mut() } {
        let _ = write!(write, "{info}");
    }

    loop {}

}

/// Register a panic writer to be used on panics.
/// 
/// **This function is unsafe to call because no synchronization is guaranteed
/// when accessing the global variable. Therefore you should call this on startup 
/// and ensure that it run on a single hart.**
#[cfg(feature = "panic")]
pub unsafe fn register_panic_writer(writer: &'static mut dyn core::fmt::Write) {
    PANIC_WRITER = Some(writer);
}


// /// The default handler for exceptions, panicking with the code message if valid.
// pub fn default_exception_handler(code: usize, val: usize) {

//     let _ = val;
//     let exc_message = match code {
//         0 => "instruction address misaligned",
//         1 => "instruction access fault",
//         2 => "illegal instruction",
//         3 => "breakpoint",
//         4 => "load address misaligned",
//         5 => "load access fault",
//         6 => "store/amo address misaligned",
//         7 => "store/amo access fault",
//         8 => "environment call from user mode",
//         9 => "environment call from supervisor mode",
//         11 => "environment call from machine mode",
//         12 => "instruction page fault",
//         13 => "load page fault",
//         15 => "store/amo page fault",
//         _ => panic!("unhandled cpu exception with unknown code: {code}")
//     };

//     panic!("unhandled cpu exception: {exc_message}");

// }


// /// Register an exception handler for the given exception code.
// /// 
// /// **This function is unsafe to call because no synchronization is guaranteed
// /// when accessing the handlers table. Therefore you should call this on startup 
// /// and ensure that it run on a single hart.**
// pub unsafe fn register_exception_handler(exc: usize, handler: TrapHandlerData) {
//     EXCEPTION_HANDLERS[exc] = handler;
// }


// /// Acquire the control of an IRQ if not already controlled.
// /// 
// /// The returned [`IrqGuard`] will automatically release the control of the IRQ
// /// at the end of its scope. In this interval you can freely change the handler
// /// for this specific interrupt, the handler is a closure that is lifetime
// /// constrained by the lifetime of the IRQ guard.
// /// 
// /// **Do not use this function inside an interrupt handler.
// pub fn acquire_irq(num: IrqNum) -> Option<IrqGuard> {
//     todo!("use this for acquiring control of an IRQ");
// }


// /// Register an interrupt handler for the given IRQ number.
// pub unsafe fn register_interrupt_handler<'a>(num: usize, handler: &'a mut dyn FnMut(usize, usize)) -> TrapGuard<'a> {
//     // let data = &mut INTERRUPT_HANDLERS[num];

//     // let closure = &mut *data.closure.get();
//     // *closure = handler;
//     // TrapGuard { closure: handler }
//     todo!()
// }
