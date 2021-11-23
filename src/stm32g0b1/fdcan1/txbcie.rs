///Register `TXBCIE` reader
pub struct R(crate::R<TXBCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCIE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBCIE` writer
pub struct W(crate::W<TXBCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCIE_SPEC>;
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
impl From<crate::W<TXBCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCIE_SPEC>) -> Self {
        W(writer)
    }
}
///Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFIE_A {
    ///0: Cancellation finished interrupt disabled
    B_0X0 = 0,
    ///1: Cancellation finished interrupt enabled
    B_0X1 = 1,
}
impl From<CFIE_A> for u8 {
    #[inline(always)]
    fn from(variant: CFIE_A) -> Self {
        variant as _
    }
}
///Field `CFIE` reader - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
pub struct CFIE_R(crate::FieldReader<u8, CFIE_A>);
impl CFIE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CFIE_A> {
        match self.bits {
            0 => Some(CFIE_A::B_0X0),
            1 => Some(CFIE_A::B_0X1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CFIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CFIE_A::B_0X1
    }
}
impl core::ops::Deref for CFIE_R {
    type Target = crate::FieldReader<u8, CFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CFIE` writer - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
pub struct CFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CFIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Cancellation finished interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CFIE_A::B_0X0)
    }
    ///Cancellation finished interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CFIE_A::B_0X1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Cancellation finished interrupt enable. Each Tx buffer has its own CFIE bit.
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W {
        CFIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx buffer cancellation finished interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbcie](index.html) module
pub struct TXBCIE_SPEC;
impl crate::RegisterSpec for TXBCIE_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbcie::R](R) reader structure
impl crate::Readable for TXBCIE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbcie::W](W) writer structure
impl crate::Writable for TXBCIE_SPEC {
    type Writer = W;
}
///`reset()` method sets TXBCIE to value 0
impl crate::Resettable for TXBCIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
