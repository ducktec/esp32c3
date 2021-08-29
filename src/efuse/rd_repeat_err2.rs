#[doc = "Register `RD_REPEAT_ERR2` reader"]
pub struct R(crate::R<RD_REPEAT_ERR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASH_TPUW_ERR` reader - "]
pub struct FLASH_TPUW_ERR_R(crate::FieldReader<u8, u8>);
impl FLASH_TPUW_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_TPUW_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_TPUW_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED0_ERR` reader - "]
pub struct RPT4_RESERVED0_ERR_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED0_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED0_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED0_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - "]
pub struct SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - "]
pub struct SECURE_BOOT_EN_ERR_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_EN_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_EN_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_EN_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPT4_RESERVED3_ERR` reader - "]
pub struct RPT4_RESERVED3_ERR_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED3_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED3_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED3_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_5_ERR` reader - "]
pub struct KEY_PURPOSE_5_ERR_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_5_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_5_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_5_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_4_ERR` reader - "]
pub struct KEY_PURPOSE_4_ERR_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_4_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_4_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_4_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_3_ERR` reader - "]
pub struct KEY_PURPOSE_3_ERR_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_3_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_3_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_3_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_2_ERR` reader - "]
pub struct KEY_PURPOSE_2_ERR_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_2_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_2_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_2_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn rpt4_reserved0_err(&self) -> RPT4_RESERVED0_ERR_R {
        RPT4_RESERVED0_ERR_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rpt4_reserved3_err(&self) -> RPT4_RESERVED3_ERR_R {
        RPT4_RESERVED3_ERR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KEY_PURPOSE_5_ERR_R {
        KEY_PURPOSE_5_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KEY_PURPOSE_4_ERR_R {
        KEY_PURPOSE_4_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KEY_PURPOSE_3_ERR_R {
        KEY_PURPOSE_3_ERR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KEY_PURPOSE_2_ERR_R {
        KEY_PURPOSE_2_ERR_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "EFUSE_RD_REPEAT_ERR2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err2](index.html) module"]
pub struct RD_REPEAT_ERR2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err2::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR2 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
