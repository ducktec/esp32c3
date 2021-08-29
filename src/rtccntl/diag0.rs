#[doc = "Register `DIAG0` reader"]
pub struct R(crate::R<DIAG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIAG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIAG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIAG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOW_POWER_DIAG1` reader - "]
pub struct LOW_POWER_DIAG1_R(crate::FieldReader<u32, u32>);
impl LOW_POWER_DIAG1_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOW_POWER_DIAG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_POWER_DIAG1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn low_power_diag1(&self) -> LOW_POWER_DIAG1_R {
        LOW_POWER_DIAG1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RTC_CNTL_DIAG0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diag0](index.html) module"]
pub struct DIAG0_SPEC;
impl crate::RegisterSpec for DIAG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diag0::R](R) reader structure"]
impl crate::Readable for DIAG0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIAG0 to value 0"]
impl crate::Resettable for DIAG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
