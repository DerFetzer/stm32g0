///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDY_A {
    ///0: The RNG_DR register is not yet valid, no random data is available.
    B_0X0 = 0,
    ///1: The RNG_DR register contains valid random data.
    B_0X1 = 1,
}
impl From<DRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRDY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DRDY` reader - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1.
pub struct DRDY_R(crate::FieldReader<bool, DRDY_A>);
impl DRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DRDY_A {
        match self.bits {
            false => DRDY_A::B_0X0,
            true => DRDY_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DRDY_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DRDY_A::B_0X1
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, DRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECS_A {
    ///0: The RNG clock is correct (fRNGCLK> fHCLK/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered.
    B_0X0 = 0,
    ///1: The RNG clock is too slow (fRNGCLK< fHCLK/32).
    B_0X1 = 1,
}
impl From<CECS_A> for bool {
    #[inline(always)]
    fn from(variant: CECS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CECS` reader - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
pub struct CECS_R(crate::FieldReader<bool, CECS_A>);
impl CECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CECS_A {
        match self.bits {
            false => CECS_A::B_0X0,
            true => CECS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CECS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CECS_A::B_0X1
    }
}
impl core::ops::Deref for CECS_R {
    type Target = crate::FieldReader<bool, CECS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (??????0?????? or ??????1??????), or more than 32 consecutive occurrence of two bit patterns (??????01?????? or ??????10??????) Both noise sources have delivered more than 32 consecutive bits at a constant value (??????0?????? or ??????1??????), or more than 16 consecutive occurrence of two bit patterns (??????01?????? or ??????10??????)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECS_A {
    ///0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered.
    B_0X0 = 0,
    ///1: At least one of the following faulty sequence has been detected:
    B_0X1 = 1,
}
impl From<SECS_A> for bool {
    #[inline(always)]
    fn from(variant: SECS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SECS` reader - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (??????0?????? or ??????1??????), or more than 32 consecutive occurrence of two bit patterns (??????01?????? or ??????10??????) Both noise sources have delivered more than 32 consecutive bits at a constant value (??????0?????? or ??????1??????), or more than 16 consecutive occurrence of two bit patterns (??????01?????? or ??????10??????)
pub struct SECS_R(crate::FieldReader<bool, SECS_A>);
impl SECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SECS_A {
        match self.bits {
            false => SECS_A::B_0X0,
            true => SECS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SECS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SECS_A::B_0X1
    }
}
impl core::ops::Deref for SECS_R {
    type Target = crate::FieldReader<bool, SECS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIS_A {
    ///0: The RNG clock is correct (fRNGCLK> fHCLK/32)
    B_0X0 = 0,
    ///1: The RNG has been detected too slow (fRNGCLK< fHCLK/32)
    B_0X1 = 1,
}
impl From<CEIS_A> for bool {
    #[inline(always)]
    fn from(variant: CEIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CEIS` reader - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub struct CEIS_R(crate::FieldReader<bool, CEIS_A>);
impl CEIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CEIS_A {
        match self.bits {
            false => CEIS_A::B_0X0,
            true => CEIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CEIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CEIS_A::B_0X1
    }
}
impl core::ops::Deref for CEIS_R {
    type Target = crate::FieldReader<bool, CEIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CEIS` writer - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub struct CEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CEIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The RNG clock is correct (fRNGCLK> fHCLK/32)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CEIS_A::B_0X0)
    }
    ///The RNG has been detected too slow (fRNGCLK< fHCLK/32)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CEIS_A::B_0X1)
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
///Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIS_A {
    ///0: No faulty sequence detected
    B_0X0 = 0,
    ///1: At least one faulty sequence has been detected. See SECS bit description for details.
    B_0X1 = 1,
}
impl From<SEIS_A> for bool {
    #[inline(always)]
    fn from(variant: SEIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SEIS` reader - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub struct SEIS_R(crate::FieldReader<bool, SEIS_A>);
impl SEIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SEIS_A {
        match self.bits {
            false => SEIS_A::B_0X0,
            true => SEIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SEIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SEIS_A::B_0X1
    }
}
impl core::ops::Deref for SEIS_R {
    type Target = crate::FieldReader<bool, SEIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SEIS` writer - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
pub struct SEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SEIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No faulty sequence detected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SEIS_A::B_0X0)
    }
    ///At least one faulty sequence has been detected. See SECS bit description for details.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SEIS_A::B_0X1)
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
impl R {
    ///Bit 0 - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1.
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (??????0?????? or ??????1??????), or more than 32 consecutive occurrence of two bit patterns (??????01?????? or ??????10??????) Both noise sources have delivered more than 32 consecutive bits at a constant value (??????0?????? or ??????1??????), or more than 16 consecutive occurrence of two bit patterns (??????01?????? or ??????10??????)
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    ///Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W {
        CEIS_W { w: self }
    }
    ///Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W {
        SEIS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
