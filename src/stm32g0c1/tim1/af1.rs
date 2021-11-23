///Register `AF1` reader
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AF1` writer
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
///BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâs BRK input. BKIN input is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKINE_A {
    ///0: BKIN input disabled
    B_0X0 = 0,
    ///1: BKIN input enabled
    B_0X1 = 1,
}
impl From<BKINE_A> for bool {
    #[inline(always)]
    fn from(variant: BKINE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKINE` reader - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâs BRK input. BKIN input is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKINE_R(crate::FieldReader<bool, BKINE_A>);
impl BKINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKINE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKINE_A {
        match self.bits {
            false => BKINE_A::B_0X0,
            true => BKINE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BKINE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BKINE_A::B_0X1
    }
}
impl core::ops::Deref for BKINE_R {
    type Target = crate::FieldReader<bool, BKINE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKINE` writer - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâs BRK input. BKIN input is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKINE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKINE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///BKIN input disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKINE_A::B_0X0)
    }
    ///BKIN input enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKINE_A::B_0X1)
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
///BRK COMP1 enable This bit enables the COMP1 for the timerâs BRK input. COMP1 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP1E_A {
    ///0: COMP1 input disabled
    B_0X0 = 0,
    ///1: COMP1 input enabled
    B_0X1 = 1,
}
impl From<BKCMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKCMP1E` reader - BRK COMP1 enable This bit enables the COMP1 for the timerâs BRK input. COMP1 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP1E_R(crate::FieldReader<bool, BKCMP1E_A>);
impl BKCMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP1E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKCMP1E_A {
        match self.bits {
            false => BKCMP1E_A::B_0X0,
            true => BKCMP1E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BKCMP1E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BKCMP1E_A::B_0X1
    }
}
impl core::ops::Deref for BKCMP1E_R {
    type Target = crate::FieldReader<bool, BKCMP1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKCMP1E` writer - BRK COMP1 enable This bit enables the COMP1 for the timerâs BRK input. COMP1 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKCMP1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COMP1 input disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP1E_A::B_0X0)
    }
    ///COMP1 input enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP1E_A::B_0X1)
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
///BRK COMP2 enable This bit enables the COMP2 for the timerâs BRK input. COMP2 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP2E_A {
    ///0: COMP2 input disabled
    B_0X0 = 0,
    ///1: COMP2 input enabled
    B_0X1 = 1,
}
impl From<BKCMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2E_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKCMP2E` reader - BRK COMP2 enable This bit enables the COMP2 for the timerâs BRK input. COMP2 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP2E_R(crate::FieldReader<bool, BKCMP2E_A>);
impl BKCMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP2E_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKCMP2E_A {
        match self.bits {
            false => BKCMP2E_A::B_0X0,
            true => BKCMP2E_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BKCMP2E_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BKCMP2E_A::B_0X1
    }
}
impl core::ops::Deref for BKCMP2E_R {
    type Target = crate::FieldReader<bool, BKCMP2E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKCMP2E` writer - BRK COMP2 enable This bit enables the COMP2 for the timerâs BRK input. COMP2 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2E_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKCMP2E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COMP2 input disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP2E_A::B_0X0)
    }
    ///COMP2 input enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP2E_A::B_0X1)
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
///BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKINP_A {
    ///0: BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    B_0X0 = 0,
    ///1: BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)
    B_0X1 = 1,
}
impl From<BKINP_A> for bool {
    #[inline(always)]
    fn from(variant: BKINP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKINP` reader - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKINP_R(crate::FieldReader<bool, BKINP_A>);
impl BKINP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKINP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKINP_A {
        match self.bits {
            false => BKINP_A::B_0X0,
            true => BKINP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BKINP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BKINP_A::B_0X1
    }
}
impl core::ops::Deref for BKINP_R {
    type Target = crate::FieldReader<bool, BKINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKINP` writer - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKINP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKINP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///BKIN input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKINP_A::B_0X0)
    }
    ///BKIN input polarity is inverted (active high if BKP=0, active low if BKP=1)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKINP_A::B_0X1)
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
///BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP1P_A {
    ///0: COMP1 input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    B_0X0 = 0,
    ///1: COMP1 input polarity is inverted (active high if BKP=0, active low if BKP=1)
    B_0X1 = 1,
}
impl From<BKCMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKCMP1P` reader - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP1P_R(crate::FieldReader<bool, BKCMP1P_A>);
impl BKCMP1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP1P_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKCMP1P_A {
        match self.bits {
            false => BKCMP1P_A::B_0X0,
            true => BKCMP1P_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BKCMP1P_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BKCMP1P_A::B_0X1
    }
}
impl core::ops::Deref for BKCMP1P_R {
    type Target = crate::FieldReader<bool, BKCMP1P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKCMP1P` writer - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKCMP1P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COMP1 input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP1P_A::B_0X0)
    }
    ///COMP1 input polarity is inverted (active high if BKP=0, active low if BKP=1)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP1P_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP2P_A {
    ///0: COMP2 input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    B_0X0 = 0,
    ///1: COMP2 input polarity is inverted (active high if BKP=0, active low if BKP=1)
    B_0X1 = 1,
}
impl From<BKCMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2P_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BKCMP2P` reader - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP2P_R(crate::FieldReader<bool, BKCMP2P_A>);
impl BKCMP2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKCMP2P_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKCMP2P_A {
        match self.bits {
            false => BKCMP2P_A::B_0X0,
            true => BKCMP2P_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BKCMP2P_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BKCMP2P_A::B_0X1
    }
}
impl core::ops::Deref for BKCMP2P_R {
    type Target = crate::FieldReader<bool, BKCMP2P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BKCMP2P` writer - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct BKCMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2P_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BKCMP2P_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///COMP2 input polarity is not inverted (active low if BKP=0, active high if BKP=1)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP2P_A::B_0X0)
    }
    ///COMP2 input polarity is inverted (active high if BKP=0, active low if BKP=1)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP2P_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRSEL_A {
    ///0: ETR legacy mode
    B_0X0 = 0,
    ///1: COMP1 output
    B_0X1 = 1,
    ///2: COMP2 output
    B_0X2 = 2,
    ///3: ADC1 AWD1
    B_0X3 = 3,
    ///4: ADC1 AWD2
    B_0X4 = 4,
    ///5: ADC1 AWD3
    B_0X5 = 5,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
///Field `ETRSEL` reader - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct ETRSEL_R(crate::FieldReader<u8, ETRSEL_A>);
impl ETRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETRSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ETRSEL_A> {
        match self.bits {
            0 => Some(ETRSEL_A::B_0X0),
            1 => Some(ETRSEL_A::B_0X1),
            2 => Some(ETRSEL_A::B_0X2),
            3 => Some(ETRSEL_A::B_0X3),
            4 => Some(ETRSEL_A::B_0X4),
            5 => Some(ETRSEL_A::B_0X5),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ETRSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ETRSEL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == ETRSEL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == ETRSEL_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == ETRSEL_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == ETRSEL_A::B_0X5
    }
}
impl core::ops::Deref for ETRSEL_R {
    type Target = crate::FieldReader<u8, ETRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETRSEL` writer - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///ETR legacy mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X0)
    }
    ///COMP1 output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X1)
    }
    ///COMP2 output
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X2)
    }
    ///ADC1 AWD1
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X3)
    }
    ///ADC1 AWD2
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X4)
    }
    ///ADC1 AWD3
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X5)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
impl R {
    ///Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâs BRK input. BKIN input is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timerâs BRK input. COMP1 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timerâs BRK input. COMP2 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâs BRK input. BKIN input is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W {
        BKINE_W { w: self }
    }
    ///Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timerâs BRK input. COMP1 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W {
        BKCMP1E_W { w: self }
    }
    ///Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timerâs BRK input. COMP2 output is 'ORedâ with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W {
        BKCMP2E_W { w: self }
    }
    ///Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W {
        BKINP_W { w: self }
    }
    ///Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W {
        BKCMP1P_W { w: self }
    }
    ///Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W {
        BKCMP2P_W { w: self }
    }
    ///Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [af1](index.html) module
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [af1::R](R) reader structure
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [af1::W](W) writer structure
impl crate::Writable for AF1_SPEC {
    type Writer = W;
}
///`reset()` method sets AF1 to value 0x01
impl crate::Resettable for AF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
