///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DMAMUX request line multiplexer channel x configuration register
    pub c0cr: crate::Reg<c0cr::C0CR_SPEC>,
    ///0x04 - DMAMUX request line multiplexer channel x configuration register
    pub c1cr: crate::Reg<c1cr::C1CR_SPEC>,
    ///0x08 - DMAMUX request line multiplexer channel x configuration register
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    ///0x0c - DMAMUX request line multiplexer channel x configuration register
    pub c3cr: crate::Reg<c3cr::C3CR_SPEC>,
    ///0x10 - DMAMUX request line multiplexer channel x configuration register
    pub c4cr: crate::Reg<c4cr::C4CR_SPEC>,
    ///0x14 - DMAMUX request line multiplexer channel x configuration register
    pub c5cr: crate::Reg<c5cr::C5CR_SPEC>,
    ///0x18 - DMAMUX request line multiplexer channel x configuration register
    pub c6cr: crate::Reg<c6cr::C6CR_SPEC>,
    _reserved7: [u8; 0x64],
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    pub csr: crate::Reg<csr::CSR_SPEC>,
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    _reserved9: [u8; 0x78],
    ///0x100 - DMAMUX request generator channel x configuration register
    pub rg0cr: crate::Reg<rg0cr::RG0CR_SPEC>,
    ///0x104 - DMAMUX request generator channel x configuration register
    pub rg1cr: crate::Reg<rg1cr::RG1CR_SPEC>,
    ///0x108 - DMAMUX request generator channel x configuration register
    pub rg2cr: crate::Reg<rg2cr::RG2CR_SPEC>,
    ///0x10c - DMAMUX request generator channel x configuration register
    pub rg3cr: crate::Reg<rg3cr::RG3CR_SPEC>,
    _reserved13: [u8; 0x30],
    ///0x140 - DMAMUX request generator interrupt status register
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    ///0x144 - DMAMUX request generator interrupt clear flag register
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
}
///C0CR register accessor: an alias for `Reg<C0CR_SPEC>`
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c0cr;
///C1CR register accessor: an alias for `Reg<C1CR_SPEC>`
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c1cr;
///C2CR register accessor: an alias for `Reg<C2CR_SPEC>`
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c2cr;
///C3CR register accessor: an alias for `Reg<C3CR_SPEC>`
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c3cr;
///C4CR register accessor: an alias for `Reg<C4CR_SPEC>`
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c4cr;
///C5CR register accessor: an alias for `Reg<C5CR_SPEC>`
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c5cr;
///C6CR register accessor: an alias for `Reg<C6CR_SPEC>`
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
///DMAMUX request line multiplexer channel x configuration register
pub mod c6cr;
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
///CFR register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
///RG0CR register accessor: an alias for `Reg<RG0CR_SPEC>`
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
///DMAMUX request generator channel x configuration register
pub mod rg0cr;
///RG1CR register accessor: an alias for `Reg<RG1CR_SPEC>`
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
///DMAMUX request generator channel x configuration register
pub mod rg1cr;
///RG2CR register accessor: an alias for `Reg<RG2CR_SPEC>`
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
///DMAMUX request generator channel x configuration register
pub mod rg2cr;
///RG3CR register accessor: an alias for `Reg<RG3CR_SPEC>`
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
///DMAMUX request generator channel x configuration register
pub mod rg3cr;
///RGSR register accessor: an alias for `Reg<RGSR_SPEC>`
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
///DMAMUX request generator interrupt status register
pub mod rgsr;
///RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
///DMAMUX request generator interrupt clear flag register
pub mod rgcfr;
