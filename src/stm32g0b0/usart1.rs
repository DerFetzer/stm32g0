///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr1_fifo: [u8; 0x04],
    ///0x04 - Control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - Control register 3
    pub cr3: crate::Reg<cr3::CR3_SPEC>,
    ///0x0c - Baud rate register
    pub brr: crate::Reg<brr::BRR_SPEC>,
    ///0x10 - Guard time and prescaler register
    pub gtpr: crate::Reg<gtpr::GTPR_SPEC>,
    ///0x14 - Receiver timeout register
    pub rtor: crate::Reg<rtor::RTOR_SPEC>,
    ///0x18 - Request register
    pub rqr: crate::Reg<rqr::RQR_SPEC>,
    _reserved_7_isr_fifo: [u8; 0x04],
    ///0x20 - Interrupt flag clear register
    pub icr: crate::Reg<icr::ICR_SPEC>,
    ///0x24 - Receive data register
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    ///0x28 - Transmit data register
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    ///0x2c - Prescaler register
    pub presc: crate::Reg<presc::PRESC_SPEC>,
}
impl RegisterBlock {
    ///0x00 - Control register 1
    #[inline(always)]
    pub fn cr1_fifo_disabled(&self) -> &crate::Reg<cr1_fifo_disabled::CR1_FIFO_DISABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<cr1_fifo_disabled::CR1_FIFO_DISABLED_SPEC>)
        }
    }
    ///0x00 - Control register 1
    #[inline(always)]
    pub fn cr1_fifo_enabled(&self) -> &crate::Reg<cr1_fifo_enabled::CR1_FIFO_ENABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<cr1_fifo_enabled::CR1_FIFO_ENABLED_SPEC>)
        }
    }
    ///0x1c - Interrupt & status register
    #[inline(always)]
    pub fn isr_fifo_disabled(&self) -> &crate::Reg<isr_fifo_disabled::ISR_FIFO_DISABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<isr_fifo_disabled::ISR_FIFO_DISABLED_SPEC>)
        }
    }
    ///0x1c - Interrupt & status register
    #[inline(always)]
    pub fn isr_fifo_enabled(&self) -> &crate::Reg<isr_fifo_enabled::ISR_FIFO_ENABLED_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<isr_fifo_enabled::ISR_FIFO_ENABLED_SPEC>)
        }
    }
}
///CR1_FIFO_ENABLED register accessor: an alias for `Reg<CR1_FIFO_ENABLED_SPEC>`
pub type CR1_FIFO_ENABLED = crate::Reg<cr1_fifo_enabled::CR1_FIFO_ENABLED_SPEC>;
///Control register 1
pub mod cr1_fifo_enabled;
///CR1_FIFO_DISABLED register accessor: an alias for `Reg<CR1_FIFO_DISABLED_SPEC>`
pub type CR1_FIFO_DISABLED = crate::Reg<cr1_fifo_disabled::CR1_FIFO_DISABLED_SPEC>;
///Control register 1
pub mod cr1_fifo_disabled;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///CR3 register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///Control register 3
pub mod cr3;
///BRR register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///Baud rate register
pub mod brr;
///GTPR register accessor: an alias for `Reg<GTPR_SPEC>`
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
///Guard time and prescaler register
pub mod gtpr;
///RTOR register accessor: an alias for `Reg<RTOR_SPEC>`
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
///Receiver timeout register
pub mod rtor;
///RQR register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///Request register
pub mod rqr;
///ISR_FIFO_ENABLED register accessor: an alias for `Reg<ISR_FIFO_ENABLED_SPEC>`
pub type ISR_FIFO_ENABLED = crate::Reg<isr_fifo_enabled::ISR_FIFO_ENABLED_SPEC>;
///Interrupt & status register
pub mod isr_fifo_enabled;
///ISR_FIFO_DISABLED register accessor: an alias for `Reg<ISR_FIFO_DISABLED_SPEC>`
pub type ISR_FIFO_DISABLED = crate::Reg<isr_fifo_disabled::ISR_FIFO_DISABLED_SPEC>;
///Interrupt & status register
pub mod isr_fifo_disabled;
///ICR register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt flag clear register
pub mod icr;
///RDR register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///Receive data register
pub mod rdr;
///TDR register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///Transmit data register
pub mod tdr;
///PRESC register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///Prescaler register
pub mod presc;
