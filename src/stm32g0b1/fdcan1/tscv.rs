///Register `TSCV` reader
pub struct R(crate::R<TSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TSCV` writer
pub struct W(crate::W<TSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCV_SPEC>;
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
impl From<crate::W<TSCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCV_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSC` reader - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
///= 01, the timestamp counter is incremented in multiples of CAN bit times \[1 â¦ 16\]
///depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.
pub struct TSC_R(crate::FieldReader<u16, u16>);
impl TSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSC` writer - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
///= 01, the timestamp counter is incremented in multiples of CAN bit times \[1 â¦ 16\]
///depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.
pub struct TSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    ///Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
    ///= 01, the timestamp counter is incremented in multiples of CAN bit times \[1 â¦ 16\]
    ///depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\[TSS\]
    ///= 01, the timestamp counter is incremented in multiples of CAN bit times \[1 â¦ 16\]
    ///depending on the configuration of TSCC\[TCP\]. A wrap around sets interrupt flag IR\[TSW\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact.
    #[inline(always)]
    pub fn tsc(&mut self) -> TSC_W {
        TSC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN timestamp counter value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tscv](index.html) module
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
///`read()` method returns [tscv::R](R) reader structure
impl crate::Readable for TSCV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tscv::W](W) writer structure
impl crate::Writable for TSCV_SPEC {
    type Writer = W;
}
///`reset()` method sets TSCV to value 0
impl crate::Resettable for TSCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
