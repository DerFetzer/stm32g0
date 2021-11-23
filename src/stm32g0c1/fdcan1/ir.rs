///Register `IR` reader
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IR` writer
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
///Rx FIFO 0 new message
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0N_A {
    ///0: No new message written to Rx FIFO 0
    B_0X0 = 0,
    ///1: New message written to Rx FIFO 0
    B_0X1 = 1,
}
impl From<RF0N_A> for bool {
    #[inline(always)]
    fn from(variant: RF0N_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0N` reader - Rx FIFO 0 new message
pub struct RF0N_R(crate::FieldReader<bool, RF0N_A>);
impl RF0N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0N_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0N_A {
        match self.bits {
            false => RF0N_A::B_0X0,
            true => RF0N_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0N_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0N_A::B_0X1
    }
}
impl core::ops::Deref for RF0N_R {
    type Target = crate::FieldReader<bool, RF0N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0N` writer - Rx FIFO 0 new message
pub struct RF0N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF0N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No new message written to Rx FIFO 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0N_A::B_0X0)
    }
    ///New message written to Rx FIFO 0
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0N_A::B_0X1)
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
///Rx FIFO 0 full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0F_A {
    ///0: Rx FIFO 0 not full
    B_0X0 = 0,
    ///1: Rx FIFO 0 full
    B_0X1 = 1,
}
impl From<RF0F_A> for bool {
    #[inline(always)]
    fn from(variant: RF0F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0F` reader - Rx FIFO 0 full
pub struct RF0F_R(crate::FieldReader<bool, RF0F_A>);
impl RF0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0F_A {
        match self.bits {
            false => RF0F_A::B_0X0,
            true => RF0F_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0F_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0F_A::B_0X1
    }
}
impl core::ops::Deref for RF0F_R {
    type Target = crate::FieldReader<bool, RF0F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0F` writer - Rx FIFO 0 full
pub struct RF0F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF0F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rx FIFO 0 not full
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0F_A::B_0X0)
    }
    ///Rx FIFO 0 full
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0F_A::B_0X1)
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
///Rx FIFO 0 message lost
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0L_A {
    ///0: No Rx FIFO 0 message lost
    B_0X0 = 0,
    ///1: Rx FIFO 0 message lost
    B_0X1 = 1,
}
impl From<RF0L_A> for bool {
    #[inline(always)]
    fn from(variant: RF0L_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0L` reader - Rx FIFO 0 message lost
pub struct RF0L_R(crate::FieldReader<bool, RF0L_A>);
impl RF0L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0L_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0L_A {
        match self.bits {
            false => RF0L_A::B_0X0,
            true => RF0L_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0L_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0L_A::B_0X1
    }
}
impl core::ops::Deref for RF0L_R {
    type Target = crate::FieldReader<bool, RF0L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF0L` writer - Rx FIFO 0 message lost
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF0L_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No Rx FIFO 0 message lost
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X0)
    }
    ///Rx FIFO 0 message lost
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X1)
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
///Rx FIFO 1 new message
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1N_A {
    ///0: No new message written to Rx FIFO 1
    B_0X0 = 0,
    ///1: New message written to Rx FIFO 1
    B_0X1 = 1,
}
impl From<RF1N_A> for bool {
    #[inline(always)]
    fn from(variant: RF1N_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1N` reader - Rx FIFO 1 new message
pub struct RF1N_R(crate::FieldReader<bool, RF1N_A>);
impl RF1N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1N_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1N_A {
        match self.bits {
            false => RF1N_A::B_0X0,
            true => RF1N_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1N_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1N_A::B_0X1
    }
}
impl core::ops::Deref for RF1N_R {
    type Target = crate::FieldReader<bool, RF1N_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1N` writer - Rx FIFO 1 new message
pub struct RF1N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1N_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF1N_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No new message written to Rx FIFO 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1N_A::B_0X0)
    }
    ///New message written to Rx FIFO 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1N_A::B_0X1)
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
///Rx FIFO 1 full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1F_A {
    ///0: Rx FIFO 1 not full
    B_0X0 = 0,
    ///1: Rx FIFO 1 full
    B_0X1 = 1,
}
impl From<RF1F_A> for bool {
    #[inline(always)]
    fn from(variant: RF1F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1F` reader - Rx FIFO 1 full
pub struct RF1F_R(crate::FieldReader<bool, RF1F_A>);
impl RF1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1F_A {
        match self.bits {
            false => RF1F_A::B_0X0,
            true => RF1F_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1F_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1F_A::B_0X1
    }
}
impl core::ops::Deref for RF1F_R {
    type Target = crate::FieldReader<bool, RF1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1F` writer - Rx FIFO 1 full
pub struct RF1F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF1F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Rx FIFO 1 not full
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1F_A::B_0X0)
    }
    ///Rx FIFO 1 full
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1F_A::B_0X1)
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
///Rx FIFO 1 message lost
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1L_A {
    ///0: No Rx FIFO 1 message lost
    B_0X0 = 0,
    ///1: Rx FIFO 1 message lost
    B_0X1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1L` reader - Rx FIFO 1 message lost
pub struct RF1L_R(crate::FieldReader<bool, RF1L_A>);
impl RF1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0X0,
            true => RF1L_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1L_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1L_A::B_0X1
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, RF1L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RF1L` writer - Rx FIFO 1 message lost
pub struct RF1L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1L_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RF1L_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No Rx FIFO 1 message lost
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1L_A::B_0X0)
    }
    ///Rx FIFO 1 message lost
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1L_A::B_0X1)
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
///High-priority message
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_A {
    ///0: No high-priority message received
    B_0X0 = 0,
    ///1: High-priority message received
    B_0X1 = 1,
}
impl From<HPM_A> for bool {
    #[inline(always)]
    fn from(variant: HPM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HPM` reader - High-priority message
pub struct HPM_R(crate::FieldReader<bool, HPM_A>);
impl HPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HPM_A {
        match self.bits {
            false => HPM_A::B_0X0,
            true => HPM_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HPM_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HPM_A::B_0X1
    }
}
impl core::ops::Deref for HPM_R {
    type Target = crate::FieldReader<bool, HPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HPM` writer - High-priority message
pub struct HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HPM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No high-priority message received
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HPM_A::B_0X0)
    }
    ///High-priority message received
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HPM_A::B_0X1)
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
///Transmission completed
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    ///0: No transmission completed
    B_0X0 = 0,
    ///1: Transmission completed
    B_0X1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Transmission completed
pub struct TC_R(crate::FieldReader<bool, TC_A>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::B_0X0,
            true => TC_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TC_A::B_0X1
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, TC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TC` writer - Transmission completed
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No transmission completed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TC_A::B_0X0)
    }
    ///Transmission completed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TC_A::B_0X1)
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
///Transmission cancellation finished
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    ///0: No transmission cancellation finished
    B_0X0 = 0,
    ///1: Transmission cancellation finished
    B_0X1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCF` reader - Transmission cancellation finished
pub struct TCF_R(crate::FieldReader<bool, TCF_A>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::B_0X0,
            true => TCF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TCF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TCF_A::B_0X1
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, TCF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCF` writer - Transmission cancellation finished
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No transmission cancellation finished
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCF_A::B_0X0)
    }
    ///Transmission cancellation finished
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCF_A::B_0X1)
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
///Tx FIFO empty
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
    ///0: Tx FIFO non-empty
    B_0X0 = 0,
    ///1: Tx FIFO empty
    B_0X1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFE` reader - Tx FIFO empty
pub struct TFE_R(crate::FieldReader<bool, TFE_A>);
impl TFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::B_0X0,
            true => TFE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TFE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TFE_A::B_0X1
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, TFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFE` writer - Tx FIFO empty
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tx FIFO non-empty
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TFE_A::B_0X0)
    }
    ///Tx FIFO empty
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TFE_A::B_0X1)
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
///Tx event FIFO New Entry
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFN_A {
    ///0: Tx event FIFO unchanged
    B_0X0 = 0,
    ///1: Tx handler wrote Tx event FIFO element.
    B_0X1 = 1,
}
impl From<TEFN_A> for bool {
    #[inline(always)]
    fn from(variant: TEFN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFN` reader - Tx event FIFO New Entry
pub struct TEFN_R(crate::FieldReader<bool, TEFN_A>);
impl TEFN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEFN_A {
        match self.bits {
            false => TEFN_A::B_0X0,
            true => TEFN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEFN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEFN_A::B_0X1
    }
}
impl core::ops::Deref for TEFN_R {
    type Target = crate::FieldReader<bool, TEFN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFN` writer - Tx event FIFO New Entry
pub struct TEFN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEFN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tx event FIFO unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFN_A::B_0X0)
    }
    ///Tx handler wrote Tx event FIFO element.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFN_A::B_0X1)
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
///Tx event FIFO full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFF_A {
    ///0: Tx event FIFO Not full
    B_0X0 = 0,
    ///1: Tx event FIFO full
    B_0X1 = 1,
}
impl From<TEFF_A> for bool {
    #[inline(always)]
    fn from(variant: TEFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFF` reader - Tx event FIFO full
pub struct TEFF_R(crate::FieldReader<bool, TEFF_A>);
impl TEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEFF_A {
        match self.bits {
            false => TEFF_A::B_0X0,
            true => TEFF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEFF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEFF_A::B_0X1
    }
}
impl core::ops::Deref for TEFF_R {
    type Target = crate::FieldReader<bool, TEFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFF` writer - Tx event FIFO full
pub struct TEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tx event FIFO Not full
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFF_A::B_0X0)
    }
    ///Tx event FIFO full
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFF_A::B_0X1)
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
///Tx event FIFO element lost
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFL_A {
    ///0: No Tx event FIFO element lost
    B_0X0 = 0,
    ///1: Tx event FIFO element lost
    B_0X1 = 1,
}
impl From<TEFL_A> for bool {
    #[inline(always)]
    fn from(variant: TEFL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFL` reader - Tx event FIFO element lost
pub struct TEFL_R(crate::FieldReader<bool, TEFL_A>);
impl TEFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEFL_A {
        match self.bits {
            false => TEFL_A::B_0X0,
            true => TEFL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEFL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEFL_A::B_0X1
    }
}
impl core::ops::Deref for TEFL_R {
    type Target = crate::FieldReader<bool, TEFL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFL` writer - Tx event FIFO element lost
pub struct TEFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEFL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No Tx event FIFO element lost
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFL_A::B_0X0)
    }
    ///Tx event FIFO element lost
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFL_A::B_0X1)
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
///Timestamp wraparound
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSW_A {
    ///0: No timestamp counter wrap-around
    B_0X0 = 0,
    ///1: Timestamp counter wrapped around
    B_0X1 = 1,
}
impl From<TSW_A> for bool {
    #[inline(always)]
    fn from(variant: TSW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TSW` reader - Timestamp wraparound
pub struct TSW_R(crate::FieldReader<bool, TSW_A>);
impl TSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSW_A {
        match self.bits {
            false => TSW_A::B_0X0,
            true => TSW_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TSW_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TSW_A::B_0X1
    }
}
impl core::ops::Deref for TSW_R {
    type Target = crate::FieldReader<bool, TSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSW` writer - Timestamp wraparound
pub struct TSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No timestamp counter wrap-around
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSW_A::B_0X0)
    }
    ///Timestamp counter wrapped around
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSW_A::B_0X1)
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
///Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted Operation Mode (see mode). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRAF_A {
    ///0: No Message RAM access failure occurred
    B_0X0 = 0,
    ///1: Message RAM access failure occurred
    B_0X1 = 1,
}
impl From<MRAF_A> for bool {
    #[inline(always)]
    fn from(variant: MRAF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MRAF` reader - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted Operation Mode (see mode). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM.
pub struct MRAF_R(crate::FieldReader<bool, MRAF_A>);
impl MRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRAF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MRAF_A {
        match self.bits {
            false => MRAF_A::B_0X0,
            true => MRAF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MRAF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MRAF_A::B_0X1
    }
}
impl core::ops::Deref for MRAF_R {
    type Target = crate::FieldReader<bool, MRAF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MRAF` writer - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted Operation Mode (see mode). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM.
pub struct MRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MRAF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No Message RAM access failure occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MRAF_A::B_0X0)
    }
    ///Message RAM access failure occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MRAF_A::B_0X1)
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
///Timeout occurred
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOO_A {
    ///0: No timeout
    B_0X0 = 0,
    ///1: Timeout reached
    B_0X1 = 1,
}
impl From<TOO_A> for bool {
    #[inline(always)]
    fn from(variant: TOO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TOO` reader - Timeout occurred
pub struct TOO_R(crate::FieldReader<bool, TOO_A>);
impl TOO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TOO_A {
        match self.bits {
            false => TOO_A::B_0X0,
            true => TOO_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TOO_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TOO_A::B_0X1
    }
}
impl core::ops::Deref for TOO_R {
    type Target = crate::FieldReader<bool, TOO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOO` writer - Timeout occurred
pub struct TOO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TOO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No timeout
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOO_A::B_0X0)
    }
    ///Timeout reached
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOO_A::B_0X1)
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
///Error logging overflow
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELO_A {
    ///0: CAN error logging counter did not overflow.
    B_0X0 = 0,
    ///1: Overflow of CAN error logging counter occurred.
    B_0X1 = 1,
}
impl From<ELO_A> for bool {
    #[inline(always)]
    fn from(variant: ELO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ELO` reader - Error logging overflow
pub struct ELO_R(crate::FieldReader<bool, ELO_A>);
impl ELO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ELO_A {
        match self.bits {
            false => ELO_A::B_0X0,
            true => ELO_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ELO_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ELO_A::B_0X1
    }
}
impl core::ops::Deref for ELO_R {
    type Target = crate::FieldReader<bool, ELO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ELO` writer - Error logging overflow
pub struct ELO_W<'a> {
    w: &'a mut W,
}
impl<'a> ELO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ELO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CAN error logging counter did not overflow.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ELO_A::B_0X0)
    }
    ///Overflow of CAN error logging counter occurred.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ELO_A::B_0X1)
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
///Error passive
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_A {
    ///0: Error_Passive status unchanged
    B_0X0 = 0,
    ///1: Error_Passive status changed
    B_0X1 = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EP` reader - Error passive
pub struct EP_R(crate::FieldReader<bool, EP_A>);
impl EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::B_0X0,
            true => EP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EP_A::B_0X1
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, EP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EP` writer - Error passive
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Error_Passive status unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EP_A::B_0X0)
    }
    ///Error_Passive status changed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EP_A::B_0X1)
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
///Warning status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_A {
    ///0: Error_Warning status unchanged
    B_0X0 = 0,
    ///1: Error_Warning status changed
    B_0X1 = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EW` reader - Warning status
pub struct EW_R(crate::FieldReader<bool, EW_A>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::B_0X0,
            true => EW_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EW_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EW_A::B_0X1
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, EW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EW` writer - Warning status
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Error_Warning status unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EW_A::B_0X0)
    }
    ///Error_Warning status changed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EW_A::B_0X1)
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
pub enum BO_A {
    ///0: Bus_Off status unchanged
    B_0X0 = 0,
    ///1: Bus_Off status changed
    B_0X1 = 1,
}
impl From<BO_A> for bool {
    #[inline(always)]
    fn from(variant: BO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BO` reader - Bus_Off status
pub struct BO_R(crate::FieldReader<bool, BO_A>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BO_A {
        match self.bits {
            false => BO_A::B_0X0,
            true => BO_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BO_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BO_A::B_0X1
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, BO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BO` writer - Bus_Off status
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Bus_Off status unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BO_A::B_0X0)
    }
    ///Bus_Off status changed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BO_A::B_0X1)
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
///Watchdog interrupt
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDI_A {
    ///0: No message RAM watchdog event occurred
    B_0X0 = 0,
    ///1: Message RAM watchdog event due to missing READY
    B_0X1 = 1,
}
impl From<WDI_A> for bool {
    #[inline(always)]
    fn from(variant: WDI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WDI` reader - Watchdog interrupt
pub struct WDI_R(crate::FieldReader<bool, WDI_A>);
impl WDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDI_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDI_A {
        match self.bits {
            false => WDI_A::B_0X0,
            true => WDI_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WDI_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WDI_A::B_0X1
    }
}
impl core::ops::Deref for WDI_R {
    type Target = crate::FieldReader<bool, WDI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDI` writer - Watchdog interrupt
pub struct WDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No message RAM watchdog event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDI_A::B_0X0)
    }
    ///Message RAM watchdog event due to missing READY
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDI_A::B_0X1)
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
///Protocol error in arbitration phase (nominal bit time is used)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEA_A {
    ///0: No protocol error in arbitration phase
    B_0X0 = 0,
    ///1: Protocol error in arbitration phase detected (PSR.LEC different from 0,7)
    B_0X1 = 1,
}
impl From<PEA_A> for bool {
    #[inline(always)]
    fn from(variant: PEA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)
pub struct PEA_R(crate::FieldReader<bool, PEA_A>);
impl PEA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEA_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PEA_A {
        match self.bits {
            false => PEA_A::B_0X0,
            true => PEA_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PEA_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PEA_A::B_0X1
    }
}
impl core::ops::Deref for PEA_R {
    type Target = crate::FieldReader<bool, PEA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)
pub struct PEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PEA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PEA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No protocol error in arbitration phase
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PEA_A::B_0X0)
    }
    ///Protocol error in arbitration phase detected (PSR.LEC different from 0,7)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PEA_A::B_0X1)
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
///Protocol error in data phase (data bit time is used)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PED_A {
    ///0: No protocol error in data phase
    B_0X0 = 0,
    ///1: Protocol error in data phase detected (PSR.DLEC different from 0,7)
    B_0X1 = 1,
}
impl From<PED_A> for bool {
    #[inline(always)]
    fn from(variant: PED_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PED` reader - Protocol error in data phase (data bit time is used)
pub struct PED_R(crate::FieldReader<bool, PED_A>);
impl PED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PED_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PED_A {
        match self.bits {
            false => PED_A::B_0X0,
            true => PED_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PED_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PED_A::B_0X1
    }
}
impl core::ops::Deref for PED_R {
    type Target = crate::FieldReader<bool, PED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PED` writer - Protocol error in data phase (data bit time is used)
pub struct PED_W<'a> {
    w: &'a mut W,
}
impl<'a> PED_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No protocol error in data phase
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PED_A::B_0X0)
    }
    ///Protocol error in data phase detected (PSR.DLEC different from 0,7)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PED_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Access to reserved address
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARA_A {
    ///0: No access to reserved address occurred
    B_0X0 = 0,
    ///1: Access to reserved address occurred
    B_0X1 = 1,
}
impl From<ARA_A> for bool {
    #[inline(always)]
    fn from(variant: ARA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ARA` reader - Access to reserved address
pub struct ARA_R(crate::FieldReader<bool, ARA_A>);
impl ARA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARA_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARA_A {
        match self.bits {
            false => ARA_A::B_0X0,
            true => ARA_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ARA_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ARA_A::B_0X1
    }
}
impl core::ops::Deref for ARA_R {
    type Target = crate::FieldReader<bool, ARA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARA` writer - Access to reserved address
pub struct ARA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARA_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ARA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No access to reserved address occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ARA_A::B_0X0)
    }
    ///Access to reserved address occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ARA_A::B_0X1)
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
impl R {
    ///Bit 0 - Rx FIFO 0 new message
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Rx FIFO 0 full
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Rx FIFO 1 new message
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rx FIFO 1 full
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - High-priority message
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Transmission completed
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Transmission cancellation finished
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Tx FIFO empty
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Tx event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Tx event FIFO full
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Timestamp wraparound
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted Operation Mode (see mode). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM.
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Timeout occurred
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Error logging overflow
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Error passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Warning status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Bus_Off status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - Watchdog interrupt
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - Protocol error in arbitration phase (nominal bit time is used)
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - Protocol error in data phase (data bit time is used)
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - Access to reserved address
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 new message
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W {
        RF0N_W { w: self }
    }
    ///Bit 1 - Rx FIFO 0 full
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W {
        RF0F_W { w: self }
    }
    ///Bit 2 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
    ///Bit 3 - Rx FIFO 1 new message
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W {
        RF1N_W { w: self }
    }
    ///Bit 4 - Rx FIFO 1 full
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W {
        RF1F_W { w: self }
    }
    ///Bit 5 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W {
        RF1L_W { w: self }
    }
    ///Bit 6 - High-priority message
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W {
        HPM_W { w: self }
    }
    ///Bit 7 - Transmission completed
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    ///Bit 8 - Transmission cancellation finished
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    ///Bit 9 - Tx FIFO empty
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    ///Bit 10 - Tx event FIFO New Entry
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W {
        TEFN_W { w: self }
    }
    ///Bit 11 - Tx event FIFO full
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W {
        TEFF_W { w: self }
    }
    ///Bit 12 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    ///Bit 13 - Timestamp wraparound
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W {
        TSW_W { w: self }
    }
    ///Bit 14 - Message RAM access failure The flag is set when the Rx handler: has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. was unable to write a message to the message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated. The partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the FDCAN is switched into Restricted Operation Mode (see mode). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM.
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W {
        MRAF_W { w: self }
    }
    ///Bit 15 - Timeout occurred
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W {
        TOO_W { w: self }
    }
    ///Bit 16 - Error logging overflow
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W {
        ELO_W { w: self }
    }
    ///Bit 17 - Error passive
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    ///Bit 18 - Warning status
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    ///Bit 19 - Bus_Off status
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    ///Bit 20 - Watchdog interrupt
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W {
        WDI_W { w: self }
    }
    ///Bit 21 - Protocol error in arbitration phase (nominal bit time is used)
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W {
        PEA_W { w: self }
    }
    ///Bit 22 - Protocol error in data phase (data bit time is used)
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W {
        PED_W { w: self }
    }
    ///Bit 23 - Access to reserved address
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W {
        ARA_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ir](index.html) module
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ir::R](R) reader structure
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ir::W](W) writer structure
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
