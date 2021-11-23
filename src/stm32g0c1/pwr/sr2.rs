///Register `SR2` reader
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PVMODAC` reader - VDDA monitoring output flag
pub struct PVMODAC_R(crate::FieldReader<bool, bool>);
impl PVMODAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMODAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMODAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVMOUSB` reader - USB supply voltage monitoring output flag
pub struct PVMOUSB_R(crate::FieldReader<bool, bool>);
impl PVMOUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVMOUSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVMOUSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `PVDO` reader - Power voltage detector output
pub struct PVDO_R(crate::FieldReader<bool, bool>);
impl PVDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `VOSF` reader - Voltage scaling flag
pub struct VOSF_R(crate::FieldReader<bool, bool>);
impl VOSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REGLPF` reader - Low-power regulator flag
pub struct REGLPF_R(crate::FieldReader<bool, bool>);
impl REGLPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGLPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGLPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `REGLPS` reader - Low-power regulator started
pub struct REGLPS_R(crate::FieldReader<bool, bool>);
impl REGLPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGLPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGLPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FLASH_RDY` reader - Flash ready flag
pub struct FLASH_RDY_R(crate::FieldReader<bool, bool>);
impl FLASH_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bit 15 - VDDA monitoring output flag
    #[inline(always)]
    pub fn pvmodac(&self) -> PVMODAC_R {
        PVMODAC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    ///Bit 12 - USB supply voltage monitoring output flag
    #[inline(always)]
    pub fn pvmousb(&self) -> PVMOUSB_R {
        PVMOUSB_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    ///Bit 10 - Voltage scaling flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 9 - Low-power regulator flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 8 - Low-power regulator started
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 7 - Flash ready flag
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
///Power status register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr2](index.html) module
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr2::R](R) reader structure
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
