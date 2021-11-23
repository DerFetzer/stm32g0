///Register `CEC_CFGR` reader
pub struct R(crate::R<CEC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CEC_CFGR` writer
pub struct W(crate::W<CEC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CFGR_SPEC>;
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
impl From<crate::W<CEC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SFT_A {
    ///1: 0.5 nominal data bit periods
    B_0X1 = 1,
    ///2: 1.5 nominal data bit periods
    B_0X2 = 2,
    ///3: 2.5 nominal data bit periods
    B_0X3 = 3,
    ///4: 3.5 nominal data bit periods
    B_0X4 = 4,
    ///5: 4.5 nominal data bit periods
    B_0X5 = 5,
    ///6: 5.5 nominal data bit periods
    B_0X6 = 6,
    ///7: 6.5 nominal data bit periods
    B_0X7 = 7,
}
impl From<SFT_A> for u8 {
    #[inline(always)]
    fn from(variant: SFT_A) -> Self {
        variant as _
    }
}
///Field `SFT` reader - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
pub struct SFT_R(crate::FieldReader<u8, SFT_A>);
impl SFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SFT_A> {
        match self.bits {
            1 => Some(SFT_A::B_0X1),
            2 => Some(SFT_A::B_0X2),
            3 => Some(SFT_A::B_0X3),
            4 => Some(SFT_A::B_0X4),
            5 => Some(SFT_A::B_0X5),
            6 => Some(SFT_A::B_0X6),
            7 => Some(SFT_A::B_0X7),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SFT_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == SFT_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == SFT_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == SFT_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == SFT_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == SFT_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == SFT_A::B_0X7
    }
}
impl core::ops::Deref for SFT_R {
    type Target = crate::FieldReader<u8, SFT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SFT` writer - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
pub struct SFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SFT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///0.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SFT_A::B_0X1)
    }
    ///1.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SFT_A::B_0X2)
    }
    ///2.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SFT_A::B_0X3)
    }
    ///3.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SFT_A::B_0X4)
    }
    ///4.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SFT_A::B_0X5)
    }
    ///5.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(SFT_A::B_0X6)
    }
    ///6.5 nominal data bit periods
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(SFT_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOL_A {
    ///0: Standard tolerance margin:
    B_0X0 = 0,
    ///1: Extended tolerance
    B_0X1 = 1,
}
impl From<RXTOL_A> for bool {
    #[inline(always)]
    fn from(variant: RXTOL_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXTOL` reader - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall
pub struct RXTOL_R(crate::FieldReader<bool, RXTOL_A>);
impl RXTOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTOL_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXTOL_A {
        match self.bits {
            false => RXTOL_A::B_0X0,
            true => RXTOL_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXTOL_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXTOL_A::B_0X1
    }
}
impl core::ops::Deref for RXTOL_R {
    type Target = crate::FieldReader<bool, RXTOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXTOL` writer - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall
pub struct RXTOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTOL_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXTOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Standard tolerance margin:
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXTOL_A::B_0X0)
    }
    ///Extended tolerance
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXTOL_A::B_0X1)
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
///Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRESTP_A {
    ///0: BRE detection does not stop reception of the CEC message. Data bit is sampled at 1.05 ms.
    B_0X0 = 0,
    ///1: BRE detection stops message reception.
    B_0X1 = 1,
}
impl From<BRESTP_A> for bool {
    #[inline(always)]
    fn from(variant: BRESTP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRESTP` reader - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
pub struct BRESTP_R(crate::FieldReader<bool, BRESTP_A>);
impl BRESTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRESTP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BRESTP_A {
        match self.bits {
            false => BRESTP_A::B_0X0,
            true => BRESTP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BRESTP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BRESTP_A::B_0X1
    }
}
impl core::ops::Deref for BRESTP_R {
    type Target = crate::FieldReader<bool, BRESTP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BRESTP` writer - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
pub struct BRESTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRESTP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BRESTP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///BRE detection does not stop reception of the CEC message. Data bit is sampled at 1.05 ms.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRESTP_A::B_0X0)
    }
    ///BRE detection stops message reception.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRESTP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREGEN_A {
    ///0: BRE detection does not generate an error-bit on the CEC line.
    B_0X0 = 0,
    ///1: BRE detection generates an error-bit on the CEC line (if BRESTP is set).
    B_0X1 = 1,
}
impl From<BREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BREGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BREGEN` reader - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
pub struct BREGEN_R(crate::FieldReader<bool, BREGEN_A>);
impl BREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREGEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BREGEN_A {
        match self.bits {
            false => BREGEN_A::B_0X0,
            true => BREGEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BREGEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BREGEN_A::B_0X1
    }
}
impl core::ops::Deref for BREGEN_R {
    type Target = crate::FieldReader<bool, BREGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BREGEN` writer - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
