///Register `CCIPR` reader
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR` writer
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART1SEL` reader - USART1 clock source selection
pub struct USART1SEL_R(crate::FieldReader<u8, u8>);
impl USART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART1SEL` writer - USART1 clock source selection
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
///Field `USART2SEL` reader - USART2 clock source selection
pub struct USART2SEL_R(crate::FieldReader<u8, u8>);
impl USART2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART2SEL` writer - USART2 clock source selection
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
///Field `USART3SEL` reader - USART3 clock source selection
pub struct USART3SEL_R(crate::FieldReader<u8, u8>);
impl USART3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART3SEL` writer - USART3 clock source selection
pub struct USART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
///Field `CECSEL` reader - HDMI CEC clock source selection
pub struct CECSEL_R(crate::FieldReader<bool, bool>);
impl CECSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CECSEL` writer - HDMI CEC clock source selection
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
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
///Field `LPUART2SEL` reader - LPUART2 clock source selection
pub struct LPUART2SEL_R(crate::FieldReader<u8, u8>);
impl LPUART2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART2SEL` writer - LPUART2 clock source selection
pub struct LPUART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
///Field `LPUART1SEL` reader - LPUART1 clock source selection
pub struct LPUART1SEL_R(crate::FieldReader<u8, u8>);
impl LPUART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1SEL` writer - LPUART1 clock source selection
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
///Field `I2C1SEL` reader - I2C1 clock source selection
pub struct I2C1SEL_R(crate::FieldReader<u8, u8>);
impl I2C1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection
pub struct I2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
///Field `I2S2SEL` reader - I2S1 clock source selection
pub struct I2S2SEL_R(crate::FieldReader<u8, u8>);
impl I2S2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2S2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2S2SEL` writer - I2S1 clock source selection
pub struct I2S2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
///Field `LPTIM1SEL` reader - LPTIM1 clock source selection
pub struct LPTIM1SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM1SEL` writer - LPTIM1 clock source selection
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
///Field `LPTIM2SEL` reader - LPTIM2 clock source selection
pub struct LPTIM2SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM2SEL` writer - LPTIM2 clock source selection
pub struct LPTIM2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
///Field `TIM1SEL` reader - TIM1 clock source selection
pub struct TIM1SEL_R(crate::FieldReader<bool, bool>);
impl TIM1SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM1SEL` writer - TIM1 clock source selection
pub struct TIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
///Field `TIM15SEL` reader - TIM15 clock source selection
pub struct TIM15SEL_R(crate::FieldReader<bool, bool>);
impl TIM15SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM15SEL` writer - TIM15 clock source selection
pub struct TIM15SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
///Field `RNGSEL` reader - RNG clock source selection
pub struct RNGSEL_R(crate::FieldReader<u8, u8>);
impl RNGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNGSEL` writer - RNG clock source selection
pub struct RNGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
///Field `RNGDIV` reader - Division factor of RNG clock divider
pub struct RNGDIV_R(crate::FieldReader<u8, u8>);
impl RNGDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNGDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RNGDIV` writer - Division factor of RNG clock divider
pub struct RNGDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
///Field `ADCSEL` reader - ADCs clock source selection
pub struct ADCSEL_R(crate::FieldReader<u8, u8>);
impl ADCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ADCSEL` writer - ADCs clock source selection
pub struct ADCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEL_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    ///Bits 4:5 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    ///Bit 6 - HDMI CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bits 8:9 - LPUART2 clock source selection
    #[inline(always)]
    pub fn lpuart2sel(&self) -> LPUART2SEL_R {
        LPUART2SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    ///Bits 14:15 - I2S1 clock source selection
    #[inline(always)]
    pub fn i2s2sel(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    ///Bits 18:19 - LPTIM1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    ///Bits 20:21 - LPTIM2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    ///Bit 22 - TIM1 clock source selection
    #[inline(always)]
    pub fn tim1sel(&self) -> TIM1SEL_R {
        TIM1SEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 24 - TIM15 clock source selection
    #[inline(always)]
    pub fn tim15sel(&self) -> TIM15SEL_R {
        TIM15SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bits 26:27 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    ///Bits 28:29 - Division factor of RNG clock divider
    #[inline(always)]
    pub fn rngdiv(&self) -> RNGDIV_R {
        RNGDIV_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    ///Bits 30:31 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    ///Bits 4:5 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W {
        USART3SEL_W { w: self }
    }
    ///Bit 6 - HDMI CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W { w: self }
    }
    ///Bits 8:9 - LPUART2 clock source selection
    #[inline(always)]
    pub fn lpuart2sel(&mut self) -> LPUART2SEL_W {
        LPUART2SEL_W { w: self }
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    ///Bits 14:15 - I2S1 clock source selection
    #[inline(always)]
    pub fn i2s2sel(&mut self) -> I2S2SEL_W {
        I2S2SEL_W { w: self }
    }
    ///Bits 18:19 - LPTIM1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    ///Bits 20:21 - LPTIM2 clock source selection
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W {
        LPTIM2SEL_W { w: self }
    }
    ///Bit 22 - TIM1 clock source selection
    #[inline(always)]
    pub fn tim1sel(&mut self) -> TIM1SEL_W {
        TIM1SEL_W { w: self }
    }
    ///Bit 24 - TIM15 clock source selection
    #[inline(always)]
    pub fn tim15sel(&mut self) -> TIM15SEL_W {
        TIM15SEL_W { w: self }
    }
    ///Bits 26:27 - RNG clock source selection
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W {
        RNGSEL_W { w: self }
    }
    ///Bits 28:29 - Division factor of RNG clock divider
    #[inline(always)]
    pub fn rngdiv(&mut self) -> RNGDIV_W {
        RNGDIV_W { w: self }
    }
    ///Bits 30:31 - ADCs clock source selection
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Peripherals independent clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr](index.html) module
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr::R](R) reader structure
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr::W](W) writer structure
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
