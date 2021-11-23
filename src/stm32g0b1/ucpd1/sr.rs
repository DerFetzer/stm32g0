///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Transmit interrupt status The flag indicates that the UCPD_TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the UCPD_TXDR register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIS_A {
    ///0: New Tx data write not required
    B_0X0 = 0,
    ///1: New Tx data write required
    B_0X1 = 1,
}
impl From<TXIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIS` reader - Transmit interrupt status The flag indicates that the UCPD_TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the UCPD_TXDR register.
pub struct TXIS_R(crate::FieldReader<bool, TXIS_A>);
impl TXIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXIS_A {
        match self.bits {
            false => TXIS_A::B_0X0,
            true => TXIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXIS_A::B_0X1
    }
}
impl core::ops::Deref for TXIS_R {
    type Target = crate::FieldReader<bool, TXIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSGDISC_A {
    ///0: No Tx message discarded
    B_0X0 = 0,
    ///1: Tx message discarded
    B_0X1 = 1,
}
impl From<TXMSGDISC_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSGDISC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGDISC` reader - Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle.
pub struct TXMSGDISC_R(crate::FieldReader<bool, TXMSGDISC_A>);
impl TXMSGDISC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGDISC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXMSGDISC_A {
        match self.bits {
            false => TXMSGDISC_A::B_0X0,
            true => TXMSGDISC_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMSGDISC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMSGDISC_A::B_0X1
    }
}
impl core::ops::Deref for TXMSGDISC_R {
    type Target = crate::FieldReader<bool, TXMSGDISC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSGSENT_A {
    ///0: No Tx message completed
    B_0X0 = 0,
    ///1: Tx message completed
    B_0X1 = 1,
}
impl From<TXMSGSENT_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSGSENT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGSENT` reader - Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised.
pub struct TXMSGSENT_R(crate::FieldReader<bool, TXMSGSENT_A>);
impl TXMSGSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGSENT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXMSGSENT_A {
        match self.bits {
            false => TXMSGSENT_A::B_0X0,
            true => TXMSGSENT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMSGSENT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMSGSENT_A::B_0X1
    }
}
impl core::ops::Deref for TXMSGSENT_R {
    type Target = crate::FieldReader<bool, TXMSGSENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSGABT_A {
    ///0: No transmit message abort
    B_0X0 = 0,
    ///1: Transmit message abort
    B_0X1 = 1,
}
impl From<TXMSGABT_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSGABT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGABT` reader - Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit.
pub struct TXMSGABT_R(crate::FieldReader<bool, TXMSGABT_A>);
impl TXMSGABT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGABT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXMSGABT_A {
        match self.bits {
            false => TXMSGABT_A::B_0X0,
            true => TXMSGABT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMSGABT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMSGABT_A::B_0X1
    }
}
impl core::ops::Deref for TXMSGABT_R {
    type Target = crate::FieldReader<bool, TXMSGABT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSTDISC_A {
    ///0: No Hard Reset discarded
    B_0X0 = 0,
    ///1: Hard Reset discarded
    B_0X1 = 1,
}
impl From<HRSTDISC_A> for bool {
    #[inline(always)]
    fn from(variant: HRSTDISC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HRSTDISC` reader - Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit.
pub struct HRSTDISC_R(crate::FieldReader<bool, HRSTDISC_A>);
impl HRSTDISC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTDISC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HRSTDISC_A {
        match self.bits {
            false => HRSTDISC_A::B_0X0,
            true => HRSTDISC_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HRSTDISC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HRSTDISC_A::B_0X1
    }
}
impl core::ops::Deref for HRSTDISC_R {
    type Target = crate::FieldReader<bool, HRSTDISC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSTSENT_A {
    ///0: No Hard Reset message sent
    B_0X0 = 0,
    ///1: Hard Reset message sent
    B_0X1 = 1,
}
impl From<HRSTSENT_A> for bool {
    #[inline(always)]
    fn from(variant: HRSTSENT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HRSTSENT` reader - Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit.
pub struct HRSTSENT_R(crate::FieldReader<bool, HRSTSENT_A>);
impl HRSTSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTSENT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HRSTSENT_A {
        match self.bits {
            false => HRSTSENT_A::B_0X0,
            true => HRSTSENT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HRSTSENT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HRSTSENT_A::B_0X1
    }
}
impl core::ops::Deref for HRSTSENT_R {
    type Target = crate::FieldReader<bool, HRSTSENT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Tx data underrun detection The flag indicates that the Tx data register (UCPD_TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUND_A {
    ///0: No Tx data underrun detected
    B_0X0 = 0,
    ///1: Tx data underrun detected
    B_0X1 = 1,
}
impl From<TXUND_A> for bool {
    #[inline(always)]
    fn from(variant: TXUND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXUND` reader - Tx data underrun detection The flag indicates that the Tx data register (UCPD_TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit.
pub struct TXUND_R(crate::FieldReader<bool, TXUND_A>);
impl TXUND_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUND_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXUND_A {
        match self.bits {
            false => TXUND_A::B_0X0,
            true => TXUND_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXUND_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXUND_A::B_0X1
    }
}
impl core::ops::Deref for TXUND_R {
    type Target = crate::FieldReader<bool, TXUND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Receive data register not empty detection The flag indicates that the UCPD_RXDR register is not empty. It is automatically cleared upon reading UCPD_RXDR.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    ///0: Rx data register empty
    B_0X0 = 0,
    ///1: Rx data register not empty
    B_0X1 = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Receive data register not empty detection The flag indicates that the UCPD_RXDR register is not empty. It is automatically cleared upon reading UCPD_RXDR.
pub struct RXNE_R(crate::FieldReader<bool, RXNE_A>);
impl RXNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::B_0X0,
            true => RXNE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXNE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXNE_A::B_0X1
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, RXNE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\[2:0\]
///bitfield of the UCPD_RX_ORDSET register. It is cleared by setting the RXORDDETCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXORDDET_A {
    ///0: No ordered set detected
    B_0X0 = 0,
    ///1: A new ordered set detected
    B_0X1 = 1,
}
impl From<RXORDDET_A> for bool {
    #[inline(always)]
    fn from(variant: RXORDDET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXORDDET` reader - Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\[2:0\]
///bitfield of the UCPD_RX_ORDSET register. It is cleared by setting the RXORDDETCF bit.
pub struct RXORDDET_R(crate::FieldReader<bool, RXORDDET_A>);
impl RXORDDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXORDDET_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXORDDET_A {
        match self.bits {
            false => RXORDDET_A::B_0X0,
            true => RXORDDET_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXORDDET_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXORDDET_A::B_0X1
    }
}
impl core::ops::Deref for RXORDDET_R {
    type Target = crate::FieldReader<bool, RXORDDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXHRSTDET_A {
    ///0: Hard Reset not received
    B_0X0 = 0,
    ///1: Hard Reset received
    B_0X1 = 1,
}
impl From<RXHRSTDET_A> for bool {
    #[inline(always)]
    fn from(variant: RXHRSTDET_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXHRSTDET` reader - Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit.
pub struct RXHRSTDET_R(crate::FieldReader<bool, RXHRSTDET_A>);
impl RXHRSTDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXHRSTDET_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXHRSTDET_A {
        match self.bits {
            false => RXHRSTDET_A::B_0X0,
            true => RXHRSTDET_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXHRSTDET_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXHRSTDET_A::B_0X1
    }
}
impl core::ops::Deref for RXHRSTDET_R {
    type Target = crate::FieldReader<bool, RXHRSTDET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVR_A {
    ///0: No overflow
    B_0X0 = 0,
    ///1: Overflow
    B_0X1 = 1,
}
impl From<RXOVR_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXOVR` reader - Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough.
pub struct RXOVR_R(crate::FieldReader<bool, RXOVR_A>);
impl RXOVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXOVR_A {
        match self.bits {
            false => RXOVR_A::B_0X0,
            true => RXOVR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXOVR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXOVR_A::B_0X1
    }
}
impl core::ops::Deref for RXOVR_R {
    type Target = crate::FieldReader<bool, RXOVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSGEND_A {
    ///0: No new Rx message received
    B_0X0 = 0,
    ///1: A new Rx message received
    B_0X1 = 1,
}
impl From<RXMSGEND_A> for bool {
    #[inline(always)]
    fn from(variant: RXMSGEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXMSGEND` reader - Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message.
pub struct RXMSGEND_R(crate::FieldReader<bool, RXMSGEND_A>);
impl RXMSGEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMSGEND_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXMSGEND_A {
        match self.bits {
            false => RXMSGEND_A::B_0X0,
            true => RXMSGEND_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXMSGEND_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXMSGEND_A::B_0X1
    }
}
impl core::ops::Deref for RXMSGEND_R {
    type Target = crate::FieldReader<bool, RXMSGEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERR_A {
    ///0: No error detected
    B_0X0 = 0,
    ///1: Error(s) detected
    B_0X1 = 1,
}
impl From<RXERR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXERR` reader - Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set.
pub struct RXERR_R(crate::FieldReader<bool, RXERR_A>);
impl RXERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXERR_A {
        match self.bits {
            false => RXERR_A::B_0X0,
            true => RXERR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXERR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXERR_A::B_0X1
    }
}
impl core::ops::Deref for RXERR_R {
    type Target = crate::FieldReader<bool, RXERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\[1:0\]
///bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPECEVT1_A {
    ///0: No new event
    B_0X0 = 0,
    ///1: A new Type-C event
    B_0X1 = 1,
}
impl From<TYPECEVT1_A> for bool {
    #[inline(always)]
    fn from(variant: TYPECEVT1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TYPECEVT1` reader - Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\[1:0\]
///bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
pub struct TYPECEVT1_R(crate::FieldReader<bool, TYPECEVT1_A>);
impl TYPECEVT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TYPECEVT1_A {
        match self.bits {
            false => TYPECEVT1_A::B_0X0,
            true => TYPECEVT1_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TYPECEVT1_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TYPECEVT1_A::B_0X1
    }
}
impl core::ops::Deref for TYPECEVT1_R {
    type Target = crate::FieldReader<bool, TYPECEVT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\[1:0\]
///bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPECEVT2_A {
    ///0: No new event
    B_0X0 = 0,
    ///1: A new Type-C event
    B_0X1 = 1,
}
impl From<TYPECEVT2_A> for bool {
    #[inline(always)]
    fn from(variant: TYPECEVT2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TYPECEVT2` reader - Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\[1:0\]
///bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
pub struct TYPECEVT2_R(crate::FieldReader<bool, TYPECEVT2_A>);
impl TYPECEVT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TYPECEVT2_A {
        match self.bits {
            false => TYPECEVT2_A::B_0X0,
            true => TYPECEVT2_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TYPECEVT2_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TYPECEVT2_A::B_0X1
    }
}
impl core::ops::Deref for TYPECEVT2_R {
    type Target = crate::FieldReader<bool, TYPECEVT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPEC_VSTATE_CC1_A {
    ///0: Lowest
    B_0X0 = 0,
    ///1: Low
    B_0X1 = 1,
    ///2: High
    B_0X2 = 2,
    ///3: Highest
    B_0X3 = 3,
}
impl From<TYPEC_VSTATE_CC1_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEC_VSTATE_CC1_A) -> Self {
        variant as _
    }
}
///Field `TYPEC_VSTATE_CC1` reader - The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.
pub struct TYPEC_VSTATE_CC1_R(crate::FieldReader<u8, TYPEC_VSTATE_CC1_A>);
impl TYPEC_VSTATE_CC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPEC_VSTATE_CC1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TYPEC_VSTATE_CC1_A {
        match self.bits {
            0 => TYPEC_VSTATE_CC1_A::B_0X0,
            1 => TYPEC_VSTATE_CC1_A::B_0X1,
            2 => TYPEC_VSTATE_CC1_A::B_0X2,
            3 => TYPEC_VSTATE_CC1_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TYPEC_VSTATE_CC1_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TYPEC_VSTATE_CC1_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TYPEC_VSTATE_CC1_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TYPEC_VSTATE_CC1_A::B_0X3
    }
}
impl core::ops::Deref for TYPEC_VSTATE_CC1_R {
    type Target = crate::FieldReader<u8, TYPEC_VSTATE_CC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPEC_VSTATE_CC2_A {
    ///0: Lowest
    B_0X0 = 0,
    ///1: Low
    B_0X1 = 1,
    ///2: High
    B_0X2 = 2,
    ///3: Highest
    B_0X3 = 3,
}
impl From<TYPEC_VSTATE_CC2_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPEC_VSTATE_CC2_A) -> Self {
        variant as _
    }
}
///Field `TYPEC_VSTATE_CC2` reader - CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.
pub struct TYPEC_VSTATE_CC2_R(crate::FieldReader<u8, TYPEC_VSTATE_CC2_A>);
impl TYPEC_VSTATE_CC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TYPEC_VSTATE_CC2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TYPEC_VSTATE_CC2_A {
        match self.bits {
            0 => TYPEC_VSTATE_CC2_A::B_0X0,
            1 => TYPEC_VSTATE_CC2_A::B_0X1,
            2 => TYPEC_VSTATE_CC2_A::B_0X2,
            3 => TYPEC_VSTATE_CC2_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TYPEC_VSTATE_CC2_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TYPEC_VSTATE_CC2_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TYPEC_VSTATE_CC2_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TYPEC_VSTATE_CC2_A::B_0X3
    }
}
impl core::ops::Deref for TYPEC_VSTATE_CC2_R {
    type Target = crate::FieldReader<u8, TYPEC_VSTATE_CC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///FRS detection event The flag is cleared by setting the FRSEVTCF bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRSEVT_A {
    ///0: No new event
    B_0X0 = 0,
    ///1: New FRS receive event occurred
    B_0X1 = 1,
}
impl From<FRSEVT_A> for bool {
    #[inline(always)]
    fn from(variant: FRSEVT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSEVT` reader - FRS detection event The flag is cleared by setting the FRSEVTCF bit.
pub struct FRSEVT_R(crate::FieldReader<bool, FRSEVT_A>);
impl FRSEVT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSEVT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRSEVT_A {
        match self.bits {
            false => FRSEVT_A::B_0X0,
            true => FRSEVT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FRSEVT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FRSEVT_A::B_0X1
    }
}
impl core::ops::Deref for FRSEVT_R {
    type Target = crate::FieldReader<bool, FRSEVT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Transmit interrupt status The flag indicates that the UCPD_TXDR register is empty and new data write is required (as the amount of data sent has not reached the payload size defined in the TXPAYSZ bitfield). The flag is cleared with the data write into the UCPD_TXDR register.
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Message transmission discarded The flag indicates that a message transmission was dropped. The flag is cleared by setting the TXMSGDISCCF bit. Transmission of a message can be dropped if there is a concurrent receive in progress or at excessive noise on the line. After a Tx message is discarded, the flag is only raised when the CC line becomes idle.
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Message transmission completed The flag indicates the completion of packet transmission. It is cleared by setting the TXMSGSENTCF bit. In the event of a message transmission interrupted by a Hard Reset, the flag is not raised.
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Transmit message abort The flag indicates that a Tx message is aborted due to a subsequent Hard Reset message send request taking priority during transmit. It is cleared by setting the TXMSGABTCF bit.
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Hard Reset discarded The flag indicates that the Hard Reset message is discarded. The flag is cleared by setting the HRSTDISCCF bit.
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Hard Reset message sent The flag indicates that the Hard Reset message is sent. The flag is cleared by setting the HRSTSENTCF bit.
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Tx data underrun detection The flag indicates that the Tx data register (UCPD_TXDR) was not written in time for a transmit message to execute normally. It is cleared by setting the TXUNDCF bit.
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - Receive data register not empty detection The flag indicates that the UCPD_RXDR register is not empty. It is automatically cleared upon reading UCPD_RXDR.
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Rx ordered set (4 K-codes) detection The flag indicates the detection of an ordered set. The relevant information is stored in the RXORDSET\[2:0\]
    ///bitfield of the UCPD_RX_ORDSET register. It is cleared by setting the RXORDDETCF bit.
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Rx Hard Reset receipt detection The flag indicates the receipt of valid Hard Reset message. It is cleared by setting the RXHRSTDETCF bit.
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Rx data overflow detection The flag indicates Rx data buffer overflow. It is cleared by setting the RXOVRCF bit. The buffer overflow can occur if the received data are not read fast enough.
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Rx message received The flag indicates whether a message (except Hard Reset message) has been received, regardless the CRC value. The flag is cleared by setting the RXMSGENDCF bit. The RXERR flag set when the RXMSGEND flag goes high indicates errors in the last-received message.
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Receive message error The flag indicates errors of the last Rx message declared (via RXMSGEND), such as incorrect CRC or truncated message (a line becoming static before EOP is met). It is asserted whenever the RXMSGEND flag is set.
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Type-C voltage level event on CC1 line The flag indicates a change of the TYPEC_VSTATE_CC1\[1:0\]
    ///bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Type-C voltage level event on CC2 line The flag indicates a change of the TYPEC_VSTATE_CC2\[1:0\]
    ///bitfield value, which corresponds to a new Type-C event. It is cleared by setting the TYPECEVT2CF bit.
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:17 - The status bitfield indicates the voltage level on the CC1 line in its steady state. The voltage variation on the CC1 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 18:19 - CC2 line voltage level The status bitfield indicates the voltage level on the CC2 line in its steady state. The voltage variation on the CC2 line during USB PD messages due to the BMC PHY modulation does not impact the bitfield value.
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bit 20 - FRS detection event The flag is cleared by setting the FRSEVTCF bit.
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
///UCPD status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
