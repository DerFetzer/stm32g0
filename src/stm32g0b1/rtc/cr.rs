///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    ///0: RTC/16 clock is selected
    B_0X0 = 0,
    ///1: RTC/8 clock is selected
    B_0X1 = 1,
    ///2: RTC/4 clock is selected
    B_0X2 = 2,
    ///3: RTC/2 clock is selected
    B_0X3 = 3,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
///Field `WUCKSEL` reader - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value
pub struct WUCKSEL_R(crate::FieldReader<u8, WUCKSEL_A>);
impl WUCKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUCKSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCKSEL_A> {
        match self.bits {
            0 => Some(WUCKSEL_A::B_0X0),
            1 => Some(WUCKSEL_A::B_0X1),
            2 => Some(WUCKSEL_A::B_0X2),
            3 => Some(WUCKSEL_A::B_0X3),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUCKSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUCKSEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == WUCKSEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == WUCKSEL_A::B_0X3
    }
}
impl core::ops::Deref for WUCKSEL_R {
    type Target = crate::FieldReader<u8, WUCKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUCKSEL` writer - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value
pub struct WUCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WUCKSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUCKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///RTC/16 clock is selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUCKSEL_A::B_0X0)
    }
    ///RTC/8 clock is selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUCKSEL_A::B_0X1)
    }
    ///RTC/4 clock is selected
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(WUCKSEL_A::B_0X2)
    }
    ///RTC/2 clock is selected
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(WUCKSEL_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEDGE_A {
    ///0: RTC_TS input rising edge generates a timestamp event
    B_0X0 = 0,
    ///1: RTC_TS input falling edge generates a timestamp event
    B_0X1 = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEDGE` reader - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
pub struct TSEDGE_R(crate::FieldReader<bool, TSEDGE_A>);
impl TSEDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEDGE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::B_0X0,
            true => TSEDGE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TSEDGE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TSEDGE_A::B_0X1
    }
}
impl core::ops::Deref for TSEDGE_R {
    type Target = crate::FieldReader<bool, TSEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEDGE` writer - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
pub struct TSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEDGE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSEDGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTC_TS input rising edge generates a timestamp event
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSEDGE_A::B_0X0)
    }
    ///RTC_TS input falling edge generates a timestamp event
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSEDGE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFCKON_A {
    ///0: RTC_REFIN detection disabled
    B_0X0 = 0,
    ///1: RTC_REFIN detection enabled
    B_0X1 = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF.
pub struct REFCKON_R(crate::FieldReader<bool, REFCKON_A>);
impl REFCKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFCKON_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::B_0X0,
            true => REFCKON_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == REFCKON_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == REFCKON_A::B_0X1
    }
}
impl core::ops::Deref for REFCKON_R {
    type Target = crate::FieldReader<bool, REFCKON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF.
pub struct REFCKON_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCKON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REFCKON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REFCKON_A::B_0X0)
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REFCKON_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPSHAD_A {
    ///0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles.
    B_0X0 = 0,
    ///1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    B_0X1 = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
pub struct BYPSHAD_R(crate::FieldReader<bool, BYPSHAD_A>);
impl BYPSHAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPSHAD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::B_0X0,
            true => BYPSHAD_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BYPSHAD_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BYPSHAD_A::B_0X1
    }
}
impl core::ops::Deref for BYPSHAD_R {
    type Target = crate::FieldReader<bool, BYPSHAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
pub struct BYPSHAD_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPSHAD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BYPSHAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BYPSHAD_A::B_0X0)
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BYPSHAD_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Hour format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMT_A {
    ///0: 24 hour/day format
    B_0X0 = 0,
    ///1: AM/PM hour format
    B_0X1 = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FMT` reader - Hour format
pub struct FMT_R(crate::FieldReader<bool, FMT_A>);
impl FMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::B_0X0,
            true => FMT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FMT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FMT_A::B_0X1
    }
}
impl core::ops::Deref for FMT_R {
    type Target = crate::FieldReader<bool, FMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FMT` writer - Hour format
pub struct FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> FMT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///24 hour/day format
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMT_A::B_0X0)
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMT_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Alarm A enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAE_A {
    ///0: Alarm A disabled
    B_0X0 = 0,
    ///1: Alarm A enabled
    B_0X1 = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAE` reader - Alarm A enable
pub struct ALRAE_R(crate::FieldReader<bool, ALRAE_A>);
impl ALRAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::B_0X0,
            true => ALRAE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ALRAE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ALRAE_A::B_0X1
    }
}
impl core::ops::Deref for ALRAE_R {
    type Target = crate::FieldReader<bool, ALRAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRAE` writer - Alarm A enable
pub struct ALRAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm A disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ALRAE_A::B_0X0)
    }
    ///Alarm A enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ALRAE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Alarm B enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBE_A {
    ///0: Alarm B disabled
    B_0X0 = 0,
    ///1: Alarm B enabled
    B_0X1 = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRBE` reader - Alarm B enable
pub struct ALRBE_R(crate::FieldReader<bool, ALRBE_A>);
impl ALRBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::B_0X0,
            true => ALRBE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ALRBE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ALRBE_A::B_0X1
    }
}
impl core::ops::Deref for ALRBE_R {
    type Target = crate::FieldReader<bool, ALRBE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRBE` writer - Alarm B enable
pub struct ALRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRBE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm B disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ALRBE_A::B_0X0)
    }
    ///Alarm B enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ALRBE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTE_A {
    ///0: Wakeup timer disabled
    B_0X0 = 0,
    ///1: Wakeup timer enabled
    B_0X1 = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTE` reader - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.
pub struct WUTE_R(crate::FieldReader<bool, WUTE_A>);
impl WUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::B_0X0,
            true => WUTE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUTE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUTE_A::B_0X1
    }
}
impl core::ops::Deref for WUTE_R {
    type Target = crate::FieldReader<bool, WUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUTE` writer - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.
pub struct WUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wakeup timer disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUTE_A::B_0X0)
    }
    ///Wakeup timer enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUTE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///timestamp enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSE_A {
    ///0: timestamp disable
    B_0X0 = 0,
    ///1: timestamp enable
    B_0X1 = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSE` reader - timestamp enable
pub struct TSE_R(crate::FieldReader<bool, TSE_A>);
impl TSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::B_0X0,
            true => TSE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TSE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TSE_A::B_0X1
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, TSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSE` writer - timestamp enable
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///timestamp disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSE_A::B_0X0)
    }
    ///timestamp enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Alarm A interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRAIE_A {
    ///0: Alarm A interrupt disabled
    B_0X0 = 0,
    ///1: Alarm A interrupt enabled
    B_0X1 = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAIE` reader - Alarm A interrupt enable
pub struct ALRAIE_R(crate::FieldReader<bool, ALRAIE_A>);
impl ALRAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::B_0X0,
            true => ALRAIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ALRAIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ALRAIE_A::B_0X1
    }
}
impl core::ops::Deref for ALRAIE_R {
    type Target = crate::FieldReader<bool, ALRAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRAIE` writer - Alarm A interrupt enable
pub struct ALRAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRAIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm A interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ALRAIE_A::B_0X0)
    }
    ///Alarm A interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ALRAIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Alarm B interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALRBIE_A {
    ///0: Alarm B interrupt disable
    B_0X0 = 0,
    ///1: Alarm B interrupt enable
    B_0X1 = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRBIE` reader - Alarm B interrupt enable
pub struct ALRBIE_R(crate::FieldReader<bool, ALRBIE_A>);
impl ALRBIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::B_0X0,
            true => ALRBIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ALRBIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ALRBIE_A::B_0X1
    }
}
impl core::ops::Deref for ALRBIE_R {
    type Target = crate::FieldReader<bool, ALRBIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALRBIE` writer - Alarm B interrupt enable
pub struct ALRBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRBIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALRBIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Alarm B interrupt disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ALRBIE_A::B_0X0)
    }
    ///Alarm B interrupt enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ALRBIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Wakeup timer interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUTIE_A {
    ///0: Wakeup timer interrupt disabled
    B_0X0 = 0,
    ///1: Wakeup timer interrupt enabled
    B_0X1 = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUTIE` reader - Wakeup timer interrupt enable
pub struct WUTIE_R(crate::FieldReader<bool, WUTIE_A>);
impl WUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::B_0X0,
            true => WUTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUTIE_A::B_0X1
    }
}
impl core::ops::Deref for WUTIE_R {
    type Target = crate::FieldReader<bool, WUTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUTIE` writer - Wakeup timer interrupt enable
pub struct WUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wakeup timer interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUTIE_A::B_0X0)
    }
    ///Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUTIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Timestamp interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIE_A {
    ///0: Timestamp interrupt disable
    B_0X0 = 0,
    ///1: Timestamp interrupt enable
    B_0X1 = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIE` reader - Timestamp interrupt enable
pub struct TSIE_R(crate::FieldReader<bool, TSIE_A>);
impl TSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::B_0X0,
            true => TSIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TSIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TSIE_A::B_0X1
    }
}
impl core::ops::Deref for TSIE_R {
    type Target = crate::FieldReader<bool, TSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSIE` writer - Timestamp interrupt enable
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Timestamp interrupt disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSIE_A::B_0X0)
    }
    ///Timestamp interrupt enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSIE_A::B_0X1)
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
///Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD1H_AW {
    ///0: No effect
    B_0X0 = 0,
    ///1: Adds 1 hour to the current time. This can be used for summer time change
    B_0X1 = 1,
}
impl From<ADD1H_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1H_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
pub struct ADD1H_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1H_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADD1H_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADD1H_AW::B_0X0)
    }
    ///Adds 1 hour to the current time. This can be used for summer time change
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADD1H_AW::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB1H_AW {
    ///0: No effect
    B_0X0 = 0,
    ///1: Subtracts 1 hour to the current time. This can be used for winter time change.
    B_0X1 = 1,
}
impl From<SUB1H_AW> for bool {
    #[inline(always)]
    fn from(variant: SUB1H_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
pub struct SUB1H_W<'a> {
    w: &'a mut W,
}
impl<'a> SUB1H_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SUB1H_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SUB1H_AW::B_0X0)
    }
    ///Subtracts 1 hour to the current time. This can be used for winter time change.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SUB1H_AW::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub struct BKP_R(crate::FieldReader<bool, bool>);
impl BKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub struct BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSEL_A {
    ///0: Calibration output is 512 Hz
    B_0X0 = 0,
    ///1: Calibration output is 1 Hz
    B_0X1 = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COSEL` reader - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .
pub struct COSEL_R(crate::FieldReader<bool, COSEL_A>);
impl COSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        COSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::B_0X0,
            true => COSEL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == COSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == COSEL_A::B_0X1
    }
}
impl core::ops::Deref for COSEL_R {
    type Target = crate::FieldReader<bool, COSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COSEL` writer - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .
pub struct COSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calibration output is 512 Hz
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COSEL_A::B_0X0)
    }
    ///Calibration output is 1 Hz
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COSEL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Output polarity This bit is used to configure the polarity of TAMPALRM output.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_A {
    ///0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).
    B_0X0 = 0,
    ///1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).
    B_0X1 = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `POL` reader - Output polarity This bit is used to configure the polarity of TAMPALRM output.
pub struct POL_R(crate::FieldReader<bool, POL_A>);
impl POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        POL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::B_0X0,
            true => POL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == POL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == POL_A::B_0X1
    }
}
impl core::ops::Deref for POL_R {
    type Target = crate::FieldReader<bool, POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POL` writer - Output polarity This bit is used to configure the polarity of TAMPALRM output.
pub struct POL_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(POL_A::B_0X0)
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\]), or when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(POL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Output selection These bits are used to select the flag to be routed to TAMPALRM output.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSEL_A {
    ///0: Output disabled
    B_0X0 = 0,
    ///1: Alarm A output enabled
    B_0X1 = 1,
    ///2: Alarm B output enabled
    B_0X2 = 2,
    ///3: Wakeup output enabled
    B_0X3 = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
///Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
pub struct OSEL_R(crate::FieldReader<u8, OSEL_A>);
impl OSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::B_0X0,
            1 => OSEL_A::B_0X1,
            2 => OSEL_A::B_0X2,
            3 => OSEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OSEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == OSEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == OSEL_A::B_0X3
    }
}
impl core::ops::Deref for OSEL_R {
    type Target = crate::FieldReader<u8, OSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
pub struct OSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Output disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OSEL_A::B_0X0)
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OSEL_A::B_0X1)
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(OSEL_A::B_0X2)
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(OSEL_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
///Calibration output enable This bit enables the CALIB output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COE_A {
    ///0: Calibration output disabled
    B_0X0 = 0,
    ///1: Calibration output enabled
    B_0X1 = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COE` reader - Calibration output enable This bit enables the CALIB output
pub struct COE_R(crate::FieldReader<bool, COE_A>);
impl COE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::B_0X0,
            true => COE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == COE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == COE_A::B_0X1
    }
}
impl core::ops::Deref for COE_R {
    type Target = crate::FieldReader<bool, COE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COE` writer - Calibration output enable This bit enables the CALIB output
pub struct COE_W<'a> {
    w: &'a mut W,
}
impl<'a> COE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Calibration output disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COE_A::B_0X0)
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///timestamp on internal event enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITSE_A {
    ///0: internal event timestamp disabled
    B_0X0 = 0,
    ///1: internal event timestamp enabled
    B_0X1 = 1,
}
impl From<ITSE_A> for bool {
    #[inline(always)]
    fn from(variant: ITSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ITSE` reader - timestamp on internal event enable
pub struct ITSE_R(crate::FieldReader<bool, ITSE_A>);
impl ITSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITSE_A {
        match self.bits {
            false => ITSE_A::B_0X0,
            true => ITSE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ITSE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ITSE_A::B_0X1
    }
}
impl core::ops::Deref for ITSE_R {
    type Target = crate::FieldReader<bool, ITSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ITSE` writer - timestamp on internal event enable
pub struct ITSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ITSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///internal event timestamp disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ITSE_A::B_0X0)
    }
    ///internal event timestamp enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ITSE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPTS_A {
    ///0: Tamper detection event does not cause a RTC timestamp to be saved
    B_0X0 = 0,
    ///1: Save RTC timestamp on tamper detection event
    B_0X1 = 1,
}
impl From<TAMPTS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPTS` reader - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.
pub struct TAMPTS_R(crate::FieldReader<bool, TAMPTS_A>);
impl TAMPTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPTS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPTS_A {
        match self.bits {
            false => TAMPTS_A::B_0X0,
            true => TAMPTS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPTS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPTS_A::B_0X1
    }
}
impl core::ops::Deref for TAMPTS_R {
    type Target = crate::FieldReader<bool, TAMPTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPTS` writer - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.
