///Register `CCCR` reader
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCCR` writer
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Initialization
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    ///0: Normal operation
    B_0X0 = 0,
    ///1: Initialization started
    B_0X1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INIT` reader - Initialization
pub struct INIT_R(crate::FieldReader<bool, INIT_A>);
impl INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::B_0X0,
            true => INIT_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == INIT_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == INIT_A::B_0X1
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INIT` writer - Initialization
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(INIT_A::B_0X0)
    }
    ///Initialization started
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(INIT_A::B_0X1)
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
///Configuration change enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    ///0: The CPU has no write access to the protected configuration registers.
    B_0X0 = 0,
    ///1: The CPU has write access to the protected configuration registers (while CCCR.INIT = 1).
    B_0X1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CCE` reader - Configuration change enable
pub struct CCE_R(crate::FieldReader<bool, CCE_A>);
impl CCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::B_0X0,
            true => CCE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CCE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CCE_A::B_0X1
    }
}
impl core::ops::Deref for CCE_R {
    type Target = crate::FieldReader<bool, CCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CCE` writer - Configuration change enable
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///The CPU has no write access to the protected configuration registers.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCE_A::B_0X0)
    }
    ///The CPU has write access to the protected configuration registers (while CCCR.INIT = 1).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCE_A::B_0X1)
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
///ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASM_A {
    ///0: Normal CAN operation
    B_0X0 = 0,
    ///1: Restricted Operation Mode active
    B_0X1 = 1,
}
impl From<ASM_A> for bool {
    #[inline(always)]
    fn from(variant: ASM_A) -> Self {
        variant as u8 != 0
    }
}
///Field `ASM` reader - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
pub struct ASM_R(crate::FieldReader<bool, ASM_A>);
impl ASM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASM_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ASM_A {
        match self.bits {
            false => ASM_A::B_0X0,
            true => ASM_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == ASM_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == ASM_A::B_0X1
    }
}
impl core::ops::Deref for ASM_R {
    type Target = crate::FieldReader<bool, ASM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ASM` writer - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
pub struct ASM_W<'a> {
    w: &'a mut W,
}
impl<'a> ASM_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: ASM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal CAN operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ASM_A::B_0X0)
    }
    ///Restricted Operation Mode active
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ASM_A::B_0X1)
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
///Clock stop acknowledge
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSA_A {
    ///0: No clock stop acknowledged
    B_0X0 = 0,
    ///1: FDCAN may be set in power down by stopping APB clock and kernel clock.
    B_0X1 = 1,
}
impl From<CSA_A> for bool {
    #[inline(always)]
    fn from(variant: CSA_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSA` reader - Clock stop acknowledge
pub struct CSA_R(crate::FieldReader<bool, CSA_A>);
impl CSA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSA_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSA_A {
        match self.bits {
            false => CSA_A::B_0X0,
            true => CSA_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CSA_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CSA_A::B_0X1
    }
}
impl core::ops::Deref for CSA_R {
    type Target = crate::FieldReader<bool, CSA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Clock stop request
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR_A {
    ///0: No clock stop requested
    B_0X0 = 0,
    ///1: Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle.
    B_0X1 = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `CSR` reader - Clock stop request
pub struct CSR_R(crate::FieldReader<bool, CSR_A>);
impl CSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::B_0X0,
            true => CSR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == CSR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == CSR_A::B_0X1
    }
}
impl core::ops::Deref for CSR_R {
    type Target = crate::FieldReader<bool, CSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CSR` writer - Clock stop request
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: CSR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///No clock stop requested
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSR_A::B_0X0)
    }
    ///Clock stop requested. When clock stop is requested, first INIT and then CSA is set after all pending transfer requests have been completed and the CAN bus reached idle.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSR_A::B_0X1)
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
///Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MON_A {
    ///0: Bus monitoring mode disabled
    B_0X0 = 0,
    ///1: Bus monitoring mode enabled
    B_0X1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
