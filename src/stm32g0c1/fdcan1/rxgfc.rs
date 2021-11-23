///Register `RXGFC` reader
pub struct R(crate::R<RXGFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXGFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXGFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXGFC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RXGFC` writer
pub struct W(crate::W<RXGFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXGFC_SPEC>;
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
impl From<crate::W<RXGFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXGFC_SPEC>) -> Self {
        W(writer)
    }
}
///Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRFE_A {
    ///0: Filter remote frames with 29-bit standard IDs
    B_0X0 = 0,
    ///1: Reject all remote frames with 29-bit standard IDs
    B_0X1 = 1,
}
impl From<RRFE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFE` reader - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct RRFE_R(crate::FieldReader<bool, RRFE_A>);
impl RRFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRFE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRFE_A {
        match self.bits {
            false => RRFE_A::B_0X0,
            true => RRFE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RRFE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RRFE_A::B_0X1
    }
}
impl core::ops::Deref for RRFE_R {
    type Target = crate::FieldReader<bool, RRFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RRFE` writer - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct RRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RRFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Filter remote frames with 29-bit standard IDs
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRFE_A::B_0X0)
    }
    ///Reject all remote frames with 29-bit standard IDs
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRFE_A::B_0X1)
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
///Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRFS_A {
    ///0: Filter remote frames with 11-bit standard IDs
    B_0X0 = 0,
    ///1: Reject all remote frames with 11-bit standard IDs
    B_0X1 = 1,
}
impl From<RRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RRFS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RRFS` reader - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct RRFS_R(crate::FieldReader<bool, RRFS_A>);
impl RRFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRFS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRFS_A {
        match self.bits {
            false => RRFS_A::B_0X0,
            true => RRFS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RRFS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RRFS_A::B_0X1
    }
}
impl core::ops::Deref for RRFS_R {
    type Target = crate::FieldReader<bool, RRFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RRFS` writer - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct RRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RRFS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Filter remote frames with 11-bit standard IDs
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRFS_A::B_0X0)
    }
    ///Reject all remote frames with 11-bit standard IDs
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRFS_A::B_0X1)
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
///Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANFE_A {
    ///0: Accept in Rx FIFO 0
    B_0X0 = 0,
    ///1: Accept in Rx FIFO 1
    B_0X1 = 1,
    ///2: Reject
    B_0X2 = 2,
    ///3: Reject
    B_0X3 = 3,
}
impl From<ANFE_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFE_A) -> Self {
        variant as _
    }
}
///Field `ANFE` reader - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct ANFE_R(crate::FieldReader<u8, ANFE_A>);
impl ANFE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANFE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANFE_A {
        match self.bits {
            0 => ANFE_A::B_0X0,
            1 => ANFE_A::B_0X1,
            2 => ANFE_A::B_0X2,
            3 => ANFE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ANFE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ANFE_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == ANFE_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == ANFE_A::B_0X3
    }
}
impl core::ops::Deref for ANFE_R {
    type Target = crate::FieldReader<u8, ANFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ANFE` writer - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct ANFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ANFE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Accept in Rx FIFO 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X0)
    }
    ///Accept in Rx FIFO 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X1)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X2)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ANFE_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANFS_A {
    ///0: Accept in Rx FIFO 0
    B_0X0 = 0,
    ///1: Accept in Rx FIFO 1
    B_0X1 = 1,
    ///2: Reject
    B_0X2 = 2,
    ///3: Reject
    B_0X3 = 3,
}
impl From<ANFS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFS_A) -> Self {
        variant as _
    }
}
///Field `ANFS` reader - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct ANFS_R(crate::FieldReader<u8, ANFS_A>);
impl ANFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ANFS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ANFS_A {
        match self.bits {
            0 => ANFS_A::B_0X0,
            1 => ANFS_A::B_0X1,
            2 => ANFS_A::B_0X2,
            3 => ANFS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ANFS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ANFS_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == ANFS_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == ANFS_A::B_0X3
    }
}
impl core::ops::Deref for ANFS_R {
    type Target = crate::FieldReader<u8, ANFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ANFS` writer - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct ANFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ANFS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Accept in Rx FIFO 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X0)
    }
    ///Accept in Rx FIFO 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X1)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X2)
    }
    ///Reject
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ANFS_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Field `F1OM` reader - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct F1OM_R(crate::FieldReader<bool, bool>);
impl F1OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        F1OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1OM` writer - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct F1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F1OM_W<'a> {
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
///Field `F0OM` reader - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct F0OM_R(crate::FieldReader<bool, bool>);
impl F0OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        F0OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0OM` writer - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct F0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0OM_W<'a> {
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
///List size standard >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSS_A {
    ///0: No standard message ID filter
    B_0X0 = 0,
    ///1: Number of standard message ID filter elements
    B_0X1 = 1,
    ///2: Number of standard message ID filter elements
    B_0X2 = 2,
    ///3: Number of standard message ID filter elements
    B_0X3 = 3,
    ///4: Number of standard message ID filter elements
    B_0X4 = 4,
    ///5: Number of standard message ID filter elements
    B_0X5 = 5,
    ///6: Number of standard message ID filter elements
    B_0X6 = 6,
    ///7: Number of standard message ID filter elements
    B_0X7 = 7,
    ///8: Number of standard message ID filter elements
    B_0X8 = 8,
    ///9: Number of standard message ID filter elements
    B_0X9 = 9,
    ///10: Number of standard message ID filter elements
    B_0XA = 10,
    ///11: Number of standard message ID filter elements
    B_0XB = 11,
    ///12: Number of standard message ID filter elements
    B_0XC = 12,
    ///13: Number of standard message ID filter elements
    B_0XD = 13,
    ///14: Number of standard message ID filter elements
    B_0XE = 14,
    ///15: Number of standard message ID filter elements
    B_0XF = 15,
    ///16: Number of standard message ID filter elements
    B_0X10 = 16,
    ///17: Number of standard message ID filter elements
    B_0X11 = 17,
    ///18: Number of standard message ID filter elements
    B_0X12 = 18,
    ///19: Number of standard message ID filter elements
    B_0X13 = 19,
    ///20: Number of standard message ID filter elements
    B_0X14 = 20,
    ///21: Number of standard message ID filter elements
    B_0X15 = 21,
    ///22: Number of standard message ID filter elements
    B_0X16 = 22,
    ///23: Number of standard message ID filter elements
    B_0X17 = 23,
    ///24: Number of standard message ID filter elements
    B_0X18 = 24,
    ///25: Number of standard message ID filter elements
    B_0X19 = 25,
    ///26: Number of standard message ID filter elements
    B_0X1A = 26,
    ///27: Number of standard message ID filter elements
    B_0X1B = 27,
    ///28: Number of standard message ID filter elements
    B_0X1C = 28,
}
impl From<LSS_A> for u8 {
    #[inline(always)]
    fn from(variant: LSS_A) -> Self {
        variant as _
    }
}
///Field `LSS` reader - List size standard >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct LSS_R(crate::FieldReader<u8, LSS_A>);
impl LSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LSS_A> {
        match self.bits {
            0 => Some(LSS_A::B_0X0),
            1 => Some(LSS_A::B_0X1),
            2 => Some(LSS_A::B_0X2),
            3 => Some(LSS_A::B_0X3),
            4 => Some(LSS_A::B_0X4),
            5 => Some(LSS_A::B_0X5),
            6 => Some(LSS_A::B_0X6),
            7 => Some(LSS_A::B_0X7),
            8 => Some(LSS_A::B_0X8),
            9 => Some(LSS_A::B_0X9),
            10 => Some(LSS_A::B_0XA),
            11 => Some(LSS_A::B_0XB),
            12 => Some(LSS_A::B_0XC),
            13 => Some(LSS_A::B_0XD),
            14 => Some(LSS_A::B_0XE),
            15 => Some(LSS_A::B_0XF),
            16 => Some(LSS_A::B_0X10),
            17 => Some(LSS_A::B_0X11),
            18 => Some(LSS_A::B_0X12),
            19 => Some(LSS_A::B_0X13),
            20 => Some(LSS_A::B_0X14),
            21 => Some(LSS_A::B_0X15),
            22 => Some(LSS_A::B_0X16),
            23 => Some(LSS_A::B_0X17),
            24 => Some(LSS_A::B_0X18),
            25 => Some(LSS_A::B_0X19),
            26 => Some(LSS_A::B_0X1A),
            27 => Some(LSS_A::B_0X1B),
            28 => Some(LSS_A::B_0X1C),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LSS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LSS_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == LSS_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == LSS_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == LSS_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == LSS_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == LSS_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == LSS_A::B_0X7
    }
    ///Checks if the value of the field is `B_0X8`
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == LSS_A::B_0X8
    }
    ///Checks if the value of the field is `B_0X9`
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        **self == LSS_A::B_0X9
    }
    ///Checks if the value of the field is `B_0XA`
    #[inline(always)]
    pub fn is_b_0xa(&self) -> bool {
        **self == LSS_A::B_0XA
    }
    ///Checks if the value of the field is `B_0XB`
    #[inline(always)]
    pub fn is_b_0xb(&self) -> bool {
        **self == LSS_A::B_0XB
    }
    ///Checks if the value of the field is `B_0XC`
    #[inline(always)]
    pub fn is_b_0xc(&self) -> bool {
        **self == LSS_A::B_0XC
    }
    ///Checks if the value of the field is `B_0XD`
    #[inline(always)]
    pub fn is_b_0xd(&self) -> bool {
        **self == LSS_A::B_0XD
    }
    ///Checks if the value of the field is `B_0XE`
    #[inline(always)]
    pub fn is_b_0xe(&self) -> bool {
        **self == LSS_A::B_0XE
    }
    ///Checks if the value of the field is `B_0XF`
    #[inline(always)]
    pub fn is_b_0xf(&self) -> bool {
        **self == LSS_A::B_0XF
    }
    ///Checks if the value of the field is `B_0X10`
    #[inline(always)]
    pub fn is_b_0x10(&self) -> bool {
        **self == LSS_A::B_0X10
    }
    ///Checks if the value of the field is `B_0X11`
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        **self == LSS_A::B_0X11
    }
    ///Checks if the value of the field is `B_0X12`
    #[inline(always)]
    pub fn is_b_0x12(&self) -> bool {
        **self == LSS_A::B_0X12
    }
    ///Checks if the value of the field is `B_0X13`
    #[inline(always)]
    pub fn is_b_0x13(&self) -> bool {
        **self == LSS_A::B_0X13
    }
    ///Checks if the value of the field is `B_0X14`
    #[inline(always)]
    pub fn is_b_0x14(&self) -> bool {
        **self == LSS_A::B_0X14
    }
    ///Checks if the value of the field is `B_0X15`
    #[inline(always)]
    pub fn is_b_0x15(&self) -> bool {
        **self == LSS_A::B_0X15
    }
    ///Checks if the value of the field is `B_0X16`
    #[inline(always)]
    pub fn is_b_0x16(&self) -> bool {
        **self == LSS_A::B_0X16
    }
    ///Checks if the value of the field is `B_0X17`
    #[inline(always)]
    pub fn is_b_0x17(&self) -> bool {
        **self == LSS_A::B_0X17
    }
    ///Checks if the value of the field is `B_0X18`
    #[inline(always)]
    pub fn is_b_0x18(&self) -> bool {
        **self == LSS_A::B_0X18
    }
    ///Checks if the value of the field is `B_0X19`
    #[inline(always)]
    pub fn is_b_0x19(&self) -> bool {
        **self == LSS_A::B_0X19
    }
    ///Checks if the value of the field is `B_0X1A`
    #[inline(always)]
    pub fn is_b_0x1a(&self) -> bool {
        **self == LSS_A::B_0X1A
    }
    ///Checks if the value of the field is `B_0X1B`
    #[inline(always)]
    pub fn is_b_0x1b(&self) -> bool {
        **self == LSS_A::B_0X1B
    }
    ///Checks if the value of the field is `B_0X1C`
    #[inline(always)]
    pub fn is_b_0x1c(&self) -> bool {
        **self == LSS_A::B_0X1C
    }
}
impl core::ops::Deref for LSS_R {
    type Target = crate::FieldReader<u8, LSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSS` writer - List size standard >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct LSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No standard message ID filter
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSS_A::B_0X0)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSS_A::B_0X1)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LSS_A::B_0X2)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LSS_A::B_0X3)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LSS_A::B_0X4)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(LSS_A::B_0X5)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(LSS_A::B_0X6)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(LSS_A::B_0X7)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(LSS_A::B_0X8)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(LSS_A::B_0X9)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0xa(self) -> &'a mut W {
        self.variant(LSS_A::B_0XA)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0xb(self) -> &'a mut W {
        self.variant(LSS_A::B_0XB)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0xc(self) -> &'a mut W {
        self.variant(LSS_A::B_0XC)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0xd(self) -> &'a mut W {
        self.variant(LSS_A::B_0XD)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0xe(self) -> &'a mut W {
        self.variant(LSS_A::B_0XE)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0xf(self) -> &'a mut W {
        self.variant(LSS_A::B_0XF)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x10(self) -> &'a mut W {
        self.variant(LSS_A::B_0X10)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut W {
        self.variant(LSS_A::B_0X11)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x12(self) -> &'a mut W {
        self.variant(LSS_A::B_0X12)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x13(self) -> &'a mut W {
        self.variant(LSS_A::B_0X13)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x14(self) -> &'a mut W {
        self.variant(LSS_A::B_0X14)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x15(self) -> &'a mut W {
        self.variant(LSS_A::B_0X15)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x16(self) -> &'a mut W {
        self.variant(LSS_A::B_0X16)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x17(self) -> &'a mut W {
        self.variant(LSS_A::B_0X17)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x18(self) -> &'a mut W {
        self.variant(LSS_A::B_0X18)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x19(self) -> &'a mut W {
        self.variant(LSS_A::B_0X19)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x1a(self) -> &'a mut W {
        self.variant(LSS_A::B_0X1A)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x1b(self) -> &'a mut W {
        self.variant(LSS_A::B_0X1B)
    }
    ///Number of standard message ID filter elements
    #[inline(always)]
    pub fn b_0x1c(self) -> &'a mut W {
        self.variant(LSS_A::B_0X1C)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
