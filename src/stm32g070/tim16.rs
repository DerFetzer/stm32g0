///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x0c - DMA/Interrupt enable register
    pub dier: crate::Reg<dier::DIER_SPEC>,
    ///0x10 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x14 - event generation register
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_5_ccmr1: [u8; 0x04],
    _reserved6: [u8; 0x04],
    ///0x20 - capture/compare enable register
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    ///0x24 - counter
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    ///0x28 - prescaler
    pub psc: crate::Reg<psc::PSC_SPEC>,
    ///0x2c - auto-reload register
    pub arr: crate::Reg<arr::ARR_SPEC>,
    ///0x30 - repetition counter register
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    ///0x34 - capture/compare register 1
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    _reserved12: [u8; 0x0c],
    ///0x44 - break and dead-time register
    pub bdtr: crate::Reg<bdtr::BDTR_SPEC>,
    ///0x48 - DMA control register
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    ///0x4c - DMA address for full transfer
    pub dmar: crate::Reg<dmar::DMAR_SPEC>,
    _reserved15: [u8; 0x10],
    ///0x60 - TIM17 option register 1
    pub af1: crate::Reg<af1::AF1_SPEC>,
    _reserved16: [u8; 0x04],
    ///0x68 - input selection register
    pub tisel: crate::Reg<tisel::TISEL_SPEC>,
}
impl RegisterBlock {
    ///0x18 - capture/compare mode register 1 (input mode)
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>)
        }
    }
    ///0x18 - capture/compare mode register (output mode)
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(24usize)
                as *const crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>)
        }
    }
}
///CR1 register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///DIER register accessor: an alias for `Reg<DIER_SPEC>`
pub type DIER = crate::Reg<dier::DIER_SPEC>;
///DMA/Interrupt enable register
pub mod dier;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///EGR register accessor: an alias for `Reg<EGR_SPEC>`
pub type EGR = crate::Reg<egr::EGR_SPEC>;
///event generation register
pub mod egr;
///CCMR1_Output register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
///capture/compare mode register (output mode)
pub mod ccmr1_output;
///CCMR1_Input register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
///capture/compare mode register 1 (input mode)
pub mod ccmr1_input;
///CCER register accessor: an alias for `Reg<CCER_SPEC>`
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
///capture/compare enable register
pub mod ccer;
///CNT register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///counter
pub mod cnt;
///PSC register accessor: an alias for `Reg<PSC_SPEC>`
pub type PSC = crate::Reg<psc::PSC_SPEC>;
///prescaler
pub mod psc;
///ARR register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///auto-reload register
pub mod arr;
///RCR register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///repetition counter register
pub mod rcr;
///CCR1 register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///capture/compare register 1
pub mod ccr1;
///BDTR register accessor: an alias for `Reg<BDTR_SPEC>`
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
///break and dead-time register
pub mod bdtr;
///DCR register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///DMA control register
pub mod dcr;
///DMAR register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///DMA address for full transfer
pub mod dmar;
///AF1 register accessor: an alias for `Reg<AF1_SPEC>`
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
///TIM17 option register 1
pub mod af1;
///TISEL register accessor: an alias for `Reg<TISEL_SPEC>`
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
///input selection register
pub mod tisel;
