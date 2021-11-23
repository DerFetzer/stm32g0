///Register `ITLINE26` reader
pub struct R(crate::R<ITLINE26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE26_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SPI2` reader - SPI2
pub struct SPI2_R(crate::FieldReader<bool, bool>);
impl SPI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - SPI2
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0x01) != 0)
    }
}
///interrupt line 26 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline26](index.html) module
pub struct ITLINE26_SPEC;
impl crate::RegisterSpec for ITLINE26_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline26::R](R) reader structure
impl crate::Readable for ITLINE26_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE26 to value 0
impl crate::Resettable for ITLINE26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
