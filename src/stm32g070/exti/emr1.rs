///Register `EMR1` reader
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EMR1` writer
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
///CPU wakeup with event mask on event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM0_A {
    ///0: Interrupt request line is masked
    MASKED = 0,
    ///1: Interrupt request line is unmasked
    UNMASKED = 1,
}
impl From<EM0_A> for bool {
    #[inline(always)]
    fn from(variant: EM0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EM0` reader - CPU wakeup with event mask on event input
pub struct EM0_R(crate::FieldReader<bool, EM0_A>);
impl EM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EM0_A {
        match self.bits {
            false => EM0_A::MASKED,
            true => EM0_A::UNMASKED,
        }
    }
    ///Checks if the value of the field is `MASKED`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == EM0_A::MASKED
    }
    ///Checks if the value of the field is `UNMASKED`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == EM0_A::UNMASKED
    }
}
impl core::ops::Deref for EM0_R {
    type Target = crate::FieldReader<bool, EM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EM0` writer - CPU wakeup with event mask on event input
pub struct EM0_W<'a> {
    w: &'a mut W,
}
impl<'a> EM0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM0_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM0_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM1_A = EM0_A;
///Field `EM1` reader - CPU wakeup with event mask on event input
pub type EM1_R = EM0_R;
///Field `EM1` writer - CPU wakeup with event mask on event input
pub struct EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EM1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM1_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM1_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
///CPU wakeup with event mask on event input
pub type EM2_A = EM0_A;
///Field `EM2` reader - CPU wakeup with event mask on event input
pub type EM2_R = EM0_R;
///Field `EM2` writer - CPU wakeup with event mask on event input
pub struct EM2_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM2_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM2_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///CPU wakeup with event mask on event input
pub type EM3_A = EM0_A;
///Field `EM3` reader - CPU wakeup with event mask on event input
pub type EM3_R = EM0_R;
///Field `EM3` writer - CPU wakeup with event mask on event input
pub struct EM3_W<'a> {
    w: &'a mut W,
}
impl<'a> EM3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM3_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM3_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM4_A = EM0_A;
///Field `EM4` reader - CPU wakeup with event mask on event input
pub type EM4_R = EM0_R;
///Field `EM4` writer - CPU wakeup with event mask on event input
pub struct EM4_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM4_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM4_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM5_A = EM0_A;
///Field `EM5` reader - CPU wakeup with event mask on event input
pub type EM5_R = EM0_R;
///Field `EM5` writer - CPU wakeup with event mask on event input
pub struct EM5_W<'a> {
    w: &'a mut W,
}
impl<'a> EM5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM5_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM5_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM6_A = EM0_A;
///Field `EM6` reader - CPU wakeup with event mask on event input
pub type EM6_R = EM0_R;
///Field `EM6` writer - CPU wakeup with event mask on event input
pub struct EM6_W<'a> {
    w: &'a mut W,
}
impl<'a> EM6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM6_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM6_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM7_A = EM0_A;
///Field `EM7` reader - CPU wakeup with event mask on event input
pub type EM7_R = EM0_R;
///Field `EM7` writer - CPU wakeup with event mask on event input
pub struct EM7_W<'a> {
    w: &'a mut W,
}
impl<'a> EM7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM7_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM7_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM8_A = EM0_A;
///Field `EM8` reader - CPU wakeup with event mask on event input
pub type EM8_R = EM0_R;
///Field `EM8` writer - CPU wakeup with event mask on event input
pub struct EM8_W<'a> {
    w: &'a mut W,
}
impl<'a> EM8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM8_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM8_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM9_A = EM0_A;
///Field `EM9` reader - CPU wakeup with event mask on event input
pub type EM9_R = EM0_R;
///Field `EM9` writer - CPU wakeup with event mask on event input
pub struct EM9_W<'a> {
    w: &'a mut W,
}
impl<'a> EM9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM9_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM9_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM10_A = EM0_A;
///Field `EM10` reader - CPU wakeup with event mask on event input
pub type EM10_R = EM0_R;
///Field `EM10` writer - CPU wakeup with event mask on event input
pub struct EM10_W<'a> {
    w: &'a mut W,
}
impl<'a> EM10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM10_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM10_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM11_A = EM0_A;
///Field `EM11` reader - CPU wakeup with event mask on event input
pub type EM11_R = EM0_R;
///Field `EM11` writer - CPU wakeup with event mask on event input
pub struct EM11_W<'a> {
    w: &'a mut W,
}
impl<'a> EM11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM11_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM11_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM12_A = EM0_A;
///Field `EM12` reader - CPU wakeup with event mask on event input
pub type EM12_R = EM0_R;
///Field `EM12` writer - CPU wakeup with event mask on event input
pub struct EM12_W<'a> {
    w: &'a mut W,
}
impl<'a> EM12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM12_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM12_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM13_A = EM0_A;
///Field `EM13` reader - CPU wakeup with event mask on event input
pub type EM13_R = EM0_R;
///Field `EM13` writer - CPU wakeup with event mask on event input
pub struct EM13_W<'a> {
    w: &'a mut W,
}
impl<'a> EM13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM13_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM13_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM14_A = EM0_A;
///Field `EM14` reader - CPU wakeup with event mask on event input
pub type EM14_R = EM0_R;
///Field `EM14` writer - CPU wakeup with event mask on event input
pub struct EM14_W<'a> {
    w: &'a mut W,
}
impl<'a> EM14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM14_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM14_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM15_A = EM0_A;
///Field `EM15` reader - CPU wakeup with event mask on event input
pub type EM15_R = EM0_R;
///Field `EM15` writer - CPU wakeup with event mask on event input
pub struct EM15_W<'a> {
    w: &'a mut W,
}
impl<'a> EM15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM15_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM15_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM16_A = EM0_A;
///Field `EM16` reader - CPU wakeup with event mask on event input
pub type EM16_R = EM0_R;
///Field `EM16` writer - CPU wakeup with event mask on event input
pub struct EM16_W<'a> {
    w: &'a mut W,
}
impl<'a> EM16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM16_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM16_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM17_A = EM0_A;
///Field `EM17` reader - CPU wakeup with event mask on event input
pub type EM17_R = EM0_R;
///Field `EM17` writer - CPU wakeup with event mask on event input
pub struct EM17_W<'a> {
    w: &'a mut W,
}
impl<'a> EM17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM17_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM17_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM18_A = EM0_A;
///Field `EM18` reader - CPU wakeup with event mask on event input
pub type EM18_R = EM0_R;
///Field `EM18` writer - CPU wakeup with event mask on event input
pub struct EM18_W<'a> {
    w: &'a mut W,
}
impl<'a> EM18_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM18_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM18_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///CPU wakeup with event mask on event input
pub type EM19_A = EM0_A;
///Field `EM19` reader - CPU wakeup with event mask on event input
pub type EM19_R = EM0_R;
///Field `EM19` writer - CPU wakeup with event mask on event input
pub struct EM19_W<'a> {
    w: &'a mut W,
}
impl<'a> EM19_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM19_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM19_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM19_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM21_A = EM0_A;
///Field `EM21` reader - CPU wakeup with event mask on event input
pub type EM21_R = EM0_R;
///Field `EM21` writer - CPU wakeup with event mask on event input
pub struct EM21_W<'a> {
    w: &'a mut W,
}
impl<'a> EM21_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM21_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM21_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///CPU wakeup with event mask on event input
pub type EM23_A = EM0_A;
///Field `EM23` reader - CPU wakeup with event mask on event input
pub type EM23_R = EM0_R;
///Field `EM23` writer - CPU wakeup with event mask on event input
pub struct EM23_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM23_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM23_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM25_A = EM0_A;
///Field `EM25` reader - CPU wakeup with event mask on event input
pub type EM25_R = EM0_R;
///Field `EM25` writer - CPU wakeup with event mask on event input
pub struct EM25_W<'a> {
    w: &'a mut W,
}
impl<'a> EM25_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM25_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM25_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM26_A = EM0_A;
///Field `EM26` reader - CPU wakeup with event mask on event input
pub type EM26_R = EM0_R;
///Field `EM26` writer - CPU wakeup with event mask on event input
pub struct EM26_W<'a> {
    w: &'a mut W,
}
impl<'a> EM26_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM26_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM26_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM27_A = EM0_A;
///Field `EM27` reader - CPU wakeup with event mask on event input
pub type EM27_R = EM0_R;
///Field `EM27` writer - CPU wakeup with event mask on event input
pub struct EM27_W<'a> {
    w: &'a mut W,
}
impl<'a> EM27_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM27_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM27_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///CPU wakeup with event mask on event input
pub type EM28_A = EM0_A;
///Field `EM28` reader - CPU wakeup with event mask on event input
pub type EM28_R = EM0_R;
///Field `EM28` writer - CPU wakeup with event mask on event input
pub struct EM28_W<'a> {
    w: &'a mut W,
}
impl<'a> EM28_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM28_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM28_A::UNMASKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///CPU wakeup with event mask on event input
pub type EM29_A = EM0_A;
///Field `EM29` reader - CPU wakeup with event mask on event input
pub type EM29_R = EM0_R;
///Field `EM29` writer - CPU wakeup with event mask on event input
pub struct EM29_W<'a> {
    w: &'a mut W,
}
impl<'a> EM29_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM29_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM29_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM30_A = EM0_A;
///Field `EM30` reader - CPU wakeup with event mask on event input
pub type EM30_R = EM0_R;
///Field `EM30` writer - CPU wakeup with event mask on event input
pub struct EM30_W<'a> {
    w: &'a mut W,
}
impl<'a> EM30_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM30_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM30_A::UNMASKED)
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
///CPU wakeup with event mask on event input
pub type EM31_A = EM0_A;
///Field `EM31` reader - CPU wakeup with event mask on event input
pub type EM31_R = EM0_R;
///Field `EM31` writer - CPU wakeup with event mask on event input
pub struct EM31_W<'a> {
    w: &'a mut W,
}
impl<'a> EM31_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EM31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM31_A::MASKED)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM31_A::UNMASKED)
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
    ///Bit 0 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 21 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 23 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 25 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em25(&self) -> EM25_R {
        EM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em26(&self) -> EM26_R {
        EM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em27(&self) -> EM27_R {
        EM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em28(&self) -> EM28_R {
        EM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em29(&self) -> EM29_R {
        EM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em30(&self) -> EM30_R {
        EM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em31(&self) -> EM31_R {
        EM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W {
        EM0_W { w: self }
    }
    ///Bit 1 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W {
        EM1_W { w: self }
    }
    ///Bit 2 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W {
        EM2_W { w: self }
    }
    ///Bit 3 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W {
        EM3_W { w: self }
    }
    ///Bit 4 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W {
        EM4_W { w: self }
    }
    ///Bit 5 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W {
        EM5_W { w: self }
    }
    ///Bit 6 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W {
        EM6_W { w: self }
    }
    ///Bit 7 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W {
        EM7_W { w: self }
    }
    ///Bit 8 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W {
        EM8_W { w: self }
    }
    ///Bit 9 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W {
        EM9_W { w: self }
    }
    ///Bit 10 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W {
        EM10_W { w: self }
    }
    ///Bit 11 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W {
        EM11_W { w: self }
    }
    ///Bit 12 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W {
        EM12_W { w: self }
    }
    ///Bit 13 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W {
        EM13_W { w: self }
    }
    ///Bit 14 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W {
        EM14_W { w: self }
    }
    ///Bit 15 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W {
        EM15_W { w: self }
    }
    ///Bit 16 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em16(&mut self) -> EM16_W {
        EM16_W { w: self }
    }
    ///Bit 17 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W {
        EM17_W { w: self }
    }
    ///Bit 18 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em18(&mut self) -> EM18_W {
        EM18_W { w: self }
    }
    ///Bit 19 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W {
        EM19_W { w: self }
    }
    ///Bit 21 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em21(&mut self) -> EM21_W {
        EM21_W { w: self }
    }
    ///Bit 23 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em23(&mut self) -> EM23_W {
        EM23_W { w: self }
    }
    ///Bit 25 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em25(&mut self) -> EM25_W {
        EM25_W { w: self }
    }
    ///Bit 26 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em26(&mut self) -> EM26_W {
        EM26_W { w: self }
    }
    ///Bit 27 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em27(&mut self) -> EM27_W {
        EM27_W { w: self }
    }
    ///Bit 28 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em28(&mut self) -> EM28_W {
        EM28_W { w: self }
    }
    ///Bit 29 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em29(&mut self) -> EM29_W {
        EM29_W { w: self }
    }
    ///Bit 30 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em30(&mut self) -> EM30_W {
        EM30_W { w: self }
    }
    ///Bit 31 - CPU wakeup with event mask on event input
    #[inline(always)]
    pub fn em31(&mut self) -> EM31_W {
        EM31_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI CPU wakeup with event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr1](index.html) module
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [emr1::R](R) reader structure
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [emr1::W](W) writer structure
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
}
///`reset()` method sets EMR1 to value 0
impl crate::Resettable for EMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
