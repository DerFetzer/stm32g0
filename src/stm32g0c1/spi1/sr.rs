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
///Receive buffer not empty
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    ///0: Rx buffer empty
    B_0X0 = 0,
    ///1: Rx buffer not empty
    B_0X1 = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Receive buffer not empty
pub struct RXNE_R(crate::FieldReader<bool, RXNE_A>);
impl RXNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::B_0X0,
            true => RXNE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXNE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXNE_A::B_0X1
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, RXNE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Transmit buffer empty
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    ///0: Tx buffer not empty
    B_0X0 = 0,
    ///1: Tx buffer empty
    B_0X1 = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit buffer empty
pub struct TXE_R(crate::FieldReader<bool, TXE_A>);
impl TXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::B_0X0,
            true => TXE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXE_A::B_0X1
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, TXE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHSIDE_A {
    ///0: Channel Left has to be transmitted or has been received
    B_0X0 = 0,
    ///1: Channel Right has to be transmitted or has been received
    B_0X1 = 1,
}
impl From<CHSIDE_A> for bool {
    #[inline(always)]
    fn from(variant: CHSIDE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHSIDE` reader - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
pub struct CHSIDE_R(crate::FieldReader<bool, CHSIDE_A>);
impl CHSIDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSIDE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHSIDE_A {
        match self.bits {
            false => CHSIDE_A::B_0X0,
            true => CHSIDE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CHSIDE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CHSIDE_A::B_0X1
    }
}
impl core::ops::Deref for CHSIDE_R {
    type Target = crate::FieldReader<bool, CHSIDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Underrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence. Note: This bit is not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDR_A {
    ///0: No underrun occurred
    B_0X0 = 0,
    ///1: Underrun occurred
    B_0X1 = 1,
}
impl From<UDR_A> for bool {
    #[inline(always)]
    fn from(variant: UDR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence. Note: This bit is not used in SPI mode.
pub struct UDR_R(crate::FieldReader<bool, UDR_A>);
impl UDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UDR_A {
        match self.bits {
            false => UDR_A::B_0X0,
            true => UDR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == UDR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == UDR_A::B_0X1
    }
}
impl core::ops::Deref for UDR_R {
    type Target = crate::FieldReader<bool, UDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    ///0: CRC value received matches the SPI_RXCRCR value
    B_0X0 = 0,
    ///1: CRC value received does not match the SPI_RXCRCR value
    B_0X1 = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCERR` reader - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
pub struct CRCERR_R(crate::FieldReader<bool, CRCERR_A>);
impl CRCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCERR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::B_0X0,
            true => CRCERR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CRCERR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CRCERR_A::B_0X1
    }
}
impl core::ops::Deref for CRCERR_R {
    type Target = crate::FieldReader<bool, CRCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRCERR` writer - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CRCERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CRC value received matches the SPI_RXCRCR value
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRCERR_A::B_0X0)
    }
    ///CRC value received does not match the SPI_RXCRCR value
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRCERR_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
///Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    ///0: No mode fault occurred
    B_0X0 = 0,
    ///1: Mode fault occurred
    B_0X1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MODF` reader - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
pub struct MODF_R(crate::FieldReader<bool, MODF_A>);
impl MODF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::B_0X0,
            true => MODF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MODF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MODF_A::B_0X1
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, MODF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    ///0: No overrun occurred
    B_0X0 = 0,
    ///1: Overrun occurred
    B_0X1 = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
pub struct OVR_R(crate::FieldReader<bool, OVR_A>);
impl OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::B_0X0,
            true => OVR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == OVR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == OVR_A::B_0X1
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, OVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSY_A {
    ///0: SPI (or I2S) not busy
    B_0X0 = 0,
    ///1: SPI (or I2S) is busy in communication or Tx buffer is not empty
    B_0X1 = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BSY` reader - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
pub struct BSY_R(crate::FieldReader<bool, BSY_A>);
impl BSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSY_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::B_0X0,
            true => BSY_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BSY_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BSY_A::B_0X1
    }
}
impl core::ops::Deref for BSY_R {
    type Target = crate::FieldReader<bool, BSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRE_A {
    ///0: No frame format error
    B_0X0 = 0,
    ///1: A frame format error occurred
    B_0X1 = 1,
}
impl From<FRE_A> for bool {
    #[inline(always)]
    fn from(variant: FRE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FRE` reader - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
pub struct FRE_R(crate::FieldReader<bool, FRE_A>);
impl FRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRE_A {
        match self.bits {
            false => FRE_A::B_0X0,
            true => FRE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FRE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FRE_A::B_0X1
    }
}
impl core::ops::Deref for FRE_R {
    type Target = crate::FieldReader<bool, FRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRLVL_A {
    ///0: FIFO empty
    B_0X0 = 0,
    ///1: 1/4 FIFO
    B_0X1 = 1,
    ///2: 1/2 FIFO
    B_0X2 = 2,
    ///3: FIFO full
    B_0X3 = 3,
}
impl From<FRLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FRLVL_A) -> Self {
        variant as _
    }
}
///Field `FRLVL` reader - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
pub struct FRLVL_R(crate::FieldReader<u8, FRLVL_A>);
impl FRLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRLVL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FRLVL_A {
        match self.bits {
            0 => FRLVL_A::B_0X0,
            1 => FRLVL_A::B_0X1,
            2 => FRLVL_A::B_0X2,
            3 => FRLVL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FRLVL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FRLVL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == FRLVL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == FRLVL_A::B_0X3
    }
}
impl core::ops::Deref for FRLVL_R {
    type Target = crate::FieldReader<u8, FRLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTLVL_A {
    ///0: FIFO empty
    B_0X0 = 0,
    ///1: 1/4 FIFO
    B_0X1 = 1,
    ///2: 1/2 FIFO
    B_0X2 = 2,
    ///3: FIFO full (considered as FULL when the FIFO threshold is greater than 1/2)
    B_0X3 = 3,
}
impl From<FTLVL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTLVL_A) -> Self {
        variant as _
    }
}
///Field `FTLVL` reader - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
pub struct FTLVL_R(crate::FieldReader<u8, FTLVL_A>);
impl FTLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTLVL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FTLVL_A {
        match self.bits {
            0 => FTLVL_A::B_0X0,
            1 => FTLVL_A::B_0X1,
            2 => FTLVL_A::B_0X2,
            3 => FTLVL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FTLVL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FTLVL_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == FTLVL_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == FTLVL_A::B_0X3
    }
}
impl core::ops::Deref for FTLVL_R {
    type Target = crate::FieldReader<u8, FTLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Receive buffer not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Transmit buffer empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Channel side Note: This bit is not used in SPI mode. It has no significance in PCM mode.
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Underrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence. Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Mode fault This flag is set by hardware and reset by a software sequence. Refer to (MODF) on pageÂ 1031 for the software sequence. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Overrun flag This flag is set by hardware and reset by a software sequence. Refer to pageÂ 1057 for the software sequence.
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Busy flag This flag is set and cleared by hardware. Note: The BSY flag must be used with caution: refer to and .
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Frame format error This flag is used for SPI in TI slave mode and I2S slave mode. Refer to error flags and . This flag is set by hardware and reset when SPI_SR is read by software.
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 9:10 - FIFO reception level These bits are set and cleared by hardware. Note: These bits are not used in IÂ²S mode and in SPI receive-only mode while CRC calculation is enabled.
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    ///Bits 11:12 - FIFO transmission level These bits are set and cleared by hardware. Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
}
impl W {
    ///Bit 4 - CRC error flag Note: This flag is set by hardware and cleared by software writing 0. This bit is not used in I2S mode.
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPI status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u16;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
///`reset()` method sets SR to value 0x02
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
