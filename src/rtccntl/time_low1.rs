#[doc = "Register `TIME_LOW1` reader"]
pub struct R(crate::R<TIME_LOW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_LOW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_LOW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_LOW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_VALUE1_LOW` reader - "]
pub struct TIMER_VALUE1_LOW_R(crate::FieldReader<u32, u32>);
impl TIMER_VALUE1_LOW_R {
    pub(crate) fn new(bits: u32) -> Self {
        TIMER_VALUE1_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_VALUE1_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer_value1_low(&self) -> TIMER_VALUE1_LOW_R {
        TIMER_VALUE1_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RTC_CNTL_TIME_LOW1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_low1](index.html) module"]
pub struct TIME_LOW1_SPEC;
impl crate::RegisterSpec for TIME_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_low1::R](R) reader structure"]
impl crate::Readable for TIME_LOW1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME_LOW1 to value 0"]
impl crate::Resettable for TIME_LOW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}