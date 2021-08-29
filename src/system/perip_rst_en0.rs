#[doc = "Register `PERIP_RST_EN0` reader"]
pub struct R(crate::R<PERIP_RST_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN0` writer"]
pub struct W(crate::W<PERIP_RST_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN0_SPEC>;
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
impl From<crate::W<PERIP_RST_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI4_RST` reader - "]
pub struct SPI4_RST_R(crate::FieldReader<bool, bool>);
impl SPI4_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI4_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI4_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI4_RST` writer - "]
pub struct SPI4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `ADC2_ARB_RST` reader - "]
pub struct ADC2_ARB_RST_R(crate::FieldReader<bool, bool>);
impl ADC2_ARB_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_ARB_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_ARB_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_ARB_RST` writer - "]
pub struct ADC2_ARB_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_ARB_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SYSTIMER_RST` reader - "]
pub struct SYSTIMER_RST_R(crate::FieldReader<bool, bool>);
impl SYSTIMER_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSTIMER_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTIMER_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTIMER_RST` writer - "]
pub struct SYSTIMER_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTIMER_RST_W<'a> {
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
#[doc = "Field `APB_SARADC_RST` reader - "]
pub struct APB_SARADC_RST_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_RST` writer - "]
pub struct APB_SARADC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `SPI3_DMA_RST` reader - "]
pub struct SPI3_DMA_RST_R(crate::FieldReader<bool, bool>);
impl SPI3_DMA_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3_DMA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3_DMA_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3_DMA_RST` writer - "]
pub struct SPI3_DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_DMA_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PWM3_RST` reader - "]
pub struct PWM3_RST_R(crate::FieldReader<bool, bool>);
impl PWM3_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM3_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM3_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM3_RST` writer - "]
pub struct PWM3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM3_RST_W<'a> {
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
#[doc = "Field `PWM2_RST` reader - "]
pub struct PWM2_RST_R(crate::FieldReader<bool, bool>);
impl PWM2_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM2_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM2_RST` writer - "]
pub struct PWM2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2_RST_W<'a> {
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
#[doc = "Field `UART_MEM_RST` reader - "]
pub struct UART_MEM_RST_R(crate::FieldReader<bool, bool>);
impl UART_MEM_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_MEM_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_MEM_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_MEM_RST` writer - "]
pub struct UART_MEM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `USB_DEVICE_RST` reader - "]
pub struct USB_DEVICE_RST_R(crate::FieldReader<bool, bool>);
impl USB_DEVICE_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_DEVICE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DEVICE_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DEVICE_RST` writer - "]
pub struct USB_DEVICE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DEVICE_RST_W<'a> {
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
#[doc = "Field `SPI2_DMA_RST` reader - "]
pub struct SPI2_DMA_RST_R(crate::FieldReader<bool, bool>);
impl SPI2_DMA_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_DMA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_DMA_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2_DMA_RST` writer - "]
pub struct SPI2_DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_DMA_RST_W<'a> {
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
#[doc = "Field `I2S1_RST` reader - "]
pub struct I2S1_RST_R(crate::FieldReader<bool, bool>);
impl I2S1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S1_RST` writer - "]
pub struct I2S1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_RST_W<'a> {
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
#[doc = "Field `PWM1_RST` reader - "]
pub struct PWM1_RST_R(crate::FieldReader<bool, bool>);
impl PWM1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM1_RST` writer - "]
pub struct PWM1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_RST_W<'a> {
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
#[doc = "Field `TWAI_RST` reader - "]
pub struct TWAI_RST_R(crate::FieldReader<bool, bool>);
impl TWAI_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWAI_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWAI_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWAI_RST` writer - "]
pub struct TWAI_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TWAI_RST_W<'a> {
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
#[doc = "Field `I2C_EXT1_RST` reader - "]
pub struct I2C_EXT1_RST_R(crate::FieldReader<bool, bool>);
impl I2C_EXT1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EXT1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EXT1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_EXT1_RST` writer - "]
pub struct I2C_EXT1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EXT1_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PWM0_RST` reader - "]
pub struct PWM0_RST_R(crate::FieldReader<bool, bool>);
impl PWM0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM0_RST` writer - "]
pub struct PWM0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SPI3_RST` reader - "]
pub struct SPI3_RST_R(crate::FieldReader<bool, bool>);
impl SPI3_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3_RST` writer - "]
pub struct SPI3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TIMERGROUP1_RST` reader - "]
pub struct TIMERGROUP1_RST_R(crate::FieldReader<bool, bool>);
impl TIMERGROUP1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMERGROUP1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERGROUP1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERGROUP1_RST` writer - "]
pub struct TIMERGROUP1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERGROUP1_RST_W<'a> {
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
#[doc = "Field `EFUSE_RST` reader - "]
pub struct EFUSE_RST_R(crate::FieldReader<bool, bool>);
impl EFUSE_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_RST` writer - "]
pub struct EFUSE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RST_W<'a> {
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
#[doc = "Field `TIMERGROUP_RST` reader - "]
pub struct TIMERGROUP_RST_R(crate::FieldReader<bool, bool>);
impl TIMERGROUP_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMERGROUP_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERGROUP_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERGROUP_RST` writer - "]
pub struct TIMERGROUP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERGROUP_RST_W<'a> {
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
#[doc = "Field `UHCI1_RST` reader - "]
pub struct UHCI1_RST_R(crate::FieldReader<bool, bool>);
impl UHCI1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UHCI1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UHCI1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UHCI1_RST` writer - "]
pub struct UHCI1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI1_RST_W<'a> {
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
#[doc = "Field `LEDC_RST` reader - "]
pub struct LEDC_RST_R(crate::FieldReader<bool, bool>);
impl LEDC_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEDC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_RST` writer - "]
pub struct LEDC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_RST_W<'a> {
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
#[doc = "Field `PCNT_RST` reader - "]
pub struct PCNT_RST_R(crate::FieldReader<bool, bool>);
impl PCNT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCNT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCNT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCNT_RST` writer - "]
pub struct PCNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_RST_W<'a> {
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
#[doc = "Field `RMT_RST` reader - "]
pub struct RMT_RST_R(crate::FieldReader<bool, bool>);
impl RMT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMT_RST` writer - "]
pub struct RMT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RST_W<'a> {
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
#[doc = "Field `UHCI0_RST` reader - "]
pub struct UHCI0_RST_R(crate::FieldReader<bool, bool>);
impl UHCI0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UHCI0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UHCI0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UHCI0_RST` writer - "]
pub struct UHCI0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI0_RST_W<'a> {
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
#[doc = "Field `I2C_EXT0_RST` reader - "]
pub struct I2C_EXT0_RST_R(crate::FieldReader<bool, bool>);
impl I2C_EXT0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EXT0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EXT0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_EXT0_RST` writer - "]
pub struct I2C_EXT0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EXT0_RST_W<'a> {
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
#[doc = "Field `SPI2_RST` reader - "]
pub struct SPI2_RST_R(crate::FieldReader<bool, bool>);
impl SPI2_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2_RST` writer - "]
pub struct SPI2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `UART1_RST` reader - "]
pub struct UART1_RST_R(crate::FieldReader<bool, bool>);
impl UART1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_RST` writer - "]
pub struct UART1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RST_W<'a> {
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
#[doc = "Field `I2S0_RST` reader - "]
pub struct I2S0_RST_R(crate::FieldReader<bool, bool>);
impl I2S0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S0_RST` writer - "]
pub struct I2S0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_RST_W<'a> {
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
#[doc = "Field `WDG_RST` reader - "]
pub struct WDG_RST_R(crate::FieldReader<bool, bool>);
impl WDG_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDG_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDG_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDG_RST` writer - "]
pub struct WDG_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WDG_RST_W<'a> {
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
#[doc = "Field `UART_RST` reader - "]
pub struct UART_RST_R(crate::FieldReader<bool, bool>);
impl UART_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_RST` writer - "]
pub struct UART_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RST_W<'a> {
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
#[doc = "Field `SPI01_RST` reader - "]
pub struct SPI01_RST_R(crate::FieldReader<bool, bool>);
impl SPI01_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI01_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI01_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI01_RST` writer - "]
pub struct SPI01_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI01_RST_W<'a> {
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
#[doc = "Field `TIMERS_RST` reader - "]
pub struct TIMERS_RST_R(crate::FieldReader<bool, bool>);
impl TIMERS_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMERS_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERS_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERS_RST` writer - "]
pub struct TIMERS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERS_RST_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi4_rst(&self) -> SPI4_RST_R {
        SPI4_RST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc2_arb_rst(&self) -> ADC2_ARB_RST_R {
        ADC2_ARB_RST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn systimer_rst(&self) -> SYSTIMER_RST_R {
        SYSTIMER_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_rst(&self) -> APB_SARADC_RST_R {
        APB_SARADC_RST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi3_dma_rst(&self) -> SPI3_DMA_RST_R {
        SPI3_DMA_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_rst(&self) -> PWM3_RST_R {
        PWM3_RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_rst(&self) -> PWM2_RST_R {
        PWM2_RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_rst(&self) -> UART_MEM_RST_R {
        UART_MEM_RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn usb_device_rst(&self) -> USB_DEVICE_RST_R {
        USB_DEVICE_RST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi2_dma_rst(&self) -> SPI2_DMA_RST_R {
        SPI2_DMA_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S1_RST_R {
        I2S1_RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_rst(&self) -> PWM1_RST_R {
        PWM1_RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_rst(&self) -> TWAI_RST_R {
        TWAI_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_rst(&self) -> I2C_EXT1_RST_R {
        I2C_EXT1_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_rst(&self) -> PWM0_RST_R {
        PWM0_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_rst(&self) -> SPI3_RST_R {
        SPI3_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_rst(&self) -> TIMERGROUP1_RST_R {
        TIMERGROUP1_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_rst(&self) -> EFUSE_RST_R {
        EFUSE_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_rst(&self) -> TIMERGROUP_RST_R {
        TIMERGROUP_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_rst(&self) -> UHCI1_RST_R {
        UHCI1_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_rst(&self) -> LEDC_RST_R {
        LEDC_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_rst(&self) -> PCNT_RST_R {
        PCNT_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_rst(&self) -> RMT_RST_R {
        RMT_RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_rst(&self) -> UHCI0_RST_R {
        UHCI0_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_ext0_rst(&self) -> I2C_EXT0_RST_R {
        I2C_EXT0_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_rst(&self) -> SPI2_RST_R {
        SPI2_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_rst(&self) -> UART1_RST_R {
        UART1_RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S0_RST_R {
        I2S0_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_rst(&self) -> WDG_RST_R {
        WDG_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_rst(&self) -> UART_RST_R {
        UART_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_rst(&self) -> SPI01_RST_R {
        SPI01_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_rst(&self) -> TIMERS_RST_R {
        TIMERS_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi4_rst(&mut self) -> SPI4_RST_W {
        SPI4_RST_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc2_arb_rst(&mut self) -> ADC2_ARB_RST_W {
        ADC2_ARB_RST_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn systimer_rst(&mut self) -> SYSTIMER_RST_W {
        SYSTIMER_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_rst(&mut self) -> APB_SARADC_RST_W {
        APB_SARADC_RST_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi3_dma_rst(&mut self) -> SPI3_DMA_RST_W {
        SPI3_DMA_RST_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_rst(&mut self) -> PWM3_RST_W {
        PWM3_RST_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_rst(&mut self) -> PWM2_RST_W {
        PWM2_RST_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_rst(&mut self) -> UART_MEM_RST_W {
        UART_MEM_RST_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn usb_device_rst(&mut self) -> USB_DEVICE_RST_W {
        USB_DEVICE_RST_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi2_dma_rst(&mut self) -> SPI2_DMA_RST_W {
        SPI2_DMA_RST_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_rst(&mut self) -> I2S1_RST_W {
        I2S1_RST_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_rst(&mut self) -> PWM1_RST_W {
        PWM1_RST_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_rst(&mut self) -> TWAI_RST_W {
        TWAI_RST_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_rst(&mut self) -> I2C_EXT1_RST_W {
        I2C_EXT1_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_rst(&mut self) -> PWM0_RST_W {
        PWM0_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_rst(&mut self) -> SPI3_RST_W {
        SPI3_RST_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_rst(&mut self) -> TIMERGROUP1_RST_W {
        TIMERGROUP1_RST_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_rst(&mut self) -> EFUSE_RST_W {
        EFUSE_RST_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_rst(&mut self) -> TIMERGROUP_RST_W {
        TIMERGROUP_RST_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_rst(&mut self) -> UHCI1_RST_W {
        UHCI1_RST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_rst(&mut self) -> LEDC_RST_W {
        LEDC_RST_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_rst(&mut self) -> PCNT_RST_W {
        PCNT_RST_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_rst(&mut self) -> RMT_RST_W {
        RMT_RST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_rst(&mut self) -> UHCI0_RST_W {
        UHCI0_RST_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_ext0_rst(&mut self) -> I2C_EXT0_RST_W {
        I2C_EXT0_RST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_rst(&mut self) -> SPI2_RST_W {
        SPI2_RST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_rst(&mut self) -> UART1_RST_W {
        UART1_RST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_rst(&mut self) -> I2S0_RST_W {
        I2S0_RST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_rst(&mut self) -> WDG_RST_W {
        WDG_RST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_rst(&mut self) -> UART_RST_W {
        UART_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_rst(&mut self) -> SPI01_RST_W {
        SPI01_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_rst(&mut self) -> TIMERS_RST_W {
        TIMERS_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTEM_PERIP_RST_EN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en0](index.html) module"]
pub struct PERIP_RST_EN0_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en0::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en0::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_RST_EN0 to value 0"]
impl crate::Resettable for PERIP_RST_EN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
