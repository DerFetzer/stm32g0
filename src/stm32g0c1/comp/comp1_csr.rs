///Register `COMP1_CSR` reader
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP1_CSR` writer
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0X0,
            true => EN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EN_A::B_0X1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EN` writer - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
///Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INMSEL_A {
    ///0: 1/4 VREFINT
    B_0X0 = 0,
    ///1: 1/2 VREFINT
    B_0X1 = 1,
    ///2: 3/4 VREFINT
    B_0X2 = 2,
    ///3: VREFINT
    B_0X3 = 3,
    ///4: DAC channel 1
    B_0X4 = 4,
    ///5: DAC channel 2
    B_0X5 = 5,
    ///6: PB1
    B_0X6 = 6,
    ///7: PC4
    B_0X7 = 7,
    ///8: PA0
    B_0X8 = 8,
}
impl From<INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INMSEL_A) -> Self {
        variant as _
    }
}
///Field `INMSEL` reader - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
pub struct INMSEL_R(crate::FieldReader<u8, INMSEL_A>);
impl INMSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INMSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<INMSEL_A> {
        match self.bits {
            0 => Some(INMSEL_A::B_0X0),
            1 => Some(INMSEL_A::B_0X1),
            2 => Some(INMSEL_A::B_0X2),
            3 => Some(INMSEL_A::B_0X3),
            4 => Some(INMSEL_A::B_0X4),
            5 => Some(INMSEL_A::B_0X5),
            6 => Some(INMSEL_A::B_0X6),
            7 => Some(INMSEL_A::B_0X7),
            8 => Some(INMSEL_A::B_0X8),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == INMSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == INMSEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == INMSEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == INMSEL_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == INMSEL_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == INMSEL_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == INMSEL_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == INMSEL_A::B_0X7
    }
    ///Checks if the value of the field is `B_0X8`
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == INMSEL_A::B_0X8
    }
}
impl core::ops::Deref for INMSEL_R {
    type Target = crate::FieldReader<u8, INMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INMSEL` writer - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
pub struct INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INMSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: INMSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///1/4 VREFINT
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X0)
    }
    ///1/2 VREFINT
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X1)
    }
    ///3/4 VREFINT
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X2)
    }
    ///VREFINT
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X3)
    }
    ///DAC channel 1
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X4)
    }
    ///DAC channel 2
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X5)
    }
    ///PB1
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X6)
    }
    ///PC4
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X7)
    }
    ///PA0
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(INMSEL_A::B_0X8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
///Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPSEL_A {
    ///0: PC5
    B_0X0 = 0,
    ///1: PB2
    B_0X1 = 1,
    ///2: PA1
    B_0X2 = 2,
    ///3: None (open)
    B_0X3 = 3,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
///Field `INPSEL` reader - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
pub struct INPSEL_R(crate::FieldReader<u8, INPSEL_A>);
impl INPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INPSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INPSEL_A {
        match self.bits {
            0 => INPSEL_A::B_0X0,
            1 => INPSEL_A::B_0X1,
            2 => INPSEL_A::B_0X2,
            3 => INPSEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == INPSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == INPSEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == INPSEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == INPSEL_A::B_0X3
    }
}
impl core::ops::Deref for INPSEL_R {
    type Target = crate::FieldReader<u8, INPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INPSEL` writer - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: INPSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///PC5
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(INPSEL_A::B_0X0)
    }
    ///PB2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(INPSEL_A::B_0X1)
    }
    ///PA1
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(INPSEL_A::B_0X2)
    }
    ///None (open)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(INPSEL_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINMODE_A {
    ///0: Signal selected with INPSEL\[1:0\]
    ///bitfield of this register
    B_0X0 = 0,
    ///1: COMP2_INP signal of the comparator 2 (required for window mode, see Figure 64)
    B_0X1 = 1,
}
impl From<WINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WINMODE` reader - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
pub struct WINMODE_R(crate::FieldReader<bool, WINMODE_A>);
impl WINMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WINMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WINMODE_A {
        match self.bits {
            false => WINMODE_A::B_0X0,
            true => WINMODE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WINMODE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WINMODE_A::B_0X1
    }
}
impl core::ops::Deref for WINMODE_R {
    type Target = crate::FieldReader<bool, WINMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WINMODE` writer - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
