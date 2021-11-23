///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DMAMux - DMA request line multiplexer channel x control register
    pub c0cr: crate::Reg<c0cr::C0CR_SPEC>,
    ///0x04 - DMAMux - DMA request line multiplexer channel x control register
    pub c1cr: crate::Reg<c1cr::C1CR_SPEC>,
    ///0x08 - DMAMux - DMA request line multiplexer channel x control register
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    ///0x0c - DMAMux - DMA request line multiplexer channel x control register
    pub c3cr: crate::Reg<c3cr::C3CR_SPEC>,
    ///0x10 - DMAMux - DMA request line multiplexer channel x control register
    pub c4cr: crate::Reg<c4cr::C4CR_SPEC>,
    ///0x14 - DMAMux - DMA request line multiplexer channel x control register
    pub c5cr: crate::Reg<c5cr::C5CR_SPEC>,
    ///0x18 - DMAMux - DMA request line multiplexer channel x control register
    pub c6cr: crate::Reg<c6cr::C6CR_SPEC>,
    _reserved7: [u8; 0x64],
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    pub csr: crate::Reg<csr::CSR_SPEC>,
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    _reserved9: [u8; 0x78],
    ///0x100 - DMAMux - DMA request generator channel x control register
    pub rg0cr: crate::Reg<rg0cr::RG0CR_SPEC>,
    ///0x104 - DMAMux - DMA request generator channel x control register
    pub rg1cr: crate::Reg<rg1cr::RG1CR_SPEC>,
    ///0x108 - DMAMux - DMA request generator channel x control register
    pub rg2cr: crate::Reg<rg2cr::RG2CR_SPEC>,
    ///0x10c - DMAMux - DMA request generator channel x control register
    pub rg3cr: crate::Reg<rg3cr::RG3CR_SPEC>,
    _reserved13: [u8; 0x30],
    ///0x140 - DMAMux - DMA request generator status register
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    ///0x144 - DMAMux - DMA request generator clear flag register
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
    _reserved15: [u8; 0x02a4],
    ///0x3ec - DMAMUX hardware configuration 2 register
    pub hwcfgr2: crate::Reg<hwcfgr2::HWCFGR2_SPEC>,
    ///0x3f0 - DMAMUX hardware configuration 1 register
    pub hwcfgr1: crate::Reg<hwcfgr1::HWCFGR1_SPEC>,
    ///0x3f4 - DMAMUX version register
    pub verr: crate::Reg<verr::VERR_SPEC>,
    ///0x3f8 - DMAMUX IP identification register
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    ///0x3fc - DMAMUX size identification register
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
///C0CR register accessor: an alias for `Reg<C0CR_SPEC>`
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c0cr;
///C1CR register accessor: an alias for `Reg<C1CR_SPEC>`
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c1cr;
///C2CR register accessor: an alias for `Reg<C2CR_SPEC>`
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c2cr;
///C3CR register accessor: an alias for `Reg<C3CR_SPEC>`
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c3cr;
///C4CR register accessor: an alias for `Reg<C4CR_SPEC>`
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c4cr;
///C5CR register accessor: an alias for `Reg<C5CR_SPEC>`
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c5cr;
///C6CR register accessor: an alias for `Reg<C6CR_SPEC>`
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod c6cr;
///RG0CR register accessor: an alias for `Reg<RG0CR_SPEC>`
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg0cr;
///RG1CR register accessor: an alias for `Reg<RG1CR_SPEC>`
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg1cr;
///RG2CR register accessor: an alias for `Reg<RG2CR_SPEC>`
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg2cr;
///RG3CR register accessor: an alias for `Reg<RG3CR_SPEC>`
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
///DMAMux - DMA request generator channel x control register
pub mod rg3cr;
///RGSR register accessor: an alias for `Reg<RGSR_SPEC>`
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
///DMAMux - DMA request generator status register
pub mod rgsr;
///RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
///DMAMux - DMA request generator clear flag register
pub mod rgcfr;
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
///CFR register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
///SIDR register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///DMAMUX size identification register
pub mod sidr;
///IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///DMAMUX IP identification register
pub mod ipidr;
///VERR register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///DMAMUX version register
pub mod verr;
///HWCFGR1 register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///DMAMUX hardware configuration 1 register
pub mod hwcfgr1;
///HWCFGR2 register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///DMAMUX hardware configuration 2 register
pub mod hwcfgr2;
