///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Tamper 1 no erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1NOER_A {
    ///0: Tamper 1 event erases the backup registers.
    B_0X0 = 0,
    ///1: Tamper 1 event does not erase the backup registers.
    B_0X1 = 1,
}
impl From<TAMP1NOER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1NOER` reader - Tamper 1 no erase
pub struct TAMP1NOER_R(crate::FieldReader<bool, TAMP1NOER_A>);
impl TAMP1NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1NOER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1NOER_A {
        match self.bits {
            false => TAMP1NOER_A::B_0X0,
            true => TAMP1NOER_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP1NOER_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP1NOER_A::B_0X1
    }
}
impl core::ops::Deref for TAMP1NOER_R {
    type Target = crate::FieldReader<bool, TAMP1NOER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP1NOER` writer - Tamper 1 no erase
pub struct TAMP1NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1NOER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper 1 event erases the backup registers.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP1NOER_A::B_0X0)
    }
    ///Tamper 1 event does not erase the backup registers.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP1NOER_A::B_0X1)
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
///Tamper 2 no erase
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP2NOER_A {
    ///0: Tamper 2 event erases the backup registers.
    B_0X0 = 0,
    ///1: Tamper 2 event does not erase the backup registers.
    B_0X1 = 1,
}
impl From<TAMP2NOER_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2NOER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP2NOER` reader - Tamper 2 no erase
pub struct TAMP2NOER_R(crate::FieldReader<bool, TAMP2NOER_A>);
impl TAMP2NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2NOER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP2NOER_A {
        match self.bits {
            false => TAMP2NOER_A::B_0X0,
            true => TAMP2NOER_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP2NOER_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP2NOER_A::B_0X1
    }
}
impl core::ops::Deref for TAMP2NOER_R {
    type Target = crate::FieldReader<bool, TAMP2NOER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP2NOER` writer - Tamper 2 no erase
pub struct TAMP2NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2NOER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2NOER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper 2 event erases the backup registers.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP2NOER_A::B_0X0)
    }
    ///Tamper 2 event does not erase the backup registers.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP2NOER_A::B_0X1)
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
///Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1MSK_A {
    ///0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection.
    B_0X0 = 0,
    ///1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased.
    B_0X1 = 1,
}
impl From<TAMP1MSK_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MSK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub struct TAMP1MSK_R(crate::FieldReader<bool, TAMP1MSK_A>);
impl TAMP1MSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1MSK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1MSK_A {
        match self.bits {
            false => TAMP1MSK_A::B_0X0,
            true => TAMP1MSK_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP1MSK_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP1MSK_A::B_0X1
    }
}
impl core::ops::Deref for TAMP1MSK_R {
    type Target = crate::FieldReader<bool, TAMP1MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub struct TAMP1MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1MSK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to allow next tamper event detection.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP1MSK_A::B_0X0)
    }
    ///Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by hardware. The backup registers are not erased.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP1MSK_A::B_0X1)
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
///Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP2MSK_A {
    ///0: Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection.
    B_0X0 = 0,
    ///1: Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased.
    B_0X1 = 1,
}
impl From<TAMP2MSK_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2MSK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub struct TAMP2MSK_R(crate::FieldReader<bool, TAMP2MSK_A>);
impl TAMP2MSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2MSK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP2MSK_A {
        match self.bits {
            false => TAMP2MSK_A::B_0X0,
            true => TAMP2MSK_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP2MSK_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP2MSK_A::B_0X1
    }
}
impl core::ops::Deref for TAMP2MSK_R {
    type Target = crate::FieldReader<bool, TAMP2MSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub struct TAMP2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2MSK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2MSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to allow next tamper event detection.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP2MSK_A::B_0X0)
    }
    ///Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by hardware. The backup registers are not erased.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP2MSK_A::B_0X1)
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
///Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1TRG_A {
    ///0: If TAMPFLT â  00 Tamper 1 input staying low triggers a tamper detection event.
    B_0X0 = 0,
    ///1: If TAMPFLT â  00 Tamper 1 input staying high triggers a tamper detection event.
    B_0X1 = 1,
}
impl From<TAMP1TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1TRG` reader - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
pub struct TAMP1TRG_R(crate::FieldReader<bool, TAMP1TRG_A>);
impl TAMP1TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1TRG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP1TRG_A {
        match self.bits {
            false => TAMP1TRG_A::B_0X0,
            true => TAMP1TRG_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP1TRG_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP1TRG_A::B_0X1
    }
}
impl core::ops::Deref for TAMP1TRG_R {
    type Target = crate::FieldReader<bool, TAMP1TRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP1TRG` writer - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
pub struct TAMP1TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1TRG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP1TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///If TAMPFLT â 00 Tamper 1 input staying low triggers a tamper detection event.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::B_0X0)
    }
    ///If TAMPFLT â 00 Tamper 1 input staying high triggers a tamper detection event.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::B_0X1)
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
///Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP2TRG_A {
    ///0: If TAMPFLT â  00 Tamper 2 input staying low triggers a tamper detection event.
    B_0X0 = 0,
    ///1: If TAMPFLT â  00 Tamper 2 input staying high triggers a tamper detection event.
    B_0X1 = 1,
}
impl From<TAMP2TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2TRG_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP2TRG` reader - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
pub struct TAMP2TRG_R(crate::FieldReader<bool, TAMP2TRG_A>);
impl TAMP2TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2TRG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMP2TRG_A {
        match self.bits {
            false => TAMP2TRG_A::B_0X0,
            true => TAMP2TRG_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TAMP2TRG_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TAMP2TRG_A::B_0X1
    }
}
impl core::ops::Deref for TAMP2TRG_R {
    type Target = crate::FieldReader<bool, TAMP2TRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TAMP2TRG` writer - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
pub struct TAMP2TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2TRG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TAMP2TRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///If TAMPFLT â 00 Tamper 2 input staying low triggers a tamper detection event.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TAMP2TRG_A::B_0X0)
    }
    ///If TAMPFLT â 00 Tamper 2 input staying high triggers a tamper detection event.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TAMP2TRG_A::B_0X1)
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
impl R {
    ///Bit 0 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Tamper 1 no erase
    #[inline(always)]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W {
        TAMP1NOER_W { w: self }
    }
    ///Bit 1 - Tamper 2 no erase
    #[inline(always)]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W {
        TAMP2NOER_W { w: self }
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W {
        TAMP1MSK_W { w: self }
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W {
        TAMP2MSK_W { w: self }
    }
    ///Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    ///Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W {
        TAMP2TRG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
