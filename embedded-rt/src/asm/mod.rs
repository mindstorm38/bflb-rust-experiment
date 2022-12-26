//! Board-specific assembly file inclusion.
//! 
//! This code is written in assembly because it's much easier 
//! for such cases.
//! 
//! We also centralize all cross calls between Assembly and 
//! Rust here using `extern "C"` definitions. The concrete
//! functions are responsible of dispatching calls to the
//! Rust modules' functions. *This is done this way in order
//! to provide an easy overview of interactions between
//! Rust and assembly.*
//! 
//! This module also defines symbols that are defined in
//! the linker script and used for RAM initialization.


#[cfg(rt_chip = "bl808_m0")]
core::arch::global_asm!(include_str!("bl808_m0.asm"));

#[cfg(rt_chip = "bl808_d0")]
core::arch::global_asm!(include_str!("bl808_d0.asm"));


extern "C" {

    // Note that we can define all symbols as "u32", because 
    // the linker script forces alignments to 4 bytes.

    pub static mut _ld_text_start: u32;
    pub static mut _ld_text_end: u32;

    pub static mut _ld_rodata_start: u32;
    pub static mut _ld_rodata_end: u32;

    pub static mut _ld_data_load_start: u32;
    pub static mut _ld_data_start: u32;
    pub static mut _ld_data_end: u32;

    pub static mut _ld_bss_start: u32;
    pub static mut _ld_bss_end: u32;

    pub static mut _ld_stack_origin: u32;
    pub static mut _ld_stack_top: u32;

}


/// Use this macro a single time to define the entry point of the 
/// program.
/// 
/// Example:
/// ```
/// entry! {
///     // my main code
/// }
/// ```
#[macro_export]
macro_rules! entry {
    ($($tok:tt)*) => {
        #[no_mangle]
        extern "C" fn asm_entry() {
            $($tok)*
        }
    };
}


/// This function is responsible for routing the trap machine-mode
/// trap handler to the corresponding module.
/// 
/// *It is called on any trap after the context has been saved in
/// the most efficient way (when possible, only dirty registers
/// should be saved).*
#[no_mangle]
extern "C" fn asm_mtrap_handler(cause: usize, val: usize) {
    crate::trap::trap_handler(cause, val);
}


/// This function is responsible for loading mutable static variables 
/// and zero-out uninitialized variables, all in RAM. 
/// 
/// *It is called just before entry by the assembly.*
#[no_mangle]
unsafe extern "C" fn asm_ram_load() {

    // Copy mutable global variables to RAM.
    let src: *mut u32 = &mut _ld_data_load_start;
    let dst: *mut u32 = &mut _ld_data_start;
    let dst_end: *mut u32 = &mut _ld_data_end;
    core::ptr::copy(src, dst, dst_end.offset_from(dst) as _);

    // Zero BSS uninit variables.
    let dst: *mut u32 = &mut _ld_bss_start;
    let dst_end: *mut u32 = &mut _ld_bss_end;
    core::ptr::write_bytes(dst, 0, dst_end.offset_from(dst) as _);

}
