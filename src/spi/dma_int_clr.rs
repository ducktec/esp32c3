#[doc = "Register `DMA_INT_CLR` writer"]
pub struct W(crate::W<DMA_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_CLR_SPEC>;
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
impl From<crate::W<DMA_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP1_INT_CLR` writer - "]
pub struct APP1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP1_INT_CLR_W<'a> {
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
#[doc = "Field `APP2_INT_CLR` writer - "]
pub struct APP2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP2_INT_CLR_W<'a> {
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
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_CLR` writer - "]
pub struct MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_CLR` writer - "]
pub struct MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SLV_CMD_ERR_INT_CLR` writer - "]
pub struct SLV_CMD_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD_ERR_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_CLR` writer - "]
pub struct SLV_BUF_ADDR_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_BUF_ADDR_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `SEG_MAGIC_ERR_INT_CLR` writer - "]
pub struct SEG_MAGIC_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG_MAGIC_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_CLR` writer - "]
pub struct DMA_SEG_TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_TRANS_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `TRANS_DONE_INT_CLR` writer - "]
pub struct TRANS_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SLV_WR_BUF_DONE_INT_CLR` writer - "]
pub struct SLV_WR_BUF_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_BUF_DONE_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SLV_RD_BUF_DONE_INT_CLR` writer - "]
pub struct SLV_RD_BUF_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_BUF_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_WR_DMA_DONE_INT_CLR` writer - "]
pub struct SLV_WR_DMA_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_DMA_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_RD_DMA_DONE_INT_CLR` writer - "]
pub struct SLV_RD_DMA_DONE_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_DMA_DONE_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_CMDA_INT_CLR` writer - "]
pub struct SLV_CMDA_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMDA_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_CMD9_INT_CLR` writer - "]
pub struct SLV_CMD9_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD9_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_CMD8_INT_CLR` writer - "]
pub struct SLV_CMD8_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD8_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_CMD7_INT_CLR` writer - "]
pub struct SLV_CMD7_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD7_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_EN_QPI_INT_CLR` writer - "]
pub struct SLV_EN_QPI_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_EN_QPI_INT_CLR_W<'a> {
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
#[doc = "Field `SLV_EX_QPI_INT_CLR` writer - "]
pub struct SLV_EX_QPI_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_EX_QPI_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_CLR` writer - "]
pub struct DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_CLR` writer - "]
pub struct DMA_INFIFO_FULL_ERR_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_FULL_ERR_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_clr(&mut self) -> APP1_INT_CLR_W {
        APP1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_clr(&mut self) -> APP2_INT_CLR_W {
        APP2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_clr(&mut self) -> MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W {
        MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_clr(&mut self) -> MST_RX_AFIFO_WFULL_ERR_INT_CLR_W {
        MST_RX_AFIFO_WFULL_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_clr(&mut self) -> SLV_CMD_ERR_INT_CLR_W {
        SLV_CMD_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_clr(&mut self) -> SLV_BUF_ADDR_ERR_INT_CLR_W {
        SLV_BUF_ADDR_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn seg_magic_err_int_clr(&mut self) -> SEG_MAGIC_ERR_INT_CLR_W {
        SEG_MAGIC_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_clr(&mut self) -> DMA_SEG_TRANS_DONE_INT_CLR_W {
        DMA_SEG_TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_clr(&mut self) -> TRANS_DONE_INT_CLR_W {
        TRANS_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_clr(&mut self) -> SLV_WR_BUF_DONE_INT_CLR_W {
        SLV_WR_BUF_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_clr(&mut self) -> SLV_RD_BUF_DONE_INT_CLR_W {
        SLV_RD_BUF_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_clr(&mut self) -> SLV_WR_DMA_DONE_INT_CLR_W {
        SLV_WR_DMA_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_clr(&mut self) -> SLV_RD_DMA_DONE_INT_CLR_W {
        SLV_RD_DMA_DONE_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slv_cmda_int_clr(&mut self) -> SLV_CMDA_INT_CLR_W {
        SLV_CMDA_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slv_cmd9_int_clr(&mut self) -> SLV_CMD9_INT_CLR_W {
        SLV_CMD9_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slv_cmd8_int_clr(&mut self) -> SLV_CMD8_INT_CLR_W {
        SLV_CMD8_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slv_cmd7_int_clr(&mut self) -> SLV_CMD7_INT_CLR_W {
        SLV_CMD7_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_en_qpi_int_clr(&mut self) -> SLV_EN_QPI_INT_CLR_W {
        SLV_EN_QPI_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slv_ex_qpi_int_clr(&mut self) -> SLV_EX_QPI_INT_CLR_W {
        SLV_EX_QPI_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_clr(&mut self) -> DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W {
        DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_clr(&mut self) -> DMA_INFIFO_FULL_ERR_INT_CLR_W {
        DMA_INFIFO_FULL_ERR_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_DMA_INT_CLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr](index.html) module"]
pub struct DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_int_clr::W](W) writer structure"]
impl crate::Writable for DMA_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DMA_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
