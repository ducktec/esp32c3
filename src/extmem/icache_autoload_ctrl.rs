#[doc = "Register `ICACHE_AUTOLOAD_CTRL` reader"]
pub struct R(crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_AUTOLOAD_CTRL` writer"]
pub struct W(crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>;
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
impl From<crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_RQST` reader - "]
pub struct ICACHE_AUTOLOAD_RQST_R(crate::FieldReader<u8, u8>);
impl ICACHE_AUTOLOAD_RQST_R {
    pub(crate) fn new(bits: u8) -> Self {
        ICACHE_AUTOLOAD_RQST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_AUTOLOAD_RQST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_RQST` writer - "]
pub struct ICACHE_AUTOLOAD_RQST_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_AUTOLOAD_RQST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_ORDER` reader - "]
pub struct ICACHE_AUTOLOAD_ORDER_R(crate::FieldReader<bool, bool>);
impl ICACHE_AUTOLOAD_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_AUTOLOAD_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_AUTOLOAD_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_ORDER` writer - "]
pub struct ICACHE_AUTOLOAD_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_AUTOLOAD_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_DONE` reader - "]
pub struct ICACHE_AUTOLOAD_DONE_R(crate::FieldReader<bool, bool>);
impl ICACHE_AUTOLOAD_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_AUTOLOAD_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_AUTOLOAD_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_ENA` reader - "]
pub struct ICACHE_AUTOLOAD_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_AUTOLOAD_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_AUTOLOAD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_AUTOLOAD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_ENA` writer - "]
pub struct ICACHE_AUTOLOAD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_AUTOLOAD_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ENA` reader - "]
pub struct ICACHE_AUTOLOAD_SCT1_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_AUTOLOAD_SCT1_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_AUTOLOAD_SCT1_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_AUTOLOAD_SCT1_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ENA` writer - "]
pub struct ICACHE_AUTOLOAD_SCT1_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_AUTOLOAD_SCT1_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_ENA` reader - "]
pub struct ICACHE_AUTOLOAD_SCT0_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_AUTOLOAD_SCT0_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_AUTOLOAD_SCT0_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_AUTOLOAD_SCT0_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT0_ENA` writer - "]
pub struct ICACHE_AUTOLOAD_SCT0_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_AUTOLOAD_SCT0_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn icache_autoload_rqst(&self) -> ICACHE_AUTOLOAD_RQST_R {
        ICACHE_AUTOLOAD_RQST_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn icache_autoload_order(&self) -> ICACHE_AUTOLOAD_ORDER_R {
        ICACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn icache_autoload_done(&self) -> ICACHE_AUTOLOAD_DONE_R {
        ICACHE_AUTOLOAD_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icache_autoload_ena(&self) -> ICACHE_AUTOLOAD_ENA_R {
        ICACHE_AUTOLOAD_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icache_autoload_sct1_ena(&self) -> ICACHE_AUTOLOAD_SCT1_ENA_R {
        ICACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_autoload_sct0_ena(&self) -> ICACHE_AUTOLOAD_SCT0_ENA_R {
        ICACHE_AUTOLOAD_SCT0_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn icache_autoload_rqst(&mut self) -> ICACHE_AUTOLOAD_RQST_W {
        ICACHE_AUTOLOAD_RQST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn icache_autoload_order(&mut self) -> ICACHE_AUTOLOAD_ORDER_W {
        ICACHE_AUTOLOAD_ORDER_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icache_autoload_ena(&mut self) -> ICACHE_AUTOLOAD_ENA_W {
        ICACHE_AUTOLOAD_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icache_autoload_sct1_ena(&mut self) -> ICACHE_AUTOLOAD_SCT1_ENA_W {
        ICACHE_AUTOLOAD_SCT1_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_autoload_sct0_ena(&mut self) -> ICACHE_AUTOLOAD_SCT0_ENA_W {
        ICACHE_AUTOLOAD_SCT0_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_ICACHE_AUTOLOAD_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_autoload_ctrl](index.html) module"]
pub struct ICACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_autoload_ctrl::R](R) reader structure"]
impl crate::Readable for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_autoload_ctrl::W](W) writer structure"]
impl crate::Writable for ICACHE_AUTOLOAD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_AUTOLOAD_CTRL to value 0"]
impl crate::Resettable for ICACHE_AUTOLOAD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
