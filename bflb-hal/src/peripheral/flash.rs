//! Flash interaction on BL808.
//! 
//! The flash is communicating using QSPI.

use super::CpuGroup;
use super::Chip;
use super::addr;


/// Methods for Serial Flash (SF).
impl Chip {


    /// Read the flash JEDEC identifier. The flash is initialized depending
    /// on its JEDEC identifier because any QSPI flash may be linked to the
    /// chip, and SPI commands depends on the flash.
    /// 
    /// *Zero is returned in case of invalid flash identifier.*
    pub fn get_flash_jedec_id(&self) -> u32 {
        let mut reg = self.glb.hw_rsv1().get();
        if reg.flash_id_valid().get() != 0 {
            reg.flash_id().get()
        } else {
            0
        }
    }

    /// Read a value in flash via cache at the given address and generic type.
    /// 
    /// The given address is the address offset from the flash base.
    pub fn read_flash_via_cache<T>(&self, group: CpuGroup, bank: FlashBank, addr: usize) -> T {

        // Rebase the given address on flash base.
        let addr = addr::FLASH1_XIP_BASE | (addr & (addr::FLASH1_XIP_END - addr::FLASH1_XIP_BASE - 1));

        let image_offset = self.get_flash_image_offset(group, bank);
        self.set_flash_image_offset(group, bank, 0);
        let val = unsafe { (addr as *const T).read_volatile() };
        self.set_flash_image_offset(group, bank, image_offset);
        val

    }

    /// Get the configured flash image offset.
    pub fn get_flash_image_offset(&self, group: CpuGroup, bank: FlashBank) -> u32 {
        match (group, bank) {
            (CpuGroup::Group0, FlashBank::Bank0) => self.sf_ctrl.sf_id0_offset()
                .get().sf_id0_offset().get(),
            (CpuGroup::Group0, FlashBank::Bank1) => self.sf_ctrl.sf_bk2_id0_offset()
                .get().sf_bk2_id0_offset().get(),
            (CpuGroup::Group1, FlashBank::Bank0) => self.sf_ctrl.sf_id1_offset()
                .get().sf_id1_offset().get(),
            (CpuGroup::Group1, FlashBank::Bank1) => self.sf_ctrl.sf_bk2_id1_offset()
                .get().sf_bk2_id1_offset().get(),
        }
    }

    /// Get the configured flash image offset.
    pub fn set_flash_image_offset(&self, group: CpuGroup, bank: FlashBank, offset: u32) {
        match (group, bank) {
            (CpuGroup::Group0, FlashBank::Bank0) => self.sf_ctrl.sf_id0_offset()
                .modify(|reg| reg.sf_id0_offset().set(offset)),
            (CpuGroup::Group0, FlashBank::Bank1) => self.sf_ctrl.sf_bk2_id0_offset()
                .modify(|reg| reg.sf_bk2_id0_offset().set(offset)),
            (CpuGroup::Group1, FlashBank::Bank0) => self.sf_ctrl.sf_id1_offset()
                .modify(|reg| reg.sf_id1_offset().set(offset)),
            (CpuGroup::Group1, FlashBank::Bank1) => self.sf_ctrl.sf_bk2_id1_offset()
                .modify(|reg| reg.sf_bk2_id1_offset().set(offset)),
        }
    }

}


/// Methods for low-level interaction with Flash via QSPI.
impl Chip {

    pub fn is_sflash_sbus2_enable(&self) -> bool {
        let mut ctrl = self.sf_ctrl.sf_if2_ctrl_1().get();
        ctrl.sf_if2_en().get() != 0 && ctrl.sf_if2_fn_sel().get() != 0
    }

    // TODO:
    // - https://github.com/bouffalolab/bl_mcu_sdk/blob/master/bsp/board/bl808dk/board.c#L235
    // - https://github.com/bouffalolab/bl_mcu_sdk/blob/15401c29bdb093ba2755b7416b977f07029095e6/drivers/soc/bl808/std/src/bl808_sf_ctrl.c#L1830

    // /// Send a low-level serial command to via QSPI to the flash.
    // pub fn send_sflash_cmd(&self, cmd: &FlashCmd) {

    //     const BUSY_STATE_TIMEOUT: u32 = 5 * 320 * 1000;

    //     if self.is_sflash_sbus2_enable() {
    //         self.sf_ctrl.if2_sahb()
    //     } else {
    //         self.sf_ctrl.if1_sahb()
    //     }

    // }

}


/// The type of flash bank. 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashBank {
    Bank0,
    Bank1,
}


/// A command to send to the serial flash.
#[derive(Debug)]
pub struct FlashCmd {
    /// Read/write flag.
    pub rw: bool,
    /// Command mode.
    pub cmd_mode: FlashCmdMode,
    /// Address mode.
    pub addr_mode: FlashAddrMode,
    /// Address size.
    pub addr_size: u8,
    // TODO: https://github.com/bouffalolab/bl_mcu_sdk/blob/15401c29bdb093ba2755b7416b977f07029095e6/drivers/soc/bl808/std/include/bl808_sf_ctrl.h#L260
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashCmdMode {
    /// Command in 1 line mode.
    SingleLine,
    /// Command in 4 lines mode.
    QuadLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashAddrMode {
    /// Address in 1 line mode.
    SingleLine,
    /// Address in 2 lines mode.
    DoubleLine,
    /// Address in 4 lines mode.
    QuadLine,
}


// TODO: https://github.com/bouffalolab/bl_mcu_sdk/blob/15401c29bdb093ba2755b7416b977f07029095e6/drivers/soc/bl808/std/include/bl808_sflash.h
// #[repr(packed)]
// pub struct FlashCfg {
//     pub io_mode: FlashCfgIoMode,
//     pub cread_support: FlashCfgCReadSupport,
//     pub clock_delay: FlashCfgClockDelay,
//     pub clock_invert: FlashCfgClockInvert,
//     pub reset_cmd: u8,
//     pub reset_cread_cmd: u8,
//     // TODO: finish
// }


// emhal::mmio_reg! {

//     pub struct FlashCfgIoMode: u8 {
//         [0..4] if_mode,
//         [4..5] unwrap,
//         [5..6] addr_mode_32bits,
//     }

//     pub struct FlashCfgCReadSupport: u8 {
//         [0..1] cread_mode_support,
//         [1..2] read_mode_config,
//     }

//     pub struct FlashCfgClockDelay: u8 {
//         [0..4] delay,
//         [4..7] pad_delay,
//     }

//     pub struct FlashCfgClockInvert: u8 {
//         [0..1] clock_invert,
//         [1..2] rx_invert,
//         [2..5] pad_delay0,
//         [5..8] pad_delay1,
//     }

// }
