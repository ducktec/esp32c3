#[doc = "Register `DMA_OUT_STATE_CH1` reader"]
pub struct R(crate::R<DMA_OUT_STATE_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_STATE_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_STATE_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_STATE_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUT_STATE_CH1` reader - "]
pub struct DMA_OUT_STATE_CH1_R(crate::FieldReader<u8, u8>);
impl DMA_OUT_STATE_CH1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUT_STATE_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_STATE_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_DSCR_STATE_CH1` reader - "]
pub struct DMA_OUT_DSCR_STATE_CH1_R(crate::FieldReader<u8, u8>);
impl DMA_OUT_DSCR_STATE_CH1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUT_DSCR_STATE_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_DSCR_STATE_CH1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTLINK_DSCR_ADDR_CH1` reader - "]
pub struct DMA_OUTLINK_DSCR_ADDR_CH1_R(crate::FieldReader<u32, u32>);
impl DMA_OUTLINK_DSCR_ADDR_CH1_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_OUTLINK_DSCR_ADDR_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_DSCR_ADDR_CH1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dma_out_state_ch1(&self) -> DMA_OUT_STATE_CH1_R {
        DMA_OUT_STATE_CH1_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dma_out_dscr_state_ch1(&self) -> DMA_OUT_DSCR_STATE_CH1_R {
        DMA_OUT_DSCR_STATE_CH1_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn dma_outlink_dscr_addr_ch1(&self) -> DMA_OUTLINK_DSCR_ADDR_CH1_R {
        DMA_OUTLINK_DSCR_ADDR_CH1_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "DMA_OUT_STATE_CH1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_state_ch1](index.html) module"]
pub struct DMA_OUT_STATE_CH1_SPEC;
impl crate::RegisterSpec for DMA_OUT_STATE_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_state_ch1::R](R) reader structure"]
impl crate::Readable for DMA_OUT_STATE_CH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUT_STATE_CH1 to value 0"]
impl crate::Resettable for DMA_OUT_STATE_CH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
