///Register `CCIPR2` reader
pub struct R(crate::R<CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR2` writer
pub struct W(crate::W<CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR2_SPEC>;
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
impl From<crate::W<CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2S1SEL` reader - 2S1SEL
pub struct I2S1SEL_R(crate::FieldReader<u8, u8>);
impl I2S1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2S1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2S1SEL` writer - 2S1SEL
pub struct I2S1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Field `I2S2SEL` reader - I2S2SEL
pub struct I2S2SEL_R(crate::FieldReader<u8, u8>);
impl I2S2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2S2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2S2SEL` writer - I2S2SEL
pub struct I2S2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Field `FDCANSEL` reader - FDCANSEL
pub struct FDCANSEL_R(crate::FieldReader<u8, u8>);
impl FDCANSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDCANSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FDCANSEL` writer - FDCANSEL
pub struct FDCANSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Field `USBSEL` reader - USBSEL
pub struct USBSEL_R(crate::FieldReader<bool, bool>);
impl USBSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USBSEL` writer - USBSEL
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    ///Bits 0:1 - 2S1SEL
    #[inline(always)]
    pub fn i2s1sel(&self) -> I2S1SEL_R {
        I2S1SEL_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - I2S2SEL
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 8:9 - FDCANSEL
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 12 - USBSEL
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:1 - 2S1SEL
    #[inline(always)]
    pub fn i2s1sel(&mut self) -> I2S1SEL_W {
        I2S1SEL_W { w: self }
    }
    ///Bits 2:3 - I2S2SEL
    #[inline(always)]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W {
        I2S2SEL_W { w: self }
    }
    ///Bits 8:9 - FDCANSEL
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W {
        FDCANSEL_W { w: self }
    }
    ///Bit 12 - USBSEL
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Peripherals independent clock configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr2](index.html) module
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr2::R](R) reader structure
impl crate::Readable for CCIPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr2::W](W) writer structure
impl crate::Writable for CCIPR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
