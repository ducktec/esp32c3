#[doc = "Register `RX_TIMING` reader"]
pub struct R(crate::R<RX_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_TIMING` writer"]
pub struct W(crate::W<RX_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_TIMING_SPEC>;
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
impl From<crate::W<RX_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_BCK_IN_DM` reader - "]
pub struct RX_BCK_IN_DM_R(crate::FieldReader<u8, u8>);
impl RX_BCK_IN_DM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_BCK_IN_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BCK_IN_DM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BCK_IN_DM` writer - "]
pub struct RX_BCK_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `RX_WS_IN_DM` reader - "]
pub struct RX_WS_IN_DM_R(crate::FieldReader<u8, u8>);
impl RX_WS_IN_DM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_WS_IN_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WS_IN_DM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WS_IN_DM` writer - "]
pub struct RX_WS_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `RX_BCK_OUT_DM` reader - "]
pub struct RX_BCK_OUT_DM_R(crate::FieldReader<u8, u8>);
impl RX_BCK_OUT_DM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_BCK_OUT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BCK_OUT_DM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BCK_OUT_DM` writer - "]
pub struct RX_BCK_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `RX_WS_OUT_DM` reader - "]
pub struct RX_WS_OUT_DM_R(crate::FieldReader<u8, u8>);
impl RX_WS_OUT_DM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_WS_OUT_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WS_OUT_DM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WS_OUT_DM` writer - "]
pub struct RX_WS_OUT_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_OUT_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RX_SD_IN_DM` reader - "]
pub struct RX_SD_IN_DM_R(crate::FieldReader<u8, u8>);
impl RX_SD_IN_DM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_SD_IN_DM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SD_IN_DM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SD_IN_DM` writer - "]
pub struct RX_SD_IN_DM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SD_IN_DM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rx_bck_in_dm(&self) -> RX_BCK_IN_DM_R {
        RX_BCK_IN_DM_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_ws_in_dm(&self) -> RX_WS_IN_DM_R {
        RX_WS_IN_DM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rx_bck_out_dm(&self) -> RX_BCK_OUT_DM_R {
        RX_BCK_OUT_DM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_dm(&self) -> RX_WS_OUT_DM_R {
        RX_WS_OUT_DM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rx_sd_in_dm(&self) -> RX_SD_IN_DM_R {
        RX_SD_IN_DM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rx_bck_in_dm(&mut self) -> RX_BCK_IN_DM_W {
        RX_BCK_IN_DM_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rx_ws_in_dm(&mut self) -> RX_WS_IN_DM_W {
        RX_WS_IN_DM_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rx_bck_out_dm(&mut self) -> RX_BCK_OUT_DM_W {
        RX_BCK_OUT_DM_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rx_ws_out_dm(&mut self) -> RX_WS_OUT_DM_W {
        RX_WS_OUT_DM_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rx_sd_in_dm(&mut self) -> RX_SD_IN_DM_W {
        RX_SD_IN_DM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_RX_TIMING\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_timing](index.html) module"]
pub struct RX_TIMING_SPEC;
impl crate::RegisterSpec for RX_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_timing::R](R) reader structure"]
impl crate::Readable for RX_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_timing::W](W) writer structure"]
impl crate::Writable for RX_TIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_TIMING to value 0"]
impl crate::Resettable for RX_TIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
