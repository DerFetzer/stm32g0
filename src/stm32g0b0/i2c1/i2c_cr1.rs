///Register `I2C_CR1` reader
pub struct R(crate::R<I2C_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2C_CR1` writer
pub struct W(crate::W<I2C_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CR1_SPEC>;
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
impl From<crate::W<I2C_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    ///0: Peripheral disable
    B_0X0 = 0,
    ///1: Peripheral enable
    B_0X1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.
pub struct PE_R(crate::FieldReader<bool, PE_A>);
impl PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::B_0X0,
            true => PE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PE_A::B_0X1
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<bool, PE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PE` writer - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Peripheral disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PE_A::B_0X0)
    }
    ///Peripheral enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PE_A::B_0X1)
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
///TX Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIE_A {
    ///0: Transmit (TXIS) interrupt disabled
    B_0X0 = 0,
    ///1: Transmit (TXIS) interrupt enabled
    B_0X1 = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIE` reader - TX Interrupt enable
pub struct TXIE_R(crate::FieldReader<bool, TXIE_A>);
impl TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::B_0X0,
            true => TXIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXIE_A::B_0X1
    }
}
impl core::ops::Deref for TXIE_R {
    type Target = crate::FieldReader<bool, TXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXIE` writer - TX Interrupt enable
pub struct TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transmit (TXIS) interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXIE_A::B_0X0)
    }
    ///Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXIE_A::B_0X1)
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
///RX Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    ///0: Receive (RXNE) interrupt disabled
    B_0X0 = 0,
    ///1: Receive (RXNE) interrupt enabled
    B_0X1 = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXIE` reader - RX Interrupt enable
pub struct RXIE_R(crate::FieldReader<bool, RXIE_A>);
impl RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::B_0X0,
            true => RXIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXIE_A::B_0X1
    }
}
impl core::ops::Deref for RXIE_R {
    type Target = crate::FieldReader<bool, RXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXIE` writer - RX Interrupt enable
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Receive (RXNE) interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXIE_A::B_0X0)
    }
    ///Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXIE_A::B_0X1)
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
///Address match Interrupt enable (slave only)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRIE_A {
    ///0: Address match (ADDR) interrupts disabled
    B_0X0 = 0,
    ///1: Address match (ADDR) interrupts enabled
    B_0X1 = 1,
}
impl From<ADDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRIE` reader - Address match Interrupt enable (slave only)
pub struct ADDRIE_R(crate::FieldReader<bool, ADDRIE_A>);
impl ADDRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADDRIE_A {
        match self.bits {
            false => ADDRIE_A::B_0X0,
            true => ADDRIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ADDRIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ADDRIE_A::B_0X1
    }
}
impl core::ops::Deref for ADDRIE_R {
    type Target = crate::FieldReader<bool, ADDRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADDRIE` writer - Address match Interrupt enable (slave only)
pub struct ADDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ADDRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Address match (ADDR) interrupts disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADDRIE_A::B_0X0)
    }
    ///Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADDRIE_A::B_0X1)
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
///Not acknowledge received Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIE_A {
    ///0: Not acknowledge (NACKF) received interrupts disabled
    B_0X0 = 0,
    ///1: Not acknowledge (NACKF) received interrupts enabled
    B_0X1 = 1,
}
impl From<NACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKIE` reader - Not acknowledge received Interrupt enable
pub struct NACKIE_R(crate::FieldReader<bool, NACKIE_A>);
impl NACKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACKIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NACKIE_A {
        match self.bits {
            false => NACKIE_A::B_0X0,
            true => NACKIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == NACKIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == NACKIE_A::B_0X1
    }
}
impl core::ops::Deref for NACKIE_R {
    type Target = crate::FieldReader<bool, NACKIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NACKIE` writer - Not acknowledge received Interrupt enable
