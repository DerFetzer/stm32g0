///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HBITCLKDIV_A {
    ///0: 1 (bypass)
    B_0X0 = 0,
    ///26: 27
    B_0X1A = 26,
    ///63: 64
    B_0X3F = 63,
}
impl From<HBITCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HBITCLKDIV_A) -> Self {
        variant as _
    }
}
///Field `HBITCLKDIV` reader - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk).
pub struct HBITCLKDIV_R(crate::FieldReader<u8, HBITCLKDIV_A>);
impl HBITCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBITCLKDIV_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<HBITCLKDIV_A> {
        match self.bits {
            0 => Some(HBITCLKDIV_A::B_0X0),
            26 => Some(HBITCLKDIV_A::B_0X1A),
            63 => Some(HBITCLKDIV_A::B_0X3F),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == HBITCLKDIV_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1A`
    #[inline(always)]
    pub fn is_b_0x1a(&self) -> bool {
        **self == HBITCLKDIV_A::B_0X1A
    }
    ///Checks if the value of the field is `B_0X3F`
    #[inline(always)]
    pub fn is_b_0x3f(&self) -> bool {
        **self == HBITCLKDIV_A::B_0X3F
    }
}
impl core::ops::Deref for HBITCLKDIV_R {
    type Target = crate::FieldReader<u8, HBITCLKDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `HBITCLKDIV` writer - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk).
pub struct HBITCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HBITCLKDIV_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: HBITCLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///1 (bypass)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HBITCLKDIV_A::B_0X0)
    }
    ///27
    #[inline(always)]
    pub fn b_0x1a(self) -> &'a mut W {
        self.variant(HBITCLKDIV_A::B_0X1A)
    }
    ///64
    #[inline(always)]
    pub fn b_0x3f(self) -> &'a mut W {
        self.variant(HBITCLKDIV_A::B_0X3F)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
///Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IFRGAP_A {
    ///0: Not supported
    B_0X0 = 0,
    ///1: 2
    B_0X1 = 1,
    ///13: 14
    B_0XD = 13,
    ///14: 15
    B_0XE = 14,
    ///15: 16
    B_0XF = 15,
    ///31: 32
    B_0X1F = 31,
}
impl From<IFRGAP_A> for u8 {
    #[inline(always)]
    fn from(variant: IFRGAP_A) -> Self {
        variant as _
    }
}
///Field `IFRGAP` reader - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal.
pub struct IFRGAP_R(crate::FieldReader<u8, IFRGAP_A>);
impl IFRGAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFRGAP_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<IFRGAP_A> {
        match self.bits {
            0 => Some(IFRGAP_A::B_0X0),
            1 => Some(IFRGAP_A::B_0X1),
            13 => Some(IFRGAP_A::B_0XD),
            14 => Some(IFRGAP_A::B_0XE),
            15 => Some(IFRGAP_A::B_0XF),
            31 => Some(IFRGAP_A::B_0X1F),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == IFRGAP_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == IFRGAP_A::B_0X1
    }
    ///Checks if the value of the field is `B_0XD`
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        **self == IFRGAP_A::B_0XD
    }
    ///Checks if the value of the field is `B_0XE`
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        **self == IFRGAP_A::B_0XE
    }
    ///Checks if the value of the field is `B_0XF`
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        **self == IFRGAP_A::B_0XF
    }
    ///Checks if the value of the field is `B_0X1F`
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        **self == IFRGAP_A::B_0X1F
    }
}
impl core::ops::Deref for IFRGAP_R {
    type Target = crate::FieldReader<u8, IFRGAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `IFRGAP` writer - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal.
pub struct IFRGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> IFRGAP_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: IFRGAP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Not supported
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IFRGAP_A::B_0X0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IFRGAP_A::B_0X1)
    }
    ///14
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(IFRGAP_A::B_0XD)
    }
    ///15
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(IFRGAP_A::B_0XE)
    }
    ///16
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(IFRGAP_A::B_0XF)
    }
    ///32
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut W {
        self.variant(IFRGAP_A::B_0X1F)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
///Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\[5:0\]
///bitfield setting.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRANSWIN_A {
    ///0: Not supported
    B_0X0 = 0,
    ///1: 2
    B_0X1 = 1,
    ///9: 10 (recommended)
    B_0X9 = 9,
    ///31: 32
    B_0X1F = 31,
}
impl From<TRANSWIN_A> for u8 {
    #[inline(always)]
    fn from(variant: TRANSWIN_A) -> Self {
        variant as _
    }
}
///Field `TRANSWIN` reader - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\[5:0\]
///bitfield setting.
pub struct TRANSWIN_R(crate::FieldReader<u8, TRANSWIN_A>);
impl TRANSWIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRANSWIN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TRANSWIN_A> {
        match self.bits {
            0 => Some(TRANSWIN_A::B_0X0),
            1 => Some(TRANSWIN_A::B_0X1),
            9 => Some(TRANSWIN_A::B_0X9),
            31 => Some(TRANSWIN_A::B_0X1F),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TRANSWIN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TRANSWIN_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X9`
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        **self == TRANSWIN_A::B_0X9
    }
    ///Checks if the value of the field is `B_0X1F`
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        **self == TRANSWIN_A::B_0X1F
    }
}
impl core::ops::Deref for TRANSWIN_R {
    type Target = crate::FieldReader<u8, TRANSWIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRANSWIN` writer - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\[5:0\]
///bitfield setting.
pub struct TRANSWIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSWIN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TRANSWIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Not supported
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRANSWIN_A::B_0X0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRANSWIN_A::B_0X1)
    }
    ///10 (recommended)
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(TRANSWIN_A::B_0X9)
    }
    ///32
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut W {
        self.variant(TRANSWIN_A::B_0X1F)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
///Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_USBPDCLK_A {
    ///0: 1 (bypass)
    B_0X0 = 0,
    ///1: 2
    B_0X1 = 1,
    ///2: 4
    B_0X2 = 2,
    ///3: 8
    B_0X3 = 3,
    ///4: 16
    B_0X4 = 4,
}
impl From<PSC_USBPDCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_USBPDCLK_A) -> Self {
        variant as _
    }
}
///Field `PSC_USBPDCLK` reader - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz.
pub struct PSC_USBPDCLK_R(crate::FieldReader<u8, PSC_USBPDCLK_A>);
impl PSC_USBPDCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSC_USBPDCLK_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PSC_USBPDCLK_A> {
        match self.bits {
            0 => Some(PSC_USBPDCLK_A::B_0X0),
            1 => Some(PSC_USBPDCLK_A::B_0X1),
            2 => Some(PSC_USBPDCLK_A::B_0X2),
            3 => Some(PSC_USBPDCLK_A::B_0X3),
            4 => Some(PSC_USBPDCLK_A::B_0X4),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == PSC_USBPDCLK_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == PSC_USBPDCLK_A::B_0X1
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == PSC_USBPDCLK_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X3`
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        **self == PSC_USBPDCLK_A::B_0X3
    }
    ///Checks if the value of the field is `B_0X4`
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        **self == PSC_USBPDCLK_A::B_0X4
    }
}
impl core::ops::Deref for PSC_USBPDCLK_R {
    type Target = crate::FieldReader<u8, PSC_USBPDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PSC_USBPDCLK` writer - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz.
pub struct PSC_USBPDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_USBPDCLK_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: PSC_USBPDCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///1 (bypass)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PSC_USBPDCLK_A::B_0X0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PSC_USBPDCLK_A::B_0X1)
    }
    ///4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PSC_USBPDCLK_A::B_0X2)
    }
    ///8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PSC_USBPDCLK_A::B_0X3)
    }
    ///16
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PSC_USBPDCLK_A::B_0X4)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
///Field `RXORDSETEN` reader - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled
pub struct RXORDSETEN_R(crate::FieldReader<u16, u16>);
impl RXORDSETEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXORDSETEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORDSETEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXORDSETEN` writer - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled
pub struct RXORDSETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDSETEN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | ((value as u32 & 0x01ff) << 20);
        self.w
    }
}
///Transmission DMA mode enable When set, the bit enables DMA mode for transmission.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - Transmission DMA mode enable When set, the bit enables DMA mode for transmission.
pub struct TXDMAEN_R(crate::FieldReader<bool, TXDMAEN_A>);
impl TXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::B_0X0,
            true => TXDMAEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == TXDMAEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == TXDMAEN_A::B_0X1
    }
}
impl core::ops::Deref for TXDMAEN_R {
    type Target = crate::FieldReader<bool, TXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXDMAEN` writer - Transmission DMA mode enable When set, the bit enables DMA mode for transmission.
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXDMAEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
///Reception DMA mode enable When set, the bit enables DMA mode for reception.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - Reception DMA mode enable When set, the bit enables DMA mode for reception.
pub struct RXDMAEN_R(crate::FieldReader<bool, RXDMAEN_A>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::B_0X0,
            true => RXDMAEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == RXDMAEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == RXDMAEN_A::B_0X1
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, RXDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `RXDMAEN` writer - Reception DMA mode enable When set, the bit enables DMA mode for reception.
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: RXDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RXDMAEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RXDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
///UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCPDEN_A {
    ///0: Disable
    B_0X0 = 0,
    ///1: Enable
    B_0X1 = 1,
}
impl From<UCPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCPDEN_A) -> Self {
        variant as u8 != 0
    }
}
///Field `UCPDEN` reader - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state.
pub struct UCPDEN_R(crate::FieldReader<bool, UCPDEN_A>);
impl UCPDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPDEN_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UCPDEN_A {
        match self.bits {
            false => UCPDEN_A::B_0X0,
            true => UCPDEN_A::B_0X1,
        }
    }
    ///Checks if the value of the field is `B_0X0`
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        **self == UCPDEN_A::B_0X0
    }
    ///Checks if the value of the field is `B_0X1`
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        **self == UCPDEN_A::B_0X1
    }
}
impl core::ops::Deref for UCPDEN_R {
    type Target = crate::FieldReader<bool, UCPDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `UCPDEN` writer - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state.
pub struct UCPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPDEN_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: UCPDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UCPDEN_A::B_0X0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UCPDEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    ///Bits 0:5 - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk).
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:10 - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal.
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 11:15 - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\[5:0\]
    ///bitfield setting.
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    ///Bits 17:19 - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz.
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    ///Bits 20:28 - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    ///Bit 29 - Transmission DMA mode enable When set, the bit enables DMA mode for transmission.
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    ///Bit 30 - Reception DMA mode enable When set, the bit enables DMA mode for reception.
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    ///Bit 31 - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state.
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:5 - Division ratio for producing half-bit clock The bitfield determines the division ratio (the bitfield value plus one) of a ucpd_clk divider producing half-bit clock (hbit_clk).
    #[inline(always)]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W {
        HBITCLKDIV_W { w: self }
    }
    ///Bits 6:10 - Division ratio for producing inter-frame gap timer clock The bitfield determines the division ratio (the bitfield value minus one) of a ucpd_clk divider producing inter-frame gap timer clock (tInterFrameGap). The division ratio 15 is to apply for Tx clock at the USB PD 2.0 specification nominal value. The division ratios below 15 are to apply for Tx clock below nominal, and the division ratios above 15 for Tx clock above nominal.
    #[inline(always)]
    pub fn ifrgap(&mut self) -> IFRGAP_W {
        IFRGAP_W { w: self }
    }
    ///Bits 11:15 - Transition window duration The bitfield determines the division ratio (the bitfield value minus one) of a hbit_clk divider producing tTransitionWindow interval. Set a value that produces an interval of 12 to 20 us, taking into account the ucpd_clk frequency and the HBITCLKDIV\[5:0\]
    ///bitfield setting.
    #[inline(always)]
    pub fn transwin(&mut self) -> TRANSWIN_W {
        TRANSWIN_W { w: self }
    }
    ///Bits 17:19 - Pre-scaler division ratio for generating ucpd_clk The bitfield determines the division ratio of a kernel clock pre-scaler producing UCPD peripheral clock (ucpd_clk). It is recommended to use the pre-scaler so as to set the ucpd_clk frequency in the range from 6 to 9 MHz.
    #[inline(always)]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W {
        PSC_USBPDCLK_W { w: self }
    }
    ///Bits 20:28 - Receiver ordered set enable The bitfield determines the types of ordered sets that the receiver must detect. When set/cleared, each bit enables/disables a specific function: 0bxxxxxxxx1: SOP detect enabled 0bxxxxxxx1x: SOP' detect enabled 0bxxxxxx1xx: SOP'' detect enabled 0bxxxxx1xxx: Hard Reset detect enabled 0bxxxx1xxxx: Cable Detect reset enabled 0bxxx1xxxxx: SOP'_Debug enabled 0bxx1xxxxxx: SOP''_Debug enabled 0bx1xxxxxxx: SOP extension#1 enabled 0b1xxxxxxxx: SOP extension#2 enabled
    #[inline(always)]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W {
        RXORDSETEN_W { w: self }
    }
    ///Bit 29 - Transmission DMA mode enable When set, the bit enables DMA mode for transmission.
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    ///Bit 30 - Reception DMA mode enable When set, the bit enables DMA mode for reception.
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    ///Bit 31 - UCPD peripheral enable General enable of the UCPD peripheral. Upon disabling, the peripheral instantly quits any ongoing activity and all control bits and bitfields default to their reset values. They must be set to their desired values each time the peripheral transits from disabled to enabled state.
    #[inline(always)]
    pub fn ucpden(&mut self) -> UCPDEN_W {
        UCPDEN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
