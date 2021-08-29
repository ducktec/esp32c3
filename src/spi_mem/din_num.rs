#[doc = "Register `DIN_NUM` reader"]
pub struct R(crate::R<DIN_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIN_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIN_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIN_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIN_NUM` writer"]
pub struct W(crate::W<DIN_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIN_NUM_SPEC>;
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
impl From<crate::W<DIN_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIN_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN3_NUM` reader - "]
pub struct DIN3_NUM_R(crate::FieldReader<u8, u8>);
impl DIN3_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIN3_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN3_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN3_NUM` writer - "]
pub struct DIN3_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN3_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DIN2_NUM` reader - "]
pub struct DIN2_NUM_R(crate::FieldReader<u8, u8>);
impl DIN2_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIN2_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN2_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN2_NUM` writer - "]
pub struct DIN2_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN2_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DIN1_NUM` reader - "]
pub struct DIN1_NUM_R(crate::FieldReader<u8, u8>);
impl DIN1_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIN1_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN1_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN1_NUM` writer - "]
pub struct DIN1_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN1_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DIN0_NUM` reader - "]
pub struct DIN0_NUM_R(crate::FieldReader<u8, u8>);
impl DIN0_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIN0_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN0_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIN0_NUM` writer - "]
pub struct DIN0_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIN0_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn din3_num(&self) -> DIN3_NUM_R {
        DIN3_NUM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn din2_num(&self) -> DIN2_NUM_R {
        DIN2_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn din1_num(&self) -> DIN1_NUM_R {
        DIN1_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn din0_num(&self) -> DIN0_NUM_R {
        DIN0_NUM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn din3_num(&mut self) -> DIN3_NUM_W {
        DIN3_NUM_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn din2_num(&mut self) -> DIN2_NUM_W {
        DIN2_NUM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn din1_num(&mut self) -> DIN1_NUM_W {
        DIN1_NUM_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn din0_num(&mut self) -> DIN0_NUM_W {
        DIN0_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_DIN_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din_num](index.html) module"]
pub struct DIN_NUM_SPEC;
impl crate::RegisterSpec for DIN_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [din_num::R](R) reader structure"]
impl crate::Readable for DIN_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [din_num::W](W) writer structure"]
impl crate::Writable for DIN_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIN_NUM to value 0"]
impl crate::Resettable for DIN_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