pub struct BREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BREGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///BRE detection does not generate an error-bit on the CEC line.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BREGEN_A::B_0X0)
    }
    ///BRE detection generates an error-bit on the CEC line (if BRESTP is set).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BREGEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBPEGEN_A {
    ///0: LBPE detection does not generate an error-bit on the CEC line.
    B_0X0 = 0,
    ///1: LBPE detection generates an error-bit on the CEC line.
    B_0X1 = 1,
}
impl From<LBPEGEN_A> for bool {
    #[inline(always)]
    fn from(variant: LBPEGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LBPEGEN` reader - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
pub struct LBPEGEN_R(crate::FieldReader<bool, LBPEGEN_A>);
impl LBPEGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBPEGEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBPEGEN_A {
        match self.bits {
            false => LBPEGEN_A::B_0X0,
            true => LBPEGEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LBPEGEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LBPEGEN_A::B_0X1
    }
}
impl core::ops::Deref for LBPEGEN_R {
    type Target = crate::FieldReader<bool, LBPEGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LBPEGEN` writer - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
pub struct LBPEGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LBPEGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///LBPE detection does not generate an error-bit on the CEC line.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LBPEGEN_A::B_0X0)
    }
    ///LBPE detection generates an error-bit on the CEC line.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LBPEGEN_A::B_0X1)
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
///Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDNOGEN_A {
    ///0: BRE detection with BRESTP = 1 and BREGEN = 0 on a broadcast message generates an    
    B_0X0 = 0,
    ///1: Error-bit is not generated in the same condition as above. An error-bit is not generated even in case of an SBPE detection in a broadcast message if listen mode is set.
    B_0X1 = 1,
}
impl From<BRDNOGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRDNOGEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRDNOGEN` reader - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
pub struct BRDNOGEN_R(crate::FieldReader<bool, BRDNOGEN_A>);
impl BRDNOGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRDNOGEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BRDNOGEN_A {
        match self.bits {
            false => BRDNOGEN_A::B_0X0,
            true => BRDNOGEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BRDNOGEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BRDNOGEN_A::B_0X1
    }
}
impl core::ops::Deref for BRDNOGEN_R {
    type Target = crate::FieldReader<bool, BRDNOGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BRDNOGEN` writer - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
