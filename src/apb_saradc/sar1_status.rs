#[doc = "Register `SAR1_STATUS` reader"]
pub struct R(crate::R<SAR1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAR1_STATUS` reader - "]
pub struct SAR1_STATUS_R(crate::FieldReader<u32, u32>);
impl SAR1_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SAR1_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR1_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sar1_status(&self) -> SAR1_STATUS_R {
        SAR1_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "APB_SARADC_SAR1_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar1_status](index.html) module"]
pub struct SAR1_STATUS_SPEC;
impl crate::RegisterSpec for SAR1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar1_status::R](R) reader structure"]
impl crate::Readable for SAR1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR1_STATUS to value 0"]
impl crate::Resettable for SAR1_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}