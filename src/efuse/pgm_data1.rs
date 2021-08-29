#[doc = "Register `PGM_DATA1` reader"]
pub struct R(crate::R<PGM_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PGM_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PGM_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PGM_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PGM_DATA1` writer"]
pub struct W(crate::W<PGM_DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PGM_DATA1_SPEC>;
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
impl From<crate::W<PGM_DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PGM_DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_GLITCH_DSENSE` reader - "]
pub struct POWER_GLITCH_DSENSE_R(crate::FieldReader<u8, u8>);
impl POWER_GLITCH_DSENSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        POWER_GLITCH_DSENSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWER_GLITCH_DSENSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWER_GLITCH_DSENSE` writer - "]
pub struct POWER_GLITCH_DSENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_GLITCH_DSENSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `POWERGLITCH_EN` reader - "]
pub struct POWERGLITCH_EN_R(crate::FieldReader<bool, bool>);
impl POWERGLITCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        POWERGLITCH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POWERGLITCH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POWERGLITCH_EN` writer - "]
pub struct POWERGLITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERGLITCH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `BTLC_GPIO_ENABLE` reader - "]
pub struct BTLC_GPIO_ENABLE_R(crate::FieldReader<u8, u8>);
impl BTLC_GPIO_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BTLC_GPIO_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTLC_GPIO_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTLC_GPIO_ENABLE` writer - "]
pub struct BTLC_GPIO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTLC_GPIO_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `VDD_SPI_AS_GPIO` reader - "]
pub struct VDD_SPI_AS_GPIO_R(crate::FieldReader<bool, bool>);
impl VDD_SPI_AS_GPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDD_SPI_AS_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_SPI_AS_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_SPI_AS_GPIO` writer - "]
pub struct VDD_SPI_AS_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_SPI_AS_GPIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `USB_EXCHG_PINS` reader - "]
pub struct USB_EXCHG_PINS_R(crate::FieldReader<bool, bool>);
impl USB_EXCHG_PINS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_EXCHG_PINS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_EXCHG_PINS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_EXCHG_PINS` writer - "]
pub struct USB_EXCHG_PINS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_EXCHG_PINS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `USB_DREFL` reader - "]
pub struct USB_DREFL_R(crate::FieldReader<u8, u8>);
impl USB_DREFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USB_DREFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DREFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DREFL` writer - "]
pub struct USB_DREFL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DREFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `USB_DREFH` reader - "]
pub struct USB_DREFH_R(crate::FieldReader<u8, u8>);
impl USB_DREFH_R {
    pub(crate) fn new(bits: u8) -> Self {
        USB_DREFH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DREFH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DREFH` writer - "]
pub struct USB_DREFH_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DREFH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - "]
pub struct DIS_DOWNLOAD_MANUAL_ENCRYPT_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` writer - "]
pub struct DIS_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DOWNLOAD_MANUAL_ENCRYPT_W<'a> {
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
#[doc = "Field `DIS_PAD_JTAG` reader - "]
pub struct DIS_PAD_JTAG_R(crate::FieldReader<bool, bool>);
impl DIS_PAD_JTAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_PAD_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_PAD_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_PAD_JTAG` writer - "]
pub struct DIS_PAD_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_PAD_JTAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SOFT_DIS_JTAG` reader - "]
pub struct SOFT_DIS_JTAG_R(crate::FieldReader<u8, u8>);
impl SOFT_DIS_JTAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOFT_DIS_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_DIS_JTAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_DIS_JTAG` writer - "]
pub struct SOFT_DIS_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_DIS_JTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `JTAG_SEL_ENABLE` reader - "]
pub struct JTAG_SEL_ENABLE_R(crate::FieldReader<bool, bool>);
impl JTAG_SEL_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_SEL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_SEL_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_SEL_ENABLE` writer - "]
pub struct JTAG_SEL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_SEL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `DIS_TWAI` reader - "]
pub struct DIS_TWAI_R(crate::FieldReader<bool, bool>);
impl DIS_TWAI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_TWAI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_TWAI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_TWAI` writer - "]
pub struct DIS_TWAI_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TWAI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RPT4_RESERVED6_ERR` reader - "]
pub struct RPT4_RESERVED6_ERR_R(crate::FieldReader<bool, bool>);
impl RPT4_RESERVED6_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPT4_RESERVED6_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED6_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_FORCE_DOWNLOAD` reader - "]
pub struct DIS_FORCE_DOWNLOAD_R(crate::FieldReader<bool, bool>);
impl DIS_FORCE_DOWNLOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_FORCE_DOWNLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_FORCE_DOWNLOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_FORCE_DOWNLOAD` writer - "]
pub struct DIS_FORCE_DOWNLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_FORCE_DOWNLOAD_W<'a> {
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
#[doc = "Field `DIS_USB_DEVICE` reader - "]
pub struct DIS_USB_DEVICE_R(crate::FieldReader<bool, bool>);
impl DIS_USB_DEVICE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_DEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_DEVICE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_DEVICE` writer - "]
pub struct DIS_USB_DEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_USB_DEVICE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DIS_DOWNLOAD_ICACHE` reader - "]
pub struct DIS_DOWNLOAD_ICACHE_R(crate::FieldReader<bool, bool>);
impl DIS_DOWNLOAD_ICACHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_DOWNLOAD_ICACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_DOWNLOAD_ICACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_DOWNLOAD_ICACHE` writer - "]
pub struct DIS_DOWNLOAD_ICACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DOWNLOAD_ICACHE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DIS_USB_JTAG` reader - "]
pub struct DIS_USB_JTAG_R(crate::FieldReader<bool, bool>);
impl DIS_USB_JTAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_USB_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_USB_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_USB_JTAG` writer - "]
pub struct DIS_USB_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_USB_JTAG_W<'a> {
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
#[doc = "Field `DIS_ICACHE` reader - "]
pub struct DIS_ICACHE_R(crate::FieldReader<bool, bool>);
impl DIS_ICACHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_ICACHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_ICACHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_ICACHE` writer - "]
pub struct DIS_ICACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_ICACHE_W<'a> {
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
#[doc = "Field `DIS_RTC_RAM_BOOT` reader - "]
pub struct DIS_RTC_RAM_BOOT_R(crate::FieldReader<bool, bool>);
impl DIS_RTC_RAM_BOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_RTC_RAM_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_RTC_RAM_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_RTC_RAM_BOOT` writer - "]
pub struct DIS_RTC_RAM_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_RTC_RAM_BOOT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RD_DIS` reader - "]
pub struct RD_DIS_R(crate::FieldReader<u8, u8>);
impl RD_DIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_DIS` writer - "]
pub struct RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn power_glitch_dsense(&self) -> POWER_GLITCH_DSENSE_R {
        POWER_GLITCH_DSENSE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn powerglitch_en(&self) -> POWERGLITCH_EN_R {
        POWERGLITCH_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn btlc_gpio_enable(&self) -> BTLC_GPIO_ENABLE_R {
        BTLC_GPIO_ENABLE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vdd_spi_as_gpio(&self) -> VDD_SPI_AS_GPIO_R {
        VDD_SPI_AS_GPIO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_exchg_pins(&self) -> USB_EXCHG_PINS_R {
        USB_EXCHG_PINS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn usb_drefl(&self) -> USB_DREFL_R {
        USB_DREFL_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn usb_drefh(&self) -> USB_DREFH_R {
        USB_DREFH_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn soft_dis_jtag(&self) -> SOFT_DIS_JTAG_R {
        SOFT_DIS_JTAG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn jtag_sel_enable(&self) -> JTAG_SEL_ENABLE_R {
        JTAG_SEL_ENABLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dis_twai(&self) -> DIS_TWAI_R {
        DIS_TWAI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rpt4_reserved6_err(&self) -> RPT4_RESERVED6_ERR_R {
        RPT4_RESERVED6_ERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dis_force_download(&self) -> DIS_FORCE_DOWNLOAD_R {
        DIS_FORCE_DOWNLOAD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dis_usb_device(&self) -> DIS_USB_DEVICE_R {
        DIS_USB_DEVICE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dis_download_icache(&self) -> DIS_DOWNLOAD_ICACHE_R {
        DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dis_icache(&self) -> DIS_ICACHE_R {
        DIS_ICACHE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dis_rtc_ram_boot(&self) -> DIS_RTC_RAM_BOOT_R {
        DIS_RTC_RAM_BOOT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn power_glitch_dsense(&mut self) -> POWER_GLITCH_DSENSE_W {
        POWER_GLITCH_DSENSE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn powerglitch_en(&mut self) -> POWERGLITCH_EN_W {
        POWERGLITCH_EN_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn btlc_gpio_enable(&mut self) -> BTLC_GPIO_ENABLE_W {
        BTLC_GPIO_ENABLE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn vdd_spi_as_gpio(&mut self) -> VDD_SPI_AS_GPIO_W {
        VDD_SPI_AS_GPIO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_exchg_pins(&mut self) -> USB_EXCHG_PINS_W {
        USB_EXCHG_PINS_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn usb_drefl(&mut self) -> USB_DREFL_W {
        USB_DREFL_W { w: self }
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn usb_drefh(&mut self) -> USB_DREFH_W {
        USB_DREFH_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&mut self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_W {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dis_pad_jtag(&mut self) -> DIS_PAD_JTAG_W {
        DIS_PAD_JTAG_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn soft_dis_jtag(&mut self) -> SOFT_DIS_JTAG_W {
        SOFT_DIS_JTAG_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn jtag_sel_enable(&mut self) -> JTAG_SEL_ENABLE_W {
        JTAG_SEL_ENABLE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dis_twai(&mut self) -> DIS_TWAI_W {
        DIS_TWAI_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dis_force_download(&mut self) -> DIS_FORCE_DOWNLOAD_W {
        DIS_FORCE_DOWNLOAD_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dis_usb_device(&mut self) -> DIS_USB_DEVICE_W {
        DIS_USB_DEVICE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dis_download_icache(&mut self) -> DIS_DOWNLOAD_ICACHE_W {
        DIS_DOWNLOAD_ICACHE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dis_usb_jtag(&mut self) -> DIS_USB_JTAG_W {
        DIS_USB_JTAG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dis_icache(&mut self) -> DIS_ICACHE_W {
        DIS_ICACHE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dis_rtc_ram_boot(&mut self) -> DIS_RTC_RAM_BOOT_W {
        DIS_RTC_RAM_BOOT_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rd_dis(&mut self) -> RD_DIS_W {
        RD_DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE_PGM_DATA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pgm_data1](index.html) module"]
pub struct PGM_DATA1_SPEC;
impl crate::RegisterSpec for PGM_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pgm_data1::R](R) reader structure"]
impl crate::Readable for PGM_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pgm_data1::W](W) writer structure"]
impl crate::Writable for PGM_DATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PGM_DATA1 to value 0"]
impl crate::Resettable for PGM_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
