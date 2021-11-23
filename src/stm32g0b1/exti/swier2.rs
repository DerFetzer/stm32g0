///Register `SWIER2` reader
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWIER2` writer
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
///Software rising edge event trigger on line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI2_A {
    ///1: Generates an interrupt request
    PEND = 1,
}
impl From<SWI2_A> for bool {
    #[inline(always)]
    fn from(variant: SWI2_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWI2` reader - Software rising edge event trigger on line 34
pub struct SWI2_R(crate::FieldReader<bool, SWI2_A>);
impl SWI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI2_A> {
        match self.bits {
            true => Some(SWI2_A::PEND),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PEND`
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        **self == SWI2_A::PEND
    }
}
impl core::ops::Deref for SWI2_R {
    type Target = crate::FieldReader<bool, SWI2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SWI2` writer - Software rising edge event trigger on line 34
pub struct SWI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWI2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI2_A::PEND)
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
    ///Bit 2 - Software rising edge event trigger on line 34
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    ///Bit 2 - Software rising edge event trigger on line 34
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W {
        SWI2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI software interrupt event register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier2](index.html) module
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
///`read()` method returns [swier2::R](R) reader structure
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swier2::W](W) writer structure
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
