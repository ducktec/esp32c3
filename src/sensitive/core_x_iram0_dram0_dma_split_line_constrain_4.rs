#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4` reader"]
pub struct R(crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4` writer"]
pub struct W(crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>;
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
impl From<crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR` reader - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR` writer - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 14)) | ((value as u32 & 0xff) << 14);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2` reader - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2` writer - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1` reader - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1` writer - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0` reader - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R(crate::FieldReader<u8, u8>);
impl CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0` writer - "]
pub struct CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:21"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_splitaddr(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_2(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_1(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_0(
        &self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:21"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_splitaddr(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_SPLITADDR_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_2(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_2_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_1(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_1_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn core_x_dram0_dma_sram_line_0_category_0(
        &mut self,
    ) -> CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W {
        CORE_X_DRAM0_DMA_SRAM_LINE_0_CATEGORY_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_x_iram0_dram0_dma_split_line_constrain_4](index.html) module"]
pub struct CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC;
impl crate::RegisterSpec for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_x_iram0_dram0_dma_split_line_constrain_4::R](R) reader structure"]
impl crate::Readable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_x_iram0_dram0_dma_split_line_constrain_4::W](W) writer structure"]
impl crate::Writable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4 to value 0"]
impl crate::Resettable for CORE_X_IRAM0_DRAM0_DMA_SPLIT_LINE_CONSTRAIN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
