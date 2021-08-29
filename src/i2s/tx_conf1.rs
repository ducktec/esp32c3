#[doc = "Register `TX_CONF1` reader"]
pub struct R(crate::R<TX_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CONF1` writer"]
pub struct W(crate::W<TX_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CONF1_SPEC>;
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
impl From<crate::W<TX_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_MSB_SHIFT` reader - "]
pub struct TX_MSB_SHIFT_R(crate::FieldReader<bool, bool>);
impl TX_MSB_SHIFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_MSB_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MSB_SHIFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MSB_SHIFT` writer - "]
pub struct TX_MSB_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MSB_SHIFT_W<'a> {
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
#[doc = "Field `TX_TDM_CHAN_BITS` reader - "]
pub struct TX_TDM_CHAN_BITS_R(crate::FieldReader<u8, u8>);
impl TX_TDM_CHAN_BITS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_TDM_CHAN_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TDM_CHAN_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TDM_CHAN_BITS` writer - "]
pub struct TX_TDM_CHAN_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TDM_CHAN_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `TX_HALF_SAMPLE_BITS` reader - "]
pub struct TX_HALF_SAMPLE_BITS_R(crate::FieldReader<u8, u8>);
impl TX_HALF_SAMPLE_BITS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_HALF_SAMPLE_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HALF_SAMPLE_BITS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HALF_SAMPLE_BITS` writer - "]
pub struct TX_HALF_SAMPLE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HALF_SAMPLE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `TX_BITS_MOD` reader - "]
pub struct TX_BITS_MOD_R(crate::FieldReader<u8, u8>);
impl TX_BITS_MOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_BITS_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BITS_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BITS_MOD` writer - "]
pub struct TX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | ((value as u32 & 0x1f) << 13);
        self.w
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` reader - "]
pub struct TX_BCK_DIV_NUM_R(crate::FieldReader<u8, u8>);
impl TX_BCK_DIV_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_BCK_DIV_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BCK_DIV_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BCK_DIV_NUM` writer - "]
pub struct TX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `TX_TDM_WS_WIDTH` reader - "]
pub struct TX_TDM_WS_WIDTH_R(crate::FieldReader<u8, u8>);
impl TX_TDM_WS_WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_TDM_WS_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TDM_WS_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TDM_WS_WIDTH` writer - "]
pub struct TX_TDM_WS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TDM_WS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tx_msb_shift(&self) -> TX_MSB_SHIFT_R {
        TX_MSB_SHIFT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn tx_tdm_chan_bits(&self) -> TX_TDM_CHAN_BITS_R {
        TX_TDM_CHAN_BITS_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn tx_half_sample_bits(&self) -> TX_HALF_SAMPLE_BITS_R {
        TX_HALF_SAMPLE_BITS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> TX_BITS_MOD_R {
        TX_BITS_MOD_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> TX_BCK_DIV_NUM_R {
        TX_BCK_DIV_NUM_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_tdm_ws_width(&self) -> TX_TDM_WS_WIDTH_R {
        TX_TDM_WS_WIDTH_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tx_msb_shift(&mut self) -> TX_MSB_SHIFT_W {
        TX_MSB_SHIFT_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn tx_tdm_chan_bits(&mut self) -> TX_TDM_CHAN_BITS_W {
        TX_TDM_CHAN_BITS_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn tx_half_sample_bits(&mut self) -> TX_HALF_SAMPLE_BITS_W {
        TX_HALF_SAMPLE_BITS_W { w: self }
    }
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&mut self) -> TX_BITS_MOD_W {
        TX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn tx_bck_div_num(&mut self) -> TX_BCK_DIV_NUM_W {
        TX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_tdm_ws_width(&mut self) -> TX_TDM_WS_WIDTH_W {
        TX_TDM_WS_WIDTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_TX_CONF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_conf1](index.html) module"]
pub struct TX_CONF1_SPEC;
impl crate::RegisterSpec for TX_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_conf1::R](R) reader structure"]
impl crate::Readable for TX_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_conf1::W](W) writer structure"]
impl crate::Writable for TX_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CONF1 to value 0"]
impl crate::Resettable for TX_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
