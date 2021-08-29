#[doc = "Register `CORE_0_DRAM0_PMS_MONITOR_3` reader"]
pub struct R(crate::R<CORE_0_DRAM0_PMS_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_DRAM0_PMS_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_DRAM0_PMS_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_DRAM0_PMS_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN` reader - "]
pub struct CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R(crate::FieldReader<u8, u8>);
impl CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR` reader - "]
pub struct CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_status_byteen(
        &self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_BYTEEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_status_wr(
        &self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_dram0_pms_monitor_3](index.html) module"]
pub struct CORE_0_DRAM0_PMS_MONITOR_3_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_PMS_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_dram0_pms_monitor_3::R](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_PMS_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_DRAM0_PMS_MONITOR_3 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_PMS_MONITOR_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
