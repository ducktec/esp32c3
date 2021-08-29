#[doc = "Register `CH1_TX_LIM` reader"]
pub struct R(crate::R<CH1_TX_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1_TX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1_TX_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1_TX_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1_TX_LIM` writer"]
pub struct W(crate::W<CH1_TX_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1_TX_LIM_SPEC>;
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
impl From<crate::W<CH1_TX_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1_TX_LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOP_COUNT_RESET_CH1` writer - "]
pub struct LOOP_COUNT_RESET_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_COUNT_RESET_CH1_W<'a> {
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
#[doc = "Field `TX_LOOP_CNT_EN_CH1` reader - "]
pub struct TX_LOOP_CNT_EN_CH1_R(crate::FieldReader<bool, bool>);
impl TX_LOOP_CNT_EN_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_LOOP_CNT_EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LOOP_CNT_EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LOOP_CNT_EN_CH1` writer - "]
pub struct TX_LOOP_CNT_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOOP_CNT_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TX_LOOP_NUM_CH1` reader - "]
pub struct TX_LOOP_NUM_CH1_R(crate::FieldReader<u16, u16>);
impl TX_LOOP_NUM_CH1_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_LOOP_NUM_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LOOP_NUM_CH1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LOOP_NUM_CH1` writer - "]
pub struct TX_LOOP_NUM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOOP_NUM_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | ((value as u32 & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Field `TX_LIM_CH1` reader - "]
pub struct TX_LIM_CH1_R(crate::FieldReader<u16, u16>);
impl TX_LIM_CH1_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_LIM_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LIM_CH1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LIM_CH1` writer - "]
pub struct TX_LIM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch1(&self) -> TX_LOOP_CNT_EN_CH1_R {
        TX_LOOP_CNT_EN_CH1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn tx_loop_num_ch1(&self) -> TX_LOOP_NUM_CH1_R {
        TX_LOOP_NUM_CH1_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn tx_lim_ch1(&self) -> TX_LIM_CH1_R {
        TX_LIM_CH1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn loop_count_reset_ch1(&mut self) -> LOOP_COUNT_RESET_CH1_W {
        LOOP_COUNT_RESET_CH1_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_loop_cnt_en_ch1(&mut self) -> TX_LOOP_CNT_EN_CH1_W {
        TX_LOOP_CNT_EN_CH1_W { w: self }
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn tx_loop_num_ch1(&mut self) -> TX_LOOP_NUM_CH1_W {
        TX_LOOP_NUM_CH1_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn tx_lim_ch1(&mut self) -> TX_LIM_CH1_W {
        TX_LIM_CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH1_TX_LIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_tx_lim](index.html) module"]
pub struct CH1_TX_LIM_SPEC;
impl crate::RegisterSpec for CH1_TX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1_tx_lim::R](R) reader structure"]
impl crate::Readable for CH1_TX_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1_tx_lim::W](W) writer structure"]
impl crate::Writable for CH1_TX_LIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1_TX_LIM to value 0"]
impl crate::Resettable for CH1_TX_LIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
