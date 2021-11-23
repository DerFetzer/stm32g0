///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1=0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    ///0: DAC channel1 is connected to external pin with Buffer enabled
    B_0X0 = 0,
    ///1: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled
    B_0X1 = 1,
    ///2: DAC channel1 is connected to external pin with Buffer disabled
    B_0X2 = 2,
    ///3: DAC channel1 is connected to on chip peripherals with Buffer disabled
    B_0X3 = 3,
    ///4: DAC channel1 is connected to external pin with Buffer enabled
    B_0X4 = 4,
    ///5: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled
    B_0X5 = 5,
    ///6: DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled
    B_0X6 = 6,
    ///7: DAC channel1 is connected to on chip peripherals with Buffer disabled
    B_0X7 = 7,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
///Field `MODE1` reader - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1=0.
pub struct MODE1_R(crate::FieldReader<u8, MODE1_A>);
impl MODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE1_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::B_0X0,
            1 => MODE1_A::B_0X1,
            2 => MODE1_A::B_0X2,
            3 => MODE1_A::B_0X3,
            4 => MODE1_A::B_0X4,
            5 => MODE1_A::B_0X5,
            6 => MODE1_A::B_0X6,
            7 => MODE1_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MODE1_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MODE1_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == MODE1_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == MODE1_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == MODE1_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == MODE1_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == MODE1_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == MODE1_A::B_0X7
    }
}
impl core::ops::Deref for MODE1_R {
    type Target = crate::FieldReader<u8, MODE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE1` writer - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1=0.
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///DAC channel1 is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X0)
    }
    ///DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X1)
    }
    ///DAC channel1 is connected to external pin with Buffer disabled
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X2)
    }
    ///DAC channel1 is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X3)
    }
    ///DAC channel1 is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X4)
    }
    ///DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X5)
    }
    ///DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X6)
    }
    ///DAC channel1 is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MODE1_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
///DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE2_A {
    ///0: DAC channel2 is connected to external pin with Buffer enabled
    B_0X0 = 0,
    ///1: DAC channel2 is connected to external pin and to on chip peripherals with buffer enabled
    B_0X1 = 1,
    ///2: DAC channel2 is connected to external pin with buffer disabled
    B_0X2 = 2,
    ///3: DAC channel2 is connected to on chip peripherals with Buffer disabled
    B_0X3 = 3,
    ///4: DAC channel2 is connected to external pin with Buffer enabled
    B_0X4 = 4,
    ///5: DAC channel2 is connected to external pin and to on chip peripherals with Buffer enabled
    B_0X5 = 5,
    ///6: DAC channel2 is connected to external pin and to on chip peripherals with Buffer disabled
    B_0X6 = 6,
    ///7: DAC channel2 is connected to on chip peripherals with Buffer disabled
    B_0X7 = 7,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as _
    }
}
///Field `MODE2` reader - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2.
pub struct MODE2_R(crate::FieldReader<u8, MODE2_A>);
impl MODE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE2_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::B_0X0,
            1 => MODE2_A::B_0X1,
            2 => MODE2_A::B_0X2,
            3 => MODE2_A::B_0X3,
            4 => MODE2_A::B_0X4,
            5 => MODE2_A::B_0X5,
            6 => MODE2_A::B_0X6,
            7 => MODE2_A::B_0X7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MODE2_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MODE2_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == MODE2_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == MODE2_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == MODE2_A::B_0X4
    }
    ///Checks if the value of the field is `B_0X5`
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        **self == MODE2_A::B_0X5
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == MODE2_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X7`
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        **self == MODE2_A::B_0X7
    }
}
impl core::ops::Deref for MODE2_R {
    type Target = crate::FieldReader<u8, MODE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE2` writer - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2.
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///DAC channel2 is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X0)
    }
    ///DAC channel2 is connected to external pin and to on chip peripherals with buffer enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X1)
    }
    ///DAC channel2 is connected to external pin with buffer disabled
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X2)
    }
    ///DAC channel2 is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X3)
    }
    ///DAC channel2 is connected to external pin with Buffer enabled
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X4)
    }
    ///DAC channel2 is connected to external pin and to on chip peripherals with Buffer enabled
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X5)
    }
    ///DAC channel2 is connected to external pin and to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X6)
    }
    ///DAC channel2 is connected to on chip peripherals with Buffer disabled
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MODE2_A::B_0X7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1=0.
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x07) as u8)
    }
    ///Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2.
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    ///Bits 0:2 - DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1=0.
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    ///Bits 16:18 - DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2.
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
