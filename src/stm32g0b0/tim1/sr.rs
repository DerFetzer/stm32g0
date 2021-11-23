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
///Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIF_A {
    ///0: No update occurred.
    B_0X0 = 0,
    ///1: Update interrupt pending. This bit is set by hardware when the registers are updated:
    B_0X1 = 1,
}
impl From<UIF_A> for bool {
    #[inline(always)]
    fn from(variant: UIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub struct UIF_R(crate::FieldReader<bool, UIF_A>);
impl UIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UIF_A {
        match self.bits {
            false => UIF_A::B_0X0,
            true => UIF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == UIF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == UIF_A::B_0X1
    }
}
impl core::ops::Deref for UIF_R {
    type Target = crate::FieldReader<bool, UIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
pub struct UIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No update occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UIF_A::B_0X0)
    }
    ///Update interrupt pending. This bit is set by hardware when the registers are updated:
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UIF_A::B_0X1)
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
///Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1IF_A {
    ///0: No compare match / No input capture occurred
    B_0X0 = 0,
    ///1: A compare match or an input capture occurred.
    B_0X1 = 1,
}
impl From<CC1IF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1IF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IF` reader - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
pub struct CC1IF_R(crate::FieldReader<bool, CC1IF_A>);
impl CC1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1IF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1IF_A {
        match self.bits {
            false => CC1IF_A::B_0X0,
            true => CC1IF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1IF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1IF_A::B_0X1
    }
}
impl core::ops::Deref for CC1IF_R {
    type Target = crate::FieldReader<bool, CC1IF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1IF` writer - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
