//! Module for BL808 M0 core runtime.

#[cfg(not(target_arch = "riscv32"))]
compile_error!("bl808_m0 chip requires riscv32 target architecture");

core::arch::global_asm!(include_str!("asm/bl808_m0.asm"));

use riscv_hal::clic::Clic;
use crate::clic::ClicVectorTable;


/// 16 RISC-V codes + 64 BouffaloLab codes.
pub const IRQ_COUNT: usize = 16 + 64;


/// On the M0/LP core, we use the CLIC for interruption handling.
const CLIC: Clic = Clic(0xE0800000 as _);


/// Machine Trap Vector Table.
#[no_mangle]
static mut _rust_mtrap_tvt: ClicVectorTable<IRQ_COUNT> = ClicVectorTable::new(crate::sym::_mtrap_generic_handler);


pub fn init() {

    let nlbits = CLIC.info().get().control_bits().get();
    CLIC.cfg().modify(|reg| reg.nlbits().set(nlbits));

    for irq_num in 0..IRQ_COUNT {
        let int = CLIC.int(irq_num);
        int.enable().set(0);
        int.pending().set(0);
        int.attr().modify(|reg| reg.vectored().fill());
    }

}

pub fn irq_set_enable(num: IrqNum, enable: bool) {
    CLIC.int(num as _).enable().set(enable as _);
}

pub fn irq_is_pending(num: IrqNum) -> bool {
    CLIC.int(num as _).pending().get() != 0
}


/// All supported IRQ types on BL808.
pub enum IrqNum {
    // Standard RISC-V IRQs.
    SupervisorSoftware  = 1,
    MachineSofware      = 3,
    SupervisorTimer     = 5,
    MachineTimer        = 7,
    SupervisorExternal  = 9,
    MachineExternal     = 11,
    // M0/LP IRQs.
    BmxMcuBusErr        = 16 + 0,
    BmxMcuTo            = 16 + 1,
    IpcM0               = 16 + 3,
    Audio               = 16 + 4,
    RfTopInt0           = 16 + 5,
    RfTopInt1           = 16 + 6,
    Lz4d                = 16 + 7,
    GaugeItf            = 16 + 8,
    SecEngId1           = 16 + 9,
    SecEngId0           = 16 + 10,
    SecEngId1Cdet       = 16 + 11,
    SecEngId0Cdet       = 16 + 12,
    SfCtrlId1           = 16 + 13,
    SfCtrlId0           = 16 + 14,
    Dma0All             = 16 + 15,
    Dma1All             = 16 + 16,
    Sdh                 = 16 + 17,
    MmAll               = 16 + 18,
    IrTx                = 16 + 19,
    IrRx                = 16 + 20,
    Usb                 = 16 + 21,
    Aupdm               = 16 + 22,
    Emac                = 16 + 24,
    GpadcDma            = 16 + 25,
    Efuse               = 16 + 26,
    Spi0                = 16 + 27,
    Uart0               = 16 + 28,
    Uart1               = 16 + 29,
    Uart2               = 16 + 30,
    GpioDma             = 16 + 31,
    I2c0                = 16 + 32,
    Pwn                 = 16 + 33,
    IpcRsvd             = 16 + 34,
    IpcLp               = 16 + 35,
    /// Timer0 Channel 0 Interrupt.
    Timer0Ch0           = 16 + 36,
    /// Timer0 Channel 1 Interrupt.
    Timer0Ch1           = 16 + 37,
    /// Timer0 Watch Dog Interrupt.
    Timer0Wdt           = 16 + 38,
    I2c1                = 16 + 39,
    I2s                 = 16 + 40,
    AnaOcpOutToCpu0     = 16 + 41,
    AnaOcpOutToCpu1     = 16 + 42,
    AnaOcpOutToCpu2     = 16 + 43,
    GpioInt0            = 16 + 44,
    Dm                  = 16 + 45,
    /// Bluetooh interrupt.
    Bl                  = 16 + 46,
    M154ReqAck          = 16 + 47,
    M154Int             = 16 + 48,
    M154Aes             = 16 + 49,
    PdsWakeUp           = 16 + 50,
    HbnOut0             = 16 + 51,
    HbnOut1             = 16 + 52,
    Bor                 = 16 + 53,
    Wifi                = 16 + 54,
    BzPhyInt            = 16 + 55,
    /// Bluetooh Low Energy interrupt.
    Ble                 = 16 + 56,
    MacTxRxTimer        = 16 + 57,
    MacTxRxMisc         = 16 + 58,
    MacRxTrigger        = 16 + 59,
    MacTxTrigger        = 16 + 60,
    MacGen              = 16 + 61,
    MacPortTrigger      = 16 + 62,
    WifiIpc             = 16 + 63,
}
