#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_10` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_10` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_1_h(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_1_l(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_0_h(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_0_l(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_1_h(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_H_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_1_l(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_1_L_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_0_h(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_H_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_rtcfast_world_0_l(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W {
        CORE_0_PIF_PMS_CONSTRAIN_RTCFAST_WORLD_0_L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_10](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_10_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_10::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_10::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_10 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
