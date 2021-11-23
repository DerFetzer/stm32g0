///Register `RXF1S` reader
pub struct R(crate::R<RXF1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1S_SPEC>) -> Self {
        R(reader)
    }
}
///Field `F1FL` reader - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3.
pub struct F1FL_R(crate::FieldReader<u8, u8>);
impl F1FL_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1GI` reader - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2.
pub struct F1GI_R(crate::FieldReader<u8, u8>);
impl F1GI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1PI` reader - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2.
pub struct F1PI_R(crate::FieldReader<u8, u8>);
impl F1PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx FIFO 1 full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F1F_A {
    ///0: Rx FIFO 1 not full
    B_0X0 = 0,
    ///1: Rx FIFO 1 full
    B_0X1 = 1,
}
impl From<F1F_A> for bool {
    #[inline(always)]
    fn from(variant: F1F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `F1F` reader - Rx FIFO 1 full
pub struct F1F_R(crate::FieldReader<bool, F1F_A>);
impl F1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        F1F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> F1F_A {
        match self.bits {
            false => F1F_A::B_0X0,
            true => F1F_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == F1F_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == F1F_A::B_0X1
    }
}
impl core::ops::Deref for F1F_R {
    type Target = crate::FieldReader<bool, F1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\[RF1L\]. When IR\[RF1L\]
///is reset, this bit is also reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1L_A {
    ///0: No Rx FIFO 1 message lost
    B_0X0 = 0,
    ///1: Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0
    B_0X1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1L` reader - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\[RF1L\]. When IR\[RF1L\]
///is reset, this bit is also reset.
pub struct RF1L_R(crate::FieldReader<bool, RF1L_A>);
impl RF1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0X0,
            true => RF1L_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF1L_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF1L_A::B_0X1
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, RF1L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:3 - Rx FIFO 1 fill level Number of elements stored in Rx FIFO 1, range 0 to 3.
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Rx FIFO 1 get index Rx FIFO 1 read index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 16:17 - Rx FIFO 1 put index Rx FIFO 1 write index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 24 - Rx FIFO 1 full
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Rx FIFO 1 message lost This bit is a copy of interrupt flag IR\[RF1L\]. When IR\[RF1L\]
    ///is reset, this bit is also reset.
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
///FDCAN Rx FIFO 1 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf1s](index.html) module
pub struct RXF1S_SPEC;
impl crate::RegisterSpec for RXF1S_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf1s::R](R) reader structure
impl crate::Readable for RXF1S_SPEC {
    type Reader = R;
}
///`reset()` method sets RXF1S to value 0
impl crate::Resettable for RXF1S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
