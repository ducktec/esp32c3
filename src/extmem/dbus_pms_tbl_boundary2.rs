#[doc = "Register `DBUS_PMS_TBL_BOUNDARY2` reader"]
pub struct R(crate::R<DBUS_PMS_TBL_BOUNDARY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS_PMS_TBL_BOUNDARY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS_PMS_TBL_BOUNDARY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS_PMS_TBL_BOUNDARY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBUS_PMS_TBL_BOUNDARY2` writer"]
pub struct W(crate::W<DBUS_PMS_TBL_BOUNDARY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBUS_PMS_TBL_BOUNDARY2_SPEC>;
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
impl From<crate::W<DBUS_PMS_TBL_BOUNDARY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBUS_PMS_TBL_BOUNDARY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBUS_PMS_BOUNDARY2` reader - "]
pub struct DBUS_PMS_BOUNDARY2_R(crate::FieldReader<u16, u16>);
impl DBUS_PMS_BOUNDARY2_R {
    pub(crate) fn new(bits: u16) -> Self {
        DBUS_PMS_BOUNDARY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUS_PMS_BOUNDARY2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUS_PMS_BOUNDARY2` writer - "]
pub struct DBUS_PMS_BOUNDARY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_PMS_BOUNDARY2_W<'a> {
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
    pub fn dbus_pms_boundary2(&self) -> DBUS_PMS_BOUNDARY2_R {
        DBUS_PMS_BOUNDARY2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dbus_pms_boundary2(&mut self) -> DBUS_PMS_BOUNDARY2_W {
        DBUS_PMS_BOUNDARY2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus_pms_tbl_boundary2](index.html) module"]
pub struct DBUS_PMS_TBL_BOUNDARY2_SPEC;
impl crate::RegisterSpec for DBUS_PMS_TBL_BOUNDARY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus_pms_tbl_boundary2::R](R) reader structure"]
impl crate::Readable for DBUS_PMS_TBL_BOUNDARY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbus_pms_tbl_boundary2::W](W) writer structure"]
impl crate::Writable for DBUS_PMS_TBL_BOUNDARY2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBUS_PMS_TBL_BOUNDARY2 to value 0"]
impl crate::Resettable for DBUS_PMS_TBL_BOUNDARY2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}