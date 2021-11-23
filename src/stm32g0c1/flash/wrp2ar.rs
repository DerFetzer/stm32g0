///Register `WRP2AR` reader
pub struct R(crate::R<WRP2AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRP2AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRP2AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRP2AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WRP2AR` writer
pub struct W(crate::W<WRP2AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRP2AR_SPEC>;
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
impl From<crate::W<WRP2AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRP2AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WRP2A_STRT` reader - WRP area A start offset, Bank 2
pub struct WRP2A_STRT_R(crate::FieldReader<u8, u8>);
impl WRP2A_STRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP2A_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP2A_STRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WRP2A_STRT` writer - WRP area A start offset, Bank 2
pub struct WRP2A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2A_STRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
///Field `WRP2A_END` reader - WRP area A end offset, Bank 2
pub struct WRP2A_END_R(crate::FieldReader<u8, u8>);
impl WRP2A_END_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP2A_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP2A_END_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WRP2A_END` writer - WRP area A end offset, Bank 2
pub struct WRP2A_END_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP2A_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:6 - WRP area A start offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_strt(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - WRP area A end offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_end(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - WRP area A start offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_strt(&mut self) -> WRP2A_STRT_W {
        WRP2A_STRT_W { w: self }
    }
    ///Bits 16:22 - WRP area A end offset, Bank 2
    #[inline(always)]
    pub fn wrp2a_end(&mut self) -> WRP2A_END_W {
        WRP2A_END_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash WRP2 area A address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wrp2ar](index.html) module
pub struct WRP2AR_SPEC;
impl crate::RegisterSpec for WRP2AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wrp2ar::R](R) reader structure
impl crate::Readable for WRP2AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wrp2ar::W](W) writer structure
impl crate::Writable for WRP2AR_SPEC {
    type Writer = W;
}
///`reset()` method sets WRP2AR to value 0
impl crate::Resettable for WRP2AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
