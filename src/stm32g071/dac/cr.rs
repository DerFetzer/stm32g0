///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN1` reader - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
pub struct EN1_R(crate::FieldReader<bool, bool>);
impl EN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EN1` writer - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
pub struct EN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1_W<'a> {
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
///Field `TEN1` reader - DAC channel1 trigger enable
pub struct TEN1_R(crate::FieldReader<bool, bool>);
impl TEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEN1` writer - DAC channel1 trigger enable
pub struct TEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN1_W<'a> {
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
///Field `TSEL1` reader - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
pub struct TSEL1_R(crate::FieldReader<u8, u8>);
impl TSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEL1` writer - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
///Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
pub struct WAVE1_R(crate::FieldReader<u8, u8>);
impl WAVE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
pub struct WAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
///Field `MAMP1` reader - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
///of LFSR/ triangle amplitude equal to 4095
pub struct MAMP1_R(crate::FieldReader<u8, u8>);
impl MAMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MAMP1` writer - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
///of LFSR/ triangle amplitude equal to 4095
pub struct MAMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP1_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
///Field `DMAEN1` reader - DAC channel1 DMA enable This bit is set and cleared by software.
pub struct DMAEN1_R(crate::FieldReader<bool, bool>);
impl DMAEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN1` writer - DAC channel1 DMA enable This bit is set and cleared by software.
pub struct DMAEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
pub struct DMAUDRIE1_R(crate::FieldReader<bool, bool>);
impl DMAUDRIE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAUDRIE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
pub struct DMAUDRIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///Field `CEN1` reader - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub struct CEN1_R(crate::FieldReader<bool, bool>);
impl CEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CEN1` writer - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub struct CEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Field `EN2` reader - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
pub struct EN2_R(crate::FieldReader<bool, bool>);
impl EN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EN2` writer - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
pub struct EN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
///Field `TEN2` reader - DAC channel2 trigger enable
pub struct TEN2_R(crate::FieldReader<bool, bool>);
impl TEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEN2` writer - DAC channel2 trigger enable
pub struct TEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
///Field `TSEL2` reader - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled).
pub struct TSEL2_R(crate::FieldReader<u8, u8>);
impl TSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEL2` writer - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled).
pub struct TSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
///Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
pub struct WAVE2_R(crate::FieldReader<u8, u8>);
impl WAVE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
pub struct WAVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
///Field `MAMP2` reader - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
///of LFSR/ triangle amplitude equal to 4095
pub struct MAMP2_R(crate::FieldReader<u8, u8>);
impl MAMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MAMP2` writer - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
///of LFSR/ triangle amplitude equal to 4095
pub struct MAMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMP2_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
///Field `DMAEN2` reader - DAC channel2 DMA enable This bit is set and cleared by software.
pub struct DMAEN2_R(crate::FieldReader<bool, bool>);
impl DMAEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAEN2` writer - DAC channel2 DMA enable This bit is set and cleared by software.
pub struct DMAEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
///Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
pub struct DMAUDRIE2_R(crate::FieldReader<bool, bool>);
impl DMAUDRIE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDRIE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAUDRIE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
pub struct DMAUDRIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDRIE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Field `CEN2` reader - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub struct CEN2_R(crate::FieldReader<bool, bool>);
impl CEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CEN2` writer - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
pub struct CEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    ///Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
    ///of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled).
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
    ///of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W {
        EN1_W { w: self }
    }
    ///Bit 1 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W {
        TEN1_W { w: self }
    }
    ///Bits 2:5 - DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W {
        WAVE1_W { w: self }
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
    ///of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W {
        MAMP1_W { w: self }
    }
    ///Bit 12 - DAC channel1 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W {
        DMAEN1_W { w: self }
    }
    ///Bit 13 - DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W {
        DMAUDRIE1_W { w: self }
    }
    ///Bit 14 - DAC Channel 1 calibration enable This bit is set and cleared by software to enable/disable DAC channel 1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W {
        CEN1_W { w: self }
    }
    ///Bit 16 - DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2.
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W {
        EN2_W { w: self }
    }
    ///Bit 17 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W {
        TEN2_W { w: self }
    }
    ///Bits 18:21 - DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled).
    #[inline(always)]
    pub fn tsel2(&mut self) -> TSEL2_W {
        TSEL2_W { w: self }
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W {
        WAVE2_W { w: self }
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. = 1011: Unmask bits\[11:0\]
    ///of LFSR/ triangle amplitude equal to 4095
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W {
        MAMP2_W { w: self }
    }
    ///Bit 28 - DAC channel2 DMA enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W {
        DMAEN2_W { w: self }
    }
    ///Bit 29 - DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W {
        DMAUDRIE2_W { w: self }
    }
    ///Bit 30 - DAC Channel 2 calibration enable This bit is set and cleared by software to enable/disable DAC channel 2 calibration, it can be written only if bit EN2=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN2_W {
        CEN2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
