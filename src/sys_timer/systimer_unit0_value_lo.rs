#[doc = "Register `SYSTIMER_UNIT0_VALUE_LO` reader"]
pub struct R(crate::R<SYSTIMER_UNIT0_VALUE_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_UNIT0_VALUE_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_UNIT0_VALUE_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_UNIT0_VALUE_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_UNIT0_VALUE_LO` reader - "]
pub struct TIMER_UNIT0_VALUE_LO_R(crate::FieldReader<u32, u32>);
impl TIMER_UNIT0_VALUE_LO_R {
    pub(crate) fn new(bits: u32) -> Self {
        TIMER_UNIT0_VALUE_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT0_VALUE_LO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn timer_unit0_value_lo(&self) -> TIMER_UNIT0_VALUE_LO_R {
        TIMER_UNIT0_VALUE_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_unit0_value_lo](index.html) module"]
pub struct SYSTIMER_UNIT0_VALUE_LO_SPEC;
impl crate::RegisterSpec for SYSTIMER_UNIT0_VALUE_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_unit0_value_lo::R](R) reader structure"]
impl crate::Readable for SYSTIMER_UNIT0_VALUE_LO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTIMER_UNIT0_VALUE_LO to value 0"]
impl crate::Resettable for SYSTIMER_UNIT0_VALUE_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
