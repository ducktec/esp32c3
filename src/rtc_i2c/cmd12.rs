#[doc = "Register `CMD12` reader"]
pub struct R(crate::R<CMD12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD12` writer"]
pub struct W(crate::W<CMD12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD12_SPEC>;
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
impl From<crate::W<CMD12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND12_DONE` reader - "]
pub struct COMMAND12_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND12_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND12_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND12_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND12` reader - "]
pub struct COMMAND12_R(crate::FieldReader<u16, u16>);
impl COMMAND12_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND12_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND12` writer - "]
pub struct COMMAND12_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND12_W<'a> {
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
    pub fn command12_done(&self) -> COMMAND12_DONE_R {
        COMMAND12_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command12(&self) -> COMMAND12_R {
        COMMAND12_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command12(&mut self) -> COMMAND12_W {
        COMMAND12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_I2C_CMD12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd12](index.html) module"]
pub struct CMD12_SPEC;
impl crate::RegisterSpec for CMD12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd12::R](R) reader structure"]
impl crate::Readable for CMD12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd12::W](W) writer structure"]
impl crate::Writable for CMD12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD12 to value 0"]
impl crate::Resettable for CMD12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
