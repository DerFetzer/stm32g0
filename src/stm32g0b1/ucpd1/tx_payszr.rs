///Register `TX_PAYSZR` reader
pub struct R(crate::R<TX_PAYSZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PAYSZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PAYSZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PAYSZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TX_PAYSZR` writer
pub struct W(crate::W<TX_PAYSZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PAYSZR_SPEC>;
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
impl From<crate::W<TX_PAYSZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PAYSZR_SPEC>) -> Self {
        W(writer)
    }
}
///Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum TXPAYSZ_A {
    ///2: 2 bytes - the size of Control message from the protocol layer
    B_0X2 = 2,
    ///6: 6 bytes - the shortest Data message allowed from the protocol layer)
    B_0X6 = 6,
    ///30: 30 bytes - the longest non-extended Data message allowed from the protocol layer
    B_0X1E = 30,
    ///262: 262 bytes - the longest possible extended message
    B_0X106 = 262,
    ///1023: 1024 bytes - the longest possible payload (for future expansion)
    B_0X3FF = 1023,
}
impl From<TXPAYSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: TXPAYSZ_A) -> Self {
        variant as _
    }
}
///Field `TXPAYSZ` reader - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
pub struct TXPAYSZ_R(crate::FieldReader<u16, TXPAYSZ_A>);
impl TXPAYSZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXPAYSZ_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TXPAYSZ_A> {
        match self.bits {
            2 => Some(TXPAYSZ_A::B_0X2),
            6 => Some(TXPAYSZ_A::B_0X6),
            30 => Some(TXPAYSZ_A::B_0X1E),
            262 => Some(TXPAYSZ_A::B_0X106),
            1023 => Some(TXPAYSZ_A::B_0X3FF),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == TXPAYSZ_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == TXPAYSZ_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X1E`
    #[inline(always)]
    pub fn is_b_0x1e(&self) -> bool {
        **self == TXPAYSZ_A::B_0X1E
    }
    ///Checks if the value of the field is `B_0X106`
    #[inline(always)]
    pub fn is_b_0x106(&self) -> bool {
        **self == TXPAYSZ_A::B_0X106
    }
    ///Checks if the value of the field is `B_0X3FF`
    #[inline(always)]
    pub fn is_b_0x3ff(&self) -> bool {
        **self == TXPAYSZ_A::B_0X3FF
    }
}
impl core::ops::Deref for TXPAYSZ_R {
    type Target = crate::FieldReader<u16, TXPAYSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TXPAYSZ` writer - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
pub struct TXPAYSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAYSZ_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: TXPAYSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///2 bytes - the size of Control message from the protocol layer
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TXPAYSZ_A::B_0X2)
    }
    ///6 bytes - the shortest Data message allowed from the protocol layer)
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(TXPAYSZ_A::B_0X6)
    }
    ///30 bytes - the longest non-extended Data message allowed from the protocol layer
    #[inline(always)]
    pub fn b_0x1e(self) -> &'a mut W {
        self.variant(TXPAYSZ_A::B_0X1E)
    }
    ///262 bytes - the longest possible extended message
    #[inline(always)]
    pub fn b_0x106(self) -> &'a mut W {
        self.variant(TXPAYSZ_A::B_0X106)
    }
    ///1024 bytes - the longest possible payload (for future expansion)
    #[inline(always)]
    pub fn b_0x3ff(self) -> &'a mut W {
        self.variant(TXPAYSZ_A::B_0X3FF)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    ///Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
    #[inline(always)]
    pub fn txpaysz(&self) -> TXPAYSZ_R {
        TXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Payload size yet to transmit The bitfield is modified by software and by hardware. It contains the number of bytes of a payload (including header but excluding CRC) yet to transmit: each time a data byte is written into the UCPD_TXDR register, the bitfield value decrements and the TXIS bit is set, except when the bitfield value reaches zero. The enumerated values are standard payload sizes before the start of transmission.
    #[inline(always)]
    pub fn txpaysz(&mut self) -> TXPAYSZ_W {
        TXPAYSZ_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD Tx payload size register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tx_payszr](index.html) module
pub struct TX_PAYSZR_SPEC;
impl crate::RegisterSpec for TX_PAYSZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tx_payszr::R](R) reader structure
impl crate::Readable for TX_PAYSZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tx_payszr::W](W) writer structure
impl crate::Writable for TX_PAYSZR_SPEC {
    type Writer = W;
}
///`reset()` method sets TX_PAYSZR to value 0
impl crate::Resettable for TX_PAYSZR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
