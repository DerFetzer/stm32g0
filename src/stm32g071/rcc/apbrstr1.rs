///Register `APBRSTR1` reader
pub struct R(crate::R<APBRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBRSTR1` writer
pub struct W(crate::W<APBRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBRSTR1_SPEC>;
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
impl From<crate::W<APBRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2RST` reader - TIM2 timer reset
pub struct TIM2RST_R(crate::FieldReader<bool, bool>);
impl TIM2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM2RST` writer - TIM2 timer reset
pub struct TIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2RST_W<'a> {
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
///Field `TIM3RST` reader - TIM3 timer reset
pub struct TIM3RST_R(crate::FieldReader<bool, bool>);
impl TIM3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM3RST` writer - TIM3 timer reset
pub struct TIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3RST_W<'a> {
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
///Field `TIM6RST` reader - TIM6 timer reset
pub struct TIM6RST_R(crate::FieldReader<bool, bool>);
impl TIM6RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM6RST` writer - TIM6 timer reset
pub struct TIM6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6RST_W<'a> {
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
///Field `TIM7RST` reader - TIM7 timer reset
pub struct TIM7RST_R(crate::FieldReader<bool, bool>);
impl TIM7RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM7RST` writer - TIM7 timer reset
pub struct TIM7RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7RST_W<'a> {
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
///Field `SPI2RST` reader - SPI2 reset
pub struct SPI2RST_R(crate::FieldReader<bool, bool>);
impl SPI2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPI2RST` writer - SPI2 reset
pub struct SPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RST_W<'a> {
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
///Field `USART2RST` reader - USART2 reset
pub struct USART2RST_R(crate::FieldReader<bool, bool>);
impl USART2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART2RST` writer - USART2 reset
pub struct USART2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2RST_W<'a> {
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
///Field `USART3RST` reader - USART3 reset
pub struct USART3RST_R(crate::FieldReader<bool, bool>);
impl USART3RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART3RST` writer - USART3 reset
pub struct USART3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
///Field `USART4RST` reader - USART4 reset
pub struct USART4RST_R(crate::FieldReader<bool, bool>);
impl USART4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART4RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART4RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART4RST` writer - USART4 reset
pub struct USART4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART4RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
///Field `LPUART1RST` reader - LPUART1 reset
pub struct LPUART1RST_R(crate::FieldReader<bool, bool>);
impl LPUART1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1RST` writer - LPUART1 reset
pub struct LPUART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
///Field `I2C1RST` reader - I2C1 reset
pub struct I2C1RST_R(crate::FieldReader<bool, bool>);
impl I2C1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1RST` writer - I2C1 reset
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
///Field `I2C2RST` reader - I2C2 reset
pub struct I2C2RST_R(crate::FieldReader<bool, bool>);
impl I2C2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C2RST` writer - I2C2 reset
pub struct I2C2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2RST_W<'a> {
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
///Field `CECRST` reader - HDMI CEC reset
pub struct CECRST_R(crate::FieldReader<bool, bool>);
impl CECRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CECRST` writer - HDMI CEC reset
pub struct CECRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CECRST_W<'a> {
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
///Field `UCPD1RST` reader - UCPD1 reset
pub struct UCPD1RST_R(crate::FieldReader<bool, bool>);
impl UCPD1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPD1RST` writer - UCPD1 reset
pub struct UCPD1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
///Field `UCPD2RST` reader - UCPD2 reset
pub struct UCPD2RST_R(crate::FieldReader<bool, bool>);
impl UCPD2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPD2RST` writer - UCPD2 reset
pub struct UCPD2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD2RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
///Field `DBGRST` reader - Debug support reset
pub struct DBGRST_R(crate::FieldReader<bool, bool>);
impl DBGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBGRST` writer - Debug support reset
pub struct DBGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
///Field `PWRRST` reader - Power interface reset
pub struct PWRRST_R(crate::FieldReader<bool, bool>);
impl PWRRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PWRRST` writer - Power interface reset
pub struct PWRRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRRST_W<'a> {
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
///Field `DAC1RST` reader - DAC1 interface reset
pub struct DAC1RST_R(crate::FieldReader<bool, bool>);
impl DAC1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DAC1RST` writer - DAC1 interface reset
pub struct DAC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1RST_W<'a> {
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
///Field `LPTIM2RST` reader - Low Power Timer 2 reset
pub struct LPTIM2RST_R(crate::FieldReader<bool, bool>);
impl LPTIM2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM2RST` writer - Low Power Timer 2 reset
pub struct LPTIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2RST_W<'a> {
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
///Field `LPTIM1RST` reader - Low Power Timer 1 reset
pub struct LPTIM1RST_R(crate::FieldReader<bool, bool>);
impl LPTIM1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM1RST` writer - Low Power Timer 1 reset
pub struct LPTIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1RST_W<'a> {
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
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - USART4 reset
    #[inline(always)]
    pub fn usart4rst(&self) -> USART4RST_R {
        USART4RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 24 - HDMI CEC reset
    #[inline(always)]
    pub fn cecrst(&self) -> CECRST_R {
        CECRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rst(&self) -> UCPD1RST_R {
        UCPD1RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - UCPD2 reset
    #[inline(always)]
    pub fn ucpd2rst(&self) -> UCPD2RST_R {
        UCPD2RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Debug support reset
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - DAC1 interface reset
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Low Power Timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 timer reset
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W {
        TIM2RST_W { w: self }
    }
    ///Bit 1 - TIM3 timer reset
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W {
        TIM3RST_W { w: self }
    }
    ///Bit 4 - TIM6 timer reset
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W {
        TIM6RST_W { w: self }
    }
    ///Bit 5 - TIM7 timer reset
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W {
        TIM7RST_W { w: self }
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W { w: self }
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W {
        USART2RST_W { w: self }
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W {
        USART3RST_W { w: self }
    }
    ///Bit 19 - USART4 reset
    #[inline(always)]
    pub fn usart4rst(&mut self) -> USART4RST_W {
        USART4RST_W { w: self }
    }
    ///Bit 20 - LPUART1 reset
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W {
        LPUART1RST_W { w: self }
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W { w: self }
    }
    ///Bit 24 - HDMI CEC reset
    #[inline(always)]
    pub fn cecrst(&mut self) -> CECRST_W {
        CECRST_W { w: self }
    }
    ///Bit 25 - UCPD1 reset
    #[inline(always)]
    pub fn ucpd1rst(&mut self) -> UCPD1RST_W {
        UCPD1RST_W { w: self }
    }
    ///Bit 26 - UCPD2 reset
    #[inline(always)]
    pub fn ucpd2rst(&mut self) -> UCPD2RST_W {
        UCPD2RST_W { w: self }
    }
    ///Bit 27 - Debug support reset
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W {
        DBGRST_W { w: self }
    }
    ///Bit 28 - Power interface reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W {
        PWRRST_W { w: self }
    }
    ///Bit 29 - DAC1 interface reset
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W {
        DAC1RST_W { w: self }
    }
    ///Bit 30 - Low Power Timer 2 reset
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W {
        LPTIM2RST_W { w: self }
    }
    ///Bit 31 - Low Power Timer 1 reset
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W {
        LPTIM1RST_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB peripheral reset register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbrstr1](index.html) module
pub struct APBRSTR1_SPEC;
impl crate::RegisterSpec for APBRSTR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbrstr1::R](R) reader structure
impl crate::Readable for APBRSTR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbrstr1::W](W) writer structure
impl crate::Writable for APBRSTR1_SPEC {
    type Writer = W;
}
///`reset()` method sets APBRSTR1 to value 0
impl crate::Resettable for APBRSTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
