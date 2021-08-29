#[doc = "Register `IDLE_CONF` reader"]
pub struct R(crate::R<IDLE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLE_CONF` writer"]
pub struct W(crate::W<IDLE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLE_CONF_SPEC>;
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
impl From<crate::W<IDLE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_IDLE_NUM` reader - "]
pub struct TX_IDLE_NUM_R(crate::FieldReader<u16, u16>);
impl TX_IDLE_NUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_IDLE_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IDLE_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_IDLE_NUM` writer - "]
pub struct TX_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `RX_IDLE_THRHD` reader - "]
pub struct RX_IDLE_THRHD_R(crate::FieldReader<u16, u16>);
impl RX_IDLE_THRHD_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_IDLE_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_IDLE_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_IDLE_THRHD` writer - "]
pub struct RX_IDLE_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IDLE_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TX_IDLE_NUM_R {
        TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RX_IDLE_THRHD_R {
        RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_idle_num(&mut self) -> TX_IDLE_NUM_W {
        TX_IDLE_NUM_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rx_idle_thrhd(&mut self) -> RX_IDLE_THRHD_W {
        RX_IDLE_THRHD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_IDLE_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_conf](index.html) module"]
pub struct IDLE_CONF_SPEC;
impl crate::RegisterSpec for IDLE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle_conf::R](R) reader structure"]
impl crate::Readable for IDLE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idle_conf::W](W) writer structure"]
impl crate::Writable for IDLE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDLE_CONF to value 0"]
impl crate::Resettable for IDLE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
