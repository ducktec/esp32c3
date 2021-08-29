#[doc = "Register `RD_REPEAT_ERR3` reader"]
pub struct R(crate::R<RD_REPEAT_ERR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED1_ERR` reader - "]
pub struct RPT4_RESERVED1_ERR_R(crate::FieldReader<u8, u8>);
impl RPT4_RESERVED1_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT4_RESERVED1_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED1_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_VERSION_ERR` reader - "]
pub struct SECURE_VERSION_ERR_R(crate::FieldReader<u16, u16>);
impl SECURE_VERSION_ERR_R {
    pub(crate) fn new(bits: u16) -> Self {
        SECURE_VERSION_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_VERSION_ERR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - "]
pub struct FORCE_SEND_RESUME_ERR_R(crate::FieldReader<bool, bool>);
impl FORCE_SEND_RESUME_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_SEND_RESUME_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_SEND_RESUME_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `FLASH_TYPE_ERR` reader - "]
pub struct FLASH_TYPE_ERR_R(crate::FieldReader<bool, bool>);
impl FLASH_TYPE_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_TYPE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_TYPE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_POWER_SELECTION_ERR` reader - "]
pub struct PIN_POWER_SELECTION_ERR_R(crate::FieldReader<bool, bool>);
impl PIN_POWER_SELECTION_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_POWER_SELECTION_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_POWER_SELECTION_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - "]
pub struct UART_PRINT_CONTROL_ERR_R(crate::FieldReader<u8, u8>);
impl UART_PRINT_CONTROL_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_PRINT_CONTROL_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PRINT_CONTROL_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - "]
pub struct ENABLE_SECURITY_DOWNLOAD_ERR_R(crate::FieldReader<bool, bool>);
impl ENABLE_SECURITY_DOWNLOAD_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_SECURITY_DOWNLOAD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_SECURITY_DOWNLOAD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_DOWNLOAD_MODE_ERR` reader - "]
pub struct DIS_USB_DOWNLOAD_MODE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_USB_DOWNLOAD_MODE_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_DOWNLOAD_MODE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_DOWNLOAD_MODE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ECC_MODE_ERR` reader - "]
pub struct FLASH_ECC_MODE_ERR_R(crate::FieldReader<bool, bool>);
impl FLASH_ECC_MODE_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_ECC_MODE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ECC_MODE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_PRINT_CHANNEL_ERR` reader - "]
pub struct UART_PRINT_CHANNEL_ERR_R(crate::FieldReader<bool, bool>);
impl UART_PRINT_CHANNEL_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_PRINT_CHANNEL_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PRINT_CHANNEL_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_LEGACY_SPI_BOOT_ERR` reader - "]
pub struct DIS_LEGACY_SPI_BOOT_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_LEGACY_SPI_BOOT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_LEGACY_SPI_BOOT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_LEGACY_SPI_BOOT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - "]
pub struct DIS_DOWNLOAD_MODE_ERR_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_MODE_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_MODE_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_MODE_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rpt4_reserved1_err(&self) -> RPT4_RESERVED1_ERR_R {
        RPT4_RESERVED1_ERR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
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
    pub fn flash_type_err(&self) -> FLASH_TYPE_ERR_R {
        FLASH_TYPE_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pin_power_selection_err(&self) -> PIN_POWER_SELECTION_ERR_R {
        PIN_POWER_SELECTION_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dis_usb_download_mode_err(&self) -> DIS_USB_DOWNLOAD_MODE_ERR_R {
        DIS_USB_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_ecc_mode_err(&self) -> FLASH_ECC_MODE_ERR_R {
        FLASH_ECC_MODE_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_print_channel_err(&self) -> UART_PRINT_CHANNEL_ERR_R {
        UART_PRINT_CHANNEL_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dis_legacy_spi_boot_err(&self) -> DIS_LEGACY_SPI_BOOT_ERR_R {
        DIS_LEGACY_SPI_BOOT_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "EFUSE_RD_REPEAT_ERR3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err3](index.html) module"]
pub struct RD_REPEAT_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err3::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR3 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
