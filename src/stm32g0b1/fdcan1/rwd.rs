///Register `RWD` reader
pub struct R(crate::R<RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RWD` writer
pub struct W(crate::W<RWD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWD_SPEC>;
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
impl From<crate::W<RWD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WDC` reader - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of FDCAN_CCCR register are set to 1.
pub struct WDC_R(crate::FieldReader<u8, u8>);
impl WDC_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WDC` writer - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of FDCAN_CCCR register are set to 1.
pub struct WDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
///Field `WDV` reader - Watchdog value Actual message RAM watchdog counter value.
pub struct WDV_R(crate::FieldReader<u8, u8>);
impl WDV_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of FDCAN_CCCR register are set to 1.
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Watchdog value Actual message RAM watchdog counter value.
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of FDCAN_CCCR register are set to 1.
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W {
        WDC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN RAM watchdog register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rwd](index.html) module
pub struct RWD_SPEC;
impl crate::RegisterSpec for RWD_SPEC {
    type Ux = u32;
}
///`read()` method returns [rwd::R](R) reader structure
impl crate::Readable for RWD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rwd::W](W) writer structure
impl crate::Writable for RWD_SPEC {
    type Writer = W;
}
///`reset()` method sets RWD to value 0
impl crate::Resettable for RWD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
