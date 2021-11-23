///Register `APB_FZ1` reader
pub struct R(crate::R<APB_FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FZ1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB_FZ1` writer
pub struct W(crate::W<APB_FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FZ1_SPEC>;
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
impl From<crate::W<APB_FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FZ1_SPEC>) -> Self {
        W(writer)
    }
}
///Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM2_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM2_STOP` reader - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:
pub struct DBG_TIM2_STOP_R(crate::FieldReader<bool, DBG_TIM2_STOP_A>);
impl DBG_TIM2_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM2_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM2_STOP_A {
        match self.bits {
            false => DBG_TIM2_STOP_A::B_0X0,
            true => DBG_TIM2_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM2_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM2_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM2_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM2_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM2_STOP` writer - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:
pub struct DBG_TIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM2_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM2_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM2_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM2_STOP_A::B_0X1)
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
///Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM3_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
pub struct DBG_TIM3_STOP_R(crate::FieldReader<bool, DBG_TIM3_STOP_A>);
impl DBG_TIM3_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM3_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM3_STOP_A {
        match self.bits {
            false => DBG_TIM3_STOP_A::B_0X0,
            true => DBG_TIM3_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM3_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM3_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM3_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM3_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
pub struct DBG_TIM3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM3_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM3_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM3_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM3_STOP_A::B_0X1)
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
///Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM6_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM6_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM6_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM6_STOP` reader - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:
pub struct DBG_TIM6_STOP_R(crate::FieldReader<bool, DBG_TIM6_STOP_A>);
impl DBG_TIM6_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM6_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM6_STOP_A {
        match self.bits {
            false => DBG_TIM6_STOP_A::B_0X0,
            true => DBG_TIM6_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM6_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM6_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM6_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM6_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM6_STOP` writer - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:
pub struct DBG_TIM6_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM6_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM6_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM6_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM6_STOP_A::B_0X1)
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
///Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM7_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM7_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM7_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM7_STOP` reader - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:
pub struct DBG_TIM7_STOP_R(crate::FieldReader<bool, DBG_TIM7_STOP_A>);
impl DBG_TIM7_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM7_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM7_STOP_A {
        match self.bits {
            false => DBG_TIM7_STOP_A::B_0X0,
            true => DBG_TIM7_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM7_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM7_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM7_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM7_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM7_STOP` writer - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:
pub struct DBG_TIM7_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM7_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM7_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM7_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM7_STOP_A::B_0X1)
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
///Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_RTC_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_RTC_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_RTC_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
pub struct DBG_RTC_STOP_R(crate::FieldReader<bool, DBG_RTC_STOP_A>);
impl DBG_RTC_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_RTC_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_RTC_STOP_A {
        match self.bits {
            false => DBG_RTC_STOP_A::B_0X0,
            true => DBG_RTC_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_RTC_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_RTC_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_RTC_STOP_R {
    type Target = crate::FieldReader<bool, DBG_RTC_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
pub struct DBG_RTC_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_RTC_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_RTC_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_RTC_STOP_A::B_0X1)
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
///Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_WWDG_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_WWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_WWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
pub struct DBG_WWDG_STOP_R(crate::FieldReader<bool, DBG_WWDG_STOP_A>);
impl DBG_WWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WWDG_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_WWDG_STOP_A {
        match self.bits {
            false => DBG_WWDG_STOP_A::B_0X0,
            true => DBG_WWDG_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_WWDG_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_WWDG_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_WWDG_STOP_R {
    type Target = crate::FieldReader<bool, DBG_WWDG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
pub struct DBG_WWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_WWDG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_WWDG_STOP_A::B_0X1)
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
///Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_IWDG_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_IWDG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_IWDG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
pub struct DBG_IWDG_STOP_R(crate::FieldReader<bool, DBG_IWDG_STOP_A>);
impl DBG_IWDG_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_IWDG_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_IWDG_STOP_A {
        match self.bits {
            false => DBG_IWDG_STOP_A::B_0X0,
            true => DBG_IWDG_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_IWDG_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_IWDG_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_IWDG_STOP_R {
    type Target = crate::FieldReader<bool, DBG_IWDG_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
pub struct DBG_IWDG_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_IWDG_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_IWDG_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_IWDG_STOP_A::B_0X1)
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
///SMBUS timeout when core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_I2C1_SMBUS_TIMEOUT_A {
    ///0: Same behavior as in normal mode
    B_0X0 = 0,
    ///1: The SMBUS timeout is frozen
    B_0X1 = 1,
}
impl From<DBG_I2C1_SMBUS_TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_I2C1_SMBUS_TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted
pub struct DBG_I2C1_SMBUS_TIMEOUT_R(crate::FieldReader<bool, DBG_I2C1_SMBUS_TIMEOUT_A>);
impl DBG_I2C1_SMBUS_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C1_SMBUS_TIMEOUT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_I2C1_SMBUS_TIMEOUT_A {
        match self.bits {
            false => DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0,
            true => DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1
    }
}
impl core::ops::Deref for DBG_I2C1_SMBUS_TIMEOUT_R {
    type Target = crate::FieldReader<bool, DBG_I2C1_SMBUS_TIMEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted
pub struct DBG_I2C1_SMBUS_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_SMBUS_TIMEOUT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_I2C1_SMBUS_TIMEOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Same behavior as in normal mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT_A::B_0X0)
    }
    ///The SMBUS timeout is frozen
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_I2C1_SMBUS_TIMEOUT_A::B_0X1)
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
///Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_LPTIM2_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_LPTIM2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_LPTIM2_STOP` reader - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:
pub struct DBG_LPTIM2_STOP_R(crate::FieldReader<bool, DBG_LPTIM2_STOP_A>);
impl DBG_LPTIM2_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM2_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIM2_STOP_A {
        match self.bits {
            false => DBG_LPTIM2_STOP_A::B_0X0,
            true => DBG_LPTIM2_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_LPTIM2_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_LPTIM2_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_LPTIM2_STOP_R {
    type Target = crate::FieldReader<bool, DBG_LPTIM2_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_LPTIM2_STOP` writer - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:
pub struct DBG_LPTIM2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM2_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_LPTIM2_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_LPTIM2_STOP_A::B_0X1)
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
///Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_LPTIM1_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_LPTIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_LPTIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_LPTIM1_STOP` reader - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:
pub struct DBG_LPTIM1_STOP_R(crate::FieldReader<bool, DBG_LPTIM1_STOP_A>);
impl DBG_LPTIM1_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM1_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_LPTIM1_STOP_A {
        match self.bits {
            false => DBG_LPTIM1_STOP_A::B_0X0,
            true => DBG_LPTIM1_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_LPTIM1_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_LPTIM1_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_LPTIM1_STOP_R {
    type Target = crate::FieldReader<bool, DBG_LPTIM1_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_LPTIM1_STOP` writer - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:
pub struct DBG_LPTIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM1_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_LPTIM1_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_LPTIM1_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_LPTIM1_STOP_A::B_0X1)
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
    ///Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 21 - SMBUS timeout when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 30 - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W {
        DBG_TIM2_STOP_W { w: self }
    }
    ///Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W {
        DBG_TIM3_STOP_W { w: self }
    }
    ///Bit 4 - Clocking of TIM6 counter when the core is halted This bit enables/disables the clock to the counter of TIM6 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W {
        DBG_TIM6_STOP_W { w: self }
    }
    ///Bit 5 - Clocking of TIM7 counter when the core is halted. This bit enables/disables the clock to the counter of ITIM7 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W {
        DBG_TIM7_STOP_W { w: self }
    }
    ///Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W {
        DBG_RTC_STOP_W { w: self }
    }
    ///Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W {
        DBG_WWDG_STOP_W { w: self }
    }
    ///Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W {
        DBG_IWDG_STOP_W { w: self }
    }
    ///Bit 21 - SMBUS timeout when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W {
        DBG_I2C1_SMBUS_TIMEOUT_W { w: self }
    }
    ///Bit 30 - Clocking of LPTIMER2 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER2 when the core is halted:
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W {
        DBG_LPTIM2_STOP_W { w: self }
    }
    ///Bit 31 - Clocking of LPTIMER1 counter when the core is halted This bit enables/disables the clock to the counter of LPTIMER1 when the core is halted:
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W {
        DBG_LPTIM1_STOP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBG APB freeze register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb_fz1](index.html) module
pub struct APB_FZ1_SPEC;
impl crate::RegisterSpec for APB_FZ1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb_fz1::R](R) reader structure
impl crate::Readable for APB_FZ1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb_fz1::W](W) writer structure
impl crate::Writable for APB_FZ1_SPEC {
    type Writer = W;
}
///`reset()` method sets APB_FZ1 to value 0
impl crate::Resettable for APB_FZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
