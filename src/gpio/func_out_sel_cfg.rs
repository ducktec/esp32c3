#[doc = "Register `FUNC%s_OUT_SEL_CFG` reader"]
pub struct R(crate::R<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_OUT_SEL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_OUT_SEL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC%s_OUT_SEL_CFG` writer"]
pub struct W(crate::W<FUNC_OUT_SEL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_OUT_SEL_CFG_SPEC>;
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
impl From<crate::W<FUNC_OUT_SEL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_OUT_SEL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OEN_INV_SEL` reader - "]
pub struct OEN_INV_SEL_R(crate::FieldReader<bool, bool>);
impl OEN_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEN_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEN_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEN_INV_SEL` writer - "]
pub struct OEN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEN_INV_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `OEN_SEL` reader - "]
pub struct OEN_SEL_R(crate::FieldReader<bool, bool>);
impl OEN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEN_SEL` writer - "]
pub struct OEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OEN_SEL_W<'a> {
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
#[doc = "Field `OUT_INV_SEL` reader - "]
pub struct OUT_INV_SEL_R(crate::FieldReader<bool, bool>);
impl OUT_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_INV_SEL` writer - "]
pub struct OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_INV_SEL_W<'a> {
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
#[doc = "Field `OUT_SEL` reader - "]
pub struct OUT_SEL_R(crate::FieldReader<u8, u8>);
impl OUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_SEL` writer - "]
pub struct OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oen_inv_sel(&self) -> OEN_INV_SEL_R {
        OEN_INV_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oen_sel(&self) -> OEN_SEL_R {
        OEN_SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_inv_sel(&self) -> OUT_INV_SEL_R {
        OUT_INV_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn out_sel(&self) -> OUT_SEL_R {
        OUT_SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oen_inv_sel(&mut self) -> OEN_INV_SEL_W {
        OEN_INV_SEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oen_sel(&mut self) -> OEN_SEL_W {
        OEN_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_inv_sel(&mut self) -> OUT_INV_SEL_W {
        OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn out_sel(&mut self) -> OUT_SEL_W {
        OUT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_FUNC%s_OUT_SEL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_out_sel_cfg](index.html) module"]
pub struct FUNC_OUT_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC_OUT_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_out_sel_cfg::R](R) reader structure"]
impl crate::Readable for FUNC_OUT_SEL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_out_sel_cfg::W](W) writer structure"]
impl crate::Writable for FUNC_OUT_SEL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC%s_OUT_SEL_CFG to value 0"]
impl crate::Resettable for FUNC_OUT_SEL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
