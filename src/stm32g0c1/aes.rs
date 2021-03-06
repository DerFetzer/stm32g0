///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - AES control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - AES status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x08 - AES data input register
    pub dinr: crate::Reg<dinr::DINR_SPEC>,
    ///0x0c - AES data output register
    pub doutr: crate::Reg<doutr::DOUTR_SPEC>,
    ///0x10 - AES key register 0
    pub keyr0: crate::Reg<keyr0::KEYR0_SPEC>,
    ///0x14 - AES key register 1
    pub keyr1: crate::Reg<keyr1::KEYR1_SPEC>,
    ///0x18 - AES key register 2
    pub keyr2: crate::Reg<keyr2::KEYR2_SPEC>,
    ///0x1c - AES key register 3
    pub keyr3: crate::Reg<keyr3::KEYR3_SPEC>,
    ///0x20 - AES initialization vector register 0
    pub ivr0: crate::Reg<ivr0::IVR0_SPEC>,
    ///0x24 - AES initialization vector register 1
    pub ivr1: crate::Reg<ivr1::IVR1_SPEC>,
    ///0x28 - AES initialization vector register 2
    pub ivr2: crate::Reg<ivr2::IVR2_SPEC>,
    ///0x2c - AES initialization vector register 3
    pub ivr3: crate::Reg<ivr3::IVR3_SPEC>,
    ///0x30 - AES key register 4
    pub keyr4: crate::Reg<keyr4::KEYR4_SPEC>,
    ///0x34 - AES key register 5
    pub keyr5: crate::Reg<keyr5::KEYR5_SPEC>,
    ///0x38 - AES key register 6
    pub keyr6: crate::Reg<keyr6::KEYR6_SPEC>,
    ///0x3c - AES key register 7
    pub keyr7: crate::Reg<keyr7::KEYR7_SPEC>,
    ///0x40 - AES suspend registers
    pub susp0r: crate::Reg<susp0r::SUSP0R_SPEC>,
    ///0x44 - AES suspend registers
    pub susp1r: crate::Reg<susp1r::SUSP1R_SPEC>,
    ///0x48 - AES suspend registers
    pub susp2r: crate::Reg<susp2r::SUSP2R_SPEC>,
    ///0x4c - AES suspend registers
    pub susp3r: crate::Reg<susp3r::SUSP3R_SPEC>,
    ///0x50 - AES suspend registers
    pub susp4r: crate::Reg<susp4r::SUSP4R_SPEC>,
    ///0x54 - AES suspend registers
    pub susp5r: crate::Reg<susp5r::SUSP5R_SPEC>,
    ///0x58 - AES suspend registers
    pub susp6r: crate::Reg<susp6r::SUSP6R_SPEC>,
    ///0x5c - AES suspend registers
    pub susp7r: crate::Reg<susp7r::SUSP7R_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///AES control register
pub mod cr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///AES status register
pub mod sr;
///DINR register accessor: an alias for `Reg<DINR_SPEC>`
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
///AES data input register
pub mod dinr;
///DOUTR register accessor: an alias for `Reg<DOUTR_SPEC>`
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
///AES data output register
pub mod doutr;
///KEYR0 register accessor: an alias for `Reg<KEYR0_SPEC>`
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
///AES key register 0
pub mod keyr0;
///KEYR1 register accessor: an alias for `Reg<KEYR1_SPEC>`
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
///AES key register 1
pub mod keyr1;
///KEYR2 register accessor: an alias for `Reg<KEYR2_SPEC>`
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
///AES key register 2
pub mod keyr2;
///KEYR3 register accessor: an alias for `Reg<KEYR3_SPEC>`
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
///AES key register 3
pub mod keyr3;
///IVR0 register accessor: an alias for `Reg<IVR0_SPEC>`
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
///AES initialization vector register 0
pub mod ivr0;
///IVR1 register accessor: an alias for `Reg<IVR1_SPEC>`
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
///AES initialization vector register 1
pub mod ivr1;
///IVR2 register accessor: an alias for `Reg<IVR2_SPEC>`
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
///AES initialization vector register 2
pub mod ivr2;
///IVR3 register accessor: an alias for `Reg<IVR3_SPEC>`
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
///AES initialization vector register 3
pub mod ivr3;
///KEYR4 register accessor: an alias for `Reg<KEYR4_SPEC>`
pub type KEYR4 = crate::Reg<keyr4::KEYR4_SPEC>;
///AES key register 4
pub mod keyr4;
///KEYR5 register accessor: an alias for `Reg<KEYR5_SPEC>`
pub type KEYR5 = crate::Reg<keyr5::KEYR5_SPEC>;
///AES key register 5
pub mod keyr5;
///KEYR6 register accessor: an alias for `Reg<KEYR6_SPEC>`
pub type KEYR6 = crate::Reg<keyr6::KEYR6_SPEC>;
///AES key register 6
pub mod keyr6;
///KEYR7 register accessor: an alias for `Reg<KEYR7_SPEC>`
pub type KEYR7 = crate::Reg<keyr7::KEYR7_SPEC>;
///AES key register 7
pub mod keyr7;
///SUSP0R register accessor: an alias for `Reg<SUSP0R_SPEC>`
pub type SUSP0R = crate::Reg<susp0r::SUSP0R_SPEC>;
///AES suspend registers
pub mod susp0r;
///SUSP1R register accessor: an alias for `Reg<SUSP1R_SPEC>`
pub type SUSP1R = crate::Reg<susp1r::SUSP1R_SPEC>;
///AES suspend registers
pub mod susp1r;
///SUSP2R register accessor: an alias for `Reg<SUSP2R_SPEC>`
pub type SUSP2R = crate::Reg<susp2r::SUSP2R_SPEC>;
///AES suspend registers
pub mod susp2r;
///SUSP3R register accessor: an alias for `Reg<SUSP3R_SPEC>`
pub type SUSP3R = crate::Reg<susp3r::SUSP3R_SPEC>;
///AES suspend registers
pub mod susp3r;
///SUSP4R register accessor: an alias for `Reg<SUSP4R_SPEC>`
pub type SUSP4R = crate::Reg<susp4r::SUSP4R_SPEC>;
///AES suspend registers
pub mod susp4r;
///SUSP5R register accessor: an alias for `Reg<SUSP5R_SPEC>`
pub type SUSP5R = crate::Reg<susp5r::SUSP5R_SPEC>;
///AES suspend registers
pub mod susp5r;
///SUSP6R register accessor: an alias for `Reg<SUSP6R_SPEC>`
pub type SUSP6R = crate::Reg<susp6r::SUSP6R_SPEC>;
///AES suspend registers
pub mod susp6r;
///SUSP7R register accessor: an alias for `Reg<SUSP7R_SPEC>`
pub type SUSP7R = crate::Reg<susp7r::SUSP7R_SPEC>;
///AES suspend registers
pub mod susp7r;
