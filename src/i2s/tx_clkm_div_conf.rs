#[doc = "Register `TX_CLKM_DIV_CONF` reader"]
pub struct R(crate::R<TX_CLKM_DIV_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CLKM_DIV_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CLKM_DIV_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CLKM_DIV_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CLKM_DIV_CONF` writer"]
pub struct W(crate::W<TX_CLKM_DIV_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CLKM_DIV_CONF_SPEC>;
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
impl From<crate::W<TX_CLKM_DIV_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CLKM_DIV_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CLKM_DIV_YN1` reader - "]
pub struct TX_CLKM_DIV_YN1_R(crate::FieldReader<bool, bool>);
impl TX_CLKM_DIV_YN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CLKM_DIV_YN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLKM_DIV_YN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLKM_DIV_YN1` writer - "]
pub struct TX_CLKM_DIV_YN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLKM_DIV_YN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `TX_CLKM_DIV_X` reader - "]
pub struct TX_CLKM_DIV_X_R(crate::FieldReader<u16, u16>);
impl TX_CLKM_DIV_X_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_CLKM_DIV_X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLKM_DIV_X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLKM_DIV_X` writer - "]
pub struct TX_CLKM_DIV_X_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLKM_DIV_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 18)) | ((value as u32 & 0x01ff) << 18);
        self.w
    }
}
#[doc = "Field `TX_CLKM_DIV_Y` reader - "]
pub struct TX_CLKM_DIV_Y_R(crate::FieldReader<u16, u16>);
impl TX_CLKM_DIV_Y_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_CLKM_DIV_Y_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLKM_DIV_Y_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLKM_DIV_Y` writer - "]
pub struct TX_CLKM_DIV_Y_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLKM_DIV_Y_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | ((value as u32 & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Field `TX_CLKM_DIV_Z` reader - "]
pub struct TX_CLKM_DIV_Z_R(crate::FieldReader<u16, u16>);
impl TX_CLKM_DIV_Z_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_CLKM_DIV_Z_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CLKM_DIV_Z_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CLKM_DIV_Z` writer - "]
pub struct TX_CLKM_DIV_Z_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLKM_DIV_Z_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tx_clkm_div_yn1(&self) -> TX_CLKM_DIV_YN1_R {
        TX_CLKM_DIV_YN1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 18:26"]
    #[inline(always)]
    pub fn tx_clkm_div_x(&self) -> TX_CLKM_DIV_X_R {
        TX_CLKM_DIV_X_R::new(((self.bits >> 18) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn tx_clkm_div_y(&self) -> TX_CLKM_DIV_Y_R {
        TX_CLKM_DIV_Y_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn tx_clkm_div_z(&self) -> TX_CLKM_DIV_Z_R {
        TX_CLKM_DIV_Z_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tx_clkm_div_yn1(&mut self) -> TX_CLKM_DIV_YN1_W {
        TX_CLKM_DIV_YN1_W { w: self }
    }
    #[doc = "Bits 18:26"]
    #[inline(always)]
    pub fn tx_clkm_div_x(&mut self) -> TX_CLKM_DIV_X_W {
        TX_CLKM_DIV_X_W { w: self }
    }
    #[doc = "Bits 9:17"]
    #[inline(always)]
    pub fn tx_clkm_div_y(&mut self) -> TX_CLKM_DIV_Y_W {
        TX_CLKM_DIV_Y_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn tx_clkm_div_z(&mut self) -> TX_CLKM_DIV_Z_W {
        TX_CLKM_DIV_Z_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_TX_CLKM_DIV_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_clkm_div_conf](index.html) module"]
pub struct TX_CLKM_DIV_CONF_SPEC;
impl crate::RegisterSpec for TX_CLKM_DIV_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_clkm_div_conf::R](R) reader structure"]
impl crate::Readable for TX_CLKM_DIV_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_clkm_div_conf::W](W) writer structure"]
impl crate::Writable for TX_CLKM_DIV_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CLKM_DIV_CONF to value 0"]
impl crate::Resettable for TX_CLKM_DIV_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
