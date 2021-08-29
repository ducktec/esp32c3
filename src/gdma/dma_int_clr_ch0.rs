#[doc = "Register `DMA_INT_CLR_CH0` writer"]
pub struct W(crate::W<DMA_INT_CLR_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_CLR_CH0_SPEC>;
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
impl From<crate::W<DMA_INT_CLR_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_CLR_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_OUTFIFO_UDF_CH0_INT_CLR` writer - "]
pub struct DMA_OUTFIFO_UDF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_UDF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_OUTFIFO_OVF_CH0_INT_CLR` writer - "]
pub struct DMA_OUTFIFO_OVF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTFIFO_OVF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_INFIFO_UDF_CH0_INT_CLR` writer - "]
pub struct DMA_INFIFO_UDF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_UDF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_INFIFO_OVF_CH0_INT_CLR` writer - "]
pub struct DMA_INFIFO_OVF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_INFIFO_OVF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_OUT_TOTAL_EOF_CH0_INT_CLR` writer - "]
pub struct DMA_OUT_TOTAL_EOF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_TOTAL_EOF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_IN_DSCR_EMPTY_CH0_INT_CLR` writer - "]
pub struct DMA_IN_DSCR_EMPTY_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DSCR_EMPTY_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_OUT_DSCR_ERR_CH0_INT_CLR` writer - "]
pub struct DMA_OUT_DSCR_ERR_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_DSCR_ERR_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_IN_DSCR_ERR_CH0_INT_CLR` writer - "]
pub struct DMA_IN_DSCR_ERR_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DSCR_ERR_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_OUT_EOF_CH0_INT_CLR` writer - "]
pub struct DMA_OUT_EOF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_EOF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_OUT_DONE_CH0_INT_CLR` writer - "]
pub struct DMA_OUT_DONE_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUT_DONE_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_IN_ERR_EOF_CH0_INT_CLR` writer - "]
pub struct DMA_IN_ERR_EOF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_ERR_EOF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_IN_SUC_EOF_CH0_INT_CLR` writer - "]
pub struct DMA_IN_SUC_EOF_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_SUC_EOF_CH0_INT_CLR_W<'a> {
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
#[doc = "Field `DMA_IN_DONE_CH0_INT_CLR` writer - "]
pub struct DMA_IN_DONE_CH0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DONE_CH0_INT_CLR_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_outfifo_udf_ch0_int_clr(&mut self) -> DMA_OUTFIFO_UDF_CH0_INT_CLR_W {
        DMA_OUTFIFO_UDF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dma_outfifo_ovf_ch0_int_clr(&mut self) -> DMA_OUTFIFO_OVF_CH0_INT_CLR_W {
        DMA_OUTFIFO_OVF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dma_infifo_udf_ch0_int_clr(&mut self) -> DMA_INFIFO_UDF_CH0_INT_CLR_W {
        DMA_INFIFO_UDF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dma_infifo_ovf_ch0_int_clr(&mut self) -> DMA_INFIFO_OVF_CH0_INT_CLR_W {
        DMA_INFIFO_OVF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_out_total_eof_ch0_int_clr(&mut self) -> DMA_OUT_TOTAL_EOF_CH0_INT_CLR_W {
        DMA_OUT_TOTAL_EOF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dma_in_dscr_empty_ch0_int_clr(&mut self) -> DMA_IN_DSCR_EMPTY_CH0_INT_CLR_W {
        DMA_IN_DSCR_EMPTY_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dma_out_dscr_err_ch0_int_clr(&mut self) -> DMA_OUT_DSCR_ERR_CH0_INT_CLR_W {
        DMA_OUT_DSCR_ERR_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dma_in_dscr_err_ch0_int_clr(&mut self) -> DMA_IN_DSCR_ERR_CH0_INT_CLR_W {
        DMA_IN_DSCR_ERR_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dma_out_eof_ch0_int_clr(&mut self) -> DMA_OUT_EOF_CH0_INT_CLR_W {
        DMA_OUT_EOF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_out_done_ch0_int_clr(&mut self) -> DMA_OUT_DONE_CH0_INT_CLR_W {
        DMA_OUT_DONE_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_in_err_eof_ch0_int_clr(&mut self) -> DMA_IN_ERR_EOF_CH0_INT_CLR_W {
        DMA_IN_ERR_EOF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_in_suc_eof_ch0_int_clr(&mut self) -> DMA_IN_SUC_EOF_CH0_INT_CLR_W {
        DMA_IN_SUC_EOF_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_in_done_ch0_int_clr(&mut self) -> DMA_IN_DONE_CH0_INT_CLR_W {
        DMA_IN_DONE_CH0_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INT_CLR_CH0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_clr_ch0](index.html) module"]
pub struct DMA_INT_CLR_CH0_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_int_clr_ch0::W](W) writer structure"]
impl crate::Writable for DMA_INT_CLR_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT_CLR_CH0 to value 0"]
impl crate::Resettable for DMA_INT_CLR_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
