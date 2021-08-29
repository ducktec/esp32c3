#[doc = "Register `BACKUP_BUS_PMS_MONITOR_1` reader"]
pub struct R(crate::R<BACKUP_BUS_PMS_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKUP_BUS_PMS_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKUP_BUS_PMS_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKUP_BUS_PMS_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACKUP_BUS_PMS_MONITOR_1` writer"]
pub struct W(crate::W<BACKUP_BUS_PMS_MONITOR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACKUP_BUS_PMS_MONITOR_1_SPEC>;
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
impl From<crate::W<BACKUP_BUS_PMS_MONITOR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACKUP_BUS_PMS_MONITOR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_EN` reader - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R(crate::FieldReader<bool, bool>);
impl BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_EN` writer - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W<'a> {
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
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR` reader - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R(crate::FieldReader<bool, bool>);
impl BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR` writer - "]
pub struct BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_en(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_clr(&self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_en(&mut self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn backup_bus_pms_monitor_violate_clr(&mut self) -> BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W {
        BACKUP_BUS_PMS_MONITOR_VIOLATE_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_BACKUP_BUS_PMS_MONITOR_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backup_bus_pms_monitor_1](index.html) module"]
pub struct BACKUP_BUS_PMS_MONITOR_1_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [backup_bus_pms_monitor_1::R](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [backup_bus_pms_monitor_1::W](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_MONITOR_1 to value 0"]
impl crate::Resettable for BACKUP_BUS_PMS_MONITOR_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
