///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SYSCFG configuration register 1
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    _reserved1: [u8; 0x14],
    ///0x18 - SYSCFG configuration register 1
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    _reserved2: [u8; 0x14],
    ///0x30 - VREFBUF control and status register
    pub vrefbuf_csr: crate::Reg<vrefbuf_csr::VREFBUF_CSR_SPEC>,
    ///0x34 - VREFBUF calibration control register
    pub vrefbuf_ccr: crate::Reg<vrefbuf_ccr::VREFBUF_CCR_SPEC>,
    _reserved4: [u8; 0x48],
    ///0x80 - interrupt line 0 status register
    pub itline0: crate::Reg<itline0::ITLINE0_SPEC>,
    ///0x84 - interrupt line 1 status register
    pub itline1: crate::Reg<itline1::ITLINE1_SPEC>,
    ///0x88 - interrupt line 2 status register
    pub itline2: crate::Reg<itline2::ITLINE2_SPEC>,
    ///0x8c - interrupt line 3 status register
    pub itline3: crate::Reg<itline3::ITLINE3_SPEC>,
    ///0x90 - interrupt line 4 status register
    pub itline4: crate::Reg<itline4::ITLINE4_SPEC>,
    ///0x94 - interrupt line 5 status register
    pub itline5: crate::Reg<itline5::ITLINE5_SPEC>,
    ///0x98 - interrupt line 6 status register
    pub itline6: crate::Reg<itline6::ITLINE6_SPEC>,
    ///0x9c - interrupt line 7 status register
    pub itline7: crate::Reg<itline7::ITLINE7_SPEC>,
    ///0xa0 - interrupt line 8 status register
    pub itline8: crate::Reg<itline8::ITLINE8_SPEC>,
    ///0xa4 - interrupt line 9 status register
    pub itline9: crate::Reg<itline9::ITLINE9_SPEC>,
    ///0xa8 - interrupt line 10 status register
    pub itline10: crate::Reg<itline10::ITLINE10_SPEC>,
    ///0xac - interrupt line 11 status register
    pub itline11: crate::Reg<itline11::ITLINE11_SPEC>,
    ///0xb0 - interrupt line 12 status register
    pub itline12: crate::Reg<itline12::ITLINE12_SPEC>,
    ///0xb4 - interrupt line 13 status register
    pub itline13: crate::Reg<itline13::ITLINE13_SPEC>,
    ///0xb8 - interrupt line 14 status register
    pub itline14: crate::Reg<itline14::ITLINE14_SPEC>,
    ///0xbc - interrupt line 15 status register
    pub itline15: crate::Reg<itline15::ITLINE15_SPEC>,
    ///0xc0 - interrupt line 16 status register
    pub itline16: crate::Reg<itline16::ITLINE16_SPEC>,
    ///0xc4 - interrupt line 17 status register
    pub itline17: crate::Reg<itline17::ITLINE17_SPEC>,
    ///0xc8 - interrupt line 18 status register
    pub itline18: crate::Reg<itline18::ITLINE18_SPEC>,
    ///0xcc - interrupt line 19 status register
    pub itline19: crate::Reg<itline19::ITLINE19_SPEC>,
    ///0xd0 - interrupt line 20 status register
    pub itline20: crate::Reg<itline20::ITLINE20_SPEC>,
    ///0xd4 - interrupt line 21 status register
    pub itline21: crate::Reg<itline21::ITLINE21_SPEC>,
    ///0xd8 - interrupt line 22 status register
    pub itline22: crate::Reg<itline22::ITLINE22_SPEC>,
    ///0xdc - interrupt line 23 status register
    pub itline23: crate::Reg<itline23::ITLINE23_SPEC>,
    ///0xe0 - interrupt line 24 status register
    pub itline24: crate::Reg<itline24::ITLINE24_SPEC>,
    ///0xe4 - interrupt line 25 status register
    pub itline25: crate::Reg<itline25::ITLINE25_SPEC>,
    ///0xe8 - interrupt line 26 status register
    pub itline26: crate::Reg<itline26::ITLINE26_SPEC>,
    ///0xec - interrupt line 27 status register
    pub itline27: crate::Reg<itline27::ITLINE27_SPEC>,
    ///0xf0 - interrupt line 28 status register
    pub itline28: crate::Reg<itline28::ITLINE28_SPEC>,
    ///0xf4 - interrupt line 29 status register
    pub itline29: crate::Reg<itline29::ITLINE29_SPEC>,
    ///0xf8 - interrupt line 30 status register
    pub itline30: crate::Reg<itline30::ITLINE30_SPEC>,
    ///0xfc - interrupt line 31 status register
    pub itline31: crate::Reg<itline31::ITLINE31_SPEC>,
}
///VREFBUF_CSR register accessor: an alias for `Reg<VREFBUF_CSR_SPEC>`
pub type VREFBUF_CSR = crate::Reg<vrefbuf_csr::VREFBUF_CSR_SPEC>;
///VREFBUF control and status register
pub mod vrefbuf_csr;
///VREFBUF_CCR register accessor: an alias for `Reg<VREFBUF_CCR_SPEC>`
pub type VREFBUF_CCR = crate::Reg<vrefbuf_ccr::VREFBUF_CCR_SPEC>;
///VREFBUF calibration control register
pub mod vrefbuf_ccr;
///CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///SYSCFG configuration register 1
pub mod cfgr1;
///CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///SYSCFG configuration register 1
pub mod cfgr2;
///ITLINE0 register accessor: an alias for `Reg<ITLINE0_SPEC>`
pub type ITLINE0 = crate::Reg<itline0::ITLINE0_SPEC>;
///interrupt line 0 status register
pub mod itline0;
///ITLINE1 register accessor: an alias for `Reg<ITLINE1_SPEC>`
pub type ITLINE1 = crate::Reg<itline1::ITLINE1_SPEC>;
///interrupt line 1 status register
pub mod itline1;
///ITLINE2 register accessor: an alias for `Reg<ITLINE2_SPEC>`
pub type ITLINE2 = crate::Reg<itline2::ITLINE2_SPEC>;
///interrupt line 2 status register
pub mod itline2;
///ITLINE3 register accessor: an alias for `Reg<ITLINE3_SPEC>`
pub type ITLINE3 = crate::Reg<itline3::ITLINE3_SPEC>;
///interrupt line 3 status register
pub mod itline3;
///ITLINE4 register accessor: an alias for `Reg<ITLINE4_SPEC>`
pub type ITLINE4 = crate::Reg<itline4::ITLINE4_SPEC>;
///interrupt line 4 status register
pub mod itline4;
///ITLINE5 register accessor: an alias for `Reg<ITLINE5_SPEC>`
pub type ITLINE5 = crate::Reg<itline5::ITLINE5_SPEC>;
///interrupt line 5 status register
pub mod itline5;
///ITLINE6 register accessor: an alias for `Reg<ITLINE6_SPEC>`
pub type ITLINE6 = crate::Reg<itline6::ITLINE6_SPEC>;
///interrupt line 6 status register
pub mod itline6;
///ITLINE7 register accessor: an alias for `Reg<ITLINE7_SPEC>`
pub type ITLINE7 = crate::Reg<itline7::ITLINE7_SPEC>;
///interrupt line 7 status register
pub mod itline7;
///ITLINE8 register accessor: an alias for `Reg<ITLINE8_SPEC>`
pub type ITLINE8 = crate::Reg<itline8::ITLINE8_SPEC>;
///interrupt line 8 status register
pub mod itline8;
///ITLINE9 register accessor: an alias for `Reg<ITLINE9_SPEC>`
pub type ITLINE9 = crate::Reg<itline9::ITLINE9_SPEC>;
///interrupt line 9 status register
pub mod itline9;
///ITLINE10 register accessor: an alias for `Reg<ITLINE10_SPEC>`
pub type ITLINE10 = crate::Reg<itline10::ITLINE10_SPEC>;
///interrupt line 10 status register
pub mod itline10;
///ITLINE11 register accessor: an alias for `Reg<ITLINE11_SPEC>`
pub type ITLINE11 = crate::Reg<itline11::ITLINE11_SPEC>;
///interrupt line 11 status register
pub mod itline11;
///ITLINE12 register accessor: an alias for `Reg<ITLINE12_SPEC>`
pub type ITLINE12 = crate::Reg<itline12::ITLINE12_SPEC>;
///interrupt line 12 status register
pub mod itline12;
///ITLINE13 register accessor: an alias for `Reg<ITLINE13_SPEC>`
pub type ITLINE13 = crate::Reg<itline13::ITLINE13_SPEC>;
///interrupt line 13 status register
pub mod itline13;
///ITLINE14 register accessor: an alias for `Reg<ITLINE14_SPEC>`
pub type ITLINE14 = crate::Reg<itline14::ITLINE14_SPEC>;
///interrupt line 14 status register
pub mod itline14;
///ITLINE15 register accessor: an alias for `Reg<ITLINE15_SPEC>`
pub type ITLINE15 = crate::Reg<itline15::ITLINE15_SPEC>;
///interrupt line 15 status register
pub mod itline15;
///ITLINE16 register accessor: an alias for `Reg<ITLINE16_SPEC>`
pub type ITLINE16 = crate::Reg<itline16::ITLINE16_SPEC>;
///interrupt line 16 status register
pub mod itline16;
///ITLINE17 register accessor: an alias for `Reg<ITLINE17_SPEC>`
pub type ITLINE17 = crate::Reg<itline17::ITLINE17_SPEC>;
///interrupt line 17 status register
pub mod itline17;
///ITLINE18 register accessor: an alias for `Reg<ITLINE18_SPEC>`
pub type ITLINE18 = crate::Reg<itline18::ITLINE18_SPEC>;
///interrupt line 18 status register
pub mod itline18;
///ITLINE19 register accessor: an alias for `Reg<ITLINE19_SPEC>`
pub type ITLINE19 = crate::Reg<itline19::ITLINE19_SPEC>;
///interrupt line 19 status register
pub mod itline19;
///ITLINE20 register accessor: an alias for `Reg<ITLINE20_SPEC>`
pub type ITLINE20 = crate::Reg<itline20::ITLINE20_SPEC>;
///interrupt line 20 status register
pub mod itline20;
///ITLINE21 register accessor: an alias for `Reg<ITLINE21_SPEC>`
pub type ITLINE21 = crate::Reg<itline21::ITLINE21_SPEC>;
///interrupt line 21 status register
pub mod itline21;
///ITLINE22 register accessor: an alias for `Reg<ITLINE22_SPEC>`
pub type ITLINE22 = crate::Reg<itline22::ITLINE22_SPEC>;
///interrupt line 22 status register
pub mod itline22;
///ITLINE23 register accessor: an alias for `Reg<ITLINE23_SPEC>`
pub type ITLINE23 = crate::Reg<itline23::ITLINE23_SPEC>;
///interrupt line 23 status register
pub mod itline23;
///ITLINE24 register accessor: an alias for `Reg<ITLINE24_SPEC>`
pub type ITLINE24 = crate::Reg<itline24::ITLINE24_SPEC>;
///interrupt line 24 status register
pub mod itline24;
///ITLINE25 register accessor: an alias for `Reg<ITLINE25_SPEC>`
pub type ITLINE25 = crate::Reg<itline25::ITLINE25_SPEC>;
///interrupt line 25 status register
pub mod itline25;
///ITLINE26 register accessor: an alias for `Reg<ITLINE26_SPEC>`
pub type ITLINE26 = crate::Reg<itline26::ITLINE26_SPEC>;
///interrupt line 26 status register
pub mod itline26;
///ITLINE27 register accessor: an alias for `Reg<ITLINE27_SPEC>`
pub type ITLINE27 = crate::Reg<itline27::ITLINE27_SPEC>;
///interrupt line 27 status register
pub mod itline27;
///ITLINE28 register accessor: an alias for `Reg<ITLINE28_SPEC>`
pub type ITLINE28 = crate::Reg<itline28::ITLINE28_SPEC>;
///interrupt line 28 status register
pub mod itline28;
///ITLINE29 register accessor: an alias for `Reg<ITLINE29_SPEC>`
pub type ITLINE29 = crate::Reg<itline29::ITLINE29_SPEC>;
///interrupt line 29 status register
pub mod itline29;
///ITLINE30 register accessor: an alias for `Reg<ITLINE30_SPEC>`
pub type ITLINE30 = crate::Reg<itline30::ITLINE30_SPEC>;
///interrupt line 30 status register
pub mod itline30;
///ITLINE31 register accessor: an alias for `Reg<ITLINE31_SPEC>`
pub type ITLINE31 = crate::Reg<itline31::ITLINE31_SPEC>;
///interrupt line 31 status register
pub mod itline31;
