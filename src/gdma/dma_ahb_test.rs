#[doc = "Register `DMA_AHB_TEST` reader"]
pub struct R(crate::R<DMA_AHB_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_AHB_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_AHB_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_AHB_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_AHB_TEST` writer"]
pub struct W(crate::W<DMA_AHB_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_AHB_TEST_SPEC>;
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
impl From<crate::W<DMA_AHB_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_AHB_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_AHB_TESTADDR` reader - "]
pub struct DMA_AHB_TESTADDR_R(crate::FieldReader<u8, u8>);
impl DMA_AHB_TESTADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_AHB_TESTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_AHB_TESTADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_AHB_TESTADDR` writer - "]
pub struct DMA_AHB_TESTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AHB_TESTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DMA_AHB_TESTMODE` reader - "]
pub struct DMA_AHB_TESTMODE_R(crate::FieldReader<u8, u8>);
impl DMA_AHB_TESTMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_AHB_TESTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_AHB_TESTMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_AHB_TESTMODE` writer - "]
pub struct DMA_AHB_TESTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_AHB_TESTMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dma_ahb_testaddr(&self) -> DMA_AHB_TESTADDR_R {
        DMA_AHB_TESTADDR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dma_ahb_testmode(&self) -> DMA_AHB_TESTMODE_R {
        DMA_AHB_TESTMODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dma_ahb_testaddr(&mut self) -> DMA_AHB_TESTADDR_W {
        DMA_AHB_TESTADDR_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn dma_ahb_testmode(&mut self) -> DMA_AHB_TESTMODE_W {
        DMA_AHB_TESTMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_AHB_TEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_ahb_test](index.html) module"]
pub struct DMA_AHB_TEST_SPEC;
impl crate::RegisterSpec for DMA_AHB_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_ahb_test::R](R) reader structure"]
impl crate::Readable for DMA_AHB_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_ahb_test::W](W) writer structure"]
impl crate::Writable for DMA_AHB_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_AHB_TEST to value 0"]
impl crate::Resettable for DMA_AHB_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
