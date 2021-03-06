///Register `APB_FZ2` reader
pub struct R(crate::R<APB_FZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_FZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_FZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_FZ2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB_FZ2` writer
pub struct W(crate::W<APB_FZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_FZ2_SPEC>;
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
impl From<crate::W<APB_FZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_FZ2_SPEC>) -> Self {
        W(writer)
    }
}
///Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM1_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
pub struct DBG_TIM1_STOP_R(crate::FieldReader<bool, DBG_TIM1_STOP_A>);
impl DBG_TIM1_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM1_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM1_STOP_A {
        match self.bits {
            false => DBG_TIM1_STOP_A::B_0X0,
            true => DBG_TIM1_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM1_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM1_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM1_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM1_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
pub struct DBG_TIM1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM1_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM1_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM1_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM1_STOP_A::B_0X1)
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
///Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM14_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM14_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM14_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
pub struct DBG_TIM14_STOP_R(crate::FieldReader<bool, DBG_TIM14_STOP_A>);
impl DBG_TIM14_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM14_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM14_STOP_A {
        match self.bits {
            false => DBG_TIM14_STOP_A::B_0X0,
            true => DBG_TIM14_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM14_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM14_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM14_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM14_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
pub struct DBG_TIM14_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM14_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM14_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM14_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM14_STOP_A::B_0X1)
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
///Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM15_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM15_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM15_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM15_STOP` reader - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.
pub struct DBG_TIM15_STOP_R(crate::FieldReader<bool, DBG_TIM15_STOP_A>);
impl DBG_TIM15_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM15_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM15_STOP_A {
        match self.bits {
            false => DBG_TIM15_STOP_A::B_0X0,
            true => DBG_TIM15_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM15_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM15_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM15_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM15_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM15_STOP` writer - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.
pub struct DBG_TIM15_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM15_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM15_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM15_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM15_STOP_A::B_0X1)
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
///Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM16_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM16_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM16_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
pub struct DBG_TIM16_STOP_R(crate::FieldReader<bool, DBG_TIM16_STOP_A>);
impl DBG_TIM16_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM16_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM16_STOP_A {
        match self.bits {
            false => DBG_TIM16_STOP_A::B_0X0,
            true => DBG_TIM16_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM16_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM16_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM16_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM16_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
pub struct DBG_TIM16_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM16_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM16_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM16_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM16_STOP_A::B_0X1)
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
///Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_TIM17_STOP_A {
    ///0: Enable
    B_0X0 = 0,
    ///1: Disable
    B_0X1 = 1,
}
impl From<DBG_TIM17_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM17_STOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
pub struct DBG_TIM17_STOP_R(crate::FieldReader<bool, DBG_TIM17_STOP_A>);
impl DBG_TIM17_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM17_STOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIM17_STOP_A {
        match self.bits {
            false => DBG_TIM17_STOP_A::B_0X0,
            true => DBG_TIM17_STOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DBG_TIM17_STOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DBG_TIM17_STOP_A::B_0X1
    }
}
impl core::ops::Deref for DBG_TIM17_STOP_R {
    type Target = crate::FieldReader<bool, DBG_TIM17_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
pub struct DBG_TIM17_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM17_STOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DBG_TIM17_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBG_TIM17_STOP_A::B_0X0)
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBG_TIM17_STOP_A::B_0X1)
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
    ///Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    ///Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W {
        DBG_TIM1_STOP_W { w: self }
    }
    ///Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W {
        DBG_TIM14_STOP_W { w: self }
    }
    ///Bit 16 - Clocking of TIM15 counter when the core is halted This bit enables/disables the clock to the counter of TIM15 when the core is halted: Only available on STM32G071xx and STM32G081xx, reserved on STM32G031xx and STM32G041xx.
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W {
        DBG_TIM15_STOP_W { w: self }
    }
    ///Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W {
        DBG_TIM16_STOP_W { w: self }
    }
    ///Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W {
        DBG_TIM17_STOP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBG APB freeze register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb_fz2](index.html) module
pub struct APB_FZ2_SPEC;
impl crate::RegisterSpec for APB_FZ2_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb_fz2::R](R) reader structure
impl crate::Readable for APB_FZ2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb_fz2::W](W) writer structure
impl crate::Writable for APB_FZ2_SPEC {
    type Writer = W;
}
///`reset()` method sets APB_FZ2 to value 0
impl crate::Resettable for APB_FZ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
