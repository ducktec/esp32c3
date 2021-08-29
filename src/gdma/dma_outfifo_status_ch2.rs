#[doc = "Register `DMA_OUTFIFO_STATUS_CH2` reader"]
pub struct R(crate::R<DMA_OUTFIFO_STATUS_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUTFIFO_STATUS_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUTFIFO_STATUS_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUTFIFO_STATUS_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUT_REMAIN_UNDER_4B_CH2` reader - "]
pub struct DMA_OUT_REMAIN_UNDER_4B_CH2_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_REMAIN_UNDER_4B_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_REMAIN_UNDER_4B_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_REMAIN_UNDER_4B_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_REMAIN_UNDER_3B_CH2` reader - "]
pub struct DMA_OUT_REMAIN_UNDER_3B_CH2_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_REMAIN_UNDER_3B_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_REMAIN_UNDER_3B_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_REMAIN_UNDER_3B_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_REMAIN_UNDER_2B_CH2` reader - "]
pub struct DMA_OUT_REMAIN_UNDER_2B_CH2_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_REMAIN_UNDER_2B_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_REMAIN_UNDER_2B_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_REMAIN_UNDER_2B_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUT_REMAIN_UNDER_1B_CH2` reader - "]
pub struct DMA_OUT_REMAIN_UNDER_1B_CH2_R(crate::FieldReader<bool, bool>);
impl DMA_OUT_REMAIN_UNDER_1B_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUT_REMAIN_UNDER_1B_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUT_REMAIN_UNDER_1B_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_CNT_CH2` reader - "]
pub struct DMA_OUTFIFO_CNT_CH2_R(crate::FieldReader<u8, u8>);
impl DMA_OUTFIFO_CNT_CH2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_OUTFIFO_CNT_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_CNT_CH2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_EMPTY_CH2` reader - "]
pub struct DMA_OUTFIFO_EMPTY_CH2_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_EMPTY_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_EMPTY_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_EMPTY_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTFIFO_FULL_CH2` reader - "]
pub struct DMA_OUTFIFO_FULL_CH2_R(crate::FieldReader<bool, bool>);
impl DMA_OUTFIFO_FULL_CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTFIFO_FULL_CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTFIFO_FULL_CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dma_out_remain_under_4b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_4B_CH2_R {
        DMA_OUT_REMAIN_UNDER_4B_CH2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dma_out_remain_under_3b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_3B_CH2_R {
        DMA_OUT_REMAIN_UNDER_3B_CH2_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dma_out_remain_under_2b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_2B_CH2_R {
        DMA_OUT_REMAIN_UNDER_2B_CH2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_out_remain_under_1b_ch2(&self) -> DMA_OUT_REMAIN_UNDER_1B_CH2_R {
        DMA_OUT_REMAIN_UNDER_1B_CH2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn dma_outfifo_cnt_ch2(&self) -> DMA_OUTFIFO_CNT_CH2_R {
        DMA_OUTFIFO_CNT_CH2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_outfifo_empty_ch2(&self) -> DMA_OUTFIFO_EMPTY_CH2_R {
        DMA_OUTFIFO_EMPTY_CH2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_outfifo_full_ch2(&self) -> DMA_OUTFIFO_FULL_CH2_R {
        DMA_OUTFIFO_FULL_CH2_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DMA_OUTFIFO_STATUS_CH2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_outfifo_status_ch2](index.html) module"]
pub struct DMA_OUTFIFO_STATUS_CH2_SPEC;
impl crate::RegisterSpec for DMA_OUTFIFO_STATUS_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_outfifo_status_ch2::R](R) reader structure"]
impl crate::Readable for DMA_OUTFIFO_STATUS_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUTFIFO_STATUS_CH2 to value 0"]
impl crate::Resettable for DMA_OUTFIFO_STATUS_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
