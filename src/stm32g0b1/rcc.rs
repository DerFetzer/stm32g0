///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - Internal clock sources calibration register
    pub icscr: crate::Reg<icscr::ICSCR_SPEC>,
    ///0x08 - Clock configuration register
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    ///0x0c - PLL configuration register
    pub pllcfgr: crate::Reg<pllcfgr::PLLCFGR_SPEC>,
    _reserved4: [u8; 0x04],
    ///0x14 - RCC clock recovery RC register
    pub crrcr: crate::Reg<crrcr::CRRCR_SPEC>,
    ///0x18 - Clock interrupt enable register
    pub cier: crate::Reg<cier::CIER_SPEC>,
    ///0x1c - Clock interrupt flag register
    pub cifr: crate::Reg<cifr::CIFR_SPEC>,
    ///0x20 - Clock interrupt clear register
    pub cicr: crate::Reg<cicr::CICR_SPEC>,
    ///0x24 - I/O port reset register
    pub ioprstr: crate::Reg<ioprstr::IOPRSTR_SPEC>,
    ///0x28 - AHB peripheral reset register
    pub ahbrstr: crate::Reg<ahbrstr::AHBRSTR_SPEC>,
    ///0x2c - APB peripheral reset register 1
    pub apbrstr1: crate::Reg<apbrstr1::APBRSTR1_SPEC>,
    ///0x30 - APB peripheral reset register 2
    pub apbrstr2: crate::Reg<apbrstr2::APBRSTR2_SPEC>,
    ///0x34 - GPIO clock enable register
    pub iopenr: crate::Reg<iopenr::IOPENR_SPEC>,
    ///0x38 - AHB peripheral clock enable register
    pub ahbenr: crate::Reg<ahbenr::AHBENR_SPEC>,
    ///0x3c - APB peripheral clock enable register 1
    pub apbenr1: crate::Reg<apbenr1::APBENR1_SPEC>,
    ///0x40 - APB peripheral clock enable register 2
    pub apbenr2: crate::Reg<apbenr2::APBENR2_SPEC>,
    ///0x44 - GPIO in Sleep mode clock enable register
    pub iopsmenr: crate::Reg<iopsmenr::IOPSMENR_SPEC>,
    ///0x48 - AHB peripheral clock enable in Sleep mode register
    pub ahbsmenr: crate::Reg<ahbsmenr::AHBSMENR_SPEC>,
    ///0x4c - APB peripheral clock enable in Sleep mode register 1
    pub apbsmenr1: crate::Reg<apbsmenr1::APBSMENR1_SPEC>,
    ///0x50 - APB peripheral clock enable in Sleep mode register 2
    pub apbsmenr2: crate::Reg<apbsmenr2::APBSMENR2_SPEC>,
    ///0x54 - Peripherals independent clock configuration register
    pub ccipr: crate::Reg<ccipr::CCIPR_SPEC>,
    ///0x58 - Peripherals independent clock configuration register 2
    pub ccipr2: crate::Reg<ccipr2::CCIPR2_SPEC>,
    ///0x5c - RTC domain control register
    pub bdcr: crate::Reg<bdcr::BDCR_SPEC>,
    ///0x60 - Control/status register
    pub csr: crate::Reg<csr::CSR_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Clock control register
pub mod cr;
///ICSCR register accessor: an alias for `Reg<ICSCR_SPEC>`
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
///Internal clock sources calibration register
pub mod icscr;
///CFGR register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Clock configuration register
pub mod cfgr;
///PLLCFGR register accessor: an alias for `Reg<PLLCFGR_SPEC>`
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
///PLL configuration register
pub mod pllcfgr;
///CRRCR register accessor: an alias for `Reg<CRRCR_SPEC>`
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
///RCC clock recovery RC register
pub mod crrcr;
///CIER register accessor: an alias for `Reg<CIER_SPEC>`
pub type CIER = crate::Reg<cier::CIER_SPEC>;
///Clock interrupt enable register
pub mod cier;
///CIFR register accessor: an alias for `Reg<CIFR_SPEC>`
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
///Clock interrupt flag register
pub mod cifr;
///CICR register accessor: an alias for `Reg<CICR_SPEC>`
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
///Clock interrupt clear register
pub mod cicr;
///IOPRSTR register accessor: an alias for `Reg<IOPRSTR_SPEC>`
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
///I/O port reset register
pub mod ioprstr;
///AHBRSTR register accessor: an alias for `Reg<AHBRSTR_SPEC>`
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
///AHB peripheral reset register
pub mod ahbrstr;
///APBRSTR1 register accessor: an alias for `Reg<APBRSTR1_SPEC>`
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1_SPEC>;
///APB peripheral reset register 1
pub mod apbrstr1;
///APBRSTR2 register accessor: an alias for `Reg<APBRSTR2_SPEC>`
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2_SPEC>;
///APB peripheral reset register 2
pub mod apbrstr2;
///IOPENR register accessor: an alias for `Reg<IOPENR_SPEC>`
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
///GPIO clock enable register
pub mod iopenr;
///AHBENR register accessor: an alias for `Reg<AHBENR_SPEC>`
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
///AHB peripheral clock enable register
pub mod ahbenr;
///APBENR1 register accessor: an alias for `Reg<APBENR1_SPEC>`
pub type APBENR1 = crate::Reg<apbenr1::APBENR1_SPEC>;
///APB peripheral clock enable register 1
pub mod apbenr1;
///APBENR2 register accessor: an alias for `Reg<APBENR2_SPEC>`
pub type APBENR2 = crate::Reg<apbenr2::APBENR2_SPEC>;
///APB peripheral clock enable register 2
pub mod apbenr2;
///IOPSMENR register accessor: an alias for `Reg<IOPSMENR_SPEC>`
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENR_SPEC>;
///GPIO in Sleep mode clock enable register
pub mod iopsmenr;
///AHBSMENR register accessor: an alias for `Reg<AHBSMENR_SPEC>`
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
///AHB peripheral clock enable in Sleep mode register
pub mod ahbsmenr;
///APBSMENR1 register accessor: an alias for `Reg<APBSMENR1_SPEC>`
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1_SPEC>;
///APB peripheral clock enable in Sleep mode register 1
pub mod apbsmenr1;
///APBSMENR2 register accessor: an alias for `Reg<APBSMENR2_SPEC>`
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2_SPEC>;
///APB peripheral clock enable in Sleep mode register 2
pub mod apbsmenr2;
///CCIPR register accessor: an alias for `Reg<CCIPR_SPEC>`
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
///Peripherals independent clock configuration register
pub mod ccipr;
///CCIPR2 register accessor: an alias for `Reg<CCIPR2_SPEC>`
pub type CCIPR2 = crate::Reg<ccipr2::CCIPR2_SPEC>;
///Peripherals independent clock configuration register 2
pub mod ccipr2;
///BDCR register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///RTC domain control register
pub mod bdcr;
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///Control/status register
pub mod csr;
