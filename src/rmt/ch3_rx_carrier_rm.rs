#[doc = "Register `CH3_RX_CARRIER_RM` reader"]
pub struct R(crate::R<CH3_RX_CARRIER_RM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3_RX_CARRIER_RM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3_RX_CARRIER_RM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3_RX_CARRIER_RM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3_RX_CARRIER_RM` writer"]
pub struct W(crate::W<CH3_RX_CARRIER_RM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3_RX_CARRIER_RM_SPEC>;
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
impl From<crate::W<CH3_RX_CARRIER_RM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3_RX_CARRIER_RM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER_HIGH_THRES_CH3` reader - "]
pub struct CARRIER_HIGH_THRES_CH3_R(crate::FieldReader<u16, u16>);
impl CARRIER_HIGH_THRES_CH3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CARRIER_HIGH_THRES_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_HIGH_THRES_CH3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_HIGH_THRES_CH3` writer - "]
pub struct CARRIER_HIGH_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_HIGH_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CARRIER_LOW_THRES_CH3` reader - "]
pub struct CARRIER_LOW_THRES_CH3_R(crate::FieldReader<u16, u16>);
impl CARRIER_LOW_THRES_CH3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CARRIER_LOW_THRES_CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_LOW_THRES_CH3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_LOW_THRES_CH3` writer - "]
pub struct CARRIER_LOW_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_LOW_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn carrier_high_thres_ch3(&self) -> CARRIER_HIGH_THRES_CH3_R {
        CARRIER_HIGH_THRES_CH3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn carrier_low_thres_ch3(&self) -> CARRIER_LOW_THRES_CH3_R {
        CARRIER_LOW_THRES_CH3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn carrier_high_thres_ch3(&mut self) -> CARRIER_HIGH_THRES_CH3_W {
        CARRIER_HIGH_THRES_CH3_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn carrier_low_thres_ch3(&mut self) -> CARRIER_LOW_THRES_CH3_W {
        CARRIER_LOW_THRES_CH3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH3_RX_CARRIER_RM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_rx_carrier_rm](index.html) module"]
pub struct CH3_RX_CARRIER_RM_SPEC;
impl crate::RegisterSpec for CH3_RX_CARRIER_RM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3_rx_carrier_rm::R](R) reader structure"]
impl crate::Readable for CH3_RX_CARRIER_RM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3_rx_carrier_rm::W](W) writer structure"]
impl crate::Writable for CH3_RX_CARRIER_RM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH3_RX_CARRIER_RM to value 0"]
impl crate::Resettable for CH3_RX_CARRIER_RM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
