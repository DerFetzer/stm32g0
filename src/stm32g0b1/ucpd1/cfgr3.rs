///Register `CFGR3` reader
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR3` writer
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIM1_NG_CCRPD` reader - SW trim value for RPD resistors on the CC1 line
pub struct TRIM1_NG_CCRPD_R(crate::FieldReader<u8, u8>);
impl TRIM1_NG_CCRPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM1_NG_CCRPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM1_NG_CCRPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIM1_NG_CCRPD` writer - SW trim value for RPD resistors on the CC1 line
pub struct TRIM1_NG_CCRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CCRPD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
///Field `TRIM1_NG_CC1A5` reader - SW trim value for RP1A5 resistors on the CC1 line
pub struct TRIM1_NG_CC1A5_R(crate::FieldReader<u8, u8>);
impl TRIM1_NG_CC1A5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM1_NG_CC1A5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM1_NG_CC1A5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIM1_NG_CC1A5` writer - SW trim value for RP1A5 resistors on the CC1 line
pub struct TRIM1_NG_CC1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CC1A5_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
///Field `TRIM1_NG_CC3A0` reader - SW trim value for RP3A0 resistors on the CC1 line
pub struct TRIM1_NG_CC3A0_R(crate::FieldReader<u8, u8>);
impl TRIM1_NG_CC3A0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM1_NG_CC3A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM1_NG_CC3A0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIM1_NG_CC3A0` writer - SW trim value for RP3A0 resistors on the CC1 line
pub struct TRIM1_NG_CC3A0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM1_NG_CC3A0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
///Field `TRIM2_NG_CCRPD` reader - SW trim value for RPD resistors on the CC2 line
pub struct TRIM2_NG_CCRPD_R(crate::FieldReader<u8, u8>);
impl TRIM2_NG_CCRPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM2_NG_CCRPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM2_NG_CCRPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIM2_NG_CCRPD` writer - SW trim value for RPD resistors on the CC2 line
pub struct TRIM2_NG_CCRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CCRPD_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
///Field `TRIM2_NG_CC1A5` reader - SW trim value for RP1A5 resistors on the CC2 line
pub struct TRIM2_NG_CC1A5_R(crate::FieldReader<u8, u8>);
impl TRIM2_NG_CC1A5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM2_NG_CC1A5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM2_NG_CC1A5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIM2_NG_CC1A5` writer - SW trim value for RP1A5 resistors on the CC2 line
pub struct TRIM2_NG_CC1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CC1A5_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
///Field `TRIM2_NG_CC3A0` reader - SW trim value for RP3A0 resistors on the CC2 line
pub struct TRIM2_NG_CC3A0_R(crate::FieldReader<u8, u8>);
impl TRIM2_NG_CC3A0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM2_NG_CC3A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM2_NG_CC3A0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRIM2_NG_CC3A0` writer - SW trim value for RP3A0 resistors on the CC2 line
pub struct TRIM2_NG_CC3A0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM2_NG_CC3A0_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
impl R {
    ///Bits 0:3 - SW trim value for RPD resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:8 - SW trim value for RP1A5 resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&self) -> TRIM1_NG_CC1A5_R {
        TRIM1_NG_CC1A5_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    ///Bits 9:12 - SW trim value for RP3A0 resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 16:19 - SW trim value for RPD resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:24 - SW trim value for RP1A5 resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&self) -> TRIM2_NG_CC1A5_R {
        TRIM2_NG_CC1A5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:28 - SW trim value for RP3A0 resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - SW trim value for RPD resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W {
        TRIM1_NG_CCRPD_W { w: self }
    }
    ///Bits 4:8 - SW trim value for RP1A5 resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&mut self) -> TRIM1_NG_CC1A5_W {
        TRIM1_NG_CC1A5_W { w: self }
    }
    ///Bits 9:12 - SW trim value for RP3A0 resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W {
        TRIM1_NG_CC3A0_W { w: self }
    }
    ///Bits 16:19 - SW trim value for RPD resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W {
        TRIM2_NG_CCRPD_W { w: self }
    }
    ///Bits 20:24 - SW trim value for RP1A5 resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&mut self) -> TRIM2_NG_CC1A5_W {
        TRIM2_NG_CC1A5_W { w: self }
    }
    ///Bits 25:28 - SW trim value for RP3A0 resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W {
        TRIM2_NG_CC3A0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr3](index.html) module
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr3::R](R) reader structure
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr3::W](W) writer structure
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR3 to value 0
impl crate::Resettable for CFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