pub struct WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WINMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WINMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Signal selected with INPSEL\[1:0\]
    ///bitfield of this register
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WINMODE_A::B_0X0)
    }
    ///COMP2_INP signal of the comparator 2 (required for window mode, see Figure 64)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WINMODE_A::B_0X1)
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
///Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINOUT_A {
    ///0: COMP1_VALUE
    B_0X0 = 0,
    ///1: COMP1_VALUE XOR COMP2_VALUE (required for window mode, see Figure 64)
    B_0X1 = 1,
}
impl From<WINOUT_A> for bool {
    #[inline(always)]
    fn from(variant: WINOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WINOUT` reader - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
pub struct WINOUT_R(crate::FieldReader<bool, WINOUT_A>);
impl WINOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WINOUT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WINOUT_A {
        match self.bits {
            false => WINOUT_A::B_0X0,
            true => WINOUT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WINOUT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WINOUT_A::B_0X1
    }
}
impl core::ops::Deref for WINOUT_R {
    type Target = crate::FieldReader<bool, WINOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WINOUT` writer - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
pub struct WINOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WINOUT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WINOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COMP1_VALUE
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WINOUT_A::B_0X0)
    }
    ///COMP1_VALUE XOR COMP2_VALUE (required for window mode, see Figure 64)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WINOUT_A::B_0X1)
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
///Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITY_A {
    ///0: Non-inverted
    B_0X0 = 0,
    ///1: Inverted
    B_0X1 = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `POLARITY` reader - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
