#[doc = "Register `PG_CTRL` reader"]
pub struct R(crate::R<PG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PG_CTRL` writer"]
pub struct W(crate::W<PG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_CTRL_SPEC>;
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
impl From<crate::W<PG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_GLITCH_EN` reader - "]
pub struct POWER_GLITCH_EN_R(crate::FieldReader<bool, bool>);
impl POWER_GLITCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        POWER_GLITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_EN` writer - "]
pub struct POWER_GLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_EN_W<'a> {
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
#[doc = "Field `POWER_GLITCH_EFUSE_SEL` reader - "]
pub struct POWER_GLITCH_EFUSE_SEL_R(crate::FieldReader<bool, bool>);
impl POWER_GLITCH_EFUSE_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        POWER_GLITCH_EFUSE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_EFUSE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_EFUSE_SEL` writer - "]
pub struct POWER_GLITCH_EFUSE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_EFUSE_SEL_W<'a> {
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
#[doc = "Field `POWER_GLITCH_FORCE_PU` reader - "]
pub struct POWER_GLITCH_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl POWER_GLITCH_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        POWER_GLITCH_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_FORCE_PU` writer - "]
pub struct POWER_GLITCH_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_FORCE_PU_W<'a> {
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
#[doc = "Field `POWER_GLITCH_FORCE_PD` reader - "]
pub struct POWER_GLITCH_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl POWER_GLITCH_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        POWER_GLITCH_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_FORCE_PD` writer - "]
pub struct POWER_GLITCH_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_FORCE_PD_W<'a> {
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
#[doc = "Field `POWER_GLITCH_DSENSE` reader - "]
pub struct POWER_GLITCH_DSENSE_R(crate::FieldReader<u8, u8>);
impl POWER_GLITCH_DSENSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        POWER_GLITCH_DSENSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_DSENSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_DSENSE` writer - "]
pub struct POWER_GLITCH_DSENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_DSENSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn power_glitch_en(&self) -> POWER_GLITCH_EN_R {
        POWER_GLITCH_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn power_glitch_efuse_sel(&self) -> POWER_GLITCH_EFUSE_SEL_R {
        POWER_GLITCH_EFUSE_SEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn power_glitch_force_pu(&self) -> POWER_GLITCH_FORCE_PU_R {
        POWER_GLITCH_FORCE_PU_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn power_glitch_force_pd(&self) -> POWER_GLITCH_FORCE_PD_R {
        POWER_GLITCH_FORCE_PD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn power_glitch_dsense(&self) -> POWER_GLITCH_DSENSE_R {
        POWER_GLITCH_DSENSE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn power_glitch_en(&mut self) -> POWER_GLITCH_EN_W {
        POWER_GLITCH_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn power_glitch_efuse_sel(&mut self) -> POWER_GLITCH_EFUSE_SEL_W {
        POWER_GLITCH_EFUSE_SEL_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn power_glitch_force_pu(&mut self) -> POWER_GLITCH_FORCE_PU_W {
        POWER_GLITCH_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn power_glitch_force_pd(&mut self) -> POWER_GLITCH_FORCE_PD_W {
        POWER_GLITCH_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn power_glitch_dsense(&mut self) -> POWER_GLITCH_DSENSE_W {
        POWER_GLITCH_DSENSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_PG_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_ctrl](index.html) module"]
pub struct PG_CTRL_SPEC;
impl crate::RegisterSpec for PG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_ctrl::R](R) reader structure"]
impl crate::Readable for PG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_ctrl::W](W) writer structure"]
impl crate::Writable for PG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PG_CTRL to value 0"]
impl crate::Resettable for PG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
