///Register `APBENR1` reader
pub struct R(crate::R<APBENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBENR1` writer
pub struct W(crate::W<APBENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBENR1_SPEC>;
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
impl From<crate::W<APBENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2EN` reader - TIM2 timer clock enable
pub struct TIM2EN_R(crate::FieldReader<bool, bool>);
impl TIM2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM2EN` writer - TIM2 timer clock enable
pub struct TIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2EN_W<'a> {
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
///Field `TIM3EN` reader - TIM3 timer clock enable
pub struct TIM3EN_R(crate::FieldReader<bool, bool>);
impl TIM3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM3EN` writer - TIM3 timer clock enable
pub struct TIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3EN_W<'a> {
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
///Field `TIM4EN` reader - TIM4 timer clock enable
pub struct TIM4EN_R(crate::FieldReader<bool, bool>);
impl TIM4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM4EN` writer - TIM4 timer clock enable
pub struct TIM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Field `TIM6EN` reader - TIM6 timer clock enable
pub struct TIM6EN_R(crate::FieldReader<bool, bool>);
impl TIM6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM6EN` writer - TIM6 timer clock enable
pub struct TIM6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6EN_W<'a> {
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
///Field `TIM7EN` reader - TIM7 timer clock enable
pub struct TIM7EN_R(crate::FieldReader<bool, bool>);
impl TIM7EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM7EN` writer - TIM7 timer clock enable
pub struct TIM7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7EN_W<'a> {
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
///Field `LPUART2EN` reader - LPUART2 clock enable
pub struct LPUART2EN_R(crate::FieldReader<bool, bool>);
impl LPUART2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART2EN` writer - LPUART2 clock enable
pub struct LPUART2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2EN_W<'a> {
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
///Field `USART5EN` reader - USART5EN
pub struct USART5EN_R(crate::FieldReader<bool, bool>);
impl USART5EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART5EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART5EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART5EN` writer - USART5EN
pub struct USART5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART5EN_W<'a> {
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
///Field `USART6EN` reader - USART6EN
pub struct USART6EN_R(crate::FieldReader<bool, bool>);
impl USART6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART6EN` writer - USART6EN
pub struct USART6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6EN_W<'a> {
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
///Field `RTCAPBEN` reader - RTC APB clock enable
pub struct RTCAPBEN_R(crate::FieldReader<bool, bool>);
impl RTCAPBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAPBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAPBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTCAPBEN` writer - RTC APB clock enable
pub struct RTCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Field `WWDGEN` reader - WWDG clock enable
pub struct WWDGEN_R(crate::FieldReader<bool, bool>);
impl WWDGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WWDGEN` writer - WWDG clock enable
pub struct WWDGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
///Field `FDCANEN` reader - USBEN
pub struct FDCANEN_R(crate::FieldReader<bool, bool>);
impl FDCANEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCANEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FDCANEN` writer - USBEN
pub struct FDCANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANEN_W<'a> {
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
///Field `USBEN` reader - USBEN
pub struct USBEN_R(crate::FieldReader<bool, bool>);
impl USBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USBEN` writer - USBEN
pub struct USBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_W<'a> {
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
///Field `SPI2EN` reader - SPI2 clock enable
pub struct SPI2EN_R(crate::FieldReader<bool, bool>);
impl SPI2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPI2EN` writer - SPI2 clock enable
pub struct SPI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2EN_W<'a> {
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
///Field `SPI3EN` reader - SPI3 clock enable
pub struct SPI3EN_R(crate::FieldReader<bool, bool>);
impl SPI3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPI3EN` writer - SPI3 clock enable
pub struct SPI3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
///Field `CRSEN` reader - CRSEN
pub struct CRSEN_R(crate::FieldReader<bool, bool>);
impl CRSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRSEN` writer - CRSEN
pub struct CRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSEN_W<'a> {
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
///Field `USART2EN` reader - USART2 clock enable
pub struct USART2EN_R(crate::FieldReader<bool, bool>);
impl USART2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART2EN` writer - USART2 clock enable
pub struct USART2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2EN_W<'a> {
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
///Field `USART3EN` reader - USART3 clock enable
pub struct USART3EN_R(crate::FieldReader<bool, bool>);
impl USART3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART3EN` writer - USART3 clock enable
pub struct USART3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3EN_W<'a> {
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
///Field `USART4EN` reader - USART4 clock enable
pub struct USART4EN_R(crate::FieldReader<bool, bool>);
impl USART4EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART4EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART4EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART4EN` writer - USART4 clock enable
pub struct USART4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART4EN_W<'a> {
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
///Field `LPUART1EN` reader - LPUART1 clock enable
pub struct LPUART1EN_R(crate::FieldReader<bool, bool>);
impl LPUART1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1EN` writer - LPUART1 clock enable
pub struct LPUART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1EN_W<'a> {
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
///Field `I2C1EN` reader - I2C1 clock enable
pub struct I2C1EN_R(crate::FieldReader<bool, bool>);
impl I2C1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1EN` writer - I2C1 clock enable
pub struct I2C1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1EN_W<'a> {
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
///Field `I2C2EN` reader - I2C2 clock enable
pub struct I2C2EN_R(crate::FieldReader<bool, bool>);
impl I2C2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C2EN` writer - I2C2 clock enable
pub struct I2C2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2EN_W<'a> {
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
///Field `I2C3EN` reader - I2C3 clock enable
pub struct I2C3EN_R(crate::FieldReader<bool, bool>);
impl I2C3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C3EN` writer - I2C3 clock enable
pub struct I2C3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
///Field `CECEN` reader - HDMI CEC clock enable
pub struct CECEN_R(crate::FieldReader<bool, bool>);
impl CECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CECEN` writer - HDMI CEC clock enable
pub struct CECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECEN_W<'a> {
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
///Field `UCPD1EN` reader - UCPD1 clock enable
pub struct UCPD1EN_R(crate::FieldReader<bool, bool>);
impl UCPD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPD1EN` writer - UCPD1 clock enable
pub struct UCPD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1EN_W<'a> {
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
///Field `UCPD2EN` reader - UCPD2 clock enable
pub struct UCPD2EN_R(crate::FieldReader<bool, bool>);
impl UCPD2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPD2EN` writer - UCPD2 clock enable
pub struct UCPD2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD2EN_W<'a> {
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
///Field `DBGEN` reader - Debug support clock enable
pub struct DBGEN_R(crate::FieldReader<bool, bool>);
impl DBGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBGEN` writer - Debug support clock enable
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
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
///Field `PWREN` reader - Power interface clock enable
pub struct PWREN_R(crate::FieldReader<bool, bool>);
impl PWREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PWREN` writer - Power interface clock enable
pub struct PWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWREN_W<'a> {
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
///Field `DAC1EN` reader - DAC1 interface clock enable
pub struct DAC1EN_R(crate::FieldReader<bool, bool>);
impl DAC1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DAC1EN` writer - DAC1 interface clock enable
pub struct DAC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1EN_W<'a> {
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
///Field `LPTIM2EN` reader - LPTIM2 clock enable
pub struct LPTIM2EN_R(crate::FieldReader<bool, bool>);
impl LPTIM2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM2EN` writer - LPTIM2 clock enable
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
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
///Field `LPTIM1EN` reader - LPTIM1 clock enable
pub struct LPTIM1EN_R(crate::FieldReader<bool, bool>);
impl LPTIM1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM1EN` writer - LPTIM1 clock enable
pub struct LPTIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1EN_W<'a> {
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
    ///Bit 0 - TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TIM4 timer clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - LPUART2 clock enable
    #[inline(always)]
    pub fn lpuart2en(&self) -> LPUART2EN_R {
        LPUART2EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - USART5EN
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - USART6EN
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - WWDG clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - USBEN
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - USBEN
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - CRSEN
    #[inline(always)]
    pub fn crsen(&self) -> CRSEN_R {
        CRSEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - USART4 clock enable
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - LPUART1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - HDMI CEC clock enable
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - UCPD1 clock enable
    #[inline(always)]
    pub fn ucpd1en(&self) -> UCPD1EN_R {
        UCPD1EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - UCPD2 clock enable
    #[inline(always)]
    pub fn ucpd2en(&self) -> UCPD2EN_R {
        UCPD2EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Debug support clock enable
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - DAC1 interface clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - LPTIM2 clock enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - LPTIM1 clock enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W {
        TIM2EN_W { w: self }
    }
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W {
        TIM3EN_W { w: self }
    }
    ///Bit 2 - TIM4 timer clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W {
        TIM4EN_W { w: self }
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W {
        TIM6EN_W { w: self }
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W {
        TIM7EN_W { w: self }
    }
    ///Bit 7 - LPUART2 clock enable
    #[inline(always)]
    pub fn lpuart2en(&mut self) -> LPUART2EN_W {
        LPUART2EN_W { w: self }
    }
    ///Bit 8 - USART5EN
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W {
        USART5EN_W { w: self }
    }
    ///Bit 9 - USART6EN
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W {
        USART6EN_W { w: self }
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W {
        RTCAPBEN_W { w: self }
    }
    ///Bit 11 - WWDG clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W {
        WWDGEN_W { w: self }
    }
    ///Bit 12 - USBEN
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W {
        FDCANEN_W { w: self }
    }
    ///Bit 13 - USBEN
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W {
        USBEN_W { w: self }
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W {
        SPI2EN_W { w: self }
    }
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W {
        SPI3EN_W { w: self }
    }
    ///Bit 16 - CRSEN
    #[inline(always)]
    pub fn crsen(&mut self) -> CRSEN_W {
        CRSEN_W { w: self }
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W {
        USART2EN_W { w: self }
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W {
        USART3EN_W { w: self }
    }
    ///Bit 19 - USART4 clock enable
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W {
        USART4EN_W { w: self }
    }
    ///Bit 20 - LPUART1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W {
        LPUART1EN_W { w: self }
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W { w: self }
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W {
        I2C2EN_W { w: self }
    }
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W {
        I2C3EN_W { w: self }
    }
    ///Bit 24 - HDMI CEC clock enable
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W {
        CECEN_W { w: self }
    }
    ///Bit 25 - UCPD1 clock enable
    #[inline(always)]
    pub fn ucpd1en(&mut self) -> UCPD1EN_W {
        UCPD1EN_W { w: self }
    }
    ///Bit 26 - UCPD2 clock enable
    #[inline(always)]
    pub fn ucpd2en(&mut self) -> UCPD2EN_W {
        UCPD2EN_W { w: self }
    }
    ///Bit 27 - Debug support clock enable
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W {
        PWREN_W { w: self }
    }
    ///Bit 29 - DAC1 interface clock enable
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W {
        DAC1EN_W { w: self }
    }
    ///Bit 30 - LPTIM2 clock enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    ///Bit 31 - LPTIM1 clock enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W {
        LPTIM1EN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB peripheral clock enable register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbenr1](index.html) module
pub struct APBENR1_SPEC;
impl crate::RegisterSpec for APBENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbenr1::R](R) reader structure
impl crate::Readable for APBENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbenr1::W](W) writer structure
impl crate::Writable for APBENR1_SPEC {
    type Writer = W;
}
///`reset()` method sets APBENR1 to value 0
impl crate::Resettable for APBENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
