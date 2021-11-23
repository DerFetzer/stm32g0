///Register `TXEFS` reader
pub struct R(crate::R<TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EFFL` reader - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3.
pub struct EFFL_R(crate::FieldReader<u8, u8>);
impl EFFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFGI` reader - Event FIFO get index Tx Event FIFO read index pointer, range 0 to 3.
pub struct EFGI_R(crate::FieldReader<u8, u8>);
impl EFGI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFPI` reader - Event FIFO put index Tx Event FIFO write index pointer, range 0 to 3.
pub struct EFPI_R(crate::FieldReader<u8, u8>);
impl EFPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Event FIFO full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFF_A {
    ///0: Tx event FIFO not full
    B_0X0 = 0,
    ///1: Tx event FIFO full
    B_0X1 = 1,
}
impl From<EFF_A> for bool {
    #[inline(always)]
    fn from(variant: EFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EFF` reader - Event FIFO full
pub struct EFF_R(crate::FieldReader<bool, EFF_A>);
impl EFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EFF_A {
        match self.bits {
            false => EFF_A::B_0X0,
            true => EFF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EFF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EFF_A::B_0X1
    }
}
impl core::ops::Deref for EFF_R {
    type Target = crate::FieldReader<bool, EFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEFL` reader - Tx Event FIFO element lost This bit is a copy of interrupt flag IR\[TEFL\]. When IR\[TEFL\]
///is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx Event FIFO of size 0.
pub struct TEFL_R(crate::FieldReader<bool, bool>);
impl TEFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3.
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 8:9 - Event FIFO get index Tx Event FIFO read index pointer, range 0 to 3.
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 16:17 - Event FIFO put index Tx Event FIFO write index pointer, range 0 to 3.
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 24 - Event FIFO full
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Tx Event FIFO element lost This bit is a copy of interrupt flag IR\[TEFL\]. When IR\[TEFL\]
    ///is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx Event FIFO of size 0.
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
///FDCAN Tx event FIFO status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txefs](index.html) module
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
///`read()` method returns [txefs::R](R) reader structure
impl crate::Readable for TXEFS_SPEC {
    type Reader = R;
}
///`reset()` method sets TXEFS to value 0
impl crate::Resettable for TXEFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
