#[doc = "Register `BACKUP_BUS_PMS_MONITOR_2` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE` reader - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R(crate::FieldReader<bool, bool>);
impl BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE` reader - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS` reader - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R(crate::FieldReader<u8, u8>);
impl BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R {
    pub(crate) fn new(bits: u8) -> Self {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR` reader - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader<bool, bool>);
impl BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_status_hwrite(
        &self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HWRITE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_status_hsize(
        &self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HSIZE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_status_htrans(
        &self,
    ) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_STATUS_HTRANS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_intr(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_monitor_2](index.html) module"]
pub struct BACKUP_BUS_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
