#[doc = "Register `DMA_INT_ST` reader"]
pub struct R(crate::R<DMA_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APP1_INT_ST` reader - "]
pub struct APP1_INT_ST_R(crate::FieldReader<bool, bool>);
impl APP1_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        APP1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP2_INT_ST` reader - "]
pub struct APP2_INT_ST_R(crate::FieldReader<bool, bool>);
impl APP2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        APP2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_ST` reader - "]
pub struct MST_TX_AFIFO_REMPTY_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_TX_AFIFO_REMPTY_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_ST` reader - "]
pub struct MST_RX_AFIFO_WFULL_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_RX_AFIFO_WFULL_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD_ERR_INT_ST` reader - "]
pub struct SLV_CMD_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_ST` reader - "]
pub struct SLV_BUF_ADDR_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_BUF_ADDR_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_BUF_ADDR_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_BUF_ADDR_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEG_MAGIC_ERR_INT_ST` reader - "]
pub struct SEG_MAGIC_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl SEG_MAGIC_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEG_MAGIC_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEG_MAGIC_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_ST` reader - "]
pub struct DMA_SEG_TRANS_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_SEG_TRANS_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_SEG_TRANS_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SEG_TRANS_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_DONE_INT_ST` reader - "]
pub struct TRANS_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl TRANS_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_BUF_DONE_INT_ST` reader - "]
pub struct SLV_WR_BUF_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_WR_BUF_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_BUF_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_BUF_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_BUF_DONE_INT_ST` reader - "]
pub struct SLV_RD_BUF_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_RD_BUF_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RD_BUF_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_BUF_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WR_DMA_DONE_INT_ST` reader - "]
pub struct SLV_WR_DMA_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_WR_DMA_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WR_DMA_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WR_DMA_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RD_DMA_DONE_INT_ST` reader - "]
pub struct SLV_RD_DMA_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_RD_DMA_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RD_DMA_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RD_DMA_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMDA_INT_ST` reader - "]
pub struct SLV_CMDA_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMDA_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMDA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMDA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD9_INT_ST` reader - "]
pub struct SLV_CMD9_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD9_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD9_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD9_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD8_INT_ST` reader - "]
pub struct SLV_CMD8_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD8_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD8_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD8_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_CMD7_INT_ST` reader - "]
pub struct SLV_CMD7_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_CMD7_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_CMD7_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_CMD7_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_EN_QPI_INT_ST` reader - "]
pub struct SLV_EN_QPI_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_EN_QPI_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_EN_QPI_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_EN_QPI_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_EX_QPI_INT_ST` reader - "]
pub struct SLV_EX_QPI_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_EX_QPI_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_EX_QPI_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_EX_QPI_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_ST` reader - "]
pub struct DMA_OUTFIFO_EMPTY_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_EMPTY_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_ST` reader - "]
pub struct DMA_INFIFO_FULL_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_INFIFO_FULL_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_INFIFO_FULL_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INFIFO_FULL_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_st(&self) -> APP1_INT_ST_R {
        APP1_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_st(&self) -> APP2_INT_ST_R {
        APP2_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_st(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_st(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_st(&self) -> SLV_CMD_ERR_INT_ST_R {
        SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_st(&self) -> SLV_BUF_ADDR_ERR_INT_ST_R {
        SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn seg_magic_err_int_st(&self) -> SEG_MAGIC_ERR_INT_ST_R {
        SEG_MAGIC_ERR_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_st(&self) -> DMA_SEG_TRANS_DONE_INT_ST_R {
        DMA_SEG_TRANS_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_st(&self) -> TRANS_DONE_INT_ST_R {
        TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_st(&self) -> SLV_WR_BUF_DONE_INT_ST_R {
        SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_st(&self) -> SLV_RD_BUF_DONE_INT_ST_R {
        SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_st(&self) -> SLV_WR_DMA_DONE_INT_ST_R {
        SLV_WR_DMA_DONE_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_st(&self) -> SLV_RD_DMA_DONE_INT_ST_R {
        SLV_RD_DMA_DONE_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn slv_cmda_int_st(&self) -> SLV_CMDA_INT_ST_R {
        SLV_CMDA_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slv_cmd9_int_st(&self) -> SLV_CMD9_INT_ST_R {
        SLV_CMD9_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slv_cmd8_int_st(&self) -> SLV_CMD8_INT_ST_R {
        SLV_CMD8_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slv_cmd7_int_st(&self) -> SLV_CMD7_INT_ST_R {
        SLV_CMD7_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_en_qpi_int_st(&self) -> SLV_EN_QPI_INT_ST_R {
        SLV_EN_QPI_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slv_ex_qpi_int_st(&self) -> SLV_EX_QPI_INT_ST_R {
        SLV_EX_QPI_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_st(&self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
        DMA_OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_st(&self) -> DMA_INFIFO_FULL_ERR_INT_ST_R {
        DMA_INFIFO_FULL_ERR_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SPI_DMA_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st](index.html) module"]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_st::R](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
