#[doc = "Register `TX_CLKM_CONF` reader"]
pub struct R(crate::R<TX_CLKM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CLKM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CLKM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CLKM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CLKM_CONF` writer"]
pub struct W(crate::W<TX_CLKM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CLKM_CONF_SPEC>;
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
impl From<crate::W<TX_CLKM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CLKM_CONF_SPEC>) -> Self {
        W(writer)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `TX_CLK_SEL` reader - "]
pub struct TX_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl TX_CLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLK_SEL` writer - "]
pub struct TX_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `TX_CLK_ACTIVE` reader - "]
pub struct TX_CLK_ACTIVE_R(crate::FieldReader<bool, bool>);
impl TX_CLK_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CLK_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLK_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLK_ACTIVE` writer - "]
pub struct TX_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `TX_CLKM_DIV_NUM` reader - "]
pub struct TX_CLKM_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl TX_CLKM_DIV_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_CLKM_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLKM_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLKM_DIV_NUM` writer - "]
pub struct TX_CLKM_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLKM_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn tx_clk_sel(&self) -> TX_CLK_SEL_R {
        TX_CLK_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tx_clk_active(&self) -> TX_CLK_ACTIVE_R {
        TX_CLK_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_clkm_div_num(&self) -> TX_CLKM_DIV_NUM_R {
        TX_CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn tx_clk_sel(&mut self) -> TX_CLK_SEL_W {
        TX_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tx_clk_active(&mut self) -> TX_CLK_ACTIVE_W {
        TX_CLK_ACTIVE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_clkm_div_num(&mut self) -> TX_CLKM_DIV_NUM_W {
        TX_CLKM_DIV_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_TX_CLKM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_clkm_conf](index.html) module"]
pub struct TX_CLKM_CONF_SPEC;
impl crate::RegisterSpec for TX_CLKM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_clkm_conf::R](R) reader structure"]
impl crate::Readable for TX_CLKM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_clkm_conf::W](W) writer structure"]
impl crate::Writable for TX_CLKM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CLKM_CONF to value 0"]
impl crate::Resettable for TX_CLKM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
