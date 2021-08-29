#[doc = "Register `FILTER_CFG` reader"]
pub struct R(crate::R<FILTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_CFG` writer"]
pub struct W(crate::W<FILTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_CFG_SPEC>;
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
impl From<crate::W<FILTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_FILTER_EN` reader - "]
pub struct SDA_FILTER_EN_R(crate::FieldReader<bool, bool>);
impl SDA_FILTER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDA_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_FILTER_EN` writer - "]
pub struct SDA_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SCL_FILTER_EN` reader - "]
pub struct SCL_FILTER_EN_R(crate::FieldReader<bool, bool>);
impl SCL_FILTER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCL_FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_FILTER_EN` writer - "]
pub struct SCL_FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FILTER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SDA_FILTER_THRES` reader - "]
pub struct SDA_FILTER_THRES_R(crate::FieldReader<u8, u8>);
impl SDA_FILTER_THRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDA_FILTER_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_FILTER_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_FILTER_THRES` writer - "]
pub struct SDA_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SCL_FILTER_THRES` reader - "]
pub struct SCL_FILTER_THRES_R(crate::FieldReader<u8, u8>);
impl SCL_FILTER_THRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCL_FILTER_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_FILTER_THRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_FILTER_THRES` writer - "]
pub struct SCL_FILTER_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_FILTER_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn scl_filter_en(&self) -> SCL_FILTER_EN_R {
        SCL_FILTER_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scl_filter_thres(&self) -> SCL_FILTER_THRES_R {
        SCL_FILTER_THRES_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W {
        SDA_FILTER_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn scl_filter_en(&mut self) -> SCL_FILTER_EN_W {
        SCL_FILTER_EN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W {
        SDA_FILTER_THRES_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scl_filter_thres(&mut self) -> SCL_FILTER_THRES_W {
        SCL_FILTER_THRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_FILTER_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_cfg](index.html) module"]
pub struct FILTER_CFG_SPEC;
impl crate::RegisterSpec for FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_cfg::R](R) reader structure"]
impl crate::Readable for FILTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_cfg::W](W) writer structure"]
impl crate::Writable for FILTER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FILTER_CFG to value 0"]
impl crate::Resettable for FILTER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
