#[doc = "Register `MISO_DLEN` reader"]
pub struct R(crate::R<MISO_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISO_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISO_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISO_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISO_DLEN` writer"]
pub struct W(crate::W<MISO_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISO_DLEN_SPEC>;
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
impl From<crate::W<MISO_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISO_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_MISO_DBITLEN` reader - "]
pub struct USR_MISO_DBITLEN_R(crate::FieldReader<u16, u16>);
impl USR_MISO_DBITLEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        USR_MISO_DBITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MISO_DBITLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MISO_DBITLEN` writer - "]
pub struct USR_MISO_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_DBITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn usr_miso_dbitlen(&self) -> USR_MISO_DBITLEN_R {
        USR_MISO_DBITLEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn usr_miso_dbitlen(&mut self) -> USR_MISO_DBITLEN_W {
        USR_MISO_DBITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_MISO_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso_dlen](index.html) module"]
pub struct MISO_DLEN_SPEC;
impl crate::RegisterSpec for MISO_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miso_dlen::R](R) reader structure"]
impl crate::Readable for MISO_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miso_dlen::W](W) writer structure"]
impl crate::Writable for MISO_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISO_DLEN to value 0"]
impl crate::Resettable for MISO_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
