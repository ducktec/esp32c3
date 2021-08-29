#[doc = "Register `APB_ADC_CLKM_CONF` reader"]
pub struct R(crate::R<APB_ADC_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_ADC_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_ADC_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_ADC_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_ADC_CLKM_CONF` writer"]
pub struct W(crate::W<APB_ADC_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_ADC_CLKM_CONF_SPEC>;
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
impl From<crate::W<APB_ADC_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_ADC_CLKM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_SEL` reader - "]
pub struct CLK_SEL_R(crate::FieldReader<u8, u8>);
impl CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SEL` writer - "]
pub struct CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - "]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - "]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CLKM_DIV_A` reader - "]
pub struct CLKM_DIV_A_R(crate::FieldReader<u8, u8>);
impl CLKM_DIV_A_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKM_DIV_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKM_DIV_A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKM_DIV_A` writer - "]
pub struct CLKM_DIV_A_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_DIV_A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | ((value as u32 & 0x3f) << 14);
        self.w
    }
}
#[doc = "Field `CLKM_DIV_B` reader - "]
pub struct CLKM_DIV_B_R(crate::FieldReader<u8, u8>);
impl CLKM_DIV_B_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKM_DIV_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKM_DIV_B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKM_DIV_B` writer - "]
pub struct CLKM_DIV_B_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_DIV_B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `CLKM_DIV_NUM` reader - "]
pub struct CLKM_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl CLKM_DIV_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKM_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKM_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKM_DIV_NUM` writer - "]
pub struct CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn clkm_div_a(&self) -> CLKM_DIV_A_R {
        CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn clkm_div_b(&self) -> CLKM_DIV_B_R {
        CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkm_div_num(&self) -> CLKM_DIV_NUM_R {
        CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn clk_sel(&mut self) -> CLK_SEL_W {
        CLK_SEL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn clkm_div_a(&mut self) -> CLKM_DIV_A_W {
        CLKM_DIV_A_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn clkm_div_b(&mut self) -> CLKM_DIV_B_W {
        CLKM_DIV_B_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkm_div_num(&mut self) -> CLKM_DIV_NUM_W {
        CLKM_DIV_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_SARADC_APB_ADC_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_adc_clkm_conf](index.html) module"]
pub struct APB_ADC_CLKM_CONF_SPEC;
impl crate::RegisterSpec for APB_ADC_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_adc_clkm_conf::R](R) reader structure"]
impl crate::Readable for APB_ADC_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_adc_clkm_conf::W](W) writer structure"]
impl crate::Writable for APB_ADC_CLKM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB_ADC_CLKM_CONF to value 0"]
impl crate::Resettable for APB_ADC_CLKM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
