///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OTRIM1` reader - DAC channel1 offset trimming value
pub struct OTRIM1_R(crate::FieldReader<u8, u8>);
impl OTRIM1_R {
    pub(crate) fn new(bits: u8) -> Self {
        OTRIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTRIM1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OTRIM1` writer - DAC channel1 offset trimming value
pub struct OTRIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> OTRIM1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
///Field `OTRIM2` reader - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation.
pub struct OTRIM2_R(crate::FieldReader<u8, u8>);
impl OTRIM2_R {
    pub(crate) fn new(bits: u8) -> Self {
        OTRIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTRIM2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OTRIM2` writer - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation.
pub struct OTRIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> OTRIM2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:4 - DAC channel1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 16:20 - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - DAC channel1 offset trimming value
    #[inline(always)]
    pub fn otrim1(&mut self) -> OTRIM1_W {
        OTRIM1_W { w: self }
    }
    ///Bits 16:20 - DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn otrim2(&mut self) -> OTRIM2_W {
        OTRIM2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC calibration control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
