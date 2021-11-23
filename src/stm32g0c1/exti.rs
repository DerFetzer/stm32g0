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
    _reserved5: [u8; 0x14],
    ///0x28 - EXTI rising trigger selection register 2
    pub rtsr2: crate::Reg<rtsr2::RTSR2_SPEC>,
    ///0x2c - EXTI falling trigger selection register 2
    pub ftsr2: crate::Reg<ftsr2::FTSR2_SPEC>,
    ///0x30 - EXTI software interrupt event register 2
    pub swier2: crate::Reg<swier2::SWIER2_SPEC>,
    ///0x34 - EXTI rising edge pending register 2
    pub rpr2: crate::Reg<rpr2::RPR2_SPEC>,
    ///0x38 - EXTI falling edge pending register 2
    pub fpr2: crate::Reg<fpr2::FPR2_SPEC>,
    _reserved10: [u8; 0x24],
    ///0x60 - EXTI external interrupt selection register
    pub exticr1: crate::Reg<exticr1::EXTICR1_SPEC>,
    ///0x64 - EXTI external interrupt selection register
    pub exticr2: crate::Reg<exticr2::EXTICR2_SPEC>,
    ///0x68 - EXTI external interrupt selection register
    pub exticr3: crate::Reg<exticr3::EXTICR3_SPEC>,
    ///0x6c - EXTI external interrupt selection register
    pub exticr4: crate::Reg<exticr4::EXTICR4_SPEC>,
    _reserved14: [u8; 0x10],
    ///0x80 - EXTI CPU wakeup with interrupt mask register
    pub imr1: crate::Reg<imr1::IMR1_SPEC>,
    ///0x84 - EXTI CPU wakeup with event mask register
    pub emr1: crate::Reg<emr1::EMR1_SPEC>,
    _reserved16: [u8; 0x08],
    ///0x90 - EXTI CPU wakeup with interrupt mask register
    pub imr2: crate::Reg<imr2::IMR2_SPEC>,
    ///0x94 - EXTI CPU wakeup with event mask register
    pub emr2: crate::Reg<emr2::EMR2_SPEC>,
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
///RTSR2 register accessor: an alias for `Reg<RTSR2_SPEC>`
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
///EXTI rising trigger selection register 2
pub mod rtsr2;
///FTSR2 register accessor: an alias for `Reg<FTSR2_SPEC>`
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
///EXTI falling trigger selection register 2
pub mod ftsr2;
///SWIER2 register accessor: an alias for `Reg<SWIER2_SPEC>`
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
///EXTI software interrupt event register 2
pub mod swier2;
///RPR2 register accessor: an alias for `Reg<RPR2_SPEC>`
pub type RPR2 = crate::Reg<rpr2::RPR2_SPEC>;
///EXTI rising edge pending register 2
pub mod rpr2;
///FPR2 register accessor: an alias for `Reg<FPR2_SPEC>`
pub type FPR2 = crate::Reg<fpr2::FPR2_SPEC>;
///EXTI falling edge pending register 2
pub mod fpr2;
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
