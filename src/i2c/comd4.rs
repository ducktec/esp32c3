#[doc = "Register `COMD4` reader"]
pub struct R(crate::R<COMD4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD4` writer"]
pub struct W(crate::W<COMD4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD4_SPEC>;
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
impl From<crate::W<COMD4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND4_DONE` reader - "]
pub struct COMMAND4_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND4_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND4_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND4_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND4_DONE` writer - "]
pub struct COMMAND4_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND4_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `COMMAND4` reader - "]
pub struct COMMAND4_R(crate::FieldReader<u16, u16>);
impl COMMAND4_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND4` writer - "]
pub struct COMMAND4_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND4_W<'a> {
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
    pub fn command4_done(&self) -> COMMAND4_DONE_R {
        COMMAND4_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command4(&self) -> COMMAND4_R {
        COMMAND4_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn command4_done(&mut self) -> COMMAND4_DONE_W {
        COMMAND4_DONE_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command4(&mut self) -> COMMAND4_W {
        COMMAND4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_COMD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd4](index.html) module"]
pub struct COMD4_SPEC;
impl crate::RegisterSpec for COMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd4::R](R) reader structure"]
impl crate::Readable for COMD4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd4::W](W) writer structure"]
impl crate::Writable for COMD4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD4 to value 0"]
impl crate::Resettable for COMD4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
