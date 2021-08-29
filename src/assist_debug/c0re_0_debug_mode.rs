#[doc = "Register `C0RE_0_DEBUG_MODE` reader"]
pub struct R(crate::R<C0RE_0_DEBUG_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0RE_0_DEBUG_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0RE_0_DEBUG_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0RE_0_DEBUG_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DEBUG_MODULE_ACTIVE` reader - "]
pub struct CORE_0_DEBUG_MODULE_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CORE_0_DEBUG_MODULE_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DEBUG_MODULE_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DEBUG_MODULE_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_DEBUG_MODE` reader - "]
pub struct CORE_0_DEBUG_MODE_R(crate::FieldReader<bool, bool>);
impl CORE_0_DEBUG_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE_0_DEBUG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_DEBUG_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn core_0_debug_module_active(&self) -> CORE_0_DEBUG_MODULE_ACTIVE_R {
        CORE_0_DEBUG_MODULE_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn core_0_debug_mode(&self) -> CORE_0_DEBUG_MODE_R {
        CORE_0_DEBUG_MODE_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0re_0_debug_mode](index.html) module"]
pub struct C0RE_0_DEBUG_MODE_SPEC;
impl crate::RegisterSpec for C0RE_0_DEBUG_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c0re_0_debug_mode::R](R) reader structure"]
impl crate::Readable for C0RE_0_DEBUG_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C0RE_0_DEBUG_MODE to value 0"]
impl crate::Resettable for C0RE_0_DEBUG_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
