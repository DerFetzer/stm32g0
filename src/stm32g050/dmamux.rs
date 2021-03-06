#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMUX request line multiplexer channel x configuration register"]
    pub c0cr: crate::Reg<c0cr::C0CR_SPEC>,
    #[doc = "0x04 - DMAMUX request line multiplexer channel x configuration register"]
    pub c1cr: crate::Reg<c1cr::C1CR_SPEC>,
    #[doc = "0x08 - DMAMUX request line multiplexer channel x configuration register"]
    pub c2cr: crate::Reg<c2cr::C2CR_SPEC>,
    #[doc = "0x0c - DMAMUX request line multiplexer channel x configuration register"]
    pub c3cr: crate::Reg<c3cr::C3CR_SPEC>,
    #[doc = "0x10 - DMAMUX request line multiplexer channel x configuration register"]
    pub c4cr: crate::Reg<c4cr::C4CR_SPEC>,
    #[doc = "0x14 - DMAMUX request line multiplexer channel x configuration register"]
    pub c5cr: crate::Reg<c5cr::C5CR_SPEC>,
    #[doc = "0x18 - DMAMUX request line multiplexer channel x configuration register"]
    pub c6cr: crate::Reg<c6cr::C6CR_SPEC>,
    #[doc = "0x1c - DMAMUX request line multiplexer channel x configuration register"]
    pub c7cr: crate::Reg<c7cr::C7CR_SPEC>,
    #[doc = "0x20 - DMAMUX request line multiplexer channel x configuration register"]
    pub c8cr: crate::Reg<c8cr::C8CR_SPEC>,
    #[doc = "0x24 - DMAMUX request line multiplexer channel x configuration register"]
    pub c9cr: crate::Reg<c9cr::C9CR_SPEC>,
    #[doc = "0x28 - DMAMUX request line multiplexer channel x configuration register"]
    pub c10cr: crate::Reg<c10cr::C10CR_SPEC>,
    #[doc = "0x2c - DMAMUX request line multiplexer channel x configuration register"]
    pub c11cr: crate::Reg<c11cr::C11CR_SPEC>,
    _reserved12: [u8; 0x50],
    #[doc = "0x80 - DMAMUX request line multiplexer interrupt channel status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x84 - DMAMUX request line multiplexer interrupt clear flag register"]
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    _reserved14: [u8; 0x78],
    #[doc = "0x100 - DMAMUX request generator channel x configuration register"]
    pub rg0cr: crate::Reg<rg0cr::RG0CR_SPEC>,
    #[doc = "0x104 - DMAMUX request generator channel x configuration register"]
    pub rg1cr: crate::Reg<rg1cr::RG1CR_SPEC>,
    #[doc = "0x108 - DMAMUX request generator channel x configuration register"]
    pub rg2cr: crate::Reg<rg2cr::RG2CR_SPEC>,
    #[doc = "0x10c - DMAMUX request generator channel x configuration register"]
    pub rg3cr: crate::Reg<rg3cr::RG3CR_SPEC>,
    _reserved18: [u8; 0x30],
    #[doc = "0x140 - DMAMUX request generator interrupt status register"]
    pub rgsr: crate::Reg<rgsr::RGSR_SPEC>,
    #[doc = "0x144 - DMAMUX request generator interrupt clear flag register"]
    pub rgcfr: crate::Reg<rgcfr::RGCFR_SPEC>,
}
#[doc = "C0CR register accessor: an alias for `Reg<C0CR_SPEC>`"]
pub type C0CR = crate::Reg<c0cr::C0CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c0cr;
#[doc = "C1CR register accessor: an alias for `Reg<C1CR_SPEC>`"]
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c1cr;
#[doc = "C2CR register accessor: an alias for `Reg<C2CR_SPEC>`"]
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c2cr;
#[doc = "C3CR register accessor: an alias for `Reg<C3CR_SPEC>`"]
pub type C3CR = crate::Reg<c3cr::C3CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c3cr;
#[doc = "C4CR register accessor: an alias for `Reg<C4CR_SPEC>`"]
pub type C4CR = crate::Reg<c4cr::C4CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c4cr;
#[doc = "C5CR register accessor: an alias for `Reg<C5CR_SPEC>`"]
pub type C5CR = crate::Reg<c5cr::C5CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c5cr;
#[doc = "C6CR register accessor: an alias for `Reg<C6CR_SPEC>`"]
pub type C6CR = crate::Reg<c6cr::C6CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c6cr;
#[doc = "C7CR register accessor: an alias for `Reg<C7CR_SPEC>`"]
pub type C7CR = crate::Reg<c7cr::C7CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c7cr;
#[doc = "C8CR register accessor: an alias for `Reg<C8CR_SPEC>`"]
pub type C8CR = crate::Reg<c8cr::C8CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c8cr;
#[doc = "C9CR register accessor: an alias for `Reg<C9CR_SPEC>`"]
pub type C9CR = crate::Reg<c9cr::C9CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c9cr;
#[doc = "C10CR register accessor: an alias for `Reg<C10CR_SPEC>`"]
pub type C10CR = crate::Reg<c10cr::C10CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c10cr;
#[doc = "C11CR register accessor: an alias for `Reg<C11CR_SPEC>`"]
pub type C11CR = crate::Reg<c11cr::C11CR_SPEC>;
#[doc = "DMAMUX request line multiplexer channel x configuration register"]
pub mod c11cr;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt channel status register"]
pub mod csr;
#[doc = "CFR register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "DMAMUX request line multiplexer interrupt clear flag register"]
pub mod cfr;
#[doc = "RG0CR register accessor: an alias for `Reg<RG0CR_SPEC>`"]
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg0cr;
#[doc = "RG1CR register accessor: an alias for `Reg<RG1CR_SPEC>`"]
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg1cr;
#[doc = "RG2CR register accessor: an alias for `Reg<RG2CR_SPEC>`"]
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg2cr;
#[doc = "RG3CR register accessor: an alias for `Reg<RG3CR_SPEC>`"]
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
#[doc = "DMAMUX request generator channel x configuration register"]
pub mod rg3cr;
#[doc = "RGSR register accessor: an alias for `Reg<RGSR_SPEC>`"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMAMUX request generator interrupt status register"]
pub mod rgsr;
#[doc = "RGCFR register accessor: an alias for `Reg<RGCFR_SPEC>`"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMAMUX request generator interrupt clear flag register"]
pub mod rgcfr;
