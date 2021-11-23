///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - USB endpoint/channel 0 register
    pub chep0r: crate::Reg<chep0r::CHEP0R_SPEC>,
    ///0x04 - USB endpoint/channel 1 register
    pub chep1r: crate::Reg<chep1r::CHEP1R_SPEC>,
    ///0x08 - USB endpoint/channel 2 register
    pub chep2r: crate::Reg<chep2r::CHEP2R_SPEC>,
    ///0x0c - USB endpoint/channel 3 register
    pub chep3r: crate::Reg<chep3r::CHEP3R_SPEC>,
    ///0x10 - USB endpoint/channel 4 register
    pub chep4r: crate::Reg<chep4r::CHEP4R_SPEC>,
    ///0x14 - USB endpoint/channel 5 register
    pub chep5r: crate::Reg<chep5r::CHEP5R_SPEC>,
    ///0x18 - USB endpoint/channel 6 register
    pub chep6r: crate::Reg<chep6r::CHEP6R_SPEC>,
    ///0x1c - USB endpoint/channel 7 register
    pub chep7r: crate::Reg<chep7r::CHEP7R_SPEC>,
    _reserved8: [u8; 0x20],
    ///0x40 - USB control register
    pub cntr: crate::Reg<cntr::CNTR_SPEC>,
    ///0x44 - USB interrupt status register
    pub istr: crate::Reg<istr::ISTR_SPEC>,
    ///0x48 - USB frame number register
    pub fnr: crate::Reg<fnr::FNR_SPEC>,
    ///0x4c - USB device address
    pub daddr: crate::Reg<daddr::DADDR_SPEC>,
    _reserved12: [u8; 0x04],
    ///0x54 - LPM control and status register
    pub lpmcsr: crate::Reg<lpmcsr::LPMCSR_SPEC>,
    ///0x58 - Battery charging detector
    pub bcdr: crate::Reg<bcdr::BCDR_SPEC>,
}
///CHEP0R register accessor: an alias for `Reg<CHEP0R_SPEC>`
pub type CHEP0R = crate::Reg<chep0r::CHEP0R_SPEC>;
///USB endpoint/channel 0 register
pub mod chep0r;
///CHEP1R register accessor: an alias for `Reg<CHEP1R_SPEC>`
pub type CHEP1R = crate::Reg<chep1r::CHEP1R_SPEC>;
///USB endpoint/channel 1 register
pub mod chep1r;
///CHEP2R register accessor: an alias for `Reg<CHEP2R_SPEC>`
pub type CHEP2R = crate::Reg<chep2r::CHEP2R_SPEC>;
///USB endpoint/channel 2 register
pub mod chep2r;
///CHEP3R register accessor: an alias for `Reg<CHEP3R_SPEC>`
pub type CHEP3R = crate::Reg<chep3r::CHEP3R_SPEC>;
///USB endpoint/channel 3 register
pub mod chep3r;
///CHEP4R register accessor: an alias for `Reg<CHEP4R_SPEC>`
pub type CHEP4R = crate::Reg<chep4r::CHEP4R_SPEC>;
///USB endpoint/channel 4 register
pub mod chep4r;
///CHEP5R register accessor: an alias for `Reg<CHEP5R_SPEC>`
pub type CHEP5R = crate::Reg<chep5r::CHEP5R_SPEC>;
///USB endpoint/channel 5 register
pub mod chep5r;
///CHEP6R register accessor: an alias for `Reg<CHEP6R_SPEC>`
pub type CHEP6R = crate::Reg<chep6r::CHEP6R_SPEC>;
///USB endpoint/channel 6 register
pub mod chep6r;
///CHEP7R register accessor: an alias for `Reg<CHEP7R_SPEC>`
pub type CHEP7R = crate::Reg<chep7r::CHEP7R_SPEC>;
///USB endpoint/channel 7 register
pub mod chep7r;
///CNTR register accessor: an alias for `Reg<CNTR_SPEC>`
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
///USB control register
pub mod cntr;
///ISTR register accessor: an alias for `Reg<ISTR_SPEC>`
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
///USB interrupt status register
pub mod istr;
///FNR register accessor: an alias for `Reg<FNR_SPEC>`
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
///USB frame number register
pub mod fnr;
///DADDR register accessor: an alias for `Reg<DADDR_SPEC>`
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
///USB device address
pub mod daddr;
///LPMCSR register accessor: an alias for `Reg<LPMCSR_SPEC>`
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSR_SPEC>;
///LPM control and status register
pub mod lpmcsr;
///BCDR register accessor: an alias for `Reg<BCDR_SPEC>`
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
///Battery charging detector
pub mod bcdr;
