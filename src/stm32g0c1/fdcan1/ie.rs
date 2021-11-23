///Register `IE` reader
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IE` writer
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
///Rx FIFO 0 new message interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0NE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<RF0NE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0NE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable
pub struct RF0NE_R(crate::FieldReader<bool, RF0NE_A>);
impl RF0NE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0NE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0NE_A {
        match self.bits {
            false => RF0NE_A::B_0X0,
            true => RF0NE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0NE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0NE_A::B_0X1
    }
}
impl core::ops::Deref for RF0NE_R {
    type Target = crate::FieldReader<bool, RF0NE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable
pub struct RF0NE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0NE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF0NE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0NE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0NE_A::B_0X1)
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
///Rx FIFO 0 full interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0FE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<RF0FE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0FE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0FE` reader - Rx FIFO 0 full interrupt enable
pub struct RF0FE_R(crate::FieldReader<bool, RF0FE_A>);
impl RF0FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0FE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0FE_A {
        match self.bits {
            false => RF0FE_A::B_0X0,
            true => RF0FE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0FE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0FE_A::B_0X1
    }
}
impl core::ops::Deref for RF0FE_R {
    type Target = crate::FieldReader<bool, RF0FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0FE` writer - Rx FIFO 0 full interrupt enable
pub struct RF0FE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0FE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF0FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0FE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0FE_A::B_0X1)
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
///Rx FIFO 0 message lost interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0LE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<RF0LE_A> for bool {
    #[inline(always)]
    fn from(variant: RF0LE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable
pub struct RF0LE_R(crate::FieldReader<bool, RF0LE_A>);
impl RF0LE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0LE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0LE_A {
        match self.bits {
            false => RF0LE_A::B_0X0,
            true => RF0LE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0LE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0LE_A::B_0X1
    }
}
impl core::ops::Deref for RF0LE_R {
    type Target = crate::FieldReader<bool, RF0LE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable
pub struct RF0LE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0LE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF0LE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0LE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0LE_A::B_0X1)
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
///Rx FIFO 1 new message interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1NE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<RF1NE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1NE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable
pub struct RF1NE_R(crate::FieldReader<bool, RF1NE_A>);
impl RF1NE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1NE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1NE_A {
        match self.bits {
            false => RF1NE_A::B_0X0,
            true => RF1NE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1NE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1NE_A::B_0X1
    }
}
impl core::ops::Deref for RF1NE_R {
    type Target = crate::FieldReader<bool, RF1NE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable
pub struct RF1NE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1NE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF1NE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1NE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1NE_A::B_0X1)
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
///Rx FIFO 1 full interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1FE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<RF1FE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1FE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1FE` reader - Rx FIFO 1 full interrupt enable
pub struct RF1FE_R(crate::FieldReader<bool, RF1FE_A>);
impl RF1FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1FE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1FE_A {
        match self.bits {
            false => RF1FE_A::B_0X0,
            true => RF1FE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1FE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1FE_A::B_0X1
    }
}
impl core::ops::Deref for RF1FE_R {
    type Target = crate::FieldReader<bool, RF1FE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1FE` writer - Rx FIFO 1 full interrupt enable
pub struct RF1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1FE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF1FE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1FE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1FE_A::B_0X1)
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
///Rx FIFO 1 message lost interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1LE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<RF1LE_A> for bool {
    #[inline(always)]
    fn from(variant: RF1LE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable
pub struct RF1LE_R(crate::FieldReader<bool, RF1LE_A>);
impl RF1LE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1LE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1LE_A {
        match self.bits {
            false => RF1LE_A::B_0X0,
            true => RF1LE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1LE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1LE_A::B_0X1
    }
}
impl core::ops::Deref for RF1LE_R {
    type Target = crate::FieldReader<bool, RF1LE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable
pub struct RF1LE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1LE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF1LE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1LE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1LE_A::B_0X1)
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
///High-priority message interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPME_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<HPME_A> for bool {
    #[inline(always)]
    fn from(variant: HPME_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HPME` reader - High-priority message interrupt enable
pub struct HPME_R(crate::FieldReader<bool, HPME_A>);
impl HPME_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPME_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HPME_A {
        match self.bits {
            false => HPME_A::B_0X0,
            true => HPME_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HPME_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HPME_A::B_0X1
    }
}
impl core::ops::Deref for HPME_R {
    type Target = crate::FieldReader<bool, HPME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HPME` writer - High-priority message interrupt enable
pub struct HPME_W<'a> {
    w: &'a mut W,
}
impl<'a> HPME_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HPME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HPME_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HPME_A::B_0X1)
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
///Transmission completed interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCE` reader - Transmission completed interrupt enable
pub struct TCE_R(crate::FieldReader<bool, TCE_A>);
impl TCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::B_0X0,
            true => TCE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TCE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TCE_A::B_0X1
    }
}
impl core::ops::Deref for TCE_R {
    type Target = crate::FieldReader<bool, TCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCE` writer - Transmission completed interrupt enable
pub struct TCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCE_A::B_0X1)
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
///Transmission cancellation finished interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCFE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TCFE_A> for bool {
    #[inline(always)]
    fn from(variant: TCFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFE` reader - Transmission cancellation finished interrupt enable
pub struct TCFE_R(crate::FieldReader<bool, TCFE_A>);
impl TCFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCFE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCFE_A {
        match self.bits {
            false => TCFE_A::B_0X0,
            true => TCFE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TCFE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TCFE_A::B_0X1
    }
}
impl core::ops::Deref for TCFE_R {
    type Target = crate::FieldReader<bool, TCFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCFE` writer - Transmission cancellation finished interrupt enable
pub struct TCFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCFE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCFE_A::B_0X1)
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
///Tx FIFO empty interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFEE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TFEE_A> for bool {
    #[inline(always)]
    fn from(variant: TFEE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFEE` reader - Tx FIFO empty interrupt enable
pub struct TFEE_R(crate::FieldReader<bool, TFEE_A>);
impl TFEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFEE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TFEE_A {
        match self.bits {
            false => TFEE_A::B_0X0,
            true => TFEE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TFEE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TFEE_A::B_0X1
    }
}
impl core::ops::Deref for TFEE_R {
    type Target = crate::FieldReader<bool, TFEE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFEE` writer - Tx FIFO empty interrupt enable
pub struct TFEE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFEE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TFEE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TFEE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TFEE_A::B_0X1)
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
///Tx event FIFO new entry interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFNE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TEFNE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFNE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFNE` reader - Tx event FIFO new entry interrupt enable
pub struct TEFNE_R(crate::FieldReader<bool, TEFNE_A>);
impl TEFNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFNE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEFNE_A {
        match self.bits {
            false => TEFNE_A::B_0X0,
            true => TEFNE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEFNE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEFNE_A::B_0X1
    }
}
impl core::ops::Deref for TEFNE_R {
    type Target = crate::FieldReader<bool, TEFNE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFNE` writer - Tx event FIFO new entry interrupt enable
pub struct TEFNE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFNE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEFNE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFNE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFNE_A::B_0X1)
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
///Tx event FIFO full interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFFE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TEFFE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFFE` reader - Tx event FIFO full interrupt enable
pub struct TEFFE_R(crate::FieldReader<bool, TEFFE_A>);
impl TEFFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFFE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEFFE_A {
        match self.bits {
            false => TEFFE_A::B_0X0,
            true => TEFFE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEFFE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEFFE_A::B_0X1
    }
}
impl core::ops::Deref for TEFFE_R {
    type Target = crate::FieldReader<bool, TEFFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFFE` writer - Tx event FIFO full interrupt enable
pub struct TEFFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEFFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFFE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFFE_A::B_0X1)
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
///Tx event FIFO element lost interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFLE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TEFLE_A> for bool {
    #[inline(always)]
    fn from(variant: TEFLE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFLE` reader - Tx event FIFO element lost interrupt enable
pub struct TEFLE_R(crate::FieldReader<bool, TEFLE_A>);
impl TEFLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFLE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEFLE_A {
        match self.bits {
            false => TEFLE_A::B_0X0,
            true => TEFLE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEFLE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEFLE_A::B_0X1
    }
}
impl core::ops::Deref for TEFLE_R {
    type Target = crate::FieldReader<bool, TEFLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFLE` writer - Tx event FIFO element lost interrupt enable
pub struct TEFLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFLE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEFLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFLE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFLE_A::B_0X1)
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
///Timestamp wraparound interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSWE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TSWE_A> for bool {
    #[inline(always)]
    fn from(variant: TSWE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSWE` reader - Timestamp wraparound interrupt enable
pub struct TSWE_R(crate::FieldReader<bool, TSWE_A>);
impl TSWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSWE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSWE_A {
        match self.bits {
            false => TSWE_A::B_0X0,
            true => TSWE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TSWE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TSWE_A::B_0X1
    }
}
impl core::ops::Deref for TSWE_R {
    type Target = crate::FieldReader<bool, TSWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSWE` writer - Timestamp wraparound interrupt enable
pub struct TSWE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSWE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSWE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSWE_A::B_0X1)
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
///Message RAM access failure interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRAFE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<MRAFE_A> for bool {
    #[inline(always)]
    fn from(variant: MRAFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MRAFE` reader - Message RAM access failure interrupt enable
pub struct MRAFE_R(crate::FieldReader<bool, MRAFE_A>);
impl MRAFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRAFE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MRAFE_A {
        match self.bits {
            false => MRAFE_A::B_0X0,
            true => MRAFE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MRAFE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MRAFE_A::B_0X1
    }
}
impl core::ops::Deref for MRAFE_R {
    type Target = crate::FieldReader<bool, MRAFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MRAFE` writer - Message RAM access failure interrupt enable
pub struct MRAFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MRAFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MRAFE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MRAFE_A::B_0X1)
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
///Timeout occurred interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOOE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<TOOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOOE` reader - Timeout occurred interrupt enable
pub struct TOOE_R(crate::FieldReader<bool, TOOE_A>);
impl TOOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TOOE_A {
        match self.bits {
            false => TOOE_A::B_0X0,
            true => TOOE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TOOE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TOOE_A::B_0X1
    }
}
impl core::ops::Deref for TOOE_R {
    type Target = crate::FieldReader<bool, TOOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOOE` writer - Timeout occurred interrupt enable
pub struct TOOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TOOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOOE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOOE_A::B_0X1)
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
///Error logging overflow interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELOE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<ELOE_A> for bool {
    #[inline(always)]
    fn from(variant: ELOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELOE` reader - Error logging overflow interrupt enable
pub struct ELOE_R(crate::FieldReader<bool, ELOE_A>);
impl ELOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ELOE_A {
        match self.bits {
            false => ELOE_A::B_0X0,
            true => ELOE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ELOE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ELOE_A::B_0X1
    }
}
impl core::ops::Deref for ELOE_R {
    type Target = crate::FieldReader<bool, ELOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ELOE` writer - Error logging overflow interrupt enable
pub struct ELOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ELOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ELOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ELOE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ELOE_A::B_0X1)
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
///Error passive interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<EPE_A> for bool {
    #[inline(always)]
    fn from(variant: EPE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EPE` reader - Error passive interrupt enable
pub struct EPE_R(crate::FieldReader<bool, EPE_A>);
impl EPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EPE_A {
        match self.bits {
            false => EPE_A::B_0X0,
            true => EPE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EPE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EPE_A::B_0X1
    }
}
impl core::ops::Deref for EPE_R {
    type Target = crate::FieldReader<bool, EPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EPE` writer - Error passive interrupt enable
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EPE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EPE_A::B_0X1)
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
///Warning status interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<EWE_A> for bool {
    #[inline(always)]
    fn from(variant: EWE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EWE` reader - Warning status interrupt enable
pub struct EWE_R(crate::FieldReader<bool, EWE_A>);
impl EWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWE_A {
        match self.bits {
            false => EWE_A::B_0X0,
            true => EWE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EWE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EWE_A::B_0X1
    }
}
impl core::ops::Deref for EWE_R {
    type Target = crate::FieldReader<bool, EWE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWE` writer - Warning status interrupt enable
pub struct EWE_W<'a> {
    w: &'a mut W,
}
impl<'a> EWE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EWE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EWE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EWE_A::B_0X1)
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
///Bus_Off status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<BOE_A> for bool {
    #[inline(always)]
    fn from(variant: BOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BOE` reader - Bus_Off status
pub struct BOE_R(crate::FieldReader<bool, BOE_A>);
impl BOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BOE_A {
        match self.bits {
            false => BOE_A::B_0X0,
            true => BOE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BOE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BOE_A::B_0X1
    }
}
impl core::ops::Deref for BOE_R {
    type Target = crate::FieldReader<bool, BOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BOE` writer - Bus_Off status
pub struct BOE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BOE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BOE_A::B_0X1)
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
///Watchdog interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDIE_A {
    ///0: Interrupt disabled
    B_0X0 = 0,
    ///1: Interrupt enabled
    B_0X1 = 1,
}
impl From<WDIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WDIE` reader - Watchdog interrupt enable
pub struct WDIE_R(crate::FieldReader<bool, WDIE_A>);
impl WDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDIE_A {
        match self.bits {
            false => WDIE_A::B_0X0,
            true => WDIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WDIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WDIE_A::B_0X1
    }
}
impl core::ops::Deref for WDIE_R {
    type Target = crate::FieldReader<bool, WDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDIE` writer - Watchdog interrupt enable
pub struct WDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDIE_A::B_0X0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDIE_A::B_0X1)
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
///Field `PEAE` reader - Protocol error in arbitration phase enable
pub struct PEAE_R(crate::FieldReader<bool, bool>);
impl PEAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEAE` writer - Protocol error in arbitration phase enable
pub struct PEAE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAE_W<'a> {
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
///Field `PEDE` reader - Protocol error in data phase enable
pub struct PEDE_R(crate::FieldReader<bool, bool>);
impl PEDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEDE` writer - Protocol error in data phase enable
pub struct PEDE_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Field `ARAE` reader - Access to reserved address enable
pub struct ARAE_R(crate::FieldReader<bool, bool>);
impl ARAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARAE` writer - Access to reserved address enable
pub struct ARAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARAE_W<'a> {
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
impl R {
    ///Bit 0 - Rx FIFO 0 new message interrupt enable
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Rx FIFO 0 full interrupt enable
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rx FIFO 0 message lost interrupt enable
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Rx FIFO 1 new message interrupt enable
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rx FIFO 1 full interrupt enable
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Rx FIFO 1 message lost interrupt enable
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - High-priority message interrupt enable
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Transmission completed interrupt enable
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Transmission cancellation finished interrupt enable
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Tx event FIFO new entry interrupt enable
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Tx event FIFO full interrupt enable
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Tx event FIFO element lost interrupt enable
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Timestamp wraparound interrupt enable
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Message RAM access failure interrupt enable
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Timeout occurred interrupt enable
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Error logging overflow interrupt enable
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Error passive interrupt enable
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Warning status interrupt enable
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Bus_Off status
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Watchdog interrupt enable
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Protocol error in arbitration phase enable
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Protocol error in data phase enable
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Access to reserved address enable
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 new message interrupt enable
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W {
        RF0NE_W { w: self }
    }
    ///Bit 1 - Rx FIFO 0 full interrupt enable
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W {
        RF0FE_W { w: self }
    }
    ///Bit 2 - Rx FIFO 0 message lost interrupt enable
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W {
        RF0LE_W { w: self }
    }
    ///Bit 3 - Rx FIFO 1 new message interrupt enable
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W {
        RF1NE_W { w: self }
    }
    ///Bit 4 - Rx FIFO 1 full interrupt enable
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W {
        RF1FE_W { w: self }
    }
    ///Bit 5 - Rx FIFO 1 message lost interrupt enable
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W {
        RF1LE_W { w: self }
    }
    ///Bit 6 - High-priority message interrupt enable
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W {
        HPME_W { w: self }
    }
    ///Bit 7 - Transmission completed interrupt enable
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W {
        TCE_W { w: self }
    }
    ///Bit 8 - Transmission cancellation finished interrupt enable
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W {
        TCFE_W { w: self }
    }
    ///Bit 9 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn tfee(&mut self) -> TFEE_W {
        TFEE_W { w: self }
    }
    ///Bit 10 - Tx event FIFO new entry interrupt enable
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W {
        TEFNE_W { w: self }
    }
    ///Bit 11 - Tx event FIFO full interrupt enable
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W {
        TEFFE_W { w: self }
    }
    ///Bit 12 - Tx event FIFO element lost interrupt enable
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W {
        TEFLE_W { w: self }
    }
    ///Bit 13 - Timestamp wraparound interrupt enable
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W {
        TSWE_W { w: self }
    }
    ///Bit 14 - Message RAM access failure interrupt enable
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W {
        MRAFE_W { w: self }
    }
    ///Bit 15 - Timeout occurred interrupt enable
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W {
        TOOE_W { w: self }
    }
    ///Bit 16 - Error logging overflow interrupt enable
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W {
        ELOE_W { w: self }
    }
    ///Bit 17 - Error passive interrupt enable
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    ///Bit 18 - Warning status interrupt enable
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W {
        EWE_W { w: self }
    }
    ///Bit 19 - Bus_Off status
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W {
        BOE_W { w: self }
    }
    ///Bit 20 - Watchdog interrupt enable
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W {
        WDIE_W { w: self }
    }
    ///Bit 21 - Protocol error in arbitration phase enable
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W {
        PEAE_W { w: self }
    }
    ///Bit 22 - Protocol error in data phase enable
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W {
        PEDE_W { w: self }
    }
    ///Bit 23 - Access to reserved address enable
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W {
        ARAE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ie](index.html) module
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
///`read()` method returns [ie::R](R) reader structure
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ie::W](W) writer structure
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
///`reset()` method sets IE to value 0
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
