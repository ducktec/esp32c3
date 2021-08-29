#[doc = "Register `SYSTIMER_INT_ST` reader"]
pub struct R(crate::R<SYSTIMER_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TARGET2_INT_ST` reader - "]
pub struct TARGET2_INT_ST_R(crate::FieldReader<bool, bool>);
impl TARGET2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET1_INT_ST` reader - "]
pub struct TARGET1_INT_ST_R(crate::FieldReader<bool, bool>);
impl TARGET1_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET0_INT_ST` reader - "]
pub struct TARGET0_INT_ST_R(crate::FieldReader<bool, bool>);
impl TARGET0_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn target2_int_st(&self) -> TARGET2_INT_ST_R {
        TARGET2_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn target1_int_st(&self) -> TARGET1_INT_ST_R {
        TARGET1_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn target0_int_st(&self) -> TARGET0_INT_ST_R {
        TARGET0_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SYS_TIMER_SYSTIMER_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_int_st](index.html) module"]
pub struct SYSTIMER_INT_ST_SPEC;
impl crate::RegisterSpec for SYSTIMER_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_int_st::R](R) reader structure"]
impl crate::Readable for SYSTIMER_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSTIMER_INT_ST to value 0"]
impl crate::Resettable for SYSTIMER_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
