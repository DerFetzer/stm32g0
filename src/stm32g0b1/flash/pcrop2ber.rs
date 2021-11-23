///Register `PCROP2BER` reader
pub struct R(crate::R<PCROP2BER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP2BER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP2BER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP2BER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCROP2BER` writer
pub struct W(crate::W<PCROP2BER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP2BER_SPEC>;
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
impl From<crate::W<PCROP2BER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP2BER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP2B_END` reader - PCROP2B area end offset, Bank 2
pub struct PCROP2B_END_R(crate::FieldReader<u16, u16>);
impl PCROP2B_END_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCROP2B_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP2B_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCROP2B_END` writer - PCROP2B area end offset, Bank 2
pub struct PCROP2B_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP2B_END_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    ///Bits 0:8 - PCROP2B area end offset, Bank 2
    #[inline(always)]
    pub fn pcrop2b_end(&self) -> PCROP2B_END_R {
        PCROP2B_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:8 - PCROP2B area end offset, Bank 2
    #[inline(always)]
    pub fn pcrop2b_end(&mut self) -> PCROP2B_END_W {
        PCROP2B_END_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH PCROP2 area B end address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop2ber](index.html) module
pub struct PCROP2BER_SPEC;
impl crate::RegisterSpec for PCROP2BER_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop2ber::R](R) reader structure
impl crate::Readable for PCROP2BER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcrop2ber::W](W) writer structure
impl crate::Writable for PCROP2BER_SPEC {
    type Writer = W;
}
///`reset()` method sets PCROP2BER to value 0
impl crate::Resettable for PCROP2BER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
