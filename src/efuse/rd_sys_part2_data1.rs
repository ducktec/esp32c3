#[doc = "Register `RD_SYS_PART2_DATA1` reader"]
pub struct R(crate::R<RD_SYS_PART2_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_SYS_PART2_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_SYS_PART2_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_SYS_PART2_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DATA_PART2_1` reader - "]
pub struct SYS_DATA_PART2_1_R(crate::FieldReader<u32, u32>);
impl SYS_DATA_PART2_1_R {
    pub(crate) fn new(bits: u32) -> Self {
        SYS_DATA_PART2_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_DATA_PART2_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_data_part2_1(&self) -> SYS_DATA_PART2_1_R {
        SYS_DATA_PART2_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "EFUSE_RD_SYS_PART2_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part2_data1](index.html) module"]
pub struct RD_SYS_PART2_DATA1_SPEC;
impl crate::RegisterSpec for RD_SYS_PART2_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_sys_part2_data1::R](R) reader structure"]
impl crate::Readable for RD_SYS_PART2_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_SYS_PART2_DATA1 to value 0"]
impl crate::Resettable for RD_SYS_PART2_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
