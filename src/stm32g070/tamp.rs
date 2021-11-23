///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x0c - TAMP filter control register
    pub fltcr: crate::Reg<fltcr::FLTCR_SPEC>,
    _reserved3: [u8; 0x1c],
    ///0x2c - TAMP interrupt enable register
    pub ier: crate::Reg<ier::IER_SPEC>,
    ///0x30 - TAMP status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x34 - TAMP masked interrupt status register
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x3c - TAMP status clear register
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved7: [u8; 0xc0],
    ///0x100 - TAMP backup register
    pub bkp0r: crate::Reg<bkp0r::BKP0R_SPEC>,
    ///0x104 - TAMP backup register
    pub bkp1r: crate::Reg<bkp1r::BKP1R_SPEC>,
    ///0x108 - TAMP backup register
    pub bkp2r: crate::Reg<bkp2r::BKP2R_SPEC>,
    ///0x10c - TAMP backup register
    pub bkp3r: crate::Reg<bkp3r::BKP3R_SPEC>,
    ///0x110 - TAMP backup register
    pub bkp4r: crate::Reg<bkp4r::BKP4R_SPEC>,
    _reserved12: [u8; 0x02d8],
    ///0x3ec - TAMP hardware configuration register 2
    pub hwcfgr2: crate::Reg<hwcfgr2::HWCFGR2_SPEC>,
    ///0x3f0 - TAMP hardware configuration register 1
    pub hwcfgr1: crate::Reg<hwcfgr1::HWCFGR1_SPEC>,
    ///0x3f4 - EXTI IP Version register
    pub verr: crate::Reg<verr::VERR_SPEC>,
    ///0x3f8 - EXTI Identification register
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    ///0x3fc - EXTI Size ID register
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///FLTCR register accessor: an alias for `Reg<FLTCR_SPEC>`
pub type FLTCR = crate::Reg<fltcr::FLTCR_SPEC>;
///TAMP filter control register
pub mod fltcr;
///IER register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///TAMP interrupt enable register
pub mod ier;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///TAMP status register
pub mod sr;
///MISR register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///TAMP masked interrupt status register
pub mod misr;
///SCR register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///TAMP status clear register
pub mod scr;
///BKP0R register accessor: an alias for `Reg<BKP0R_SPEC>`
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
///TAMP backup register
pub mod bkp0r;
///BKP1R register accessor: an alias for `Reg<BKP1R_SPEC>`
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
///TAMP backup register
pub mod bkp1r;
///BKP2R register accessor: an alias for `Reg<BKP2R_SPEC>`
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
///TAMP backup register
pub mod bkp2r;
///BKP3R register accessor: an alias for `Reg<BKP3R_SPEC>`
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
///TAMP backup register
pub mod bkp3r;
///BKP4R register accessor: an alias for `Reg<BKP4R_SPEC>`
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
///TAMP backup register
pub mod bkp4r;
///HWCFGR2 register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///TAMP hardware configuration register 2
pub mod hwcfgr2;
///HWCFGR1 register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///TAMP hardware configuration register 1
pub mod hwcfgr1;
///VERR register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///EXTI IP Version register
pub mod verr;
///IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///EXTI Identification register
pub mod ipidr;
///SIDR register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///EXTI Size ID register
pub mod sidr;
