///Register `PCROP1BER` reader
pub struct R(crate::R<PCROP1BER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1BER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1BER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1BER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCROP1BER` writer
pub struct W(crate::W<PCROP1BER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1BER_SPEC>;
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
impl From<crate::W<PCROP1BER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1BER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP1B_END` reader - PCROP1B area end offset
pub struct PCROP1B_END_R(crate::FieldReader<u16, u16>);
impl PCROP1B_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCROP1B_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1B_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCROP1B_END` writer - PCROP1B area end offset
pub struct PCROP1B_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1B_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    ///Bits 0:8 - PCROP1B area end offset
    #[inline(always)]
    pub fn pcrop1b_end(&self) -> PCROP1B_END_R {
        PCROP1B_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:8 - PCROP1B area end offset
    #[inline(always)]
    pub fn pcrop1b_end(&mut self) -> PCROP1B_END_W {
        PCROP1B_END_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash PCROP area B End address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1ber](index.html) module
pub struct PCROP1BER_SPEC;
impl crate::RegisterSpec for PCROP1BER_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop1ber::R](R) reader structure
impl crate::Readable for PCROP1BER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcrop1ber::W](W) writer structure
impl crate::Writable for PCROP1BER_SPEC {
    type Writer = W;
}
///`reset()` method sets PCROP1BER to value 0xf000_0000
impl crate::Resettable for PCROP1BER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
