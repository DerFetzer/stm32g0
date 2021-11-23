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
///Field `HSION` reader - HSI16 clock enable
pub struct HSION_R(crate::FieldReader<bool, bool>);
impl HSION_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSION` writer - HSI16 clock enable
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
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
///Field `HSIKERON` reader - HSI16 always enable for peripheral kernels
pub struct HSIKERON_R(crate::FieldReader<bool, bool>);
impl HSIKERON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIKERON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIKERON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSIKERON` writer - HSI16 always enable for peripheral kernels
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
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
///Field `HSIRDY` reader - HSI16 clock ready flag
pub struct HSIRDY_R(crate::FieldReader<bool, bool>);
impl HSIRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSIDIV` reader - HSI16 clock division factor
pub struct HSIDIV_R(crate::FieldReader<u8, u8>);
impl HSIDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSIDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSIDIV` writer - HSI16 clock division factor
pub struct HSIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDIV_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
///Field `HSEON` reader - HSE clock enable
pub struct HSEON_R(crate::FieldReader<bool, bool>);
impl HSEON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSEON` writer - HSE clock enable
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
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
///Field `HSERDY` reader - HSE clock ready flag
pub struct HSERDY_R(crate::FieldReader<bool, bool>);
impl HSERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSERDY` writer - HSE clock ready flag
pub struct HSERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDY_W<'a> {
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
///Field `HSEBYP` reader - HSE crystal oscillator bypass
pub struct HSEBYP_R(crate::FieldReader<bool, bool>);
impl HSEBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSEBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSEBYP` writer - HSE crystal oscillator bypass
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
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
///Field `CSSON` reader - Clock security system enable
pub struct CSSON_R(crate::FieldReader<bool, bool>);
impl CSSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CSSON` writer - Clock security system enable
pub struct CSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSON_W<'a> {
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
///Field `HSI48ON` reader - HSI48ON
pub struct HSI48ON_R(crate::FieldReader<bool, bool>);
impl HSI48ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HSI48ON` writer - HSI48ON
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
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
///Field `HSI48RDY` reader - HSI48RDY
pub struct HSI48RDY_R(crate::FieldReader<bool, bool>);
impl HSI48RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PLLON` reader - PLL enable
pub struct PLLON_R(crate::FieldReader<bool, bool>);
impl PLLON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PLLON` writer - PLL enable
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
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
///Field `PLLRDY` reader - PLL clock ready flag
pub struct PLLRDY_R(crate::FieldReader<bool, bool>);
impl PLLRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bits 11:13 - HSI16 clock division factor
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    ///Bit 22 - HSI48ON
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    ///Bit 23 - HSI48RDY
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    ///Bit 9 - HSI16 always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    ///Bits 11:13 - HSI16 clock division factor
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W {
        HSIDIV_W { w: self }
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&mut self) -> HSERDY_W {
        HSERDY_W { w: self }
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W {
        CSSON_W { w: self }
    }
    ///Bit 22 - HSI48ON
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
    ///Bit 24 - PLL enable
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock control register
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
///`reset()` method sets CR to value 0x0500
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0500
    }
}
