///Register `TXEFA` reader
pub struct R(crate::R<TXEFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXEFA` writer
pub struct W(crate::W<TXEFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEFA_SPEC>;
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
impl From<crate::W<TXEFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEFA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EFAI` reader - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\]
///to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
pub struct EFAI_R(crate::FieldReader<u8, u8>);
impl EFAI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFAI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFAI` writer - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\]
///to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
pub struct EFAI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFAI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    ///Bits 0:1 - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\]
    ///to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    ///Bits 0:1 - Event FIFO acknowledge index After the Host has read an element or a sequence of elements from the Tx event FIFO, it has to write the index of the last element read from Tx event FIFO to EFAI. This sets the Tx event FIFO get index TXEFS\[EFGI\]
    ///to EFAI + 1 and updates the FIFO 0 fill level TXEFS\[EFFL\].
    #[inline(always)]
    pub fn efai(&mut self) -> EFAI_W {
        EFAI_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx event FIFO acknowledge register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txefa](index.html) module
pub struct TXEFA_SPEC;
impl crate::RegisterSpec for TXEFA_SPEC {
    type Ux = u32;
}
///`read()` method returns [txefa::R](R) reader structure
impl crate::Readable for TXEFA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txefa::W](W) writer structure
impl crate::Writable for TXEFA_SPEC {
    type Writer = W;
}
///`reset()` method sets TXEFA to value 0
impl crate::Resettable for TXEFA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
