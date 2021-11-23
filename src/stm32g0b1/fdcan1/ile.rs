///Register `ILE` reader
pub struct R(crate::R<ILE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ILE` writer
pub struct W(crate::W<ILE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILE_SPEC>;
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
impl From<crate::W<ILE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILE_SPEC>) -> Self {
        W(writer)
    }
}
///Enable interrupt line 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT0_A {
    ///0: Interrupt line fdcan_intr1_it disabled
    B_0X0 = 0,
    ///1: Interrupt line fdcan_intr1_it enabled
    B_0X1 = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EINT0` reader - Enable interrupt line 0
pub struct EINT0_R(crate::FieldReader<bool, EINT0_A>);
impl EINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT0_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EINT0_A {
        match self.bits {
            false => EINT0_A::B_0X0,
            true => EINT0_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EINT0_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EINT0_A::B_0X1
    }
}
impl core::ops::Deref for EINT0_R {
    type Target = crate::FieldReader<bool, EINT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EINT0` writer - Enable interrupt line 0
pub struct EINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT0_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EINT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt line fdcan_intr1_it disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EINT0_A::B_0X0)
    }
    ///Interrupt line fdcan_intr1_it enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EINT0_A::B_0X1)
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
///Enable interrupt line 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT1_A {
    ///0: Interrupt line fdcan_intr0_it disabled
    B_0X0 = 0,
    ///1: Interrupt line fdcan_intr0_it enabled
    B_0X1 = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EINT1` reader - Enable interrupt line 1
pub struct EINT1_R(crate::FieldReader<bool, EINT1_A>);
impl EINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EINT1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EINT1_A {
        match self.bits {
            false => EINT1_A::B_0X0,
            true => EINT1_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EINT1_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EINT1_A::B_0X1
    }
}
impl core::ops::Deref for EINT1_R {
    type Target = crate::FieldReader<bool, EINT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EINT1` writer - Enable interrupt line 1
pub struct EINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EINT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt line fdcan_intr0_it disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EINT1_A::B_0X0)
    }
    ///Interrupt line fdcan_intr0_it enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EINT1_A::B_0X1)
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
impl R {
    ///Bit 0 - Enable interrupt line 0
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Enable interrupt line 1
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Enable interrupt line 0
    #[inline(always)]
    pub fn eint0(&mut self) -> EINT0_W {
        EINT0_W { w: self }
    }
    ///Bit 1 - Enable interrupt line 1
    #[inline(always)]
    pub fn eint1(&mut self) -> EINT1_W {
        EINT1_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN interrupt line enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ile](index.html) module
pub struct ILE_SPEC;
impl crate::RegisterSpec for ILE_SPEC {
    type Ux = u32;
}
///`read()` method returns [ile::R](R) reader structure
impl crate::Readable for ILE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ile::W](W) writer structure
impl crate::Writable for ILE_SPEC {
    type Writer = W;
}
///`reset()` method sets ILE to value 0
impl crate::Resettable for ILE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
