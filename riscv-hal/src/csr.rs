//! This module defines many abstraction registers and access functions
//! for Control and Status Registers. This module only concerns the
//! main specification's CSRs. Extensions' CSRs are defined in 
//! specialized modules.


#[macro_export]
macro_rules! csrw {
    ($addr:literal, $val:ident) => {
        unsafe { core::arch::asm!(concat!("csrw ", $addr, "{}"), in(reg) $val) }
    };
}

#[macro_export]
macro_rules! csrr {
    ($addr:literal) => {{
        let val;
        unsafe { core::arch::asm!(concat!("csrr {}, ", $addr), out(reg) val) }
        val
    }};
}


/// Machine-privilege CSRs.
pub mod machine {

    pub const FS_VS_OFF: u32        = 0;
    pub const FS_VS_INITIAL: u32    = 1;
    pub const FS_VS_CLEAN: u32      = 2;
    pub const FS_VS_DIRTY: u32      = 3;

    #[cfg(target_pointer_width = "64")]
    embedded_util::reg! {

        /// Definition of `mstatus` on RV64.
        pub struct Status: u64 {
            /// Supervisor Interrupt Enable. This can be used to 
            /// globally enable or disable interrupts for supervisor
            /// privilege level.
            /// 
            /// This bit is cleared when a trap is taken, and is set
            /// to `spie`. It is then re-set to `spie` when `sret` is
            /// called.
            [01..02] sie,
            /// Machine Interrupt Enable. This can be used to globally
            /// enable or disable interrupts for machine privilege
            /// level.
            /// 
            /// This bit is cleared when a trap is taken, and is set
            /// to `mpie`. It is then re-set to `mpie` when `mret` is
            /// called.
            [03..04] mie,
            /// Supervisor Previous Interrupt Enable. When a trap
            /// is taken, it contains the previous value of `sie`.
            [05..06] spie,
            /// User privilege Big Endian. 
            [06..07] ube,
            /// Supervisor Previous Interrupt Enable. When a trap
            /// is taken, it contains the previous value of `mie`.
            [07..08] mpie,
            /// Supervisor Previous Privilege level. When a trap is 
            /// taken, it contains the previous privilege level.
            /// 
            /// The previous privilege level can only by two values:
            /// User (0) and Supervisor (1).
            [08..09] spp,
            /// Vector registers State. These 2 bits contains the
            /// dirty state of v0-v31 and CSRs vcsr, vxrm, vxsat, 
            /// vstart, vl, vtype, vlenb.
            [09..11] vs,
            /// Machine Previous Privilege level. When a trap is 
            /// taken, it contains the previous privilege level.
            /// 
            /// The previous privilege level takes 2 bits to represent
            /// all privilege levels: User (0), Supervisor (1) or
            /// Machine (3).
            [11..13] mpp,
            /// Float registers State. These 2 bits contains the dirty
            /// state of f0-f31 and CSRs fcsr, frm, fflags.
            [13..15] fs,
            /// Extension registers State, it represents the summary
            /// of all extensions' status (`vs` and `fs`).
            [15..17] xs,
            /// Modify PRiVilege. It modifies the effective privilege 
            /// mode, i.e., the privilege level at which loads and 
            /// stores execute
            [17..18] mprv,
            /// Supervisor User Memory. It modifies the privilege with 
            /// which S-mode loads and stores access virtual memory.
            /// 
            /// When SUM=0, S-mode memory accesses to pages that are 
            /// accessible by U-mode will fault.
            /// 
            /// When SUM=1, these accesses are permitted.
            /// 
            /// SUM has no effect when page-based virtual memory is 
            /// not in effect. Note that, while SUM is ordinarily 
            /// ignored when not executing in S-mode, it is in effect 
            /// when MPRV=1 and MPP=S. SUM is read-only 0 if S-mode 
            /// is not supported or if satp.MODE is read-only 0.
            [18..19] sum,
            /// Make eXecutable Readable. It modifies the privilege 
            /// with which loads access virtual memory. 
            /// 
            /// When MXR=0, only loads from pages marked readable will
            /// succeed.
            /// 
            /// When MXR=1, loads from pages marked either readable 
            /// or executable (R=1 or X=1) will succeed. MXR has no 
            /// effect when page-based virtual memory is not in 
            /// effect. MXR is read-only 0 if S-mode is not supported.
            [19..20] mxr,
            /// Trap Virtual Memory. (Virtualization)
            [20..21] tvm,
            /// Timeout Wait. (Virtualization)
            [21..22] tw,
            /// Trap SRET. (Virtualization)
            [22..23] tsr,
            /// User XLEN. It control the value of XLEN for User 
            /// privilege level. This doesn't exists on RV32, where
            /// it's forced to 32.
            [32..34] uxl,
            /// Supervisor XLEN. It control the value of XLEN for
            /// Supervisor privilege level. This doesn't exists on 
            /// RV32, where it's forced to 32.
            [34..36] sxl,
            /// Supervisor privilege Big Endian.
            [36..37] sbe,
            /// Machine privilege Big Endian.
            [37..38] mbe,
            /// The SD bit is a read-only bit that summarizes whether 
            /// either the FS, VS, or XS fields signal the presence 
            /// of some dirty state that will require saving extended 
            /// user context to memory. If FS, XS, and VS are all 
            /// read-only zero, then SD is also always zero.
            [63..64] sd,
        }

    }