pub struct TAMPTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPTS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPTS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper detection event does not cause a RTC timestamp to be saved
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPTS_A::B_0X0)
    }
    ///Save RTC timestamp on tamper detection event
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPTS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Tamper detection output enable on TAMPALRM
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPOE_A {
    ///0: The tamper flag is not routed on TAMPALRM
    B_0X0 = 0,
    ///1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL.
    B_0X1 = 1,
}
impl From<TAMPOE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM
pub struct TAMPOE_R(crate::FieldReader<bool, TAMPOE_A>);
impl TAMPOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPOE_A {
        match self.bits {
            false => TAMPOE_A::B_0X0,
            true => TAMPOE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPOE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPOE_A::B_0X1
    }
}
impl core::ops::Deref for TAMPOE_R {
    type Target = crate::FieldReader<bool, TAMPOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM
pub struct TAMPOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The tamper flag is not routed on TAMPALRM
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPOE_A::B_0X0)
    }
    ///The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPOE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///TAMPALRM pull-up enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPALRM_PU_A {
    ///0: No pull-up is applied on TAMPALRM output
    B_0X0 = 0,
    ///1: A pull-up is applied on TAMPALRM output
    B_0X1 = 1,
}
impl From<TAMPALRM_PU_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_PU_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable
pub struct TAMPALRM_PU_R(crate::FieldReader<bool, TAMPALRM_PU_A>);
impl TAMPALRM_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPALRM_PU_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPALRM_PU_A {
        match self.bits {
            false => TAMPALRM_PU_A::B_0X0,
            true => TAMPALRM_PU_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPALRM_PU_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPALRM_PU_A::B_0X1
    }
}
impl core::ops::Deref for TAMPALRM_PU_R {
    type Target = crate::FieldReader<bool, TAMPALRM_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable
pub struct TAMPALRM_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPALRM_PU_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPALRM_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPALRM_PU_A::B_0X0)
    }
    ///A pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPALRM_PU_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///TAMPALRM output type
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPALRM_TYPE_A {
    ///0: TAMPALRM is push-pull output
    B_0X0 = 0,
    ///1: TAMPALRM is open-drain output
    B_0X1 = 1,
}
impl From<TAMPALRM_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPALRM_TYPE` reader - TAMPALRM output type
pub struct TAMPALRM_TYPE_R(crate::FieldReader<bool, TAMPALRM_TYPE_A>);
impl TAMPALRM_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPALRM_TYPE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPALRM_TYPE_A {
        match self.bits {
            false => TAMPALRM_TYPE_A::B_0X0,
            true => TAMPALRM_TYPE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMPALRM_TYPE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMPALRM_TYPE_A::B_0X1
    }
}
impl core::ops::Deref for TAMPALRM_TYPE_R {
    type Target = crate::FieldReader<bool, TAMPALRM_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMPALRM_TYPE` writer - TAMPALRM output type
