#[doc = "Register `AT_CMD_POSTCNT` reader"]
pub struct R(crate::R<AT_CMD_POSTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AT_CMD_POSTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AT_CMD_POSTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AT_CMD_POSTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AT_CMD_POSTCNT` writer"]
pub struct W(crate::W<AT_CMD_POSTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AT_CMD_POSTCNT_SPEC>;
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
impl From<crate::W<AT_CMD_POSTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AT_CMD_POSTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POST_IDLE_NUM` reader - "]
pub struct POST_IDLE_NUM_R(crate::FieldReader<u16, u16>);
impl POST_IDLE_NUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        POST_IDLE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POST_IDLE_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POST_IDLE_NUM` writer - "]
pub struct POST_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn post_idle_num(&self) -> POST_IDLE_NUM_R {
        POST_IDLE_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn post_idle_num(&mut self) -> POST_IDLE_NUM_W {
        POST_IDLE_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_AT_CMD_POSTCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_postcnt](index.html) module"]
pub struct AT_CMD_POSTCNT_SPEC;
impl crate::RegisterSpec for AT_CMD_POSTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [at_cmd_postcnt::R](R) reader structure"]
impl crate::Readable for AT_CMD_POSTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [at_cmd_postcnt::W](W) writer structure"]
impl crate::Writable for AT_CMD_POSTCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AT_CMD_POSTCNT to value 0"]
impl crate::Resettable for AT_CMD_POSTCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
