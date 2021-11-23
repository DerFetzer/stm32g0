///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Key register
    pub kr: crate::Reg<kr::KR_SPEC>,
    ///0x04 - Prescaler register
    pub pr: crate::Reg<pr::PR_SPEC>,
    ///0x08 - Reload register
    pub rlr: crate::Reg<rlr::RLR_SPEC>,
    ///0x0c - Status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x10 - Window register
    pub winr: crate::Reg<winr::WINR_SPEC>,
    _reserved5: [u8; 0x03dc],
    ///0x3f0 - hardware configuration register
    pub hwcfgr: crate::Reg<hwcfgr::HWCFGR_SPEC>,
    ///0x3f4 - EXTI IP Version register
    pub verr: crate::Reg<verr::VERR_SPEC>,
    ///0x3f8 - EXTI Identification register
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    ///0x3fc - EXTI Size ID register
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
///KR register accessor: an alias for `Reg<KR_SPEC>`
pub type KR = crate::Reg<kr::KR_SPEC>;
///Key register
pub mod kr;
///PR register accessor: an alias for `Reg<PR_SPEC>`
pub type PR = crate::Reg<pr::PR_SPEC>;
///Prescaler register
pub mod pr;
///RLR register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///Reload register
pub mod rlr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///WINR register accessor: an alias for `Reg<WINR_SPEC>`
pub type WINR = crate::Reg<winr::WINR_SPEC>;
///Window register
pub mod winr;
///HWCFGR register accessor: an alias for `Reg<HWCFGR_SPEC>`
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
///hardware configuration register
pub mod hwcfgr;
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
