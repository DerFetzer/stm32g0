///Register `FPR1` reader
pub struct R(crate::R<FPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPR1` writer
pub struct W(crate::W<FPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR1_SPEC>;
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
impl From<crate::W<FPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR1_SPEC>) -> Self {
        W(writer)
    }
}
///configurable event inputs x falling edge pending bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPIF0_A {
    ///0: No trigger request occurred
    NOTPENDING = 0,
    ///1: Selected trigger request occurred
    PENDING = 1,
}
impl From<FPIF0_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF0` reader - configurable event inputs x falling edge pending bit.
pub struct FPIF0_R(crate::FieldReader<bool, FPIF0_A>);
impl FPIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPIF0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPIF0_A {
        match self.bits {
            false => FPIF0_A::NOTPENDING,
            true => FPIF0_A::PENDING,
        }
    }
    ///Checks if the value of the field is `NOTPENDING`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == FPIF0_A::NOTPENDING
    }
    ///Checks if the value of the field is `PENDING`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == FPIF0_A::PENDING
    }
}
impl core::ops::Deref for FPIF0_R {
    type Target = crate::FieldReader<bool, FPIF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///configurable event inputs x falling edge pending bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPIF0_AW {
    ///1: Clears pending bit
    CLEAR = 1,
}
impl From<FPIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: FPIF0_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF0` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF0_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF1_A = FPIF0_A;
///Field `FPIF1` reader - configurable event inputs x falling edge pending bit.
pub type FPIF1_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF1_AW = FPIF0_AW;
///Field `FPIF1` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF1_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF2_A = FPIF0_A;
///Field `FPIF2` reader - configurable event inputs x falling edge pending bit.
pub type FPIF2_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF2_AW = FPIF0_AW;
///Field `FPIF2` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF2_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF3_A = FPIF0_A;
///Field `FPIF3` reader - configurable event inputs x falling edge pending bit.
pub type FPIF3_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF3_AW = FPIF0_AW;
///Field `FPIF3` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF3_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF4_A = FPIF0_A;
///Field `FPIF4` reader - configurable event inputs x falling edge pending bit.
pub type FPIF4_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF4_AW = FPIF0_AW;
///Field `FPIF4` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF4_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF5_A = FPIF0_A;
///Field `FPIF5` reader - configurable event inputs x falling edge pending bit.
pub type FPIF5_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF5_AW = FPIF0_AW;
///Field `FPIF5` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF5_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF6_A = FPIF0_A;
///Field `FPIF6` reader - configurable event inputs x falling edge pending bit.
pub type FPIF6_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF6_AW = FPIF0_AW;
///Field `FPIF6` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF6_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF7_A = FPIF0_A;
///Field `FPIF7` reader - configurable event inputs x falling edge pending bit.
pub type FPIF7_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF7_AW = FPIF0_AW;
///Field `FPIF7` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF7_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF8_A = FPIF0_A;
///Field `FPIF8` reader - configurable event inputs x falling edge pending bit.
pub type FPIF8_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF8_AW = FPIF0_AW;
///Field `FPIF8` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF8_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF9_A = FPIF0_A;
///Field `FPIF9` reader - configurable event inputs x falling edge pending bit.
pub type FPIF9_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF9_AW = FPIF0_AW;
///Field `FPIF9` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF9_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF10_A = FPIF0_A;
///Field `FPIF10` reader - configurable event inputs x falling edge pending bit.
pub type FPIF10_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF10_AW = FPIF0_AW;
///Field `FPIF10` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF10_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF11_A = FPIF0_A;
///Field `FPIF11` reader - configurable event inputs x falling edge pending bit.
pub type FPIF11_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF11_AW = FPIF0_AW;
///Field `FPIF11` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF11_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF12_A = FPIF0_A;
///Field `FPIF12` reader - configurable event inputs x falling edge pending bit.
pub type FPIF12_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF12_AW = FPIF0_AW;
///Field `FPIF12` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF12_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF13_A = FPIF0_A;
///Field `FPIF13` reader - configurable event inputs x falling edge pending bit.
pub type FPIF13_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF13_AW = FPIF0_AW;
///Field `FPIF13` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF13_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF14_A = FPIF0_A;
///Field `FPIF14` reader - configurable event inputs x falling edge pending bit.
pub type FPIF14_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF14_AW = FPIF0_AW;
///Field `FPIF14` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF14_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF15_A = FPIF0_A;
///Field `FPIF15` reader - configurable event inputs x falling edge pending bit.
pub type FPIF15_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF15_AW = FPIF0_AW;
///Field `FPIF15` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF15_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF16_A = FPIF0_A;
///Field `FPIF16` reader - configurable event inputs x falling edge pending bit.
pub type FPIF16_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF16_AW = FPIF0_AW;
///Field `FPIF16` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF16_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF16_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF17_A = FPIF0_A;
///Field `FPIF17` reader - configurable event inputs x falling edge pending bit.
pub type FPIF17_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF17_AW = FPIF0_AW;
///Field `FPIF17` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF17_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF17_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF17_AW::CLEAR)
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
///configurable event inputs x falling edge pending bit.
pub type FPIF18_A = FPIF0_A;
///Field `FPIF18` reader - configurable event inputs x falling edge pending bit.
pub type FPIF18_R = FPIF0_R;
///configurable event inputs x falling edge pending bit.
pub type FPIF18_AW = FPIF0_AW;
///Field `FPIF18` writer - configurable event inputs x falling edge pending bit.
pub struct FPIF18_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF18_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FPIF18_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF18_AW::CLEAR)
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
impl R {
    ///Bit 0 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif17(&self) -> FPIF17_R {
        FPIF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif18(&self) -> FPIF18_R {
        FPIF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif0(&mut self) -> FPIF0_W {
        FPIF0_W { w: self }
    }
    ///Bit 1 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif1(&mut self) -> FPIF1_W {
        FPIF1_W { w: self }
    }
    ///Bit 2 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W {
        FPIF2_W { w: self }
    }
    ///Bit 3 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif3(&mut self) -> FPIF3_W {
        FPIF3_W { w: self }
    }
    ///Bit 4 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif4(&mut self) -> FPIF4_W {
        FPIF4_W { w: self }
    }
    ///Bit 5 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif5(&mut self) -> FPIF5_W {
        FPIF5_W { w: self }
    }
    ///Bit 6 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif6(&mut self) -> FPIF6_W {
        FPIF6_W { w: self }
    }
    ///Bit 7 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif7(&mut self) -> FPIF7_W {
        FPIF7_W { w: self }
    }
    ///Bit 8 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif8(&mut self) -> FPIF8_W {
        FPIF8_W { w: self }
    }
    ///Bit 9 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif9(&mut self) -> FPIF9_W {
        FPIF9_W { w: self }
    }
    ///Bit 10 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif10(&mut self) -> FPIF10_W {
        FPIF10_W { w: self }
    }
    ///Bit 11 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif11(&mut self) -> FPIF11_W {
        FPIF11_W { w: self }
    }
    ///Bit 12 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif12(&mut self) -> FPIF12_W {
        FPIF12_W { w: self }
    }
    ///Bit 13 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif13(&mut self) -> FPIF13_W {
        FPIF13_W { w: self }
    }
    ///Bit 14 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif14(&mut self) -> FPIF14_W {
        FPIF14_W { w: self }
    }
    ///Bit 15 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif15(&mut self) -> FPIF15_W {
        FPIF15_W { w: self }
    }
    ///Bit 16 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif16(&mut self) -> FPIF16_W {
        FPIF16_W { w: self }
    }
    ///Bit 17 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif17(&mut self) -> FPIF17_W {
        FPIF17_W { w: self }
    }
    ///Bit 18 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif18(&mut self) -> FPIF18_W {
        FPIF18_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling edge pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpr1](index.html) module
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpr1::R](R) reader structure
impl crate::Readable for FPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpr1::W](W) writer structure
impl crate::Writable for FPR1_SPEC {
    type Writer = W;
}
///`reset()` method sets FPR1 to value 0
impl crate::Resettable for FPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
