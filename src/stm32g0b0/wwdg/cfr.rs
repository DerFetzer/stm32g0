///Register `CFR` reader
pub struct R(crate::R<CFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFR` writer
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub struct W_R(crate::FieldReader<u8, u8>);
impl W_R {
    pub(crate) fn new(bits: u8) -> Self {
        W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub struct W_W<'a> {
    w: &'a mut W,
}
impl<'a> W_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
///Field `EWI` reader - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
pub struct EWI_R(crate::FieldReader<bool, bool>);
impl EWI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EWI` writer - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
pub struct EWI_W<'a> {
    w: &'a mut W,
}
impl<'a> EWI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Timer base The timebase of the prescaler can be modified as follows:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDGTB_A {
    ///0: CK Counter Clock (PCLK div 4096) div 1
    B_0X0 = 0,
    ///1: CK Counter Clock (PCLK div 4096) div 2
    B_0X1 = 1,
    ///2: CK Counter Clock (PCLK div 4096) div 4
    B_0X2 = 2,
    ///3: CK Counter Clock (PCLK div 4096) div 8
    B_0X3 = 3,
    ///4: CK Counter Clock (PCLK div 4096) div 16
    B_0X4 = 4,
    ///5: CK Counter Clock (PCLK div 4096) div 32
    B_0X5 = 5,
    ///6: CK Counter Clock (PCLK div 4096) div 64
    B_0X6 = 6,
    ///7: CK Counter Clock (PCLK div 4096) div 128
    B_0X7 = 7,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
///Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:
pub struct WDGTB_R(crate::FieldReader<u8, WDGTB_A>);
impl WDGTB_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDGTB_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::B_0X0,
            1 => WDGTB_A::B_0X1,
            2 => WDGTB_A::B_0X2,
            3 => WDGTB_A::B_0X3,
            4 => WDGTB_A::B_0X4,
            5 => WDGTB_A::B_0X5,
            6 => WDGTB_A::B_0X6,
            7 => WDGTB_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WDGTB_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WDGTB_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == WDGTB_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == WDGTB_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == WDGTB_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == WDGTB_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == WDGTB_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == WDGTB_A::B_0X7
    }
}
impl core::ops::Deref for WDGTB_R {
    type Target = crate::FieldReader<u8, WDGTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:
pub struct WDGTB_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGTB_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WDGTB_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///CK Counter Clock (PCLK div 4096) div 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X0)
    }
    ///CK Counter Clock (PCLK div 4096) div 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X1)
    }
    ///CK Counter Clock (PCLK div 4096) div 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X2)
    }
    ///CK Counter Clock (PCLK div 4096) div 8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X3)
    }
    ///CK Counter Clock (PCLK div 4096) div 16
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X4)
    }
    ///CK Counter Clock (PCLK div 4096) div 32
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X5)
    }
    ///CK Counter Clock (PCLK div 4096) div 64
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X6)
    }
    ///CK Counter Clock (PCLK div 4096) div 128
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
impl R {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W { w: self }
    }
    ///Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset.
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W {
        EWI_W { w: self }
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W {
        WDGTB_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfr](index.html) module
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfr::R](R) reader structure
impl crate::Readable for CFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfr::W](W) writer structure
impl crate::Writable for CFR_SPEC {
    type Writer = W;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
