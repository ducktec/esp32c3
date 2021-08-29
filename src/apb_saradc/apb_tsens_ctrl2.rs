#[doc = "Register `APB_TSENS_CTRL2` reader"]
pub struct R(crate::R<APB_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_TSENS_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_TSENS_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_TSENS_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_TSENS_CTRL2` writer"]
pub struct W(crate::W<APB_TSENS_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_TSENS_CTRL2_SPEC>;
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
impl From<crate::W<APB_TSENS_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_TSENS_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSENS_CLK_SEL` reader - "]
pub struct TSENS_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl TSENS_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_SEL` writer - "]
pub struct TSENS_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `TSENS_CLK_INV` reader - "]
pub struct TSENS_CLK_INV_R(crate::FieldReader<bool, bool>);
impl TSENS_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSENS_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_CLK_INV` writer - "]
pub struct TSENS_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_CLK_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TSENS_XPD_FORCE` reader - "]
pub struct TSENS_XPD_FORCE_R(crate::FieldReader<u8, u8>);
impl TSENS_XPD_FORCE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSENS_XPD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_XPD_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_XPD_FORCE` writer - "]
pub struct TSENS_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_XPD_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `TSENS_XPD_WAIT` reader - "]
pub struct TSENS_XPD_WAIT_R(crate::FieldReader<u16, u16>);
impl TSENS_XPD_WAIT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSENS_XPD_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS_XPD_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_XPD_WAIT` writer - "]
pub struct TSENS_XPD_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS_XPD_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TSENS_CLK_SEL_R {
        TSENS_CLK_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_inv(&self) -> TSENS_CLK_INV_R {
        TSENS_CLK_INV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tsens_xpd_force(&self) -> TSENS_XPD_FORCE_R {
        TSENS_XPD_FORCE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&self) -> TSENS_XPD_WAIT_R {
        TSENS_XPD_WAIT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tsens_clk_sel(&mut self) -> TSENS_CLK_SEL_W {
        TSENS_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tsens_clk_inv(&mut self) -> TSENS_CLK_INV_W {
        TSENS_CLK_INV_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tsens_xpd_force(&mut self) -> TSENS_XPD_FORCE_W {
        TSENS_XPD_FORCE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsens_xpd_wait(&mut self) -> TSENS_XPD_WAIT_W {
        TSENS_XPD_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_SARADC_APB_TSENS_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_tsens_ctrl2](index.html) module"]
pub struct APB_TSENS_CTRL2_SPEC;
impl crate::RegisterSpec for APB_TSENS_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_tsens_ctrl2::R](R) reader structure"]
impl crate::Readable for APB_TSENS_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_tsens_ctrl2::W](W) writer structure"]
impl crate::Writable for APB_TSENS_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_TSENS_CTRL2 to value 0"]
impl crate::Resettable for APB_TSENS_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
