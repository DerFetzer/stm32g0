///Register `FLTCR` reader
pub struct R(crate::R<FLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLTCR` writer
pub struct W(crate::W<FLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCR_SPEC>;
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
impl From<crate::W<FLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    ///0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    B_0X0 = 0,
    ///1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    B_0X1 = 1,
    ///2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    B_0X2 = 2,
    ///3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    B_0X3 = 3,
    ///4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    B_0X4 = 4,
    ///5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    B_0X5 = 5,
    ///6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    B_0X6 = 6,
    ///7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    B_0X7 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
///Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
pub struct TAMPFREQ_R(crate::FieldReader<u8, TAMPFREQ_A>);
impl TAMPFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFREQ_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::B_0X0,
            1 => TAMPFREQ_A::B_0X1,
            2 => TAMPFREQ_A::B_0X2,
            3 => TAMPFREQ_A::B_0X3,
            4 => TAMPFREQ_A::B_0X4,
            5 => TAMPFREQ_A::B_0X5,
            6 => TAMPFREQ_A::B_0X6,
            7 => TAMPFREQ_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPFREQ_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPFREQ_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TAMPFREQ_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TAMPFREQ_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == TAMPFREQ_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == TAMPFREQ_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == TAMPFREQ_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == TAMPFREQ_A::B_0X7
    }
}
impl core::ops::Deref for TAMPFREQ_R {
    type Target = crate::FieldReader<u8, TAMPFREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
pub struct TAMPFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFREQ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPFREQ_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X0)
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X1)
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X2)
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X3)
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X4)
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X5)
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X6)
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    ///0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input).
    B_0X0 = 0,
    ///1: Tamper event is activated after 2 consecutive samples at the active level.
    B_0X1 = 1,
    ///2: Tamper event is activated after 4 consecutive samples at the active level.
    B_0X2 = 2,
    ///3: Tamper event is activated after 8 consecutive samples at the active level.
    B_0X3 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
///Field `TAMPFLT` reader - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
pub struct TAMPFLT_R(crate::FieldReader<u8, TAMPFLT_A>);
impl TAMPFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFLT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::B_0X0,
            1 => TAMPFLT_A::B_0X1,
            2 => TAMPFLT_A::B_0X2,
            3 => TAMPFLT_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPFLT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPFLT_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TAMPFLT_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TAMPFLT_A::B_0X3
    }
}
impl core::ops::Deref for TAMPFLT_R {
    type Target = crate::FieldReader<u8, TAMPFLT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPFLT` writer - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
pub struct TAMPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFLT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPFLT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPFLT_A::B_0X0)
    }
    ///Tamper event is activated after 2 consecutive samples at the active level.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPFLT_A::B_0X1)
    }
    ///Tamper event is activated after 4 consecutive samples at the active level.
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TAMPFLT_A::B_0X2)
    }
    ///Tamper event is activated after 8 consecutive samples at the active level.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TAMPFLT_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
///TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    ///0: 1 RTCCLK cycle
    B_0X0 = 0,
    ///1: 2 RTCCLK cycles
    B_0X1 = 1,
    ///2: 4 RTCCLK cycles
    B_0X2 = 2,
    ///3: 8 RTCCLK cycles
    B_0X3 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
///Field `TAMPPRCH` reader - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
pub struct TAMPPRCH_R(crate::FieldReader<u8, TAMPPRCH_A>);
impl TAMPPRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPPRCH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::B_0X0,
            1 => TAMPPRCH_A::B_0X1,
            2 => TAMPPRCH_A::B_0X2,
            3 => TAMPPRCH_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPPRCH_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPPRCH_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TAMPPRCH_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TAMPPRCH_A::B_0X3
    }
}
impl core::ops::Deref for TAMPPRCH_R {
    type Target = crate::FieldReader<u8, TAMPPRCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPPRCH` writer - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
pub struct TAMPPRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPPRCH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::B_0X0)
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::B_0X1)
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::B_0X2)
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
///TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPPUDIS_A {
    ///0: Precharge TAMP_INx pins before sampling (enable internal pull-up)
    B_0X0 = 0,
    ///1: Disable precharge of TAMP_INx pins.
    B_0X1 = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPPUDIS` reader - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
pub struct TAMPPUDIS_R(crate::FieldReader<bool, TAMPPUDIS_A>);
impl TAMPPUDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPPUDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::B_0X0,
            true => TAMPPUDIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPPUDIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPPUDIS_A::B_0X1
    }
}
impl core::ops::Deref for TAMPPUDIS_R {
    type Target = crate::FieldReader<bool, TAMPPUDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPPUDIS` writer - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
pub struct TAMPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPUDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPPUDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Precharge TAMP_INx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::B_0X0)
    }
    ///Disable precharge of TAMP_INx pins.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    ///Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled.
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    ///Bits 3:4 - TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    ///Bits 5:6 - TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W {
        TAMPPRCH_W { w: self }
    }
    ///Bit 7 - TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample.
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W {
        TAMPPUDIS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltcr](index.html) module
pub struct FLTCR_SPEC;
impl crate::RegisterSpec for FLTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fltcr::R](R) reader structure
impl crate::Readable for FLTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fltcr::W](W) writer structure
impl crate::Writable for FLTCR_SPEC {
    type Writer = W;
}
///`reset()` method sets FLTCR to value 0
impl crate::Resettable for FLTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
