#[doc = "Register `MS_DLEN` reader"]
pub struct R(crate::R<MS_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MS_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MS_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS_DLEN` writer"]
pub struct W(crate::W<MS_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS_DLEN_SPEC>;
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
impl From<crate::W<MS_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MS_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MS_DATA_BITLEN` reader - "]
pub struct MS_DATA_BITLEN_R(crate::FieldReader<u32, u32>);
impl MS_DATA_BITLEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        MS_DATA_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_DATA_BITLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS_DATA_BITLEN` writer - "]
pub struct MS_DATA_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MS_DATA_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn ms_data_bitlen(&self) -> MS_DATA_BITLEN_R {
        MS_DATA_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn ms_data_bitlen(&mut self) -> MS_DATA_BITLEN_W {
        MS_DATA_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MS_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms_dlen](index.html) module"]
pub struct MS_DLEN_SPEC;
impl crate::RegisterSpec for MS_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms_dlen::R](R) reader structure"]
impl crate::Readable for MS_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms_dlen::W](W) writer structure"]
impl crate::Writable for MS_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MS_DLEN to value 0"]
impl crate::Resettable for MS_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
