#[doc = "Register `IBUS_PMS_TBL_BOUNDARY1` reader"]
pub struct R(crate::R<IBUS_PMS_TBL_BOUNDARY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS_PMS_TBL_BOUNDARY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS_PMS_TBL_BOUNDARY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS_PMS_TBL_BOUNDARY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBUS_PMS_TBL_BOUNDARY1` writer"]
pub struct W(crate::W<IBUS_PMS_TBL_BOUNDARY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBUS_PMS_TBL_BOUNDARY1_SPEC>;
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
impl From<crate::W<IBUS_PMS_TBL_BOUNDARY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBUS_PMS_TBL_BOUNDARY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_PMS_BOUNDARY1` reader - "]
pub struct IBUS_PMS_BOUNDARY1_R(crate::FieldReader<u16, u16>);
impl IBUS_PMS_BOUNDARY1_R {
    pub(crate) fn new(bits: u16) -> Self {
        IBUS_PMS_BOUNDARY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUS_PMS_BOUNDARY1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUS_PMS_BOUNDARY1` writer - "]
pub struct IBUS_PMS_BOUNDARY1_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUS_PMS_BOUNDARY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ibus_pms_boundary1(&self) -> IBUS_PMS_BOUNDARY1_R {
        IBUS_PMS_BOUNDARY1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn ibus_pms_boundary1(&mut self) -> IBUS_PMS_BOUNDARY1_W {
        IBUS_PMS_BOUNDARY1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus_pms_tbl_boundary1](index.html) module"]
pub struct IBUS_PMS_TBL_BOUNDARY1_SPEC;
impl crate::RegisterSpec for IBUS_PMS_TBL_BOUNDARY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus_pms_tbl_boundary1::R](R) reader structure"]
impl crate::Readable for IBUS_PMS_TBL_BOUNDARY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibus_pms_tbl_boundary1::W](W) writer structure"]
impl crate::Writable for IBUS_PMS_TBL_BOUNDARY1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBUS_PMS_TBL_BOUNDARY1 to value 0"]
impl crate::Resettable for IBUS_PMS_TBL_BOUNDARY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
