#[doc = "Register `CACHE_MMU_ACCESS_0` reader"]
pub struct R(crate::R<CACHE_MMU_ACCESS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_ACCESS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_ACCESS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_ACCESS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MMU_ACCESS_0` writer"]
pub struct W(crate::W<CACHE_MMU_ACCESS_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MMU_ACCESS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CACHE_MMU_ACCESS_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MMU_ACCESS_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_MMU_ACCESS_LOCK` reader - "]
pub struct CACHE_MMU_ACCESS_LOCK_R(crate::FieldReader<bool, bool>);
impl CACHE_MMU_ACCESS_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_MMU_ACCESS_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_MMU_ACCESS_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_MMU_ACCESS_LOCK` writer - "]
pub struct CACHE_MMU_ACCESS_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_MMU_ACCESS_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cache_mmu_access_lock(&self) -> CACHE_MMU_ACCESS_LOCK_R {
        CACHE_MMU_ACCESS_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cache_mmu_access_lock(&mut self) -> CACHE_MMU_ACCESS_LOCK_W {
        CACHE_MMU_ACCESS_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_access_0](index.html) module"]
pub struct CACHE_MMU_ACCESS_0_SPEC;
impl crate::RegisterSpec for CACHE_MMU_ACCESS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_access_0::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_ACCESS_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mmu_access_0::W](W) writer structure"]
impl crate::Writable for CACHE_MMU_ACCESS_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_MMU_ACCESS_0 to value 0"]
impl crate::Resettable for CACHE_MMU_ACCESS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