pub struct NACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NACKIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Not acknowledge (NACKF) received interrupts disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NACKIE_A::B_0X0)
    }
    ///Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NACKIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Stop detection Interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPIE_A {
    ///0: Stop detection (STOPF) interrupt disabled
    B_0X0 = 0,
    ///1: Stop detection (STOPF) interrupt enabled
    B_0X1 = 1,
}
impl From<STOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPIE` reader - Stop detection Interrupt enable
pub struct STOPIE_R(crate::FieldReader<bool, STOPIE_A>);
impl STOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPIE_A {
        match self.bits {
            false => STOPIE_A::B_0X0,
            true => STOPIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == STOPIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == STOPIE_A::B_0X1
    }
}
impl core::ops::Deref for STOPIE_R {
    type Target = crate::FieldReader<bool, STOPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `STOPIE` writer - Stop detection Interrupt enable
pub struct STOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: STOPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Stop detection (STOPF) interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STOPIE_A::B_0X0)
    }
    ///Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STOPIE_A::B_0X1)
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
///Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    ///0: Transfer Complete interrupt disabled
    B_0X0 = 0,
    ///1: Transfer Complete interrupt enabled
    B_0X1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)
pub struct TCIE_R(crate::FieldReader<bool, TCIE_A>);
impl TCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::B_0X0,
            true => TCIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TCIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TCIE_A::B_0X1
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, TCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TCIE` writer - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Transfer Complete interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCIE_A::B_0X0)
    }
    ///Transfer Complete interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCIE_A::B_0X1)
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
///Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    ///0: Error detection interrupts disabled
    B_0X0 = 0,
    ///1: Error detection interrupts enabled
    B_0X1 = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
pub struct ERRIE_R(crate::FieldReader<bool, ERRIE_A>);
impl ERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::B_0X0,
            true => ERRIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ERRIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ERRIE_A::B_0X1
    }
}
impl core::ops::Deref for ERRIE_R {
    type Target = crate::FieldReader<bool, ERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Error detection interrupts disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ERRIE_A::B_0X0)
    }
    ///Error detection interrupts enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ERRIE_A::B_0X1)
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
///Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\]
///* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DNF_A {
    ///0: Digital filter disabled
    B_0X0 = 0,
    ///1: Digital filter enabled and filtering capability up to 1 tI2CCLK
    B_0X1 = 1,
    ///15: digital filter enabled and filtering capability up to15 tI2CCLK
    B_0XF = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
///Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\]
///* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).
pub struct DNF_R(crate::FieldReader<u8, DNF_A>);
impl DNF_R {
    pub(crate) fn new(bits: u8) -> Self {
        DNF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DNF_A> {
        match self.bits {
            0 => Some(DNF_A::B_0X0),
            1 => Some(DNF_A::B_0X1),
            15 => Some(DNF_A::B_0XF),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DNF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DNF_A::B_0X1
    }
    ///Checks if the value of the field is `B_0XF`
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        **self == DNF_A::B_0XF
    }
}
impl core::ops::Deref for DNF_R {
    type Target = crate::FieldReader<u8, DNF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\]
///* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).
pub struct DNF_W<'a> {
    w: &'a mut W,
}
impl<'a> DNF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DNF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Digital filter disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DNF_A::B_0X0)
    }
    ///Digital filter enabled and filtering capability up to 1 tI2CCLK
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DNF_A::B_0X1)
    }
    ///digital filter enabled and filtering capability up to15 tI2CCLK
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(DNF_A::B_0XF)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANFOFF_A {
    ///0: Analog noise filter enabled
    B_0X0 = 0,
    ///1: Analog noise filter disabled
    B_0X1 = 1,
}
impl From<ANFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).
pub struct ANFOFF_R(crate::FieldReader<bool, ANFOFF_A>);
impl ANFOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANFOFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANFOFF_A {
        match self.bits {
            false => ANFOFF_A::B_0X0,
            true => ANFOFF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ANFOFF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ANFOFF_A::B_0X1
    }
}
impl core::ops::Deref for ANFOFF_R {
    type Target = crate::FieldReader<bool, ANFOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).
pub struct ANFOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFOFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ANFOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFOFF_A::B_0X0)
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFOFF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///DMA transmission requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    ///0: DMA mode disabled for transmission
    B_0X0 = 0,
    ///1: DMA mode enabled for transmission
    B_0X1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - DMA transmission requests enable
