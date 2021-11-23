///Register `I2SCFGR` reader
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2SCFGR` writer
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHLEN_A {
    ///0: 16-bit wide
    B_0X0 = 0,
    ///1: 32-bit wide
    B_0X1 = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CHLEN` reader - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.
pub struct CHLEN_R(crate::FieldReader<bool, CHLEN_A>);
impl CHLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHLEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::B_0X0,
            true => CHLEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CHLEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CHLEN_A::B_0X1
    }
}
impl core::ops::Deref for CHLEN_R {
    type Target = crate::FieldReader<bool, CHLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CHLEN` writer - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.
pub struct CHLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHLEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CHLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///16-bit wide
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CHLEN_A::B_0X0)
    }
    ///32-bit wide
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CHLEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
///Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATLEN_A {
    ///0: 16-bit data length
    B_0X0 = 0,
    ///1: 24-bit data length
    B_0X1 = 1,
    ///2: 32-bit data length
    B_0X2 = 2,
    ///3: Not allowed
    B_0X3 = 3,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
///Field `DATLEN` reader - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
pub struct DATLEN_R(crate::FieldReader<u8, DATLEN_A>);
impl DATLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATLEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DATLEN_A {
        match self.bits {
            0 => DATLEN_A::B_0X0,
            1 => DATLEN_A::B_0X1,
            2 => DATLEN_A::B_0X2,
            3 => DATLEN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DATLEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DATLEN_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == DATLEN_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == DATLEN_A::B_0X3
    }
}
impl core::ops::Deref for DATLEN_R {
    type Target = crate::FieldReader<u8, DATLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DATLEN` writer - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
pub struct DATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATLEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DATLEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///16-bit data length
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DATLEN_A::B_0X0)
    }
    ///24-bit data length
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DATLEN_A::B_0X1)
    }
    ///32-bit data length
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DATLEN_A::B_0X2)
    }
    ///Not allowed
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DATLEN_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u16 & 0x03) << 1);
        self.w
    }
}
///Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKPOL_A {
    ///0: I2S clock inactive state is low level
    B_0X0 = 0,
    ///1: I2S clock inactive state is high level
    B_0X1 = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CKPOL` reader - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
pub struct CKPOL_R(crate::FieldReader<bool, CKPOL_A>);
impl CKPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::B_0X0,
            true => CKPOL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CKPOL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CKPOL_A::B_0X1
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<bool, CKPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CKPOL` writer - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CKPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///I2S clock inactive state is low level
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKPOL_A::B_0X0)
    }
    ///I2S clock inactive state is high level
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKPOL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
///I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SSTD_A {
    ///0: I2S Philips standard
    B_0X0 = 0,
    ///1: MSB justified standard (left justified)
    B_0X1 = 1,
    ///2: LSB justified standard (right justified)
    B_0X2 = 2,
    ///3: PCM standard
    B_0X3 = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
///Field `I2SSTD` reader - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
pub struct I2SSTD_R(crate::FieldReader<u8, I2SSTD_A>);
impl I2SSTD_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SSTD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::B_0X0,
            1 => I2SSTD_A::B_0X1,
            2 => I2SSTD_A::B_0X2,
            3 => I2SSTD_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == I2SSTD_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == I2SSTD_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == I2SSTD_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == I2SSTD_A::B_0X3
    }
}
impl core::ops::Deref for I2SSTD_R {
    type Target = crate::FieldReader<u8, I2SSTD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SSTD` writer - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
pub struct I2SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSTD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SSTD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///I2S Philips standard
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2SSTD_A::B_0X0)
    }
    ///MSB justified standard (left justified)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2SSTD_A::B_0X1)
    }
    ///LSB justified standard (right justified)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2SSTD_A::B_0X2)
    }
    ///PCM standard
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(I2SSTD_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u16 & 0x03) << 4);
        self.w
    }
}
///PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCMSYNC_A {
    ///0: Short frame synchronization
    B_0X0 = 0,
    ///1: Long frame synchronization
    B_0X1 = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PCMSYNC` reader - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
pub struct PCMSYNC_R(crate::FieldReader<bool, PCMSYNC_A>);
impl PCMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCMSYNC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::B_0X0,
            true => PCMSYNC_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PCMSYNC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PCMSYNC_A::B_0X1
    }
}
impl core::ops::Deref for PCMSYNC_R {
    type Target = crate::FieldReader<bool, PCMSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PCMSYNC` writer - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
pub struct PCMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMSYNC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PCMSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Short frame synchronization
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PCMSYNC_A::B_0X0)
    }
    ///Long frame synchronization
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PCMSYNC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
///I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2SCFG_A {
    ///0: Slave - transmit
    B_0X0 = 0,
    ///1: Slave - receive
    B_0X1 = 1,
    ///2: Master - transmit
    B_0X2 = 2,
    ///3: Master - receive
    B_0X3 = 3,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
///Field `I2SCFG` reader - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.
pub struct I2SCFG_R(crate::FieldReader<u8, I2SCFG_A>);
impl I2SCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SCFG_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SCFG_A {
        match self.bits {
            0 => I2SCFG_A::B_0X0,
            1 => I2SCFG_A::B_0X1,
            2 => I2SCFG_A::B_0X2,
            3 => I2SCFG_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == I2SCFG_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == I2SCFG_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == I2SCFG_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == I2SCFG_A::B_0X3
    }
}
impl core::ops::Deref for I2SCFG_R {
    type Target = crate::FieldReader<u8, I2SCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SCFG` writer - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.
pub struct I2SCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SCFG_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Slave - transmit
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2SCFG_A::B_0X0)
    }
    ///Slave - receive
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2SCFG_A::B_0X1)
    }
    ///Master - transmit
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2SCFG_A::B_0X2)
    }
    ///Master - receive
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(I2SCFG_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
///I2S enable Note: This bit is not used in SPI mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SE_A {
    ///0: I2S peripheral is disabled
    B_0X0 = 0,
    ///1: I2S peripheral is enabled
    B_0X1 = 1,
}
impl From<I2SE_A> for bool {
    #[inline(always)]
    fn from(variant: I2SE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SE` reader - I2S enable Note: This bit is not used in SPI mode.
pub struct I2SE_R(crate::FieldReader<bool, I2SE_A>);
impl I2SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SE_A {
        match self.bits {
            false => I2SE_A::B_0X0,
            true => I2SE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == I2SE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == I2SE_A::B_0X1
    }
}
impl core::ops::Deref for I2SE_R {
    type Target = crate::FieldReader<bool, I2SE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SE` writer - I2S enable Note: This bit is not used in SPI mode.
pub struct I2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///I2S peripheral is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2SE_A::B_0X0)
    }
    ///I2S peripheral is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2SE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
///I2S mode selection Note: This bit should be configured when the SPI is disabled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SMOD_A {
    ///0: SPI mode is selected
    B_0X0 = 0,
    ///1: I2S mode is selected
    B_0X1 = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2SMOD` reader - I2S mode selection Note: This bit should be configured when the SPI is disabled.
pub struct I2SMOD_R(crate::FieldReader<bool, I2SMOD_A>);
impl I2SMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2SMOD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::B_0X0,
            true => I2SMOD_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == I2SMOD_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == I2SMOD_A::B_0X1
    }
}
impl core::ops::Deref for I2SMOD_R {
    type Target = crate::FieldReader<bool, I2SMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2SMOD` writer - I2S mode selection Note: This bit should be configured when the SPI is disabled.
pub struct I2SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SMOD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2SMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SPI mode is selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2SMOD_A::B_0X0)
    }
    ///I2S mode is selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2SMOD_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
///Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASTRTEN_A {
    ///0: The Asynchronous start is disabled.
    B_0X0 = 0,
    ///1: The Asynchronous start is enabled.
    B_0X1 = 1,
}
impl From<ASTRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASTRTEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASTRTEN` reader - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.
pub struct ASTRTEN_R(crate::FieldReader<bool, ASTRTEN_A>);
impl ASTRTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASTRTEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ASTRTEN_A {
        match self.bits {
            false => ASTRTEN_A::B_0X0,
            true => ASTRTEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ASTRTEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ASTRTEN_A::B_0X1
    }
}
impl core::ops::Deref for ASTRTEN_R {
    type Target = crate::FieldReader<bool, ASTRTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ASTRTEN` writer - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.
pub struct ASTRTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTRTEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ASTRTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The Asynchronous start is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ASTRTEN_A::B_0X0)
    }
    ///The Asynchronous start is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ASTRTEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
impl R {
    ///Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bit 10 - I2S enable Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled.
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.
    #[inline(always)]
    pub fn astrten(&self) -> ASTRTEN_R {
        ASTRTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Channel length (number of bits per audio channel) The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode.
    #[inline(always)]
    pub fn chlen(&mut self) -> CHLEN_W {
        CHLEN_W { w: self }
    }
    ///Bits 1:2 - Data length to be transferred Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
    #[inline(always)]
    pub fn datlen(&mut self) -> DATLEN_W {
        DATLEN_W { w: self }
    }
    ///Bit 3 - Inactive state clock polarity Note: For correct operation, this bit should be configured when the I2S is disabled. It is not used in SPI mode. The bit CKPOL does not affect the CK edge sensitivity used to receive or transmit the SD and WS signals.
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    ///Bits 4:5 - I2S standard selection For more details on I2S standards, refer to Note: For correct operation, these bits should be configured when the I2S is disabled. They are not used in SPI mode.
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2SSTD_W {
        I2SSTD_W { w: self }
    }
    ///Bit 7 - PCM frame synchronization Note: This bit has a meaning only if I2SSTD = 11 (PCM standard is used). It is not used in SPI mode.
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PCMSYNC_W {
        PCMSYNC_W { w: self }
    }
    ///Bits 8:9 - I2S configuration mode Note: These bits should be configured when the I2S is disabled. They are not used in SPI mode.
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2SCFG_W {
        I2SCFG_W { w: self }
    }
    ///Bit 10 - I2S enable Note: This bit is not used in SPI mode.
    #[inline(always)]
    pub fn i2se(&mut self) -> I2SE_W {
        I2SE_W { w: self }
    }
    ///Bit 11 - I2S mode selection Note: This bit should be configured when the SPI is disabled.
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2SMOD_W {
        I2SMOD_W { w: self }
    }
    ///Bit 12 - Asynchronous start enable. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and an appropriate transition is detected on the WS signal. When the I2S is enabled in slave mode, the hardware starts the transfer when the I2S clock is received and the appropriate level is detected on the WS signal. Note: The appropriate transition is a falling edge on WS signal when I2S Philips Standard is used, or a rising edge for other standards. The appropriate level is a low level on WS signal when I2S Philips Standard is used, or a high level for other standards. Please refer to for additional information.
    #[inline(always)]
    pub fn astrten(&mut self) -> ASTRTEN_W {
        ASTRTEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPI_I2S configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i2scfgr](index.html) module
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u16;
}
///`read()` method returns [i2scfgr::R](R) reader structure
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2scfgr::W](W) writer structure
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
