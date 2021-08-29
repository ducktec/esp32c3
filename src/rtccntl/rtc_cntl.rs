#[doc = "Register `RTC_CNTL` reader"]
pub struct R(crate::R<RTC_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL` writer"]
pub struct W(crate::W<RTC_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_SPEC>;
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
impl From<crate::W<RTC_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGULATOR_FORCE_PU` reader - "]
pub struct REGULATOR_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl REGULATOR_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGULATOR_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGULATOR_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGULATOR_FORCE_PU` writer - "]
pub struct REGULATOR_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PU_W<'a> {
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
#[doc = "Field `REGULATOR_FORCE_PD` reader - "]
pub struct REGULATOR_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl REGULATOR_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGULATOR_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGULATOR_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGULATOR_FORCE_PD` writer - "]
pub struct REGULATOR_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REGULATOR_FORCE_PD_W<'a> {
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
#[doc = "Field `DBOOST_FORCE_PU` reader - "]
pub struct DBOOST_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PU` writer - "]
pub struct DBOOST_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PU_W<'a> {
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
#[doc = "Field `DBOOST_FORCE_PD` reader - "]
pub struct DBOOST_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DBOOST_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBOOST_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBOOST_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBOOST_FORCE_PD` writer - "]
pub struct DBOOST_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBOOST_FORCE_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn regulator_force_pu(&self) -> REGULATOR_FORCE_PU_R {
        REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn regulator_force_pd(&self) -> REGULATOR_FORCE_PD_R {
        REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W {
        REGULATOR_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W {
        REGULATOR_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W {
        DBOOST_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W {
        DBOOST_FORCE_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl](index.html) module"]
pub struct RTC_CNTL_SPEC;
impl crate::RegisterSpec for RTC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL to value 0"]
impl crate::Resettable for RTC_CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
