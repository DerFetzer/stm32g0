///Register `SHIFTR` writer
pub struct W(crate::W<SHIFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTR_SPEC>;
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
impl From<crate::W<SHIFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADD1S` writer - Add one second
pub struct ADD1S_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1S_W<'a> {
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
///Field `SUBFS` writer - Subtract a fraction of a second
pub struct SUBFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBFS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl W {
    ///Bit 31 - Add one second
    #[inline(always)]
    pub fn add1s(&mut self) -> ADD1S_W {
        ADD1S_W { w: self }
    }
    ///Bits 0:14 - Subtract a fraction of a second
    #[inline(always)]
    pub fn subfs(&mut self) -> SUBFS_W {
        SUBFS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///shift control register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [shiftr](index.html) module
pub struct SHIFTR_SPEC;
impl crate::RegisterSpec for SHIFTR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [shiftr::W](W) writer structure
impl crate::Writable for SHIFTR_SPEC {
    type Writer = W;
}
///`reset()` method sets SHIFTR to value 0
impl crate::Resettable for SHIFTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
