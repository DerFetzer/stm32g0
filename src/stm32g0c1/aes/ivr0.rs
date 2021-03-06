///Register `IVR0` reader
pub struct R(crate::R<IVR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR0` writer
pub struct W(crate::W<IVR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR0_SPEC>;
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
impl From<crate::W<IVR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IVI` reader - Initialization vector input, bits \[31:0\]
///Refer to for description of the IVI\[127:0\]
///bitfield. The initialization vector is only used in chaining modes other than ECB. The AES_IVRx registers may be written only when the AES peripheral is disabled
pub struct IVI_R(crate::FieldReader<u32, u32>);
impl IVI_R {
    pub(crate) fn new(bits: u32) -> Self {
        IVI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IVI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IVI` writer - Initialization vector input, bits \[31:0\]
///Refer to for description of the IVI\[127:0\]
///bitfield. The initialization vector is only used in chaining modes other than ECB. The AES_IVRx registers may be written only when the AES peripheral is disabled
pub struct IVI_W<'a> {
    w: &'a mut W,
}
impl<'a> IVI_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    ///Bits 0:31 - Initialization vector input, bits \[31:0\]
    ///Refer to for description of the IVI\[127:0\]
    ///bitfield. The initialization vector is only used in chaining modes other than ECB. The AES_IVRx registers may be written only when the AES peripheral is disabled
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    ///Bits 0:31 - Initialization vector input, bits \[31:0\]
    ///Refer to for description of the IVI\[127:0\]
    ///bitfield. The initialization vector is only used in chaining modes other than ECB. The AES_IVRx registers may be written only when the AES peripheral is disabled
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W {
        IVI_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AES initialization vector register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr0](index.html) module
pub struct IVR0_SPEC;
impl crate::RegisterSpec for IVR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr0::R](R) reader structure
impl crate::Readable for IVR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr0::W](W) writer structure
impl crate::Writable for IVR0_SPEC {
    type Writer = W;
}
///`reset()` method sets IVR0 to value 0
impl crate::Resettable for IVR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
