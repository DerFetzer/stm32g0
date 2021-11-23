///Register `RXF0S` reader
pub struct R(crate::R<RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
///Field `F0FL` reader - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3.
pub struct F0FL_R(crate::FieldReader<u8, u8>);
impl F0FL_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0GI` reader - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2.
pub struct F0GI_R(crate::FieldReader<u8, u8>);
impl F0GI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0PI` reader - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2.
pub struct F0PI_R(crate::FieldReader<u8, u8>);
impl F0PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx FIFO 0 full
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F0F_A {
    ///0: Rx FIFO 0 not full
    B_0X0 = 0,
    ///1: Rx FIFO 0 full
    B_0X1 = 1,
}
impl From<F0F_A> for bool {
    #[inline(always)]
    fn from(variant: F0F_A) -> Self {
        variant as u8 != 0
    }
}
///Field `F0F` reader - Rx FIFO 0 full
pub struct F0F_R(crate::FieldReader<bool, F0F_A>);
impl F0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        F0F_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> F0F_A {
        match self.bits {
            false => F0F_A::B_0X0,
            true => F0F_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == F0F_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == F0F_A::B_0X1
    }
}
impl core::ops::Deref for F0F_R {
    type Target = crate::FieldReader<bool, F0F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\[RF0L\]. When IR\[RF0L\]
///is reset, this bit is also reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0L_A {
    ///0: No Rx FIFO 0 message lost
    B_0X0 = 0,
    ///1: Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0
    B_0X1 = 1,
}
impl From<RF0L_A> for bool {
    #[inline(always)]
    fn from(variant: RF0L_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0L` reader - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\[RF0L\]. When IR\[RF0L\]
///is reset, this bit is also reset.
pub struct RF0L_R(crate::FieldReader<bool, RF0L_A>);
impl RF0L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0L_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RF0L_A {
        match self.bits {
            false => RF0L_A::B_0X0,
            true => RF0L_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RF0L_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RF0L_A::B_0X1
    }
}
impl core::ops::Deref for RF0L_R {
    type Target = crate::FieldReader<bool, RF0L_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:3 - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3.
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 16:17 - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    ///Bit 24 - Rx FIFO 0 full
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\[RF0L\]. When IR\[RF0L\]
    ///is reset, this bit is also reset.
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
///FDCAN Rx FIFO 0 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf0s](index.html) module
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf0s::R](R) reader structure
impl crate::Readable for RXF0S_SPEC {
    type Reader = R;
}
///`reset()` method sets RXF0S to value 0
impl crate::Resettable for RXF0S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
