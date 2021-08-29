#[doc = "Register `2_DATA_STATUS` reader"]
pub struct R(crate::R<_2_DATA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_2_DATA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_2_DATA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_2_DATA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC2_DATA` reader - "]
pub struct ADC2_DATA_R(crate::FieldReader<u32, u32>);
impl ADC2_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADC2_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn adc2_data(&self) -> ADC2_DATA_R {
        ADC2_DATA_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
#[doc = "APB_SARADC_2_DATA_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_data_status](index.html) module"]
pub struct _2_DATA_STATUS_SPEC;
impl crate::RegisterSpec for _2_DATA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_2_data_status::R](R) reader structure"]
impl crate::Readable for _2_DATA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets 2_DATA_STATUS to value 0"]
impl crate::Resettable for _2_DATA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