///Field `MON` reader - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
pub struct MON_R(crate::FieldReader<bool, MON_A>);
impl MON_R {
    pub(crate) fn new(bits: bool) -> Self {
        MON_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::B_0X0,
            true => MON_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == MON_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == MON_A::B_0X1
    }
}
impl core::ops::Deref for MON_R {
    type Target = crate::FieldReader<bool, MON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MON` writer - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Bus monitoring mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MON_A::B_0X0)
    }
    ///Bus monitoring mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MON_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
///Disable automatic retransmission
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR_A {
    ///0: Automatic retransmission of messages not transmitted successfully enabled
    B_0X0 = 0,
    ///1: Automatic retransmission disabled
    B_0X1 = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DAR` reader - Disable automatic retransmission
pub struct DAR_R(crate::FieldReader<bool, DAR_A>);
impl DAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAR_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::B_0X0,
            true => DAR_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == DAR_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == DAR_A::B_0X1
    }
}
impl core::ops::Deref for DAR_R {
    type Target = crate::FieldReader<bool, DAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DAR` writer - Disable automatic retransmission
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Automatic retransmission of messages not transmitted successfully enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAR_A::B_0X0)
    }
    ///Automatic retransmission disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAR_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Test mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_A {
    ///0: Normal operation, register TEST holds reset values
    B_0X0 = 0,
    ///1: Test Mode, write access to register TEST enabled
    B_0X1 = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TEST` reader - Test mode enable
pub struct TEST_R(crate::FieldReader<bool, TEST_A>);
impl TEST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::B_0X0,
            true => TEST_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TEST_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TEST_A::B_0X1
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<bool, TEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TEST` writer - Test mode enable
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TEST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal operation, register TEST holds reset values
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEST_A::B_0X0)
    }
    ///Test Mode, write access to register TEST enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///FD operation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDOE_A {
    ///0: FD operation disabled
    B_0X0 = 0,
    ///1: FD operation enabled
    B_0X1 = 1,
}
impl From<FDOE_A> for bool {
    #[inline(always)]
    fn from(variant: FDOE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FDOE` reader - FD operation enable
pub struct FDOE_R(crate::FieldReader<bool, FDOE_A>);
impl FDOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDOE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FDOE_A {
        match self.bits {
            false => FDOE_A::B_0X0,
            true => FDOE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == FDOE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == FDOE_A::B_0X1
    }
}
impl core::ops::Deref for FDOE_R {
    type Target = crate::FieldReader<bool, FDOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FDOE` writer - FD operation enable
pub struct FDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FDOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///FD operation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDOE_A::B_0X0)
    }
    ///FD operation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDOE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///FDCAN bit rate switching
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSE_A {
    ///0: Bit rate switching for transmissions disabled
    B_0X0 = 0,
    ///1: Bit rate switching for transmissions enabled
    B_0X1 = 1,
}
impl From<BRSE_A> for bool {
    #[inline(always)]
    fn from(variant: BRSE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `BRSE` reader - FDCAN bit rate switching
pub struct BRSE_R(crate::FieldReader<bool, BRSE_A>);
impl BRSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRSE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BRSE_A {
        match self.bits {
            false => BRSE_A::B_0X0,
            true => BRSE_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == BRSE_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == BRSE_A::B_0X1
    }
}
impl core::ops::Deref for BRSE_R {
    type Target = crate::FieldReader<bool, BRSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `BRSE` writer - FDCAN bit rate switching
pub struct BRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: BRSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Bit rate switching for transmissions disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRSE_A::B_0X0)
    }
    ///Bit rate switching for transmissions enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRSE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Protocol exception handling disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PXHD_A {
    ///0: Protocol exception handling enabled
    B_0X0 = 0,
    ///1: Protocol exception handling disabled
    B_0X1 = 1,
}
impl From<PXHD_A> for bool {
    #[inline(always)]
    fn from(variant: PXHD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `PXHD` reader - Protocol exception handling disable
pub struct PXHD_R(crate::FieldReader<bool, PXHD_A>);
impl PXHD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PXHD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PXHD_A {
        match self.bits {
            false => PXHD_A::B_0X0,
            true => PXHD_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PXHD_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PXHD_A::B_0X1
    }
}
impl core::ops::Deref for PXHD_R {
    type Target = crate::FieldReader<bool, PXHD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PXHD` writer - Protocol exception handling disable
pub struct PXHD_W<'a> {
    w: &'a mut W,
}
impl<'a> PXHD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PXHD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Protocol exception handling enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PXHD_A::B_0X0)
    }
    ///Protocol exception handling disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PXHD_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
