///Register `CKDIV` reader
pub struct R(crate::R<CKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKDIV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CKDIV` writer
pub struct W(crate::W<CKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKDIV_SPEC>;
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
impl From<crate::W<CKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKDIV_SPEC>) -> Self {
        W(writer)
    }
}
///input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PDIV_A {
    ///0: Divide by 1
    B_0X0 = 0,
    ///1: Divide by 2
    B_0X1 = 1,
    ///2: Divide by 4
    B_0X2 = 2,
    ///3: Divide by 6
    B_0X3 = 3,
    ///4: Divide by 8
    B_0X4 = 4,
    ///5: Divide by 10
    B_0X5 = 5,
    ///6: Divide by 12
    B_0X6 = 6,
    ///7: Divide by 14
    B_0X7 = 7,
    ///8: Divide by 16
    B_0X8 = 8,
    ///9: Divide by 18
    B_0X9 = 9,
    ///10: Divide by 20
    B_0XA = 10,
    ///11: Divide by 22
    B_0XB = 11,
    ///12: Divide by 24
    B_0XC = 12,
    ///13: Divide by 26
    B_0XD = 13,
    ///14: Divide by 28
    B_0XE = 14,
    ///15: Divide by 30
    B_0XF = 15,
}
impl From<PDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PDIV_A) -> Self {
        variant as _
    }
}
///Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct PDIV_R(crate::FieldReader<u8, PDIV_A>);
impl PDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDIV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PDIV_A {
        match self.bits {
            0 => PDIV_A::B_0X0,
            1 => PDIV_A::B_0X1,
            2 => PDIV_A::B_0X2,
            3 => PDIV_A::B_0X3,
            4 => PDIV_A::B_0X4,
            5 => PDIV_A::B_0X5,
            6 => PDIV_A::B_0X6,
            7 => PDIV_A::B_0X7,
            8 => PDIV_A::B_0X8,
            9 => PDIV_A::B_0X9,
            10 => PDIV_A::B_0XA,
            11 => PDIV_A::B_0XB,
            12 => PDIV_A::B_0XC,
            13 => PDIV_A::B_0XD,
            14 => PDIV_A::B_0XE,
            15 => PDIV_A::B_0XF,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PDIV_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PDIV_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == PDIV_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == PDIV_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == PDIV_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == PDIV_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == PDIV_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == PDIV_A::B_0X7
    }
    ///Checks if the value of the field is `B_0X8`
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == PDIV_A::B_0X8
    }
    ///Checks if the value of the field is `B_0X9`
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        **self == PDIV_A::B_0X9
    }
    ///Checks if the value of the field is `B_0XA`
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        **self == PDIV_A::B_0XA
    }
    ///Checks if the value of the field is `B_0XB`
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        **self == PDIV_A::B_0XB
    }
    ///Checks if the value of the field is `B_0XC`
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        **self == PDIV_A::B_0XC
    }
    ///Checks if the value of the field is `B_0XD`
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        **self == PDIV_A::B_0XD
    }
    ///Checks if the value of the field is `B_0XE`
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        **self == PDIV_A::B_0XE
    }
    ///Checks if the value of the field is `B_0XF`
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        **self == PDIV_A::B_0XF
    }
}
impl core::ops::Deref for PDIV_R {
    type Target = crate::FieldReader<u8, PDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDIV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Divide by 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X0)
    }
    ///Divide by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X1)
    }
    ///Divide by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X2)
    }
    ///Divide by 6
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X3)
    }
    ///Divide by 8
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X4)
    }
    ///Divide by 10
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X5)
    }
    ///Divide by 12
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X6)
    }
    ///Divide by 14
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X7)
    }
    ///Divide by 16
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X8)
    }
    ///Divide by 18
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(PDIV_A::B_0X9)
    }
    ///Divide by 20
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(PDIV_A::B_0XA)
    }
    ///Divide by 22
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(PDIV_A::B_0XB)
    }
    ///Divide by 24
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(PDIV_A::B_0XC)
    }
    ///Divide by 26
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(PDIV_A::B_0XD)
    }
    ///Divide by 28
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(PDIV_A::B_0XE)
    }
    ///Divide by 30
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(PDIV_A::B_0XF)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W {
        PDIV_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN CFG clock divider register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ckdiv](index.html) module
pub struct CKDIV_SPEC;
impl crate::RegisterSpec for CKDIV_SPEC {
    type Ux = u32;
}
///`read()` method returns [ckdiv::R](R) reader structure
impl crate::Readable for CKDIV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ckdiv::W](W) writer structure
impl crate::Writable for CKDIV_SPEC {
    type Writer = W;
}
///`reset()` method sets CKDIV to value 0
impl crate::Resettable for CKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