pub struct BRDNOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDNOGEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BRDNOGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///BRE detection with BRESTP = 1 and BREGEN = 0 on a broadcast message generates an
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRDNOGEN_A::B_0X0)
    }
    ///Error-bit is not generated in the same condition as above. An error-bit is not generated even in case of an SBPE detection in a broadcast message if listen mode is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRDNOGEN_A::B_0X1)
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
///SFT option bit The SFTOPT bit is set and cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SFTOP_A {
    ///0: SFT timer starts when TXSOM is set by software.
    B_0X0 = 0,
    ///1: SFT timer starts automatically at the end of message transmission/reception.
    B_0X1 = 1,
}
impl From<SFTOP_A> for bool {
    #[inline(always)]
    fn from(variant: SFTOP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SFTOP` reader - SFT option bit The SFTOPT bit is set and cleared by software.
pub struct SFTOP_R(crate::FieldReader<bool, SFTOP_A>);
impl SFTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFTOP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SFTOP_A {
        match self.bits {
            false => SFTOP_A::B_0X0,
            true => SFTOP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == SFTOP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == SFTOP_A::B_0X1
    }
}
impl core::ops::Deref for SFTOP_R {
    type Target = crate::FieldReader<bool, SFTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SFTOP` writer - SFT option bit The SFTOPT bit is set and cleared by software.
pub struct SFTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTOP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SFTOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///SFT timer starts when TXSOM is set by software.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SFTOP_A::B_0X0)
    }
    ///SFT timer starts automatically at the end of message transmission/reception.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SFTOP_A::B_0X1)
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
///Field `OAR` reader - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
pub struct OAR_R(crate::FieldReader<u16, u16>);
impl OAR_R {
    pub(crate) fn new(bits: u16) -> Self {
        OAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OAR` writer - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
pub struct OAR_W<'a> {
    w: &'a mut W,
}
impl<'a> OAR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | ((value as u32 & 0x7fff) << 16);
        self.w
    }
}
///Listen mode LSTN bit is set and cleared by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSTN_A {
    ///0: CEC peripheral receives only message addressed to its own address (OAR). Messages addressed to different destination are ignored. Broadcast messages are always received.
    B_0X0 = 0,
    ///1: CEC peripheral receives messages addressed to its own address (OAR) with positive acknowledge. Messages addressed to different destination are received, but without interfering with the CEC bus: no acknowledge sent.
    B_0X1 = 1,
}
impl From<LSTN_A> for bool {
    #[inline(always)]
    fn from(variant: LSTN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LSTN` reader - Listen mode LSTN bit is set and cleared by software.
pub struct LSTN_R(crate::FieldReader<bool, LSTN_A>);
impl LSTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSTN_A {
        match self.bits {
            false => LSTN_A::B_0X0,
            true => LSTN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LSTN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LSTN_A::B_0X1
    }
}
impl core::ops::Deref for LSTN_R {
    type Target = crate::FieldReader<bool, LSTN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LSTN` writer - Listen mode LSTN bit is set and cleared by software.
pub struct LSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LSTN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CEC peripheral receives only message addressed to its own address (OAR). Messages addressed to different destination are ignored. Broadcast messages are always received.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSTN_A::B_0X0)
    }
    ///CEC peripheral receives messages addressed to its own address (OAR) with positive acknowledge. Messages addressed to different destination are received, but without interfering with the CEC bus: no acknowledge sent.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSTN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new((self.bits & 0x07) as u8)
    }
    ///Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall
    #[inline(always)]
    pub fn rxtol(&self) -> RXTOL_R {
        RXTOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
    #[inline(always)]
    pub fn brestp(&self) -> BRESTP_R {
        BRESTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
    #[inline(always)]
    pub fn bregen(&self) -> BREGEN_R {
        BREGEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
    #[inline(always)]
    pub fn lbpegen(&self) -> LBPEGEN_R {
        LBPEGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
    #[inline(always)]
    pub fn brdnogen(&self) -> BRDNOGEN_R {
        BRDNOGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software.
    #[inline(always)]
    pub fn sftop(&self) -> SFTOP_R {
        SFTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    ///Bit 31 - Listen mode LSTN bit is set and cleared by software.
    #[inline(always)]
    pub fn lstn(&self) -> LSTN_R {
        LSTN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:2 - Signal free time SFT bits are set by software. In the SFT = 0x0 configuration, the number of nominal data bit periods waited before transmission is ruled by hardware according to the transmission history. In all the other configurations the SFT number is determined by software. 0x0 2.5 data-bit periods if CEC is the last bus initiator with unsuccessful transmission (ARBLST = 1, TXERR = 1, TXUDR = 1 or TXACKE = 1) 4 data-bit periods if CEC is the new bus initiator 6 data-bit periods if CEC is the last bus initiator with successful transmission (TXEOM = 1)
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W {
        SFT_W { w: self }
    }
    ///Bit 3 - Rx-tolerance The RXTOL bit is set and cleared by software. Start-bit, +/- 200 µs rise, +/- 200 µs fall Data-bit: +/- 200 µs rise. +/- 350 µs fall Start-bit: +/- 400 µs rise, +/- 400 µs fall Data-bit: +/-300 µs rise, +/- 500 µs fall
    #[inline(always)]
    pub fn rxtol(&mut self) -> RXTOL_W {
        RXTOL_W { w: self }
    }
    ///Bit 4 - Rx-stop on bit rising error The BRESTP bit is set and cleared by software.
    #[inline(always)]
    pub fn brestp(&mut self) -> BRESTP_W {
        BRESTP_W { w: self }
    }
    ///Bit 5 - Generate error-bit on bit rising error The BREGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon BRE detection with BRESTP = 1 in broadcast even if BREGEN = 0.
    #[inline(always)]
    pub fn bregen(&mut self) -> BREGEN_W {
        BREGEN_W { w: self }
    }
    ///Bit 6 - Generate error-bit on long bit period error The LBPEGEN bit is set and cleared by software. Note: If BRDNOGEN = 0, an error-bit is generated upon LBPE detection in broadcast even if LBPEGEN = 0.
    #[inline(always)]
    pub fn lbpegen(&mut self) -> LBPEGEN_W {
        LBPEGEN_W { w: self }
    }
    ///Bit 7 - Avoid error-bit generation in broadcast The BRDNOGEN bit is set and cleared by software. error-bit on the CEC line. LBPE detection with LBPEGEN = 0 on a broadcast message generates an error-bit on the CEC line.
    #[inline(always)]
    pub fn brdnogen(&mut self) -> BRDNOGEN_W {
        BRDNOGEN_W { w: self }
    }
    ///Bit 8 - SFT option bit The SFTOPT bit is set and cleared by software.
    #[inline(always)]
    pub fn sftop(&mut self) -> SFTOP_W {
        SFTOP_W { w: self }
    }
    ///Bits 16:30 - Own addresses configuration The OAR bits are set by software to select which destination logical addresses has to be considered in receive mode. Each bit, when set, enables the CEC logical address identified by the given bit position. At the end of HEADER reception, the received destination address is compared with the enabled addresses. In case of matching address, the incoming message is acknowledged and received. In case of non-matching address, the incoming message is received only in listen mode (LSTN = 1), but without acknowledge sent. Broadcast messages are always received. Example: OAR = 0b000 0000 0010 0001 means that CEC acknowledges addresses 0x0 and 0x5. Consequently, each message directed to one of these addresses is received.
    #[inline(always)]
    pub fn oar(&mut self) -> OAR_W {
        OAR_W { w: self }
    }
    ///Bit 31 - Listen mode LSTN bit is set and cleared by software.
    #[inline(always)]
    pub fn lstn(&mut self) -> LSTN_W {
        LSTN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to configure the HDMI-CEC controller. It is mandatory to write CEC_CFGR only when CECEN=0.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cec_cfgr](index.html) module
pub struct CEC_CFGR_SPEC;
impl crate::RegisterSpec for CEC_CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cec_cfgr::R](R) reader structure
impl crate::Readable for CEC_CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cec_cfgr::W](W) writer structure
impl crate::Writable for CEC_CFGR_SPEC {
    type Writer = W;
}
///`reset()` method sets CEC_CFGR to value 0
impl crate::Resettable for CEC_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
