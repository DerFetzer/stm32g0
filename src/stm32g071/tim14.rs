///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    _reserved1: [u8; 0x08],
    ///0x0c - DMA/Interrupt enable register
    pub dier: crate::Reg<dier::DIER_SPEC>,
    ///0x10 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x14 - event generation register
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_4_ccmr1: [u8; 0x04],
    _reserved5: [u8; 0x04],
    ///0x20 - capture/compare enable register
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    ///0x24 - counter
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    ///0x28 - prescaler
    pub psc: crate::Reg<psc::PSC_SPEC>,
    ///0x2c - auto-reload register
    pub arr: crate::Reg<arr::ARR_SPEC>,
    _reserved9: [u8; 0x04],
    ///0x34 - capture/compare register 1
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    _reserved10: [u8; 0x30],
    ///0x68 - TIM timer input selection register
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
    ///0x18 - capture/compare mode register 1 (output mode)
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
///capture/compare mode register 1 (output mode)
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
///CCR1 register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///capture/compare register 1
pub mod ccr1;
///TISEL register accessor: an alias for `Reg<TISEL_SPEC>`
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
///TIM timer input selection register
pub mod tisel;
