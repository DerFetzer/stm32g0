///Register `CVR` reader
pub struct R(crate::R<CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CVR` writer
pub struct W(crate::W<CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVR_SPEC>;
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
impl From<crate::W<CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CURRENT` reader - Current counter value
pub struct CURRENT_R(crate::FieldReader<u32, u32>);
impl CURRENT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CURRENT` writer - Current counter value
pub struct CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CURRENT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&mut self) -> CURRENT_W {
        CURRENT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SysTick current value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cvr](index.html) module
pub struct CVR_SPEC;
impl crate::RegisterSpec for CVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cvr::R](R) reader structure
impl crate::Readable for CVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cvr::W](W) writer structure
impl crate::Writable for CVR_SPEC {
    type Writer = W;
}
///`reset()` method sets CVR to value 0
impl crate::Resettable for CVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
