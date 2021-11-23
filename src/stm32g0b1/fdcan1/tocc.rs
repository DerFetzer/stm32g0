///Register `TOCC` reader
pub struct R(crate::R<TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TOCC` writer
pub struct W(crate::W<TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCC_SPEC>;
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
impl From<crate::W<TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCC_SPEC>) -> Self {
        W(writer)
    }
}
///Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETOC_A {
    ///0: Timeout counter disabled
    B_0X0 = 0,
    ///1: Timeout counter enabled
    B_0X1 = 1,
}
impl From<ETOC_A> for bool {
    #[inline(always)]
    fn from(variant: ETOC_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ETOC` reader - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct ETOC_R(crate::FieldReader<bool, ETOC_A>);
impl ETOC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETOC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETOC_A {
        match self.bits {
            false => ETOC_A::B_0X0,
            true => ETOC_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ETOC_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ETOC_A::B_0X1
    }
}
impl core::ops::Deref for ETOC_R {
    type Target = crate::FieldReader<bool, ETOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ETOC` writer - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ETOC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Timeout counter disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETOC_A::B_0X0)
    }
    ///Timeout counter enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETOC_A::B_0X1)
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
///Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\[TOP\]
///and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\[TOP\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_A {
    ///0: Continuous operation
    B_0X0 = 0,
    ///1: Timeout controlled by Tx Event FIFO
    B_0X1 = 1,
    ///2: Timeout controlled by Rx FIFO 0
    B_0X2 = 2,
    ///3: Timeout controlled by Rx FIFO 1
    B_0X3 = 3,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
///Field `TOS` reader - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\[TOP\]
///and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\[TOP\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct TOS_R(crate::FieldReader<u8, TOS_A>);
impl TOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOS_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TOS_A {
        match self.bits {
            0 => TOS_A::B_0X0,
            1 => TOS_A::B_0X1,
            2 => TOS_A::B_0X2,
            3 => TOS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TOS_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TOS_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TOS_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TOS_A::B_0X3
    }
}
impl core::ops::Deref for TOS_R {
    type Target = crate::FieldReader<u8, TOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOS` writer - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\[TOP\]
///and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\[TOP\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
///and bit 0 \[INIT\]
///of CCCR register are set to 1.
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TOS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Continuous operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOS_A::B_0X0)
    }
    ///Timeout controlled by Tx Event FIFO
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOS_A::B_0X1)
    }
    ///Timeout controlled by Rx FIFO 0
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TOS_A::B_0X2)
    }
    ///Timeout controlled by Rx FIFO 1
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TOS_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
///Field `TOP` reader - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period.
pub struct TOP_R(crate::FieldReader<u16, u16>);
impl TOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        TOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TOP` writer - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period.
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    ///Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\[TOP\]
    ///and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\[TOP\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    ///Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period.
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - Timeout counter enable This is a protected write (P) bit, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    ///Bits 1:2 - Timeout select When operating in Continuous mode, a write to TOCV presets the counter to the value configured by TOCC\[TOP\]
    ///and continues down-counting. When the timeout counter is controlled by one of the FIFOs, an empty FIFO presets the counter to the value configured by TOCC\[TOP\]. Down-counting is started when the first FIFO element is stored. These are protected write (P) bits, write access is possible only when the bit 1 \[CCE\]
    ///and bit 0 \[INIT\]
    ///of CCCR register are set to 1.
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    ///Bits 16:31 - Timeout period Start value of the timeout counter (down-counter). Configures the timeout period.
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN timeout counter configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tocc](index.html) module
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
///`read()` method returns [tocc::R](R) reader structure
impl crate::Readable for TOCC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tocc::W](W) writer structure
impl crate::Writable for TOCC_SPEC {
    type Writer = W;
}
///`reset()` method sets TOCC to value 0xffff_0000
impl crate::Resettable for TOCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
