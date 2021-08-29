#[doc = "Register `BROWN_OUT` reader"]
pub struct R(crate::R<BROWN_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROWN_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROWN_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROWN_OUT` writer"]
pub struct W(crate::W<BROWN_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROWN_OUT_SPEC>;
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
impl From<crate::W<BROWN_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROWN_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BROWN_OUT_DET` reader - "]
pub struct BROWN_OUT_DET_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_ENA` reader - "]
pub struct BROWN_OUT_ENA_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_ENA` writer - "]
pub struct BROWN_OUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_ENA_W<'a> {
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
#[doc = "Field `BROWN_OUT_CNT_CLR` writer - "]
pub struct BROWN_OUT_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_CNT_CLR_W<'a> {
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
#[doc = "Field `BROWN_OUT_ANA_RST_EN` reader - "]
pub struct BROWN_OUT_ANA_RST_EN_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_ANA_RST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_ANA_RST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_ANA_RST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_ANA_RST_EN` writer - "]
pub struct BROWN_OUT_ANA_RST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_ANA_RST_EN_W<'a> {
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
#[doc = "Field `BROWN_OUT_RST_SEL` reader - "]
pub struct BROWN_OUT_RST_SEL_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_RST_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_RST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_RST_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_RST_SEL` writer - "]
pub struct BROWN_OUT_RST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_RST_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `BROWN_OUT_RST_ENA` reader - "]
pub struct BROWN_OUT_RST_ENA_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_RST_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_RST_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_RST_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_RST_ENA` writer - "]
pub struct BROWN_OUT_RST_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_RST_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BROWN_OUT_RST_WAIT` reader - "]
pub struct BROWN_OUT_RST_WAIT_R(crate::FieldReader<u16, u16>);
impl BROWN_OUT_RST_WAIT_R {
    pub(crate) fn new(bits: u16) -> Self {
        BROWN_OUT_RST_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_RST_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_RST_WAIT` writer - "]
pub struct BROWN_OUT_RST_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_RST_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `BROWN_OUT_PD_RF_ENA` reader - "]
pub struct BROWN_OUT_PD_RF_ENA_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_PD_RF_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_PD_RF_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_PD_RF_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_PD_RF_ENA` writer - "]
pub struct BROWN_OUT_PD_RF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_PD_RF_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `BROWN_OUT_CLOSE_FLASH_ENA` reader - "]
pub struct BROWN_OUT_CLOSE_FLASH_ENA_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_CLOSE_FLASH_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_CLOSE_FLASH_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_CLOSE_FLASH_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_CLOSE_FLASH_ENA` writer - "]
pub struct BROWN_OUT_CLOSE_FLASH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_CLOSE_FLASH_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `BROWN_OUT_INT_WAIT` reader - "]
pub struct BROWN_OUT_INT_WAIT_R(crate::FieldReader<u16, u16>);
impl BROWN_OUT_INT_WAIT_R {
    pub(crate) fn new(bits: u16) -> Self {
        BROWN_OUT_INT_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_INT_WAIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_INT_WAIT` writer - "]
pub struct BROWN_OUT_INT_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 4)) | ((value as u32 & 0x03ff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn brown_out_det(&self) -> BROWN_OUT_DET_R {
        BROWN_OUT_DET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn brown_out_ena(&self) -> BROWN_OUT_ENA_R {
        BROWN_OUT_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn brown_out_ana_rst_en(&self) -> BROWN_OUT_ANA_RST_EN_R {
        BROWN_OUT_ANA_RST_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn brown_out_rst_sel(&self) -> BROWN_OUT_RST_SEL_R {
        BROWN_OUT_RST_SEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn brown_out_rst_ena(&self) -> BROWN_OUT_RST_ENA_R {
        BROWN_OUT_RST_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn brown_out_rst_wait(&self) -> BROWN_OUT_RST_WAIT_R {
        BROWN_OUT_RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn brown_out_pd_rf_ena(&self) -> BROWN_OUT_PD_RF_ENA_R {
        BROWN_OUT_PD_RF_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn brown_out_close_flash_ena(&self) -> BROWN_OUT_CLOSE_FLASH_ENA_R {
        BROWN_OUT_CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn brown_out_int_wait(&self) -> BROWN_OUT_INT_WAIT_R {
        BROWN_OUT_INT_WAIT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn brown_out_ena(&mut self) -> BROWN_OUT_ENA_W {
        BROWN_OUT_ENA_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn brown_out_cnt_clr(&mut self) -> BROWN_OUT_CNT_CLR_W {
        BROWN_OUT_CNT_CLR_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn brown_out_ana_rst_en(&mut self) -> BROWN_OUT_ANA_RST_EN_W {
        BROWN_OUT_ANA_RST_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn brown_out_rst_sel(&mut self) -> BROWN_OUT_RST_SEL_W {
        BROWN_OUT_RST_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn brown_out_rst_ena(&mut self) -> BROWN_OUT_RST_ENA_W {
        BROWN_OUT_RST_ENA_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn brown_out_rst_wait(&mut self) -> BROWN_OUT_RST_WAIT_W {
        BROWN_OUT_RST_WAIT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn brown_out_pd_rf_ena(&mut self) -> BROWN_OUT_PD_RF_ENA_W {
        BROWN_OUT_PD_RF_ENA_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn brown_out_close_flash_ena(&mut self) -> BROWN_OUT_CLOSE_FLASH_ENA_W {
        BROWN_OUT_CLOSE_FLASH_ENA_W { w: self }
    }
    #[doc = "Bits 4:13"]
    #[inline(always)]
    pub fn brown_out_int_wait(&mut self) -> BROWN_OUT_INT_WAIT_W {
        BROWN_OUT_INT_WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_BROWN_OUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out](index.html) module"]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brown_out::R](R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brown_out::W](W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0"]
impl crate::Resettable for BROWN_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
