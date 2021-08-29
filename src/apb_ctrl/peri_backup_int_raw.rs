#[doc = "Register `PERI_BACKUP_INT_RAW` reader"]
pub struct R(crate::R<PERI_BACKUP_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERI_BACKUP_ERR_INT_RAW` reader - "]
pub struct PERI_BACKUP_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl PERI_BACKUP_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_DONE_INT_RAW` reader - "]
pub struct PERI_BACKUP_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl PERI_BACKUP_DONE_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn peri_backup_err_int_raw(&self) -> PERI_BACKUP_ERR_INT_RAW_R {
        PERI_BACKUP_ERR_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn peri_backup_done_int_raw(&self) -> PERI_BACKUP_DONE_INT_RAW_R {
        PERI_BACKUP_DONE_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_int_raw](index.html) module"]
pub struct PERI_BACKUP_INT_RAW_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_int_raw::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_RAW to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
