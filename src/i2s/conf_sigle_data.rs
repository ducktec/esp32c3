#[doc = "Register `CONF_SIGLE_DATA` reader"]
pub struct R(crate::R<CONF_SIGLE_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SIGLE_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SIGLE_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SIGLE_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_SIGLE_DATA` writer"]
pub struct W(crate::W<CONF_SIGLE_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SIGLE_DATA_SPEC>;
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
impl From<crate::W<CONF_SIGLE_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SIGLE_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLE_DATA` reader - "]
pub struct SINGLE_DATA_R(crate::FieldReader<u32, u32>);
impl SINGLE_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        SINGLE_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLE_DATA` writer - "]
pub struct SINGLE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_DATA_W<'a> {
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
    pub fn single_data(&self) -> SINGLE_DATA_R {
        SINGLE_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn single_data(&mut self) -> SINGLE_DATA_W {
        SINGLE_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_CONF_SIGLE_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_sigle_data](index.html) module"]
pub struct CONF_SIGLE_DATA_SPEC;
impl crate::RegisterSpec for CONF_SIGLE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_sigle_data::R](R) reader structure"]
impl crate::Readable for CONF_SIGLE_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_sigle_data::W](W) writer structure"]
impl crate::Writable for CONF_SIGLE_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF_SIGLE_DATA to value 0"]
impl crate::Resettable for CONF_SIGLE_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
