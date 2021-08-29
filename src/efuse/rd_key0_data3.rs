#[doc = "Register `RD_KEY0_DATA3` reader"]
pub struct R(crate::R<RD_KEY0_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY0_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY0_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY0_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY0_DATA3` reader - "]
pub struct KEY0_DATA3_R(crate::FieldReader<u32, u32>);
impl KEY0_DATA3_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY0_DATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0_DATA3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn key0_data3(&self) -> KEY0_DATA3_R {
        KEY0_DATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "EFUSE_RD_KEY0_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key0_data3](index.html) module"]
pub struct RD_KEY0_DATA3_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key0_data3::R](R) reader structure"]
impl crate::Readable for RD_KEY0_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY0_DATA3 to value 0"]
impl crate::Resettable for RD_KEY0_DATA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
