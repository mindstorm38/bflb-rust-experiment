//! Timer management on BL808.

use core::time::Duration;
use core::arch::asm;

use super::CpuId;


/// Interface for clock management of the chip.
#[derive(Clone, Copy)]
pub struct ChipTimer<'a> {
    chip: &'a super::Chip,
}

impl super::Chip {
    /// Access the clock management to the clock management subsystem.
    #[inline(always)]
    pub fn timer(&self) -> ChipTimer<'_> {
        ChipTimer { chip: self }
    }
}


impl ChipTimer<'_> {

    /// A sleep methods using architecture insctruction to sleep a 
    /// precise and low duration, usually in us.
    /// 
    /// TODO: Refactor because we don't enable cache for now and
    /// our code is executed from flash.
    pub fn sleep_arch(&self, dur: Duration) -> Result<(), ()> {
        
        let clock = self.chip.clock();

        let cycles;
        let freq;

        match self.chip.get_cpu_id()? {
            CpuId::M0 => {
                freq = clock.get_m0_cpu_freq();
                cycles = [46, 46, 66, 69];
            }
            CpuId::D0 => {
                freq = clock.get_d0_cpu_freq();
                cycles = [5, 5, 62, 34];
            }
            CpuId::LP => {
                freq = clock.get_lp_cpu_freq();
                cycles = [6, 6, 55, 85];
            }
        }

        // Ticks per loop depends on location of code.
        let ticks = match (sleep_arch_asm as *const () as usize) >> 24 {
            0x22 | 0x62 => cycles[0],
            0x3F => cycles[2],
            0x3E => cycles[3],
            _ => cycles[3], // FIXME: temporary fix
            // _ => unreachable!("sleep_arch: unexpected function pointer")
        };

        unsafe { sleep_arch_asm(freq, dur.as_micros() as u32, ticks); }
        Ok(())

    }

    #[inline(always)]
    pub fn sleep_dummy_nop(&self) {
        unsafe {
            asm!(
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
            )
        }
    }

}


#[inline(never)]
unsafe fn sleep_arch_asm(freq: u32, duration_us: u32, ticks: u32) {

    let cycles;
    let speed;

    if freq >= 1_000_000 {
        speed = freq / 100_000;
        cycles = (speed * duration_us) / 10 / ticks;
    } else {
        speed = freq / 1000;
        cycles = (speed * duration_us) / 1000 / ticks;
    }

    if cycles == 0 {
        return;
    }

    asm!(
        "mv a4, {0}",
        "li a5, 0",
        "nop",
        "nop",
        "nop",
        ".align 4",
        "1:",
        "beq a5, a4, 2f",
        "addi a5, a5, 1",
        "lui a3, 0xF0000",
        "lw a3, 0(a3)",
        "j 1b",
        "nop",
        "nop",
        "2:",
        "nop",
        in(reg) cycles,
        out("x13") _,
        out("x14") _,
        out("x15") _,
    );

}
