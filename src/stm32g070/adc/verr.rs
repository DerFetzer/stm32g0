///Register `VERR` reader
pub struct R(crate::R<VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - Minor Revision number
pub struct MINREV_R(crate::FieldReader<u8, u8>);
impl MINREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MAJREV` reader - Major Revision number
pub struct MAJREV_R(crate::FieldReader<u8, u8>);
impl MAJREV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAJREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:3 - Minor Revision number
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Major Revision number
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///EXTI IP Version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [verr](index.html) module
pub struct VERR_SPEC;
impl crate::RegisterSpec for VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [verr::R](R) reader structure
impl crate::Readable for VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets VERR to value 0
impl crate::Resettable for VERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
