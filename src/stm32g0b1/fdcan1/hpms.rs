///Register `HPMS` reader
pub struct R(crate::R<HPMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPMS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BIDX` reader - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\[1\]
///= 1.
pub struct BIDX_R(crate::FieldReader<u8, u8>);
impl BIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Message storage indicator
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSI_A {
    ///0: No FIFO selected
    B_0X0 = 0,
    ///1: FIFO overrun
    B_0X1 = 1,
    ///2: Message stored in FIFO 0
    B_0X2 = 2,
    ///3: Message stored in FIFO 1
    B_0X3 = 3,
}
impl From<MSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSI_A) -> Self {
        variant as _
    }
}
///Field `MSI` reader - Message storage indicator
pub struct MSI_R(crate::FieldReader<u8, MSI_A>);
impl MSI_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSI_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSI_A {
        match self.bits {
            0 => MSI_A::B_0X0,
            1 => MSI_A::B_0X1,
            2 => MSI_A::B_0X2,
            3 => MSI_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MSI_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MSI_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == MSI_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == MSI_A::B_0X3
    }
}
impl core::ops::Deref for MSI_R {
    type Target = crate::FieldReader<u8, MSI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FIDX` reader - Filter index Index of matching filter element. Range is 0 to RXGFC\[LSS\]
///- 1 or RXGFC\[LSE\]
///- 1.
pub struct FIDX_R(crate::FieldReader<u8, u8>);
impl FIDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Filter list Indicates the filter list of the matching filter element.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLST_A {
    ///0: Standard filter list
    B_0X0 = 0,
    ///1: Extended filter list
    B_0X1 = 1,
}
impl From<FLST_A> for bool {
    #[inline(always)]
    fn from(variant: FLST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FLST` reader - Filter list Indicates the filter list of the matching filter element.
pub struct FLST_R(crate::FieldReader<bool, FLST_A>);
impl FLST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLST_A {
        match self.bits {
            false => FLST_A::B_0X0,
            true => FLST_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FLST_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FLST_A::B_0X1
    }
}
impl core::ops::Deref for FLST_R {
    type Target = crate::FieldReader<bool, FLST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - Buffer index Index of Rx FIFO element to which the message was stored. Only valid when MSI\[1\]
    ///= 1.
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 6:7 - Message storage indicator
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 8:12 - Filter index Index of matching filter element. Range is 0 to RXGFC\[LSS\]
    ///- 1 or RXGFC\[LSE\]
    ///- 1.
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 15 - Filter list Indicates the filter list of the matching filter element.
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
///FDCAN high-priority message status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hpms](index.html) module
pub struct HPMS_SPEC;
impl crate::RegisterSpec for HPMS_SPEC {
    type Ux = u32;
}
///`read()` method returns [hpms::R](R) reader structure
impl crate::Readable for HPMS_SPEC {
    type Reader = R;
}
///`reset()` method sets HPMS to value 0
impl crate::Resettable for HPMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
