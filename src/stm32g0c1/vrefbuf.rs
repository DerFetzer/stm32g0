///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - VREFBUF control and status register
    pub csr: crate::Reg<csr::CSR_SPEC>,
    ///0x04 - VREFBUF calibration control register
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
}
///CSR register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///VREFBUF control and status register
pub mod csr;
///CCR register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///VREFBUF calibration control register
pub mod ccr;