pub struct POLARITY_R(crate::FieldReader<bool, POLARITY_A>);
impl POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        POLARITY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::B_0X0,
            true => POLARITY_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == POLARITY_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == POLARITY_A::B_0X1
    }
}
impl core::ops::Deref for POLARITY_R {
    type Target = crate::FieldReader<bool, POLARITY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POLARITY` writer - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Non-inverted
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(POLARITY_A::B_0X0)
    }
    ///Inverted
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(POLARITY_A::B_0X1)
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
///Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYST_A {
    ///0: None
    B_0X0 = 0,
    ///1: Low
    B_0X1 = 1,
    ///2: Medium
    B_0X2 = 2,
    ///3: High
    B_0X3 = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
///Field `HYST` reader - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
pub struct HYST_R(crate::FieldReader<u8, HYST_A>);
impl HYST_R {
    pub(crate) fn new(bits: u8) -> Self {
        HYST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::B_0X0,
            1 => HYST_A::B_0X1,
            2 => HYST_A::B_0X2,
            3 => HYST_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HYST_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HYST_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == HYST_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == HYST_A::B_0X3
    }
}
impl core::ops::Deref for HYST_R {
    type Target = crate::FieldReader<u8, HYST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HYST` writer - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///None
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HYST_A::B_0X0)
    }
    ///Low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HYST_A::B_0X1)
    }
    ///Medium
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(HYST_A::B_0X2)
    }
    ///High
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(HYST_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
///Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRMODE_A {
    ///0: High speed
    B_0X0 = 0,
    ///1: Medium speed
    B_0X1 = 1,
}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        variant as _
    }
}
///Field `PWRMODE` reader - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
pub struct PWRMODE_R(crate::FieldReader<u8, PWRMODE_A>);
impl PWRMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWRMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRMODE_A> {
        match self.bits {
            0 => Some(PWRMODE_A::B_0X0),
            1 => Some(PWRMODE_A::B_0X1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PWRMODE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PWRMODE_A::B_0X1
    }
}
impl core::ops::Deref for PWRMODE_R {
    type Target = crate::FieldReader<u8, PWRMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PWRMODE` writer - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
pub struct PWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PWRMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///High speed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PWRMODE_A::B_0X0)
    }
    ///Medium speed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PWRMODE_A::B_0X1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLANKSEL_A {
    ///0: None (no blanking)
    B_0X0 = 0,
}
impl From<BLANKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKSEL_A) -> Self {
        variant as _
    }
}
///Field `BLANKSEL` reader - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
pub struct BLANKSEL_R(crate::FieldReader<u8, BLANKSEL_A>);
impl BLANKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLANKSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BLANKSEL_A> {
        match self.bits {
            0 => Some(BLANKSEL_A::B_0X0),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BLANKSEL_A::B_0X0
    }
}
impl core::ops::Deref for BLANKSEL_R {
    type Target = crate::FieldReader<u8, BLANKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BLANKSEL` writer - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
pub struct BLANKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLANKSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BLANKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///None (no blanking)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BLANKSEL_A::B_0X0)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
///Field `VALUE` reader - Comparator 1 output status This bit is read-only. It reflects the level of the comparator 1 output after the polarity selector and blanking, as indicated in .
pub struct VALUE_R(crate::FieldReader<bool, bool>);
impl VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    ///0: COMP1_CSR\[31:0\]
    ///register read/write bits can be written by software
    B_0X0 = 0,
    ///1: COMP1_CSR\[31:0\]
    ///register bits can be read but not written by software
    B_0X1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::B_0X0,
            true => LOCK_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LOCK_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LOCK_A::B_0X1
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LOCK` writer - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COMP1_CSR\[31:0\]
    ///register read/write bits can be written by software
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LOCK_A::B_0X0)
    }
    ///COMP1_CSR\[31:0\]
    ///register bits can be read but not written by software
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LOCK_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bit 0 - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 4:7 - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 11 - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 14 - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bits 18:19 - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 20:24 - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bit 30 - Comparator 1 output status This bit is read-only. It reflects the level of the comparator 1 output after the polarity selector and blanking, as indicated in .
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable bit This bit is controlled by software (if not locked). It enables the comparator 1:
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    ///Bits 4:7 - Comparator 1 signal selector for inverting input INM This bitfield is controlled by software (if not locked). It selects the signal for the inverting input COMP1_INM of the comparator 1: > 1000: 1/4 VREFINT
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W {
        INMSEL_W { w: self }
    }
    ///Bits 8:9 - Comparator 1 signal selector for non-inverting input This bitfield is controlled by software (if not locked). It selects the signal for the non-inverting input COMP1_INP of the comparator 1 (also see the WINMODE bit):
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
    ///Bit 11 - Comparator 1 non-inverting input selector for window mode This bit is controlled by software (if not locked). It selects the signal for COMP1_INP input of the comparator 1:
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W {
        WINMODE_W { w: self }
    }
    ///Bit 14 - Comparator 1 output selector This bit is controlled by software (if not locked). It selects the comparator 1 output:
    #[inline(always)]
    pub fn winout(&mut self) -> WINOUT_W {
        WINOUT_W { w: self }
    }
    ///Bit 15 - Comparator 1 polarity selector This bit is controlled by software (if not locked). It selects the comparator 1 output polarity:
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    ///Bits 16:17 - Comparator 1 hysteresis selector This bitfield is controlled by software (if not locked). It selects the hysteresis of the comparator 1:
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    ///Bits 18:19 - Comparator 1 power mode selector This bitfield is controlled by software (if not locked). It selects the power consumption and as a consequence the speed of the comparator 1: others: Reserved
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W {
        PWRMODE_W { w: self }
    }
    ///Bits 20:24 - Comparator 1 blanking source selector This bitfield is controlled by software (if not locked). It selects the blanking source: xxxx1: TIM1 OC4 xxx1x: TIM1 OC5 xx1xx: TIM2 OC3 x1xxx: TIM3 OC3 1xxxx: TIM15 OC2
    #[inline(always)]
    pub fn blanksel(&mut self) -> BLANKSEL_W {
        BLANKSEL_W { w: self }
    }
    ///Bit 31 - COMP1_CSR register lock This bit is set by software and cleared by a system reset. It locks the whole content of the comparator 1 control register COMP1_CSR\[31:0\]:
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Comparator 1 control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp1_csr](index.html) module
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp1_csr::R](R) reader structure
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp1_csr::W](W) writer structure
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
}
///`reset()` method sets COMP1_CSR to value 0
impl crate::Resettable for COMP1_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
