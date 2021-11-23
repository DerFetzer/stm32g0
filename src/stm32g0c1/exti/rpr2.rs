///Register `RPR2` reader
pub struct R(crate::R<RPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RPR2` writer
pub struct W(crate::W<RPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR2_SPEC>;
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
impl From<crate::W<RPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Rising edge event pending for configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPIF2_A {
    ///0: No trigger request occurred
    NOTPENDING = 0,
    ///1: Selected trigger request occurred
    PENDING = 1,
}
impl From<RPIF2_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF2` reader - Rising edge event pending for configurable line 34
pub struct RPIF2_R(crate::FieldReader<bool, RPIF2_A>);
impl RPIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPIF2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RPIF2_A {
        match self.bits {
            false => RPIF2_A::NOTPENDING,
            true => RPIF2_A::PENDING,
        }
    }
    ///Checks if the value of the field is `NOTPENDING`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == RPIF2_A::NOTPENDING
    }
    ///Checks if the value of the field is `PENDING`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == RPIF2_A::PENDING
    }
}
impl core::ops::Deref for RPIF2_R {
    type Target = crate::FieldReader<bool, RPIF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rising edge event pending for configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPIF2_AW {
    ///1: Clears pending bit
    CLEAR = 1,
}
impl From<RPIF2_AW> for bool {
    #[inline(always)]
    fn from(variant: RPIF2_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RPIF2` writer - Rising edge event pending for configurable line 34
pub struct RPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RPIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF2_AW::CLEAR)
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
    ///Bit 2 - Rising edge event pending for configurable line 34
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Rising edge event pending for configurable line 34
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W {
        RPIF2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI rising edge pending register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rpr2](index.html) module
pub struct RPR2_SPEC;
impl crate::RegisterSpec for RPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rpr2::R](R) reader structure
impl crate::Readable for RPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rpr2::W](W) writer structure
impl crate::Writable for RPR2_SPEC {
    type Writer = W;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
