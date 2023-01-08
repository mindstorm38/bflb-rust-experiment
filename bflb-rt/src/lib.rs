//! Embedded Runtime

#![no_std]
#![no_main]
#![crate_type = "rlib"]


#[cfg(all(not(rt_chip_ok), not(feature = "__dev")))]
compile_error!("no runtime chip selected, use the crate features to select one chip");


// The following chip-specific modules must provides the following functions:
// - init() 
//   This function is called before the entry point.
// - irq_set_enable(num: IrqNum, enable: bool)
//   Enable or not the given IRQ number.
// - irq_is_pending(num: IrqNum) -> bool
//   Query if a given IRQ number is pending.

#[cfg(rt_chip = "bl808_m0")]
mod bl808_m0;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

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

// The module is intentionnaly internal.
mod clic;

/// Re-export the IRQ type enumeration.
pub use chip::IrqNum;


/// Internal type to store a trap handler state.
struct TrapHandler {
    /// The closure that will be executed when trap is taken.
    /// We don't keep the lifetime because 
    closure: UnsafeCell<*mut dyn FnMut(usize, usize)>,
    /// Indicates if the closure is considered alive, this
    /// mean that it hasn't reached it's end of lifetime.
    alive: AtomicBool,
}

unsafe impl Sync for TrapHandler {}

impl TrapHandler {
    pub const fn new(default: *mut dyn FnMut(usize, usize)) -> Self {
        Self {
            closure: UnsafeCell::new(default),
            alive: AtomicBool::new(true),
        }
    }
}



/// All exception (synchronous) handlers.
static EXCEPTION_HANDLERS: [TrapHandler; 32] = [TrapHandler::new(default_exception_handler); 32];
/// All interrupt (asynchronous) handlers.
static INTERRUPT_HANDLERS: [TrapHandler; chip::IRQ_COUNT] = [default_trap_handler; chip::IRQ_COUNT];


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
extern "C" fn _rust_mtrap_handler(cause: usize, val: usize) {
    
    // Interrupt bit is at XLEN - 1
    const INTERRUPT_MASK: usize = 1 << (usize::BITS - 1);
    // Only use the lower 12 bits in order to be compliant with
    // CLIC mode where only those are used for the code.
    const CODE_MASK: usize = 0xFFF;

    let code = cause & CODE_MASK;
    let interrupt = cause & INTERRUPT_MASK != 0;

    if interrupt {
        unsafe { INTERRUPT_HANDLERS[code](code, val) }
    } else {
        unsafe { EXCEPTION_HANDLERS[code](code, val) }
    }

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


/// The default handler for exceptions, panicking with the code message if valid.
pub fn default_exception_handler(code: usize, val: usize) {

    let _ = val;
    let exc_message = match code {
        0 => "instruction address misaligned",
        1 => "instruction access fault",
        2 => "illegal instruction",
        3 => "breakpoint",
        4 => "load address misaligned",
        5 => "load access fault",
        6 => "store/amo address misaligned",
        7 => "store/amo access fault",
        8 => "environment call from user mode",
        9 => "environment call from supervisor mode",
        11 => "environment call from machine mode",
        12 => "instruction page fault",
        13 => "load page fault",
        15 => "store/amo page fault",
        _ => panic!("unhandled cpu exception with unknown code: {code}")
    };

    panic!("unhandled cpu exception: {exc_message}");

}


/// The default trap handler, do nothing.
pub fn default_trap_handler(code: usize, val: usize) {
    let _ = code;
    let _ = val;
}


/// Register an exception handler for the given exception code.
/// 
/// **This function is unsafe to call because no synchronization is guaranteed
/// when accessing the handlers table. Therefore you should call this on startup 
/// and ensure that it run on a single hart.**
pub unsafe fn register_exception_handler(exc: usize, handler: TrapHandler) {
    EXCEPTION_HANDLERS[exc] = handler;
}


/// Register an interrupt handler for the given IRQ number.
pub fn register_interrupt_handler<H: FnMut(usize, usize)>(num: usize, handler: &H) {
    let handler = &INTERRUPT_HANDLERS[num];
    

    INTERRUPT_HANDLERS[num] = handler;
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
