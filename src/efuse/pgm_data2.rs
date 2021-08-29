#[doc = "Register `PGM_DATA2` reader"]
pub struct R(crate::R<PGM_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA2` writer"]
pub struct W(crate::W<PGM_DATA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA2_SPEC>;
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
impl From<crate::W<PGM_DATA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_PURPOSE_1` reader - "]
pub struct KEY_PURPOSE_1_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_1` writer - "]
pub struct KEY_PURPOSE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `KEY_PURPOSE_0` reader - "]
pub struct KEY_PURPOSE_0_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_0` writer - "]
pub struct KEY_PURPOSE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_PURPOSE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - "]
pub struct SECURE_BOOT_KEY_REVOKE2_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_KEY_REVOKE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_KEY_REVOKE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_KEY_REVOKE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` writer - "]
pub struct SECURE_BOOT_KEY_REVOKE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_KEY_REVOKE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - "]
pub struct SECURE_BOOT_KEY_REVOKE1_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_KEY_REVOKE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_KEY_REVOKE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_KEY_REVOKE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` writer - "]
pub struct SECURE_BOOT_KEY_REVOKE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_KEY_REVOKE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - "]
pub struct SECURE_BOOT_KEY_REVOKE0_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_KEY_REVOKE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_KEY_REVOKE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_KEY_REVOKE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` writer - "]
pub struct SECURE_BOOT_KEY_REVOKE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_BOOT_KEY_REVOKE0_W<'a> {
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
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - "]
pub struct SPI_BOOT_CRYPT_CNT_R(crate::FieldReader<u8, u8>);
impl SPI_BOOT_CRYPT_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI_BOOT_CRYPT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_BOOT_CRYPT_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_BOOT_CRYPT_CNT` writer - "]
pub struct SPI_BOOT_CRYPT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BOOT_CRYPT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `WAT_DELAY_SEL` reader - "]
pub struct WAT_DELAY_SEL_R(crate::FieldReader<u8, u8>);
impl WAT_DELAY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAT_DELAY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAT_DELAY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAT_DELAY_SEL` writer - "]
pub struct WAT_DELAY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAT_DELAY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RPT4_RESERVED2` reader - "]
pub struct RPT4_RESERVED2_R(crate::FieldReader<u16, u16>);
impl RPT4_RESERVED2_R {
    pub(crate) fn new(bits: u16) -> Self {
        RPT4_RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wat_delay_sel(&self) -> WAT_DELAY_SEL_R {
        WAT_DELAY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn key_purpose_1(&mut self) -> KEY_PURPOSE_1_W {
        KEY_PURPOSE_1_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn key_purpose_0(&mut self) -> KEY_PURPOSE_0_W {
        KEY_PURPOSE_0_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&mut self) -> SECURE_BOOT_KEY_REVOKE2_W {
        SECURE_BOOT_KEY_REVOKE2_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&mut self) -> SECURE_BOOT_KEY_REVOKE1_W {
        SECURE_BOOT_KEY_REVOKE1_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&mut self) -> SECURE_BOOT_KEY_REVOKE0_W {
        SECURE_BOOT_KEY_REVOKE0_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&mut self) -> SPI_BOOT_CRYPT_CNT_W {
        SPI_BOOT_CRYPT_CNT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wat_delay_sel(&mut self) -> WAT_DELAY_SEL_W {
        WAT_DELAY_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE_PGM_DATA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data2](index.html) module"]
pub struct PGM_DATA2_SPEC;
impl crate::RegisterSpec for PGM_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data2::R](R) reader structure"]
impl crate::Readable for PGM_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data2::W](W) writer structure"]
impl crate::Writable for PGM_DATA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_DATA2 to value 0"]
impl crate::Resettable for PGM_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
