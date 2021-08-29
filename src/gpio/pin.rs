#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_ENA` reader - "]
pub struct INT_ENA_R(crate::FieldReader<u8, u8>);
impl INT_ENA_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_ENA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_ENA` writer - "]
pub struct INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | ((value as u32 & 0x1f) << 13);
        self.w
    }
}
#[doc = "Field `CONFIG` reader - "]
pub struct CONFIG_R(crate::FieldReader<u8, u8>);
impl CONFIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONFIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONFIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONFIG` writer - "]
pub struct CONFIG_W<'a> {
    w: &'a mut W,
}
impl<'a> CONFIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `WAKEUP_ENABLE` reader - "]
pub struct WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_ENABLE` writer - "]
pub struct WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `INT_TYPE` reader - "]
pub struct INT_TYPE_R(crate::FieldReader<u8, u8>);
impl INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_TYPE` writer - "]
pub struct INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `SYNC1_BYPASS` reader - "]
pub struct SYNC1_BYPASS_R(crate::FieldReader<u8, u8>);
impl SYNC1_BYPASS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC1_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC1_BYPASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC1_BYPASS` writer - "]
pub struct SYNC1_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `PAD_DRIVER` reader - "]
pub struct PAD_DRIVER_R(crate::FieldReader<bool, bool>);
impl PAD_DRIVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD_DRIVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_DRIVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_DRIVER` writer - "]
pub struct PAD_DRIVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DRIVER_W<'a> {
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
#[doc = "Field `SYNC2_BYPASS` reader - "]
pub struct SYNC2_BYPASS_R(crate::FieldReader<u8, u8>);
impl SYNC2_BYPASS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNC2_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC2_BYPASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC2_BYPASS` writer - "]
pub struct SYNC2_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC2_BYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn int_ena(&self) -> INT_ENA_R {
        INT_ENA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wakeup_enable(&self) -> WAKEUP_ENABLE_R {
        WAKEUP_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn int_type(&self) -> INT_TYPE_R {
        INT_TYPE_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn sync1_bypass(&self) -> SYNC1_BYPASS_R {
        SYNC1_BYPASS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sync2_bypass(&self) -> SYNC2_BYPASS_R {
        SYNC2_BYPASS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 13:17"]
    #[inline(always)]
    pub fn int_ena(&mut self) -> INT_ENA_W {
        INT_ENA_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W {
        CONFIG_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wakeup_enable(&mut self) -> WAKEUP_ENABLE_W {
        WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn int_type(&mut self) -> INT_TYPE_W {
        INT_TYPE_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn sync1_bypass(&mut self) -> SYNC1_BYPASS_W {
        SYNC1_BYPASS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W {
        PAD_DRIVER_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sync2_bypass(&mut self) -> SYNC2_BYPASS_W {
        SYNC2_BYPASS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_PIN%s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
