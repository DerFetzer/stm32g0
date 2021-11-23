///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FDCAN core release register
    pub crel: crate::Reg<crel::CREL_SPEC>,
    ///0x04 - FDCAN endian register
    pub endn: crate::Reg<endn::ENDN_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x0c - FDCAN data bit timing and prescaler register
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    ///0x10 - FDCAN test register
    pub test: crate::Reg<test::TEST_SPEC>,
    ///0x14 - FDCAN RAM watchdog register
    pub rwd: crate::Reg<rwd::RWD_SPEC>,
    ///0x18 - FDCAN CC control register
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    ///0x1c - FDCAN nominal bit timing and prescaler register
    pub nbtp: crate::Reg<nbtp::NBTP_SPEC>,
    ///0x20 - FDCAN timestamp counter configuration register
    pub tscc: crate::Reg<tscc::TSCC_SPEC>,
    ///0x24 - FDCAN timestamp counter value register
    pub tscv: crate::Reg<tscv::TSCV_SPEC>,
    ///0x28 - FDCAN timeout counter configuration register
    pub tocc: crate::Reg<tocc::TOCC_SPEC>,
    ///0x2c - FDCAN timeout counter value register
    pub tocv: crate::Reg<tocv::TOCV_SPEC>,
    _reserved11: [u8; 0x10],
    ///0x40 - FDCAN error counter register
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    ///0x44 - FDCAN protocol status register
    pub psr: crate::Reg<psr::PSR_SPEC>,
    ///0x48 - FDCAN transmitter delay compensation register
    pub tdcr: crate::Reg<tdcr::TDCR_SPEC>,
    _reserved14: [u8; 0x04],
    ///0x50 - FDCAN interrupt register
    pub ir: crate::Reg<ir::IR_SPEC>,
    ///0x54 - FDCAN interrupt enable register
    pub ie: crate::Reg<ie::IE_SPEC>,
    ///0x58 - FDCAN interrupt line select register
    pub ils: crate::Reg<ils::ILS_SPEC>,
    ///0x5c - FDCAN interrupt line enable register
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved18: [u8; 0x20],
    ///0x80 - FDCAN global filter configuration register
    pub rxgfc: crate::Reg<rxgfc::RXGFC_SPEC>,
    ///0x84 - FDCAN extended ID and mask register
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    ///0x88 - FDCAN high-priority message status register
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    _reserved21: [u8; 0x04],
    ///0x90 - FDCAN Rx FIFO 0 status register
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    ///0x94 - CAN Rx FIFO 0 acknowledge register
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    ///0x98 - FDCAN Rx FIFO 1 status register
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    ///0x9c - FDCAN Rx FIFO 1 acknowledge register
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    _reserved25: [u8; 0x20],
    ///0xc0 - FDCAN Tx buffer configuration register
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    ///0xc4 - FDCAN Tx FIFO/queue status register
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,
    ///0xc8 - FDCAN Tx buffer request pending register
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    ///0xcc - FDCAN Tx buffer add request register
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    ///0xd0 - FDCAN Tx buffer cancellation request register
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    ///0xd4 - FDCAN Tx buffer transmission occurred register
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    ///0xd8 - FDCAN Tx buffer cancellation finished register
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    ///0xdc - FDCAN Tx buffer transmission interrupt enable register
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    ///0xe0 - FDCAN Tx buffer cancellation finished interrupt enable register
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,
    ///0xe4 - FDCAN Tx event FIFO status register
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    ///0xe8 - FDCAN Tx event FIFO acknowledge register
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,
    _reserved36: [u8; 0x14],
    ///0x100 - FDCAN CFG clock divider register
    pub ckdiv: crate::Reg<ckdiv::CKDIV_SPEC>,
}
///CREL register accessor: an alias for `Reg<CREL_SPEC>`
pub type CREL = crate::Reg<crel::CREL_SPEC>;
///FDCAN core release register
pub mod crel;
///ENDN register accessor: an alias for `Reg<ENDN_SPEC>`
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
///FDCAN endian register
pub mod endn;
///DBTP register accessor: an alias for `Reg<DBTP_SPEC>`
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
///FDCAN data bit timing and prescaler register
pub mod dbtp;
///TEST register accessor: an alias for `Reg<TEST_SPEC>`
pub type TEST = crate::Reg<test::TEST_SPEC>;
///FDCAN test register
pub mod test;
///RWD register accessor: an alias for `Reg<RWD_SPEC>`
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
///FDCAN RAM watchdog register
pub mod rwd;
///CCCR register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///FDCAN CC control register
pub mod cccr;
///NBTP register accessor: an alias for `Reg<NBTP_SPEC>`
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
///FDCAN nominal bit timing and prescaler register
pub mod nbtp;
///TSCC register accessor: an alias for `Reg<TSCC_SPEC>`
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
///FDCAN timestamp counter configuration register
pub mod tscc;
///TSCV register accessor: an alias for `Reg<TSCV_SPEC>`
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
///FDCAN timestamp counter value register
pub mod tscv;
///TOCC register accessor: an alias for `Reg<TOCC_SPEC>`
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
///FDCAN timeout counter configuration register
pub mod tocc;
///TOCV register accessor: an alias for `Reg<TOCV_SPEC>`
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
///FDCAN timeout counter value register
pub mod tocv;
///ECR register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///FDCAN error counter register
pub mod ecr;
///PSR register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///FDCAN protocol status register
pub mod psr;
///TDCR register accessor: an alias for `Reg<TDCR_SPEC>`
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
///FDCAN transmitter delay compensation register
pub mod tdcr;
///IR register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///FDCAN interrupt register
pub mod ir;
///IE register accessor: an alias for `Reg<IE_SPEC>`
pub type IE = crate::Reg<ie::IE_SPEC>;
///FDCAN interrupt enable register
pub mod ie;
///ILS register accessor: an alias for `Reg<ILS_SPEC>`
pub type ILS = crate::Reg<ils::ILS_SPEC>;
///FDCAN interrupt line select register
pub mod ils;
///ILE register accessor: an alias for `Reg<ILE_SPEC>`
pub type ILE = crate::Reg<ile::ILE_SPEC>;
///FDCAN interrupt line enable register
pub mod ile;
///RXGFC register accessor: an alias for `Reg<RXGFC_SPEC>`
pub type RXGFC = crate::Reg<rxgfc::RXGFC_SPEC>;
///FDCAN global filter configuration register
pub mod rxgfc;
///XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
///FDCAN extended ID and mask register
pub mod xidam;
///HPMS register accessor: an alias for `Reg<HPMS_SPEC>`
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
///FDCAN high-priority message status register
pub mod hpms;
///RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
///FDCAN Rx FIFO 0 status register
pub mod rxf0s;
///RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
///CAN Rx FIFO 0 acknowledge register
pub mod rxf0a;
///RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
///FDCAN Rx FIFO 1 status register
pub mod rxf1s;
///RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
///FDCAN Rx FIFO 1 acknowledge register
pub mod rxf1a;
///TXBC register accessor: an alias for `Reg<TXBC_SPEC>`
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
///FDCAN Tx buffer configuration register
pub mod txbc;
///TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
///FDCAN Tx FIFO/queue status register
pub mod txfqs;
///TXBRP register accessor: an alias for `Reg<TXBRP_SPEC>`
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
///FDCAN Tx buffer request pending register
pub mod txbrp;
///TXBAR register accessor: an alias for `Reg<TXBAR_SPEC>`
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
///FDCAN Tx buffer add request register
pub mod txbar;
///TXBCR register accessor: an alias for `Reg<TXBCR_SPEC>`
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
///FDCAN Tx buffer cancellation request register
pub mod txbcr;
///TXBTO register accessor: an alias for `Reg<TXBTO_SPEC>`
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
///FDCAN Tx buffer transmission occurred register
pub mod txbto;
///TXBCF register accessor: an alias for `Reg<TXBCF_SPEC>`
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
///FDCAN Tx buffer cancellation finished register
pub mod txbcf;
///TXBTIE register accessor: an alias for `Reg<TXBTIE_SPEC>`
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
///FDCAN Tx buffer transmission interrupt enable register
pub mod txbtie;
///TXBCIE register accessor: an alias for `Reg<TXBCIE_SPEC>`
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
///FDCAN Tx buffer cancellation finished interrupt enable register
pub mod txbcie;
///TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
///FDCAN Tx event FIFO status register
pub mod txefs;
///TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
///FDCAN Tx event FIFO acknowledge register
pub mod txefa;
///CKDIV register accessor: an alias for `Reg<CKDIV_SPEC>`
pub type CKDIV = crate::Reg<ckdiv::CKDIV_SPEC>;
///FDCAN CFG clock divider register
pub mod ckdiv;
