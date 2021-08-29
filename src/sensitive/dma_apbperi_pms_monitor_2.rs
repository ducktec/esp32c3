#[doc = "Register `DMA_APBPERI_PMS_MONITOR_2` reader"]
pub struct R(crate::R<DMA_APBPERI_PMS_MONITOR_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_APBPERI_PMS_MONITOR_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_APBPERI_PMS_MONITOR_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_APBPERI_PMS_MONITOR_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR` reader - "]
pub struct DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R(crate::FieldReader<u32, u32>);
impl DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD` reader - "]
pub struct DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R(crate::FieldReader<u8, u8>);
impl DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR` reader - "]
pub struct DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader<bool, bool>);
impl DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 3:26"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_addr(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_ADDR_R::new(((self.bits >> 3) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_status_world(
        &self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_STATUS_WORLD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_intr(&self) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_apbperi_pms_monitor_2](index.html) module"]
pub struct DMA_APBPERI_PMS_MONITOR_2_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_apbperi_pms_monitor_2::R](R) reader structure"]
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_APBPERI_PMS_MONITOR_2 to value 0"]
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
