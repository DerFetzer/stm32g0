#[doc = "Register `LPTIM_CNT` reader"]
pub struct R(crate::R<LPTIM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Counter value"]
pub struct CNT_R(crate::FieldReader<u16, u16>);
impl CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cnt](index.html) module"]
pub struct LPTIM_CNT_SPEC;
impl crate::RegisterSpec for LPTIM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cnt::R](R) reader structure"]
impl crate::Readable for LPTIM_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPTIM_CNT to value 0"]
impl crate::Resettable for LPTIM_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}