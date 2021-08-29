#[doc = "Register `STORE3` reader"]
pub struct R(crate::R<STORE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE3` writer"]
pub struct W(crate::W<STORE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE3_SPEC>;
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
impl From<crate::W<STORE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRATCH3` reader - "]
pub struct SCRATCH3_R(crate::FieldReader<u32, u32>);
impl SCRATCH3_R {
    pub(crate) fn new(bits: u32) -> Self {
        SCRATCH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRATCH3` writer - "]
pub struct SCRATCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH3_W<'a> {
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
    pub fn scratch3(&self) -> SCRATCH3_R {
        SCRATCH3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch3(&mut self) -> SCRATCH3_W {
        SCRATCH3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_STORE3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store3](index.html) module"]
pub struct STORE3_SPEC;
impl crate::RegisterSpec for STORE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store3::R](R) reader structure"]
impl crate::Readable for STORE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store3::W](W) writer structure"]
impl crate::Writable for STORE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STORE3 to value 0"]
impl crate::Resettable for STORE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
