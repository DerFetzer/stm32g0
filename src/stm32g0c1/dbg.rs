///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MCU Device ID Code Register
    pub idcode: crate::Reg<idcode::IDCODE_SPEC>,
    ///0x04 - DBG configuration register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x08 - DBG APB freeze register 1
    pub apb_fz1: crate::Reg<apb_fz1::APB_FZ1_SPEC>,
    ///0x0c - DBG APB freeze register 2
    pub apb_fz2: crate::Reg<apb_fz2::APB_FZ2_SPEC>,
}
///IDCODE register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///MCU Device ID Code Register
pub mod idcode;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DBG configuration register
pub mod cr;
///APB_FZ1 register accessor: an alias for `Reg<APB_FZ1_SPEC>`
pub type APB_FZ1 = crate::Reg<apb_fz1::APB_FZ1_SPEC>;
///DBG APB freeze register 1
pub mod apb_fz1;
///APB_FZ2 register accessor: an alias for `Reg<APB_FZ2_SPEC>`
pub type APB_FZ2 = crate::Reg<apb_fz2::APB_FZ2_SPEC>;
///DBG APB freeze register 2
pub mod apb_fz2;
