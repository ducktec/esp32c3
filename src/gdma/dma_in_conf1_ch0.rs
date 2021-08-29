#[doc = "Register `DMA_IN_CONF1_CH0` reader"]
pub struct R(crate::R<DMA_IN_CONF1_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_CONF1_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_CONF1_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_CONF1_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IN_CONF1_CH0` writer"]
pub struct W(crate::W<DMA_IN_CONF1_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_IN_CONF1_CH0_SPEC>;
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
impl From<crate::W<DMA_IN_CONF1_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_IN_CONF1_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_IN_CHECK_OWNER_CH0` reader - "]
pub struct DMA_IN_CHECK_OWNER_CH0_R(crate::FieldReader<bool, bool>);
impl DMA_IN_CHECK_OWNER_CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_CHECK_OWNER_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_CHECK_OWNER_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_CHECK_OWNER_CH0` writer - "]
pub struct DMA_IN_CHECK_OWNER_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_CHECK_OWNER_CH0_W<'a> {
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
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_in_check_owner_ch0(&self) -> DMA_IN_CHECK_OWNER_CH0_R {
        DMA_IN_CHECK_OWNER_CH0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dma_in_check_owner_ch0(&mut self) -> DMA_IN_CHECK_OWNER_CH0_W {
        DMA_IN_CHECK_OWNER_CH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IN_CONF1_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_conf1_ch0](index.html) module"]
pub struct DMA_IN_CONF1_CH0_SPEC;
impl crate::RegisterSpec for DMA_IN_CONF1_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_conf1_ch0::R](R) reader structure"]
impl crate::Readable for DMA_IN_CONF1_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_in_conf1_ch0::W](W) writer structure"]
impl crate::Writable for DMA_IN_CONF1_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_IN_CONF1_CH0 to value 0"]
impl crate::Resettable for DMA_IN_CONF1_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
