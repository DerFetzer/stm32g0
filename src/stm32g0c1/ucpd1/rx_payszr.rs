///Register `RX_PAYSZR` reader
pub struct R(crate::R<RX_PAYSZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_PAYSZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_PAYSZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_PAYSZR_SPEC>) -> Self {
        R(reader)
    }
}
///Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RXPAYSZ_A {
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
impl From<RXPAYSZ_A> for u16 {
    #[inline(always)]
    fn from(variant: RXPAYSZ_A) -> Self {
        variant as _
    }
}
///Field `RXPAYSZ` reader - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
pub struct RXPAYSZ_R(crate::FieldReader<u16, RXPAYSZ_A>);
impl RXPAYSZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXPAYSZ_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RXPAYSZ_A> {
        match self.bits {
            2 => Some(RXPAYSZ_A::B_0X2),
            6 => Some(RXPAYSZ_A::B_0X6),
            30 => Some(RXPAYSZ_A::B_0X1E),
            262 => Some(RXPAYSZ_A::B_0X106),
            1023 => Some(RXPAYSZ_A::B_0X3FF),
            _ => None,
        }
    }
    ///Checks if the value of the field is `B_0X2`
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        **self == RXPAYSZ_A::B_0X2
    }
    ///Checks if the value of the field is `B_0X6`
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        **self == RXPAYSZ_A::B_0X6
    }
    ///Checks if the value of the field is `B_0X1E`
    #[inline(always)]
    pub fn is_b_0x1e(&self) -> bool {
        **self == RXPAYSZ_A::B_0X1E
    }
    ///Checks if the value of the field is `B_0X106`
    #[inline(always)]
    pub fn is_b_0x106(&self) -> bool {
        **self == RXPAYSZ_A::B_0X106
    }
    ///Checks if the value of the field is `B_0X3FF`
    #[inline(always)]
    pub fn is_b_0x3ff(&self) -> bool {
        **self == RXPAYSZ_A::B_0X3FF
    }
}
impl core::ops::Deref for RXPAYSZ_R {
    type Target = crate::FieldReader<u16, RXPAYSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:9 - Rx payload size received This bitfield contains the number of bytes of a payload (including header but excluding CRC) received: each time a new data byte is received in the UCPD_RXDR register, the bitfield value increments and the RXMSGEND flag is set (and an interrupt generated if enabled). The bitfield may return a spurious value when a byte reception is ongoing (the RXMSGEND flag is low).
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
///UCPD Rx payload size register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rx_payszr](index.html) module
pub struct RX_PAYSZR_SPEC;
impl crate::RegisterSpec for RX_PAYSZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rx_payszr::R](R) reader structure
impl crate::Readable for RX_PAYSZR_SPEC {
    type Reader = R;
}
///`reset()` method sets RX_PAYSZR to value 0
impl crate::Resettable for RX_PAYSZR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
