#[doc = "Register `FILTER_CTRL0` reader"]
pub struct R(crate::R<FILTER_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_CTRL0` writer"]
pub struct W(crate::W<FILTER_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_CTRL0_SPEC>;
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
impl From<crate::W<FILTER_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_RESET` reader - "]
pub struct FILTER_RESET_R(crate::FieldReader<bool, bool>);
impl FILTER_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_RESET` writer - "]
pub struct FILTER_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_RESET_W<'a> {
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
#[doc = "Field `FILTER_CHANNEL0` reader - "]
pub struct FILTER_CHANNEL0_R(crate::FieldReader<u8, u8>);
impl FILTER_CHANNEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_CHANNEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_CHANNEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_CHANNEL0` writer - "]
pub struct FILTER_CHANNEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CHANNEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `FILTER_CHANNEL1` reader - "]
pub struct FILTER_CHANNEL1_R(crate::FieldReader<u8, u8>);
impl FILTER_CHANNEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        FILTER_CHANNEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_CHANNEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_CHANNEL1` writer - "]
pub struct FILTER_CHANNEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_CHANNEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn filter_reset(&self) -> FILTER_RESET_R {
        FILTER_RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn filter_channel0(&self) -> FILTER_CHANNEL0_R {
        FILTER_CHANNEL0_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn filter_channel1(&self) -> FILTER_CHANNEL1_R {
        FILTER_CHANNEL1_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn filter_reset(&mut self) -> FILTER_RESET_W {
        FILTER_RESET_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn filter_channel0(&mut self) -> FILTER_CHANNEL0_W {
        FILTER_CHANNEL0_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn filter_channel1(&mut self) -> FILTER_CHANNEL1_W {
        FILTER_CHANNEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_SARADC_FILTER_CTRL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_ctrl0](index.html) module"]
pub struct FILTER_CTRL0_SPEC;
impl crate::RegisterSpec for FILTER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_ctrl0::R](R) reader structure"]
impl crate::Readable for FILTER_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_ctrl0::W](W) writer structure"]
impl crate::Writable for FILTER_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTER_CTRL0 to value 0"]
impl crate::Resettable for FILTER_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
