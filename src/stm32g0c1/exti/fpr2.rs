///Register `FPR2` reader
pub struct R(crate::R<FPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPR2` writer
pub struct W(crate::W<FPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR2_SPEC>;
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
impl From<crate::W<FPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Falling edge event pending for configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPIF2_A {
    ///0: No trigger request occurred
    NOTPENDING = 0,
    ///1: Selected trigger request occurred
    PENDING = 1,
}
impl From<FPIF2_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF2` reader - Falling edge event pending for configurable line 34
pub struct FPIF2_R(crate::FieldReader<bool, FPIF2_A>);
impl FPIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPIF2_A {
        match self.bits {
            false => FPIF2_A::NOTPENDING,
            true => FPIF2_A::PENDING,
        }
    }
    ///Checks if the value of the field is `NOTPENDING`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == FPIF2_A::NOTPENDING
    }
    ///Checks if the value of the field is `PENDING`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == FPIF2_A::PENDING
    }
}
impl core::ops::Deref for FPIF2_R {
    type Target = crate::FieldReader<bool, FPIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Falling edge event pending for configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPIF2_AW {
    ///1: Clears pending bit
    CLEAR = 1,
}
impl From<FPIF2_AW> for bool {
    #[inline(always)]
    fn from(variant: FPIF2_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF2` writer - Falling edge event pending for configurable line 34
pub struct FPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF2_AW::CLEAR)
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
impl R {
    ///Bit 2 - Falling edge event pending for configurable line 34
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Falling edge event pending for configurable line 34
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W {
        FPIF2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling edge pending register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpr2](index.html) module
pub struct FPR2_SPEC;
impl crate::RegisterSpec for FPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpr2::R](R) reader structure
impl crate::Readable for FPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpr2::W](W) writer structure
impl crate::Writable for FPR2_SPEC {
    type Writer = W;
}
///`reset()` method sets FPR2 to value 0
impl crate::Resettable for FPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
