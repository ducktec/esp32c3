#[doc = "Register `CORE_0_IRAM0_EXCEPTION_MONITOR_1` reader"]
pub struct R(crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_IRAM0_RECORDING_LOADSTORE_1` reader - "]
pub struct CORE_0_IRAM0_RECORDING_LOADSTORE_1_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_RECORDING_LOADSTORE_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_RECORDING_LOADSTORE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_RECORDING_LOADSTORE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_RECORDING_WR_1` reader - "]
pub struct CORE_0_IRAM0_RECORDING_WR_1_R(crate::FieldReader<bool, bool>);
impl CORE_0_IRAM0_RECORDING_WR_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_IRAM0_RECORDING_WR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_RECORDING_WR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_IRAM0_RECORDING_ADDR_1` reader - "]
pub struct CORE_0_IRAM0_RECORDING_ADDR_1_R(crate::FieldReader<u32, u32>);
impl CORE_0_IRAM0_RECORDING_ADDR_1_R {
    pub(crate) fn new(bits: u32) -> Self {
        CORE_0_IRAM0_RECORDING_ADDR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_IRAM0_RECORDING_ADDR_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_1(&self) -> CORE_0_IRAM0_RECORDING_LOADSTORE_1_R {
        CORE_0_IRAM0_RECORDING_LOADSTORE_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_1(&self) -> CORE_0_IRAM0_RECORDING_WR_1_R {
        CORE_0_IRAM0_RECORDING_WR_1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_1(&self) -> CORE_0_IRAM0_RECORDING_ADDR_1_R {
        CORE_0_IRAM0_RECORDING_ADDR_1_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_iram0_exception_monitor_1](index.html) module"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_iram0_exception_monitor_1::R](R) reader structure"]
impl crate::Readable for CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_IRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
