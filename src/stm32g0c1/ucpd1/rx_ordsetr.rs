///Register `RX_ORDSETR` reader
pub struct R(crate::R<RX_ORDSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ORDSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ORDSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ORDSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Rx ordered set code detected
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXORDSET_A {
    ///0: SOP code detected in receiver
    B_0X0 = 0,
    ///1: SOP' code detected in receiver
    B_0X1 = 1,
    ///2: SOP'' code detected in receiver
    B_0X2 = 2,
    ///3: SOP'_Debug detected in receiver
    B_0X3 = 3,
    ///4: SOP''_Debug detected in receiver
    B_0X4 = 4,
    ///5: Cable Reset detected in receiver
    B_0X5 = 5,
    ///6: SOP extension#1 detected in receiver
    B_0X6 = 6,
    ///7: SOP extension#2 detected in receiver
    B_0X7 = 7,
}
impl From<RXORDSET_A> for u8 {
    #[inline(always)]
    fn from(variant: RXORDSET_A) -> Self {
        variant as _
    }
}
///Field `RXORDSET` reader - Rx ordered set code detected
pub struct RXORDSET_R(crate::FieldReader<u8, RXORDSET_A>);
impl RXORDSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXORDSET_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXORDSET_A {
        match self.bits {
            0 => RXORDSET_A::B_0X0,
            1 => RXORDSET_A::B_0X1,
            2 => RXORDSET_A::B_0X2,
            3 => RXORDSET_A::B_0X3,
            4 => RXORDSET_A::B_0X4,
            5 => RXORDSET_A::B_0X5,
            6 => RXORDSET_A::B_0X6,
            7 => RXORDSET_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXORDSET_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXORDSET_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == RXORDSET_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == RXORDSET_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == RXORDSET_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == RXORDSET_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == RXORDSET_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == RXORDSET_A::B_0X7
    }
}
impl core::ops::Deref for RXORDSET_R {
    type Target = crate::FieldReader<u8, RXORDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///The bit indicates the number of correct K‑codes. For debug purposes only.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSOP3OF4_A {
    ///0: 4 correct K‑codes out of 4‑
    B_0X0 = 0,
    ///1: 3 correct K‑codes out of 4‑
    B_0X1 = 1,
}
impl From<RXSOP3OF4_A> for bool {
    #[inline(always)]
    fn from(variant: RXSOP3OF4_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXSOP3OF4` reader - The bit indicates the number of correct K‑codes. For debug purposes only.
pub struct RXSOP3OF4_R(crate::FieldReader<bool, RXSOP3OF4_A>);
impl RXSOP3OF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSOP3OF4_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXSOP3OF4_A {
        match self.bits {
            false => RXSOP3OF4_A::B_0X0,
            true => RXSOP3OF4_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXSOP3OF4_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXSOP3OF4_A::B_0X1
    }
}
impl core::ops::Deref for RXSOP3OF4_R {
    type Target = crate::FieldReader<bool, RXSOP3OF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///The bitfield is for debug purposes only. Others: Invalid
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXSOPKINVALID_A {
    ///0: No K‑code corrupted
    B_0X0 = 0,
    ///1: First K‑code corrupted
    B_0X1 = 1,
    ///2: Second K‑code corrupted
    B_0X2 = 2,
    ///3: Third K‑code corrupted
    B_0X3 = 3,
    ///4: Fourth K‑code corrupted
    B_0X4 = 4,
}
impl From<RXSOPKINVALID_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSOPKINVALID_A) -> Self {
        variant as _
    }
}
///Field `RXSOPKINVALID` reader - The bitfield is for debug purposes only. Others: Invalid
pub struct RXSOPKINVALID_R(crate::FieldReader<u8, RXSOPKINVALID_A>);
impl RXSOPKINVALID_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXSOPKINVALID_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RXSOPKINVALID_A> {
        match self.bits {
            0 => Some(RXSOPKINVALID_A::B_0X0),
            1 => Some(RXSOPKINVALID_A::B_0X1),
            2 => Some(RXSOPKINVALID_A::B_0X2),
            3 => Some(RXSOPKINVALID_A::B_0X3),
            4 => Some(RXSOPKINVALID_A::B_0X4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXSOPKINVALID_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXSOPKINVALID_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == RXSOPKINVALID_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == RXSOPKINVALID_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == RXSOPKINVALID_A::B_0X4
    }
}
impl core::ops::Deref for RXSOPKINVALID_R {
    type Target = crate::FieldReader<u8, RXSOPKINVALID_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:2 - Rx ordered set code detected
    #[inline(always)]
    pub fn rxordset(&self) -> RXORDSET_R {
        RXORDSET_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 3 - The bit indicates the number of correct K‑codes. For debug purposes only.
    #[inline(always)]
    pub fn rxsop3of4(&self) -> RXSOP3OF4_R {
        RXSOP3OF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bits 4:6 - The bitfield is for debug purposes only. Others: Invalid
    #[inline(always)]
    pub fn rxsopkinvalid(&self) -> RXSOPKINVALID_R {
        RXSOPKINVALID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
///UCPD Rx ordered set register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_ordsetr](index.html) module
pub struct RX_ORDSETR_SPEC;
impl crate::RegisterSpec for RX_ORDSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_ordsetr::R](R) reader structure
impl crate::Readable for RX_ORDSETR_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_ORDSETR to value 0
impl crate::Resettable for RX_ORDSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
