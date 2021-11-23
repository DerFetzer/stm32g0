///Register `TX_ORDSETR` reader
pub struct R(crate::R<TX_ORDSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ORDSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ORDSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ORDSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TX_ORDSETR` writer
pub struct W(crate::W<TX_ORDSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_ORDSETR_SPEC>;
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
impl From<crate::W<TX_ORDSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_ORDSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXORDSET` reader - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
pub struct TXORDSET_R(crate::FieldReader<u32, u32>);
impl TXORDSET_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXORDSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXORDSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXORDSET` writer - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
pub struct TXORDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXORDSET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    ///Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
    #[inline(always)]
    pub fn txordset(&mut self) -> TXORDSET_W {
        TXORDSET_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD Tx ordered set type register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_ordsetr](index.html) module
pub struct TX_ORDSETR_SPEC;
impl crate::RegisterSpec for TX_ORDSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tx_ordsetr::R](R) reader structure
impl crate::Readable for TX_ORDSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tx_ordsetr::W](W) writer structure
impl crate::Writable for TX_ORDSETR_SPEC {
    type Writer = W;
}
///`reset()` method sets TX_ORDSETR to value 0
impl crate::Resettable for TX_ORDSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
