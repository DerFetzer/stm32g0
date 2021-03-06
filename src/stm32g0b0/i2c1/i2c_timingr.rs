///Register `I2C_TIMINGR` reader
pub struct R(crate::R<I2C_TIMINGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TIMINGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TIMINGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TIMINGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2C_TIMINGR` writer
pub struct W(crate::W<I2C_TIMINGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TIMINGR_SPEC>;
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
impl From<crate::W<I2C_TIMINGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TIMINGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SCLL` reader - SCL low period (master mode)
pub struct SCLL_R(crate::FieldReader<u8, u8>);
impl SCLL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCLL` writer - SCL low period (master mode)
pub struct SCLL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `SCLH` reader - SCL high period (master mode)
pub struct SCLH_R(crate::FieldReader<u8, u8>);
impl SCLH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCLH` writer - SCL high period (master mode)
pub struct SCLH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLH_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
///Field `SDADEL` reader - Data hold time
pub struct SDADEL_R(crate::FieldReader<u8, u8>);
impl SDADEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDADEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDADEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SDADEL` writer - Data hold time
pub struct SDADEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDADEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Field `SCLDEL` reader - Data setup time
pub struct SCLDEL_R(crate::FieldReader<u8, u8>);
impl SCLDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SCLDEL` writer - Data setup time
pub struct SCLDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLDEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
///Field `PRESC` reader - Timing prescaler
pub struct PRESC_R(crate::FieldReader<u8, u8>);
impl PRESC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PRESC` writer - Timing prescaler
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    ///Bits 0:7 - SCL low period (master mode)
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCL high period (master mode)
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Data hold time
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Data setup time
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - Timing prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:7 - SCL low period (master mode)
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W {
        SCLL_W { w: self }
    }
    ///Bits 8:15 - SCL high period (master mode)
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W {
        SCLH_W { w: self }
    }
    ///Bits 16:19 - Data hold time
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W {
        SDADEL_W { w: self }
    }
    ///Bits 20:23 - Data setup time
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W {
        SCLDEL_W { w: self }
    }
    ///Bits 28:31 - Timing prescaler
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timing register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_timingr](index.html) module
pub struct I2C_TIMINGR_SPEC;
impl crate::RegisterSpec for I2C_TIMINGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_timingr::R](R) reader structure
impl crate::Readable for I2C_TIMINGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2c_timingr::W](W) writer structure
impl crate::Writable for I2C_TIMINGR_SPEC {
    type Writer = W;
}
///`reset()` method sets I2C_TIMINGR to value 0
impl crate::Resettable for I2C_TIMINGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
