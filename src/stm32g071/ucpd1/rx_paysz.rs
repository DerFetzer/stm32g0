///Register `RX_PAYSZ` reader
pub struct R(crate::R<RX_PAYSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PAYSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PAYSZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PAYSZ_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RX_PAYSZ` writer
pub struct W(crate::W<RX_PAYSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_PAYSZ_SPEC>;
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
impl From<crate::W<RX_PAYSZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_PAYSZ_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXPAYSZ` reader - RXPAYSZ
pub struct RXPAYSZ_R(crate::FieldReader<u16, u16>);
impl RXPAYSZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXPAYSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPAYSZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXPAYSZ` writer - RXPAYSZ
pub struct RXPAYSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPAYSZ_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&mut self) -> RXPAYSZ_W {
        RXPAYSZ_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD Rx Paysize Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_paysz](index.html) module
pub struct RX_PAYSZ_SPEC;
impl crate::RegisterSpec for RX_PAYSZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_paysz::R](R) reader structure
impl crate::Readable for RX_PAYSZ_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rx_paysz::W](W) writer structure
impl crate::Writable for RX_PAYSZ_SPEC {
    type Writer = W;
}
///`reset()` method sets RX_PAYSZ to value 0
impl crate::Resettable for RX_PAYSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
