#[doc = "Register `MOSI_DLEN` reader"]
pub struct R(crate::R<MOSI_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSI_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSI_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSI_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSI_DLEN` writer"]
pub struct W(crate::W<MOSI_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSI_DLEN_SPEC>;
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
impl From<crate::W<MOSI_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSI_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_MOSI_DBITLEN` reader - "]
pub struct USR_MOSI_DBITLEN_R(crate::FieldReader<u16, u16>);
impl USR_MOSI_DBITLEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        USR_MOSI_DBITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MOSI_DBITLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MOSI_DBITLEN` writer - "]
pub struct USR_MOSI_DBITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_DBITLEN_W<'a> {
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
    pub fn usr_mosi_dbitlen(&self) -> USR_MOSI_DBITLEN_R {
        USR_MOSI_DBITLEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn usr_mosi_dbitlen(&mut self) -> USR_MOSI_DBITLEN_W {
        USR_MOSI_DBITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_MOSI_DLEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi_dlen](index.html) module"]
pub struct MOSI_DLEN_SPEC;
impl crate::RegisterSpec for MOSI_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mosi_dlen::R](R) reader structure"]
impl crate::Readable for MOSI_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mosi_dlen::W](W) writer structure"]
impl crate::Writable for MOSI_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MOSI_DLEN to value 0"]
impl crate::Resettable for MOSI_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
