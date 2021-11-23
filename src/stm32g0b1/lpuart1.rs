///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 0x04],
    ///0x04 - LPUART control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - LPUART control register 3
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    ///0x0c - LPUART baud rate register
    pub brr: crate::Reg<brr::BRR_SPEC>,
    _reserved4: [u8; 0x08],
    ///0x18 - LPUART request register
    pub rqr: crate::Reg<rqr::RQR_SPEC>,
    _reserved_5_isr: [u8; 0x04],
    ///0x20 - LPUART interrupt flag clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x24 - LPUART receive data register
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    ///0x28 - LPUART transmit data register
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    ///0x2c - LPUART prescaler register
    pub presc: crate::Reg<presc::PRESC_SPEC>,
}
impl RegisterBlock {
    ///0x00 - LPUART control register 1 \[alternate\]
    #[inline(always)]
    pub fn cr1_disabled(&self) -> &crate::Reg<cr1_disabled::CR1_DISABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<cr1_disabled::CR1_DISABLED_SPEC>)
        }
    }
    ///0x00 - LPUART control register 1 \[alternate\]
    #[inline(always)]
    pub fn cr1_enabled(&self) -> &crate::Reg<cr1_enabled::CR1_ENABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<cr1_enabled::CR1_ENABLED_SPEC>)
        }
    }
    ///0x1c - LPUART interrupt and status register \[alternate\]
    #[inline(always)]
    pub fn isr_disabled(&self) -> &crate::Reg<isr_disabled::ISR_DISABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<isr_disabled::ISR_DISABLED_SPEC>)
        }
    }
    ///0x1c - LPUART interrupt and status register \[alternate\]
    #[inline(always)]
    pub fn isr_enabled(&self) -> &crate::Reg<isr_enabled::ISR_ENABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<isr_enabled::ISR_ENABLED_SPEC>)
        }
    }
}
///CR1_enabled register accessor: an alias for `Reg<CR1_ENABLED_SPEC>`
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLED_SPEC>;
///LPUART control register 1 \[alternate\]
pub mod cr1_enabled;
///CR1_disabled register accessor: an alias for `Reg<CR1_DISABLED_SPEC>`
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLED_SPEC>;
///LPUART control register 1 \[alternate\]
pub mod cr1_disabled;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///LPUART control register 2
pub mod cr2;
///CR3 register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///LPUART control register 3
pub mod cr3;
///BRR register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///LPUART baud rate register
pub mod brr;
///RQR register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///LPUART request register
pub mod rqr;
///ISR_enabled register accessor: an alias for `Reg<ISR_ENABLED_SPEC>`
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLED_SPEC>;
///LPUART interrupt and status register \[alternate\]
pub mod isr_enabled;
///ISR_disabled register accessor: an alias for `Reg<ISR_DISABLED_SPEC>`
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLED_SPEC>;
///LPUART interrupt and status register \[alternate\]
pub mod isr_disabled;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///LPUART interrupt flag clear register
pub mod icr;
///RDR register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///LPUART receive data register
pub mod rdr;
///TDR register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///LPUART transmit data register
pub mod tdr;
///PRESC register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///LPUART prescaler register
pub mod presc;
