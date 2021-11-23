///Register `SWIER1` reader
pub struct R(crate::R<SWIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWIER1` writer
pub struct W(crate::W<SWIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER1_SPEC>;
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
impl From<crate::W<SWIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER1_SPEC>) -> Self {
        W(writer)
    }
}
///Rising trigger event configuration bit of Configurable Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER0_A {
    ///1: Generates an interrupt request
    PEND = 1,
}
impl From<SWIER0_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SWIER0` reader - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER0_R(crate::FieldReader<bool, SWIER0_A>);
impl SWIER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWIER0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER0_A> {
        match self.bits {
            true => Some(SWIER0_A::PEND),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PEND`
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        **self == SWIER0_A::PEND
    }
}
impl core::ops::Deref for SWIER0_R {
    type Target = crate::FieldReader<bool, SWIER0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SWIER0` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER0_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER1_A = SWIER0_A;
///Field `SWIER1` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER1_R = SWIER0_R;
///Field `SWIER1` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER1_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER2_A = SWIER0_A;
///Field `SWIER2` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER2_R = SWIER0_R;
///Field `SWIER2` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER2_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER3_A = SWIER0_A;
///Field `SWIER3` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER3_R = SWIER0_R;
///Field `SWIER3` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER3_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER3_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER4_A = SWIER0_A;
///Field `SWIER4` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER4_R = SWIER0_R;
///Field `SWIER4` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER4_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER4_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER5_A = SWIER0_A;
///Field `SWIER5` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER5_R = SWIER0_R;
///Field `SWIER5` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER5_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER5_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER6_A = SWIER0_A;
///Field `SWIER6` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER6_R = SWIER0_R;
///Field `SWIER6` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER6_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER6_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER7_A = SWIER0_A;
///Field `SWIER7` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER7_R = SWIER0_R;
///Field `SWIER7` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER7_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER7_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER8_A = SWIER0_A;
///Field `SWIER8` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER8_R = SWIER0_R;
///Field `SWIER8` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER8_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER8_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER9_A = SWIER0_A;
///Field `SWIER9` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER9_R = SWIER0_R;
///Field `SWIER9` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER9_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER9_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER10_A = SWIER0_A;
///Field `SWIER10` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER10_R = SWIER0_R;
///Field `SWIER10` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER10_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER10_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER11_A = SWIER0_A;
///Field `SWIER11` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER11_R = SWIER0_R;
///Field `SWIER11` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER11_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER11_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER12_A = SWIER0_A;
///Field `SWIER12` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER12_R = SWIER0_R;
///Field `SWIER12` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER12_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER12_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER13_A = SWIER0_A;
///Field `SWIER13` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER13_R = SWIER0_R;
///Field `SWIER13` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER13_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER13_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER14_A = SWIER0_A;
///Field `SWIER14` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER14_R = SWIER0_R;
///Field `SWIER14` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER14_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER14_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER15_A = SWIER0_A;
///Field `SWIER15` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER15_R = SWIER0_R;
///Field `SWIER15` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER15_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER15_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER16_A = SWIER0_A;
///Field `SWIER16` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER16_R = SWIER0_R;
///Field `SWIER16` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER16_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER16_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER17_A = SWIER0_A;
///Field `SWIER17` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER17_R = SWIER0_R;
///Field `SWIER17` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER17_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER17_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER17_A::PEND)
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
///Rising trigger event configuration bit of Configurable Event input
pub type SWIER18_A = SWIER0_A;
///Field `SWIER18` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER18_R = SWIER0_R;
///Field `SWIER18` writer - Rising trigger event configuration bit of Configurable Event input
pub struct SWIER18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER18_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWIER18_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER18_A::PEND)
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
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER0_W {
        SWIER0_W { w: self }
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER1_W {
        SWIER1_W { w: self }
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER2_W {
        SWIER2_W { w: self }
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER3_W {
        SWIER3_W { w: self }
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER4_W {
        SWIER4_W { w: self }
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER5_W {
        SWIER5_W { w: self }
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER6_W {
        SWIER6_W { w: self }
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER7_W {
        SWIER7_W { w: self }
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER8_W {
        SWIER8_W { w: self }
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER9_W {
        SWIER9_W { w: self }
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER10_W {
        SWIER10_W { w: self }
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER11_W {
        SWIER11_W { w: self }
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER12_W {
        SWIER12_W { w: self }
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER13_W {
        SWIER13_W { w: self }
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER14_W {
        SWIER14_W { w: self }
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER15_W {
        SWIER15_W { w: self }
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER16_W {
        SWIER16_W { w: self }
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier17(&mut self) -> SWIER17_W {
        SWIER17_W { w: self }
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier18(&mut self) -> SWIER18_W {
        SWIER18_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI software interrupt event register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier1](index.html) module
pub struct SWIER1_SPEC;
impl crate::RegisterSpec for SWIER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [swier1::R](R) reader structure
impl crate::Readable for SWIER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swier1::W](W) writer structure
impl crate::Writable for SWIER1_SPEC {
    type Writer = W;
}
///`reset()` method sets SWIER1 to value 0
impl crate::Resettable for SWIER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
