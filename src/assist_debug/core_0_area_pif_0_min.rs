#[doc = "Register `CORE_0_AREA_PIF_0_MIN` reader"]
pub struct R(crate::R<CORE_0_AREA_PIF_0_MIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_AREA_PIF_0_MIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_AREA_PIF_0_MIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_AREA_PIF_0_MIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_AREA_PIF_0_MIN` writer"]
pub struct W(crate::W<CORE_0_AREA_PIF_0_MIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_AREA_PIF_0_MIN_SPEC>;
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
impl From<crate::W<CORE_0_AREA_PIF_0_MIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_AREA_PIF_0_MIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_MIN` reader - "]
pub struct CORE_0_AREA_PIF_0_MIN_R(crate::FieldReader<u32, u32>);
impl CORE_0_AREA_PIF_0_MIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        CORE_0_AREA_PIF_0_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_AREA_PIF_0_MIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_AREA_PIF_0_MIN` writer - "]
pub struct CORE_0_AREA_PIF_0_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_AREA_PIF_0_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_0_area_pif_0_min(&self) -> CORE_0_AREA_PIF_0_MIN_R {
        CORE_0_AREA_PIF_0_MIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core_0_area_pif_0_min(&mut self) -> CORE_0_AREA_PIF_0_MIN_W {
        CORE_0_AREA_PIF_0_MIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_area_pif_0_min](index.html) module"]
pub struct CORE_0_AREA_PIF_0_MIN_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_PIF_0_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_area_pif_0_min::R](R) reader structure"]
impl crate::Readable for CORE_0_AREA_PIF_0_MIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_area_pif_0_min::W](W) writer structure"]
impl crate::Writable for CORE_0_AREA_PIF_0_MIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_AREA_PIF_0_MIN to value 0"]
impl crate::Resettable for CORE_0_AREA_PIF_0_MIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
