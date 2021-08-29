#[doc = "Register `FLOW_CONF` reader"]
pub struct R(crate::R<FLOW_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOW_CONF` writer"]
pub struct W(crate::W<FLOW_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_CONF_SPEC>;
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
impl From<crate::W<FLOW_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEND_XOFF` reader - "]
pub struct SEND_XOFF_R(crate::FieldReader<bool, bool>);
impl SEND_XOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_XOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_XOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_XOFF` writer - "]
pub struct SEND_XOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_XOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SEND_XON` reader - "]
pub struct SEND_XON_R(crate::FieldReader<bool, bool>);
impl SEND_XON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_XON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_XON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_XON` writer - "]
pub struct SEND_XON_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_XON_W<'a> {
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
#[doc = "Field `FORCE_XOFF` reader - "]
pub struct FORCE_XOFF_R(crate::FieldReader<bool, bool>);
impl FORCE_XOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_XOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XOFF` writer - "]
pub struct FORCE_XOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FORCE_XON` reader - "]
pub struct FORCE_XON_R(crate::FieldReader<bool, bool>);
impl FORCE_XON_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_XON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_XON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_XON` writer - "]
pub struct FORCE_XON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_XON_W<'a> {
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
#[doc = "Field `XONOFF_DEL` reader - "]
pub struct XONOFF_DEL_R(crate::FieldReader<bool, bool>);
impl XONOFF_DEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        XONOFF_DEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XONOFF_DEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XONOFF_DEL` writer - "]
pub struct XONOFF_DEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XONOFF_DEL_W<'a> {
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
#[doc = "Field `SW_FLOW_CON_EN` reader - "]
pub struct SW_FLOW_CON_EN_R(crate::FieldReader<bool, bool>);
impl SW_FLOW_CON_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_FLOW_CON_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_FLOW_CON_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_FLOW_CON_EN` writer - "]
pub struct SW_FLOW_CON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_FLOW_CON_EN_W<'a> {
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
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_xoff(&self) -> SEND_XOFF_R {
        SEND_XOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_xon(&self) -> SEND_XON_R {
        SEND_XON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn force_xoff(&self) -> FORCE_XOFF_R {
        FORCE_XOFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_xon(&self) -> FORCE_XON_R {
        FORCE_XON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xonoff_del(&self) -> XONOFF_DEL_R {
        XONOFF_DEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_flow_con_en(&self) -> SW_FLOW_CON_EN_R {
        SW_FLOW_CON_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_xoff(&mut self) -> SEND_XOFF_W {
        SEND_XOFF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_xon(&mut self) -> SEND_XON_W {
        SEND_XON_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn force_xoff(&mut self) -> FORCE_XOFF_W {
        FORCE_XOFF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn force_xon(&mut self) -> FORCE_XON_W {
        FORCE_XON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xonoff_del(&mut self) -> XONOFF_DEL_W {
        XONOFF_DEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_flow_con_en(&mut self) -> SW_FLOW_CON_EN_W {
        SW_FLOW_CON_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_FLOW_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_conf](index.html) module"]
pub struct FLOW_CONF_SPEC;
impl crate::RegisterSpec for FLOW_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow_conf::R](R) reader structure"]
impl crate::Readable for FLOW_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow_conf::W](W) writer structure"]
impl crate::Writable for FLOW_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOW_CONF to value 0"]
impl crate::Resettable for FLOW_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