pub struct CC1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1IF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1IF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No compare match / No input capture occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1IF_A::B_0X0)
    }
    ///A compare match or an input capture occurred.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1IF_A::B_0X1)
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
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag Refer to CC1IF description
pub struct CC2IF_R(crate::FieldReader<bool, bool>);
impl CC2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag Refer to CC1IF description
pub struct CC2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `CC3IF` reader - Capture/Compare 3 interrupt flag Refer to CC1IF description
pub struct CC3IF_R(crate::FieldReader<bool, bool>);
impl CC3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC3IF` writer - Capture/Compare 3 interrupt flag Refer to CC1IF description
pub struct CC3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `CC4IF` reader - Capture/Compare 4 interrupt flag Refer to CC1IF description
pub struct CC4IF_R(crate::FieldReader<bool, bool>);
impl CC4IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC4IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC4IF` writer - Capture/Compare 4 interrupt flag Refer to CC1IF description
pub struct CC4IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMIF_A {
    ///0: No COM event occurred.
    B_0X0 = 0,
    ///1: COM interrupt pending.
    B_0X1 = 1,
}
impl From<COMIF_A> for bool {
    #[inline(always)]
    fn from(variant: COMIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` reader - COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.
pub struct COMIF_R(crate::FieldReader<bool, COMIF_A>);
impl COMIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMIF_A {
        match self.bits {
            false => COMIF_A::B_0X0,
            true => COMIF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == COMIF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == COMIF_A::B_0X1
    }
}
impl core::ops::Deref for COMIF_R {
    type Target = crate::FieldReader<bool, COMIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `COMIF` writer - COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.
pub struct COMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: COMIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No COM event occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(COMIF_A::B_0X0)
    }
    ///COM interrupt pending.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(COMIF_A::B_0X1)
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
///Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
    ///0: No trigger event occurred.
    B_0X0 = 0,
    ///1: Trigger interrupt pending.
    B_0X1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TIF` reader - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub struct TIF_R(crate::FieldReader<bool, TIF_A>);
impl TIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::B_0X0,
            true => TIF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TIF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TIF_A::B_0X1
    }
}
impl core::ops::Deref for TIF_R {
    type Target = crate::FieldReader<bool, TIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIF` writer - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
pub struct TIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No trigger event occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIF_A::B_0X0)
    }
    ///Trigger interrupt pending.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIF_A::B_0X1)
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
///Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIF_A {
    ///0: No break event occurred.
    B_0X0 = 0,
    ///1: An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register.
    B_0X1 = 1,
}
impl From<BIF_A> for bool {
    #[inline(always)]
    fn from(variant: BIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` reader - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
pub struct BIF_R(crate::FieldReader<bool, BIF_A>);
impl BIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BIF_A {
        match self.bits {
            false => BIF_A::B_0X0,
            true => BIF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BIF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BIF_A::B_0X1
    }
}
impl core::ops::Deref for BIF_R {
    type Target = crate::FieldReader<bool, BIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BIF` writer - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
pub struct BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> BIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No break event occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BIF_A::B_0X0)
    }
    ///An active level has been detected on the break input. An interrupt is generated if BIE=1 in the TIMx_DIER register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BIF_A::B_0X1)
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
///Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B2IF_A {
    ///0: No break event occurred.
    B_0X0 = 0,
    ///1: An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register.
    B_0X1 = 1,
}
impl From<B2IF_A> for bool {
    #[inline(always)]
    fn from(variant: B2IF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `B2IF` reader - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
pub struct B2IF_R(crate::FieldReader<bool, B2IF_A>);
impl B2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2IF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> B2IF_A {
        match self.bits {
            false => B2IF_A::B_0X0,
            true => B2IF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == B2IF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == B2IF_A::B_0X1
    }
}
impl core::ops::Deref for B2IF_R {
    type Target = crate::FieldReader<bool, B2IF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `B2IF` writer - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
pub struct B2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> B2IF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: B2IF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No break event occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(B2IF_A::B_0X0)
    }
    ///An active level has been detected on the break 2 input. An interrupt is generated if BIE=1 in the TIMx_DIER register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(B2IF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1OF_A {
    ///0: No overcapture has been detected.
    B_0X0 = 0,
    ///1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    B_0X1 = 1,
}
impl From<CC1OF_A> for bool {
    #[inline(always)]
    fn from(variant: CC1OF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â.
pub struct CC1OF_R(crate::FieldReader<bool, CC1OF_A>);
impl CC1OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1OF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1OF_A {
        match self.bits {
            false => CC1OF_A::B_0X0,
            true => CC1OF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1OF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1OF_A::B_0X1
    }
}
impl core::ops::Deref for CC1OF_R {
    type Target = crate::FieldReader<bool, CC1OF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â.
pub struct CC1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1OF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1OF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No overcapture has been detected.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1OF_A::B_0X0)
    }
    ///The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1OF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Field `CC2OF` reader - Capture/Compare 2 overcapture flag Refer to CC1OF description
pub struct CC2OF_R(crate::FieldReader<bool, bool>);
impl CC2OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2OF` writer - Capture/Compare 2 overcapture flag Refer to CC1OF description
pub struct CC2OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `CC3OF` reader - Capture/Compare 3 overcapture flag Refer to CC1OF description
pub struct CC3OF_R(crate::FieldReader<bool, bool>);
impl CC3OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC3OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC3OF` writer - Capture/Compare 3 overcapture flag Refer to CC1OF description
pub struct CC3OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `CC4OF` reader - Capture/Compare 4 overcapture flag Refer to CC1OF description
pub struct CC4OF_R(crate::FieldReader<bool, bool>);
impl CC4OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC4OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC4OF` writer - Capture/Compare 4 overcapture flag Refer to CC1OF description
pub struct CC4OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBIF_A {
    ///0: No break event occurred.
    B_0X0 = 0,
    ///1: An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register.
    B_0X1 = 1,
}
impl From<SBIF_A> for bool {
    #[inline(always)]
    fn from(variant: SBIF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SBIF` reader - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
pub struct SBIF_R(crate::FieldReader<bool, SBIF_A>);
impl SBIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBIF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SBIF_A {
        match self.bits {
            false => SBIF_A::B_0X0,
            true => SBIF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SBIF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SBIF_A::B_0X1
    }
}
impl core::ops::Deref for SBIF_R {
    type Target = crate::FieldReader<bool, SBIF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBIF` writer - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
pub struct SBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SBIF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SBIF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No break event occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SBIF_A::B_0X0)
    }
    ///An active level has been detected on the system break input. An interrupt is generated if BIE=1 in the TIMx_DIER register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SBIF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `CC5IF` reader - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
pub struct CC5IF_R(crate::FieldReader<bool, bool>);
impl CC5IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC5IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC5IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC5IF` writer - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
pub struct CC5IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC5IF_W<'a> {
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
///Field `CC6IF` reader - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
pub struct CC6IF_R(crate::FieldReader<bool, bool>);
impl CC6IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC6IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC6IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC6IF` writer - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
pub struct CC6IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CC6IF_W<'a> {
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
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â.
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag Refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag Refer to CC1OF description
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag Refer to CC1OF description
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 16 - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value (update if repetition counter = 0) and if the UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in TIMx_EGR register, if URS=0 and UDIS=0 in the TIMx_CR1 register. When CNT is reinitialized by a trigger event (refer to control register (TIM1_SMCRTIMx_SMCR)N/A), if URS=0 and UDIS=0 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W {
        UIF_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag This flag is set by hardware. It is cleared by software (input capture or output compare mode) or by reading the TIMx_CCR1 register (input capture mode only). If channel CC1 is configured as output: this flag is set when he content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. When the content of TIMx_CCR1 is greater than the content of TIMx_ARR, the CC1IF bit goes high on the counter overflow (in up-counting and up/down-counting modes) or underflow (in down-counting mode). There are 3 possible options for flag setting in center-aligned mode, refer to the CMS bits in the TIMx_CR1 register for the full description. If channel CC1 is configured as input: this bit is set when counter value has been captured in TIMx_CCR1 register (an edge has been detected on IC1, as per the edge sensitivity defined with the CC1P and CC1NP bits setting, in TIMx_CCER).
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W {
        CC1IF_W { w: self }
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W {
        CC2IF_W { w: self }
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W {
        CC3IF_W { w: self }
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag Refer to CC1IF description
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W {
        CC4IF_W { w: self }
    }
    ///Bit 5 - COM interrupt flag This flag is set by hardware on COM event (when Capture/compare Control bits - CCxE, CCxNE, OCxM - have been updated). It is cleared by software.
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W {
        COMIF_W { w: self }
    }
    ///Bit 6 - Trigger interrupt flag This flag is set by hardware on the TRG trigger event (active edge detected on TRGI input when the slave mode controller is enabled in all modes but gated mode. It is set when the counter starts or stops when gated mode is selected. It is cleared by software.
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W {
        TIF_W { w: self }
    }
    ///Bit 7 - Break interrupt flag This flag is set by hardware as soon as the break input goes active. It can be cleared by software if the break input is not active.
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W {
        BIF_W { w: self }
    }
    ///Bit 8 - Break 2 interrupt flag This flag is set by hardware as soon as the break 2 input goes active. It can be cleared by software if the break 2 input is not active.
    #[inline(always)]
    pub fn b2if(&mut self) -> B2IF_W {
        B2IF_W { w: self }
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag This flag is set by hardware only when the corresponding channel is configured in input capture mode. It is cleared by software by writing it to '0â.
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W {
        CC1OF_W { w: self }
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag Refer to CC1OF description
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W {
        CC2OF_W { w: self }
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag Refer to CC1OF description
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W {
        CC3OF_W { w: self }
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag Refer to CC1OF description
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W {
        CC4OF_W { w: self }
    }
    ///Bit 13 - System Break interrupt flag This flag is set by hardware as soon as the system break input goes active. It can be cleared by software if the system break input is not active. This flag must be reset to re-start PWM operation.
    #[inline(always)]
    pub fn sbif(&mut self) -> SBIF_W {
        SBIF_W { w: self }
    }
    ///Bit 16 - Compare 5 interrupt flag Refer to CC1IF description (Note: Channel 5 can only be configured as output)
    #[inline(always)]
    pub fn cc5if(&mut self) -> CC5IF_W {
        CC5IF_W { w: self }
    }
    ///Bit 17 - Compare 6 interrupt flag Refer to CC1IF description (Note: Channel 6 can only be configured as output)
    #[inline(always)]
    pub fn cc6if(&mut self) -> CC6IF_W {
        CC6IF_W { w: self }
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
