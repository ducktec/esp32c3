#[doc = "Register `CORE_0_DRAM0_PMS_MONITOR_2` reader"]
pub struct R(crate::R<CORE_0_DRAM0_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_DRAM0_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_DRAM0_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_DRAM0_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - "]
pub struct CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R(crate::FieldReader<u32, u32>);
impl CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - "]
pub struct CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R(crate::FieldReader<u8, u8>);
impl CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK` reader - "]
pub struct CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR` reader - "]
pub struct CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader<bool, bool>);
impl CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:27"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_status_addr(
        &self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new(((self.bits >> 4) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_status_world(
        &self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_status_lock(
        &self,
    ) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_STATUS_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn core_0_dram0_pms_monitor_violate_intr(&self) -> CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R {
        CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SENSITIVE_CORE_0_DRAM0_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_dram0_pms_monitor_2](index.html) module"]
pub struct CORE_0_DRAM0_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_dram0_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_DRAM0_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}