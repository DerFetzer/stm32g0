///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    pub rtsr1: crate::Reg<rtsr1::RTSR1_SPEC>,
    ///0x04 - EXTI falling trigger selection register
    pub ftsr1: crate::Reg<ftsr1::FTSR1_SPEC>,
    ///0x08 - EXTI software interrupt event register
    pub swier1: crate::Reg<swier1::SWIER1_SPEC>,
    ///0x0c - EXTI rising edge pending register
    pub rpr1: crate::Reg<rpr1::RPR1_SPEC>,
    ///0x10 - EXTI falling edge pending register
    pub fpr1: crate::Reg<fpr1::FPR1_SPEC>,
    _reserved5: [u8; 0x4c],
    ///0x60 - EXTI external interrupt selection register
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    ///0x64 - EXTI external interrupt selection register
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    ///0x68 - EXTI external interrupt selection register
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    ///0x6c - EXTI external interrupt selection register
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    _reserved9: [u8; 0x10],
    ///0x80 - EXTI CPU wakeup with interrupt mask register
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    ///0x84 - EXTI CPU wakeup with event mask register
    pub emr1: crate::Reg<emr1::EMR1_SPEC>,
    _reserved11: [u8; 0x08],
    ///0x90 - EXTI CPU wakeup with interrupt mask register
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    ///0x94 - EXTI CPU wakeup with event mask register
    pub emr2: crate::Reg<emr2::EMR2_SPEC>,
    _reserved13: [u8; 0x0340],
    ///0x3d8 - Hardware configuration registers
    pub hwcfgr7: crate::Reg<hwcfgr7::HWCFGR7_SPEC>,
    ///0x3dc - Hardware configuration registers
    pub hwcfgr6: crate::Reg<hwcfgr6::HWCFGR6_SPEC>,
    ///0x3e0 - Hardware configuration registers
    pub hwcfgr5: crate::Reg<hwcfgr5::HWCFGR5_SPEC>,
    ///0x3e4 - Hardware configuration registers
    pub hwcfgr4: crate::Reg<hwcfgr4::HWCFGR4_SPEC>,
    ///0x3e8 - Hardware configuration registers
    pub hwcfgr3: crate::Reg<hwcfgr3::HWCFGR3_SPEC>,
    ///0x3ec - Hardware configuration registers
    pub hwcfgr2: crate::Reg<hwcfgr2::HWCFGR2_SPEC>,
    ///0x3f0 - Hardware configuration registers
    pub hwcfgr1: crate::Reg<hwcfgr1::HWCFGR1_SPEC>,
}
///RTSR1 register accessor: an alias for `Reg<RTSR1_SPEC>`
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
///EXTI rising trigger selection register
pub mod rtsr1;
///FTSR1 register accessor: an alias for `Reg<FTSR1_SPEC>`
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
///EXTI falling trigger selection register
pub mod ftsr1;
///SWIER1 register accessor: an alias for `Reg<SWIER1_SPEC>`
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
///EXTI software interrupt event register
pub mod swier1;
///RPR1 register accessor: an alias for `Reg<RPR1_SPEC>`
pub type RPR1 = crate::Reg<rpr1::RPR1_SPEC>;
///EXTI rising edge pending register
pub mod rpr1;
///FPR1 register accessor: an alias for `Reg<FPR1_SPEC>`
pub type FPR1 = crate::Reg<fpr1::FPR1_SPEC>;
///EXTI falling edge pending register
pub mod fpr1;
///EXTICR1 register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///EXTI external interrupt selection register
pub mod exticr1;
///EXTICR2 register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///EXTI external interrupt selection register
pub mod exticr2;
///EXTICR3 register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///EXTI external interrupt selection register
pub mod exticr3;
///EXTICR4 register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///EXTI external interrupt selection register
pub mod exticr4;
///IMR1 register accessor: an alias for `Reg<IMR1_SPEC>`
pub type IMR1 = crate::Reg<imr1::IMR1_SPEC>;
///EXTI CPU wakeup with interrupt mask register
pub mod imr1;
///EMR1 register accessor: an alias for `Reg<EMR1_SPEC>`
pub type EMR1 = crate::Reg<emr1::EMR1_SPEC>;
///EXTI CPU wakeup with event mask register
pub mod emr1;
///IMR2 register accessor: an alias for `Reg<IMR2_SPEC>`
pub type IMR2 = crate::Reg<imr2::IMR2_SPEC>;
///EXTI CPU wakeup with interrupt mask register
pub mod imr2;
///EMR2 register accessor: an alias for `Reg<EMR2_SPEC>`
pub type EMR2 = crate::Reg<emr2::EMR2_SPEC>;
///EXTI CPU wakeup with event mask register
pub mod emr2;
///HWCFGR7 register accessor: an alias for `Reg<HWCFGR7_SPEC>`
pub type HWCFGR7 = crate::Reg<hwcfgr7::HWCFGR7_SPEC>;
///Hardware configuration registers
pub mod hwcfgr7;
///HWCFGR6 register accessor: an alias for `Reg<HWCFGR6_SPEC>`
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6_SPEC>;
///Hardware configuration registers
pub mod hwcfgr6;
///HWCFGR5 register accessor: an alias for `Reg<HWCFGR5_SPEC>`
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5_SPEC>;
///Hardware configuration registers
pub mod hwcfgr5;
///HWCFGR4 register accessor: an alias for `Reg<HWCFGR4_SPEC>`
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4_SPEC>;
///Hardware configuration registers
pub mod hwcfgr4;
///HWCFGR3 register accessor: an alias for `Reg<HWCFGR3_SPEC>`
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3_SPEC>;
///Hardware configuration registers
pub mod hwcfgr3;
///HWCFGR2 register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///Hardware configuration registers
pub mod hwcfgr2;
///HWCFGR1 register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///Hardware configuration registers
pub mod hwcfgr1;
