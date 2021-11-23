///Register `TXFQS` reader
pub struct R(crate::R<TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TFFL` reader - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\[TFQM\]
///= 1).
pub struct TFFL_R(crate::FieldReader<u8, u8>);
impl TFFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFGI` reader - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)
pub struct TFGI_R(crate::FieldReader<u8, u8>);
impl TFGI_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFGI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFGI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TFQPI` reader - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3
pub struct TFQPI_R(crate::FieldReader<u8, u8>);
impl TFQPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFQPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFQPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Tx FIFO/queue full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFQF_A {
    ///0: Tx FIFO/queue not full
    B_0X0 = 0,
    ///1: Tx FIFO/queue full
    B_0X1 = 1,
}
impl From<TFQF_A> for bool {
    #[inline(always)]
    fn from(variant: TFQF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TFQF` reader - Tx FIFO/queue full
pub struct TFQF_R(crate::FieldReader<bool, TFQF_A>);
impl TFQF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFQF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TFQF_A {
        match self.bits {
            false => TFQF_A::B_0X0,
            true => TFQF_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TFQF_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TFQF_A::B_0X1
    }
}
impl core::ops::Deref for TFQF_R {
    type Target = crate::FieldReader<bool, TFQF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - Tx FIFO free level Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC\[TFQM\]
    ///= 1).
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 8:9 - Tx FIFO get index Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured (TXBC.TFQM = 1)
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 16:17 - Tx FIFO/queue put index Tx FIFO/queue write index pointer, range 0 to 3
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 21 - Tx FIFO/queue full
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
///FDCAN Tx FIFO/queue status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txfqs](index.html) module
pub struct TXFQS_SPEC;
impl crate::RegisterSpec for TXFQS_SPEC {
    type Ux = u32;
}
///`read()` method returns [txfqs::R](R) reader structure
impl crate::Readable for TXFQS_SPEC {
    type Reader = R;
}
///`reset()` method sets TXFQS to value 0x03
impl crate::Resettable for TXFQS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
