///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    ///0: Interrupt is inhibited
    B_0X0 = 0,
    ///1: An interrupt is generated when FE = 1 or ORE = 1 or NE = 1 in the LPUART_ISR       register.
    B_0X1 = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register).
pub struct EIE_R(crate::FieldReader<bool, EIE_A>);
impl EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::B_0X0,
            true => EIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EIE_A::B_0X1
    }
}
impl core::ops::Deref for EIE_R {
    type Target = crate::FieldReader<bool, EIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register).
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EIE_A::B_0X0)
    }
    ///An interrupt is generated when FE = 1 or ORE = 1 or NE = 1 in the LPUART_ISR register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EIE_A::B_0X1)
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
///Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSEL_A {
    ///0: Half duplex mode is not selected
    B_0X0 = 0,
    ///1: Half duplex mode is selected
    B_0X1 = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct HDSEL_R(crate::FieldReader<bool, HDSEL_A>);
impl HDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSEL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::B_0X0,
            true => HDSEL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HDSEL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HDSEL_A::B_0X1
    }
}
impl core::ops::Deref for HDSEL_R {
    type Target = crate::FieldReader<bool, HDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct HDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDSEL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HDSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HDSEL_A::B_0X0)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HDSEL_A::B_0X1)
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
///DMA enable receiver This bit is set/reset by software
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR_A {
    ///1: DMA mode is enabled for reception
    B_0X1 = 1,
    ///0: DMA mode is disabled for reception
    B_0X0 = 0,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAR` reader - DMA enable receiver This bit is set/reset by software
pub struct DMAR_R(crate::FieldReader<bool, DMAR_A>);
impl DMAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            true => DMAR_A::B_0X1,
            false => DMAR_A::B_0X0,
        }
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMAR_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMAR_A::B_0X0
    }
}
impl core::ops::Deref for DMAR_R {
    type Target = crate::FieldReader<bool, DMAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAR` writer - DMA enable receiver This bit is set/reset by software
pub struct DMAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAR_A::B_0X1)
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAR_A::B_0X0)
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
///DMA enable transmitter This bit is set/reset by software
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAT_A {
    ///1: DMA mode is enabled for transmission
    B_0X1 = 1,
    ///0: DMA mode is disabled for transmission
    B_0X0 = 0,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software
pub struct DMAT_R(crate::FieldReader<bool, DMAT_A>);
impl DMAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            true => DMAT_A::B_0X1,
            false => DMAT_A::B_0X0,
        }
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DMAT_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DMAT_A::B_0X0
    }
}
impl core::ops::Deref for DMAT_R {
    type Target = crate::FieldReader<bool, DMAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software
pub struct DMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DMAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAT_A::B_0X1)
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAT_A::B_0X0)
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
///RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSE_A {
    ///0: RTS hardware flow control disabled
    B_0X0 = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received.
    B_0X1 = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSE` reader - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct RTSE_R(crate::FieldReader<bool, RTSE_A>);
impl RTSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::B_0X0,
            true => RTSE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RTSE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RTSE_A::B_0X1
    }
}
impl core::ops::Deref for RTSE_R {
    type Target = crate::FieldReader<bool, RTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTSE` writer - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct RTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTSE_A::B_0X0)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTSE_A::B_0X1)
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
///CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSE_A {
    ///0: CTS hardware flow control disabled
    B_0X0 = 0,
    ///1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted.
    B_0X1 = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)
