#[doc = "Register `SYSTIMER_UNIT0_VALUE_HI` reader"]
pub struct R(crate::R<SYSTIMER_UNIT0_VALUE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_UNIT0_VALUE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_UNIT0_VALUE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_UNIT0_VALUE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_UNIT0_VALUE_HI` reader - "]
pub struct TIMER_UNIT0_VALUE_HI_R(crate::FieldReader<u32, u32>);
impl TIMER_UNIT0_VALUE_HI_R {
    pub(crate) fn new(bits: u32) -> Self {
        TIMER_UNIT0_VALUE_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT0_VALUE_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn timer_unit0_value_hi(&self) -> TIMER_UNIT0_VALUE_HI_R {
        TIMER_UNIT0_VALUE_HI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_unit0_value_hi](index.html) module"]
pub struct SYSTIMER_UNIT0_VALUE_HI_SPEC;
impl crate::RegisterSpec for SYSTIMER_UNIT0_VALUE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_unit0_value_hi::R](R) reader structure"]
impl crate::Readable for SYSTIMER_UNIT0_VALUE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTIMER_UNIT0_VALUE_HI to value 0"]
impl crate::Resettable for SYSTIMER_UNIT0_VALUE_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
