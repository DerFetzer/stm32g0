///Register `MISR` reader
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ALRAMF` reader - ALRAMF
pub struct ALRAMF_R(crate::FieldReader<bool, bool>);
impl ALRAMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRBMF` reader - ALRBMF
pub struct ALRBMF_R(crate::FieldReader<bool, bool>);
impl ALRBMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUTMF` reader - WUTMF
pub struct WUTMF_R(crate::FieldReader<bool, bool>);
impl WUTMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSMF` reader - TSMF
pub struct TSMF_R(crate::FieldReader<bool, bool>);
impl TSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSOVMF` reader - TSOVMF
pub struct TSOVMF_R(crate::FieldReader<bool, bool>);
impl TSOVMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOVMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSOVMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITSMF` reader - ITSMF
pub struct ITSMF_R(crate::FieldReader<bool, bool>);
impl ITSMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITSMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - ALRAMF
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - ALRBMF
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - WUTMF
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - TSMF
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - TSOVMF
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - ITSMF
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
///masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [misr](index.html) module
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [misr::R](R) reader structure
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
