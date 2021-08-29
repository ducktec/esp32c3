#[doc = "Register `COMD1` reader"]
pub struct R(crate::R<COMD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMD1` writer"]
pub struct W(crate::W<COMD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMD1_SPEC>;
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
impl From<crate::W<COMD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMAND1_DONE` reader - "]
pub struct COMMAND1_DONE_R(crate::FieldReader<bool, bool>);
impl COMMAND1_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND1_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND1_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND1_DONE` writer - "]
pub struct COMMAND1_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND1_DONE_W<'a> {
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
#[doc = "Field `COMMAND1` reader - "]
pub struct COMMAND1_R(crate::FieldReader<u16, u16>);
impl COMMAND1_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMMAND1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND1` writer - "]
pub struct COMMAND1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND1_W<'a> {
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
    pub fn command1_done(&self) -> COMMAND1_DONE_R {
        COMMAND1_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command1(&self) -> COMMAND1_R {
        COMMAND1_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn command1_done(&mut self) -> COMMAND1_DONE_W {
        COMMAND1_DONE_W { w: self }
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn command1(&mut self) -> COMMAND1_W {
        COMMAND1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_COMD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comd1](index.html) module"]
pub struct COMD1_SPEC;
impl crate::RegisterSpec for COMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comd1::R](R) reader structure"]
impl crate::Readable for COMD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comd1::W](W) writer structure"]
impl crate::Writable for COMD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMD1 to value 0"]
impl crate::Resettable for COMD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