///Edge filtering during bus integration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFBI_A {
    ///0: Edge filtering disabled
    B_0X0 = 0,
    ///1: Two consecutive dominant tq required to detect an edge for hard synchronization
    B_0X1 = 1,
}
impl From<EFBI_A> for bool {
    #[inline(always)]
    fn from(variant: EFBI_A) -> Self {
        variant as u8 != 0
    }
}
///Field `EFBI` reader - Edge filtering during bus integration
pub struct EFBI_R(crate::FieldReader<bool, EFBI_A>);
impl EFBI_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFBI_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EFBI_A {
        match self.bits {
            false => EFBI_A::B_0X0,
            true => EFBI_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == EFBI_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == EFBI_A::B_0X1
    }
}
impl core::ops::Deref for EFBI_R {
    type Target = crate::FieldReader<bool, EFBI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `EFBI` writer - Edge filtering during bus integration
pub struct EFBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFBI_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: EFBI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Edge filtering disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EFBI_A::B_0X0)
    }
    ///Two consecutive dominant tq required to detect an edge for hard synchronization
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EFBI_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
///If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXP_A {
    ///0: disabled
    B_0X0 = 0,
    ///1: enabled
    B_0X1 = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXP` reader - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
pub struct TXP_R(crate::FieldReader<bool, TXP_A>);
impl TXP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::B_0X0,
            true => TXP_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXP_A::B_0X1
    }
}
impl core::ops::Deref for TXP_R {
    type Target = crate::FieldReader<bool, TXP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXP` writer - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
pub struct TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXP_A::B_0X0)
    }
    ///enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
///Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NISO_A {
    ///0: CAN FD frame format according to ISO11898-1
    B_0X0 = 0,
    ///1: CAN FD frame format according to Bosch CAN FD Specification V1.0
    B_0X1 = 1,
}
impl From<NISO_A> for bool {
    #[inline(always)]
    fn from(variant: NISO_A) -> Self {
        variant as u8 != 0
    }
}
///Field `NISO` reader - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
pub struct NISO_R(crate::FieldReader<bool, NISO_A>);
impl NISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        NISO_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NISO_A {
        match self.bits {
            false => NISO_A::B_0X0,
            true => NISO_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == NISO_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == NISO_A::B_0X1
    }
}
impl core::ops::Deref for NISO_R {
    type Target = crate::FieldReader<bool, NISO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `NISO` writer - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
pub struct NISO_W<'a> {
    w: &'a mut W,
}
impl<'a> NISO_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: NISO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///CAN FD frame format according to ISO11898-1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NISO_A::B_0X0)
    }
    ///CAN FD frame format according to Bosch CAN FD Specification V1.0
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NISO_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    ///Bit 1 - Configuration change enable
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    ///Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    ///Bit 3 - Clock stop acknowledge
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    ///Bit 4 - Clock stop request
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    ///Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    ///Bit 6 - Disable automatic retransmission
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Test mode enable
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - FD operation enable
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - FDCAN bit rate switching
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 12 - Protocol exception handling disable
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 13 - Edge filtering during bus integration
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    ///Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    ///Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    ///Bit 1 - Configuration change enable
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    ///Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W {
        ASM_W { w: self }
    }
    ///Bit 4 - Clock stop request
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    ///Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    ///Bit 6 - Disable automatic retransmission
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    ///Bit 7 - Test mode enable
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    ///Bit 8 - FD operation enable
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W {
        FDOE_W { w: self }
    }
    ///Bit 9 - FDCAN bit rate switching
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W {
        BRSE_W { w: self }
    }
    ///Bit 12 - Protocol exception handling disable
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W {
        PXHD_W { w: self }
    }
    ///Bit 13 - Edge filtering during bus integration
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W {
        EFBI_W { w: self }
    }
    ///Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W {
        TXP_W { w: self }
    }
    ///Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W {
        NISO_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN CC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cccr](index.html) module
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cccr::R](R) reader structure
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cccr::W](W) writer structure
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
}
///`reset()` method sets CCCR to value 0x01
impl crate::Resettable for CCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
