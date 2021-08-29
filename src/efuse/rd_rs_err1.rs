#[doc = "Register `RD_RS_ERR1` reader"]
pub struct R(crate::R<RD_RS_ERR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_RS_ERR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_RS_ERR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_RS_ERR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_PART2_FAIL` reader - "]
pub struct SYS_PART2_FAIL_R(crate::FieldReader<bool, bool>);
impl SYS_PART2_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_PART2_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_PART2_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_PART2_ERR_NUM` reader - "]
pub struct SYS_PART2_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl SYS_PART2_ERR_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYS_PART2_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_PART2_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY5_FAIL` reader - "]
pub struct KEY5_FAIL_R(crate::FieldReader<bool, bool>);
impl KEY5_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        KEY5_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY5_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY5_ERR_NUM` reader - "]
pub struct KEY5_ERR_NUM_R(crate::FieldReader<u8, u8>);
impl KEY5_ERR_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY5_ERR_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY5_ERR_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sys_part2_fail(&self) -> SYS_PART2_FAIL_R {
        SYS_PART2_FAIL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn sys_part2_err_num(&self) -> SYS_PART2_ERR_NUM_R {
        SYS_PART2_ERR_NUM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn key5_fail(&self) -> KEY5_FAIL_R {
        KEY5_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn key5_err_num(&self) -> KEY5_ERR_NUM_R {
        KEY5_ERR_NUM_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "EFUSE_RD_RS_ERR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_rs_err1](index.html) module"]
pub struct RD_RS_ERR1_SPEC;
impl crate::RegisterSpec for RD_RS_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_rs_err1::R](R) reader structure"]
impl crate::Readable for RD_RS_ERR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_RS_ERR1 to value 0"]
impl crate::Resettable for RD_RS_ERR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
