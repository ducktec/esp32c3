#[doc = "Register `CACHE_STATE` reader"]
pub struct R(crate::R<CACHE_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICACHE_STATE` reader - "]
pub struct ICACHE_STATE_R(crate::FieldReader<u16, u16>);
impl ICACHE_STATE_R {
    pub(crate) fn new(bits: u16) -> Self {
        ICACHE_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_STATE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn icache_state(&self) -> ICACHE_STATE_R {
        ICACHE_STATE_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "EXTMEM_CACHE_STATE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_state](index.html) module"]
pub struct CACHE_STATE_SPEC;
impl crate::RegisterSpec for CACHE_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_state::R](R) reader structure"]
impl crate::Readable for CACHE_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHE_STATE to value 0"]
impl crate::Resettable for CACHE_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
