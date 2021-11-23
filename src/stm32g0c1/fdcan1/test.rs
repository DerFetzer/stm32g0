///Register `TEST` reader
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TEST` writer
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
///Loop back mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBCK_A {
    ///0: Reset value, Loop Back mode is disabled
    B_0X0 = 0,
    ///1: Loop Back mode is enabled (see Power down (Sleep mode))
    B_0X1 = 1,
}
impl From<LBCK_A> for bool {
    #[inline(always)]
    fn from(variant: LBCK_A) -> Self {
        variant as u8 != 0
    }
}
///Field `LBCK` reader - Loop back mode
pub struct LBCK_R(crate::FieldReader<bool, LBCK_A>);
impl LBCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LBCK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LBCK_A {
        match self.bits {
            false => LBCK_A::B_0X0,
            true => LBCK_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == LBCK_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == LBCK_A::B_0X1
    }
}
impl core::ops::Deref for LBCK_R {
    type Target = crate::FieldReader<bool, LBCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LBCK` writer - Loop back mode
pub struct LBCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBCK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: LBCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Reset value, Loop Back mode is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LBCK_A::B_0X0)
    }
    ///Loop Back mode is enabled (see Power down (Sleep mode))
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LBCK_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
///Control of transmit pin
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_A {
    ///0: Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time
    B_0X0 = 0,
    ///1: Sample point can be monitored at pin FDCANx_TX
    B_0X1 = 1,
    ///2: Dominant (0) level at pin FDCANx_TX
    B_0X2 = 2,
    ///3: Recessive (1) at pin FDCANx_TX
    B_0X3 = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
///Field `TX` reader - Control of transmit pin
pub struct TX_R(crate::FieldReader<u8, TX_A>);
impl TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::B_0X0,
            1 => TX_A::B_0X1,
            2 => TX_A::B_0X2,
            3 => TX_A::B_0X3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TX_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TX_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TX_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == TX_A::B_0X3
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<u8, TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TX` writer - Control of transmit pin
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Reset value, FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TX_A::B_0X0)
    }
    ///Sample point can be monitored at pin FDCANx_TX
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TX_A::B_0X1)
    }
    ///Dominant (0) level at pin FDCANx_TX
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TX_A::B_0X2)
    }
    ///Recessive (1) at pin FDCANx_TX
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TX_A::B_0X3)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
///Receive pin Monitors the actual value of pin FDCANx_RX
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    ///0: The CAN bus is dominant (FDCANx_RX = 0)
    B_0X0 = 0,
    ///1: The CAN bus is recessive (FDCANx_RX = 1)
    B_0X1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX
pub struct RX_R(crate::FieldReader<bool, RX_A>);
impl RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::B_0X0,
            true => RX_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RX_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RX_A::B_0X1
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 4 - Loop back mode
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    ///Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    ///Bit 4 - Loop back mode
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W {
        LBCK_W { w: self }
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN test register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [test](index.html) module
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [test::R](R) reader structure
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [test::W](W) writer structure
impl crate::Writable for TEST_SPEC {
    type Writer = W;
}
///`reset()` method sets TEST to value 0
impl crate::Resettable for TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
