///Register `IPIDR` reader
pub struct R(crate::R<IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - IP identification
pub struct ID_R(crate::FieldReader<u32, u32>);
impl ID_R {
    pub(crate) fn new(bits: u32) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - IP identification
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
///DMAMUX IP identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ipidr](index.html) module
pub struct IPIDR_SPEC;
impl crate::RegisterSpec for IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ipidr::R](R) reader structure
impl crate::Readable for IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets IPIDR to value 0x0010_0011
impl crate::Resettable for IPIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0011
    }
}
