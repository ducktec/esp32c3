#[doc = "Register `CORE_0_RCD_PDEBUGSP` reader"]
pub struct R(crate::R<CORE_0_RCD_PDEBUGSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_RCD_PDEBUGSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_RCD_PDEBUGSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_RCD_PDEBUGSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_RCD_PDEBUGSP` reader - "]
pub struct CORE_0_RCD_PDEBUGSP_R(crate::FieldReader<u32, u32>);
impl CORE_0_RCD_PDEBUGSP_R {
    pub(crate) fn new(bits: u32) -> Self {
        CORE_0_RCD_PDEBUGSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_RCD_PDEBUGSP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugsp(&self) -> CORE_0_RCD_PDEBUGSP_R {
        CORE_0_RCD_PDEBUGSP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_rcd_pdebugsp](index.html) module"]
pub struct CORE_0_RCD_PDEBUGSP_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_rcd_pdebugsp::R](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGSP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGSP to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGSP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
