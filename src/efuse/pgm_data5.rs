#[doc = "Register `PGM_DATA5` reader"]
pub struct R(crate::R<PGM_DATA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED4` reader - "]
pub struct RPT4_RESERVED4_R(crate::FieldReader<u32, u32>);
impl RPT4_RESERVED4_R {
    pub(crate) fn new(bits: u32) -> Self {
        RPT4_RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn rpt4_reserved4(&self) -> RPT4_RESERVED4_R {
        RPT4_RESERVED4_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "EFUSE_PGM_DATA5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data5](index.html) module"]
pub struct PGM_DATA5_SPEC;
impl crate::RegisterSpec for PGM_DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data5::R](R) reader structure"]
impl crate::Readable for PGM_DATA5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PGM_DATA5 to value 0"]
impl crate::Resettable for PGM_DATA5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
