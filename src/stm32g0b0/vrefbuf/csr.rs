///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENVR_A {
    ///0: Internal voltage reference mode disable (external voltage reference mode).
    B_0X0 = 0,
    ///1: Internal voltage reference mode (reference buffer enable or hold mode) enable.
    B_0X1 = 1,
}
impl From<ENVR_A> for bool {
    #[inline(always)]
    fn from(variant: ENVR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ENVR` reader - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.
pub struct ENVR_R(crate::FieldReader<bool, ENVR_A>);
impl ENVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENVR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENVR_A {
        match self.bits {
            false => ENVR_A::B_0X0,
            true => ENVR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ENVR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ENVR_A::B_0X1
    }
}
impl core::ops::Deref for ENVR_R {
    type Target = crate::FieldReader<bool, ENVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ENVR` writer - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.
pub struct ENVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENVR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ENVR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Internal voltage reference mode disable (external voltage reference mode).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ENVR_A::B_0X0)
    }
    ///Internal voltage reference mode (reference buffer enable or hold mode) enable.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ENVR_A::B_0X1)
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
///High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIZ_A {
    ///0: VREF+ pin is internally connected to the voltage reference buffer output.
    B_0X0 = 0,
    ///1: VREF+ pin is high impedance.
    B_0X1 = 1,
}
impl From<HIZ_A> for bool {
    #[inline(always)]
    fn from(variant: HIZ_A) -> Self {
        variant as u8 != 0
    }
}
///Field `HIZ` reader - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.
pub struct HIZ_R(crate::FieldReader<bool, HIZ_A>);
impl HIZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIZ_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HIZ_A {
        match self.bits {
            false => HIZ_A::B_0X0,
            true => HIZ_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HIZ_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == HIZ_A::B_0X1
    }
}
impl core::ops::Deref for HIZ_R {
    type Target = crate::FieldReader<bool, HIZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HIZ` writer - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.
pub struct HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> HIZ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HIZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///VREF+ pin is internally connected to the voltage reference buffer output.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HIZ_A::B_0X0)
    }
    ///VREF+ pin is high impedance.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HIZ_A::B_0X1)
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
///Voltage reference scale This bit selects the value generated by the voltage reference buffer.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRS_A {
    ///0: Voltage reference set to VREF_OUT1 (around 2.048 V).
    B_0X0 = 0,
    ///1: Voltage reference set to VREF_OUT2 (around 2.5 V).
    B_0X1 = 1,
}
impl From<VRS_A> for bool {
    #[inline(always)]
    fn from(variant: VRS_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VRS` reader - Voltage reference scale This bit selects the value generated by the voltage reference buffer.
pub struct VRS_R(crate::FieldReader<bool, VRS_A>);
impl VRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VRS_A {
        match self.bits {
            false => VRS_A::B_0X0,
            true => VRS_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == VRS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == VRS_A::B_0X1
    }
}
impl core::ops::Deref for VRS_R {
    type Target = crate::FieldReader<bool, VRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VRS` writer - Voltage reference scale This bit selects the value generated by the voltage reference buffer.
pub struct VRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VRS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: VRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Voltage reference set to VREF_OUT1 (around 2.048 V).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VRS_A::B_0X0)
    }
    ///Voltage reference set to VREF_OUT2 (around 2.5 V).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VRS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
///Voltage reference buffer ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRR_A {
    ///0: the voltage reference buffer output is not ready.
    B_0X0 = 0,
    ///1: the voltage reference buffer output reached the requested level.
    B_0X1 = 1,
}
impl From<VRR_A> for bool {
    #[inline(always)]
    fn from(variant: VRR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `VRR` reader - Voltage reference buffer ready
pub struct VRR_R(crate::FieldReader<bool, VRR_A>);
impl VRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VRR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VRR_A {
        match self.bits {
            false => VRR_A::B_0X0,
            true => VRR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == VRR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == VRR_A::B_0X1
    }
}
impl core::ops::Deref for VRR_R {
    type Target = crate::FieldReader<bool, VRR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - Voltage reference scale This bit selects the value generated by the voltage reference buffer.
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Voltage reference buffer ready
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode.
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W {
        ENVR_W { w: self }
    }
    ///Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to for the mode descriptions depending on ENVR bit configuration.
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W {
        HIZ_W { w: self }
    }
    ///Bit 2 - Voltage reference scale This bit selects the value generated by the voltage reference buffer.
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W {
        VRS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///VREFBUF control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
///`reset()` method sets CSR to value 0x02
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
