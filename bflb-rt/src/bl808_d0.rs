//! Module for BL808 D0 core runtime.
    
compile_error!("todo: rework bl808_d0 chip assembly");

#[cfg(not(target_arch = "riscv64"))]
compile_error!("bl808_d0 chip requires riscv64 target architecture");


// Include global startup assembly.
core::arch::global_asm!(include_str!("asm/bl808_d0.asm"));


/// 16 RISC-V codes + 64 BouffaloLab codes.
pub const IRQ_COUNT: usize = 16 + 64;


/// Chip-specific function to init system before entry point.
pub fn init() {

}
