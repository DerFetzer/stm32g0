///Register `RVR` reader
pub struct R(crate::R<RVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RVR` writer
pub struct W(crate::W<RVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RVR_SPEC>;
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
impl From<crate::W<RVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RELOAD` reader - RELOAD value
pub struct RELOAD_R(crate::FieldReader<u32, u32>);
impl RELOAD_R {
    pub(crate) fn new(bits: u32) -> Self {
        RELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RELOAD` writer - RELOAD value
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:23 - RELOAD value
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SysTick reload value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rvr](index.html) module
pub struct RVR_SPEC;
impl crate::RegisterSpec for RVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rvr::R](R) reader structure
impl crate::Readable for RVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rvr::W](W) writer structure
impl crate::Writable for RVR_SPEC {
    type Writer = W;
}
///`reset()` method sets RVR to value 0
impl crate::Resettable for RVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
