//! Embedded Runtime

#![no_std]
#![no_main]
#![crate_type = "rlib"]


#[cfg(all(not(rt_chip_ok), not(feature = "__dev")))]
compile_error!("no runtime chip selected, use the crate features to select one chip");

#[cfg(rt_chip = "bl808_d0")]
#[cfg(not(target_arch = "riscv64"))]
compile_error!("bl808_d0 chips requires riscv64 target architecture");

#[cfg(any(rt_chip = "bl808_m0", rt_chip = "bl808_lp"))]
#[cfg(not(target_arch = "riscv32"))]
compile_error!("bl808_m0/bl808_mp chips requires riscv32 target architecture");


pub mod asm;
pub mod trap;
pub mod chip;


/// Panic handler will only abort.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
