#[doc = "Register `DMA_IN_STATE_CH2` reader"]
pub struct R(crate::R<DMA_IN_STATE_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_STATE_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_STATE_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_STATE_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_IN_STATE_CH2` reader - "]
pub struct DMA_IN_STATE_CH2_R(crate::FieldReader<u8, u8>);
impl DMA_IN_STATE_CH2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_IN_STATE_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_STATE_CH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DSCR_STATE_CH2` reader - "]
pub struct DMA_IN_DSCR_STATE_CH2_R(crate::FieldReader<u8, u8>);
impl DMA_IN_DSCR_STATE_CH2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_IN_DSCR_STATE_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DSCR_STATE_CH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_INLINK_DSCR_ADDR_CH2` reader - "]
pub struct DMA_INLINK_DSCR_ADDR_CH2_R(crate::FieldReader<u32, u32>);
impl DMA_INLINK_DSCR_ADDR_CH2_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_INLINK_DSCR_ADDR_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_INLINK_DSCR_ADDR_CH2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dma_in_state_ch2(&self) -> DMA_IN_STATE_CH2_R {
        DMA_IN_STATE_CH2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dma_in_dscr_state_ch2(&self) -> DMA_IN_DSCR_STATE_CH2_R {
        DMA_IN_DSCR_STATE_CH2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn dma_inlink_dscr_addr_ch2(&self) -> DMA_INLINK_DSCR_ADDR_CH2_R {
        DMA_INLINK_DSCR_ADDR_CH2_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "DMA_IN_STATE_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_state_ch2](index.html) module"]
pub struct DMA_IN_STATE_CH2_SPEC;
impl crate::RegisterSpec for DMA_IN_STATE_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_state_ch2::R](R) reader structure"]
impl crate::Readable for DMA_IN_STATE_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IN_STATE_CH2 to value 0"]
impl crate::Resettable for DMA_IN_STATE_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
