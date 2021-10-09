#[doc = "Register `SIGMADELTA1` reader"]
pub struct R(crate::R<SIGMADELTA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA1` writer"]
pub struct W(crate::W<SIGMADELTA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA1_SPEC>;
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
impl From<crate::W<SIGMADELTA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD1_PRESCALE` reader - "]
pub struct SD1_PRESCALE_R(crate::FieldReader<u8, u8>);
impl SD1_PRESCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SD1_PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD1_PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD1_PRESCALE` writer - "]
pub struct SD1_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD1_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SD1_IN` reader - "]
pub struct SD1_IN_R(crate::FieldReader<u8, u8>);
impl SD1_IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SD1_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD1_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD1_IN` writer - "]
pub struct SD1_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD1_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd1_prescale(&self) -> SD1_PRESCALE_R {
        SD1_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd1_in(&self) -> SD1_IN_R {
        SD1_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd1_prescale(&mut self) -> SD1_PRESCALE_W {
        SD1_PRESCALE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd1_in(&mut self) -> SD1_IN_W {
        SD1_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_SIGMADELTA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta1](index.html) module"]
pub struct SIGMADELTA1_SPEC;
impl crate::RegisterSpec for SIGMADELTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta1::R](R) reader structure"]
impl crate::Readable for SIGMADELTA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta1::W](W) writer structure"]
impl crate::Writable for SIGMADELTA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA1 to value 0"]
impl crate::Resettable for SIGMADELTA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}