///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: crate::Reg<acr::ACR_SPEC>,
    _reserved1: [u8; 0x04],
    ///0x08 - Flash key register
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
    ///0x0c - Option byte key register
    pub optkeyr: crate::Reg<optkeyr::OPTKEYR_SPEC>,
    ///0x10 - Status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x14 - Flash control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x18 - Flash ECC register
    pub eccr: crate::Reg<eccr::ECCR_SPEC>,
    _reserved6: [u8; 0x04],
    ///0x20 - Flash option register
    pub optr: crate::Reg<optr::OPTR_SPEC>,
    ///0x24 - Flash PCROP zone A Start address register
    pub pcrop1asr: crate::Reg<pcrop1asr::PCROP1ASR_SPEC>,
    ///0x28 - Flash PCROP zone A End address register
    pub pcrop1aer: crate::Reg<pcrop1aer::PCROP1AER_SPEC>,
    ///0x2c - Flash WRP area A address register
    pub wrp1ar: crate::Reg<wrp1ar::WRP1AR_SPEC>,
    ///0x30 - Flash WRP area B address register
    pub wrp1br: crate::Reg<wrp1br::WRP1BR_SPEC>,
    ///0x34 - Flash PCROP zone B Start address register
    pub pcrop1bsr: crate::Reg<pcrop1bsr::PCROP1BSR_SPEC>,
    ///0x38 - Flash PCROP area B End address register
    pub pcrop1ber: crate::Reg<pcrop1ber::PCROP1BER_SPEC>,
    _reserved13: [u8; 0x08],
    ///0x44 - Flash PCROP2 area A start address register
    pub pcrop2asr: crate::Reg<pcrop2asr::PCROP2ASR_SPEC>,
    ///0x48 - Flash PCROP2 area A end address register
    pub pcrop2aer: crate::Reg<pcrop2aer::PCROP2AER_SPEC>,
    ///0x4c - Flash WRP2 area A address register
    pub wrp2ar: crate::Reg<wrp2ar::WRP2AR_SPEC>,
    ///0x50 - Flash WRP2 area B address register
    pub wrp2br: crate::Reg<wrp2br::WRP2BR_SPEC>,
    ///0x54 - FLASH PCROP2 area B start address register
    pub pcrop2bsr: crate::Reg<pcrop2bsr::PCROP2BSR_SPEC>,
    ///0x58 - FLASH PCROP2 area B end address register
    pub pcrop2ber: crate::Reg<pcrop2ber::PCROP2BER_SPEC>,
    _reserved19: [u8; 0x24],
    ///0x80 - Flash Security register
    pub secr: crate::Reg<secr::SECR_SPEC>,
}
///ACR register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Access control register
pub mod acr;
///KEYR register accessor: an alias for `Reg<KEYR_SPEC>`
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
///Flash key register
pub mod keyr;
///OPTKEYR register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///Option byte key register
pub mod optkeyr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Flash control register
pub mod cr;
///ECCR register accessor: an alias for `Reg<ECCR_SPEC>`
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
///Flash ECC register
pub mod eccr;
///OPTR register accessor: an alias for `Reg<OPTR_SPEC>`
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
///Flash option register
pub mod optr;
///PCROP1ASR register accessor: an alias for `Reg<PCROP1ASR_SPEC>`
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASR_SPEC>;
///Flash PCROP zone A Start address register
pub mod pcrop1asr;
///PCROP1AER register accessor: an alias for `Reg<PCROP1AER_SPEC>`
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AER_SPEC>;
///Flash PCROP zone A End address register
pub mod pcrop1aer;
///WRP1AR register accessor: an alias for `Reg<WRP1AR_SPEC>`
pub type WRP1AR = crate::Reg<wrp1ar::WRP1AR_SPEC>;
///Flash WRP area A address register
pub mod wrp1ar;
///WRP1BR register accessor: an alias for `Reg<WRP1BR_SPEC>`
pub type WRP1BR = crate::Reg<wrp1br::WRP1BR_SPEC>;
///Flash WRP area B address register
pub mod wrp1br;
///PCROP1BSR register accessor: an alias for `Reg<PCROP1BSR_SPEC>`
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSR_SPEC>;
///Flash PCROP zone B Start address register
pub mod pcrop1bsr;
///PCROP1BER register accessor: an alias for `Reg<PCROP1BER_SPEC>`
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BER_SPEC>;
///Flash PCROP area B End address register
pub mod pcrop1ber;
///PCROP2ASR register accessor: an alias for `Reg<PCROP2ASR_SPEC>`
pub type PCROP2ASR = crate::Reg<pcrop2asr::PCROP2ASR_SPEC>;
///Flash PCROP2 area A start address register
pub mod pcrop2asr;
///PCROP2AER register accessor: an alias for `Reg<PCROP2AER_SPEC>`
pub type PCROP2AER = crate::Reg<pcrop2aer::PCROP2AER_SPEC>;
///Flash PCROP2 area A end address register
pub mod pcrop2aer;
///WRP2AR register accessor: an alias for `Reg<WRP2AR_SPEC>`
pub type WRP2AR = crate::Reg<wrp2ar::WRP2AR_SPEC>;
///Flash WRP2 area A address register
pub mod wrp2ar;
///WRP2BR register accessor: an alias for `Reg<WRP2BR_SPEC>`
pub type WRP2BR = crate::Reg<wrp2br::WRP2BR_SPEC>;
///Flash WRP2 area B address register
pub mod wrp2br;
///PCROP2BSR register accessor: an alias for `Reg<PCROP2BSR_SPEC>`
pub type PCROP2BSR = crate::Reg<pcrop2bsr::PCROP2BSR_SPEC>;
///FLASH PCROP2 area B start address register
pub mod pcrop2bsr;
///PCROP2BER register accessor: an alias for `Reg<PCROP2BER_SPEC>`
pub type PCROP2BER = crate::Reg<pcrop2ber::PCROP2BER_SPEC>;
///FLASH PCROP2 area B end address register
pub mod pcrop2ber;
///SECR register accessor: an alias for `Reg<SECR_SPEC>`
pub type SECR = crate::Reg<secr::SECR_SPEC>;
///Flash Security register
pub mod secr;
