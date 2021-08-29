#[doc = "Register `RD_STATUS` reader"]
pub struct R(crate::R<RD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_STATUS` writer"]
pub struct W(crate::W<RD_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_STATUS_SPEC>;
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
impl From<crate::W<RD_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WB_MODE` reader - "]
pub struct WB_MODE_R(crate::FieldReader<u8, u8>);
impl WB_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WB_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WB_MODE` writer - "]
pub struct WB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `STATUS` reader - "]
pub struct STATUS_R(crate::FieldReader<u16, u16>);
impl STATUS_R {
    pub(crate) fn new(bits: u16) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS` writer - "]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn wb_mode(&self) -> WB_MODE_R {
        WB_MODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn wb_mode(&mut self) -> WB_MODE_W {
        WB_MODE_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_RD_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_status](index.html) module"]
pub struct RD_STATUS_SPEC;
impl crate::RegisterSpec for RD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_status::R](R) reader structure"]
impl crate::Readable for RD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_status::W](W) writer structure"]
impl crate::Writable for RD_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_STATUS to value 0"]
impl crate::Resettable for RD_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
