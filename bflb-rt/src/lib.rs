//! Embedded runtime for BouffaloLab chips.

#![no_std]
#![no_main]
#![crate_type = "rlib"]

// Our crate provides a global allocator usable by alloc.
extern crate alloc;


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


/// Module providing externally linked symbols, defined either by 
/// assembly or link script.
pub mod sym {

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

// These modules are intentionally internal.
mod clic;
mod trap;
mod task;

/// Re-exports
pub use trap::TrapHandler;

// Internal use.
use hal::irq::{Interrupt, IRQ_COUNT};
use trap::TrapHandlers;
use static_alloc::Bump;


/// The global bump allocator.
#[global_allocator]
static ALLOCATOR: Bump<[u8; 4096]> = Bump::uninit();


/// All exception (synchronous) handlers.
static EXCEPTION_HANDLERS: TrapHandlers<32> = TrapHandlers::new();
/// All interrupt (asynchronous) handlers.
static INTERRUPT_HANDLERS: TrapHandlers<IRQ_COUNT> = TrapHandlers::new();


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


/// A trait that extends the HAL `Interrupt` structure and add methods
/// for setting and getting the trap handler for this particular 
/// interrupt.
pub trait InterruptExt {

    /// Get the current trap handler for this interrupt.
    fn get_handler(&self) -> TrapHandler;

    /// Set the trap handler for this interrupt.
    /// 
    /// The given function will be called when an interrupt request is
    /// processed while the interrupt is enabled and has a sufficient 
    /// level compared to the global threshold.
    fn set_handler(&mut self, handler: TrapHandler);

}

impl<const NUM: usize> InterruptExt for Interrupt<NUM> {

    fn get_handler(&self) -> TrapHandler {
        INTERRUPT_HANDLERS.get(NUM, default_trap_handler)
    }

    fn set_handler(&mut self, handler: TrapHandler) {
        INTERRUPT_HANDLERS.set(NUM, handler);
    }
}


/// The default trap handler, do nothing.
pub fn default_trap_handler(code: usize) {
    let _ = code;
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
    // TODO: Instead of spin looping indefinitely, it might be possible to 
    // close clock gates and stop the core.
    loop { core::hint::spin_loop() }
}