    #[cfg(target_pointer_width = "32")]
    embedded_util::reg! {

        /// Definition of `mstatus` on RV32.
        pub struct Status: u32 {
            /// Supervisor Interrupt Enable. This can be used to 
            /// globally enable or disable interrupts for supervisor
            /// privilege level.
            /// 
            /// This bit is cleared when a trap is taken, and is set
            /// to `spie`. It is then re-set to `spie` when `sret` is
            /// called.
            [01..02] sie,
            /// Machine Interrupt Enable. This can be used to globally
            /// enable or disable interrupts for machine privilege
            /// level.
            /// 
            /// This bit is cleared when a trap is taken, and is set
            /// to `mpie`. It is then re-set to `mpie` when `mret` is
            /// called.
            [03..04] mie,
            /// Supervisor Previous Interrupt Enable. When a trap
            /// is taken, it contains the previous value of `sie`.
            [05..06] spie,
            /// User privilege Big Endian. 
            [06..07] ube,
            /// Supervisor Previous Interrupt Enable. When a trap
            /// is taken, it contains the previous value of `mie`.
            [07..08] mpie,
            /// Supervisor Previous Privilege level. When a trap is 
            /// taken, it contains the previous privilege level.
            /// 
            /// The previous privilege level can only by two values:
            /// User (0) and Supervisor (1).
            [08..09] spp,
            /// Vector registers State. These 2 bits contains the
            /// dirty state of v0-v31 and CSRs vcsr, vxrm, vxsat, 
            /// vstart, vl, vtype, vlenb.
            [09..11] vs,
            /// Machine Previous Privilege level. When a trap is 
            /// taken, it contains the previous privilege level.
            /// 
            /// The previous privilege level takes 2 bits to represent
            /// all privilege levels: User (0), Supervisor (1) or
            /// Machine (3).
            [11..13] mpp,
            /// Float registers State. These 2 bits contains the dirty
            /// state of f0-f31 and CSRs fcsr, frm, fflags.
            [13..15] fs,
            /// Extension registers State, it represents the summary
            /// of all extensions' status (`vs` and `fs`).
            [15..17] xs,
            /// Modify PRiVilege. It modifies the effective privilege 
            /// mode, i.e., the privilege level at which loads and 
            /// stores execute
            [17..18] mprv,
            /// Supervisor User Memory. It modifies the privilege with 
            /// which S-mode loads and stores access virtual memory.
            /// 
            /// When SUM=0, S-mode memory accesses to pages that are 
            /// accessible by U-mode will fault.
            /// 
            /// When SUM=1, these accesses are permitted.
            /// 
            /// SUM has no effect when page-based virtual memory is 
            /// not in effect. Note that, while SUM is ordinarily 
            /// ignored when not executing in S-mode, it is in effect 
            /// when MPRV=1 and MPP=S. SUM is read-only 0 if S-mode 
            /// is not supported or if satp.MODE is read-only 0.
            [18..19] sum,
            /// Make eXecutable Readable. It modifies the privilege 
            /// with which loads access virtual memory. 
            /// 
            /// When MXR=0, only loads from pages marked readable will
            /// succeed.
            /// 
            /// When MXR=1, loads from pages marked either readable 
            /// or executable (R=1 or X=1) will succeed. MXR has no 
            /// effect when page-based virtual memory is not in 
            /// effect. MXR is read-only 0 if S-mode is not supported.
            [19..20] mxr,
            /// Trap Virtual Memory. (Virtualization)
            [20..21] tvm,
            /// Timeout Wait. (Virtualization)
            [21..22] tw,
            /// Trap SRET. (Virtualization)
            [22..23] tsr,
            /// The SD bit is a read-only bit that summarizes whether 
            /// either the FS, VS, or XS fields signal the presence 
            /// of some dirty state that will require saving extended 
            /// user context to memory. If FS, XS, and VS are all 
            /// read-only zero, then SD is also always zero.
            [31..32] sd,
        }

        /// Definition of `mstatush` on RV32.
        pub struct StatusExt: u32 {
            /// Supervisor privilege Big Endian.
            [04..05] sbe,
            /// Machine privilege Big Endian.
            [05..06] mbe,
        }

    }

}
