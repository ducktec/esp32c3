#[doc = "Register `CMD2` reader"]
pub struct R(crate::R<CMD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD2` writer"]
pub struct W(crate::W<CMD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD2_SPEC>;
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
impl From<crate::W<CMD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND2_DONE` reader - "]
pub struct COMMAND2_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND2_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND2_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND2_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND2` reader - "]
pub struct COMMAND2_R(crate::FieldReader<u16, u16>);
impl COMMAND2_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND2` writer - "]
pub struct COMMAND2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND2_W<'a> {
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
    pub fn command2_done(&self) -> COMMAND2_DONE_R {
        COMMAND2_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command2(&self) -> COMMAND2_R {
        COMMAND2_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command2(&mut self) -> COMMAND2_W {
        COMMAND2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_I2C_CMD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd2](index.html) module"]
pub struct CMD2_SPEC;
impl crate::RegisterSpec for CMD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd2::R](R) reader structure"]
impl crate::Readable for CMD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd2::W](W) writer structure"]
impl crate::Writable for CMD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD2 to value 0"]
impl crate::Resettable for CMD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
