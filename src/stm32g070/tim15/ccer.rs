///Register `CCER` reader
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCER` writer
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
///Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1E_A {
    ///0: Capture mode disabled / OC1 is not active (see below)
    B_0X0 = 0,
    ///1: Capture mode enabled / OC1 signal is output on the corresponding output pin
    B_0X1 = 1,
}
impl From<CC1E_A> for bool {
    #[inline(always)]
    fn from(variant: CC1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1E` reader - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details.
pub struct CC1E_R(crate::FieldReader<bool, CC1E_A>);
impl CC1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1E_A {
        match self.bits {
            false => CC1E_A::B_0X0,
            true => CC1E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1E_A::B_0X1
    }
}
impl core::ops::Deref for CC1E_R {
    type Target = crate::FieldReader<bool, CC1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1E` writer - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details.
pub struct CC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Capture mode disabled / OC1 is not active (see below)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1E_A::B_0X0)
    }
    ///Capture mode enabled / OC1 signal is output on the corresponding output pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1E_A::B_0X1)
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
///Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1P_A {
    ///0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    B_0X0 = 0,
    ///1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    B_0X1 = 1,
}
impl From<CC1P_A> for bool {
    #[inline(always)]
    fn from(variant: CC1P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1P` reader - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub struct CC1P_R(crate::FieldReader<bool, CC1P_A>);
impl CC1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1P_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1P_A {
        match self.bits {
            false => CC1P_A::B_0X0,
            true => CC1P_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1P_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1P_A::B_0X1
    }
}
impl core::ops::Deref for CC1P_R {
    type Target = crate::FieldReader<bool, CC1P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1P` writer - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub struct CC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1P_A::B_0X0)
    }
    ///OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1P_A::B_0X1)
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
///Capture/Compare 1 complementary output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1NE_A {
    ///0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    B_0X0 = 0,
    ///1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    B_0X1 = 1,
}
impl From<CC1NE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NE` reader - Capture/Compare 1 complementary output enable
pub struct CC1NE_R(crate::FieldReader<bool, CC1NE_A>);
impl CC1NE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1NE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1NE_A {
        match self.bits {
            false => CC1NE_A::B_0X0,
            true => CC1NE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1NE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1NE_A::B_0X1
    }
}
impl core::ops::Deref for CC1NE_R {
    type Target = crate::FieldReader<bool, CC1NE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1NE` writer - Capture/Compare 1 complementary output enable
pub struct CC1NE_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1NE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1NE_A::B_0X0)
    }
    ///On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1NE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=??????00?????? (the channel is configured in output). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1NP_A {
    ///0: OC1N active high
    B_0X0 = 0,
    ///1: OC1N active low
    B_0X1 = 1,
}
impl From<CC1NP_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1NP` reader - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=??????00?????? (the channel is configured in output). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub struct CC1NP_R(crate::FieldReader<bool, CC1NP_A>);
impl CC1NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1NP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CC1NP_A {
        match self.bits {
            false => CC1NP_A::B_0X0,
            true => CC1NP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CC1NP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CC1NP_A::B_0X1
    }
}
impl core::ops::Deref for CC1NP_R {
    type Target = crate::FieldReader<bool, CC1NP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC1NP` writer - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=??????00?????? (the channel is configured in output). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
pub struct CC1NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1NP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CC1NP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///OC1N active high
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1NP_A::B_0X0)
    }
    ///OC1N active low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1NP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
///Field `CC2E` reader - Capture/Compare 2 output enable Refer to CC1E description
pub struct CC2E_R(crate::FieldReader<bool, bool>);
impl CC2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2E` writer - Capture/Compare 2 output enable Refer to CC1E description
pub struct CC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2E_W<'a> {
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
///Field `CC2P` reader - Capture/Compare 2 output polarity Refer to CC1P description
pub struct CC2P_R(crate::FieldReader<bool, bool>);
impl CC2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2P` writer - Capture/Compare 2 output polarity Refer to CC1P description
pub struct CC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2P_W<'a> {
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
///Field `CC2NP` reader - Capture/Compare 2 complementary output polarity Refer to CC1NP description
pub struct CC2NP_R(crate::FieldReader<bool, bool>);
impl CC2NP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2NP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2NP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CC2NP` writer - Capture/Compare 2 complementary output polarity Refer to CC1NP description
pub struct CC2NP_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2NP_W<'a> {
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
impl R {
    ///Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details.
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=??????00?????? (the channel is configured in output). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - Capture/Compare 2 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details.
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W {
        CC1E_W { w: self }
    }
    ///Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: this configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W { w: self }
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W {
        CC1NE_W { w: self }
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S=??????00?????? (the channel is configured in output). On channels that have a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W {
        CC1NP_W { w: self }
    }
    ///Bit 4 - Capture/Compare 2 output enable Refer to CC1E description
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W {
        CC2E_W { w: self }
    }
    ///Bit 5 - Capture/Compare 2 output polarity Refer to CC1P description
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W { w: self }
    }
    ///Bit 7 - Capture/Compare 2 complementary output polarity Refer to CC1NP description
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W {
        CC2NP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccer](index.html) module
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccer::R](R) reader structure
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccer::W](W) writer structure
impl crate::Writable for CCER_SPEC {
    type Writer = W;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
