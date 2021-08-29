#[doc = "Register `CACHE_MMU_FAULT_CONTENT` reader"]
pub struct R(crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_FAULT_CONTENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACHE_MMU_FAULT_CODE` reader - "]
pub struct CACHE_MMU_FAULT_CODE_R(crate::FieldReader<u8, u8>);
impl CACHE_MMU_FAULT_CODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CACHE_MMU_FAULT_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MMU_FAULT_CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_MMU_FAULT_CONTENT` reader - "]
pub struct CACHE_MMU_FAULT_CONTENT_R(crate::FieldReader<u16, u16>);
impl CACHE_MMU_FAULT_CONTENT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CACHE_MMU_FAULT_CONTENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MMU_FAULT_CONTENT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn cache_mmu_fault_code(&self) -> CACHE_MMU_FAULT_CODE_R {
        CACHE_MMU_FAULT_CODE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn cache_mmu_fault_content(&self) -> CACHE_MMU_FAULT_CONTENT_R {
        CACHE_MMU_FAULT_CONTENT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "EXTMEM_CACHE_MMU_FAULT_CONTENT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_fault_content](index.html) module"]
pub struct CACHE_MMU_FAULT_CONTENT_SPEC;
impl crate::RegisterSpec for CACHE_MMU_FAULT_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_fault_content::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_FAULT_CONTENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_MMU_FAULT_CONTENT to value 0"]
impl crate::Resettable for CACHE_MMU_FAULT_CONTENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
