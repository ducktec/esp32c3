#[doc = "Register `ENABLE` reader"]
pub struct R(crate::R<ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE` writer"]
pub struct W(crate::W<ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_SPEC>;
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
impl From<crate::W<ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_DATA` reader - "]
pub struct ENABLE_DATA_R(crate::FieldReader<u32, u32>);
impl ENABLE_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        ENABLE_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_DATA` writer - "]
pub struct ENABLE_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn enable_data(&self) -> ENABLE_DATA_R {
        ENABLE_DATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn enable_data(&mut self) -> ENABLE_DATA_W {
        ENABLE_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](index.html) module"]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable::R](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable::W](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
