///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Tamper 1 interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1IE_A {
    ///0: Tamper 1 interrupt disabled.
    B_0X0 = 0,
    ///1: Tamper 1 interrupt enabled.
    B_0X1 = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1IE` reader - Tamper 1 interrupt enable
pub struct TAMP1IE_R(crate::FieldReader<bool, TAMP1IE_A>);
impl TAMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::B_0X0,
            true => TAMP1IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP1IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP1IE_A::B_0X1
    }
}
impl core::ops::Deref for TAMP1IE_R {
    type Target = crate::FieldReader<bool, TAMP1IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP1IE` writer - Tamper 1 interrupt enable
pub struct TAMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper 1 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP1IE_A::B_0X0)
    }
    ///Tamper 1 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP1IE_A::B_0X1)
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
///Tamper 2 interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP2IE_A {
    ///0: Tamper 2 interrupt disabled.
    B_0X0 = 0,
    ///1: Tamper 2 interrupt enabled.
    B_0X1 = 1,
}
impl From<TAMP2IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP2IE` reader - Tamper 2 interrupt enable
pub struct TAMP2IE_R(crate::FieldReader<bool, TAMP2IE_A>);
impl TAMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP2IE_A {
        match self.bits {
            false => TAMP2IE_A::B_0X0,
            true => TAMP2IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP2IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP2IE_A::B_0X1
    }
}
impl core::ops::Deref for TAMP2IE_R {
    type Target = crate::FieldReader<bool, TAMP2IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP2IE` writer - Tamper 2 interrupt enable
pub struct TAMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper 2 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP2IE_A::B_0X0)
    }
    ///Tamper 2 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP2IE_A::B_0X1)
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
///Internal tamper 3 interrupt enable: LSE monitoring
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP3IE_A {
    ///0: Internal tamper 3 interrupt disabled.
    B_0X0 = 0,
    ///1: Internal tamper 3 interrupt enabled.
    B_0X1 = 1,
}
impl From<ITAMP3IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable: LSE monitoring
pub struct ITAMP3IE_R(crate::FieldReader<bool, ITAMP3IE_A>);
impl ITAMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP3IE_A {
        match self.bits {
            false => ITAMP3IE_A::B_0X0,
            true => ITAMP3IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP3IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP3IE_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP3IE_R {
    type Target = crate::FieldReader<bool, ITAMP3IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable: LSE monitoring
pub struct ITAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP3IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 3 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP3IE_A::B_0X0)
    }
    ///Internal tamper 3 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP3IE_A::B_0X1)
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
///Internal tamper 4 interrupt enable: HSE monitoring
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP4IE_A {
    ///0: Internal tamper 4 interrupt disabled.
    B_0X0 = 0,
    ///1: Internal tamper 4 interrupt enabled.
    B_0X1 = 1,
}
impl From<ITAMP4IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable: HSE monitoring
pub struct ITAMP4IE_R(crate::FieldReader<bool, ITAMP4IE_A>);
impl ITAMP4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP4IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP4IE_A {
        match self.bits {
            false => ITAMP4IE_A::B_0X0,
            true => ITAMP4IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP4IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP4IE_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP4IE_R {
    type Target = crate::FieldReader<bool, ITAMP4IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable: HSE monitoring
pub struct ITAMP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP4IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP4IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 4 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP4IE_A::B_0X0)
    }
    ///Internal tamper 4 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP4IE_A::B_0X1)
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
///Internal tamper 5 interrupt enable: RTC calendar overflow
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP5IE_A {
    ///0: Internal tamper 5 interrupt disabled.
    B_0X0 = 0,
    ///1: Internal tamper 5 interrupt enabled.
    B_0X1 = 1,
}
impl From<ITAMP5IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable: RTC calendar overflow
pub struct ITAMP5IE_R(crate::FieldReader<bool, ITAMP5IE_A>);
impl ITAMP5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP5IE_A {
        match self.bits {
            false => ITAMP5IE_A::B_0X0,
            true => ITAMP5IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP5IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP5IE_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP5IE_R {
    type Target = crate::FieldReader<bool, ITAMP5IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable: RTC calendar overflow
pub struct ITAMP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP5IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 5 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP5IE_A::B_0X0)
    }
    ///Internal tamper 5 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP5IE_A::B_0X1)
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
///Internal tamper 6 interrupt enable: ST manufacturer readout
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITAMP6IE_A {
    ///0: Internal tamper 6 interrupt disabled.
    B_0X0 = 0,
    ///1: Internal tamper 6 interrupt enabled.
    B_0X1 = 1,
}
impl From<ITAMP6IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable: ST manufacturer readout
pub struct ITAMP6IE_R(crate::FieldReader<bool, ITAMP6IE_A>);
impl ITAMP6IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP6IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITAMP6IE_A {
        match self.bits {
            false => ITAMP6IE_A::B_0X0,
            true => ITAMP6IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITAMP6IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITAMP6IE_A::B_0X1
    }
}
impl core::ops::Deref for ITAMP6IE_R {
    type Target = crate::FieldReader<bool, ITAMP6IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable: ST manufacturer readout
pub struct ITAMP6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP6IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITAMP6IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal tamper 6 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITAMP6IE_A::B_0X0)
    }
    ///Internal tamper 6 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITAMP6IE_A::B_0X1)
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
    ///Bit 0 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 18 - Internal tamper 3 interrupt enable: LSE monitoring
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Internal tamper 4 interrupt enable: HSE monitoring
    #[inline(always)]
    pub fn itamp4ie(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Internal tamper 5 interrupt enable: RTC calendar overflow
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Internal tamper 6 interrupt enable: ST manufacturer readout
    #[inline(always)]
    pub fn itamp6ie(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper 1 interrupt enable
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W {
        TAMP1IE_W { w: self }
    }
    ///Bit 1 - Tamper 2 interrupt enable
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W {
        TAMP2IE_W { w: self }
    }
    ///Bit 18 - Internal tamper 3 interrupt enable: LSE monitoring
    #[inline(always)]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W {
        ITAMP3IE_W { w: self }
    }
    ///Bit 19 - Internal tamper 4 interrupt enable: HSE monitoring
    #[inline(always)]
    pub fn itamp4ie(&mut self) -> ITAMP4IE_W {
        ITAMP4IE_W { w: self }
    }
    ///Bit 20 - Internal tamper 5 interrupt enable: RTC calendar overflow
    #[inline(always)]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W {
        ITAMP5IE_W { w: self }
    }
    ///Bit 21 - Internal tamper 6 interrupt enable: ST manufacturer readout
    #[inline(always)]
    pub fn itamp6ie(&mut self) -> ITAMP6IE_W {
        ITAMP6IE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
