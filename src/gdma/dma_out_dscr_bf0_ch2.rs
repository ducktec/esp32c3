#[doc = "Register `DMA_OUT_DSCR_BF0_CH2` reader"]
pub struct R(crate::R<DMA_OUT_DSCR_BF0_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_DSCR_BF0_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_DSCR_BF0_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_DSCR_BF0_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTLINK_DSCR_BF0_CH2` reader - "]
pub struct DMA_OUTLINK_DSCR_BF0_CH2_R(crate::FieldReader<u32, u32>);
impl DMA_OUTLINK_DSCR_BF0_CH2_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_OUTLINK_DSCR_BF0_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_DSCR_BF0_CH2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_outlink_dscr_bf0_ch2(&self) -> DMA_OUTLINK_DSCR_BF0_CH2_R {
        DMA_OUTLINK_DSCR_BF0_CH2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DMA_OUT_DSCR_BF0_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_dscr_bf0_ch2](index.html) module"]
pub struct DMA_OUT_DSCR_BF0_CH2_SPEC;
impl crate::RegisterSpec for DMA_OUT_DSCR_BF0_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_dscr_bf0_ch2::R](R) reader structure"]
impl crate::Readable for DMA_OUT_DSCR_BF0_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_DSCR_BF0_CH2 to value 0"]
impl crate::Resettable for DMA_OUT_DSCR_BF0_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
