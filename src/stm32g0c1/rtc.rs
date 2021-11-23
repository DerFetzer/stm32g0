///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RTC time register
    pub tr: crate::Reg<tr::TR_SPEC>,
    ///0x04 - RTC date register
    pub dr: crate::Reg<dr::DR_SPEC>,
    ///0x08 - RTC sub second register
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    ///0x0c - RTC initialization control and status register
    pub icsr: crate::Reg<icsr::ICSR_SPEC>,
    ///0x10 - RTC prescaler register
    pub prer: crate::Reg<prer::PRER_SPEC>,
    ///0x14 - RTC wakeup timer register
    pub wutr: crate::Reg<wutr::WUTR_SPEC>,
    ///0x18 - RTC control register
    pub cr: crate::Reg<cr::CR_SPEC>,
    _reserved7: [u8; 0x08],
    ///0x24 - RTC write protection register
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    ///0x28 - RTC calibration register
    pub calr: crate::Reg<calr::CALR_SPEC>,
    ///0x2c - RTC shift control register
    pub shiftr: crate::Reg<shiftr::SHIFTR_SPEC>,
    ///0x30 - RTC timestamp time register
    pub tstr: crate::Reg<tstr::TSTR_SPEC>,
    ///0x34 - RTC timestamp date register
    pub tsdr: crate::Reg<tsdr::TSDR_SPEC>,
    ///0x38 - RTC timestamp sub second register
    pub tsssr: crate::Reg<tsssr::TSSSR_SPEC>,
    _reserved13: [u8; 0x04],
    ///0x40 - RTC alarm A register
    pub alrmar: crate::Reg<alrmar::ALRMAR_SPEC>,
    ///0x44 - RTC alarm A sub second register
    pub alrmassr: crate::Reg<alrmassr::ALRMASSR_SPEC>,
    ///0x48 - RTC alarm B register
    pub alrmbr: crate::Reg<alrmbr::ALRMBR_SPEC>,
    ///0x4c - RTC alarm B sub second register
    pub alrmbssr: crate::Reg<alrmbssr::ALRMBSSR_SPEC>,
    ///0x50 - RTC status register
    pub sr: crate::Reg<sr::SR_SPEC>,
    ///0x54 - RTC masked interrupt status register
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved19: [u8; 0x04],
    ///0x5c - RTC status clear register
    pub scr: crate::Reg<scr::SCR_SPEC>,
}
///TR register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///RTC time register
pub mod tr;
///DR register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///RTC date register
pub mod dr;
///SSR register accessor: an alias for `Reg<SSR_SPEC>`
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///RTC sub second register
pub mod ssr;
///ICSR register accessor: an alias for `Reg<ICSR_SPEC>`
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
///RTC initialization control and status register
pub mod icsr;
///PRER register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///RTC prescaler register
pub mod prer;
///WUTR register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///RTC wakeup timer register
pub mod wutr;
///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///RTC control register
pub mod cr;
///WPR register accessor: an alias for `Reg<WPR_SPEC>`
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
///RTC write protection register
pub mod wpr;
///CALR register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///RTC calibration register
pub mod calr;
///SHIFTR register accessor: an alias for `Reg<SHIFTR_SPEC>`
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
///RTC shift control register
pub mod shiftr;
///TSTR register accessor: an alias for `Reg<TSTR_SPEC>`
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
///RTC timestamp time register
pub mod tstr;
///TSDR register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///RTC timestamp date register
pub mod tsdr;
///TSSSR register accessor: an alias for `Reg<TSSSR_SPEC>`
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
///RTC timestamp sub second register
pub mod tsssr;
///ALRMAR register accessor: an alias for `Reg<ALRMAR_SPEC>`
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
///RTC alarm A register
pub mod alrmar;
///ALRMASSR register accessor: an alias for `Reg<ALRMASSR_SPEC>`
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
///RTC alarm A sub second register
pub mod alrmassr;
///ALRMBR register accessor: an alias for `Reg<ALRMBR_SPEC>`
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
///RTC alarm B register
pub mod alrmbr;
///ALRMBSSR register accessor: an alias for `Reg<ALRMBSSR_SPEC>`
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
///RTC alarm B sub second register
pub mod alrmbssr;
///SR register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///RTC status register
pub mod sr;
///MISR register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///RTC masked interrupt status register
pub mod misr;
///SCR register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///RTC status clear register
pub mod scr;
