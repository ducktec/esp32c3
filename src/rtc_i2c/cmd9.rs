#[doc = "Register `CMD9` reader"]
pub struct R(crate::R<CMD9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD9` writer"]
pub struct W(crate::W<CMD9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD9_SPEC>;
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
impl From<crate::W<CMD9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND9_DONE` reader - "]
pub struct COMMAND9_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND9_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND9_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND9_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND9` reader - "]
pub struct COMMAND9_R(crate::FieldReader<u16, u16>);
impl COMMAND9_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND9_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND9` writer - "]
pub struct COMMAND9_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn command9_done(&self) -> COMMAND9_DONE_R {
        COMMAND9_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command9(&self) -> COMMAND9_R {
        COMMAND9_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command9(&mut self) -> COMMAND9_W {
        COMMAND9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_I2C_CMD9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd9](index.html) module"]
pub struct CMD9_SPEC;
impl crate::RegisterSpec for CMD9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd9::R](R) reader structure"]
impl crate::Readable for CMD9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd9::W](W) writer structure"]
impl crate::Writable for CMD9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD9 to value 0"]
impl crate::Resettable for CMD9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
