#[doc = "Register `SWD_CONF` reader"]
pub struct R(crate::R<SWD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWD_CONF` writer"]
pub struct W(crate::W<SWD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWD_CONF_SPEC>;
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
impl From<crate::W<SWD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWD_AUTO_FEED_EN` reader - "]
pub struct SWD_AUTO_FEED_EN_R(crate::FieldReader<bool, bool>);
impl SWD_AUTO_FEED_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_AUTO_FEED_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_AUTO_FEED_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_AUTO_FEED_EN` writer - "]
pub struct SWD_AUTO_FEED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_AUTO_FEED_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `SWD_DISABLE` reader - "]
pub struct SWD_DISABLE_R(crate::FieldReader<bool, bool>);
impl SWD_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_DISABLE` writer - "]
pub struct SWD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SWD_FEED` writer - "]
pub struct SWD_FEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_FEED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `SWD_RST_FLAG_CLR` writer - "]
pub struct SWD_RST_FLAG_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_RST_FLAG_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `SWD_SIGNAL_WIDTH` reader - "]
pub struct SWD_SIGNAL_WIDTH_R(crate::FieldReader<u16, u16>);
impl SWD_SIGNAL_WIDTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        SWD_SIGNAL_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_SIGNAL_WIDTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_SIGNAL_WIDTH` writer - "]
pub struct SWD_SIGNAL_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_SIGNAL_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 18)) | ((value as u32 & 0x03ff) << 18);
        self.w
    }
}
#[doc = "Field `SWD_BYPASS_RST` reader - "]
pub struct SWD_BYPASS_RST_R(crate::FieldReader<bool, bool>);
impl SWD_BYPASS_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_BYPASS_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_BYPASS_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_BYPASS_RST` writer - "]
pub struct SWD_BYPASS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_BYPASS_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SWD_FEED_INT` reader - "]
pub struct SWD_FEED_INT_R(crate::FieldReader<bool, bool>);
impl SWD_FEED_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_FEED_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_FEED_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_RESET_FLAG` reader - "]
pub struct SWD_RESET_FLAG_R(crate::FieldReader<bool, bool>);
impl SWD_RESET_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_RESET_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_RESET_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&self) -> SWD_AUTO_FEED_EN_R {
        SWD_AUTO_FEED_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn swd_disable(&self) -> SWD_DISABLE_R {
        SWD_DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn swd_signal_width(&self) -> SWD_SIGNAL_WIDTH_R {
        SWD_SIGNAL_WIDTH_R::new(((self.bits >> 18) & 0x03ff) as u16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swd_bypass_rst(&self) -> SWD_BYPASS_RST_R {
        SWD_BYPASS_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swd_feed_int(&self) -> SWD_FEED_INT_R {
        SWD_FEED_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swd_reset_flag(&self) -> SWD_RESET_FLAG_R {
        SWD_RESET_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn swd_auto_feed_en(&mut self) -> SWD_AUTO_FEED_EN_W {
        SWD_AUTO_FEED_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn swd_disable(&mut self) -> SWD_DISABLE_W {
        SWD_DISABLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn swd_feed(&mut self) -> SWD_FEED_W {
        SWD_FEED_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn swd_rst_flag_clr(&mut self) -> SWD_RST_FLAG_CLR_W {
        SWD_RST_FLAG_CLR_W { w: self }
    }
    #[doc = "Bits 18:27"]
    #[inline(always)]
    pub fn swd_signal_width(&mut self) -> SWD_SIGNAL_WIDTH_W {
        SWD_SIGNAL_WIDTH_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swd_bypass_rst(&mut self) -> SWD_BYPASS_RST_W {
        SWD_BYPASS_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_SWD_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_conf](index.html) module"]
pub struct SWD_CONF_SPEC;
impl crate::RegisterSpec for SWD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swd_conf::R](R) reader structure"]
impl crate::Readable for SWD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swd_conf::W](W) writer structure"]
impl crate::Writable for SWD_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWD_CONF to value 0"]
impl crate::Resettable for SWD_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
