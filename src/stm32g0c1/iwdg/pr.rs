///Register `PR` reader
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PR` writer
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
///Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PR_A {
    ///0: divider /4
    B_0X0 = 0,
    ///1: divider /8
    B_0X1 = 1,
    ///2: divider /16
    B_0X2 = 2,
    ///3: divider /32
    B_0X3 = 3,
    ///4: divider /64
    B_0X4 = 4,
    ///5: divider /128
    B_0X5 = 5,
    ///6: divider /256
    B_0X6 = 6,
    ///7: divider /256
    B_0X7 = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
///Field `PR` reader - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.
pub struct PR_R(crate::FieldReader<u8, PR_A>);
impl PR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::B_0X0,
            1 => PR_A::B_0X1,
            2 => PR_A::B_0X2,
            3 => PR_A::B_0X3,
            4 => PR_A::B_0X4,
            5 => PR_A::B_0X5,
            6 => PR_A::B_0X6,
            7 => PR_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PR_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == PR_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == PR_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == PR_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == PR_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == PR_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == PR_A::B_0X7
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<u8, PR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PR` writer - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///divider /4
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PR_A::B_0X0)
    }
    ///divider /8
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PR_A::B_0X1)
    }
    ///divider /16
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PR_A::B_0X2)
    }
    ///divider /32
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PR_A::B_0X3)
    }
    ///divider /64
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PR_A::B_0X4)
    }
    ///divider /128
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PR_A::B_0X5)
    }
    ///divider /256
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PR_A::B_0X6)
    }
    ///divider /256
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PR_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    ///Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr](index.html) module
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pr::R](R) reader structure
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pr::W](W) writer structure
impl crate::Writable for PR_SPEC {
    type Writer = W;
}
///`reset()` method sets PR to value 0
impl crate::Resettable for PR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
