#[doc = "Register `W2` reader"]
pub struct R(crate::R<W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W2` writer"]
pub struct W(crate::W<W2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W2_SPEC>;
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
impl From<crate::W<W2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF` reader - "]
pub struct BUF_R(crate::FieldReader<u32, u32>);
impl BUF_R {
    pub(crate) fn new(bits: u32) -> Self {
        BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUF` writer - "]
pub struct BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_W<'a> {
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
    pub fn buf(&self) -> BUF_R {
        BUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W {
        BUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_W2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w2](index.html) module"]
pub struct W2_SPEC;
impl crate::RegisterSpec for W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w2::R](R) reader structure"]
impl crate::Readable for W2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w2::W](W) writer structure"]
impl crate::Writable for W2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets W2 to value 0"]
impl crate::Resettable for W2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
