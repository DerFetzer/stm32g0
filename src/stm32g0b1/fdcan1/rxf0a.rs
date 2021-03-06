///Register `RXF0A` reader
pub struct R(crate::R<RXF0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0A_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RXF0A` writer
pub struct W(crate::W<RXF0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0A_SPEC>;
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
impl From<crate::W<RXF0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0A_SPEC>) -> Self {
        W(writer)
    }
}
///Field `F0AI` reader - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFOÂ 0 get index RXF0S\[F0GI\]
///to F0AI + 1 and update the FIFO 0 fill level RXF0S\[F0FL\].
pub struct F0AI_R(crate::FieldReader<u8, u8>);
impl F0AI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0AI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0AI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `F0AI` writer - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFOÂ 0 get index RXF0S\[F0GI\]
///to F0AI + 1 and update the FIFO 0 fill level RXF0S\[F0FL\].
pub struct F0AI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0AI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFOÂ 0 get index RXF0S\[F0GI\]
    ///to F0AI + 1 and update the FIFO 0 fill level RXF0S\[F0FL\].
    #[inline(always)]
    pub fn f0ai(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Rx FIFO 0 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This sets the Rx FIFOÂ 0 get index RXF0S\[F0GI\]
    ///to F0AI + 1 and update the FIFO 0 fill level RXF0S\[F0FL\].
    #[inline(always)]
    pub fn f0ai(&mut self) -> F0AI_W {
        F0AI_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CAN Rx FIFO 0 acknowledge register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxf0a](index.html) module
pub struct RXF0A_SPEC;
impl crate::RegisterSpec for RXF0A_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxf0a::R](R) reader structure
impl crate::Readable for RXF0A_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rxf0a::W](W) writer structure
impl crate::Writable for RXF0A_SPEC {
    type Writer = W;
}
///`reset()` method sets RXF0A to value 0
impl crate::Resettable for RXF0A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
