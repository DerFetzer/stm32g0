///Register `BKP2R` reader
pub struct R(crate::R<BKP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP2R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BKP2R` writer
pub struct W(crate::W<BKP2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP2R_SPEC>;
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
impl From<crate::W<BKP2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP2R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKP` reader - BKP
pub struct BKP_R(crate::FieldReader<u32, u32>);
impl BKP_R {
    pub(crate) fn new(bits: u32) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKP` writer - BKP
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP backup register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bkp2r](index.html) module
pub struct BKP2R_SPEC;
impl crate::RegisterSpec for BKP2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [bkp2r::R](R) reader structure
impl crate::Readable for BKP2R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bkp2r::W](W) writer structure
impl crate::Writable for BKP2R_SPEC {
    type Writer = W;
}
///`reset()` method sets BKP2R to value 0
impl crate::Resettable for BKP2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
