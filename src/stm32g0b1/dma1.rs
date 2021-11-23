///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - low interrupt status register
    pub isr: crate::Reg<isr::ISR_SPEC>,
    ///0x04 - high interrupt status register
    pub ifcr: crate::Reg<ifcr::IFCR_SPEC>,
    ///0x08..0x18 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch1: CH,
    _reserved3: [u8; 0x04],
    ///0x1c..0x2c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch2: CH,
    _reserved4: [u8; 0x04],
    ///0x30..0x40 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch3: CH,
    _reserved5: [u8; 0x04],
    ///0x44..0x54 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch4: CH,
    _reserved6: [u8; 0x04],
    ///0x58..0x68 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch5: CH,
    _reserved7: [u8; 0x04],
    ///0x6c..0x7c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch6: CH,
    _reserved8: [u8; 0x04],
    ///0x80..0x90 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch7: CH,
}
///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - DMA channel 1 configuration register
    pub cr: crate::Reg<self::ch::cr::CR_SPEC>,
    ///0x04 - DMA channel 1 number of data tegister
    pub ndtr: crate::Reg<self::ch::ndtr::NDTR_SPEC>,
    ///0x08 - DMA channel 1 peripheral address
    pub par: crate::Reg<self::ch::par::PAR_SPEC>,
    ///0x0c - DMA channel 1 memory address
    pub mar: crate::Reg<self::ch::mar::MAR_SPEC>,
}
///Register block
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub mod ch;
///ISR register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///low interrupt status register
pub mod isr;
///IFCR register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///high interrupt status register
pub mod ifcr;
