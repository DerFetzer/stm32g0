///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x08 - data input register
    pub dinr: crate::Reg<dinr::DINR_SPEC>,
    ///0x0c - data output register
    pub doutr: crate::Reg<doutr::DOUTR_SPEC>,
    ///0x10 - key register 0
    pub keyr0: crate::Reg<keyr0::KEYR0_SPEC>,
    ///0x14 - key register 1
    pub keyr1: crate::Reg<keyr1::KEYR1_SPEC>,
    ///0x18 - key register 2
    pub keyr2: crate::Reg<keyr2::KEYR2_SPEC>,
    ///0x1c - key register 3
    pub keyr3: crate::Reg<keyr3::KEYR3_SPEC>,
    ///0x20 - initialization vector register 0
    pub ivr0: crate::Reg<ivr0::IVR0_SPEC>,
    ///0x24 - initialization vector register 1
    pub ivr1: crate::Reg<ivr1::IVR1_SPEC>,
    ///0x28 - initialization vector register 2
    pub ivr2: crate::Reg<ivr2::IVR2_SPEC>,
    ///0x2c - initialization vector register 3
    pub ivr3: crate::Reg<ivr3::IVR3_SPEC>,
    ///0x30 - key register 4
    pub keyr4: crate::Reg<keyr4::KEYR4_SPEC>,
    ///0x34 - key register 5
    pub keyr5: crate::Reg<keyr5::KEYR5_SPEC>,
    ///0x38 - key register 6
    pub keyr6: crate::Reg<keyr6::KEYR6_SPEC>,
    ///0x3c - key register 7
    pub keyr7: crate::Reg<keyr7::KEYR7_SPEC>,
    ///0x40 - AES suspend register 0
    pub susp0r: crate::Reg<susp0r::SUSP0R_SPEC>,
    ///0x44 - AES suspend register 1
    pub susp1r: crate::Reg<susp1r::SUSP1R_SPEC>,
    ///0x48 - AES suspend register 2
    pub susp2r: crate::Reg<susp2r::SUSP2R_SPEC>,
    ///0x4c - AES suspend register 3
    pub susp3r: crate::Reg<susp3r::SUSP3R_SPEC>,
    ///0x50 - AES suspend register 4
    pub susp4r: crate::Reg<susp4r::SUSP4R_SPEC>,
    ///0x54 - AES suspend register 5
    pub susp5r: crate::Reg<susp5r::SUSP5R_SPEC>,
    ///0x58 - AES suspend register 6
    pub susp6r: crate::Reg<susp6r::SUSP6R_SPEC>,
    ///0x5c - AES suspend register 7
    pub susp7r: crate::Reg<susp7r::SUSP7R_SPEC>,
    _reserved24: [u8; 0x0390],
    ///0x3f0 - AES hardware configuration register
    pub hwcfr: crate::Reg<hwcfr::HWCFR_SPEC>,
    ///0x3f4 - AES version register
    pub verr: crate::Reg<verr::VERR_SPEC>,
    ///0x3f8 - AES identification register
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    ///0x3fc - AES size ID register
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DINR register accessor: an alias for `Reg<DINR_SPEC>`
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
///data input register
pub mod dinr;
///DOUTR register accessor: an alias for `Reg<DOUTR_SPEC>`
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
///data output register
pub mod doutr;
///KEYR0 register accessor: an alias for `Reg<KEYR0_SPEC>`
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
///key register 0
pub mod keyr0;
///KEYR1 register accessor: an alias for `Reg<KEYR1_SPEC>`
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
///key register 1
pub mod keyr1;
///KEYR2 register accessor: an alias for `Reg<KEYR2_SPEC>`
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
///key register 2
pub mod keyr2;
///KEYR3 register accessor: an alias for `Reg<KEYR3_SPEC>`
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
///key register 3
pub mod keyr3;
///IVR0 register accessor: an alias for `Reg<IVR0_SPEC>`
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
///initialization vector register 0
pub mod ivr0;
///IVR1 register accessor: an alias for `Reg<IVR1_SPEC>`
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
///initialization vector register 1
pub mod ivr1;
///IVR2 register accessor: an alias for `Reg<IVR2_SPEC>`
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
///initialization vector register 2
pub mod ivr2;
///IVR3 register accessor: an alias for `Reg<IVR3_SPEC>`
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
///initialization vector register 3
pub mod ivr3;
///KEYR4 register accessor: an alias for `Reg<KEYR4_SPEC>`
pub type KEYR4 = crate::Reg<keyr4::KEYR4_SPEC>;
///key register 4
pub mod keyr4;
///KEYR5 register accessor: an alias for `Reg<KEYR5_SPEC>`
pub type KEYR5 = crate::Reg<keyr5::KEYR5_SPEC>;
///key register 5
pub mod keyr5;
///KEYR6 register accessor: an alias for `Reg<KEYR6_SPEC>`
pub type KEYR6 = crate::Reg<keyr6::KEYR6_SPEC>;
///key register 6
pub mod keyr6;
///KEYR7 register accessor: an alias for `Reg<KEYR7_SPEC>`
pub type KEYR7 = crate::Reg<keyr7::KEYR7_SPEC>;
///key register 7
pub mod keyr7;
///SUSP0R register accessor: an alias for `Reg<SUSP0R_SPEC>`
pub type SUSP0R = crate::Reg<susp0r::SUSP0R_SPEC>;
///AES suspend register 0
pub mod susp0r;
///SUSP1R register accessor: an alias for `Reg<SUSP1R_SPEC>`
pub type SUSP1R = crate::Reg<susp1r::SUSP1R_SPEC>;
///AES suspend register 1
pub mod susp1r;
///SUSP2R register accessor: an alias for `Reg<SUSP2R_SPEC>`
pub type SUSP2R = crate::Reg<susp2r::SUSP2R_SPEC>;
///AES suspend register 2
pub mod susp2r;
///SUSP3R register accessor: an alias for `Reg<SUSP3R_SPEC>`
pub type SUSP3R = crate::Reg<susp3r::SUSP3R_SPEC>;
///AES suspend register 3
pub mod susp3r;
///SUSP4R register accessor: an alias for `Reg<SUSP4R_SPEC>`
pub type SUSP4R = crate::Reg<susp4r::SUSP4R_SPEC>;
///AES suspend register 4
pub mod susp4r;
///SUSP5R register accessor: an alias for `Reg<SUSP5R_SPEC>`
pub type SUSP5R = crate::Reg<susp5r::SUSP5R_SPEC>;
///AES suspend register 5
pub mod susp5r;
///SUSP6R register accessor: an alias for `Reg<SUSP6R_SPEC>`
pub type SUSP6R = crate::Reg<susp6r::SUSP6R_SPEC>;
///AES suspend register 6
pub mod susp6r;
///SUSP7R register accessor: an alias for `Reg<SUSP7R_SPEC>`
pub type SUSP7R = crate::Reg<susp7r::SUSP7R_SPEC>;
///AES suspend register 7
pub mod susp7r;
///HWCFR register accessor: an alias for `Reg<HWCFR_SPEC>`
pub type HWCFR = crate::Reg<hwcfr::HWCFR_SPEC>;
///AES hardware configuration register
pub mod hwcfr;
///VERR register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///AES version register
pub mod verr;
///IPIDR register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///AES identification register
pub mod ipidr;
///SIDR register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///AES size ID register
pub mod sidr;
