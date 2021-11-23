///Register `IMR` reader
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IMR` writer
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
///TXIS interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXISIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TXISIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXISIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXISIE` reader - TXIS interrupt enable
pub struct TXISIE_R(crate::FieldReader<bool, TXISIE_A>);
impl TXISIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXISIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXISIE_A {
        match self.bits {
            false => TXISIE_A::B_0X0,
            true => TXISIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXISIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXISIE_A::B_0X1
    }
}
impl core::ops::Deref for TXISIE_R {
    type Target = crate::FieldReader<bool, TXISIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXISIE` writer - TXIS interrupt enable
pub struct TXISIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXISIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXISIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXISIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXISIE_A::B_0X1)
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
///TXMSGDISC interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSGDISCIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TXMSGDISCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSGDISCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGDISCIE` reader - TXMSGDISC interrupt enable
pub struct TXMSGDISCIE_R(crate::FieldReader<bool, TXMSGDISCIE_A>);
impl TXMSGDISCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGDISCIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXMSGDISCIE_A {
        match self.bits {
            false => TXMSGDISCIE_A::B_0X0,
            true => TXMSGDISCIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMSGDISCIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMSGDISCIE_A::B_0X1
    }
}
impl core::ops::Deref for TXMSGDISCIE_R {
    type Target = crate::FieldReader<bool, TXMSGDISCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXMSGDISCIE` writer - TXMSGDISC interrupt enable
pub struct TXMSGDISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGDISCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXMSGDISCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXMSGDISCIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXMSGDISCIE_A::B_0X1)
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
///TXMSGSENT interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSGSENTIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TXMSGSENTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSGSENTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGSENTIE` reader - TXMSGSENT interrupt enable
pub struct TXMSGSENTIE_R(crate::FieldReader<bool, TXMSGSENTIE_A>);
impl TXMSGSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGSENTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXMSGSENTIE_A {
        match self.bits {
            false => TXMSGSENTIE_A::B_0X0,
            true => TXMSGSENTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMSGSENTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMSGSENTIE_A::B_0X1
    }
}
impl core::ops::Deref for TXMSGSENTIE_R {
    type Target = crate::FieldReader<bool, TXMSGSENTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXMSGSENTIE` writer - TXMSGSENT interrupt enable
pub struct TXMSGSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGSENTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXMSGSENTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXMSGSENTIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXMSGSENTIE_A::B_0X1)
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
///TXMSGABT interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSGABTIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TXMSGABTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXMSGABTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXMSGABTIE` reader - TXMSGABT interrupt enable
pub struct TXMSGABTIE_R(crate::FieldReader<bool, TXMSGABTIE_A>);
impl TXMSGABTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMSGABTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXMSGABTIE_A {
        match self.bits {
            false => TXMSGABTIE_A::B_0X0,
            true => TXMSGABTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXMSGABTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXMSGABTIE_A::B_0X1
    }
}
impl core::ops::Deref for TXMSGABTIE_R {
    type Target = crate::FieldReader<bool, TXMSGABTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXMSGABTIE` writer - TXMSGABT interrupt enable
pub struct TXMSGABTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGABTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXMSGABTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXMSGABTIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXMSGABTIE_A::B_0X1)
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
///HRSTDISC interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSTDISCIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<HRSTDISCIE_A> for bool {
    #[inline(always)]
    fn from(variant: HRSTDISCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HRSTDISCIE` reader - HRSTDISC interrupt enable
pub struct HRSTDISCIE_R(crate::FieldReader<bool, HRSTDISCIE_A>);
impl HRSTDISCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTDISCIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HRSTDISCIE_A {
        match self.bits {
            false => HRSTDISCIE_A::B_0X0,
            true => HRSTDISCIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HRSTDISCIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HRSTDISCIE_A::B_0X1
    }
}
impl core::ops::Deref for HRSTDISCIE_R {
    type Target = crate::FieldReader<bool, HRSTDISCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HRSTDISCIE` writer - HRSTDISC interrupt enable
pub struct HRSTDISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTDISCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HRSTDISCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HRSTDISCIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HRSTDISCIE_A::B_0X1)
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
///HRSTSENT interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSTSENTIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<HRSTSENTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HRSTSENTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HRSTSENTIE` reader - HRSTSENT interrupt enable
pub struct HRSTSENTIE_R(crate::FieldReader<bool, HRSTSENTIE_A>);
impl HRSTSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRSTSENTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HRSTSENTIE_A {
        match self.bits {
            false => HRSTSENTIE_A::B_0X0,
            true => HRSTSENTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HRSTSENTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HRSTSENTIE_A::B_0X1
    }
}
impl core::ops::Deref for HRSTSENTIE_R {
    type Target = crate::FieldReader<bool, HRSTSENTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HRSTSENTIE` writer - HRSTSENT interrupt enable
pub struct HRSTSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTSENTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HRSTSENTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HRSTSENTIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HRSTSENTIE_A::B_0X1)
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
///TXUND interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUNDIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TXUNDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXUNDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXUNDIE` reader - TXUND interrupt enable
pub struct TXUNDIE_R(crate::FieldReader<bool, TXUNDIE_A>);
impl TXUNDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXUNDIE_A {
        match self.bits {
            false => TXUNDIE_A::B_0X0,
            true => TXUNDIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXUNDIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXUNDIE_A::B_0X1
    }
}
impl core::ops::Deref for TXUNDIE_R {
    type Target = crate::FieldReader<bool, TXUNDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXUNDIE` writer - TXUND interrupt enable
pub struct TXUNDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXUNDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXUNDIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXUNDIE_A::B_0X1)
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
///RXNE interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RXNE interrupt enable
pub struct RXNEIE_R(crate::FieldReader<bool, RXNEIE_A>);
impl RXNEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNEIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::B_0X0,
            true => RXNEIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXNEIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXNEIE_A::B_0X1
    }
}
impl core::ops::Deref for RXNEIE_R {
    type Target = crate::FieldReader<bool, RXNEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXNEIE` writer - RXNE interrupt enable
pub struct RXNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNEIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXNEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXNEIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXNEIE_A::B_0X1)
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
///RXORDDET interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXORDDETIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<RXORDDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXORDDETIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXORDDETIE` reader - RXORDDET interrupt enable
pub struct RXORDDETIE_R(crate::FieldReader<bool, RXORDDETIE_A>);
impl RXORDDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXORDDETIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXORDDETIE_A {
        match self.bits {
            false => RXORDDETIE_A::B_0X0,
            true => RXORDDETIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXORDDETIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXORDDETIE_A::B_0X1
    }
}
impl core::ops::Deref for RXORDDETIE_R {
    type Target = crate::FieldReader<bool, RXORDDETIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXORDDETIE` writer - RXORDDET interrupt enable
pub struct RXORDDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDDETIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXORDDETIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXORDDETIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXORDDETIE_A::B_0X1)
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
///RXHRSTDET interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXHRSTDETIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<RXHRSTDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXHRSTDETIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXHRSTDETIE` reader - RXHRSTDET interrupt enable
pub struct RXHRSTDETIE_R(crate::FieldReader<bool, RXHRSTDETIE_A>);
impl RXHRSTDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXHRSTDETIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXHRSTDETIE_A {
        match self.bits {
            false => RXHRSTDETIE_A::B_0X0,
            true => RXHRSTDETIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXHRSTDETIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXHRSTDETIE_A::B_0X1
    }
}
impl core::ops::Deref for RXHRSTDETIE_R {
    type Target = crate::FieldReader<bool, RXHRSTDETIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXHRSTDETIE` writer - RXHRSTDET interrupt enable
pub struct RXHRSTDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXHRSTDETIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXHRSTDETIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXHRSTDETIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXHRSTDETIE_A::B_0X1)
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
///RXOVR interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVRIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<RXOVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOVRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXOVRIE` reader - RXOVR interrupt enable
pub struct RXOVRIE_R(crate::FieldReader<bool, RXOVRIE_A>);
impl RXOVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVRIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXOVRIE_A {
        match self.bits {
            false => RXOVRIE_A::B_0X0,
            true => RXOVRIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXOVRIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXOVRIE_A::B_0X1
    }
}
impl core::ops::Deref for RXOVRIE_R {
    type Target = crate::FieldReader<bool, RXOVRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXOVRIE` writer - RXOVR interrupt enable
pub struct RXOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXOVRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXOVRIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXOVRIE_A::B_0X1)
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
///RXMSGEND interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSGENDIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<RXMSGENDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXMSGENDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXMSGENDIE` reader - RXMSGEND interrupt enable
pub struct RXMSGENDIE_R(crate::FieldReader<bool, RXMSGENDIE_A>);
impl RXMSGENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMSGENDIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXMSGENDIE_A {
        match self.bits {
            false => RXMSGENDIE_A::B_0X0,
            true => RXMSGENDIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXMSGENDIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXMSGENDIE_A::B_0X1
    }
}
impl core::ops::Deref for RXMSGENDIE_R {
    type Target = crate::FieldReader<bool, RXMSGENDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXMSGENDIE` writer - RXMSGEND interrupt enable
pub struct RXMSGENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSGENDIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXMSGENDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXMSGENDIE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXMSGENDIE_A::B_0X1)
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
///Field `TYPECEVT1IE` reader - TYPECEVT1 interrupt enable
pub struct TYPECEVT1IE_R(crate::FieldReader<bool, bool>);
impl TYPECEVT1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TYPECEVT1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TYPECEVT1IE` writer - TYPECEVT1 interrupt enable
pub struct TYPECEVT1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT1IE_W<'a> {
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
///TYPECEVT2 interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPECEVT2IE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TYPECEVT2IE_A> for bool {
    #[inline(always)]
    fn from(variant: TYPECEVT2IE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TYPECEVT2IE` reader - TYPECEVT2 interrupt enable
pub struct TYPECEVT2IE_R(crate::FieldReader<bool, TYPECEVT2IE_A>);
impl TYPECEVT2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TYPECEVT2IE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TYPECEVT2IE_A {
        match self.bits {
            false => TYPECEVT2IE_A::B_0X0,
            true => TYPECEVT2IE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TYPECEVT2IE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TYPECEVT2IE_A::B_0X1
    }
}
impl core::ops::Deref for TYPECEVT2IE_R {
    type Target = crate::FieldReader<bool, TYPECEVT2IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TYPECEVT2IE` writer - TYPECEVT2 interrupt enable
pub struct TYPECEVT2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT2IE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TYPECEVT2IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TYPECEVT2IE_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TYPECEVT2IE_A::B_0X1)
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
///FRSEVT interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRSEVTIE_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<FRSEVTIE_A> for bool {
    #[inline(always)]
    fn from(variant: FRSEVTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRSEVTIE` reader - FRSEVT interrupt enable
pub struct FRSEVTIE_R(crate::FieldReader<bool, FRSEVTIE_A>);
impl FRSEVTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRSEVTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRSEVTIE_A {
        match self.bits {
            false => FRSEVTIE_A::B_0X0,
            true => FRSEVTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FRSEVTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FRSEVTIE_A::B_0X1
    }
}
impl core::ops::Deref for FRSEVTIE_R {
    type Target = crate::FieldReader<bool, FRSEVTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - TXIS interrupt enable
    #[inline(always)]
    pub fn txisie(&self) -> TXISIE_R {
        TXISIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TXMSGDISC interrupt enable
    #[inline(always)]
    pub fn txmsgdiscie(&self) -> TXMSGDISCIE_R {
        TXMSGDISCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TXMSGSENT interrupt enable
    #[inline(always)]
    pub fn txmsgsentie(&self) -> TXMSGSENTIE_R {
        TXMSGSENTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - TXMSGABT interrupt enable
    #[inline(always)]
    pub fn txmsgabtie(&self) -> TXMSGABTIE_R {
        TXMSGABTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - HRSTDISC interrupt enable
    #[inline(always)]
    pub fn hrstdiscie(&self) -> HRSTDISCIE_R {
        HRSTDISCIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - HRSTSENT interrupt enable
    #[inline(always)]
    pub fn hrstsentie(&self) -> HRSTSENTIE_R {
        HRSTSENTIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - TXUND interrupt enable
    #[inline(always)]
    pub fn txundie(&self) -> TXUNDIE_R {
        TXUNDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 8 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - RXORDDET interrupt enable
    #[inline(always)]
    pub fn rxorddetie(&self) -> RXORDDETIE_R {
        RXORDDETIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - RXHRSTDET interrupt enable
    #[inline(always)]
    pub fn rxhrstdetie(&self) -> RXHRSTDETIE_R {
        RXHRSTDETIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - RXOVR interrupt enable
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - RXMSGEND interrupt enable
    #[inline(always)]
    pub fn rxmsgendie(&self) -> RXMSGENDIE_R {
        RXMSGENDIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 14 - TYPECEVT1 interrupt enable
    #[inline(always)]
    pub fn typecevt1ie(&self) -> TYPECEVT1IE_R {
        TYPECEVT1IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - TYPECEVT2 interrupt enable
    #[inline(always)]
    pub fn typecevt2ie(&self) -> TYPECEVT2IE_R {
        TYPECEVT2IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 20 - FRSEVT interrupt enable
    #[inline(always)]
    pub fn frsevtie(&self) -> FRSEVTIE_R {
        FRSEVTIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TXIS interrupt enable
    #[inline(always)]
    pub fn txisie(&mut self) -> TXISIE_W {
        TXISIE_W { w: self }
    }
    ///Bit 1 - TXMSGDISC interrupt enable
    #[inline(always)]
    pub fn txmsgdiscie(&mut self) -> TXMSGDISCIE_W {
        TXMSGDISCIE_W { w: self }
    }
    ///Bit 2 - TXMSGSENT interrupt enable
    #[inline(always)]
    pub fn txmsgsentie(&mut self) -> TXMSGSENTIE_W {
        TXMSGSENTIE_W { w: self }
    }
    ///Bit 3 - TXMSGABT interrupt enable
    #[inline(always)]
    pub fn txmsgabtie(&mut self) -> TXMSGABTIE_W {
        TXMSGABTIE_W { w: self }
    }
    ///Bit 4 - HRSTDISC interrupt enable
    #[inline(always)]
    pub fn hrstdiscie(&mut self) -> HRSTDISCIE_W {
        HRSTDISCIE_W { w: self }
    }
    ///Bit 5 - HRSTSENT interrupt enable
    #[inline(always)]
    pub fn hrstsentie(&mut self) -> HRSTSENTIE_W {
        HRSTSENTIE_W { w: self }
    }
    ///Bit 6 - TXUND interrupt enable
    #[inline(always)]
    pub fn txundie(&mut self) -> TXUNDIE_W {
        TXUNDIE_W { w: self }
    }
    ///Bit 8 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W {
        RXNEIE_W { w: self }
    }
    ///Bit 9 - RXORDDET interrupt enable
    #[inline(always)]
    pub fn rxorddetie(&mut self) -> RXORDDETIE_W {
        RXORDDETIE_W { w: self }
    }
    ///Bit 10 - RXHRSTDET interrupt enable
    #[inline(always)]
    pub fn rxhrstdetie(&mut self) -> RXHRSTDETIE_W {
        RXHRSTDETIE_W { w: self }
    }
    ///Bit 11 - RXOVR interrupt enable
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W {
        RXOVRIE_W { w: self }
    }
    ///Bit 12 - RXMSGEND interrupt enable
    #[inline(always)]
    pub fn rxmsgendie(&mut self) -> RXMSGENDIE_W {
        RXMSGENDIE_W { w: self }
    }
    ///Bit 14 - TYPECEVT1 interrupt enable
    #[inline(always)]
    pub fn typecevt1ie(&mut self) -> TYPECEVT1IE_W {
        TYPECEVT1IE_W { w: self }
    }
    ///Bit 15 - TYPECEVT2 interrupt enable
    #[inline(always)]
    pub fn typecevt2ie(&mut self) -> TYPECEVT2IE_W {
        TYPECEVT2IE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr](index.html) module
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [imr::R](R) reader structure
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [imr::W](W) writer structure
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
