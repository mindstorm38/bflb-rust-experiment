//! Clock functions for MM (MultiMedia) peripherals. 
//! Also known as DSP (Digital Signal Processor).

use crate::bl808::{MM_GLB, GLB};
use super::Clocks;
use super::PllAudioDiv;

impl Clocks {

    /// Get the selector for MM xclock.
    pub fn get_mm_xclk_sel(&self) -> MmXclkSel {
        match MM_GLB.mm_clk_ctrl_cpu().get().xclk_clk_sel().get() {
            0 => MmXclkSel::Rc32m,
            1 => MmXclkSel::Xtal,
            _ => unreachable!()
        }
    }

    /// Set the selector for MM xclock.
    pub fn set_mm_xclk_sel(&mut self, sel: MmXclkSel) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.xclk_clk_sel().set(sel as _));
    }

    /// Get the frequency for MM xclock.
    pub fn get_mm_xclk_freq(&self) -> u32 {
        match self.get_mm_xclk_sel() {
            MmXclkSel::Rc32m => 32_000_000,
            MmXclkSel::Xtal => self.get_xtal_freq(),
        }
    }

}

impl Clocks {

    /// Get the selector for MM PLL 160 MHz clock.
    pub fn get_mm_pll160_sel(&self) -> MmPll160Sel {
        match GLB.dig_clk_cfg1().get().mm_muxpll_160m_sel().get() {
            0 => MmPll160Sel::MmWifiPll160,
            1 => MmPll160Sel::CpuPll160,
            _ => unreachable!()
        }
    }

    /// Get the frequency for MM PLL 160 MHz clock.
    pub fn get_mm_pll160_freq(&self) -> u32 {
        match self.get_mm_pll160_sel() {
            MmPll160Sel::MmWifiPll160 => self.get_wifi_pll_freq(160_000_000),
            MmPll160Sel::CpuPll160 => self.get_cpu_pll_freq(160_000_000),
        }
    }

}

impl Clocks {

    /// Get the selector for MM PLL 240 MHz clock.
    pub fn get_mm_pll240_sel(&self) -> MmPll240Sel {
        match GLB.dig_clk_cfg1().get().mm_muxpll_240m_sel().get() {
            0 => MmPll240Sel::MmWifiPll240,
            1 => MmPll240Sel::MmAudioPllDiv2,
            _ => unreachable!()
        }
    }

    /// Get the frequency for MM PLL 240 MHz clock.
    pub fn get_mm_pll240_freq(&self) -> u32 {
        match self.get_mm_pll240_sel() {
            MmPll240Sel::MmWifiPll240 => self.get_wifi_pll_freq(240_000_000),
            MmPll240Sel::MmAudioPllDiv2 => self.get_audio_pll_freq(PllAudioDiv::Div2),
        }
    }

}

impl Clocks {

    /// Get the selector for MM PLL 320 MHz clock.
    pub fn get_mm_pll320_sel(&self) -> MmPll320Sel {
        match GLB.dig_clk_cfg1().get().mm_muxpll_320m_sel().get() {
            0 => MmPll320Sel::MmWifiPll320,
            1 => MmPll320Sel::MmAudioPllDiv1,
            _ => unreachable!()
        }
    }

    /// Get the frequency for MM PLL 320 MHz clock.
    pub fn get_mm_pll320_freq(&self) -> u32 {
        match self.get_mm_pll320_sel() {
            MmPll320Sel::MmWifiPll320 => self.get_wifi_pll_freq(320_000_000),
            MmPll320Sel::MmAudioPllDiv1 => self.get_audio_pll_freq(PllAudioDiv::Div1),
        }
    }

}

impl Clocks {

    /// Get the selector for MM bclock.
    pub fn get_mm_bclk1_sel(&self) -> MmBclk1Sel {
        match MM_GLB.mm_clk_ctrl_cpu().get().bclk1x_sel().get() {
            0 | 1 => MmBclk1Sel::MmXclk,
            2 => MmBclk1Sel::MmPll160,
            3 => MmBclk1Sel::MmPll240,
            _ => unreachable!()
        }
    }

    /// Get the frequency for MM bclock.
    pub fn get_mm_bclk1_freq(&self) -> u32 {
        match self.get_mm_bclk1_sel() {
            MmBclk1Sel::MmXclk => self.get_mm_xclk_freq(),
            MmBclk1Sel::MmPll160 => self.get_mm_pll160_freq(),
            MmBclk1Sel::MmPll240 => self.get_mm_pll240_freq()
        }
    }

}

