#[doc = "Register `TIME_HIGH0` reader"]
pub struct R(crate::R<TIME_HIGH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_HIGH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_HIGH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_HIGH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_VALUE0_HIGH` reader - "]
pub struct TIMER_VALUE0_HIGH_R(crate::FieldReader<u16, u16>);
impl TIMER_VALUE0_HIGH_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMER_VALUE0_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_VALUE0_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer_value0_high(&self) -> TIMER_VALUE0_HIGH_R {
        TIMER_VALUE0_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RTC_CNTL_TIME_HIGH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_high0](index.html) module"]
pub struct TIME_HIGH0_SPEC;
impl crate::RegisterSpec for TIME_HIGH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_high0::R](R) reader structure"]
impl crate::Readable for TIME_HIGH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIME_HIGH0 to value 0"]
impl crate::Resettable for TIME_HIGH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