pub struct CTSE_R(crate::FieldReader<bool, CTSE_A>);
impl CTSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::B_0X0,
            true => CTSE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CTSE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CTSE_A::B_0X1
    }
}
impl core::ops::Deref for CTSE_R {
    type Target = crate::FieldReader<bool, CTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTSE` writer - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)
pub struct CTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CTSE_A::B_0X0)
    }
    ///CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CTSE_A::B_0X1)
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
///CTS interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    ///0: Interrupt is inhibited
    B_0X0 = 0,
    ///1: An interrupt is generated whenever CTSIF = 1 in the LPUART_ISR register  
    B_0X1 = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIE` reader - CTS interrupt enable
pub struct CTSIE_R(crate::FieldReader<bool, CTSIE_A>);
impl CTSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::B_0X0,
            true => CTSIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CTSIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CTSIE_A::B_0X1
    }
}
impl core::ops::Deref for CTSIE_R {
    type Target = crate::FieldReader<bool, CTSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CTSIE` writer - CTS interrupt enable
pub struct CTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CTSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CTSIE_A::B_0X0)
    }
    ///An interrupt is generated whenever CTSIF = 1 in the LPUART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CTSIE_A::B_0X1)
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
///Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRDIS_A {
    ///0: Overrun Error Flag, ORE is set when received data is not read before receiving new data.
    B_0X0 = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
    B_0X1 = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data.
pub struct OVRDIS_R(crate::FieldReader<bool, OVRDIS_A>);
impl OVRDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRDIS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::B_0X0,
            true => OVRDIS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OVRDIS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OVRDIS_A::B_0X1
    }
}
impl core::ops::Deref for OVRDIS_R {
    type Target = crate::FieldReader<bool, OVRDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data.
pub struct OVRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRDIS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OVRDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Overrun Error Flag, ORE is set when received data is not read before receiving new data.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OVRDIS_A::B_0X0)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OVRDIS_A::B_0X1)
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
///DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRE_A {
    ///0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred.
    B_0X0 = 0,
    ///1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE before clearing the error flag.
    B_0X1 = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error.
pub struct DDRE_R(crate::FieldReader<bool, DDRE_A>);
impl DDRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::B_0X0,
            true => DDRE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DDRE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DDRE_A::B_0X1
    }
}
impl core::ops::Deref for DDRE_R {
    type Target = crate::FieldReader<bool, DDRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error.
pub struct DDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DDRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRE_A::B_0X0)
    }
    ///DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE before clearing the error flag.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRE_A::B_0X1)
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
///Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEM_A {
    ///0: DE function is disabled.
    B_0X0 = 0,
    ///1: DE function is enabled. The DE signal is output on the RTS pin.
    B_0X1 = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct DEM_R(crate::FieldReader<bool, DEM_A>);
impl DEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::B_0X0,
            true => DEM_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DEM_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DEM_A::B_0X1
    }
}
impl core::ops::Deref for DEM_R {
    type Target = crate::FieldReader<bool, DEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct DEM_W<'a> {
    w: &'a mut W,
}
impl<'a> DEM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DE function is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DEM_A::B_0X0)
    }
    ///DE function is enabled. The DE signal is output on the RTS pin.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DEM_A::B_0X1)
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
///Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEP_A {
    ///0: DE signal is active high.
    B_0X0 = 0,
    ///1: DE signal is active low.
    B_0X1 = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DEP` reader - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct DEP_R(crate::FieldReader<bool, DEP_A>);
impl DEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::B_0X0,
            true => DEP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DEP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DEP_A::B_0X1
    }
}
impl core::ops::Deref for DEP_R {
    type Target = crate::FieldReader<bool, DEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DEP` writer - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
pub struct DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DEP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///DE signal is active high.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DEP_A::B_0X0)
    }
    ///DE signal is active low.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DEP_A::B_0X1)
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
///Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUS_A {
    ///0: WUF active on address match (as defined by ADD\[7:0\]
    ///and ADDM7)
    B_0X0 = 0,
    ///2: WUF active on Start bit detection
    B_0X2 = 2,
    ///3: WUF active on RXNE.
    B_0X3 = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
///Field `WUS` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
pub struct WUS_R(crate::FieldReader<u8, WUS_A>);
impl WUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUS_A> {
        match self.bits {
            0 => Some(WUS_A::B_0X0),
            2 => Some(WUS_A::B_0X2),
            3 => Some(WUS_A::B_0X3),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == WUS_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == WUS_A::B_0X3
    }
}
impl core::ops::Deref for WUS_R {
    type Target = crate::FieldReader<u8, WUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUS` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
pub struct WUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WUS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///WUF active on address match (as defined by ADD\[7:0\]
    ///and ADDM7)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUS_A::B_0X0)
    }
    ///WUF active on Start bit detection
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(WUS_A::B_0X2)
    }
    ///WUF active on RXNE.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(WUS_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFIE_A {
    ///0: Interrupt is inhibited
    B_0X0 = 0,
    ///1: An LPUART interrupt is generated whenever WUF = 1 in the LPUART_ISR register  
    B_0X1 = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFIE` reader - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
pub struct WUFIE_R(crate::FieldReader<bool, WUFIE_A>);
impl WUFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUFIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::B_0X0,
            true => WUFIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == WUFIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == WUFIE_A::B_0X1
    }
}
impl core::ops::Deref for WUFIE_R {
    type Target = crate::FieldReader<bool, WUFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WUFIE` writer - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
