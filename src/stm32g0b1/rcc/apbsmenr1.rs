///Register `APBSMENR1` reader
pub struct R(crate::R<APBSMENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBSMENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBSMENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBSMENR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APBSMENR1` writer
pub struct W(crate::W<APBSMENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBSMENR1_SPEC>;
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
impl From<crate::W<APBSMENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBSMENR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM2SMEN` reader - TIM2 timer clock enable during Sleep mode
pub struct TIM2SMEN_R(crate::FieldReader<bool, bool>);
impl TIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM2SMEN` writer - TIM2 timer clock enable during Sleep mode
pub struct TIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2SMEN_W<'a> {
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
///Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode
pub struct TIM3SMEN_R(crate::FieldReader<bool, bool>);
impl TIM3SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode
pub struct TIM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3SMEN_W<'a> {
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
///Field `TIM4SMEN` reader - TIM4 timer clock enable during Sleep mode
pub struct TIM4SMEN_R(crate::FieldReader<bool, bool>);
impl TIM4SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM4SMEN` writer - TIM4 timer clock enable during Sleep mode
pub struct TIM4SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4SMEN_W<'a> {
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
///Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode
pub struct TIM6SMEN_R(crate::FieldReader<bool, bool>);
impl TIM6SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode
pub struct TIM6SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6SMEN_W<'a> {
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
///Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode
pub struct TIM7SMEN_R(crate::FieldReader<bool, bool>);
impl TIM7SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode
pub struct TIM7SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7SMEN_W<'a> {
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
///Field `LPUART2SMEN` reader - LPUART2 clock enable
pub struct LPUART2SMEN_R(crate::FieldReader<bool, bool>);
impl LPUART2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART2SMEN` writer - LPUART2 clock enable
pub struct LPUART2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART2SMEN_W<'a> {
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
///Field `USART5SMEN` reader - USART5 clock enable
pub struct USART5SMEN_R(crate::FieldReader<bool, bool>);
impl USART5SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART5SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART5SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART5SMEN` writer - USART5 clock enable
pub struct USART5SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART5SMEN_W<'a> {
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
///Field `USART6SMEN` reader - USART6 clock enable
pub struct USART6SMEN_R(crate::FieldReader<bool, bool>);
impl USART6SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART6SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART6SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART6SMEN` writer - USART6 clock enable
pub struct USART6SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6SMEN_W<'a> {
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
///Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode
pub struct RTCAPBSMEN_R(crate::FieldReader<bool, bool>);
impl RTCAPBSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCAPBSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCAPBSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode
pub struct RTCAPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBSMEN_W<'a> {
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
///Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode
pub struct WWDGSMEN_R(crate::FieldReader<bool, bool>);
impl WWDGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWDGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode
pub struct WWDGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGSMEN_W<'a> {
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
///Field `FDCANSMEN` reader - FDCAN clock enable during Sleep mode
pub struct FDCANSMEN_R(crate::FieldReader<bool, bool>);
impl FDCANSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDCANSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDCANSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FDCANSMEN` writer - FDCAN clock enable during Sleep mode
pub struct FDCANSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSMEN_W<'a> {
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
///Field `USBSMEN` reader - USB clock enable during Sleep mode
pub struct USBSMEN_R(crate::FieldReader<bool, bool>);
impl USBSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USBSMEN` writer - USB clock enable during Sleep mode
pub struct USBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSMEN_W<'a> {
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
///Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode
pub struct SPI2SMEN_R(crate::FieldReader<bool, bool>);
impl SPI2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode
pub struct SPI2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2SMEN_W<'a> {
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
///Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode
pub struct SPI3SMEN_R(crate::FieldReader<bool, bool>);
impl SPI3SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode
pub struct SPI3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3SMEN_W<'a> {
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
///Field `CRSSSMEN` reader - CRSS clock enable during Sleep mode
pub struct CRSSSMEN_R(crate::FieldReader<bool, bool>);
impl CRSSSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSSSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSSSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CRSSSMEN` writer - CRSS clock enable during Sleep mode
pub struct CRSSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSSSMEN_W<'a> {
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
///Field `USART2SMEN` reader - USART2 clock enable during Sleep mode
pub struct USART2SMEN_R(crate::FieldReader<bool, bool>);
impl USART2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART2SMEN` writer - USART2 clock enable during Sleep mode
pub struct USART2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SMEN_W<'a> {
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
///Field `USART3SMEN` reader - USART3 clock enable during Sleep mode
pub struct USART3SMEN_R(crate::FieldReader<bool, bool>);
impl USART3SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART3SMEN` writer - USART3 clock enable during Sleep mode
pub struct USART3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SMEN_W<'a> {
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
///Field `USART4SMEN` reader - USART4 clock enable during Sleep mode
pub struct USART4SMEN_R(crate::FieldReader<bool, bool>);
impl USART4SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART4SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART4SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `USART4SMEN` writer - USART4 clock enable during Sleep mode
pub struct USART4SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART4SMEN_W<'a> {
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
///Field `LPUART1SMEN` reader - LPUART1 clock enable during Sleep mode
pub struct LPUART1SMEN_R(crate::FieldReader<bool, bool>);
impl LPUART1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPUART1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPUART1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPUART1SMEN` writer - LPUART1 clock enable during Sleep mode
pub struct LPUART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SMEN_W<'a> {
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
///Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode
pub struct I2C1SMEN_R(crate::FieldReader<bool, bool>);
impl I2C1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode
pub struct I2C1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SMEN_W<'a> {
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
///Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode
pub struct I2C2SMEN_R(crate::FieldReader<bool, bool>);
impl I2C2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode
pub struct I2C2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SMEN_W<'a> {
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
///Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode
pub struct I2C3SMEN_R(crate::FieldReader<bool, bool>);
impl I2C3SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode
pub struct I2C3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SMEN_W<'a> {
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
///Field `CECSMEN` reader - HDMI CEC clock enable during Sleep mode
pub struct CECSMEN_R(crate::FieldReader<bool, bool>);
impl CECSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CECSMEN` writer - HDMI CEC clock enable during Sleep mode
pub struct CECSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSMEN_W<'a> {
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
///Field `UCPD1SMEN` reader - UCPD1 clock enable during Sleep mode
pub struct UCPD1SMEN_R(crate::FieldReader<bool, bool>);
impl UCPD1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPD1SMEN` writer - UCPD1 clock enable during Sleep mode
pub struct UCPD1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1SMEN_W<'a> {
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
///Field `UCPD2SMEN` reader - UCPD2 clock enable during Sleep mode
pub struct UCPD2SMEN_R(crate::FieldReader<bool, bool>);
impl UCPD2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPD2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPD2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPD2SMEN` writer - UCPD2 clock enable during Sleep mode
pub struct UCPD2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD2SMEN_W<'a> {
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
///Field `DBGSMEN` reader - Debug support clock enable during Sleep mode
pub struct DBGSMEN_R(crate::FieldReader<bool, bool>);
impl DBGSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DBGSMEN` writer - Debug support clock enable during Sleep mode
pub struct DBGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGSMEN_W<'a> {
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
///Field `PWRSMEN` reader - Power interface clock enable during Sleep mode
pub struct PWRSMEN_R(crate::FieldReader<bool, bool>);
impl PWRSMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRSMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRSMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PWRSMEN` writer - Power interface clock enable during Sleep mode
pub struct PWRSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSMEN_W<'a> {
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
///Field `DAC1SMEN` reader - DAC1 interface clock enable during Sleep mode
pub struct DAC1SMEN_R(crate::FieldReader<bool, bool>);
impl DAC1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DAC1SMEN` writer - DAC1 interface clock enable during Sleep mode
pub struct DAC1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1SMEN_W<'a> {
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
///Field `LPTIM2SMEN` reader - Low Power Timer 2 clock enable during Sleep mode
pub struct LPTIM2SMEN_R(crate::FieldReader<bool, bool>);
impl LPTIM2SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM2SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM2SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM2SMEN` writer - Low Power Timer 2 clock enable during Sleep mode
pub struct LPTIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SMEN_W<'a> {
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
///Field `LPTIM1SMEN` reader - Low Power Timer 1 clock enable during Sleep mode
pub struct LPTIM1SMEN_R(crate::FieldReader<bool, bool>);
impl LPTIM1SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPTIM1SMEN` writer - Low Power Timer 1 clock enable during Sleep mode
pub struct LPTIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SMEN_W<'a> {
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
    ///Bit 0 - TIM2 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - TIM4 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 7 - LPUART2 clock enable
    #[inline(always)]
    pub fn lpuart2smen(&self) -> LPUART2SMEN_R {
        LPUART2SMEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - USART5 clock enable
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - USART6 clock enable
    #[inline(always)]
    pub fn usart6smen(&self) -> USART6SMEN_R {
        USART6SMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - WWDG clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - FDCAN clock enable during Sleep mode
    #[inline(always)]
    pub fn fdcansmen(&self) -> FDCANSMEN_R {
        FDCANSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - USB clock enable during Sleep mode
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 16 - CRSS clock enable during Sleep mode
    #[inline(always)]
    pub fn crsssmen(&self) -> CRSSSMEN_R {
        CRSSSMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - USART3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - USART4 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 20 - LPUART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - HDMI CEC clock enable during Sleep mode
    #[inline(always)]
    pub fn cecsmen(&self) -> CECSMEN_R {
        CECSMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - UCPD1 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    ///Bit 26 - UCPD2 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd2smen(&self) -> UCPD2SMEN_R {
        UCPD2SMEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    ///Bit 27 - Debug support clock enable during Sleep mode
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    ///Bit 29 - DAC1 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Low Power Timer 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - Low Power Timer 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W {
        TIM2SMEN_W { w: self }
    }
    ///Bit 1 - TIM3 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W {
        TIM3SMEN_W { w: self }
    }
    ///Bit 2 - TIM4 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W {
        TIM4SMEN_W { w: self }
    }
    ///Bit 4 - TIM6 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W {
        TIM6SMEN_W { w: self }
    }
    ///Bit 5 - TIM7 timer clock enable during Sleep mode
    #[inline(always)]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W {
        TIM7SMEN_W { w: self }
    }
    ///Bit 7 - LPUART2 clock enable
    #[inline(always)]
    pub fn lpuart2smen(&mut self) -> LPUART2SMEN_W {
        LPUART2SMEN_W { w: self }
    }
    ///Bit 8 - USART5 clock enable
    #[inline(always)]
    pub fn usart5smen(&mut self) -> USART5SMEN_W {
        USART5SMEN_W { w: self }
    }
    ///Bit 9 - USART6 clock enable
    #[inline(always)]
    pub fn usart6smen(&mut self) -> USART6SMEN_W {
        USART6SMEN_W { w: self }
    }
    ///Bit 10 - RTC APB clock enable during Sleep mode
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W {
        RTCAPBSMEN_W { w: self }
    }
    ///Bit 11 - WWDG clock enable during Sleep mode
    #[inline(always)]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W {
        WWDGSMEN_W { w: self }
    }
    ///Bit 12 - FDCAN clock enable during Sleep mode
    #[inline(always)]
    pub fn fdcansmen(&mut self) -> FDCANSMEN_W {
        FDCANSMEN_W { w: self }
    }
    ///Bit 13 - USB clock enable during Sleep mode
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W {
        USBSMEN_W { w: self }
    }
    ///Bit 14 - SPI2 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W {
        SPI2SMEN_W { w: self }
    }
    ///Bit 15 - SPI3 clock enable during Sleep mode
    #[inline(always)]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W {
        SPI3SMEN_W { w: self }
    }
    ///Bit 16 - CRSS clock enable during Sleep mode
    #[inline(always)]
    pub fn crsssmen(&mut self) -> CRSSSMEN_W {
        CRSSSMEN_W { w: self }
    }
    ///Bit 17 - USART2 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart2smen(&mut self) -> USART2SMEN_W {
        USART2SMEN_W { w: self }
    }
    ///Bit 18 - USART3 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart3smen(&mut self) -> USART3SMEN_W {
        USART3SMEN_W { w: self }
    }
    ///Bit 19 - USART4 clock enable during Sleep mode
    #[inline(always)]
    pub fn usart4smen(&mut self) -> USART4SMEN_W {
        USART4SMEN_W { w: self }
    }
    ///Bit 20 - LPUART1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
    ///Bit 21 - I2C1 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W {
        I2C1SMEN_W { w: self }
    }
    ///Bit 22 - I2C2 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W {
        I2C2SMEN_W { w: self }
    }
    ///Bit 23 - I2C3 clock enable during Sleep mode
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W {
        I2C3SMEN_W { w: self }
    }
    ///Bit 24 - HDMI CEC clock enable during Sleep mode
    #[inline(always)]
    pub fn cecsmen(&mut self) -> CECSMEN_W {
        CECSMEN_W { w: self }
    }
    ///Bit 25 - UCPD1 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W {
        UCPD1SMEN_W { w: self }
    }
    ///Bit 26 - UCPD2 clock enable during Sleep mode
    #[inline(always)]
    pub fn ucpd2smen(&mut self) -> UCPD2SMEN_W {
        UCPD2SMEN_W { w: self }
    }
    ///Bit 27 - Debug support clock enable during Sleep mode
    #[inline(always)]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W {
        DBGSMEN_W { w: self }
    }
    ///Bit 28 - Power interface clock enable during Sleep mode
    #[inline(always)]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W {
        PWRSMEN_W { w: self }
    }
    ///Bit 29 - DAC1 interface clock enable during Sleep mode
    #[inline(always)]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W {
        DAC1SMEN_W { w: self }
    }
    ///Bit 30 - Low Power Timer 2 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    ///Bit 31 - Low Power Timer 1 clock enable during Sleep mode
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W {
        LPTIM1SMEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB peripheral clock enable in Sleep mode register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apbsmenr1](index.html) module
pub struct APBSMENR1_SPEC;
impl crate::RegisterSpec for APBSMENR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [apbsmenr1::R](R) reader structure
impl crate::Readable for APBSMENR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apbsmenr1::W](W) writer structure
impl crate::Writable for APBSMENR1_SPEC {
    type Writer = W;
}
///`reset()` method sets APBSMENR1 to value 0xffff_ffb7
impl crate::Resettable for APBSMENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffb7
    }
}
