///Register `CFG2` reader
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFG2` writer
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXFILTDIS` reader - RXFILTDIS
pub struct RXFILTDIS_R(crate::FieldReader<bool, bool>);
impl RXFILTDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFILTDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFILTDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFILTDIS` writer - RXFILTDIS
pub struct RXFILTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFILTDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Field `RXFILT2N3` reader - RXFILT2N3
pub struct RXFILT2N3_R(crate::FieldReader<bool, bool>);
impl RXFILT2N3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFILT2N3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFILT2N3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFILT2N3` writer - RXFILT2N3
pub struct RXFILT2N3_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFILT2N3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///Field `FORCECLK` reader - FORCECLK
pub struct FORCECLK_R(crate::FieldReader<bool, bool>);
impl FORCECLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCECLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCECLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FORCECLK` writer - FORCECLK
pub struct FORCECLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCECLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `WUPEN` reader - WUPEN
pub struct WUPEN_R(crate::FieldReader<bool, bool>);
impl WUPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUPEN` writer - WUPEN
pub struct WUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    ///Bit 0 - RXFILTDIS
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - RXFILT2N3
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - FORCECLK
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - WUPEN
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - RXFILTDIS
    #[inline(always)]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W {
        RXFILTDIS_W { w: self }
    }
    ///Bit 1 - RXFILT2N3
    #[inline(always)]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W {
        RXFILT2N3_W { w: self }
    }
    ///Bit 2 - FORCECLK
    #[inline(always)]
    pub fn forceclk(&mut self) -> FORCECLK_W {
        FORCECLK_W { w: self }
    }
    ///Bit 3 - WUPEN
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W {
        WUPEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfg2](index.html) module
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfg2::R](R) reader structure
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfg2::W](W) writer structure
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
}
///`reset()` method sets CFG2 to value 0
impl crate::Resettable for CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
