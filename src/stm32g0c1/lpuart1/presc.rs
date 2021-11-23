///Register `PRESC` reader
pub struct R(crate::R<PRESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRESC` writer
pub struct W(crate::W<PRESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESC_SPEC>;
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
impl From<crate::W<PRESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESC_SPEC>) -> Self {
        W(writer)
    }
}
///Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    ///0: input clock not divided
    B_0X0 = 0,
    ///1: input clock divided by 2
    B_0X1 = 1,
    ///2: input clock divided by 4
    B_0X2 = 2,
    ///3: input clock divided by 6
    B_0X3 = 3,
    ///4: input clock divided by 8
    B_0X4 = 4,
    ///5: input clock divided by 10
    B_0X5 = 5,
    ///6: input clock divided by 12
    B_0X6 = 6,
    ///7: input clock divided by 16
    B_0X7 = 7,
    ///8: input clock divided by 32
    B_0X8 = 8,
    ///9: input clock divided by 64
    B_0X9 = 9,
    ///10: input clock divided by 128
    B_0XA = 10,
    ///11: input clock divided by 256
    B_0XB = 11,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
///Field `PRESCALER` reader - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
pub struct PRESCALER_R(crate::FieldReader<u8, PRESCALER_A>);
impl PRESCALER_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALER_A> {
        match self.bits {
            0 => Some(PRESCALER_A::B_0X0),
            1 => Some(PRESCALER_A::B_0X1),
            2 => Some(PRESCALER_A::B_0X2),
            3 => Some(PRESCALER_A::B_0X3),
            4 => Some(PRESCALER_A::B_0X4),
            5 => Some(PRESCALER_A::B_0X5),
            6 => Some(PRESCALER_A::B_0X6),
            7 => Some(PRESCALER_A::B_0X7),
            8 => Some(PRESCALER_A::B_0X8),
            9 => Some(PRESCALER_A::B_0X9),
            10 => Some(PRESCALER_A::B_0XA),
            11 => Some(PRESCALER_A::B_0XB),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PRESCALER_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PRESCALER_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == PRESCALER_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == PRESCALER_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == PRESCALER_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == PRESCALER_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == PRESCALER_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == PRESCALER_A::B_0X7
    }
    ///Checks if the value of the field is `B_0X8`
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == PRESCALER_A::B_0X8
    }
    ///Checks if the value of the field is `B_0X9`
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        **self == PRESCALER_A::B_0X9
    }
    ///Checks if the value of the field is `B_0XA`
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        **self == PRESCALER_A::B_0XA
    }
    ///Checks if the value of the field is `B_0XB`
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        **self == PRESCALER_A::B_0XB
    }
}
impl core::ops::Deref for PRESCALER_R {
    type Target = crate::FieldReader<u8, PRESCALER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PRESCALER` writer - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///input clock not divided
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X0)
    }
    ///input clock divided by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X1)
    }
    ///input clock divided by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X2)
    }
    ///input clock divided by 6
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X3)
    }
    ///input clock divided by 8
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X4)
    }
    ///input clock divided by 10
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X5)
    }
    ///input clock divided by 12
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X6)
    }
    ///input clock divided by 16
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X7)
    }
    ///input clock divided by 32
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X8)
    }
    ///input clock divided by 64
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X9)
    }
    ///input clock divided by 128
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0XA)
    }
    ///input clock divided by 256
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0XB)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Clock prescaler The LPUART input clock can be divided by a prescaler: Remaining combinations: Reserved. Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPUART prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [presc](index.html) module
pub struct PRESC_SPEC;
impl crate::RegisterSpec for PRESC_SPEC {
    type Ux = u32;
}
///`read()` method returns [presc::R](R) reader structure
impl crate::Readable for PRESC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [presc::W](W) writer structure
impl crate::Writable for PRESC_SPEC {
    type Writer = W;
}
///`reset()` method sets PRESC to value 0
impl crate::Resettable for PRESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
