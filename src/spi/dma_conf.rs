#[doc = "Register `DMA_CONF` reader"]
pub struct R(crate::R<DMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CONF` writer"]
pub struct W(crate::W<DMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONF_SPEC>;
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
impl From<crate::W<DMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_AFIFO_RST` writer - "]
pub struct DMA_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AFIFO_RST_W<'a> {
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
#[doc = "Field `BUF_AFIFO_RST` writer - "]
pub struct BUF_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_AFIFO_RST_W<'a> {
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
#[doc = "Field `RX_AFIFO_RST` writer - "]
pub struct RX_AFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_AFIFO_RST_W<'a> {
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
#[doc = "Field `DMA_TX_ENA` reader - "]
pub struct DMA_TX_ENA_R(crate::FieldReader<bool, bool>);
impl DMA_TX_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_TX_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_TX_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_TX_ENA` writer - "]
pub struct DMA_TX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_ENA_W<'a> {
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
#[doc = "Field `DMA_RX_ENA` reader - "]
pub struct DMA_RX_ENA_R(crate::FieldReader<bool, bool>);
impl DMA_RX_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_RX_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RX_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RX_ENA` writer - "]
pub struct DMA_RX_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_ENA_W<'a> {
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
#[doc = "Field `RX_EOF_EN` reader - "]
pub struct RX_EOF_EN_R(crate::FieldReader<bool, bool>);
impl RX_EOF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_EOF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_EOF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EOF_EN` writer - "]
pub struct RX_EOF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EOF_EN_W<'a> {
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
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` reader - "]
pub struct SLV_TX_SEG_TRANS_CLR_EN_R(crate::FieldReader<bool, bool>);
impl SLV_TX_SEG_TRANS_CLR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_TX_SEG_TRANS_CLR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_TX_SEG_TRANS_CLR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` writer - "]
pub struct SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_TX_SEG_TRANS_CLR_EN_W<'a> {
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
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` reader - "]
pub struct SLV_RX_SEG_TRANS_CLR_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RX_SEG_TRANS_CLR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RX_SEG_TRANS_CLR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RX_SEG_TRANS_CLR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` writer - "]
pub struct SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RX_SEG_TRANS_CLR_EN_W<'a> {
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
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` reader - "]
pub struct DMA_SLV_SEG_TRANS_EN_R(crate::FieldReader<bool, bool>);
impl DMA_SLV_SEG_TRANS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_SLV_SEG_TRANS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SLV_SEG_TRANS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` writer - "]
pub struct DMA_SLV_SEG_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SLV_SEG_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma_tx_ena(&self) -> DMA_TX_ENA_R {
        DMA_TX_ENA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dma_rx_ena(&self) -> DMA_RX_ENA_R {
        DMA_RX_ENA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_eof_en(&self) -> RX_EOF_EN_R {
        RX_EOF_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&self) -> SLV_TX_SEG_TRANS_CLR_EN_R {
        SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&self) -> SLV_RX_SEG_TRANS_CLR_EN_R {
        SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&self) -> DMA_SLV_SEG_TRANS_EN_R {
        DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma_afifo_rst(&mut self) -> DMA_AFIFO_RST_W {
        DMA_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn buf_afifo_rst(&mut self) -> BUF_AFIFO_RST_W {
        BUF_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_afifo_rst(&mut self) -> RX_AFIFO_RST_W {
        RX_AFIFO_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma_tx_ena(&mut self) -> DMA_TX_ENA_W {
        DMA_TX_ENA_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dma_rx_ena(&mut self) -> DMA_RX_ENA_W {
        DMA_RX_ENA_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rx_eof_en(&mut self) -> RX_EOF_EN_W {
        RX_EOF_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&mut self) -> SLV_TX_SEG_TRANS_CLR_EN_W {
        SLV_TX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&mut self) -> SLV_RX_SEG_TRANS_CLR_EN_W {
        SLV_RX_SEG_TRANS_CLR_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&mut self) -> DMA_SLV_SEG_TRANS_EN_W {
        DMA_SLV_SEG_TRANS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_DMA_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_conf](index.html) module"]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_conf::R](R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_conf::W](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CONF to value 0"]
impl crate::Resettable for DMA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
