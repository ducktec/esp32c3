#[doc = "Register `SLP_REJECT_CONF` reader"]
pub struct R(crate::R<SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_REJECT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_REJECT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_REJECT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_REJECT_CONF` writer"]
pub struct W(crate::W<SLP_REJECT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_REJECT_CONF_SPEC>;
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
impl From<crate::W<SLP_REJECT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_REJECT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEEP_SLP_REJECT_EN` reader - "]
pub struct DEEP_SLP_REJECT_EN_R(crate::FieldReader<bool, bool>);
impl DEEP_SLP_REJECT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEP_SLP_REJECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEP_SLP_REJECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEP_SLP_REJECT_EN` writer - "]
pub struct DEEP_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEP_SLP_REJECT_EN_W<'a> {
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
#[doc = "Field `LIGHT_SLP_REJECT_EN` reader - "]
pub struct LIGHT_SLP_REJECT_EN_R(crate::FieldReader<bool, bool>);
impl LIGHT_SLP_REJECT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIGHT_SLP_REJECT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIGHT_SLP_REJECT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIGHT_SLP_REJECT_EN` writer - "]
pub struct LIGHT_SLP_REJECT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LIGHT_SLP_REJECT_EN_W<'a> {
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
#[doc = "Field `SLEEP_REJECT_ENA` reader - "]
pub struct SLEEP_REJECT_ENA_R(crate::FieldReader<u32, u32>);
impl SLEEP_REJECT_ENA_R {
    pub(crate) fn new(bits: u32) -> Self {
        SLEEP_REJECT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_REJECT_ENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_REJECT_ENA` writer - "]
pub struct SLEEP_REJECT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_REJECT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 12)) | ((value as u32 & 0x0003_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&self) -> DEEP_SLP_REJECT_EN_R {
        DEEP_SLP_REJECT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn light_slp_reject_en(&self) -> LIGHT_SLP_REJECT_EN_R {
        LIGHT_SLP_REJECT_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 12:29"]
    #[inline(always)]
    pub fn sleep_reject_ena(&self) -> SLEEP_REJECT_ENA_R {
        SLEEP_REJECT_ENA_R::new(((self.bits >> 12) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn deep_slp_reject_en(&mut self) -> DEEP_SLP_REJECT_EN_W {
        DEEP_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn light_slp_reject_en(&mut self) -> LIGHT_SLP_REJECT_EN_W {
        LIGHT_SLP_REJECT_EN_W { w: self }
    }
    #[doc = "Bits 12:29"]
    #[inline(always)]
    pub fn sleep_reject_ena(&mut self) -> SLEEP_REJECT_ENA_W {
        SLEEP_REJECT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_SLP_REJECT_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_conf](index.html) module"]
pub struct SLP_REJECT_CONF_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_reject_conf::R](R) reader structure"]
impl crate::Readable for SLP_REJECT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_reject_conf::W](W) writer structure"]
impl crate::Writable for SLP_REJECT_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_REJECT_CONF to value 0"]
impl crate::Resettable for SLP_REJECT_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