impl Clocks {

    /// Get the selector for D0 PLL clock.
    pub fn get_d0_pll_sel(&self) -> D0PllSel {
        match MM_GLB.mm_clk_ctrl_cpu().get().cpu_clk_sel().get() {
            0 => D0PllSel::MmPll240,
            1 => D0PllSel::MmPll320,
            2 | 3 => D0PllSel::CpuPll400,
            _ => unreachable!()
        }
    }

    /// Get the frequency for D0 PLL clock.
    pub fn get_d0_pll_freq(&self) -> u32 {
        match self.get_d0_pll_sel() {
            D0PllSel::MmPll240 => self.get_mm_pll240_freq(),
            D0PllSel::MmPll320 => self.get_mm_pll320_freq(),
            D0PllSel::CpuPll400 => self.get_cpu_pll_freq(400_000_000),
        }
    }

}

impl Clocks {

    /// Get the selector for D0 root clock.
    pub fn get_d0_root_sel(&self) -> D0RootSel {
        match MM_GLB.mm_clk_ctrl_cpu().get().cpu_root_clk_sel().get() {
            0 => D0RootSel::MmXclk,
            1 => D0RootSel::D0Pll,
            _ => unreachable!()
        }
    }

    /// Set the selector for D0 root clock.
    pub fn set_d0_root_sel(&mut self, sel: D0RootSel) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.cpu_root_clk_sel().set(sel as _));
    }

    /// Get the frequency for D0 root clock.
    pub fn get_d0_root_freq(&self) -> u32 {
        match self.get_d0_root_sel() {
            D0RootSel::MmXclk => self.get_mm_xclk_freq(),
            D0RootSel::D0Pll => self.get_d0_pll_freq(),
        }
    }

}

impl Clocks {

    /// Get the divider applied to the frequency for D0 CPU frequency.
    pub fn get_d0_cpu_div(&self) -> u32 {
        MM_GLB.mm_clk_cpu().get().cpu_clk_div().get() + 1
    }

    /// Set the divider applied to the frequency for D0 CPU frequency.
    pub fn set_d0_cpu_div(&mut self, div: u32) {
        MM_GLB.mm_clk_cpu().modify(|reg| reg.cpu_clk_div().set(div - 1));
    }

    /// Get the final D0 CPU frequency.
    pub fn get_d0_cpu_freq(&self) -> u32 {
        let root_freq = self.get_d0_root_freq();
        root_freq / self.get_d0_cpu_div()
    }
    
    /// Enable or not the D0 core clock gate.
    pub fn set_d0_cpu_enable(&mut self, enable: bool) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| {
            reg.mmcpu0_clk_en().set(enable as _);
        });
    }

}

impl Clocks {

    /// Get the divider applied to the frequency for D0 secondary frequency.
    pub fn get_mm_bclk2_div(&self) -> u32 {
        MM_GLB.mm_clk_cpu().get().bclk2x_div().get() + 1
    }

    /// Get the divider applied to the frequency for D0 secondary frequency.
    pub fn set_mm_bclk2_div(&mut self, div: u32) {
        MM_GLB.mm_clk_cpu().modify(|reg| reg.bclk2x_div().set(div - 1));
    }

    pub fn set_mm_bclk2_div_act_pulse(&mut self, act: bool) {
        MM_GLB.mm_clk_ctrl_cpu().modify(|reg| reg.bclk2x_div_act_pulse().set(act as _));
    }

    pub fn get_mm_bclk2_prot_done(&self) -> bool {
        MM_GLB.mm_clk_ctrl_cpu().get().sts_bclk2x_prot_done().get() != 0
    }

    /// Get the final D0 secondary frequency.
    pub fn get_mm_bclk2_freq(&self) -> u32 {
        self.get_d0_cpu_freq() / self.get_mm_bclk2_div()
    }

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmXclkSel {
    Rc32m = 0,
    Xtal = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmPll160Sel {
    MmWifiPll160 = 0,
    CpuPll160 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmPll240Sel {
    MmWifiPll240 = 0,
    MmAudioPllDiv2 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmPll320Sel {
    MmWifiPll320 = 0,
    MmAudioPllDiv1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmBclk1Sel {
    MmXclk = 0, // or 1
    MmPll160 = 2,
    MmPll240 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum D0PllSel {
    MmPll240 = 0,
    MmPll320 = 1,
    CpuPll400 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum D0RootSel {
    MmXclk = 0,
    D0Pll = 1,
}
