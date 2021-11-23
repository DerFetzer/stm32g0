///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DAC control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x04 - DAC software trigger register
    pub swtrgr: crate::Reg<swtrgr::SWTRGR_SPEC>,
    ///0x08 - DAC channel1 12-bit right-aligned data holding register
    pub dhr12r1: crate::Reg<dhr12r1::DHR12R1_SPEC>,
    ///0x0c - DAC channel1 12-bit left aligned data holding register
    pub dhr12l1: crate::Reg<dhr12l1::DHR12L1_SPEC>,
    ///0x10 - DAC channel1 8-bit right aligned data holding register
    pub dhr8r1: crate::Reg<dhr8r1::DHR8R1_SPEC>,
    ///0x14 - DAC channel2 12-bit right aligned data holding register
    pub dhr12r2: crate::Reg<dhr12r2::DHR12R2_SPEC>,
    ///0x18 - DAC channel2 12-bit left aligned data holding register
    pub dhr12l2: crate::Reg<dhr12l2::DHR12L2_SPEC>,
    ///0x1c - DAC channel2 8-bit right-aligned data holding register
    pub dhr8r2: crate::Reg<dhr8r2::DHR8R2_SPEC>,
    ///0x20 - Dual DAC 12-bit right-aligned data holding register
    pub dhr12rd: crate::Reg<dhr12rd::DHR12RD_SPEC>,
    ///0x24 - DUAL DAC 12-bit left aligned data holding register
    pub dhr12ld: crate::Reg<dhr12ld::DHR12LD_SPEC>,
    ///0x28 - DUAL DAC 8-bit right aligned data holding register
    pub dhr8rd: crate::Reg<dhr8rd::DHR8RD_SPEC>,
    ///0x2c - DAC channel1 data output register
    pub dor1: crate::Reg<dor1::DOR1_SPEC>,
    ///0x30 - DAC channel2 data output register
    pub dor2: crate::Reg<dor2::DOR2_SPEC>,
    ///0x34 - DAC status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x38 - DAC calibration control register
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    ///0x3c - DAC mode control register
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    ///0x40 - DAC Sample and Hold sample time register 1
    pub shsr1: crate::Reg<shsr1::SHSR1_SPEC>,
    ///0x44 - DAC Sample and Hold sample time register 2
    pub shsr2: crate::Reg<shsr2::SHSR2_SPEC>,
    ///0x48 - DAC Sample and Hold hold time register
    pub shhr: crate::Reg<shhr::SHHR_SPEC>,
    ///0x4c - DAC Sample and Hold refresh time register
    pub shrr: crate::Reg<shrr::SHRR_SPEC>,
    _reserved20: [u8; 0x03a0],
    ///0x3f0 - DAC IP Hardware Configuration Register
    pub ip_hwcfgr0: crate::Reg<ip_hwcfgr0::IP_HWCFGR0_SPEC>,
    ///0x3f4 - EXTI IP Version register
    pub verr: crate::Reg<verr::VERR_SPEC>,
    ///0x3f8 - EXTI Identification register
    pub ipidr: crate::Reg<ipidr::IPIDR_SPEC>,
    ///0x3fc - EXTI Size ID register
    pub sidr: crate::Reg<sidr::SIDR_SPEC>,
}
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DAC control register
pub mod cr;
///SWTRGR register accessor: an alias for `Reg<SWTRGR_SPEC>`
pub type SWTRGR = crate::Reg<swtrgr::SWTRGR_SPEC>;
///DAC software trigger register
pub mod swtrgr;
///DHR12R1 register accessor: an alias for `Reg<DHR12R1_SPEC>`
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
///DAC channel1 12-bit right-aligned data holding register
pub mod dhr12r1;
///DHR12L1 register accessor: an alias for `Reg<DHR12L1_SPEC>`
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
///DAC channel1 12-bit left aligned data holding register
pub mod dhr12l1;
///DHR8R1 register accessor: an alias for `Reg<DHR8R1_SPEC>`
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
///DAC channel1 8-bit right aligned data holding register
pub mod dhr8r1;
///DHR12R2 register accessor: an alias for `Reg<DHR12R2_SPEC>`
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2_SPEC>;
///DAC channel2 12-bit right aligned data holding register
pub mod dhr12r2;
///DHR12L2 register accessor: an alias for `Reg<DHR12L2_SPEC>`
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2_SPEC>;
///DAC channel2 12-bit left aligned data holding register
pub mod dhr12l2;
///DHR8R2 register accessor: an alias for `Reg<DHR8R2_SPEC>`
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2_SPEC>;
///DAC channel2 8-bit right-aligned data holding register
pub mod dhr8r2;
///DHR12RD register accessor: an alias for `Reg<DHR12RD_SPEC>`
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
///Dual DAC 12-bit right-aligned data holding register
pub mod dhr12rd;
///DHR12LD register accessor: an alias for `Reg<DHR12LD_SPEC>`
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
///DUAL DAC 12-bit left aligned data holding register
pub mod dhr12ld;
///DHR8RD register accessor: an alias for `Reg<DHR8RD_SPEC>`
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
///DUAL DAC 8-bit right aligned data holding register
pub mod dhr8rd;
///DOR1 register accessor: an alias for `Reg<DOR1_SPEC>`
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
///DAC channel1 data output register
pub mod dor1;
///DOR2 register accessor: an alias for `Reg<DOR2_SPEC>`
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
///DAC channel2 data output register
pub mod dor2;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///DAC status register
pub mod sr;
///CCR register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///DAC calibration control register
pub mod ccr;
///MCR register accessor: an alias for `Reg<MCR_SPEC>`
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
///DAC mode control register
pub mod mcr;
///SHSR1 register accessor: an alias for `Reg<SHSR1_SPEC>`
pub type SHSR1 = crate::Reg<shsr1::SHSR1_SPEC>;
///DAC Sample and Hold sample time register 1
pub mod shsr1;
///SHSR2 register accessor: an alias for `Reg<SHSR2_SPEC>`
pub type SHSR2 = crate::Reg<shsr2::SHSR2_SPEC>;
///DAC Sample and Hold sample time register 2
pub mod shsr2;
///SHHR register accessor: an alias for `Reg<SHHR_SPEC>`
pub type SHHR = crate::Reg<shhr::SHHR_SPEC>;
///DAC Sample and Hold hold time register
pub mod shhr;
///SHRR register accessor: an alias for `Reg<SHRR_SPEC>`
pub type SHRR = crate::Reg<shrr::SHRR_SPEC>;
///DAC Sample and Hold refresh time register
pub mod shrr;
///IP_HWCFGR0 register accessor: an alias for `Reg<IP_HWCFGR0_SPEC>`
pub type IP_HWCFGR0 = crate::Reg<ip_hwcfgr0::IP_HWCFGR0_SPEC>;
///DAC IP Hardware Configuration Register
pub mod ip_hwcfgr0;
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
