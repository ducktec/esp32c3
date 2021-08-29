#[doc = "Register `DMA_IN_DSCR_BF1_CH1` reader"]
pub struct R(crate::R<DMA_IN_DSCR_BF1_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_DSCR_BF1_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_DSCR_BF1_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_DSCR_BF1_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_INLINK_DSCR_BF1_CH1` reader - "]
pub struct DMA_INLINK_DSCR_BF1_CH1_R(crate::FieldReader<u32, u32>);
impl DMA_INLINK_DSCR_BF1_CH1_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_INLINK_DSCR_BF1_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INLINK_DSCR_BF1_CH1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dma_inlink_dscr_bf1_ch1(&self) -> DMA_INLINK_DSCR_BF1_CH1_R {
        DMA_INLINK_DSCR_BF1_CH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "DMA_IN_DSCR_BF1_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_dscr_bf1_ch1](index.html) module"]
pub struct DMA_IN_DSCR_BF1_CH1_SPEC;
impl crate::RegisterSpec for DMA_IN_DSCR_BF1_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_dscr_bf1_ch1::R](R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_BF1_CH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IN_DSCR_BF1_CH1 to value 0"]
impl crate::Resettable for DMA_IN_DSCR_BF1_CH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
