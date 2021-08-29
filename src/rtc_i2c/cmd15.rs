#[doc = "Register `CMD15` reader"]
pub struct R(crate::R<CMD15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD15` writer"]
pub struct W(crate::W<CMD15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD15_SPEC>;
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
impl From<crate::W<CMD15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND15_DONE` reader - "]
pub struct COMMAND15_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND15_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND15_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND15_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND15` reader - "]
pub struct COMMAND15_R(crate::FieldReader<u16, u16>);
impl COMMAND15_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND15_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND15` writer - "]
pub struct COMMAND15_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND15_W<'a> {
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
    pub fn command15_done(&self) -> COMMAND15_DONE_R {
        COMMAND15_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command15(&self) -> COMMAND15_R {
        COMMAND15_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command15(&mut self) -> COMMAND15_W {
        COMMAND15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_I2C_CMD15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd15](index.html) module"]
pub struct CMD15_SPEC;
impl crate::RegisterSpec for CMD15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd15::R](R) reader structure"]
impl crate::Readable for CMD15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd15::W](W) writer structure"]
impl crate::Writable for CMD15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD15 to value 0"]
impl crate::Resettable for CMD15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
