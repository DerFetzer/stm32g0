///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Reverse output data This bit controls the reversal of the bit order of the output data.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_OUT_A {
    ///0: Bit order not affected
    B_0X0 = 0,
    ///1: Bit-reversed output format
    B_0X1 = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `REV_OUT` reader - Reverse output data This bit controls the reversal of the bit order of the output data.
pub struct REV_OUT_R(crate::FieldReader<bool, REV_OUT_A>);
impl REV_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REV_OUT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::B_0X0,
            true => REV_OUT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == REV_OUT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == REV_OUT_A::B_0X1
    }
}
impl core::ops::Deref for REV_OUT_R {
    type Target = crate::FieldReader<bool, REV_OUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REV_OUT` writer - Reverse output data This bit controls the reversal of the bit order of the output data.
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REV_OUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REV_OUT_A::B_0X0)
    }
    ///Bit-reversed output format
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REV_OUT_A::B_0X1)
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
///Reverse input data These bits control the reversal of the bit order of the input data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REV_IN_A {
    ///0: Bit order not affected
    B_0X0 = 0,
    ///1: Bit reversal done by byte
    B_0X1 = 1,
    ///2: Bit reversal done by half-word
    B_0X2 = 2,
    ///3: Bit reversal done by word
    B_0X3 = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
///Field `REV_IN` reader - Reverse input data These bits control the reversal of the bit order of the input data
pub struct REV_IN_R(crate::FieldReader<u8, REV_IN_A>);
impl REV_IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_IN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::B_0X0,
            1 => REV_IN_A::B_0X1,
            2 => REV_IN_A::B_0X2,
            3 => REV_IN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == REV_IN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == REV_IN_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == REV_IN_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == REV_IN_A::B_0X3
    }
}
impl core::ops::Deref for REV_IN_R {
    type Target = crate::FieldReader<u8, REV_IN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REV_IN` writer - Reverse input data These bits control the reversal of the bit order of the input data
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: REV_IN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Bit order not affected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X0)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X1)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X2)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
///Polynomial size These bits control the size of the polynomial.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    ///0: 32 bit polynomial
    B_0X0 = 0,
    ///1: 16 bit polynomial
    B_0X1 = 1,
    ///2: 8 bit polynomial
    B_0X2 = 2,
    ///3: 7 bit polynomial
    B_0X3 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
///Field `POLYSIZE` reader - Polynomial size These bits control the size of the polynomial.
pub struct POLYSIZE_R(crate::FieldReader<u8, POLYSIZE_A>);
impl POLYSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLYSIZE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::B_0X0,
            1 => POLYSIZE_A::B_0X1,
            2 => POLYSIZE_A::B_0X2,
            3 => POLYSIZE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == POLYSIZE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == POLYSIZE_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == POLYSIZE_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == POLYSIZE_A::B_0X3
    }
}
impl core::ops::Deref for POLYSIZE_R {
    type Target = crate::FieldReader<u8, POLYSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `POLYSIZE` writer - Polynomial size These bits control the size of the polynomial.
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: POLYSIZE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///32 bit polynomial
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X0)
    }
    ///16 bit polynomial
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X1)
    }
    ///8 bit polynomial
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X2)
    }
    ///7 bit polynomial
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
///Field `RESET` writer - RESET bit
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
impl R {
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data.
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial.
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    ///Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data.
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    ///Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    ///Bits 3:4 - Polynomial size These bits control the size of the polynomial.
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
    ///Bit 0 - RESET bit
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
