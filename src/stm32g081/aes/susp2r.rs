///Register `SUSP2R` reader
pub struct R(crate::R<SUSP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP2R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SUSP2R` writer
pub struct W(crate::W<SUSP2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP2R_SPEC>;
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
impl From<crate::W<SUSP2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP2R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_SUSP2R` reader - AES suspend register 2
pub struct AES_SUSP2R_R(crate::FieldReader<u32, u32>);
impl AES_SUSP2R_R {
    pub(crate) fn new(bits: u32) -> Self {
        AES_SUSP2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_SUSP2R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AES_SUSP2R` writer - AES suspend register 2
pub struct AES_SUSP2R_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_SUSP2R_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - AES suspend register 2
    #[inline(always)]
    pub fn aes_susp2r(&self) -> AES_SUSP2R_R {
        AES_SUSP2R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 2
    #[inline(always)]
    pub fn aes_susp2r(&mut self) -> AES_SUSP2R_W {
        AES_SUSP2R_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AES suspend register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [susp2r](index.html) module
pub struct SUSP2R_SPEC;
impl crate::RegisterSpec for SUSP2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [susp2r::R](R) reader structure
impl crate::Readable for SUSP2R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [susp2r::W](W) writer structure
impl crate::Writable for SUSP2R_SPEC {
    type Writer = W;
}
///`reset()` method sets SUSP2R to value 0
impl crate::Resettable for SUSP2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
