///CR register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DMA channel 1 configuration register
pub mod cr;
///NDTR register accessor: an alias for `Reg<NDTR_SPEC>`
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
///DMA channel 1 number of data tegister
pub mod ndtr;
///PAR register accessor: an alias for `Reg<PAR_SPEC>`
pub type PAR = crate::Reg<par::PAR_SPEC>;
///DMA channel 1 peripheral address
pub mod par;
///MAR register accessor: an alias for `Reg<MAR_SPEC>`
pub type MAR = crate::Reg<mar::MAR_SPEC>;
///DMA channel 1 memory address
pub mod mar;
