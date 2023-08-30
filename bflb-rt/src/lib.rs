//! Embedded runtime for BouffaloLab chips.

#![no_std]
#![no_main]
#![crate_type = "rlib"]

// Our crate provides a global allocator usable by alloc.
extern crate alloc;


#[cfg(not(rt_chip_ok))]
compile_error!("no runtime chip selected, use the crate features to select one chip");


/// Macro used to import the chip specific rust and assembly code
/// depending on the selected runtime chip. These modules should 
/// define the following standard functions, variables and constants:
/// - `pub fn init()`, this function is called prior to entry and 
///   should run chip specific initialization.
/// - `pub const HART_COUNT: usize`, a constant that define the
///   architectural maximum number of hart that can run on the chip.
///   This number may be greater than the actual number of activated
///   harts, but should not be under estimated because overflow harts
///   will panic.
macro_rules! use_chip {
    ($id:ident) => {
        mod $id;
        use crate::$id as chip;
    };
}

#[cfg(rt_chip = "bl808_m0")]
use_chip!(bl808_m0);
#[cfg(rt_chip = "bl808_d0")]
use_chip!(bl808_d0);


// Re-export HAL.
pub use bflb_hal as hal;

// These modules are intentionally internal.
mod clic;

// Internal use.
use hal::interrupt::{IRQ_COUNT, VECTOR};
use linked_list_allocator::LockedHeap;


/// Module providing externally linked symbols, defined either by 
/// assembly or link script.
pub mod sym {

    extern "C" {

        /// First word of the text section.
        pub static mut _ld_text_start: u32;
        /// First word **after** the text section.
        pub static mut _ld_text_end: u32;

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

        /// First word of the heap.
        pub static mut _ld_heap_start: u8;
        /// First word **after** the heap.
        pub static mut _ld_heap_end: u8;

        /// The default Machine Trap Generic Handler that is 
        /// implemented in assembly and handles context saving and 
        /// handling via exception/interrupt handlers that can be 
        /// registered using [`register_exception_handler`] and 
        /// [`register_interrupt_handler`].
        /// 
        /// ***It should only be called by hardware on interrupt, only 
        /// use this as the symbol.***
        pub fn _mtrap_generic_handler() -> !;

    }

}


/// The global bump allocator.
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();


/// All interrupt (asynchronous) handlers.
static INTERRUPT_VECTOR: [fn(usize); IRQ_COUNT] = VECTOR;


/// This function is responsible for loading mutable static variables 
/// and zero-out uninitialized variables, all in RAM. 
/// 
/// **This function is really important but will only trigger on 
/// hart 0, because memory should be initialized once, and most 
/// importantly we have kernel stack only on hart 0.**
/// 
/// *This function should not access global variables or allocate
/// memory.*
/// 
/// *It is called just before entry by the assembly and should not be
/// called from elsewhere because **it will break the program**.*
#[no_mangle]
unsafe extern "C" fn _rust_mem_init() {

    // Copy RO/RW global variables to RAM.
    let src: *mut u32 = &mut sym::_ld_data_load_start;
    let dst: *mut u32 = &mut sym::_ld_data_start;
    let dst_end: *mut u32 = &mut sym::_ld_data_end;
    core::ptr::copy(src, dst, dst_end.offset_from(dst) as _);

    // Zero BSS uninit variables.
    let dst: *mut u32 = &mut sym::_ld_bss_start;
    let dst_end: *mut u32 = &mut sym::_ld_bss_end;
    core::ptr::write_bytes(dst, 0, dst_end.offset_from(dst) as _);

    // // Init heap allocator.
    // let start: *mut u8 = &mut sym::_ld_heap_start;
    // let end: *mut u8 = &mut sym::_ld_heap_end;
    // ALLOCATOR.lock().init(start, end.offset_from(end) as _);

}


/// This function is responsible for initializing the system before
/// calling the entry point. This function internally initialize the
/// hart (with its unique ID, used for HartLocal variables) and the
/// chip **only on hart 0**.
#[no_mangle]
extern "C" fn _rust_init() {
    
    hal::hart::init();

    if hal::hart::hart_zero() {
        chip::init();
    }

}


/// This is the entry function, note that this function is called from
/// all hart. The first thread will be started on the first hart.
#[no_mangle]
extern "C" fn _rust_entry() -> ! {

    extern "Rust" {
        fn main();
    }

    if hal::hart::hart_zero() {
        unsafe { main() };
    }

    loop {
        hal::hart::spin_loop();
    }

}


/// It is called on any trap after the context has been saved in the 
/// most efficient way (when possible, only dirty registers should be 
/// saved).
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
        INTERRUPT_VECTOR[code]
    } else {
        // TODO:
        // EXCEPTION_HANDLERS.get(code, default_trap_handler)
        return
    };

    (handler)(code);

}


/// The default trap handler, do nothing.
pub fn default_trap_handler(code: usize) {
    let _ = code;
}


/// This implementation of the panic handler will simply abort without 
/// any message.
#[panic_handler]
#[cfg(feature = "panic-abort")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // TODO: Instead of spin looping indefinitely, it might be possible 
    // to close clock gates and stop the core.
    loop { hal::hart::spin_loop() }
}
