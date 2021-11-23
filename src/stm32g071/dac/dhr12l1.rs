///Register `DHR12L1` reader
pub struct R(crate::R<DHR12L1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHR12L1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DHR12L1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DHR12L1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DHR12L1` writer
pub struct W(crate::W<DHR12L1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DHR12L1_SPEC>;
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
impl From<crate::W<DHR12L1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DHR12L1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DACC1DHR` reader - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
pub struct DACC1DHR_R(crate::FieldReader<u16, u16>);
impl DACC1DHR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DACC1DHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACC1DHR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DACC1DHR` writer - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
impl R {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 4:15 - DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC channel1 12-bit left aligned data holding register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dhr12l1](index.html) module
pub struct DHR12L1_SPEC;
impl crate::RegisterSpec for DHR12L1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dhr12l1::R](R) reader structure
impl crate::Readable for DHR12L1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dhr12l1::W](W) writer structure
impl crate::Writable for DHR12L1_SPEC {
    type Writer = W;
}
///`reset()` method sets DHR12L1 to value 0
impl crate::Resettable for DHR12L1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
