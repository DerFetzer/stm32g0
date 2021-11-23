///Register `KEYR1` reader
pub struct R(crate::R<KEYR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `KEYR1` writer
pub struct W(crate::W<KEYR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR1_SPEC>;
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
impl From<crate::W<KEYR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_KEYR1` reader - AES key register (key \[63:32\])
pub struct AES_KEYR1_R(crate::FieldReader<u32, u32>);
impl AES_KEYR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        AES_KEYR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_KEYR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AES_KEYR1` writer - AES key register (key \[63:32\])
pub struct AES_KEYR1_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - AES key register (key \[63:32\])
    #[inline(always)]
    pub fn aes_keyr1(&self) -> AES_KEYR1_R {
        AES_KEYR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - AES key register (key \[63:32\])
    #[inline(always)]
    pub fn aes_keyr1(&mut self) -> AES_KEYR1_W {
        AES_KEYR1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///key register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr1](index.html) module
pub struct KEYR1_SPEC;
impl crate::RegisterSpec for KEYR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [keyr1::R](R) reader structure
impl crate::Readable for KEYR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [keyr1::W](W) writer structure
impl crate::Writable for KEYR1_SPEC {
    type Writer = W;
}
///`reset()` method sets KEYR1 to value 0
impl crate::Resettable for KEYR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
