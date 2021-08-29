#[doc = "Register `STORE0` reader"]
pub struct R(crate::R<STORE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE0` writer"]
pub struct W(crate::W<STORE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE0_SPEC>;
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
impl From<crate::W<STORE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH0` reader - "]
pub struct SCRATCH0_R(crate::FieldReader<u32, u32>);
impl SCRATCH0_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCRATCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRATCH0` writer - "]
pub struct SCRATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH0_W<'a> {
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
    pub fn scratch0(&self) -> SCRATCH0_R {
        SCRATCH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch0(&mut self) -> SCRATCH0_W {
        SCRATCH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_STORE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store0](index.html) module"]
pub struct STORE0_SPEC;
impl crate::RegisterSpec for STORE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store0::R](R) reader structure"]
impl crate::Readable for STORE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store0::W](W) writer structure"]
impl crate::Writable for STORE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STORE0 to value 0"]
impl crate::Resettable for STORE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
