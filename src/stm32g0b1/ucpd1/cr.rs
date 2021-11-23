///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXMODE_A {
    ///0: Transmission of Tx packet previously defined in other registers
    B_0X0 = 0,
    ///1: Cable Reset sequence
    B_0X1 = 1,
    ///2: BIST test sequence (BIST Carrier Mode 2)
    B_0X2 = 2,
}
impl From<TXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXMODE_A) -> Self {
        variant as _
    }
}
///Field `TXMODE` reader - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
pub struct TXMODE_R(crate::FieldReader<u8, TXMODE_A>);
impl TXMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TXMODE_A> {
        match self.bits {
            0 => Some(TXMODE_A::B_0X0),
            1 => Some(TXMODE_A::B_0X1),
            2 => Some(TXMODE_A::B_0X2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMODE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMODE_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TXMODE_A::B_0X2
    }
}
impl core::ops::Deref for TXMODE_R {
    type Target = crate::FieldReader<u8, TXMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXMODE` writer - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Transmission of Tx packet previously defined in other registers
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXMODE_A::B_0X0)
    }
    ///Cable Reset sequence
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXMODE_A::B_0X1)
    }
    ///BIST test sequence (BIST Carrier Mode 2)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TXMODE_A::B_0X2)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSEND_A {
    ///0: No effect
    B_0X0 = 0,
    ///1: Start Tx packet transmission
    B_0X1 = 1,
}
impl From<TXSEND_A> for bool {
    #[inline(always)]
    fn from(variant: TXSEND_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXSEND` reader - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
pub struct TXSEND_R(crate::FieldReader<bool, TXSEND_A>);
impl TXSEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSEND_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXSEND_A {
        match self.bits {
            false => TXSEND_A::B_0X0,
            true => TXSEND_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXSEND_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXSEND_A::B_0X1
    }
}
impl core::ops::Deref for TXSEND_R {
    type Target = crate::FieldReader<bool, TXSEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXSEND` writer - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
pub struct TXSEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEND_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXSEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXSEND_A::B_0X0)
    }
    ///Start Tx packet transmission
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXSEND_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXHRST_A {
    ///0: No effect
    B_0X0 = 0,
    ///1: Start Tx Hard Reset message
    B_0X1 = 1,
}
impl From<TXHRST_A> for bool {
    #[inline(always)]
    fn from(variant: TXHRST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXHRST` reader - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
pub struct TXHRST_R(crate::FieldReader<bool, TXHRST_A>);
impl TXHRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXHRST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXHRST_A {
        match self.bits {
            false => TXHRST_A::B_0X0,
            true => TXHRST_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXHRST_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXHRST_A::B_0X1
    }
}
impl core::ops::Deref for TXHRST_R {
    type Target = crate::FieldReader<bool, TXHRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXHRST` writer - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
pub struct TXHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXHRST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXHRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXHRST_A::B_0X0)
    }
    ///Start Tx Hard Reset message
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXHRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMODE_A {
    ///0: Normal receive mode
    B_0X0 = 0,
    ///1: BIST receive mode (BIST test data mode)
    B_0X1 = 1,
}
impl From<RXMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RXMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXMODE` reader - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
pub struct RXMODE_R(crate::FieldReader<bool, RXMODE_A>);
impl RXMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXMODE_A {
        match self.bits {
            false => RXMODE_A::B_0X0,
            true => RXMODE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXMODE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXMODE_A::B_0X1
    }
}
impl core::ops::Deref for RXMODE_R {
    type Target = crate::FieldReader<bool, RXMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXMODE` writer - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
pub struct RXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal receive mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXMODE_A::B_0X0)
    }
    ///BIST receive mode (BIST test data mode)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXMODE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYRXEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<PHYRXEN_A> for bool {
    #[inline(always)]
    fn from(variant: PHYRXEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PHYRXEN` reader - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
pub struct PHYRXEN_R(crate::FieldReader<bool, PHYRXEN_A>);
impl PHYRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYRXEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PHYRXEN_A {
        match self.bits {
            false => PHYRXEN_A::B_0X0,
            true => PHYRXEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PHYRXEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PHYRXEN_A::B_0X1
    }
}
impl core::ops::Deref for PHYRXEN_R {
    type Target = crate::FieldReader<bool, PHYRXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHYRXEN` writer - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
pub struct PHYRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYRXEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PHYRXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PHYRXEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PHYRXEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHYCCSEL_A {
    ///0: Use CC1 IO for Power Delivery communication
    B_0X0 = 0,
    ///1: Use CC2 IO for Power Delivery communication
    B_0X1 = 1,
}
impl From<PHYCCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYCCSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PHYCCSEL` reader - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
pub struct PHYCCSEL_R(crate::FieldReader<bool, PHYCCSEL_A>);
impl PHYCCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYCCSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PHYCCSEL_A {
        match self.bits {
            false => PHYCCSEL_A::B_0X0,
            true => PHYCCSEL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PHYCCSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PHYCCSEL_A::B_0X1
    }
}
impl core::ops::Deref for PHYCCSEL_R {
    type Target = crate::FieldReader<bool, PHYCCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PHYCCSEL` writer - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
pub struct PHYCCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYCCSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PHYCCSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Use CC1 IO for Power Delivery communication
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PHYCCSEL_A::B_0X0)
    }
    ///Use CC2 IO for Power Delivery communication
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PHYCCSEL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Field `ANASUBMODE` reader - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
pub struct ANASUBMODE_R(crate::FieldReader<u8, u8>);
impl ANASUBMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANASUBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANASUBMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ANASUBMODE` writer - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
pub struct ANASUBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANASUBMODE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
///Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANAMODE_A {
    ///0: Source
    B_0X0 = 0,
    ///1: Sink
    B_0X1 = 1,
}
impl From<ANAMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ANAMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANAMODE` reader - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
pub struct ANAMODE_R(crate::FieldReader<bool, ANAMODE_A>);
impl ANAMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANAMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANAMODE_A {
        match self.bits {
            false => ANAMODE_A::B_0X0,
            true => ANAMODE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ANAMODE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ANAMODE_A::B_0X1
    }
}
impl core::ops::Deref for ANAMODE_R {
    type Target = crate::FieldReader<bool, ANAMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ANAMODE` writer - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
pub struct ANAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ANAMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Source
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANAMODE_A::B_0X0)
    }
    ///Sink
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANAMODE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
///setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCENABLE_A {
    ///0: Disable both PHYs
    B_0X0 = 0,
    ///1: Enable CC1 PHY
    B_0X1 = 1,
    ///2: Enable CC2 PHY
    B_0X2 = 2,
    ///3: Enable CC1 and CC2 PHY
    B_0X3 = 3,
}
impl From<CCENABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCENABLE_A) -> Self {
        variant as _
    }
}
///Field `CCENABLE` reader - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
///setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.
pub struct CCENABLE_R(crate::FieldReader<u8, CCENABLE_A>);
impl CCENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCENABLE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCENABLE_A {
        match self.bits {
            0 => CCENABLE_A::B_0X0,
            1 => CCENABLE_A::B_0X1,
            2 => CCENABLE_A::B_0X2,
            3 => CCENABLE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCENABLE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCENABLE_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == CCENABLE_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == CCENABLE_A::B_0X3
    }
}
impl core::ops::Deref for CCENABLE_R {
    type Target = crate::FieldReader<u8, CCENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCENABLE` writer - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
///setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.
pub struct CCENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCENABLE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCENABLE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Disable both PHYs
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCENABLE_A::B_0X0)
    }
    ///Enable CC1 PHY
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCENABLE_A::B_0X1)
    }
    ///Enable CC2 PHY
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CCENABLE_A::B_0X2)
    }
    ///Enable CC1 and CC2 PHY
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CCENABLE_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///VCONN switch enable for CC1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1VCONNEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<CC1VCONNEN_A> for bool {
    #[inline(always)]
    fn from(variant: CC1VCONNEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1VCONNEN` reader - VCONN switch enable for CC1
pub struct CC1VCONNEN_R(crate::FieldReader<bool, CC1VCONNEN_A>);
impl CC1VCONNEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1VCONNEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1VCONNEN_A {
        match self.bits {
            false => CC1VCONNEN_A::B_0X0,
            true => CC1VCONNEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1VCONNEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1VCONNEN_A::B_0X1
    }
}
impl core::ops::Deref for CC1VCONNEN_R {
    type Target = crate::FieldReader<bool, CC1VCONNEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1VCONNEN` writer - VCONN switch enable for CC1
pub struct CC1VCONNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1VCONNEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1VCONNEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1VCONNEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1VCONNEN_A::B_0X1)
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
///VCONN switch enable for CC2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2VCONNEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<CC2VCONNEN_A> for bool {
    #[inline(always)]
    fn from(variant: CC2VCONNEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC2VCONNEN` reader - VCONN switch enable for CC2
pub struct CC2VCONNEN_R(crate::FieldReader<bool, CC2VCONNEN_A>);
impl CC2VCONNEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2VCONNEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC2VCONNEN_A {
        match self.bits {
            false => CC2VCONNEN_A::B_0X0,
            true => CC2VCONNEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC2VCONNEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC2VCONNEN_A::B_0X1
    }
}
impl core::ops::Deref for CC2VCONNEN_R {
    type Target = crate::FieldReader<bool, CC2VCONNEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2VCONNEN` writer - VCONN switch enable for CC2
pub struct CC2VCONNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2VCONNEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2VCONNEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC2VCONNEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC2VCONNEN_A::B_0X1)
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
///Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBATTEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<DBATTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBATTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBATTEN` reader - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
pub struct DBATTEN_R(crate::FieldReader<bool, DBATTEN_A>);
impl DBATTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBATTEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBATTEN_A {
        match self.bits {
            false => DBATTEN_A::B_0X0,
            true => DBATTEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBATTEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBATTEN_A::B_0X1
    }
}
impl core::ops::Deref for DBATTEN_R {
    type Target = crate::FieldReader<bool, DBATTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBATTEN` writer - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
pub struct DBATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBATTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBATTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBATTEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBATTEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRSRXEN_A {
    ///1: Enable
    B_0X1 = 1,
}
impl From<FRSRXEN_A> for bool {
    #[inline(always)]
    fn from(variant: FRSRXEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSRXEN` reader - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
pub struct FRSRXEN_R(crate::FieldReader<bool, FRSRXEN_A>);
impl FRSRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSRXEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FRSRXEN_A> {
        match self.bits {
            true => Some(FRSRXEN_A::B_0X1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FRSRXEN_A::B_0X1
    }
}
impl core::ops::Deref for FRSRXEN_R {
    type Target = crate::FieldReader<bool, FRSRXEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FRSRXEN` writer - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
pub struct FRSRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSRXEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FRSRXEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FRSRXEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRSTX_A {
    ///0: No effect
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<FRSTX_A> for bool {
    #[inline(always)]
    fn from(variant: FRSTX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSTX` reader - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
pub struct FRSTX_R(crate::FieldReader<bool, FRSTX_A>);
impl FRSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSTX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRSTX_A {
        match self.bits {
            false => FRSTX_A::B_0X0,
            true => FRSTX_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FRSTX_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FRSTX_A::B_0X1
    }
}
impl core::ops::Deref for FRSTX_R {
    type Target = crate::FieldReader<bool, FRSTX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FRSTX` writer - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
pub struct FRSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FRSTX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FRSTX_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FRSTX_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
///bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCH_A {
    ///0: No effect
    B_0X0 = 0,
    ///1: Rdch condition drive
    B_0X1 = 1,
}
impl From<RDCH_A> for bool {
    #[inline(always)]
    fn from(variant: RDCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RDCH` reader - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
///bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.
pub struct RDCH_R(crate::FieldReader<bool, RDCH_A>);
impl RDCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDCH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDCH_A {
        match self.bits {
            false => RDCH_A::B_0X0,
            true => RDCH_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RDCH_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RDCH_A::B_0X1
    }
}
impl core::ops::Deref for RDCH_R {
    type Target = crate::FieldReader<bool, RDCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RDCH` writer - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
///bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.
pub struct RDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RDCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RDCH_A::B_0X0)
    }
    ///Rdch condition drive
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RDCH_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1TCDIS_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<CC1TCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CC1TCDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1TCDIS` reader - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub struct CC1TCDIS_R(crate::FieldReader<bool, CC1TCDIS_A>);
impl CC1TCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1TCDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1TCDIS_A {
        match self.bits {
            false => CC1TCDIS_A::B_0X0,
            true => CC1TCDIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1TCDIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1TCDIS_A::B_0X1
    }
}
impl core::ops::Deref for CC1TCDIS_R {
    type Target = crate::FieldReader<bool, CC1TCDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1TCDIS` writer - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub struct CC1TCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1TCDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1TCDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1TCDIS_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1TCDIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2TCDIS_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<CC2TCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CC2TCDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC2TCDIS` reader - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub struct CC2TCDIS_R(crate::FieldReader<bool, CC2TCDIS_A>);
impl CC2TCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2TCDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC2TCDIS_A {
        match self.bits {
            false => CC2TCDIS_A::B_0X0,
            true => CC2TCDIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC2TCDIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC2TCDIS_A::B_0X1
    }
}
impl core::ops::Deref for CC2TCDIS_R {
    type Target = crate::FieldReader<bool, CC2TCDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2TCDIS` writer - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
pub struct CC2TCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2TCDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC2TCDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC2TCDIS_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC2TCDIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new((self.bits & 0x03) as u8)
    }
    ///Bit 2 - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
    #[inline(always)]
    pub fn txsend(&self) -> TXSEND_R {
        TXSEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
    #[inline(always)]
    pub fn txhrst(&self) -> TXHRST_R {
        TXHRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
    #[inline(always)]
    pub fn phyrxen(&self) -> PHYRXEN_R {
        PHYRXEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
    #[inline(always)]
    pub fn phyccsel(&self) -> PHYCCSEL_R {
        PHYCCSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 7:8 - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
    #[inline(always)]
    pub fn anasubmode(&self) -> ANASUBMODE_R {
        ANASUBMODE_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    ///Bit 9 - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn anamode(&self) -> ANAMODE_R {
        ANAMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 10:11 - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
    ///setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.
    #[inline(always)]
    pub fn ccenable(&self) -> CCENABLE_R {
        CCENABLE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bit 13 - VCONN switch enable for CC1
    #[inline(always)]
    pub fn cc1vconnen(&self) -> CC1VCONNEN_R {
        CC1VCONNEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - VCONN switch enable for CC2
    #[inline(always)]
    pub fn cc2vconnen(&self) -> CC2VCONNEN_R {
        CC2VCONNEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
    #[inline(always)]
    pub fn dbatten(&self) -> DBATTEN_R {
        DBATTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
    #[inline(always)]
    pub fn frsrxen(&self) -> FRSRXEN_R {
        FRSRXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
    #[inline(always)]
    pub fn frstx(&self) -> FRSTX_R {
        FRSTX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
    ///bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.
    #[inline(always)]
    pub fn rdch(&self) -> RDCH_R {
        RDCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 20 - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn cc1tcdis(&self) -> CC1TCDIS_R {
        CC1TCDIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn cc2tcdis(&self) -> CC2TCDIS_R {
        CC2TCDIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - Type of Tx packet Writing the bitfield triggers the action as follows, depending on the value: Others: invalid From V1.1 of the USB PD specification, there is a counter defined for the duration of the BIST Carrier Mode 2. To quit this mode correctly (after the "tBISTContMode" delay), disable the peripheral (UCPDEN = 0).
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
    ///Bit 2 - Command to send a Tx packet The bit is cleared by hardware as soon as the packet transmission begins or is discarded.
    #[inline(always)]
    pub fn txsend(&mut self) -> TXSEND_W {
        TXSEND_W { w: self }
    }
    ///Bit 3 - Command to send a Tx Hard Reset The bit is cleared by hardware as soon as the message transmission begins or is discarded.
    #[inline(always)]
    pub fn txhrst(&mut self) -> TXHRST_W {
        TXHRST_W { w: self }
    }
    ///Bit 4 - Receiver mode Determines the mode of the receiver. When the bit is set, RXORDSET behaves normally, RXDR no longer receives bytes yet the CRC checking still proceeds as for a normal message.
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W {
        RXMODE_W { w: self }
    }
    ///Bit 5 - USB Power Delivery receiver enable Both CC1 and CC2 receivers are disabled when the bit is cleared. Only the CC receiver selected via the PHYCCSEL bit is enabled when the bit is set.
    #[inline(always)]
    pub fn phyrxen(&mut self) -> PHYRXEN_W {
        PHYRXEN_W { w: self }
    }
    ///Bit 6 - CC1/CC2 line selector for USB Power Delivery signaling The selection depends on the cable orientation as discovered at attach.
    #[inline(always)]
    pub fn phyccsel(&mut self) -> PHYCCSEL_W {
        PHYCCSEL_W { w: self }
    }
    ///Bits 7:8 - Analog PHY sub-mode Refer to TYPEC_VSTATE_CCx for the effect of this bitfield.
    #[inline(always)]
    pub fn anasubmode(&mut self) -> ANASUBMODE_W {
        ANASUBMODE_W { w: self }
    }
    ///Bit 9 - Analog PHY operating mode The bit takes effect upon setting the UCPDx_STROBE bit of the SYS_CONFIG register. The use of CC1 and CC2 depends on CCENABLE. Refer to ANAMODE, ANASUBMODE and link with TYPEC_VSTATE_CCx for the effect of this bitfield in conjunction with ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn anamode(&mut self) -> ANAMODE_W {
        ANAMODE_W { w: self }
    }
    ///Bits 10:11 - CC line enable This bitfield enables CC1 and CC2 line analog PHYs (pull-ups and pull-downs) according to ANAMODE and ANASUBMODE\[1:0\]
    ///setting. A single line PHY can be enabled when, for example, the other line is driven by VCONN via an external VCONN switch. Enabling both PHYs is the normal usage for sink/source.
    #[inline(always)]
    pub fn ccenable(&mut self) -> CCENABLE_W {
        CCENABLE_W { w: self }
    }
    ///Bit 13 - VCONN switch enable for CC1
    #[inline(always)]
    pub fn cc1vconnen(&mut self) -> CC1VCONNEN_W {
        CC1VCONNEN_W { w: self }
    }
    ///Bit 14 - VCONN switch enable for CC2
    #[inline(always)]
    pub fn cc2vconnen(&mut self) -> CC2VCONNEN_W {
        CC2VCONNEN_W { w: self }
    }
    ///Bit 15 - Dead battery function enable The bit takes effect upon setting the USBPDstrobe bit of the SYS_CONFIG register. Dead battery function only operates if the external circuit is appropriately configured.
    #[inline(always)]
    pub fn dbatten(&mut self) -> DBATTEN_W {
        DBATTEN_W { w: self }
    }
    ///Bit 16 - FRS event detection enable Setting the bit enables FRS Rx event (FRSEVT) detection on the CC line selected through the PHYCCSEL bit. 0: Disable Clear the bit when the device is attached to an FRS-incapable source/sink.
    #[inline(always)]
    pub fn frsrxen(&mut self) -> FRSRXEN_W {
        FRSRXEN_W { w: self }
    }
    ///Bit 17 - FRS Tx signaling enable. Setting the bit enables FRS Tx signaling. The bit is cleared by hardware after a delay respecting the USB Power Delivery specification Revision 3.0.
    #[inline(always)]
    pub fn frstx(&mut self) -> FRSTX_W {
        FRSTX_W { w: self }
    }
    ///Bit 18 - Rdch condition drive The bit drives Rdch condition on the CC line selected through the PHYCCSEL bit (thus associated with VCONN), by remaining set during the source-only UnattachedWait.SRC state, to respect the Type-C state. Refer to "USB Type-C ECN for Source VCONN Discharge". The CCENABLE\[1:0\]
    ///bitfield must be set accordingly, too. Changing the bit value only takes effect upon setting the UCPDx_STROBE bit of the SYSCFG_CFGR1 register.
    #[inline(always)]
    pub fn rdch(&mut self) -> RDCH_W {
        RDCH_W { w: self }
    }
    ///Bit 20 - CC1 Type-C detector disable The bit disables the Type-C detector on the CC1 line. When enabled, the Type-C detector for CC1 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn cc1tcdis(&mut self) -> CC1TCDIS_W {
        CC1TCDIS_W { w: self }
    }
    ///Bit 21 - CC2 Type-C detector disable The bit disables the Type-C detector on the CC2 line. When enabled, the Type-C detector for CC2 is configured through ANAMODE and ANASUBMODE\[1:0\].
    #[inline(always)]
    pub fn cc2tcdis(&mut self) -> CC2TCDIS_W {
        CC2TCDIS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
