#[doc = "Register `DMA_OUT_DSCR_BF1_CH0` reader"]
pub struct R(crate::R<DMA_OUT_DSCR_BF1_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_DSCR_BF1_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_DSCR_BF1_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_DSCR_BF1_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTLINK_DSCR_BF1_CH0` reader - "]
pub struct DMA_OUTLINK_DSCR_BF1_CH0_R(crate::FieldReader<u32, u32>);
impl DMA_OUTLINK_DSCR_BF1_CH0_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_OUTLINK_DSCR_BF1_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_DSCR_BF1_CH0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_outlink_dscr_bf1_ch0(&self) -> DMA_OUTLINK_DSCR_BF1_CH0_R {
        DMA_OUTLINK_DSCR_BF1_CH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DMA_OUT_DSCR_BF1_CH0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf1_ch0](index.html) module"]
pub struct DMA_OUT_DSCR_BF1_CH0_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_BF1_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_dscr_bf1_ch0::R](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF1_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_DSCR_BF1_CH0 to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_BF1_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
