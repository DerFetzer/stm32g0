///Register `PUCRD` reader
pub struct R(crate::R<PUCRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUCRD` writer
pub struct W(crate::W<PUCRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRD_SPEC>;
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
impl From<crate::W<PUCRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PU9` reader - Port D pull-up bit y (y=0..15)
pub struct PU9_R(crate::FieldReader<bool, bool>);
impl PU9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU9` writer - Port D pull-up bit y (y=0..15)
pub struct PU9_W<'a> {
    w: &'a mut W,
}
impl<'a> PU9_W<'a> {
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
///Field `PU8` reader - Port D pull-up bit y (y=0..15)
pub struct PU8_R(crate::FieldReader<bool, bool>);
impl PU8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU8` writer - Port D pull-up bit y (y=0..15)
pub struct PU8_W<'a> {
    w: &'a mut W,
}
impl<'a> PU8_W<'a> {
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
///Field `PU6` reader - Port D pull-up bit y (y=0..15)
pub struct PU6_R(crate::FieldReader<bool, bool>);
impl PU6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU6` writer - Port D pull-up bit y (y=0..15)
pub struct PU6_W<'a> {
    w: &'a mut W,
}
impl<'a> PU6_W<'a> {
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
///Field `PU5` reader - Port D pull-up bit y (y=0..15)
pub struct PU5_R(crate::FieldReader<bool, bool>);
impl PU5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU5` writer - Port D pull-up bit y (y=0..15)
pub struct PU5_W<'a> {
    w: &'a mut W,
}
impl<'a> PU5_W<'a> {
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
///Field `PU4` reader - Port D pull-up bit y (y=0..15)
pub struct PU4_R(crate::FieldReader<bool, bool>);
impl PU4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU4` writer - Port D pull-up bit y (y=0..15)
pub struct PU4_W<'a> {
    w: &'a mut W,
}
impl<'a> PU4_W<'a> {
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
///Field `PU3` reader - Port D pull-up bit y (y=0..15)
pub struct PU3_R(crate::FieldReader<bool, bool>);
impl PU3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU3` writer - Port D pull-up bit y (y=0..15)
pub struct PU3_W<'a> {
    w: &'a mut W,
}
impl<'a> PU3_W<'a> {
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
///Field `PU2` reader - Port D pull-up bit y (y=0..15)
pub struct PU2_R(crate::FieldReader<bool, bool>);
impl PU2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU2` writer - Port D pull-up bit y (y=0..15)
pub struct PU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PU2_W<'a> {
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
///Field `PU1` reader - Port D pull-up bit y (y=0..15)
pub struct PU1_R(crate::FieldReader<bool, bool>);
impl PU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU1` writer - Port D pull-up bit y (y=0..15)
pub struct PU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PU1_W<'a> {
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
///Field `PU0` reader - Port D pull-up bit y (y=0..15)
pub struct PU0_R(crate::FieldReader<bool, bool>);
impl PU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PU0` writer - Port D pull-up bit y (y=0..15)
pub struct PU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PU0_W<'a> {
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
impl R {
    ///Bit 9 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 6 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 5 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 4 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 3 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 2 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 1 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 0 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 9 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W {
        PU9_W { w: self }
    }
    ///Bit 8 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W {
        PU8_W { w: self }
    }
    ///Bit 6 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W {
        PU6_W { w: self }
    }
    ///Bit 5 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W {
        PU5_W { w: self }
    }
    ///Bit 4 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W {
        PU4_W { w: self }
    }
    ///Bit 3 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W {
        PU3_W { w: self }
    }
    ///Bit 2 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W {
        PU2_W { w: self }
    }
    ///Bit 1 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W {
        PU1_W { w: self }
    }
    ///Bit 0 - Port D pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W {
        PU0_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port D pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrd](index.html) module
pub struct PUCRD_SPEC;
impl crate::RegisterSpec for PUCRD_SPEC {
    type Ux = u32;
}
///`read()` method returns [pucrd::R](R) reader structure
impl crate::Readable for PUCRD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pucrd::W](W) writer structure
impl crate::Writable for PUCRD_SPEC {
    type Writer = W;
}
///`reset()` method sets PUCRD to value 0
impl crate::Resettable for PUCRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
