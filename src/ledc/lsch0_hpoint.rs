#[doc = "Register `LSCH0_HPOINT` reader"]
pub struct R(crate::R<LSCH0_HPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH0_HPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH0_HPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH0_HPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH0_HPOINT` writer"]
pub struct W(crate::W<LSCH0_HPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH0_HPOINT_SPEC>;
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
impl From<crate::W<LSCH0_HPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH0_HPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOINT_LSCH0` reader - "]
pub struct HPOINT_LSCH0_R(crate::FieldReader<u16, u16>);
impl HPOINT_LSCH0_R {
    pub(crate) fn new(bits: u16) -> Self {
        HPOINT_LSCH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPOINT_LSCH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPOINT_LSCH0` writer - "]
pub struct HPOINT_LSCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOINT_LSCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn hpoint_lsch0(&self) -> HPOINT_LSCH0_R {
        HPOINT_LSCH0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn hpoint_lsch0(&mut self) -> HPOINT_LSCH0_W {
        HPOINT_LSCH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH0_HPOINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch0_hpoint](index.html) module"]
pub struct LSCH0_HPOINT_SPEC;
impl crate::RegisterSpec for LSCH0_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch0_hpoint::R](R) reader structure"]
impl crate::Readable for LSCH0_HPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch0_hpoint::W](W) writer structure"]
impl crate::Writable for LSCH0_HPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH0_HPOINT to value 0"]
impl crate::Resettable for LSCH0_HPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}