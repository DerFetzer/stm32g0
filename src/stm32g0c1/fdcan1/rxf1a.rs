///Register `RXF1A` reader
pub struct R(crate::R<RXF1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF1A_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RXF1A` writer
pub struct W(crate::W<RXF1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF1A_SPEC>;
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
impl From<crate::W<RXF1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF1A_SPEC>) -> Self {
        W(writer)
    }
}
///Field `F1AI` reader - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFOÂ 1 get index RXF1S\[F1GI\]
///to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
pub struct F1AI_R(crate::FieldReader<u8, u8>);
impl F1AI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F1AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F1AI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F1AI` writer - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFOÂ 1 get index RXF1S\[F1GI\]
///to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
pub struct F1AI_W<'a> {
    w: &'a mut W,
}
impl<'a> F1AI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFOÂ 1 get index RXF1S\[F1GI\]
    ///to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFOÂ 1 get index RXF1S\[F1GI\]
    ///to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\[F1FL\].
    #[inline(always)]
    pub fn f1ai(&mut self) -> F1AI_W {
        F1AI_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Rx FIFO 1 acknowledge register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf1a](index.html) module
pub struct RXF1A_SPEC;
impl crate::RegisterSpec for RXF1A_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf1a::R](R) reader structure
impl crate::Readable for RXF1A_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rxf1a::W](W) writer structure
impl crate::Writable for RXF1A_SPEC {
    type Writer = W;
}
///`reset()` method sets RXF1A to value 0
impl crate::Resettable for RXF1A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
