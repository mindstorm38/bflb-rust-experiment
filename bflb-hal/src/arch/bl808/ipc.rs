//! Inter-Processor Communication registers.
//! 
//! Sources:
//! - https://github.com/sipeed/M1s_BL808_SDK/blob/bf7e689cf49d57dd529e2492e5264ef602ed5b3c/components/platform/soc/bl808/bl808_std/BL808_BSP_Driver/StdDriver/Src/bl808_ipc.c
//! - https://github.com/sipeed/M1s_BL808_SDK/blob/bf7e689cf49d57dd529e2492e5264ef602ed5b3c/components/platform/soc/bl808/bl808_std/BL808_BSP_Driver/StdDriver/Inc/bl808_ipc.h

embedded_util::mmio! {
    pub struct Ipc {
        [0x000] rw cpu1_ipc_iswr: IpcCpuIswr,
        [0x004] rw cpu1_ipc_irsrr: IpcCpuIrsrr,
        [0x008] rw cpu1_ipc_icr: IpcCpuIcr,
        [0x00C] rw cpu1_ipc_iusr: IpcCpuIusr,
        [0x010] rw cpu1_ipc_iucr: IpcCpuIucr,
        [0x014] rw cpu1_ipc_ilslr: IpcCpuIlslr,
        [0x018] rw cpu1_ipc_ilshr: IpcCpuIlshr,
        [0x01C] rw cpu1_ipc_isr: IpcCpuIsr,
        [0x020] rw cpu0_ipc_iswr: IpcCpuIswr,
        [0x024] rw cpu0_ipc_irsrr: IpcCpuIrsrr,
        [0x028] rw cpu0_ipc_icr: IpcCpuIcr,
        [0x02C] rw cpu0_ipc_iusr: IpcCpuIusr,
        [0x030] rw cpu0_ipc_iucr: IpcCpuIucr,
        [0x034] rw cpu0_ipc_ilslr: IpcCpuIlslr,
        [0x038] rw cpu0_ipc_ilshr: IpcCpuIlshr,
        [0x03C] rw cpu0_ipc_isr: IpcCpuIsr,
    }
}

embedded_util::reg! {
    /// Interrupt Set Write.
    pub struct IpcCpuIswr: u32 {
        [00..16] cpu1_ipc_iswr,
    }
}

embedded_util::reg! {
    /// Interrupt raw status.
    pub struct IpcCpuIrsrr: u32 {
        [00..16] cpu1_ipc_irsrr,
    }
}

embedded_util::reg! {
    /// Interrupt Clear.
    pub struct IpcCpuIcr: u32 {
        [00..16] cpu1_ipc_icr,
    }
}

embedded_util::reg! {
    /// Interrupt Unmask Set.
    pub struct IpcCpuIusr: u32 {
        [00..16] cpu1_ipc_iusr,
    }
}

embedded_util::reg! {
    /// Interrupt Unmask Clear.
    pub struct IpcCpuIucr: u32 {
        [00..16] cpu1_ipc_iucr,
    }
}

embedded_util::reg! {
    /// Interrupt Line Sel Low.
    pub struct IpcCpuIlslr: u32 {
        [00..32] cpu1_ipc_ilslr,
    }
}

embedded_util::reg! {
    /// Interrupt Line Sel High.
    pub struct IpcCpuIlshr: u32 {
        [00..32] cpu1_ipc_ilshr,
    }
}

embedded_util::reg! {
    /// Interrupt status.
    pub struct IpcCpuIsr: u32 {
        [00..16] cpu1_ipc_isr,
    }
}