///Register `CCR4` reader
pub struct R(crate::R<CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR4` writer
pub struct W(crate::W<CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR4_SPEC>;
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
impl From<crate::W<CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR4_H` reader - High Capture/Compare value (TIM2 only)
pub struct CCR4_H_R(crate::FieldReader<u16, u16>);
impl CCR4_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR4_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR4_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCR4_H` writer - High Capture/Compare value (TIM2 only)
pub struct CCR4_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
///Field `CCR4_L` reader - Low Capture/Compare value
pub struct CCR4_L_R(crate::FieldReader<u16, u16>);
impl CCR4_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR4_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR4_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCR4_L` writer - Low Capture/Compare value
pub struct CCR4_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_L_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 16:31 - High Capture/Compare value (TIM2 only)
    #[inline(always)]
    pub fn ccr4_h(&self) -> CCR4_H_R {
        CCR4_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    ///Bits 0:15 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr4_l(&self) -> CCR4_L_R {
        CCR4_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - High Capture/Compare value (TIM2 only)
    #[inline(always)]
    pub fn ccr4_h(&mut self) -> CCR4_H_W {
        CCR4_H_W { w: self }
    }
    ///Bits 0:15 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr4_l(&mut self) -> CCR4_L_W {
        CCR4_L_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr4](index.html) module
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr4::R](R) reader structure
impl crate::Readable for CCR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr4::W](W) writer structure
impl crate::Writable for CCR4_SPEC {
    type Writer = W;
}
///`reset()` method sets CCR4 to value 0
impl crate::Resettable for CCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
