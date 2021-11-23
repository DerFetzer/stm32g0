///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    ///0x04 - control register 2
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    ///0x08 - slave mode control register
    pub smcr: crate::Reg<smcr::SMCR_SPEC>,
    ///0x0c - DMA/Interrupt enable register
    pub dier: crate::Reg<dier::DIER_SPEC>,
    ///0x10 - status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x14 - event generation register
    pub egr: crate::Reg<egr::EGR_SPEC>,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    ///0x20 - capture/compare enable register
    pub ccer: crate::Reg<ccer::CCER_SPEC>,
    _reserved_9_cnt: [u8; 0x04],
    ///0x28 - prescaler
    pub psc: crate::Reg<psc::PSC_SPEC>,
    ///0x2c - auto-reload register
    pub arr: crate::Reg<arr::ARR_SPEC>,
    _reserved12: [u8; 0x04],
    ///0x34 - capture/compare register 1
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    ///0x38 - capture/compare register 2
    pub ccr2: crate::Reg<ccr2::CCR2_SPEC>,
    ///0x3c - capture/compare register 3
    pub ccr3: crate::Reg<ccr3::CCR3_SPEC>,
    ///0x40 - capture/compare register 4
    pub ccr4: crate::Reg<ccr4::CCR4_SPEC>,
    _reserved16: [u8; 0x04],
    ///0x48 - DMA control register
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    ///0x4c - DMA address for full transfer
    pub dmar: crate::Reg<dmar::DMAR_SPEC>,
    ///0x50 - TIM option register
    pub or1: crate::Reg<or1::OR1_SPEC>,
    _reserved19: [u8; 0x0c],
    ///0x60 - TIM alternate function option register 1
    pub af1: crate::Reg<af1::AF1_SPEC>,
    _reserved20: [u8; 0x04],
    ///0x68 - TIM alternate function option register 1
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
    ///0x1c - capture/compare mode register 2 (input mode)
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>)
        }
    }
    ///0x1c - capture/compare mode register 2 (output mode)
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(28usize)
                as *const crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>)
        }
    }
    ///0x24 - counter
    #[inline(always)]
    pub fn cnt_alternate5(&self) -> &crate::Reg<cnt_alternate5::CNT_ALTERNATE5_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<cnt_alternate5::CNT_ALTERNATE5_SPEC>)
        }
    }
    ///0x24 - counter
    #[inline(always)]
    pub fn cnt(&self) -> &crate::Reg<cnt::CNT_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(36usize)
                as *const crate::Reg<cnt::CNT_SPEC>)
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
///SMCR register accessor: an alias for `Reg<SMCR_SPEC>`
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
///slave mode control register
pub mod smcr;
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
///CCMR2_Output register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
///capture/compare mode register 2 (output mode)
pub mod ccmr2_output;
///CCMR2_Input register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
///capture/compare mode register 2 (input mode)
pub mod ccmr2_input;
///CCER register accessor: an alias for `Reg<CCER_SPEC>`
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
///capture/compare enable register
pub mod ccer;
///CNT register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///counter
pub mod cnt;
///CNT_ALTERNATE5 register accessor: an alias for `Reg<CNT_ALTERNATE5_SPEC>`
pub type CNT_ALTERNATE5 = crate::Reg<cnt_alternate5::CNT_ALTERNATE5_SPEC>;
///counter
pub mod cnt_alternate5;
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
///CCR2 register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///capture/compare register 2
pub mod ccr2;
///CCR3 register accessor: an alias for `Reg<CCR3_SPEC>`
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
///capture/compare register 3
pub mod ccr3;
///CCR4 register accessor: an alias for `Reg<CCR4_SPEC>`
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
///capture/compare register 4
pub mod ccr4;
///DCR register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///DMA control register
pub mod dcr;
///DMAR register accessor: an alias for `Reg<DMAR_SPEC>`
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
///DMA address for full transfer
pub mod dmar;
///OR1 register accessor: an alias for `Reg<OR1_SPEC>`
pub type OR1 = crate::Reg<or1::OR1_SPEC>;
///TIM option register
pub mod or1;
///AF1 register accessor: an alias for `Reg<AF1_SPEC>`
pub type AF1 = crate::Reg<af1::AF1_SPEC>;
///TIM alternate function option register 1
pub mod af1;
///TISEL register accessor: an alias for `Reg<TISEL_SPEC>`
pub type TISEL = crate::Reg<tisel::TISEL_SPEC>;
///TIM alternate function option register 1
pub mod tisel;
