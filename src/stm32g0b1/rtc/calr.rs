///Register `CALR` reader
pub struct R(crate::R<CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CALR` writer
pub struct W(crate::W<CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALR_SPEC>;
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
impl From<crate::W<CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALM` reader - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Â Hz). This decreases the frequency of the calendar with a resolution of 0.9537Â ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See .
pub struct CALM_R(crate::FieldReader<u16, u16>);
impl CALM_R {
    pub(crate) fn new(bits: u16) -> Self {
        CALM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALM` writer - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Â Hz). This decreases the frequency of the calendar with a resolution of 0.9537Â ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See .
pub struct CALM_W<'a> {
    w: &'a mut W,
}
impl<'a> CALM_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
///Field `CALW16` reader - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\]
///is stuck at 0 when CALW16 = 1. Refer to calibration.
pub struct CALW16_R(crate::FieldReader<bool, bool>);
impl CALW16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALW16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALW16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALW16` writer - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\]
///is stuck at 0 when CALW16 = 1. Refer to calibration.
pub struct CALW16_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW16_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `CALW8` reader - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\]
///are stuck at 00 when CALW8 = 1. Refer to digital calibration.
pub struct CALW8_R(crate::FieldReader<bool, bool>);
impl CALW8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALW8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALW8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALW8` writer - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\]
///are stuck at 00 when CALW8 = 1. Refer to digital calibration.
pub struct CALW8_W<'a> {
    w: &'a mut W,
}
impl<'a> CALW8_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Increase frequency of RTC by 488.5Â ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768Â Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 Ã CALP) - CALM. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALP_A {
    ///0: No RTCCLK pulses are added.
    B_0X0 = 0,
    ///1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5Â ppm).
    B_0X1 = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CALP` reader - Increase frequency of RTC by 488.5Â ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768Â Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 Ã CALP) - CALM. Refer to .
pub struct CALP_R(crate::FieldReader<bool, CALP_A>);
impl CALP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::B_0X0,
            true => CALP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CALP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CALP_A::B_0X1
    }
}
impl core::ops::Deref for CALP_R {
    type Target = crate::FieldReader<bool, CALP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CALP` writer - Increase frequency of RTC by 488.5Â ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768Â Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 Ã CALP) - CALM. Refer to .
pub struct CALP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CALP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No RTCCLK pulses are added.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CALP_A::B_0X0)
    }
    ///One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by 488.5Â ppm).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CALP_A::B_0X1)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    ///Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Â Hz). This decreases the frequency of the calendar with a resolution of 0.9537Â ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See .
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\]
    ///is stuck at 0 when CALW16 = 1. Refer to calibration.
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\]
    ///are stuck at 00 when CALW8 = 1. Refer to digital calibration.
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Increase frequency of RTC by 488.5Â ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768Â Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 Ã CALP) - CALM. Refer to .
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:8 - Calibration minus The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses (32 seconds if the input frequency is 32768Â Hz). This decreases the frequency of the calendar with a resolution of 0.9537Â ppm. To increase the frequency of the calendar, this feature should be used in conjunction with CALP. See .
    #[inline(always)]
    pub fn calm(&mut self) -> CALM_W {
        CALM_W { w: self }
    }
    ///Bit 13 - Use a 16-second calibration cycle period When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must not be set to 1 if CALW8 = 1. Note: CALM\[0\]
    ///is stuck at 0 when CALW16 = 1. Refer to calibration.
    #[inline(always)]
    pub fn calw16(&mut self) -> CALW16_W {
        CALW16_W { w: self }
    }
    ///Bit 14 - Use an 8-second calibration cycle period When CALW8 is set to 1, the 8-second calibration cycle period is selected. Note: CALM\[1:0\]
    ///are stuck at 00 when CALW8 = 1. Refer to digital calibration.
    #[inline(always)]
    pub fn calw8(&mut self) -> CALW8_W {
        CALW8_W { w: self }
    }
    ///Bit 15 - Increase frequency of RTC by 488.5Â ppm This feature is intended to be used in conjunction with CALM, which lowers the frequency of the calendar with a fine resolution. if the input frequency is 32768Â Hz, the number of RTCCLK pulses added during a 32-second window is calculated as follows: (512 Ã CALP) - CALM. Refer to .
    #[inline(always)]
    pub fn calp(&mut self) -> CALP_W {
        CALP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calr](index.html) module
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
///`read()` method returns [calr::R](R) reader structure
impl crate::Readable for CALR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [calr::W](W) writer structure
impl crate::Writable for CALR_SPEC {
    type Writer = W;
}
///`reset()` method sets CALR to value 0
impl crate::Resettable for CALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
