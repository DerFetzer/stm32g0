///Register `I2SCFGR` reader
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2SCFGR` writer
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CHLEN` reader - Channel length (number of bits per audio channel)
pub struct CHLEN_R(crate::FieldReader<bool, bool>);
impl CHLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CHLEN` writer - Channel length (number of bits per audio channel)
pub struct CHLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHLEN_W<'a> {
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
///Field `DATLEN` reader - Data length to be transferred
pub struct DATLEN_R(crate::FieldReader<u8, u8>);
impl DATLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATLEN` writer - Data length to be transferred
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
///Field `CKPOL` reader - Inactive state clock polarity
pub struct CKPOL_R(crate::FieldReader<bool, bool>);
impl CKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CKPOL` writer - Inactive state clock polarity
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
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
///Field `I2SSTD` reader - standard selection
pub struct I2SSTD_R(crate::FieldReader<u8, u8>);
impl I2SSTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SSTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SSTD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SSTD` writer - standard selection
pub struct I2SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSTD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Field `PCMSYNC` reader - PCM frame synchronization
pub struct PCMSYNC_R(crate::FieldReader<bool, bool>);
impl PCMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCMSYNC` writer - PCM frame synchronization
pub struct PCMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Field `I2SCFG` reader - I2S configuration mode
pub struct I2SCFG_R(crate::FieldReader<u8, u8>);
impl I2SCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SCFG` writer - I2S configuration mode
pub struct I2SCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SCFG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Field `SE2` reader - I2S enable
pub struct SE2_R(crate::FieldReader<bool, bool>);
impl SE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SE2` writer - I2S enable
pub struct SE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `I2SMOD` reader - I2S mode selection
pub struct I2SMOD_R(crate::FieldReader<bool, bool>);
impl I2SMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SMOD` writer - I2S mode selection
pub struct I2SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    ///Bit 0 - Channel length (number of bits per audio channel)
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - Data length to be transferred
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bit 3 - Inactive state clock polarity
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - standard selection
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 7 - PCM frame synchronization
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:9 - I2S configuration mode
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 10 - I2S enable
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - I2S mode selection
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Channel length (number of bits per audio channel)
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    ///Bits 1:2 - Data length to be transferred
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    ///Bit 3 - Inactive state clock polarity
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    ///Bits 4:5 - standard selection
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W { w: self }
    }
    ///Bit 7 - PCM frame synchronization
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W {
        PCMSYNC_W { w: self }
    }
    ///Bits 8:9 - I2S configuration mode
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
    }
    ///Bit 10 - I2S enable
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W {
        SE2_W { w: self }
    }
    ///Bit 11 - I2S mode selection
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2scfgr](index.html) module
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2scfgr::R](R) reader structure
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2scfgr::W](W) writer structure
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
