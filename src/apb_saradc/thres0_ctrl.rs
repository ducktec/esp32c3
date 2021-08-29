#[doc = "Register `THRES0_CTRL` reader"]
pub struct R(crate::R<THRES0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES0_CTRL` writer"]
pub struct W(crate::W<THRES0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES0_CTRL_SPEC>;
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
impl From<crate::W<THRES0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES0_LOW` reader - "]
pub struct THRES0_LOW_R(crate::FieldReader<u16, u16>);
impl THRES0_LOW_R {
    pub(crate) fn new(bits: u16) -> Self {
        THRES0_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_LOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_LOW` writer - "]
pub struct THRES0_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES0_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 18)) | ((value as u32 & 0x1fff) << 18);
        self.w
    }
}
#[doc = "Field `THRES0_HIGH` reader - "]
pub struct THRES0_HIGH_R(crate::FieldReader<u16, u16>);
impl THRES0_HIGH_R {
    pub(crate) fn new(bits: u16) -> Self {
        THRES0_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_HIGH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_HIGH` writer - "]
pub struct THRES0_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES0_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 5)) | ((value as u32 & 0x1fff) << 5);
        self.w
    }
}
#[doc = "Field `THRES0_CHANNEL` reader - "]
pub struct THRES0_CHANNEL_R(crate::FieldReader<u8, u8>);
impl THRES0_CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        THRES0_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_CHANNEL` writer - "]
pub struct THRES0_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES0_CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:30"]
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(((self.bits >> 18) & 0x1fff) as u16)
    }
    #[doc = "Bits 5:17"]
    #[inline(always)]
    pub fn thres0_high(&self) -> THRES0_HIGH_R {
        THRES0_HIGH_R::new(((self.bits >> 5) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn thres0_channel(&self) -> THRES0_CHANNEL_R {
        THRES0_CHANNEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:30"]
    #[inline(always)]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W {
        THRES0_LOW_W { w: self }
    }
    #[doc = "Bits 5:17"]
    #[inline(always)]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W {
        THRES0_HIGH_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn thres0_channel(&mut self) -> THRES0_CHANNEL_W {
        THRES0_CHANNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_SARADC_THRES0_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres0_ctrl](index.html) module"]
pub struct THRES0_CTRL_SPEC;
impl crate::RegisterSpec for THRES0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres0_ctrl::R](R) reader structure"]
impl crate::Readable for THRES0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres0_ctrl::W](W) writer structure"]
impl crate::Writable for THRES0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THRES0_CTRL to value 0"]
impl crate::Resettable for THRES0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
