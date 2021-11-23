///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFILTDIS_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<RXFILTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RXFILTDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFILTDIS` reader - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler).
pub struct RXFILTDIS_R(crate::FieldReader<bool, RXFILTDIS_A>);
impl RXFILTDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFILTDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFILTDIS_A {
        match self.bits {
            false => RXFILTDIS_A::B_0X0,
            true => RXFILTDIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXFILTDIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXFILTDIS_A::B_0X1
    }
}
impl core::ops::Deref for RXFILTDIS_R {
    type Target = crate::FieldReader<bool, RXFILTDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFILTDIS` writer - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler).
pub struct RXFILTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFILTDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFILTDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXFILTDIS_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXFILTDIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFILT2N3_A {
    ///0: 3 samples
    B_0X0 = 0,
    ///1: 2 samples
    B_0X1 = 1,
}
impl From<RXFILT2N3_A> for bool {
    #[inline(always)]
    fn from(variant: RXFILT2N3_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFILT2N3` reader - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value.
pub struct RXFILT2N3_R(crate::FieldReader<bool, RXFILT2N3_A>);
impl RXFILT2N3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFILT2N3_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFILT2N3_A {
        match self.bits {
            false => RXFILT2N3_A::B_0X0,
            true => RXFILT2N3_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXFILT2N3_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXFILT2N3_A::B_0X1
    }
}
impl core::ops::Deref for RXFILT2N3_R {
    type Target = crate::FieldReader<bool, RXFILT2N3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFILT2N3` writer - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value.
pub struct RXFILT2N3_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFILT2N3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFILT2N3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///3 samples
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXFILT2N3_A::B_0X0)
    }
    ///2 samples
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXFILT2N3_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Force ClkReq clock request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCECLK_A {
    ///0: Do not force clock request
    B_0X0 = 0,
    ///1: Force clock request
    B_0X1 = 1,
}
impl From<FORCECLK_A> for bool {
    #[inline(always)]
    fn from(variant: FORCECLK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FORCECLK` reader - Force ClkReq clock request
pub struct FORCECLK_R(crate::FieldReader<bool, FORCECLK_A>);
impl FORCECLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCECLK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FORCECLK_A {
        match self.bits {
            false => FORCECLK_A::B_0X0,
            true => FORCECLK_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FORCECLK_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FORCECLK_A::B_0X1
    }
}
impl core::ops::Deref for FORCECLK_R {
    type Target = crate::FieldReader<bool, FORCECLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FORCECLK` writer - Force ClkReq clock request
pub struct FORCECLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCECLK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FORCECLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Do not force clock request
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FORCECLK_A::B_0X0)
    }
    ///Force clock request
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FORCECLK_A::B_0X1)
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
///Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPEN` reader - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal.
pub struct WUPEN_R(crate::FieldReader<bool, WUPEN_A>);
impl WUPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUPEN_A {
        match self.bits {
            false => WUPEN_A::B_0X0,
            true => WUPEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUPEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUPEN_A::B_0X1
    }
}
impl core::ops::Deref for WUPEN_R {
    type Target = crate::FieldReader<bool, WUPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUPEN` writer - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal.
pub struct WUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUPEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUPEN_A::B_0X1)
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
impl R {
    ///Bit 0 - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler).
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value.
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Force ClkReq clock request
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal.
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - BMC decoder Rx pre-filter enable The sampling clock is that of the receiver (that is, after pre-scaler).
    #[inline(always)]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W {
        RXFILTDIS_W { w: self }
    }
    ///Bit 1 - BMC decoder Rx pre-filter sampling method Number of consistent consecutive samples before confirming a new value.
    #[inline(always)]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W {
        RXFILT2N3_W { w: self }
    }
    ///Bit 2 - Force ClkReq clock request
    #[inline(always)]
    pub fn forceclk(&mut self) -> FORCECLK_W {
        FORCECLK_W { w: self }
    }
    ///Bit 3 - Wakeup from Stop mode enable Setting the bit enables the UCPD_ASYNC_INT signal.
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W {
        WUPEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
