///Register `PSR` reader
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PSR` writer
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
///Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.
///
///Value on reset: 7
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEC_A {
    ///0: No Error: No error occurred since LEC has been reset by successful reception or transmission.
    B_0X0 = 0,
    ///1: Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed.
    B_0X1 = 1,
    ///2: Form Error: A fixed format part of a received frame has the wrong format.
    B_0X2 = 2,
    ///3: AckError: The message transmitted by the FDCAN was not acknowledged by another node.
    B_0X3 = 3,
    ///4: Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant.
    B_0X4 = 4,
    ///5: Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed).
    B_0X5 = 5,
    ///6: CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data.
    B_0X6 = 6,
    ///7: NoChange: Any read access to the Protocol status register re-initializes the LEC to '7â. When the LEC shows the value '7â, no CAN bus event was detected since the last CPU read access to the Protocol status register.
    B_0X7 = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
///Field `LEC` reader - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.
pub struct LEC_R(crate::FieldReader<u8, LEC_A>);
impl LEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::B_0X0,
            1 => LEC_A::B_0X1,
            2 => LEC_A::B_0X2,
            3 => LEC_A::B_0X3,
            4 => LEC_A::B_0X4,
            5 => LEC_A::B_0X5,
            6 => LEC_A::B_0X6,
            7 => LEC_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LEC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LEC_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == LEC_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == LEC_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == LEC_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == LEC_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == LEC_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == LEC_A::B_0X7
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, LEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LEC` writer - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LEC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No Error: No error occurred since LEC has been reset by successful reception or transmission.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LEC_A::B_0X0)
    }
    ///Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LEC_A::B_0X1)
    }
    ///Form Error: A fixed format part of a received frame has the wrong format.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LEC_A::B_0X2)
    }
    ///AckError: The message transmitted by the FDCAN was not acknowledged by another node.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LEC_A::B_0X3)
    }
    ///Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LEC_A::B_0X4)
    }
    ///Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed).
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(LEC_A::B_0X5)
    }
    ///CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data.
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(LEC_A::B_0X6)
    }
    ///NoChange: Any read access to the Protocol status register re-initializes the LEC to '7â. When the LEC shows the value '7â, no CAN bus event was detected since the last CPU read access to the Protocol status register.
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(LEC_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///Activity Monitors the moduleâs CAN communication state.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACT_A {
    ///0: Synchronizing: node is synchronizing on CAN communication.
    B_0X0 = 0,
    ///1: Idle: node is neither receiver nor transmitter.
    B_0X1 = 1,
    ///2: Receiver: node is operating as receiver.
    B_0X2 = 2,
    ///3: Transmitter: node is operating as transmitter.
    B_0X3 = 3,
}
impl From<ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACT_A) -> Self {
        variant as _
    }
}
///Field `ACT` reader - Activity Monitors the moduleâs CAN communication state.
pub struct ACT_R(crate::FieldReader<u8, ACT_A>);
impl ACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ACT_A {
        match self.bits {
            0 => ACT_A::B_0X0,
            1 => ACT_A::B_0X1,
            2 => ACT_A::B_0X2,
            3 => ACT_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ACT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ACT_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == ACT_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == ACT_A::B_0X3
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<u8, ACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Error passive
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_A {
    ///0: The FDCAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected.
    B_0X0 = 0,
    ///1: The FDCAN is in the Error_Passive state.
    B_0X1 = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EP` reader - Error passive
pub struct EP_R(crate::FieldReader<bool, EP_A>);
impl EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::B_0X0,
            true => EP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EP_A::B_0X1
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, EP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Warning Sstatus
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_A {
    ///0: Both error counters are below the Error_Warning limit of 96.
    B_0X0 = 0,
    ///1: At least one of error counter has reached the Error_Warning limit of 96.
    B_0X1 = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EW` reader - Warning Sstatus
pub struct EW_R(crate::FieldReader<bool, EW_A>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::B_0X0,
            true => EW_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EW_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EW_A::B_0X1
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, EW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Bus_Off status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BO_A {
    ///0: The FDCAN is not Bus_Off.
    B_0X0 = 0,
    ///1: The FDCAN is in Bus_Off state.
    B_0X1 = 1,
}
impl From<BO_A> for bool {
    #[inline(always)]
    fn from(variant: BO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BO` reader - Bus_Off status
pub struct BO_R(crate::FieldReader<bool, BO_A>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BO_A {
        match self.bits {
            false => BO_A::B_0X0,
            true => BO_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BO_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BO_A::B_0X1
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, BO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DLEC` reader - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read.
pub struct DLEC_R(crate::FieldReader<u8, u8>);
impl DLEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DLEC` writer - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read.
pub struct DLEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
///ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESI_A {
    ///0: Last received FDCAN message did not have its ESI flag set.
    B_0X0 = 0,
    ///1: Last received FDCAN message had its ESI flag set.
    B_0X1 = 1,
}
impl From<RESI_A> for bool {
    #[inline(always)]
    fn from(variant: RESI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RESI` reader - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
pub struct RESI_R(crate::FieldReader<bool, RESI_A>);
impl RESI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESI_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RESI_A {
        match self.bits {
            false => RESI_A::B_0X0,
            true => RESI_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RESI_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RESI_A::B_0X1
    }
}
impl core::ops::Deref for RESI_R {
    type Target = crate::FieldReader<bool, RESI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RESI` writer - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
pub struct RESI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RESI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Last received FDCAN message did not have its ESI flag set.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RESI_A::B_0X0)
    }
    ///Last received FDCAN message had its ESI flag set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RESI_A::B_0X1)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRS_A {
    ///0: Last received FDCAN message did not have its BRS flag set.
    B_0X0 = 0,
    ///1: Last received FDCAN message had its BRS flag set.
    B_0X1 = 1,
}
impl From<RBRS_A> for bool {
    #[inline(always)]
    fn from(variant: RBRS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RBRS` reader - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
pub struct RBRS_R(crate::FieldReader<bool, RBRS_A>);
impl RBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBRS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RBRS_A {
        match self.bits {
            false => RBRS_A::B_0X0,
            true => RBRS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RBRS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RBRS_A::B_0X1
    }
}
impl core::ops::Deref for RBRS_R {
    type Target = crate::FieldReader<bool, RBRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RBRS` writer - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
pub struct RBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RBRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Last received FDCAN message did not have its BRS flag set.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RBRS_A::B_0X0)
    }
    ///Last received FDCAN message had its BRS flag set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RBRS_A::B_0X1)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REDL_A {
    ///0: Since this bit was reset by the CPU, no FDCAN message has been received.
    B_0X0 = 0,
    ///1: Message in FDCAN format with EDL flag set has been received.
    B_0X1 = 1,
}
impl From<REDL_A> for bool {
    #[inline(always)]
    fn from(variant: REDL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REDL` reader - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.
pub struct REDL_R(crate::FieldReader<bool, REDL_A>);
impl REDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REDL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REDL_A {
        match self.bits {
            false => REDL_A::B_0X0,
            true => REDL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == REDL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == REDL_A::B_0X1
    }
}
impl core::ops::Deref for REDL_R {
    type Target = crate::FieldReader<bool, REDL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REDL` writer - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.
pub struct REDL_W<'a> {
    w: &'a mut W,
}
impl<'a> REDL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REDL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Since this bit was reset by the CPU, no FDCAN message has been received.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REDL_A::B_0X0)
    }
    ///Message in FDCAN format with EDL flag set has been received.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REDL_A::B_0X1)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Protocol exception event
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PXE_A {
    ///0: No protocol exception event occurred since last read access
    B_0X0 = 0,
    ///1: Protocol exception event occurred
    B_0X1 = 1,
}
impl From<PXE_A> for bool {
    #[inline(always)]
    fn from(variant: PXE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PXE` reader - Protocol exception event
pub struct PXE_R(crate::FieldReader<bool, PXE_A>);
impl PXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PXE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PXE_A {
        match self.bits {
            false => PXE_A::B_0X0,
            true => PXE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PXE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PXE_A::B_0X1
    }
}
impl core::ops::Deref for PXE_R {
    type Target = crate::FieldReader<bool, PXE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PXE` writer - Protocol exception event
pub struct PXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PXE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No protocol exception event occurred since last read access
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PXE_A::B_0X0)
    }
    ///Protocol exception event occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PXE_A::B_0X1)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Field `TDCV` reader - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq.
pub struct TDCV_R(crate::FieldReader<u8, u8>);
impl TDCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - Activity Monitors the moduleâs CAN communication state.
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bit 5 - Error passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Warning Sstatus
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Bus_Off status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read.
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    ///Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Protocol exception event
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bits 16:22 - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq.
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read.
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    ///Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read.
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W {
        DLEC_W { w: self }
    }
    ///Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W {
        RESI_W { w: self }
    }
    ///Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read.
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W {
        RBRS_W { w: self }
    }
    ///Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read.
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W {
        REDL_W { w: self }
    }
    ///Bit 14 - Protocol exception event
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W {
        PXE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN protocol status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psr](index.html) module
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [psr::R](R) reader structure
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [psr::W](W) writer structure
impl crate::Writable for PSR_SPEC {
    type Writer = W;
}
///`reset()` method sets PSR to value 0x0707
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0707
    }
}