///List size extended >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSE_A {
    ///0: No extended message ID filter
    B_0X0 = 0,
    ///1: Number of extended message ID filter elements
    B_0X1 = 1,
    ///2: Number of extended message ID filter elements
    B_0X2 = 2,
    ///3: Number of extended message ID filter elements
    B_0X3 = 3,
    ///4: Number of extended message ID filter elements
    B_0X4 = 4,
    ///5: Number of extended message ID filter elements
    B_0X5 = 5,
    ///6: Number of extended message ID filter elements
    B_0X6 = 6,
    ///7: Number of extended message ID filter elements
    B_0X7 = 7,
    ///8: Number of extended message ID filter elements
    B_0X8 = 8,
}
impl From<LSE_A> for u8 {
    #[inline(always)]
    fn from(variant: LSE_A) -> Self {
        variant as _
    }
}
///Field `LSE` reader - List size extended >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct LSE_R(crate::FieldReader<u8, LSE_A>);
impl LSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LSE_A> {
        match self.bits {
            0 => Some(LSE_A::B_0X0),
            1 => Some(LSE_A::B_0X1),
            2 => Some(LSE_A::B_0X2),
            3 => Some(LSE_A::B_0X3),
            4 => Some(LSE_A::B_0X4),
            5 => Some(LSE_A::B_0X5),
            6 => Some(LSE_A::B_0X6),
            7 => Some(LSE_A::B_0X7),
            8 => Some(LSE_A::B_0X8),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LSE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LSE_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == LSE_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == LSE_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == LSE_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == LSE_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == LSE_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == LSE_A::B_0X7
    }
    ///Checks if the value of the field is `B_0X8`
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        **self == LSE_A::B_0X8
    }
}
impl core::ops::Deref for LSE_R {
    type Target = crate::FieldReader<u8, LSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSE` writer - List size extended >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct LSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///No extended message ID filter
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSE_A::B_0X0)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSE_A::B_0X1)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LSE_A::B_0X2)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LSE_A::B_0X3)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LSE_A::B_0X4)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(LSE_A::B_0X5)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(LSE_A::B_0X6)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(LSE_A::B_0X7)
    }
    ///Number of extended message ID filter elements
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(LSE_A::B_0X8)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    ///Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn f1om(&self) -> F1OM_R {
        F1OM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn f0om(&self) -> F0OM_R {
        F0OM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bits 16:20 - List size standard >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:27 - List size extended >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Reject remote frames extended These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W {
        RRFE_W { w: self }
    }
    ///Bit 1 - Reject remote frames standard These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W {
        RRFS_W { w: self }
    }
    ///Bits 2:3 - Accept non-matching frames extended Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W {
        ANFE_W { w: self }
    }
    ///Bits 4:5 - Accept Non-matching frames standard Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W {
        ANFS_W { w: self }
    }
    ///Bit 8 - FIFO 1 operation mode (overwrite or blocking) This is a protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn f1om(&mut self) -> F1OM_W {
        F1OM_W { w: self }
    }
    ///Bit 9 - FIFO 0 operation mode (overwrite or blocking) This is protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn f0om(&mut self) -> F0OM_W {
        F0OM_W { w: self }
    }
    ///Bits 16:20 - List size standard >28: Values greater than 28 are interpreted as 28. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W {
        LSS_W { w: self }
    }
    ///Bits 24:27 - List size extended >8: Values greater than 8 are interpreted as 8. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W {
        LSE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN global filter configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxgfc](index.html) module
pub struct RXGFC_SPEC;
impl crate::RegisterSpec for RXGFC_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxgfc::R](R) reader structure
impl crate::Readable for RXGFC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rxgfc::W](W) writer structure
impl crate::Writable for RXGFC_SPEC {
    type Writer = W;
}
///`reset()` method sets RXGFC to value 0
impl crate::Resettable for RXGFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
