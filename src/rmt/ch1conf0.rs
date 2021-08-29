#[doc = "Register `CH1CONF0` reader"]
pub struct R(crate::R<CH1CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CONF0` writer"]
pub struct W(crate::W<CH1CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CONF0_SPEC>;
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
impl From<crate::W<CH1CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONF_UPDATE_CH1` writer - "]
pub struct CONF_UPDATE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CONF_UPDATE_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `AFIFO_RST_CH1` writer - "]
pub struct AFIFO_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIFO_RST_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH1` reader - "]
pub struct CARRIER_OUT_LV_CH1_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV_CH1` writer - "]
pub struct CARRIER_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CARRIER_EN_CH1` reader - "]
pub struct CARRIER_EN_CH1_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN_CH1` writer - "]
pub struct CARRIER_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CARRIER_EFF_EN_CH1` reader - "]
pub struct CARRIER_EFF_EN_CH1_R(crate::FieldReader<bool, bool>);
impl CARRIER_EFF_EN_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EFF_EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EFF_EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EFF_EN_CH1` writer - "]
pub struct CARRIER_EFF_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EFF_EN_CH1_W<'a> {
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
#[doc = "Field `MEM_SIZE_CH1` reader - "]
pub struct MEM_SIZE_CH1_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_CH1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE_CH1` writer - "]
pub struct MEM_SIZE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `DIV_CNT_CH1` reader - "]
pub struct DIV_CNT_CH1_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_CH1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT_CH1` writer - "]
pub struct DIV_CNT_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TX_STOP_CH1` reader - "]
pub struct TX_STOP_CH1_R(crate::FieldReader<bool, bool>);
impl TX_STOP_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_STOP_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_STOP_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_STOP_CH1` writer - "]
pub struct TX_STOP_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STOP_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `IDLE_OUT_EN_CH1` reader - "]
pub struct IDLE_OUT_EN_CH1_R(crate::FieldReader<bool, bool>);
impl IDLE_OUT_EN_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_OUT_EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_OUT_EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_OUT_EN_CH1` writer - "]
pub struct IDLE_OUT_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IDLE_OUT_LV_CH1` reader - "]
pub struct IDLE_OUT_LV_CH1_R(crate::FieldReader<bool, bool>);
impl IDLE_OUT_LV_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_OUT_LV_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_OUT_LV_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_OUT_LV_CH1` writer - "]
pub struct IDLE_OUT_LV_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_OUT_LV_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MEM_TX_WRAP_EN_CH1` reader - "]
pub struct MEM_TX_WRAP_EN_CH1_R(crate::FieldReader<bool, bool>);
impl MEM_TX_WRAP_EN_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_TX_WRAP_EN_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TX_WRAP_EN_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_TX_WRAP_EN_CH1` writer - "]
pub struct MEM_TX_WRAP_EN_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_TX_WRAP_EN_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TX_CONTI_MODE_CH1` reader - "]
pub struct TX_CONTI_MODE_CH1_R(crate::FieldReader<bool, bool>);
impl TX_CONTI_MODE_CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CONTI_MODE_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CONTI_MODE_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CONTI_MODE_CH1` writer - "]
pub struct TX_CONTI_MODE_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CONTI_MODE_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `APB_MEM_RST_CH1` writer - "]
pub struct APB_MEM_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_RST_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MEM_RD_RST_CH1` writer - "]
pub struct MEM_RD_RST_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_RD_RST_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TX_START_CH1` writer - "]
pub struct TX_START_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_CH1_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn carrier_out_lv_ch1(&self) -> CARRIER_OUT_LV_CH1_R {
        CARRIER_OUT_LV_CH1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn carrier_en_ch1(&self) -> CARRIER_EN_CH1_R {
        CARRIER_EN_CH1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn carrier_eff_en_ch1(&self) -> CARRIER_EFF_EN_CH1_R {
        CARRIER_EFF_EN_CH1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mem_size_ch1(&self) -> MEM_SIZE_CH1_R {
        MEM_SIZE_CH1_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn div_cnt_ch1(&self) -> DIV_CNT_CH1_R {
        DIV_CNT_CH1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_stop_ch1(&self) -> TX_STOP_CH1_R {
        TX_STOP_CH1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idle_out_en_ch1(&self) -> IDLE_OUT_EN_CH1_R {
        IDLE_OUT_EN_CH1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idle_out_lv_ch1(&self) -> IDLE_OUT_LV_CH1_R {
        IDLE_OUT_LV_CH1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch1(&self) -> MEM_TX_WRAP_EN_CH1_R {
        MEM_TX_WRAP_EN_CH1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_conti_mode_ch1(&self) -> TX_CONTI_MODE_CH1_R {
        TX_CONTI_MODE_CH1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn conf_update_ch1(&mut self) -> CONF_UPDATE_CH1_W {
        CONF_UPDATE_CH1_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn afifo_rst_ch1(&mut self) -> AFIFO_RST_CH1_W {
        AFIFO_RST_CH1_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn carrier_out_lv_ch1(&mut self) -> CARRIER_OUT_LV_CH1_W {
        CARRIER_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn carrier_en_ch1(&mut self) -> CARRIER_EN_CH1_W {
        CARRIER_EN_CH1_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn carrier_eff_en_ch1(&mut self) -> CARRIER_EFF_EN_CH1_W {
        CARRIER_EFF_EN_CH1_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn mem_size_ch1(&mut self) -> MEM_SIZE_CH1_W {
        MEM_SIZE_CH1_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn div_cnt_ch1(&mut self) -> DIV_CNT_CH1_W {
        DIV_CNT_CH1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_stop_ch1(&mut self) -> TX_STOP_CH1_W {
        TX_STOP_CH1_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn idle_out_en_ch1(&mut self) -> IDLE_OUT_EN_CH1_W {
        IDLE_OUT_EN_CH1_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn idle_out_lv_ch1(&mut self) -> IDLE_OUT_LV_CH1_W {
        IDLE_OUT_LV_CH1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_tx_wrap_en_ch1(&mut self) -> MEM_TX_WRAP_EN_CH1_W {
        MEM_TX_WRAP_EN_CH1_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_conti_mode_ch1(&mut self) -> TX_CONTI_MODE_CH1_W {
        TX_CONTI_MODE_CH1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn apb_mem_rst_ch1(&mut self) -> APB_MEM_RST_CH1_W {
        APB_MEM_RST_CH1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mem_rd_rst_ch1(&mut self) -> MEM_RD_RST_CH1_W {
        MEM_RD_RST_CH1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_start_ch1(&mut self) -> TX_START_CH1_W {
        TX_START_CH1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH1CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1conf0](index.html) module"]
pub struct CH1CONF0_SPEC;
impl crate::RegisterSpec for CH1CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1conf0::R](R) reader structure"]
impl crate::Readable for CH1CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1conf0::W](W) writer structure"]
impl crate::Writable for CH1CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1CONF0 to value 0"]
impl crate::Resettable for CH1CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
