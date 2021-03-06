///Register `HWCFGR4` reader
pub struct R(crate::R<HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HWCFGR4` writer
pub struct W(crate::W<HWCFGR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR4_SPEC>;
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
impl From<crate::W<HWCFGR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EVENT_TRG` reader - HW configuration event trigger type
pub struct EVENT_TRG_R(crate::FieldReader<u32, u32>);
impl EVENT_TRG_R {
    pub(crate) fn new(bits: u32) -> Self {
        EVENT_TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_TRG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EVENT_TRG` writer - HW configuration event trigger type
pub struct EVENT_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_TRG_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&mut self) -> EVENT_TRG_W {
        EVENT_TRG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Hardware configuration registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr4](index.html) module
pub struct HWCFGR4_SPEC;
impl crate::RegisterSpec for HWCFGR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr4::R](R) reader structure
impl crate::Readable for HWCFGR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hwcfgr4::W](W) writer structure
impl crate::Writable for HWCFGR4_SPEC {
    type Writer = W;
}
///`reset()` method sets HWCFGR4 to value 0
impl crate::Resettable for HWCFGR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
