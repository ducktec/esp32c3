#[doc = "Register `HUNG_CONF` reader"]
pub struct R(crate::R<HUNG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUNG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUNG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUNG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUNG_CONF` writer"]
pub struct W(crate::W<HUNG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUNG_CONF_SPEC>;
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
impl From<crate::W<HUNG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUNG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_TIMEOUT_ENA` reader - "]
pub struct RXFIFO_TIMEOUT_ENA_R(crate::FieldReader<bool, bool>);
impl RXFIFO_TIMEOUT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_TIMEOUT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TIMEOUT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TIMEOUT_ENA` writer - "]
pub struct RXFIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TIMEOUT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` reader - "]
pub struct RXFIFO_TIMEOUT_SHIFT_R(crate::FieldReader<u8, u8>);
impl RXFIFO_TIMEOUT_SHIFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_TIMEOUT_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TIMEOUT_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TIMEOUT_SHIFT` writer - "]
pub struct RXFIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `RXFIFO_TIMEOUT` reader - "]
pub struct RXFIFO_TIMEOUT_R(crate::FieldReader<u8, u8>);
impl RXFIFO_TIMEOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TIMEOUT` writer - "]
pub struct RXFIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `TXFIFO_TIMEOUT_ENA` reader - "]
pub struct TXFIFO_TIMEOUT_ENA_R(crate::FieldReader<bool, bool>);
impl TXFIFO_TIMEOUT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_TIMEOUT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_TIMEOUT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_TIMEOUT_ENA` writer - "]
pub struct TXFIFO_TIMEOUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_TIMEOUT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` reader - "]
pub struct TXFIFO_TIMEOUT_SHIFT_R(crate::FieldReader<u8, u8>);
impl TXFIFO_TIMEOUT_SHIFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_TIMEOUT_SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_TIMEOUT_SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_TIMEOUT_SHIFT` writer - "]
pub struct TXFIFO_TIMEOUT_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_TIMEOUT_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `TXFIFO_TIMEOUT` reader - "]
pub struct TXFIFO_TIMEOUT_R(crate::FieldReader<u8, u8>);
impl TXFIFO_TIMEOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_TIMEOUT` writer - "]
pub struct TXFIFO_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&self) -> RXFIFO_TIMEOUT_ENA_R {
        RXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&self) -> RXFIFO_TIMEOUT_SHIFT_R {
        RXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn rxfifo_timeout(&self) -> RXFIFO_TIMEOUT_R {
        RXFIFO_TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&self) -> TXFIFO_TIMEOUT_ENA_R {
        TXFIFO_TIMEOUT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&self) -> TXFIFO_TIMEOUT_SHIFT_R {
        TXFIFO_TIMEOUT_SHIFT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txfifo_timeout(&self) -> TXFIFO_TIMEOUT_R {
        TXFIFO_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rxfifo_timeout_ena(&mut self) -> RXFIFO_TIMEOUT_ENA_W {
        RXFIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rxfifo_timeout_shift(&mut self) -> RXFIFO_TIMEOUT_SHIFT_W {
        RXFIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn rxfifo_timeout(&mut self) -> RXFIFO_TIMEOUT_W {
        RXFIFO_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txfifo_timeout_ena(&mut self) -> TXFIFO_TIMEOUT_ENA_W {
        TXFIFO_TIMEOUT_ENA_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn txfifo_timeout_shift(&mut self) -> TXFIFO_TIMEOUT_SHIFT_W {
        TXFIFO_TIMEOUT_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txfifo_timeout(&mut self) -> TXFIFO_TIMEOUT_W {
        TXFIFO_TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI_HUNG_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hung_conf](index.html) module"]
pub struct HUNG_CONF_SPEC;
impl crate::RegisterSpec for HUNG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hung_conf::R](R) reader structure"]
impl crate::Readable for HUNG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hung_conf::W](W) writer structure"]
impl crate::Writable for HUNG_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HUNG_CONF to value 0"]
impl crate::Resettable for HUNG_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
