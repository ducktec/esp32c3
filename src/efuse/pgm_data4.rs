#[doc = "Register `PGM_DATA4` reader"]
pub struct R(crate::R<PGM_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA4` writer"]
pub struct W(crate::W<PGM_DATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA4_SPEC>;
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
impl From<crate::W<PGM_DATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPT4_RESERVED1` reader - "]
pub struct RPT4_RESERVED1_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_VERSION` reader - "]
pub struct SECURE_VERSION_R(crate::FieldReader<u16, u16>);
impl SECURE_VERSION_R {
    pub(crate) fn new(bits: u16) -> Self {
        SECURE_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_VERSION` writer - "]
pub struct SECURE_VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURE_VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | ((value as u32 & 0xffff) << 14);
        self.w
    }
}
#[doc = "Field `FORCE_SEND_RESUME` reader - "]
pub struct FORCE_SEND_RESUME_R(crate::FieldReader<bool, bool>);
impl FORCE_SEND_RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_SEND_RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_SEND_RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_SEND_RESUME` writer - "]
pub struct FORCE_SEND_RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_SEND_RESUME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FLASH_ECC_EN` reader - "]
pub struct FLASH_ECC_EN_R(crate::FieldReader<bool, bool>);
impl FLASH_ECC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_ECC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ECC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ECC_EN` writer - "]
pub struct FLASH_ECC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ECC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FLASH_PAGE_SIZE` reader - "]
pub struct FLASH_PAGE_SIZE_R(crate::FieldReader<u8, u8>);
impl FLASH_PAGE_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_PAGE_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PAGE_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PAGE_SIZE` writer - "]
pub struct FLASH_PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PAGE_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `FLASH_TYPE` reader - "]
pub struct FLASH_TYPE_R(crate::FieldReader<bool, bool>);
impl FLASH_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_TYPE` writer - "]
pub struct FLASH_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_TYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PIN_POWER_SELECTION` reader - "]
pub struct PIN_POWER_SELECTION_R(crate::FieldReader<bool, bool>);
impl PIN_POWER_SELECTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_POWER_SELECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_POWER_SELECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_POWER_SELECTION` writer - "]
pub struct PIN_POWER_SELECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_POWER_SELECTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `UART_PRINT_CONTROL` reader - "]
pub struct UART_PRINT_CONTROL_R(crate::FieldReader<u8, u8>);
impl UART_PRINT_CONTROL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_PRINT_CONTROL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PRINT_CONTROL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PRINT_CONTROL` writer - "]
pub struct UART_PRINT_CONTROL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PRINT_CONTROL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - "]
pub struct ENABLE_SECURITY_DOWNLOAD_R(crate::FieldReader<bool, bool>);
impl ENABLE_SECURITY_DOWNLOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_SECURITY_DOWNLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_SECURITY_DOWNLOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` writer - "]
pub struct ENABLE_SECURITY_DOWNLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SECURITY_DOWNLOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DIS_USB_DOWNLOAD_MODE` reader - "]
pub struct DIS_USB_DOWNLOAD_MODE_R(crate::FieldReader<bool, bool>);
impl DIS_USB_DOWNLOAD_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_DOWNLOAD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_DOWNLOAD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_DOWNLOAD_MODE` writer - "]
pub struct DIS_USB_DOWNLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_USB_DOWNLOAD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FLASH_ECC_MODE` reader - "]
pub struct FLASH_ECC_MODE_R(crate::FieldReader<bool, bool>);
impl FLASH_ECC_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_ECC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ECC_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ECC_MODE` writer - "]
pub struct FLASH_ECC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ECC_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `UART_PRINT_CHANNEL` reader - "]
pub struct UART_PRINT_CHANNEL_R(crate::FieldReader<bool, bool>);
impl UART_PRINT_CHANNEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_PRINT_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PRINT_CHANNEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PRINT_CHANNEL` writer - "]
pub struct UART_PRINT_CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PRINT_CHANNEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DIS_LEGACY_SPI_BOOT` reader - "]
pub struct DIS_LEGACY_SPI_BOOT_R(crate::FieldReader<bool, bool>);
impl DIS_LEGACY_SPI_BOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_LEGACY_SPI_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_LEGACY_SPI_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_LEGACY_SPI_BOOT` writer - "]
pub struct DIS_LEGACY_SPI_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_LEGACY_SPI_BOOT_W<'a> {
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
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - "]
pub struct DIS_DOWNLOAD_MODE_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE` writer - "]
pub struct DIS_DOWNLOAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DOWNLOAD_MODE_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rpt4_reserved1(&self) -> RPT4_RESERVED1_R {
        RPT4_RESERVED1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flash_ecc_en(&self) -> FLASH_ECC_EN_R {
        FLASH_ECC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn flash_page_size(&self) -> FLASH_PAGE_SIZE_R {
        FLASH_PAGE_SIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn flash_type(&self) -> FLASH_TYPE_R {
        FLASH_TYPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin_power_selection(&self) -> PIN_POWER_SELECTION_R {
        PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dis_usb_download_mode(&self) -> DIS_USB_DOWNLOAD_MODE_R {
        DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_ecc_mode(&self) -> FLASH_ECC_MODE_R {
        FLASH_ECC_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_print_channel(&self) -> UART_PRINT_CHANNEL_R {
        UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&self) -> DIS_LEGACY_SPI_BOOT_R {
        DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn secure_version(&mut self) -> SECURE_VERSION_W {
        SECURE_VERSION_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_send_resume(&mut self) -> FORCE_SEND_RESUME_W {
        FORCE_SEND_RESUME_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flash_ecc_en(&mut self) -> FLASH_ECC_EN_W {
        FLASH_ECC_EN_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn flash_page_size(&mut self) -> FLASH_PAGE_SIZE_W {
        FLASH_PAGE_SIZE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn flash_type(&mut self) -> FLASH_TYPE_W {
        FLASH_TYPE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin_power_selection(&mut self) -> PIN_POWER_SELECTION_W {
        PIN_POWER_SELECTION_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uart_print_control(&mut self) -> UART_PRINT_CONTROL_W {
        UART_PRINT_CONTROL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn enable_security_download(&mut self) -> ENABLE_SECURITY_DOWNLOAD_W {
        ENABLE_SECURITY_DOWNLOAD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dis_usb_download_mode(&mut self) -> DIS_USB_DOWNLOAD_MODE_W {
        DIS_USB_DOWNLOAD_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_ecc_mode(&mut self) -> FLASH_ECC_MODE_W {
        FLASH_ECC_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_print_channel(&mut self) -> UART_PRINT_CHANNEL_W {
        UART_PRINT_CHANNEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&mut self) -> DIS_LEGACY_SPI_BOOT_W {
        DIS_LEGACY_SPI_BOOT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dis_download_mode(&mut self) -> DIS_DOWNLOAD_MODE_W {
        DIS_DOWNLOAD_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE_PGM_DATA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data4](index.html) module"]
pub struct PGM_DATA4_SPEC;
impl crate::RegisterSpec for PGM_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data4::R](R) reader structure"]
impl crate::Readable for PGM_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data4::W](W) writer structure"]
impl crate::Writable for PGM_DATA4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_DATA4 to value 0"]
impl crate::Resettable for PGM_DATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
