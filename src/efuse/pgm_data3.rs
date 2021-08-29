#[doc = "Register `PGM_DATA3` reader"]
pub struct R(crate::R<PGM_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA3` writer"]
pub struct W(crate::W<PGM_DATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA3_SPEC>;
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
impl From<crate::W<PGM_DATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_TPUW` reader - "]
pub struct FLASH_TPUW_R(crate::FieldReader<u8, u8>);
impl FLASH_TPUW_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_TPUW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_TPUW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_TPUW` writer - "]
pub struct FLASH_TPUW_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_TPUW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `RPT4_RESERVED0` reader - "]
pub struct RPT4_RESERVED0_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - "]
pub struct SECURE_BOOT_AGGRESSIVE_REVOKE_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_AGGRESSIVE_REVOKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_AGGRESSIVE_REVOKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` writer - "]
pub struct SECURE_BOOT_AGGRESSIVE_REVOKE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_AGGRESSIVE_REVOKE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SECURE_BOOT_EN` reader - "]
pub struct SECURE_BOOT_EN_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_EN` writer - "]
pub struct SECURE_BOOT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RPT4_RESERVED3` reader - "]
pub struct RPT4_RESERVED3_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_5` reader - "]
pub struct KEY_PURPOSE_5_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_5` writer - "]
pub struct KEY_PURPOSE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `KEY_PURPOSE_4` reader - "]
pub struct KEY_PURPOSE_4_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_4` writer - "]
pub struct KEY_PURPOSE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `KEY_PURPOSE_3` reader - "]
pub struct KEY_PURPOSE_3_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_3` writer - "]
pub struct KEY_PURPOSE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `KEY_PURPOSE_2` reader - "]
pub struct KEY_PURPOSE_2_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_2` writer - "]
pub struct KEY_PURPOSE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27"]
    #[inline(always)]
    pub fn rpt4_reserved0(&self) -> RPT4_RESERVED0_R {
        RPT4_RESERVED0_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rpt4_reserved3(&self) -> RPT4_RESERVED3_R {
        RPT4_RESERVED3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn flash_tpuw(&mut self) -> FLASH_TPUW_W {
        FLASH_TPUW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&mut self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_W {
        SECURE_BOOT_AGGRESSIVE_REVOKE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn secure_boot_en(&mut self) -> SECURE_BOOT_EN_W {
        SECURE_BOOT_EN_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn key_purpose_5(&mut self) -> KEY_PURPOSE_5_W {
        KEY_PURPOSE_5_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn key_purpose_4(&mut self) -> KEY_PURPOSE_4_W {
        KEY_PURPOSE_4_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn key_purpose_3(&mut self) -> KEY_PURPOSE_3_W {
        KEY_PURPOSE_3_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn key_purpose_2(&mut self) -> KEY_PURPOSE_2_W {
        KEY_PURPOSE_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE_PGM_DATA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data3](index.html) module"]
pub struct PGM_DATA3_SPEC;
impl crate::RegisterSpec for PGM_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data3::R](R) reader structure"]
impl crate::Readable for PGM_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data3::W](W) writer structure"]
impl crate::Writable for PGM_DATA3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_DATA3 to value 0"]
impl crate::Resettable for PGM_DATA3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
