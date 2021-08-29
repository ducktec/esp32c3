#[doc = "Register `T0LOADLO` reader"]
pub struct R(crate::R<T0LOADLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LOADLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LOADLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LOADLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0LOADLO` writer"]
pub struct W(crate::W<T0LOADLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOADLO_SPEC>;
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
impl From<crate::W<T0LOADLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOADLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_LOAD_LO` reader - "]
pub struct T0_LOAD_LO_R(crate::FieldReader<u32, u32>);
impl T0_LOAD_LO_R {
    pub(crate) fn new(bits: u32) -> Self {
        T0_LOAD_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_LOAD_LO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_LOAD_LO` writer - "]
pub struct T0_LOAD_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_LOAD_LO_W<'a> {
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
    pub fn t0_load_lo(&self) -> T0_LOAD_LO_R {
        T0_LOAD_LO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t0_load_lo(&mut self) -> T0_LOAD_LO_W {
        T0_LOAD_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_T0LOADLO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadlo](index.html) module"]
pub struct T0LOADLO_SPEC;
impl crate::RegisterSpec for T0LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0loadlo::R](R) reader structure"]
impl crate::Readable for T0LOADLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0loadlo::W](W) writer structure"]
impl crate::Writable for T0LOADLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0LOADLO to value 0"]
impl crate::Resettable for T0LOADLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
