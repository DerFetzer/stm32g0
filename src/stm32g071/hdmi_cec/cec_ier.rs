///Register `CEC_IER` reader
pub struct R(crate::R<CEC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CEC_IER` writer
pub struct W(crate::W<CEC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_IER_SPEC>;
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
impl From<crate::W<CEC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXBRIE` reader - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
pub struct RXBRIE_R(crate::FieldReader<bool, bool>);
impl RXBRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXBRIE` writer - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
pub struct RXBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRIE_W<'a> {
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
///Field `RXENDIE` reader - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
pub struct RXENDIE_R(crate::FieldReader<bool, bool>);
impl RXENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXENDIE` writer - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
pub struct RXENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENDIE_W<'a> {
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
///Field `RXOVRIE` reader - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
pub struct RXOVRIE_R(crate::FieldReader<bool, bool>);
impl RXOVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXOVRIE` writer - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
pub struct RXOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRIE_W<'a> {
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
///Field `BREIE` reader - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
pub struct BREIE_R(crate::FieldReader<bool, bool>);
impl BREIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BREIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BREIE` writer - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
pub struct BREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BREIE_W<'a> {
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
///Field `SBPEIE` reader - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
pub struct SBPEIE_R(crate::FieldReader<bool, bool>);
impl SBPEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBPEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBPEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SBPEIE` writer - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
pub struct SBPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBPEIE_W<'a> {
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
///Field `LBPEIE` reader - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
pub struct LBPEIE_R(crate::FieldReader<bool, bool>);
impl LBPEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBPEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LBPEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LBPEIE` writer - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
pub struct LBPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBPEIE_W<'a> {
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
///Field `RXACKIE` reader - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
pub struct RXACKIE_R(crate::FieldReader<bool, bool>);
impl RXACKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXACKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXACKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXACKIE` writer - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
pub struct RXACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXACKIE_W<'a> {
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
///Field `ARBLSTIE` reader - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
pub struct ARBLSTIE_R(crate::FieldReader<bool, bool>);
impl ARBLSTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBLSTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBLSTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ARBLSTIE` writer - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
pub struct ARBLSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBLSTIE_W<'a> {
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
///Field `TXBRIE` reader - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
pub struct TXBRIE_R(crate::FieldReader<bool, bool>);
impl TXBRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXBRIE` writer - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
pub struct TXBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBRIE_W<'a> {
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
///Field `TXENDIE` reader - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
pub struct TXENDIE_R(crate::FieldReader<bool, bool>);
impl TXENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXENDIE` writer - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
pub struct TXENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXENDIE_W<'a> {
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
///Field `TXUDRIE` reader - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
pub struct TXUDRIE_R(crate::FieldReader<bool, bool>);
impl TXUDRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUDRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUDRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXUDRIE` writer - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
pub struct TXUDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUDRIE_W<'a> {
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
///Field `TXERRIE` reader - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
pub struct TXERRIE_R(crate::FieldReader<bool, bool>);
impl TXERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXERRIE` writer - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
pub struct TXERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRIE_W<'a> {
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
///Field `TXACKIE` reader - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
pub struct TXACKIE_R(crate::FieldReader<bool, bool>);
impl TXACKIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACKIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXACKIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXACKIE` writer - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
pub struct TXACKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKIE_W<'a> {
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
impl R {
    ///Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RXBRIE_W {
        RXBRIE_W { w: self }
    }
    ///Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxendie(&mut self) -> RXENDIE_W {
        RXENDIE_W { w: self }
    }
    ///Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W {
        RXOVRIE_W { w: self }
    }
    ///Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
    #[inline(always)]
    pub fn breie(&mut self) -> BREIE_W {
        BREIE_W { w: self }
    }
    ///Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SBPEIE_W {
        SBPEIE_W { w: self }
    }
    ///Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LBPEIE_W {
        LBPEIE_W { w: self }
    }
    ///Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxackie(&mut self) -> RXACKIE_W {
        RXACKIE_W { w: self }
    }
    ///Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
    #[inline(always)]
    pub fn arblstie(&mut self) -> ARBLSTIE_W {
        ARBLSTIE_W { w: self }
    }
    ///Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txbrie(&mut self) -> TXBRIE_W {
        TXBRIE_W { w: self }
    }
    ///Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W {
        TXENDIE_W { w: self }
    }
    ///Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txudrie(&mut self) -> TXUDRIE_W {
        TXUDRIE_W { w: self }
    }
    ///Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txerrie(&mut self) -> TXERRIE_W {
        TXERRIE_W { w: self }
    }
    ///Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txackie(&mut self) -> TXACKIE_W {
        TXACKIE_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CEC interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cec_ier](index.html) module
pub struct CEC_IER_SPEC;
impl crate::RegisterSpec for CEC_IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [cec_ier::R](R) reader structure
impl crate::Readable for CEC_IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cec_ier::W](W) writer structure
impl crate::Writable for CEC_IER_SPEC {
    type Writer = W;
}
///`reset()` method sets CEC_IER to value 0
impl crate::Resettable for CEC_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
