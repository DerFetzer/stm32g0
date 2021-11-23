///Register `AFRL` reader
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AFRL` writer
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AFSEL7` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL7_R(crate::FieldReader<u8, u8>);
impl AFSEL7_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL7` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL7_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
///Field `AFSEL6` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL6_R(crate::FieldReader<u8, u8>);
impl AFSEL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL6` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL6_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
///Field `AFSEL5` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL5_R(crate::FieldReader<u8, u8>);
impl AFSEL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL5` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL5_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
///Field `AFSEL4` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL4_R(crate::FieldReader<u8, u8>);
impl AFSEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL4` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL4_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Field `AFSEL3` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL3_R(crate::FieldReader<u8, u8>);
impl AFSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL3` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL3_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
///Field `AFSEL2` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL2_R(crate::FieldReader<u8, u8>);
impl AFSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL2` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Field `AFSEL1` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL1_R(crate::FieldReader<u8, u8>);
impl AFSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL1` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///Field `AFSEL0` reader - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL0_R(crate::FieldReader<u8, u8>);
impl AFSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        AFSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFSEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `AFSEL0` writer - Alternate function selection for port x bit y (y = 0..7)
pub struct AFSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSEL0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel7(&mut self) -> AFSEL7_W {
        AFSEL7_W { w: self }
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel6(&mut self) -> AFSEL6_W {
        AFSEL6_W { w: self }
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel5(&mut self) -> AFSEL5_W {
        AFSEL5_W { w: self }
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel4(&mut self) -> AFSEL4_W {
        AFSEL4_W { w: self }
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W {
        AFSEL3_W { w: self }
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel2(&mut self) -> AFSEL2_W {
        AFSEL2_W { w: self }
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel1(&mut self) -> AFSEL1_W {
        AFSEL1_W { w: self }
    }
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    #[inline(always)]
    pub fn afsel0(&mut self) -> AFSEL0_W {
        AFSEL0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO alternate function low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrl](index.html) module
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [afrl::R](R) reader structure
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [afrl::W](W) writer structure
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