pub struct TXDMAEN_R(crate::FieldReader<bool, TXDMAEN_A>);
impl TXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::B_0X0,
            true => TXDMAEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXDMAEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXDMAEN_A::B_0X1
    }
}
impl core::ops::Deref for TXDMAEN_R {
    type Target = crate::FieldReader<bool, TXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXDMAEN` writer - DMA transmission requests enable
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA mode disabled for transmission
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXDMAEN_A::B_0X0)
    }
    ///DMA mode enabled for transmission
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///DMA reception requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    ///0: DMA mode disabled for reception
    B_0X0 = 0,
    ///1: DMA mode enabled for reception
    B_0X1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - DMA reception requests enable
pub struct RXDMAEN_R(crate::FieldReader<bool, RXDMAEN_A>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::B_0X0,
            true => RXDMAEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXDMAEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXDMAEN_A::B_0X1
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, RXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXDMAEN` writer - DMA reception requests enable
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA mode disabled for reception
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXDMAEN_A::B_0X0)
    }
    ///DMA mode enabled for reception
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Slave byte control This bit is used to enable hardware byte control in slave mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC_A {
    ///0: Slave byte control disabled
    B_0X0 = 0,
    ///1: Slave byte control enabled
    B_0X1 = 1,
}
impl From<SBC_A> for bool {
    #[inline(always)]
    fn from(variant: SBC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode.
pub struct SBC_R(crate::FieldReader<bool, SBC_A>);
impl SBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SBC_A {
        match self.bits {
            false => SBC_A::B_0X0,
            true => SBC_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SBC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SBC_A::B_0X1
    }
}
impl core::ops::Deref for SBC_R {
    type Target = crate::FieldReader<bool, SBC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode.
pub struct SBC_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SBC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Slave byte control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SBC_A::B_0X0)
    }
    ///Slave byte control enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SBC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTRETCH_A {
    ///0: Clock stretching enabled
    B_0X0 = 0,
    ///1: Clock stretching disabled
    B_0X1 = 1,
}
impl From<NOSTRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).
pub struct NOSTRETCH_R(crate::FieldReader<bool, NOSTRETCH_A>);
impl NOSTRETCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOSTRETCH_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NOSTRETCH_A {
        match self.bits {
            false => NOSTRETCH_A::B_0X0,
            true => NOSTRETCH_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == NOSTRETCH_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == NOSTRETCH_A::B_0X1
    }
}
impl core::ops::Deref for NOSTRETCH_R {
    type Target = crate::FieldReader<bool, NOSTRETCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).
pub struct NOSTRETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSTRETCH_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NOSTRETCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Clock stretching enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::B_0X0)
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to . Note: WUPEN can be set only when DNF = '0000â
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPEN_A {
    ///0: Wakeup from Stop mode disable.
    B_0X0 = 0,
    ///1: Wakeup from Stop mode enable.
    B_0X1 = 1,
}
impl From<WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPEN` reader - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to . Note: WUPEN can be set only when DNF = '0000â
pub struct WUPEN_R(crate::FieldReader<bool, WUPEN_A>);
impl WUPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUPEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUPEN_A {
        match self.bits {
            false => WUPEN_A::B_0X0,
            true => WUPEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUPEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUPEN_A::B_0X1
    }
}
impl core::ops::Deref for WUPEN_R {
    type Target = crate::FieldReader<bool, WUPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUPEN` writer - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to . Note: WUPEN can be set only when DNF = '0000â
pub struct WUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WUPEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Wakeup from Stop mode disable.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUPEN_A::B_0X0)
    }
    ///Wakeup from Stop mode enable.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///General call enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCEN_A {
    ///0: General call disabled. Address 0b00000000 is NACKed.
    B_0X0 = 0,
    ///1: General call enabled. Address 0b00000000 is ACKed.
    B_0X1 = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `GCEN` reader - General call enable
