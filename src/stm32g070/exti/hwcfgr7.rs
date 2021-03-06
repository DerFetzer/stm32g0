///Register `HWCFGR7` reader
pub struct R(crate::R<HWCFGR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR7_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HWCFGR7` writer
pub struct W(crate::W<HWCFGR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR7_SPEC>;
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
impl From<crate::W<HWCFGR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR7_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CPUEVENT` reader - HW configuration CPU event generation
pub struct CPUEVENT_R(crate::FieldReader<u32, u32>);
impl CPUEVENT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CPUEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPUEVENT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CPUEVENT` writer - HW configuration CPU event generation
pub struct CPUEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUEVENT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - HW configuration CPU event generation
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - HW configuration CPU event generation
    #[inline(always)]
    pub fn cpuevent(&mut self) -> CPUEVENT_W {
        CPUEVENT_W { w: self }
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
///For information about available fields see [hwcfgr7](index.html) module
pub struct HWCFGR7_SPEC;
impl crate::RegisterSpec for HWCFGR7_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr7::R](R) reader structure
impl crate::Readable for HWCFGR7_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hwcfgr7::W](W) writer structure
impl crate::Writable for HWCFGR7_SPEC {
    type Writer = W;
}
///`reset()` method sets HWCFGR7 to value 0
impl crate::Resettable for HWCFGR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
