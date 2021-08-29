#[doc = "Register `DMA_OUT_LINK_CH0` reader"]
pub struct R(crate::R<DMA_OUT_LINK_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUT_LINK_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUT_LINK_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUT_LINK_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_OUT_LINK_CH0` writer"]
pub struct W(crate::W<DMA_OUT_LINK_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_OUT_LINK_CH0_SPEC>;
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
impl From<crate::W<DMA_OUT_LINK_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_OUT_LINK_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_OUTLINK_PARK_CH0` reader - "]
pub struct DMA_OUTLINK_PARK_CH0_R(crate::FieldReader<bool, bool>);
impl DMA_OUTLINK_PARK_CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTLINK_PARK_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_PARK_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTLINK_RESTART_CH0` reader - "]
pub struct DMA_OUTLINK_RESTART_CH0_R(crate::FieldReader<bool, bool>);
impl DMA_OUTLINK_RESTART_CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTLINK_RESTART_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_RESTART_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTLINK_RESTART_CH0` writer - "]
pub struct DMA_OUTLINK_RESTART_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_RESTART_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `DMA_OUTLINK_START_CH0` reader - "]
pub struct DMA_OUTLINK_START_CH0_R(crate::FieldReader<bool, bool>);
impl DMA_OUTLINK_START_CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTLINK_START_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_START_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTLINK_START_CH0` writer - "]
pub struct DMA_OUTLINK_START_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_START_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `DMA_OUTLINK_STOP_CH0` reader - "]
pub struct DMA_OUTLINK_STOP_CH0_R(crate::FieldReader<bool, bool>);
impl DMA_OUTLINK_STOP_CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_OUTLINK_STOP_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_STOP_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTLINK_STOP_CH0` writer - "]
pub struct DMA_OUTLINK_STOP_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_STOP_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `DMA_OUTLINK_ADDR_CH0` reader - "]
pub struct DMA_OUTLINK_ADDR_CH0_R(crate::FieldReader<u32, u32>);
impl DMA_OUTLINK_ADDR_CH0_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMA_OUTLINK_ADDR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_OUTLINK_ADDR_CH0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_OUTLINK_ADDR_CH0` writer - "]
pub struct DMA_OUTLINK_ADDR_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_OUTLINK_ADDR_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dma_outlink_park_ch0(&self) -> DMA_OUTLINK_PARK_CH0_R {
        DMA_OUTLINK_PARK_CH0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_outlink_restart_ch0(&self) -> DMA_OUTLINK_RESTART_CH0_R {
        DMA_OUTLINK_RESTART_CH0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_outlink_start_ch0(&self) -> DMA_OUTLINK_START_CH0_R {
        DMA_OUTLINK_START_CH0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_outlink_stop_ch0(&self) -> DMA_OUTLINK_STOP_CH0_R {
        DMA_OUTLINK_STOP_CH0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dma_outlink_addr_ch0(&self) -> DMA_OUTLINK_ADDR_CH0_R {
        DMA_OUTLINK_ADDR_CH0_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dma_outlink_restart_ch0(&mut self) -> DMA_OUTLINK_RESTART_CH0_W {
        DMA_OUTLINK_RESTART_CH0_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dma_outlink_start_ch0(&mut self) -> DMA_OUTLINK_START_CH0_W {
        DMA_OUTLINK_START_CH0_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dma_outlink_stop_ch0(&mut self) -> DMA_OUTLINK_STOP_CH0_W {
        DMA_OUTLINK_STOP_CH0_W { w: self }
    }
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dma_outlink_addr_ch0(&mut self) -> DMA_OUTLINK_ADDR_CH0_W {
        DMA_OUTLINK_ADDR_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_LINK_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_out_link_ch0](index.html) module"]
pub struct DMA_OUT_LINK_CH0_SPEC;
impl crate::RegisterSpec for DMA_OUT_LINK_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_out_link_ch0::R](R) reader structure"]
impl crate::Readable for DMA_OUT_LINK_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_out_link_ch0::W](W) writer structure"]
impl crate::Writable for DMA_OUT_LINK_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_OUT_LINK_CH0 to value 0"]
impl crate::Resettable for DMA_OUT_LINK_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
