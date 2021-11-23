///Register `ALRMASSR` reader
pub struct R(crate::R<ALRMASSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMASSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMASSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMASSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ALRMASSR` writer
pub struct W(crate::W<ALRMASSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMASSR_SPEC>;
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
impl From<crate::W<ALRMASSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMASSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
pub struct SS_R(crate::FieldReader<u16, u16>);
impl SS_R {
    pub(crate) fn new(bits: u16) -> Self {
        SS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
///Mask the most-significant bits starting at this bit 2: SS\[14:2\]
///are don't care in alarm A comparison. Only SS\[1:0\]
///are compared. 3: SS\[14:3\]
///are don't care in alarm A comparison. Only SS\[2:0\]
///are compared. ... 12: SS\[14:12\]
///are don't care in alarm A comparison. SS\[11:0\]
///are compared. 13: SS\[14:13\]
///are don't care in alarm A comparison. SS\[12:0\]
///are compared. 14: SS\[14\]
///is don't care in alarm A comparison. SS\[13:0\]
///are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASKSS_A {
    ///0: No comparison on sub seconds for alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match).
    B_0X0 = 0,
    ///1: SS\[14:1\]
    ///are don't care in alarm A comparison. Only SS\[0\]
    ///is compared.
    B_0X1 = 1,
}
impl From<MASKSS_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKSS_A) -> Self {
        variant as _
    }
}
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit 2: SS\[14:2\]
///are don't care in alarm A comparison. Only SS\[1:0\]
///are compared. 3: SS\[14:3\]
///are don't care in alarm A comparison. Only SS\[2:0\]
///are compared. ... 12: SS\[14:12\]
///are don't care in alarm A comparison. SS\[11:0\]
///are compared. 13: SS\[14:13\]
///are don't care in alarm A comparison. SS\[12:0\]
///are compared. 14: SS\[14\]
///is don't care in alarm A comparison. SS\[13:0\]
///are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
pub struct MASKSS_R(crate::FieldReader<u8, MASKSS_A>);
impl MASKSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASKSS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MASKSS_A> {
        match self.bits {
            0 => Some(MASKSS_A::B_0X0),
            1 => Some(MASKSS_A::B_0X1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MASKSS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MASKSS_A::B_0X1
    }
}
impl core::ops::Deref for MASKSS_R {
    type Target = crate::FieldReader<u8, MASKSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit 2: SS\[14:2\]
///are don't care in alarm A comparison. Only SS\[1:0\]
///are compared. 3: SS\[14:3\]
///are don't care in alarm A comparison. Only SS\[2:0\]
///are compared. ... 12: SS\[14:12\]
///are don't care in alarm A comparison. SS\[11:0\]
///are compared. 13: SS\[14:13\]
///are don't care in alarm A comparison. SS\[12:0\]
///are compared. 14: SS\[14\]
///is don't care in alarm A comparison. SS\[13:0\]
///are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
pub struct MASKSS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKSS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MASKSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No comparison on sub seconds for alarm A. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MASKSS_A::B_0X0)
    }
    ///SS\[14:1\]
    ///are don't care in alarm A comparison. Only SS\[0\]
    ///is compared.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MASKSS_A::B_0X1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    ///Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit 2: SS\[14:2\]
    ///are don't care in alarm A comparison. Only SS\[1:0\]
    ///are compared. 3: SS\[14:3\]
    ///are don't care in alarm A comparison. Only SS\[2:0\]
    ///are compared. ... 12: SS\[14:12\]
    ///are don't care in alarm A comparison. SS\[11:0\]
    ///are compared. 13: SS\[14:13\]
    ///are don't care in alarm A comparison. SS\[12:0\]
    ///are compared. 14: SS\[14\]
    ///is don't care in alarm A comparison. SS\[13:0\]
    ///are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    ///Bits 24:27 - Mask the most-significant bits starting at this bit 2: SS\[14:2\]
    ///are don't care in alarm A comparison. Only SS\[1:0\]
    ///are compared. 3: SS\[14:3\]
    ///are don't care in alarm A comparison. Only SS\[2:0\]
    ///are compared. ... 12: SS\[14:12\]
    ///are don't care in alarm A comparison. SS\[11:0\]
    ///are compared. 13: SS\[14:13\]
    ///are don't care in alarm A comparison. SS\[12:0\]
    ///are compared. 14: SS\[14\]
    ///is don't care in alarm A comparison. SS\[13:0\]
    ///are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.
    #[inline(always)]
    pub fn maskss(&mut self) -> MASKSS_W {
        MASKSS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC alarm A sub second register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmassr](index.html) module
pub struct ALRMASSR_SPEC;
impl crate::RegisterSpec for ALRMASSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [alrmassr::R](R) reader structure
impl crate::Readable for ALRMASSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [alrmassr::W](W) writer structure
impl crate::Writable for ALRMASSR_SPEC {
    type Writer = W;
}
///`reset()` method sets ALRMASSR to value 0
impl crate::Resettable for ALRMASSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
