///Register `SECR` reader
pub struct R(crate::R<SECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECR` writer
pub struct W(crate::W<SECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECR_SPEC>;
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
impl From<crate::W<SECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC_SIZE` reader - Securable memory area size
pub struct SEC_SIZE_R(crate::FieldReader<u8, u8>);
impl SEC_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SEC_SIZE` writer - Securable memory area size
pub struct SEC_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_SIZE_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `BOOT_LOCK` reader - used to force boot from user area
pub struct BOOT_LOCK_R(crate::FieldReader<bool, bool>);
impl BOOT_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BOOT_LOCK` writer - used to force boot from user area
pub struct BOOT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `SEC_SIZE2` reader - Securable memory area size
pub struct SEC_SIZE2_R(crate::FieldReader<u8, u8>);
impl SEC_SIZE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC_SIZE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC_SIZE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SEC_SIZE2` writer - Securable memory area size
pub struct SEC_SIZE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_SIZE2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | ((value as u32 & 0xff) << 20);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Securable memory area size
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bits 20:27 - Securable memory area size
    #[inline(always)]
    pub fn sec_size2(&self) -> SEC_SIZE2_R {
        SEC_SIZE2_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Securable memory area size
    #[inline(always)]
    pub fn sec_size(&mut self) -> SEC_SIZE_W {
        SEC_SIZE_W { w: self }
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W {
        BOOT_LOCK_W { w: self }
    }
    ///Bits 20:27 - Securable memory area size
    #[inline(always)]
    pub fn sec_size2(&mut self) -> SEC_SIZE2_W {
        SEC_SIZE2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash Security register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secr](index.html) module
pub struct SECR_SPEC;
impl crate::RegisterSpec for SECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [secr::R](R) reader structure
impl crate::Readable for SECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [secr::W](W) writer structure
impl crate::Writable for SECR_SPEC {
    type Writer = W;
}
///`reset()` method sets SECR to value 0xf000_0000
impl crate::Resettable for SECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
