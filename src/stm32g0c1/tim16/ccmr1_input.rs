///Register `CCMR1_Input` reader
pub struct R(crate::R<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1_Input` writer
pub struct W(crate::W<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_INPUT_SPEC>;
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
impl From<crate::W<CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â in TIMx_CCER).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    ///0: CC1 channel is configured as output
    B_0X0 = 0,
    ///1: CC1 channel is configured as input, IC1 is mapped on TI1
    B_0X1 = 1,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
///Field `CC1S` reader - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â in TIMx_CCER).
pub struct CC1S_R(crate::FieldReader<u8, CC1S_A>);
impl CC1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC1S_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1S_A> {
        match self.bits {
            0 => Some(CC1S_A::B_0X0),
            1 => Some(CC1S_A::B_0X1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1S_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1S_A::B_0X1
    }
}
impl core::ops::Deref for CC1S_R {
    type Target = crate::FieldReader<u8, CC1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1S` writer - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â in TIMx_CCER).
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///CC1 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X0)
    }
    ///CC1 channel is configured as input, IC1 is mapped on TI1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X1)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â0â (TIMx_CCER register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1PSC_A {
    ///0: no prescaler, capture is done each time an edge is detected on the capture input.
    B_0X0 = 0,
    ///1: capture is done once every 2 events
    B_0X1 = 1,
    ///2: capture is done once every 4 events
    B_0X2 = 2,
    ///3: capture is done once every 8 events
    B_0X3 = 3,
}
impl From<IC1PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC_A) -> Self {
        variant as _
    }
}
///Field `IC1PSC` reader - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â0â (TIMx_CCER register).
pub struct IC1PSC_R(crate::FieldReader<u8, IC1PSC_A>);
impl IC1PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1PSC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1PSC_A {
        match self.bits {
            0 => IC1PSC_A::B_0X0,
            1 => IC1PSC_A::B_0X1,
            2 => IC1PSC_A::B_0X2,
            3 => IC1PSC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IC1PSC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IC1PSC_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == IC1PSC_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == IC1PSC_A::B_0X3
    }
}
impl core::ops::Deref for IC1PSC_R {
    type Target = crate::FieldReader<u8, IC1PSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IC1PSC` writer - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â0â (TIMx_CCER register).
pub struct IC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1PSC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC1PSC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///no prescaler, capture is done each time an edge is detected on the capture input.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X0)
    }
    ///capture is done once every 2 events
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X1)
    }
    ///capture is done once every 4 events
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X2)
    }
    ///capture is done once every 8 events
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1F_A {
    ///0: No filter, sampling is done at fDTS
    B_0X0 = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    B_0X1 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    B_0X2 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    B_0X3 = 3,
    ///4: fSAMPLING=fDTS/2, N=
    B_0X4 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    B_0X5 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    B_0X6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    B_0X7 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    B_0X8 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    B_0X9 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    B_0XA = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    B_0XB = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    B_0XC = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    B_0XD = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    B_0XE = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    B_0XF = 15,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
///Field `IC1F` reader - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub struct IC1F_R(crate::FieldReader<u8, IC1F_A>);
impl IC1F_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::B_0X0,
            1 => IC1F_A::B_0X1,
            2 => IC1F_A::B_0X2,
            3 => IC1F_A::B_0X3,
            4 => IC1F_A::B_0X4,
            5 => IC1F_A::B_0X5,
            6 => IC1F_A::B_0X6,
            7 => IC1F_A::B_0X7,
            8 => IC1F_A::B_0X8,
            9 => IC1F_A::B_0X9,
            10 => IC1F_A::B_0XA,
            11 => IC1F_A::B_0XB,
            12 => IC1F_A::B_0XC,
            13 => IC1F_A::B_0XD,
            14 => IC1F_A::B_0XE,
            15 => IC1F_A::B_0XF,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IC1F_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IC1F_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == IC1F_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == IC1F_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == IC1F_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == IC1F_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == IC1F_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == IC1F_A::B_0X7
    }
    ///Checks if the value of the field is `B_0X8`
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == IC1F_A::B_0X8
    }
    ///Checks if the value of the field is `B_0X9`
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        **self == IC1F_A::B_0X9
    }
    ///Checks if the value of the field is `B_0XA`
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        **self == IC1F_A::B_0XA
    }
    ///Checks if the value of the field is `B_0XB`
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        **self == IC1F_A::B_0XB
    }
    ///Checks if the value of the field is `B_0XC`
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        **self == IC1F_A::B_0XC
    }
    ///Checks if the value of the field is `B_0XD`
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        **self == IC1F_A::B_0XD
    }
    ///Checks if the value of the field is `B_0XE`
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        **self == IC1F_A::B_0XE
    }
    ///Checks if the value of the field is `B_0XF`
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        **self == IC1F_A::B_0XF
    }
}
impl core::ops::Deref for IC1F_R {
    type Target = crate::FieldReader<u8, IC1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IC1F` writer - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
pub struct IC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1F_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IC1F_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X0)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X1)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X2)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X3)
    }
    ///fSAMPLING=fDTS/2, N=
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X4)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X5)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X7)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X8)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X9)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XA)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XB)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XC)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XD)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XE)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XF)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â0â (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 1 Selection This bit-field defines the direction of the channel (input/output) as well as the used input. Others: Reserved Note: CC1S bits are writable only when the channel is OFF (CC1E = '0â in TIMx_CCER).
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    ///Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â0â (TIMx_CCER register).
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W { w: self }
    }
    ///Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare mode register 1 (input mode)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1_input](index.html) module
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1_input::R](R) reader structure
impl crate::Readable for CCMR1_INPUT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1_input::W](W) writer structure
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Writer = W;
}
///`reset()` method sets CCMR1_Input to value 0
impl crate::Resettable for CCMR1_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
