#[doc = "Register `CORE_0_INTR_RAW` reader"]
pub struct R(crate::R<CORE_0_INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DRAM0_EXCEPTION_MONITOR_RAW` reader - "]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_EXCEPTION_MONITOR_RAW` reader - "]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MAX_RAW` reader - "]
pub struct CORE_0_SP_SPILL_MAX_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MAX_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MAX_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MAX_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_SP_SPILL_MIN_RAW` reader - "]
pub struct CORE_0_SP_SPILL_MIN_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_SP_SPILL_MIN_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_SP_SPILL_MIN_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_SP_SPILL_MIN_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_WR_RAW` reader - "]
pub struct CORE_0_AREA_PIF_1_WR_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_WR_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_WR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_WR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_1_RD_RAW` reader - "]
pub struct CORE_0_AREA_PIF_1_RD_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_1_RD_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_1_RD_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_1_RD_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_WR_RAW` reader - "]
pub struct CORE_0_AREA_PIF_0_WR_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_WR_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_WR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_WR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_RD_RAW` reader - "]
pub struct CORE_0_AREA_PIF_0_RD_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_PIF_0_RD_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_PIF_0_RD_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_RD_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_WR_RAW` reader - "]
pub struct CORE_0_AREA_DRAM0_1_WR_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_WR_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_WR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_WR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_1_RD_RAW` reader - "]
pub struct CORE_0_AREA_DRAM0_1_RD_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_1_RD_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_1_RD_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_1_RD_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RAW` reader - "]
pub struct CORE_0_AREA_DRAM0_0_WR_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_WR_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_WR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_WR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RAW` reader - "]
pub struct CORE_0_AREA_DRAM0_0_RD_RAW_R(crate::FieldReader<bool, bool>);
impl CORE_0_AREA_DRAM0_0_RD_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_AREA_DRAM0_0_RD_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_DRAM0_0_RD_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn core_0_dram0_exception_monitor_raw(&self) -> CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R {
        CORE_0_DRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn core_0_iram0_exception_monitor_raw(&self) -> CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R {
        CORE_0_IRAM0_EXCEPTION_MONITOR_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn core_0_sp_spill_max_raw(&self) -> CORE_0_SP_SPILL_MAX_RAW_R {
        CORE_0_SP_SPILL_MAX_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn core_0_sp_spill_min_raw(&self) -> CORE_0_SP_SPILL_MIN_RAW_R {
        CORE_0_SP_SPILL_MIN_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn core_0_area_pif_1_wr_raw(&self) -> CORE_0_AREA_PIF_1_WR_RAW_R {
        CORE_0_AREA_PIF_1_WR_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn core_0_area_pif_1_rd_raw(&self) -> CORE_0_AREA_PIF_1_RD_RAW_R {
        CORE_0_AREA_PIF_1_RD_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn core_0_area_pif_0_wr_raw(&self) -> CORE_0_AREA_PIF_0_WR_RAW_R {
        CORE_0_AREA_PIF_0_WR_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn core_0_area_pif_0_rd_raw(&self) -> CORE_0_AREA_PIF_0_RD_RAW_R {
        CORE_0_AREA_PIF_0_RD_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_wr_raw(&self) -> CORE_0_AREA_DRAM0_1_WR_RAW_R {
        CORE_0_AREA_DRAM0_1_WR_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn core_0_area_dram0_1_rd_raw(&self) -> CORE_0_AREA_DRAM0_1_RD_RAW_R {
        CORE_0_AREA_DRAM0_1_RD_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_raw(&self) -> CORE_0_AREA_DRAM0_0_WR_RAW_R {
        CORE_0_AREA_DRAM0_0_WR_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_raw(&self) -> CORE_0_AREA_DRAM0_0_RD_RAW_R {
        CORE_0_AREA_DRAM0_0_RD_RAW_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_raw](index.html) module"]
pub struct CORE_0_INTR_RAW_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_raw::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_INTR_RAW to value 0"]
impl crate::Resettable for CORE_0_INTR_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
