#[doc = "Register `SWFC_CONF0` reader"]
pub struct R(crate::R<SWFC_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWFC_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWFC_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWFC_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWFC_CONF0` writer"]
pub struct W(crate::W<SWFC_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWFC_CONF0_SPEC>;
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
impl From<crate::W<SWFC_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWFC_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOFF_CHAR` reader - "]
pub struct XOFF_CHAR_R(crate::FieldReader<u8, u8>);
impl XOFF_CHAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        XOFF_CHAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOFF_CHAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOFF_CHAR` writer - "]
pub struct XOFF_CHAR_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_CHAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | ((value as u32 & 0xff) << 9);
        self.w
    }
}
#[doc = "Field `XOFF_THRESHOLD` reader - "]
pub struct XOFF_THRESHOLD_R(crate::FieldReader<u16, u16>);
impl XOFF_THRESHOLD_R {
    pub(crate) fn new(bits: u16) -> Self {
        XOFF_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOFF_THRESHOLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOFF_THRESHOLD` writer - "]
pub struct XOFF_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFF_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn xoff_char(&self) -> XOFF_CHAR_R {
        XOFF_CHAR_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn xoff_threshold(&self) -> XOFF_THRESHOLD_R {
        XOFF_THRESHOLD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 9:16"]
    #[inline(always)]
    pub fn xoff_char(&mut self) -> XOFF_CHAR_W {
        XOFF_CHAR_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn xoff_threshold(&mut self) -> XOFF_THRESHOLD_W {
        XOFF_THRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_SWFC_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swfc_conf0](index.html) module"]
pub struct SWFC_CONF0_SPEC;
impl crate::RegisterSpec for SWFC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swfc_conf0::R](R) reader structure"]
impl crate::Readable for SWFC_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swfc_conf0::W](W) writer structure"]
impl crate::Writable for SWFC_CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWFC_CONF0 to value 0"]
impl crate::Resettable for SWFC_CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
