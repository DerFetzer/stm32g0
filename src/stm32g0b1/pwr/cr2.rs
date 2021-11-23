///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub struct PVDE_R(crate::FieldReader<bool, bool>);
impl PVDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVDE` writer - Power voltage detector enable
pub struct PVDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDE_W<'a> {
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
///Field `PVDFT` reader - Power voltage detector falling threshold selection
pub struct PVDFT_R(crate::FieldReader<u8, u8>);
impl PVDFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PVDFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVDFT` writer - Power voltage detector falling threshold selection
pub struct PVDFT_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDFT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
///Field `PVDRT` reader - Power voltage detector rising threshold selection
pub struct PVDRT_R(crate::FieldReader<u8, u8>);
impl PVDRT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PVDRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVDRT` writer - Power voltage detector rising threshold selection
pub struct PVDRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDRT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
///Field `PVMENDAC` reader - PVMENDAC
pub struct PVMENDAC_R(crate::FieldReader<bool, bool>);
impl PVMENDAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMENDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMENDAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVMENDAC` writer - PVMENDAC
pub struct PVMENDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMENDAC_W<'a> {
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
///Field `PVMENUSB` reader - PVMENUSB
pub struct PVMENUSB_R(crate::FieldReader<bool, bool>);
impl PVMENUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMENUSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMENUSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVMENUSB` writer - PVMENUSB
pub struct PVMENUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PVMENUSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Field `IOSV` reader - IOSV
pub struct IOSV_R(crate::FieldReader<bool, bool>);
impl IOSV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOSV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOSV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IOSV` writer - IOSV
pub struct IOSV_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `USV` reader - USV
pub struct USV_R(crate::FieldReader<bool, bool>);
impl USV_R {
    pub(crate) fn new(bits: bool) -> Self {
        USV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USV` writer - USV
pub struct USV_W<'a> {
    w: &'a mut W,
}
impl<'a> USV_W<'a> {
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
impl R {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:3 - Power voltage detector falling threshold selection
    #[inline(always)]
    pub fn pvdft(&self) -> PVDFT_R {
        PVDFT_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    ///Bits 4:6 - Power voltage detector rising threshold selection
    #[inline(always)]
    pub fn pvdrt(&self) -> PVDRT_R {
        PVDRT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    ///Bit 7 - PVMENDAC
    #[inline(always)]
    pub fn pvmendac(&self) -> PVMENDAC_R {
        PVMENDAC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - PVMENUSB
    #[inline(always)]
    pub fn pvmenusb(&self) -> PVMENUSB_R {
        PVMENUSB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - IOSV
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - USV
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W {
        PVDE_W { w: self }
    }
    ///Bits 1:3 - Power voltage detector falling threshold selection
    #[inline(always)]
    pub fn pvdft(&mut self) -> PVDFT_W {
        PVDFT_W { w: self }
    }
    ///Bits 4:6 - Power voltage detector rising threshold selection
    #[inline(always)]
    pub fn pvdrt(&mut self) -> PVDRT_W {
        PVDRT_W { w: self }
    }
    ///Bit 7 - PVMENDAC
    #[inline(always)]
    pub fn pvmendac(&mut self) -> PVMENDAC_W {
        PVMENDAC_W { w: self }
    }
    ///Bit 8 - PVMENUSB
    #[inline(always)]
    pub fn pvmenusb(&mut self) -> PVMENUSB_W {
        PVMENUSB_W { w: self }
    }
    ///Bit 9 - IOSV
    #[inline(always)]
    pub fn iosv(&mut self) -> IOSV_W {
        IOSV_W { w: self }
    }
    ///Bit 10 - USV
    #[inline(always)]
    pub fn usv(&mut self) -> USV_W {
        USV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
