#[doc = "Register `PERI_BACKUP_INT_ENA` reader"]
pub struct R(crate::R<PERI_BACKUP_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_BACKUP_INT_ENA` writer"]
pub struct W(crate::W<PERI_BACKUP_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_BACKUP_INT_ENA_SPEC>;
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
impl From<crate::W<PERI_BACKUP_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_BACKUP_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_ERR_INT_ENA` reader - "]
pub struct PERI_BACKUP_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PERI_BACKUP_ERR_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_ERR_INT_ENA` writer - "]
pub struct PERI_BACKUP_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `PERI_BACKUP_DONE_INT_ENA` reader - "]
pub struct PERI_BACKUP_DONE_INT_ENA_R(crate::FieldReader<bool, bool>);
impl PERI_BACKUP_DONE_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_DONE_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_DONE_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_DONE_INT_ENA` writer - "]
pub struct PERI_BACKUP_DONE_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_DONE_INT_ENA_W<'a> {
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
    pub fn peri_backup_err_int_ena(&self) -> PERI_BACKUP_ERR_INT_ENA_R {
        PERI_BACKUP_ERR_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn peri_backup_done_int_ena(&self) -> PERI_BACKUP_DONE_INT_ENA_R {
        PERI_BACKUP_DONE_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn peri_backup_err_int_ena(&mut self) -> PERI_BACKUP_ERR_INT_ENA_W {
        PERI_BACKUP_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn peri_backup_done_int_ena(&mut self) -> PERI_BACKUP_DONE_INT_ENA_W {
        PERI_BACKUP_DONE_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_ena](index.html) module"]
pub struct PERI_BACKUP_INT_ENA_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_int_ena::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_backup_int_ena::W](W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_ENA to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
