#[doc = "Register `ONETIME_SAMPLE` reader"]
pub struct R(crate::R<ONETIME_SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONETIME_SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ONETIME_SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ONETIME_SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONETIME_SAMPLE` writer"]
pub struct W(crate::W<ONETIME_SAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONETIME_SAMPLE_SPEC>;
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
impl From<crate::W<ONETIME_SAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONETIME_SAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC1_ONETIME_SAMPLE` reader - "]
pub struct APB_SARADC1_ONETIME_SAMPLE_R(crate::FieldReader<bool, bool>);
impl APB_SARADC1_ONETIME_SAMPLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC1_ONETIME_SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC1_ONETIME_SAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC1_ONETIME_SAMPLE` writer - "]
pub struct APB_SARADC1_ONETIME_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC1_ONETIME_SAMPLE_W<'a> {
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
#[doc = "Field `APB_SARADC2_ONETIME_SAMPLE` reader - "]
pub struct APB_SARADC2_ONETIME_SAMPLE_R(crate::FieldReader<bool, bool>);
impl APB_SARADC2_ONETIME_SAMPLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC2_ONETIME_SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC2_ONETIME_SAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC2_ONETIME_SAMPLE` writer - "]
pub struct APB_SARADC2_ONETIME_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC2_ONETIME_SAMPLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ONETIME_START` reader - "]
pub struct ONETIME_START_R(crate::FieldReader<bool, bool>);
impl ONETIME_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONETIME_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONETIME_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONETIME_START` writer - "]
pub struct ONETIME_START_W<'a> {
    w: &'a mut W,
}
impl<'a> ONETIME_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `ONETIME_CHANNEL` reader - "]
pub struct ONETIME_CHANNEL_R(crate::FieldReader<u8, u8>);
impl ONETIME_CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ONETIME_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONETIME_CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONETIME_CHANNEL` writer - "]
pub struct ONETIME_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ONETIME_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "Field `ONETIME_ATTEN` reader - "]
pub struct ONETIME_ATTEN_R(crate::FieldReader<u8, u8>);
impl ONETIME_ATTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ONETIME_ATTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONETIME_ATTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONETIME_ATTEN` writer - "]
pub struct ONETIME_ATTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONETIME_ATTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc1_onetime_sample(&self) -> APB_SARADC1_ONETIME_SAMPLE_R {
        APB_SARADC1_ONETIME_SAMPLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc2_onetime_sample(&self) -> APB_SARADC2_ONETIME_SAMPLE_R {
        APB_SARADC2_ONETIME_SAMPLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn onetime_start(&self) -> ONETIME_START_R {
        ONETIME_START_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn onetime_channel(&self) -> ONETIME_CHANNEL_R {
        ONETIME_CHANNEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn onetime_atten(&self) -> ONETIME_ATTEN_R {
        ONETIME_ATTEN_R::new(((self.bits >> 23) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn apb_saradc1_onetime_sample(&mut self) -> APB_SARADC1_ONETIME_SAMPLE_W {
        APB_SARADC1_ONETIME_SAMPLE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn apb_saradc2_onetime_sample(&mut self) -> APB_SARADC2_ONETIME_SAMPLE_W {
        APB_SARADC2_ONETIME_SAMPLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn onetime_start(&mut self) -> ONETIME_START_W {
        ONETIME_START_W { w: self }
    }
    #[doc = "Bits 25:28"]
    #[inline(always)]
    pub fn onetime_channel(&mut self) -> ONETIME_CHANNEL_W {
        ONETIME_CHANNEL_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn onetime_atten(&mut self) -> ONETIME_ATTEN_W {
        ONETIME_ATTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_SARADC_ONETIME_SAMPLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [onetime_sample](index.html) module"]
pub struct ONETIME_SAMPLE_SPEC;
impl crate::RegisterSpec for ONETIME_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [onetime_sample::R](R) reader structure"]
impl crate::Readable for ONETIME_SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [onetime_sample::W](W) writer structure"]
impl crate::Writable for ONETIME_SAMPLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ONETIME_SAMPLE to value 0"]
impl crate::Resettable for ONETIME_SAMPLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