pub struct GCEN_R(crate::FieldReader<bool, GCEN_A>);
impl GCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::B_0X0,
            true => GCEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == GCEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == GCEN_A::B_0X1
    }
}
impl core::ops::Deref for GCEN_R {
    type Target = crate::FieldReader<bool, GCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `GCEN` writer - General call enable
pub struct GCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: GCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///General call disabled. Address 0b00000000 is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GCEN_A::B_0X0)
    }
    ///General call enabled. Address 0b00000000 is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GCEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBHEN_A {
    ///0: Host Address disabled. Address 0b0001000x is NACKed.
    B_0X0 = 0,
    ///1: Host Address enabled. Address 0b0001000x is ACKed.
    B_0X1 = 1,
}
impl From<SMBHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBHEN` reader - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct SMBHEN_R(crate::FieldReader<bool, SMBHEN_A>);
impl SMBHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBHEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBHEN_A {
        match self.bits {
            false => SMBHEN_A::B_0X0,
            true => SMBHEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SMBHEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SMBHEN_A::B_0X1
    }
}
impl core::ops::Deref for SMBHEN_R {
    type Target = crate::FieldReader<bool, SMBHEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SMBHEN` writer - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct SMBHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBHEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMBHEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Host Address disabled. Address 0b0001000x is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMBHEN_A::B_0X0)
    }
    ///Host Address enabled. Address 0b0001000x is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMBHEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBDEN_A {
    ///0: Device Default Address disabled. Address 0b1100001x is NACKed.
    B_0X0 = 0,
    ///1: Device Default Address enabled. Address 0b1100001x is ACKed.
    B_0X1 = 1,
}
impl From<SMBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBDEN` reader - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct SMBDEN_R(crate::FieldReader<bool, SMBDEN_A>);
impl SMBDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBDEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMBDEN_A {
        match self.bits {
            false => SMBDEN_A::B_0X0,
            true => SMBDEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SMBDEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SMBDEN_A::B_0X1
    }
}
impl core::ops::Deref for SMBDEN_R {
    type Target = crate::FieldReader<bool, SMBDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SMBDEN` writer - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct SMBDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SMBDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Device Default Address disabled. Address 0b1100001x is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SMBDEN_A::B_0X0)
    }
    ///Device Default Address enabled. Address 0b1100001x is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SMBDEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTEN_A {
    ///0: The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK).
    B_0X0 = 0,
    ///1: The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK).
    B_0X1 = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERTEN` reader - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct ALERTEN_R(crate::FieldReader<bool, ALERTEN_A>);
impl ALERTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALERTEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::B_0X0,
            true => ALERTEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ALERTEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ALERTEN_A::B_0X1
    }
}
impl core::ops::Deref for ALERTEN_R {
    type Target = crate::FieldReader<bool, ALERTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ALERTEN` writer - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct ALERTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ALERTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ALERTEN_A::B_0X0)
    }
    ///The SMBus alert pin is supported in host mode (SMBHEN=1). In device mode (SMBHEN=0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ALERTEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECEN_A {
    ///0: PEC calculation disabled
    B_0X0 = 0,
    ///1: PEC calculation enabled
    B_0X1 = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PECEN` reader - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct PECEN_R(crate::FieldReader<bool, PECEN_A>);
impl PECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::B_0X0,
            true => PECEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PECEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PECEN_A::B_0X1
    }
}
impl core::ops::Deref for PECEN_R {
    type Target = crate::FieldReader<bool, PECEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PECEN` writer - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PECEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///PEC calculation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PECEN_A::B_0X0)
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PECEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    ///Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TX Interrupt enable
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - RX Interrupt enable
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Address match Interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Not acknowledge received Interrupt enable
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Stop detection Interrupt enable
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\]
    ///* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode.
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to . Note: WUPEN can be set only when DNF = '0000â
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    ///Bit 1 - TX Interrupt enable
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W {
        TXIE_W { w: self }
    }
    ///Bit 2 - RX Interrupt enable
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    ///Bit 3 - Address match Interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&mut self) -> ADDRIE_W {
        ADDRIE_W { w: self }
    }
    ///Bit 4 - Not acknowledge received Interrupt enable
    #[inline(always)]
    pub fn nackie(&mut self) -> NACKIE_W {
        NACKIE_W { w: self }
    }
    ///Bit 5 - Stop detection Interrupt enable
    #[inline(always)]
    pub fn stopie(&mut self) -> STOPIE_W {
        STOPIE_W { w: self }
    }
    ///Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    ///Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    ///Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\]
    ///* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W {
        DNF_W { w: self }
    }
    ///Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn anfoff(&mut self) -> ANFOFF_W {
        ANFOFF_W { w: self }
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    ///Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode.
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W {
        SBC_W { w: self }
    }
    ///Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W {
        NOSTRETCH_W { w: self }
    }
    ///Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to . Note: WUPEN can be set only when DNF = '0000â
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W {
        WUPEN_W { w: self }
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W { w: self }
    }
    ///Bit 20 - SMBus Host Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn smbhen(&mut self) -> SMBHEN_W {
        SMBHEN_W { w: self }
    }
    ///Bit 21 - SMBus Device Default Address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn smbden(&mut self) -> SMBDEN_W {
        SMBDEN_W { w: self }
    }
    ///Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W {
        ALERTEN_W { w: self }
    }
    ///Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to '0â. Refer to .
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2c_cr1](index.html) module
pub struct I2C_CR1_SPEC;
impl crate::RegisterSpec for I2C_CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2c_cr1::R](R) reader structure
impl crate::Readable for I2C_CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2c_cr1::W](W) writer structure
impl crate::Writable for I2C_CR1_SPEC {
    type Writer = W;
}
///`reset()` method sets I2C_CR1 to value 0
impl crate::Resettable for I2C_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
