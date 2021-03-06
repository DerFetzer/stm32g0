///Register `RX_ORDEXT2` reader
pub struct R(crate::R<RX_ORDEXT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDEXT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDEXT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDEXT2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RX_ORDEXT2` writer
pub struct W(crate::W<RX_ORDEXT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_ORDEXT2_SPEC>;
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
impl From<crate::W<RX_ORDEXT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_ORDEXT2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXSOPX2` reader - RXSOPX2
pub struct RXSOPX2_R(crate::FieldReader<u32, u32>);
impl RXSOPX2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXSOPX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSOPX2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXSOPX2` writer - RXSOPX2
pub struct RXSOPX2_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSOPX2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:19 - RXSOPX2
    #[inline(always)]
    pub fn rxsopx2(&self) -> RXSOPX2_R {
        RXSOPX2_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    ///Bits 0:19 - RXSOPX2
    #[inline(always)]
    pub fn rxsopx2(&mut self) -> RXSOPX2_W {
        RXSOPX2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD Rx Ordered Set Extension Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_ordext2](index.html) module
pub struct RX_ORDEXT2_SPEC;
impl crate::RegisterSpec for RX_ORDEXT2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_ordext2::R](R) reader structure
impl crate::Readable for RX_ORDEXT2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rx_ordext2::W](W) writer structure
impl crate::Writable for RX_ORDEXT2_SPEC {
    type Writer = W;
}
///`reset()` method sets RX_ORDEXT2 to value 0
impl crate::Resettable for RX_ORDEXT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