pub struct TAMPALRM_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPALRM_TYPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMPALRM_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///TAMPALRM is push-pull output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMPALRM_TYPE_A::B_0X0)
    }
    ///TAMPALRM is open-drain output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMPALRM_TYPE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///Field `OUT2EN` reader - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1.
pub struct OUT2EN_R(crate::FieldReader<bool, bool>);
impl OUT2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OUT2EN` writer - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1.
pub struct OUT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF.
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Timestamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output.
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    ///Bit 23 - Calibration output enable This bit enables the CALIB output
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - Tamper detection output enable on TAMPALRM
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 29 - TAMPALRM pull-up enable
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - TAMPALRM output type
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1.
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W {
        WUCKSEL_W { w: self }
    }
    ///Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W {
        TSEDGE_W { w: self }
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF.
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W {
        REFCKON_W { w: self }
    }
    ///Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W {
        BYPSHAD_W { w: self }
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W {
        FMT_W { w: self }
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W {
        ALRAE_W { w: self }
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W {
        ALRBE_W { w: self }
    }
    ///Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again.
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W {
        WUTE_W { w: self }
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W {
        ALRAIE_W { w: self }
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W {
        ALRBIE_W { w: self }
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W {
        WUTIE_W { w: self }
    }
    ///Bit 15 - Timestamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    ///Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W {
        ADD1H_W { w: self }
    }
    ///Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W {
        SUB1H_W { w: self }
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W { w: self }
    }
    ///Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to .
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W {
        COSEL_W { w: self }
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output.
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W { w: self }
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W {
        OSEL_W { w: self }
    }
    ///Bit 23 - Calibration output enable This bit enables the CALIB output
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W {
        COE_W { w: self }
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W {
        ITSE_W { w: self }
    }
    ///Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts.
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W {
        TAMPTS_W { w: self }
    }
    ///Bit 26 - Tamper detection output enable on TAMPALRM
    #[inline(always)]
    pub fn tampoe(&mut self) -> TAMPOE_W {
        TAMPOE_W { w: self }
    }
    ///Bit 29 - TAMPALRM pull-up enable
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W {
        TAMPALRM_PU_W { w: self }
    }
    ///Bit 30 - TAMPALRM output type
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W {
        TAMPALRM_TYPE_W { w: self }
    }
    ///Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1.
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W {
        OUT2EN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
