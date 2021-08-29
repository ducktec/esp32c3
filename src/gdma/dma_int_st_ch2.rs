#[doc = "Register `DMA_INT_ST_CH2` reader"]
pub struct R(crate::R<DMA_INT_ST_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ST_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ST_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ST_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTFIFO_UDF_CH2_INT_ST` reader - "]
pub struct DMA_OUTFIFO_UDF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_UDF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_UDF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_UDF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_OVF_CH2_INT_ST` reader - "]
pub struct DMA_OUTFIFO_OVF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_OVF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_OVF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_OVF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_INFIFO_UDF_CH2_INT_ST` reader - "]
pub struct DMA_INFIFO_UDF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_INFIFO_UDF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_INFIFO_UDF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INFIFO_UDF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_INFIFO_OVF_CH2_INT_ST` reader - "]
pub struct DMA_INFIFO_OVF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_INFIFO_OVF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_INFIFO_OVF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INFIFO_OVF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_TOTAL_EOF_CH2_INT_ST` reader - "]
pub struct DMA_OUT_TOTAL_EOF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_TOTAL_EOF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_TOTAL_EOF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_TOTAL_EOF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DSCR_EMPTY_CH2_INT_ST` reader - "]
pub struct DMA_IN_DSCR_EMPTY_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_IN_DSCR_EMPTY_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_DSCR_EMPTY_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DSCR_EMPTY_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_DSCR_ERR_CH2_INT_ST` reader - "]
pub struct DMA_OUT_DSCR_ERR_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_DSCR_ERR_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_DSCR_ERR_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_DSCR_ERR_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DSCR_ERR_CH2_INT_ST` reader - "]
pub struct DMA_IN_DSCR_ERR_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_IN_DSCR_ERR_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_DSCR_ERR_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DSCR_ERR_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_EOF_CH2_INT_ST` reader - "]
pub struct DMA_OUT_EOF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_EOF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_EOF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_EOF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_DONE_CH2_INT_ST` reader - "]
pub struct DMA_OUT_DONE_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_DONE_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_DONE_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_DONE_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_ERR_EOF_CH2_INT_ST` reader - "]
pub struct DMA_IN_ERR_EOF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_IN_ERR_EOF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_ERR_EOF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_ERR_EOF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_SUC_EOF_CH2_INT_ST` reader - "]
pub struct DMA_IN_SUC_EOF_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_IN_SUC_EOF_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_SUC_EOF_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_SUC_EOF_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DONE_CH2_INT_ST` reader - "]
pub struct DMA_IN_DONE_CH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DMA_IN_DONE_CH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_DONE_CH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DONE_CH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_outfifo_udf_ch2_int_st(&self) -> DMA_OUTFIFO_UDF_CH2_INT_ST_R {
        DMA_OUTFIFO_UDF_CH2_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_outfifo_ovf_ch2_int_st(&self) -> DMA_OUTFIFO_OVF_CH2_INT_ST_R {
        DMA_OUTFIFO_OVF_CH2_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_infifo_udf_ch2_int_st(&self) -> DMA_INFIFO_UDF_CH2_INT_ST_R {
        DMA_INFIFO_UDF_CH2_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_infifo_ovf_ch2_int_st(&self) -> DMA_INFIFO_OVF_CH2_INT_ST_R {
        DMA_INFIFO_OVF_CH2_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_out_total_eof_ch2_int_st(&self) -> DMA_OUT_TOTAL_EOF_CH2_INT_ST_R {
        DMA_OUT_TOTAL_EOF_CH2_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_in_dscr_empty_ch2_int_st(&self) -> DMA_IN_DSCR_EMPTY_CH2_INT_ST_R {
        DMA_IN_DSCR_EMPTY_CH2_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_out_dscr_err_ch2_int_st(&self) -> DMA_OUT_DSCR_ERR_CH2_INT_ST_R {
        DMA_OUT_DSCR_ERR_CH2_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_in_dscr_err_ch2_int_st(&self) -> DMA_IN_DSCR_ERR_CH2_INT_ST_R {
        DMA_IN_DSCR_ERR_CH2_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_out_eof_ch2_int_st(&self) -> DMA_OUT_EOF_CH2_INT_ST_R {
        DMA_OUT_EOF_CH2_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_done_ch2_int_st(&self) -> DMA_OUT_DONE_CH2_INT_ST_R {
        DMA_OUT_DONE_CH2_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_in_err_eof_ch2_int_st(&self) -> DMA_IN_ERR_EOF_CH2_INT_ST_R {
        DMA_IN_ERR_EOF_CH2_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_suc_eof_ch2_int_st(&self) -> DMA_IN_SUC_EOF_CH2_INT_ST_R {
        DMA_IN_SUC_EOF_CH2_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_done_ch2_int_st(&self) -> DMA_IN_DONE_CH2_INT_ST_R {
        DMA_IN_DONE_CH2_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DMA_INT_ST_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st_ch2](index.html) module"]
pub struct DMA_INT_ST_CH2_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_st_ch2::R](R) reader structure"]
impl crate::Readable for DMA_INT_ST_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INT_ST_CH2 to value 0"]
impl crate::Resettable for DMA_INT_ST_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
