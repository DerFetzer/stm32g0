///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADC interrupt and status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x04 - ADC interrupt enable register
    pub ier: crate::Reg<ier::IER_SPEC>,
    ///0x08 - ADC control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    ///0x0c - ADC configuration register 1
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    ///0x10 - ADC configuration register 2
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    ///0x14 - ADC sampling time register
    pub smpr: crate::Reg<smpr::SMPR_SPEC>,
    _reserved6: [u8; 0x08],
    ///0x20 - ADC watchdog threshold register
    pub awd1tr: crate::Reg<awd1tr::AWD1TR_SPEC>,
    ///0x24 - ADC watchdog threshold register
    pub awd2tr: crate::Reg<awd2tr::AWD2TR_SPEC>,
    _reserved_8_chselr: [u8; 0x04],
    ///0x2c - ADC watchdog threshold register
    pub awd3tr: crate::Reg<awd3tr::AWD3TR_SPEC>,
    _reserved10: [u8; 0x10],
    ///0x40 - ADC data register
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved11: [u8; 0x5c],
    ///0xa0 - ADC Analog Watchdog 2 Configuration register
    pub awd2cr: crate::Reg<awd2cr::AWD2CR_SPEC>,
    ///0xa4 - ADC Analog Watchdog 3 Configuration register
    pub awd3cr: crate::Reg<awd3cr::AWD3CR_SPEC>,
    _reserved13: [u8; 0x0c],
    ///0xb4 - ADC Calibration factor
    pub calfact: crate::Reg<calfact::CALFACT_SPEC>,
    _reserved14: [u8; 0x0250],
    ///0x308 - ADC common configuration register
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
}
impl RegisterBlock {
    ///0x28 - channel selection register CHSELRMOD = 1 in ADC_CFGR1
    #[inline(always)]
    pub fn chselr_1(&self) -> &crate::Reg<chselr_1::CHSELR_1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<chselr_1::CHSELR_1_SPEC>)
        }
    }
    ///0x28 - ADC channel selection register \[alternate\]
    #[inline(always)]
    pub fn chselr(&self) -> &crate::Reg<chselr::CHSELR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<chselr::CHSELR_SPEC>)
        }
    }
}
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///ADC interrupt and status register
pub mod isr;
///IER register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///ADC interrupt enable register
pub mod ier;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///ADC control register
pub mod cr;
///CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///ADC configuration register 1
pub mod cfgr1;
///CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///ADC configuration register 2
pub mod cfgr2;
///SMPR register accessor: an alias for `Reg<SMPR_SPEC>`
pub type SMPR = crate::Reg<smpr::SMPR_SPEC>;
///ADC sampling time register
pub mod smpr;
///AWD1TR register accessor: an alias for `Reg<AWD1TR_SPEC>`
pub type AWD1TR = crate::Reg<awd1tr::AWD1TR_SPEC>;
///ADC watchdog threshold register
pub mod awd1tr;
///AWD2TR register accessor: an alias for `Reg<AWD2TR_SPEC>`
pub type AWD2TR = crate::Reg<awd2tr::AWD2TR_SPEC>;
///ADC watchdog threshold register
pub mod awd2tr;
///CHSELR register accessor: an alias for `Reg<CHSELR_SPEC>`
pub type CHSELR = crate::Reg<chselr::CHSELR_SPEC>;
///ADC channel selection register \[alternate\]
pub mod chselr;
///CHSELR_1 register accessor: an alias for `Reg<CHSELR_1_SPEC>`
pub type CHSELR_1 = crate::Reg<chselr_1::CHSELR_1_SPEC>;
///channel selection register CHSELRMOD = 1 in ADC_CFGR1
pub mod chselr_1;
///AWD3TR register accessor: an alias for `Reg<AWD3TR_SPEC>`
pub type AWD3TR = crate::Reg<awd3tr::AWD3TR_SPEC>;
///ADC watchdog threshold register
pub mod awd3tr;
///DR register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///ADC data register
pub mod dr;
///AWD2CR register accessor: an alias for `Reg<AWD2CR_SPEC>`
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
///ADC Analog Watchdog 2 Configuration register
pub mod awd2cr;
///AWD3CR register accessor: an alias for `Reg<AWD3CR_SPEC>`
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
///ADC Analog Watchdog 3 Configuration register
pub mod awd3cr;
///CALFACT register accessor: an alias for `Reg<CALFACT_SPEC>`
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
///ADC Calibration factor
pub mod calfact;
///CCR register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///ADC common configuration register
pub mod ccr;
