#[doc = "Register `LSCH2_DUTY_R` reader"]
pub struct R(crate::R<LSCH2_DUTY_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH2_DUTY_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH2_DUTY_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH2_DUTY_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUTY_LSCH2` reader - "]
pub struct DUTY_LSCH2_R(crate::FieldReader<u32, u32>);
impl DUTY_LSCH2_R {
    pub(crate) fn new(bits: u32) -> Self {
        DUTY_LSCH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_LSCH2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn duty_lsch2(&self) -> DUTY_LSCH2_R {
        DUTY_LSCH2_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
#[doc = "LEDC_LSCH2_DUTY_R\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch2_duty_r](index.html) module"]
pub struct LSCH2_DUTY_R_SPEC;
impl crate::RegisterSpec for LSCH2_DUTY_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch2_duty_r::R](R) reader structure"]
impl crate::Readable for LSCH2_DUTY_R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LSCH2_DUTY_R to value 0"]
impl crate::Resettable for LSCH2_DUTY_R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
