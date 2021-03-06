///Register `ITLINE29` reader
pub struct R(crate::R<ITLINE29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE29_SPEC>) -> Self {
        R(reader)
    }
}
///Field `USART3` reader - USART3
pub struct USART3_R(crate::FieldReader<bool, bool>);
impl USART3_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART4` reader - USART4
pub struct USART4_R(crate::FieldReader<bool, bool>);
impl USART4_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART5` reader - USART5
pub struct USART5_R(crate::FieldReader<bool, bool>);
impl USART5_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - USART3
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - USART4
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - USART5
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
///interrupt line 29 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline29](index.html) module
pub struct ITLINE29_SPEC;
impl crate::RegisterSpec for ITLINE29_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline29::R](R) reader structure
impl crate::Readable for ITLINE29_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE29 to value 0
impl crate::Resettable for ITLINE29_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
