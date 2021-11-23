///Register `PCROP1BER` reader
pub struct R(crate::R<PCROP1BER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1BER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1BER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1BER_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PCROP1B_END` reader - PCROP1B area end offset
pub struct PCROP1B_END_R(crate::FieldReader<u8, u8>);
impl PCROP1B_END_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCROP1B_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1B_END_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:7 - PCROP1B area end offset
    #[inline(always)]
    pub fn pcrop1b_end(&self) -> PCROP1B_END_R {
        PCROP1B_END_R::new((self.bits & 0xff) as u8)
    }
}
///Flash PCROP zone B End address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1ber](index.html) module
pub struct PCROP1BER_SPEC;
impl crate::RegisterSpec for PCROP1BER_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop1ber::R](R) reader structure
impl crate::Readable for PCROP1BER_SPEC {
    type Reader = R;
}
///`reset()` method sets PCROP1BER to value 0xf000_0000
impl crate::Resettable for PCROP1BER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}
