///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Tamper detection on TAMP_IN1 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1E_A {
    ///0: Tamper detection on TAMP_IN1 is disabled.
    B_0X0 = 0,
    ///1: Tamper detection on TAMP_IN1 is enabled.
    B_0X1 = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable
pub struct TAMP1E_R(crate::FieldReader<bool, TAMP1E_A>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::B_0X0,
            true => TAMP1E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP1E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP1E_A::B_0X1
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, TAMP1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable
pub struct TAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper detection on TAMP_IN1 is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP1E_A::B_0X0)
    }
    ///Tamper detection on TAMP_IN1 is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP1E_A::B_0X1)
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
///Tamper detection on TAMP_IN2 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP2E_A {
    ///0: Tamper detection on TAMP_IN2 is disabled.
    B_0X0 = 0,
    ///1: Tamper detection on TAMP_IN2 is enabled.
    B_0X1 = 1,
}
impl From<TAMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable
pub struct TAMP2E_R(crate::FieldReader<bool, TAMP2E_A>);
impl TAMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP2E_A {
        match self.bits {
            false => TAMP2E_A::B_0X0,
            true => TAMP2E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP2E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP2E_A::B_0X1
    }
}
impl core::ops::Deref for TAMP2E_R {
    type Target = crate::FieldReader<bool, TAMP2E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable
pub struct TAMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper detection on TAMP_IN2 is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP2E_A::B_0X0)
    }
    ///Tamper detection on TAMP_IN2 is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP2E_A::B_0X1)
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
///Internal tamper 3 enable: LSE monitoring
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3E_A {
    ///0: Internal tamper 3 disabled.
    B_0X0 = 0,
    ///1: Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds.
    B_0X1 = 1,
}
impl From<ITAMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3E` reader - Internal tamper 3 enable: LSE monitoring
pub struct ITAMP3E_R(crate::FieldReader<bool, ITAMP3E_A>);
impl ITAMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3E_A {
        match self.bits {
            false => ITAMP3E_A::B_0X0,
            true => ITAMP3E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP3E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP3E_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP3E_R {
    type Target = crate::FieldReader<bool, ITAMP3E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP3E` writer - Internal tamper 3 enable: LSE monitoring
pub struct ITAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP3E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 3 disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP3E_A::B_0X0)
    }
    ///Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP3E_A::B_0X1)
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
///Internal tamper 4 enable: HSE monitoring
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP4E_A {
    ///0: Internal tamper 4 disabled.
    B_0X0 = 0,
    ///1: Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds.
    B_0X1 = 1,
}
impl From<ITAMP4E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP4E` reader - Internal tamper 4 enable: HSE monitoring
pub struct ITAMP4E_R(crate::FieldReader<bool, ITAMP4E_A>);
impl ITAMP4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP4E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP4E_A {
        match self.bits {
            false => ITAMP4E_A::B_0X0,
            true => ITAMP4E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP4E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP4E_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP4E_R {
    type Target = crate::FieldReader<bool, ITAMP4E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP4E` writer - Internal tamper 4 enable: HSE monitoring
pub struct ITAMP4E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP4E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP4E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 4 disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP4E_A::B_0X0)
    }
    ///Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP4E_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Internal tamper 5 enable: RTC calendar overflow
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP5E_A {
    ///0: Internal tamper 5 disabled.
    B_0X0 = 0,
    ///1: Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow.
    B_0X1 = 1,
}
impl From<ITAMP5E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP5E` reader - Internal tamper 5 enable: RTC calendar overflow
pub struct ITAMP5E_R(crate::FieldReader<bool, ITAMP5E_A>);
impl ITAMP5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP5E_A {
        match self.bits {
            false => ITAMP5E_A::B_0X0,
            true => ITAMP5E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP5E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP5E_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP5E_R {
    type Target = crate::FieldReader<bool, ITAMP5E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP5E` writer - Internal tamper 5 enable: RTC calendar overflow
pub struct ITAMP5E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP5E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 5 disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP5E_A::B_0X0)
    }
    ///Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP5E_A::B_0X1)
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
///Internal tamper 6 enable: ST manufacturer readout
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP6E_A {
    ///0: Internal tamper 6 disabled.
    B_0X0 = 0,
    ///1: Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout.
    B_0X1 = 1,
}
impl From<ITAMP6E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP6E` reader - Internal tamper 6 enable: ST manufacturer readout
pub struct ITAMP6E_R(crate::FieldReader<bool, ITAMP6E_A>);
impl ITAMP6E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP6E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP6E_A {
        match self.bits {
            false => ITAMP6E_A::B_0X0,
            true => ITAMP6E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP6E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP6E_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP6E_R {
    type Target = crate::FieldReader<bool, ITAMP6E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP6E` writer - Internal tamper 6 enable: ST manufacturer readout
pub struct ITAMP6E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP6E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP6E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 6 disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP6E_A::B_0X0)
    }
    ///Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP6E_A::B_0X1)
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
    ///Bit 0 - Tamper detection on TAMP_IN1 enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Tamper detection on TAMP_IN2 enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 18 - Internal tamper 3 enable: LSE monitoring
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Internal tamper 4 enable: HSE monitoring
    #[inline(always)]
    pub fn itamp4e(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Internal tamper 5 enable: RTC calendar overflow
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Internal tamper 6 enable: ST manufacturer readout
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper detection on TAMP_IN1 enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    ///Bit 1 - Tamper detection on TAMP_IN2 enable
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W {
        TAMP2E_W { w: self }
    }
    ///Bit 18 - Internal tamper 3 enable: LSE monitoring
    #[inline(always)]
    pub fn itamp3e(&mut self) -> ITAMP3E_W {
        ITAMP3E_W { w: self }
    }
    ///Bit 19 - Internal tamper 4 enable: HSE monitoring
    #[inline(always)]
    pub fn itamp4e(&mut self) -> ITAMP4E_W {
        ITAMP4E_W { w: self }
    }
    ///Bit 20 - Internal tamper 5 enable: RTC calendar overflow
    #[inline(always)]
    pub fn itamp5e(&mut self) -> ITAMP5E_W {
        ITAMP5E_W { w: self }
    }
    ///Bit 21 - Internal tamper 6 enable: ST manufacturer readout
    #[inline(always)]
    pub fn itamp6e(&mut self) -> ITAMP6E_W {
        ITAMP6E_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CR1 to value 0xffff_0000
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