pub struct WUFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUFIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: WUFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WUFIE_A::B_0X0)
    }
    ///An LPUART interrupt is generated whenever WUF = 1 in the LPUART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WUFIE_A::B_0X1)
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
///TXFIFO threshold interrupt enable This bit is set and cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFTIE_A {
    ///0: Interrupt is inhibited
    B_0X0 = 0,
    ///1: A LPUART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    B_0X1 = 1,
}
impl From<TXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software.
pub struct TXFTIE_R(crate::FieldReader<bool, TXFTIE_A>);
impl TXFTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXFTIE_A {
        match self.bits {
            false => TXFTIE_A::B_0X0,
            true => TXFTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXFTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXFTIE_A::B_0X1
    }
}
impl core::ops::Deref for TXFTIE_R {
    type Target = crate::FieldReader<bool, TXFTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software.
pub struct TXFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXFTIE_A::B_0X0)
    }
    ///A LPUART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXFTIE_A::B_0X1)
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
///Receive FIFO threshold configuration Remaining combinations: Reserved.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFTCFG_A {
    ///0: Receive FIFO reaches 1/8 of its depth.
    B_0X0 = 0,
    ///1: Receive FIFO reaches 1/4 of its depth.
    B_0X1 = 1,
    ///6: Receive FIFO reaches 1/2 of its depth.
    B_0X6 = 6,
    ///3: Receive FIFO reaches 3/4 of its depth.
    B_0X3 = 3,
    ///4: Receive FIFO reaches 7/8 of its depth.
    B_0X4 = 4,
    ///5: Receive FIFO becomes full.
    B_0X5 = 5,
}
impl From<RXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG_A) -> Self {
        variant as _
    }
}
///Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved.
pub struct RXFTCFG_R(crate::FieldReader<u8, RXFTCFG_A>);
impl RXFTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFTCFG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFTCFG_A> {
        match self.bits {
            0 => Some(RXFTCFG_A::B_0X0),
            1 => Some(RXFTCFG_A::B_0X1),
            6 => Some(RXFTCFG_A::B_0X6),
            3 => Some(RXFTCFG_A::B_0X3),
            4 => Some(RXFTCFG_A::B_0X4),
            5 => Some(RXFTCFG_A::B_0X5),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXFTCFG_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXFTCFG_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == RXFTCFG_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == RXFTCFG_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == RXFTCFG_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == RXFTCFG_A::B_0X5
    }
}
impl core::ops::Deref for RXFTCFG_R {
    type Target = crate::FieldReader<u8, RXFTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved.
pub struct RXFTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Receive FIFO reaches 1/8 of its depth.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X0)
    }
    ///Receive FIFO reaches 1/4 of its depth.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X1)
    }
    ///Receive FIFO reaches 1/2 of its depth.
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X6)
    }
    ///Receive FIFO reaches 3/4 of its depth.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X3)
    }
    ///Receive FIFO reaches 7/8 of its depth.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X4)
    }
    ///Receive FIFO becomes full.
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(RXFTCFG_A::B_0X5)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
///RXFIFO threshold interrupt enable This bit is set and cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFTIE_A {
    ///0: Interrupt is inhibited
    B_0X0 = 0,
    ///1: An LPUART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    B_0X1 = 1,
}
impl From<RXFTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software.
pub struct RXFTIE_R(crate::FieldReader<bool, RXFTIE_A>);
impl RXFTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFTIE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXFTIE_A {
        match self.bits {
            false => RXFTIE_A::B_0X0,
            true => RXFTIE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXFTIE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXFTIE_A::B_0X1
    }
}
impl core::ops::Deref for RXFTIE_R {
    type Target = crate::FieldReader<bool, RXFTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software.
pub struct RXFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTIE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXFTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXFTIE_A::B_0X0)
    }
    ///An LPUART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXFTIE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///TXFIFO threshold configuration Remaining combinations: Reserved.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFTCFG_A {
    ///0: TXFIFO reaches 1/8 of its depth.
    B_0X0 = 0,
    ///1: TXFIFO reaches 1/4 of its depth.
    B_0X1 = 1,
    ///6: TXFIFO reaches 1/2 of its depth.
    B_0X6 = 6,
    ///3: TXFIFO reaches 3/4 of its depth.
    B_0X3 = 3,
    ///4: TXFIFO reaches 7/8 of its depth.
    B_0X4 = 4,
    ///5: TXFIFO becomes empty.
    B_0X5 = 5,
}
impl From<TXFTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG_A) -> Self {
        variant as _
    }
}
///Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved.
pub struct TXFTCFG_R(crate::FieldReader<u8, TXFTCFG_A>);
impl TXFTCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFTCFG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFTCFG_A> {
        match self.bits {
            0 => Some(TXFTCFG_A::B_0X0),
            1 => Some(TXFTCFG_A::B_0X1),
            6 => Some(TXFTCFG_A::B_0X6),
            3 => Some(TXFTCFG_A::B_0X3),
            4 => Some(TXFTCFG_A::B_0X4),
            5 => Some(TXFTCFG_A::B_0X5),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXFTCFG_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXFTCFG_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == TXFTCFG_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TXFTCFG_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == TXFTCFG_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == TXFTCFG_A::B_0X5
    }
}
impl core::ops::Deref for TXFTCFG_R {
    type Target = crate::FieldReader<u8, TXFTCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved.
pub struct TXFTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXFTCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///TXFIFO reaches 1/8 of its depth.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X0)
    }
    ///TXFIFO reaches 1/4 of its depth.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X1)
    }
    ///TXFIFO reaches 1/2 of its depth.
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X6)
    }
    ///TXFIFO reaches 3/4 of its depth.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X3)
    }
    ///TXFIFO reaches 7/8 of its depth.
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X4)
    }
    ///TXFIFO becomes empty.
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(TXFTCFG_A::B_0X5)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
impl R {
    ///Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register).
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 6 - DMA enable receiver This bit is set/reset by software
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - DMA enable transmitter This bit is set/reset by software
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data.
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error.
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved.
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved.
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    ///Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register).
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    ///Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W {
        HDSEL_W { w: self }
    }
    ///Bit 6 - DMA enable receiver This bit is set/reset by software
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W {
        DMAR_W { w: self }
    }
    ///Bit 7 - DMA enable transmitter This bit is set/reset by software
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W {
        DMAT_W { w: self }
    }
    ///Bit 8 - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W {
        RTSE_W { w: self }
    }
    ///Bit 9 - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W {
        CTSE_W { w: self }
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W { w: self }
    }
    ///Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data.
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W {
        OVRDIS_W { w: self }
    }
    ///Bit 13 - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error.
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W {
        DDRE_W { w: self }
    }
    ///Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W {
        DEM_W { w: self }
    }
    ///Bit 15 - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W { w: self }
    }
    ///Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W {
        WUS_W { w: self }
    }
    ///Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W {
        WUFIE_W { w: self }
    }
    ///Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W {
        TXFTIE_W { w: self }
    }
    ///Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved.
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W {
        RXFTCFG_W { w: self }
    }
    ///Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W {
        RXFTIE_W { w: self }
    }
    ///Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved.
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W {
        TXFTCFG_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPUART control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
