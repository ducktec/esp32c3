#[doc = "Register `LOG_MEM_WRITING_ADDR` reader"]
pub struct R(crate::R<LOG_MEM_WRITING_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MEM_WRITING_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MEM_WRITING_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MEM_WRITING_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOG_MEM_WRITING_ADDR` reader - "]
pub struct LOG_MEM_WRITING_ADDR_R(crate::FieldReader<u32, u32>);
impl LOG_MEM_WRITING_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOG_MEM_WRITING_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOG_MEM_WRITING_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn log_mem_writing_addr(&self) -> LOG_MEM_WRITING_ADDR_R {
        LOG_MEM_WRITING_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_writing_addr](index.html) module"]
pub struct LOG_MEM_WRITING_ADDR_SPEC;
impl crate::RegisterSpec for LOG_MEM_WRITING_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_mem_writing_addr::R](R) reader structure"]
impl crate::Readable for LOG_MEM_WRITING_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOG_MEM_WRITING_ADDR to value 0"]
impl crate::Resettable for LOG_MEM_WRITING_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
