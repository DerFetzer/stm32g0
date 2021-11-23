///Register `TXBTIE` reader
pub struct R(crate::R<TXBTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTIE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBTIE` writer
pub struct W(crate::W<TXBTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTIE_SPEC>;
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
impl From<crate::W<TXBTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTIE_SPEC>) -> Self {
        W(writer)
    }
}
///Transmission interrupt enable Each Tx buffer has its own TIE bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIE_A {
    ///0: Transmission interrupt disabled
    B_0X0 = 0,
    ///1: Transmission interrupt enable
    B_0X1 = 1,
}
impl From<TIE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as _
    }
}
///Field `TIE` reader - Transmission interrupt enable Each Tx buffer has its own TIE bit.
pub struct TIE_R(crate::FieldReader<u8, TIE_A>);
impl TIE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TIE_A> {
        match self.bits {
            0 => Some(TIE_A::B_0X0),
            1 => Some(TIE_A::B_0X1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TIE_A::B_0X1
    }
}
impl core::ops::Deref for TIE_R {
    type Target = crate::FieldReader<u8, TIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIE` writer - Transmission interrupt enable Each Tx buffer has its own TIE bit.
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Transmission interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIE_A::B_0X0)
    }
    ///Transmission interrupt enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIE_A::B_0X1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit.
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit.
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx buffer transmission interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbtie](index.html) module
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbtie::R](R) reader structure
impl crate::Readable for TXBTIE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbtie::W](W) writer structure
impl crate::Writable for TXBTIE_SPEC {
    type Writer = W;
}
///`reset()` method sets TXBTIE to value 0
impl crate::Resettable for TXBTIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
