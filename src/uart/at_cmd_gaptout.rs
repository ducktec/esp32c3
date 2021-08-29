#[doc = "Register `AT_CMD_GAPTOUT` reader"]
pub struct R(crate::R<AT_CMD_GAPTOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AT_CMD_GAPTOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AT_CMD_GAPTOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AT_CMD_GAPTOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AT_CMD_GAPTOUT` writer"]
pub struct W(crate::W<AT_CMD_GAPTOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AT_CMD_GAPTOUT_SPEC>;
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
impl From<crate::W<AT_CMD_GAPTOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AT_CMD_GAPTOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_GAP_TOUT` reader - "]
pub struct RX_GAP_TOUT_R(crate::FieldReader<u16, u16>);
impl RX_GAP_TOUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_GAP_TOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_GAP_TOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_GAP_TOUT` writer - "]
pub struct RX_GAP_TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_GAP_TOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_gap_tout(&self) -> RX_GAP_TOUT_R {
        RX_GAP_TOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_gap_tout(&mut self) -> RX_GAP_TOUT_W {
        RX_GAP_TOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_AT_CMD_GAPTOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [at_cmd_gaptout](index.html) module"]
pub struct AT_CMD_GAPTOUT_SPEC;
impl crate::RegisterSpec for AT_CMD_GAPTOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [at_cmd_gaptout::R](R) reader structure"]
impl crate::Readable for AT_CMD_GAPTOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [at_cmd_gaptout::W](W) writer structure"]
impl crate::Writable for AT_CMD_GAPTOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AT_CMD_GAPTOUT to value 0"]
impl crate::Resettable for AT_CMD_GAPTOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
