///Register `SWTRGR` writer
pub struct W(crate::W<SWTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRGR_SPEC>;
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
impl From<crate::W<SWTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRGR_SPEC>) -> Self {
        W(writer)
    }
}
///DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG1_AW {
    ///0: No trigger
    B_0X0 = 0,
    ///1: Trigger
    B_0X1 = 1,
}
impl From<SWTRIG1_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No trigger
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::B_0X0)
    }
    ///Trigger
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::B_0X1)
    }
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
///DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG2_AW {
    ///0: No trigger
    B_0X0 = 0,
    ///1: Trigger
    B_0X1 = 1,
}
impl From<SWTRIG2_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG2_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation.
pub struct SWTRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No trigger
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::B_0X0)
    }
    ///Trigger
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SWTRIG2_AW::B_0X1)
    }
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
impl W {
    ///Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
    ///Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn swtrig2(&mut self) -> SWTRIG2_W {
        SWTRIG2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC software trigger register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swtrgr](index.html) module
pub struct SWTRGR_SPEC;
impl crate::RegisterSpec for SWTRGR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [swtrgr::W](W) writer structure
impl crate::Writable for SWTRGR_SPEC {
    type Writer = W;
}
///`reset()` method sets SWTRGR to value 0
impl crate::Resettable for SWTRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
