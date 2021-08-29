#[doc = "Register `PERIP_CLK_EN0` reader"]
pub struct R(crate::R<PERIP_CLK_EN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN0` writer"]
pub struct W(crate::W<PERIP_CLK_EN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN0_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI4_CLK_EN` reader - "]
pub struct SPI4_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI4_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI4_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI4_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI4_CLK_EN` writer - "]
pub struct SPI4_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4_CLK_EN_W<'a> {
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
#[doc = "Field `ADC2_ARB_CLK_EN` reader - "]
pub struct ADC2_ARB_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ADC2_ARB_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_ARB_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_ARB_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_ARB_CLK_EN` writer - "]
pub struct ADC2_ARB_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_ARB_CLK_EN_W<'a> {
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
#[doc = "Field `SYSTIMER_CLK_EN` reader - "]
pub struct SYSTIMER_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SYSTIMER_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSTIMER_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTIMER_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSTIMER_CLK_EN` writer - "]
pub struct SYSTIMER_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTIMER_CLK_EN_W<'a> {
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
#[doc = "Field `APB_SARADC_CLK_EN` reader - "]
pub struct APB_SARADC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl APB_SARADC_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_SARADC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SARADC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SARADC_CLK_EN` writer - "]
pub struct APB_SARADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SARADC_CLK_EN_W<'a> {
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
#[doc = "Field `SPI3_DMA_CLK_EN` reader - "]
pub struct SPI3_DMA_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI3_DMA_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3_DMA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3_DMA_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3_DMA_CLK_EN` writer - "]
pub struct SPI3_DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_DMA_CLK_EN_W<'a> {
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
#[doc = "Field `PWM3_CLK_EN` reader - "]
pub struct PWM3_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PWM3_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM3_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM3_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM3_CLK_EN` writer - "]
pub struct PWM3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM3_CLK_EN_W<'a> {
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
#[doc = "Field `PWM2_CLK_EN` reader - "]
pub struct PWM2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PWM2_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM2_CLK_EN` writer - "]
pub struct PWM2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2_CLK_EN_W<'a> {
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
#[doc = "Field `UART_MEM_CLK_EN` reader - "]
pub struct UART_MEM_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART_MEM_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_MEM_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_MEM_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_MEM_CLK_EN` writer - "]
pub struct UART_MEM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_CLK_EN_W<'a> {
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
#[doc = "Field `USB_DEVICE_CLK_EN` reader - "]
pub struct USB_DEVICE_CLK_EN_R(crate::FieldReader<bool, bool>);
impl USB_DEVICE_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_DEVICE_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DEVICE_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_DEVICE_CLK_EN` writer - "]
pub struct USB_DEVICE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DEVICE_CLK_EN_W<'a> {
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
#[doc = "Field `SPI2_DMA_CLK_EN` reader - "]
pub struct SPI2_DMA_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI2_DMA_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_DMA_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_DMA_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2_DMA_CLK_EN` writer - "]
pub struct SPI2_DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_DMA_CLK_EN_W<'a> {
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
#[doc = "Field `I2S1_CLK_EN` reader - "]
pub struct I2S1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2S1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S1_CLK_EN` writer - "]
pub struct I2S1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_CLK_EN_W<'a> {
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
#[doc = "Field `PWM1_CLK_EN` reader - "]
pub struct PWM1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PWM1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM1_CLK_EN` writer - "]
pub struct PWM1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_CLK_EN_W<'a> {
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
#[doc = "Field `TWAI_CLK_EN` reader - "]
pub struct TWAI_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TWAI_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWAI_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWAI_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWAI_CLK_EN` writer - "]
pub struct TWAI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TWAI_CLK_EN_W<'a> {
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
#[doc = "Field `I2C_EXT1_CLK_EN` reader - "]
pub struct I2C_EXT1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2C_EXT1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EXT1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EXT1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_EXT1_CLK_EN` writer - "]
pub struct I2C_EXT1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EXT1_CLK_EN_W<'a> {
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
#[doc = "Field `PWM0_CLK_EN` reader - "]
pub struct PWM0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PWM0_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWM0_CLK_EN` writer - "]
pub struct PWM0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_CLK_EN_W<'a> {
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
#[doc = "Field `SPI3_CLK_EN` reader - "]
pub struct SPI3_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI3_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI3_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI3_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI3_CLK_EN` writer - "]
pub struct SPI3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_CLK_EN_W<'a> {
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
#[doc = "Field `TIMERGROUP1_CLK_EN` reader - "]
pub struct TIMERGROUP1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIMERGROUP1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMERGROUP1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERGROUP1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERGROUP1_CLK_EN` writer - "]
pub struct TIMERGROUP1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERGROUP1_CLK_EN_W<'a> {
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
#[doc = "Field `EFUSE_CLK_EN` reader - "]
pub struct EFUSE_CLK_EN_R(crate::FieldReader<bool, bool>);
impl EFUSE_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EFUSE_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_CLK_EN` writer - "]
pub struct EFUSE_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CLK_EN_W<'a> {
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
#[doc = "Field `TIMERGROUP_CLK_EN` reader - "]
pub struct TIMERGROUP_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIMERGROUP_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMERGROUP_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERGROUP_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERGROUP_CLK_EN` writer - "]
pub struct TIMERGROUP_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERGROUP_CLK_EN_W<'a> {
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
#[doc = "Field `UHCI1_CLK_EN` reader - "]
pub struct UHCI1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UHCI1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UHCI1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UHCI1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UHCI1_CLK_EN` writer - "]
pub struct UHCI1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI1_CLK_EN_W<'a> {
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
#[doc = "Field `LEDC_CLK_EN` reader - "]
pub struct LEDC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl LEDC_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEDC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEDC_CLK_EN` writer - "]
pub struct LEDC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_CLK_EN_W<'a> {
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
#[doc = "Field `PCNT_CLK_EN` reader - "]
pub struct PCNT_CLK_EN_R(crate::FieldReader<bool, bool>);
impl PCNT_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCNT_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCNT_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCNT_CLK_EN` writer - "]
pub struct PCNT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_CLK_EN_W<'a> {
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
#[doc = "Field `RMT_CLK_EN` reader - "]
pub struct RMT_CLK_EN_R(crate::FieldReader<bool, bool>);
impl RMT_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMT_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMT_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMT_CLK_EN` writer - "]
pub struct RMT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CLK_EN_W<'a> {
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
#[doc = "Field `UHCI0_CLK_EN` reader - "]
pub struct UHCI0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UHCI0_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UHCI0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UHCI0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UHCI0_CLK_EN` writer - "]
pub struct UHCI0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI0_CLK_EN_W<'a> {
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
#[doc = "Field `I2C_EXT0_CLK_EN` reader - "]
pub struct I2C_EXT0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2C_EXT0_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_EXT0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_EXT0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_EXT0_CLK_EN` writer - "]
pub struct I2C_EXT0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_EXT0_CLK_EN_W<'a> {
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
#[doc = "Field `SPI2_CLK_EN` reader - "]
pub struct SPI2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI2_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2_CLK_EN` writer - "]
pub struct SPI2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_CLK_EN_W<'a> {
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
#[doc = "Field `UART1_CLK_EN` reader - "]
pub struct UART1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_CLK_EN` writer - "]
pub struct UART1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CLK_EN_W<'a> {
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
#[doc = "Field `I2S0_CLK_EN` reader - "]
pub struct I2S0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2S0_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S0_CLK_EN` writer - "]
pub struct I2S0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_CLK_EN_W<'a> {
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
#[doc = "Field `WDG_CLK_EN` reader - "]
pub struct WDG_CLK_EN_R(crate::FieldReader<bool, bool>);
impl WDG_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDG_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDG_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDG_CLK_EN` writer - "]
pub struct WDG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDG_CLK_EN_W<'a> {
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
#[doc = "Field `UART_CLK_EN` reader - "]
pub struct UART_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_CLK_EN` writer - "]
pub struct UART_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_EN_W<'a> {
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
#[doc = "Field `SPI01_CLK_EN` reader - "]
pub struct SPI01_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI01_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI01_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI01_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI01_CLK_EN` writer - "]
pub struct SPI01_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI01_CLK_EN_W<'a> {
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
#[doc = "Field `TIMERS_CLK_EN` reader - "]
pub struct TIMERS_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIMERS_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMERS_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMERS_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMERS_CLK_EN` writer - "]
pub struct TIMERS_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERS_CLK_EN_W<'a> {
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
    pub fn spi4_clk_en(&self) -> SPI4_CLK_EN_R {
        SPI4_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&self) -> ADC2_ARB_CLK_EN_R {
        ADC2_ARB_CLK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&self) -> APB_SARADC_CLK_EN_R {
        APB_SARADC_CLK_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&self) -> SPI3_DMA_CLK_EN_R {
        SPI3_DMA_CLK_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_clk_en(&self) -> PWM3_CLK_EN_R {
        PWM3_CLK_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_clk_en(&self) -> PWM2_CLK_EN_R {
        PWM2_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&self) -> UART_MEM_CLK_EN_R {
        UART_MEM_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn usb_device_clk_en(&self) -> USB_DEVICE_CLK_EN_R {
        USB_DEVICE_CLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&self) -> SPI2_DMA_CLK_EN_R {
        SPI2_DMA_CLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_clk_en(&self) -> TWAI_CLK_EN_R {
        TWAI_CLK_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&self) -> I2C_EXT1_CLK_EN_R {
        I2C_EXT1_CLK_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&self) -> TIMERGROUP1_CLK_EN_R {
        TIMERGROUP1_CLK_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_clk_en(&self) -> EFUSE_CLK_EN_R {
        EFUSE_CLK_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_clk_en(&self) -> TIMERGROUP_CLK_EN_R {
        TIMERGROUP_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_clk_en(&self) -> UHCI1_CLK_EN_R {
        UHCI1_CLK_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_clk_en(&self) -> LEDC_CLK_EN_R {
        LEDC_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_clk_en(&self) -> UHCI0_CLK_EN_R {
        UHCI0_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&self) -> I2C_EXT0_CLK_EN_R {
        I2C_EXT0_CLK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_clk_en(&self) -> WDG_CLK_EN_R {
        WDG_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UART_CLK_EN_R {
        UART_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_clk_en(&self) -> SPI01_CLK_EN_R {
        SPI01_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_clk_en(&self) -> TIMERS_CLK_EN_R {
        TIMERS_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi4_clk_en(&mut self) -> SPI4_CLK_EN_W {
        SPI4_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc2_arb_clk_en(&mut self) -> ADC2_ARB_CLK_EN_W {
        ADC2_ARB_CLK_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W {
        SYSTIMER_CLK_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn apb_saradc_clk_en(&mut self) -> APB_SARADC_CLK_EN_W {
        APB_SARADC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi3_dma_clk_en(&mut self) -> SPI3_DMA_CLK_EN_W {
        SPI3_DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pwm3_clk_en(&mut self) -> PWM3_CLK_EN_W {
        PWM3_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwm2_clk_en(&mut self) -> PWM2_CLK_EN_W {
        PWM2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart_mem_clk_en(&mut self) -> UART_MEM_CLK_EN_W {
        UART_MEM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn usb_device_clk_en(&mut self) -> USB_DEVICE_CLK_EN_W {
        USB_DEVICE_CLK_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn spi2_dma_clk_en(&mut self) -> SPI2_DMA_CLK_EN_W {
        SPI2_DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W {
        I2S1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W {
        PWM1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn twai_clk_en(&mut self) -> TWAI_CLK_EN_W {
        TWAI_CLK_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn i2c_ext1_clk_en(&mut self) -> I2C_EXT1_CLK_EN_W {
        I2C_EXT1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W {
        PWM0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W {
        SPI3_CLK_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timergroup1_clk_en(&mut self) -> TIMERGROUP1_CLK_EN_W {
        TIMERGROUP1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn efuse_clk_en(&mut self) -> EFUSE_CLK_EN_W {
        EFUSE_CLK_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timergroup_clk_en(&mut self) -> TIMERGROUP_CLK_EN_W {
        TIMERGROUP_CLK_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn uhci1_clk_en(&mut self) -> UHCI1_CLK_EN_W {
        UHCI1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ledc_clk_en(&mut self) -> LEDC_CLK_EN_W {
        LEDC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W {
        PCNT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W {
        RMT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn uhci0_clk_en(&mut self) -> UHCI0_CLK_EN_W {
        UHCI0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2c_ext0_clk_en(&mut self) -> I2C_EXT0_CLK_EN_W {
        I2C_EXT0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W {
        SPI2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W {
        I2S0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdg_clk_en(&mut self) -> WDG_CLK_EN_W {
        WDG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_clk_en(&mut self) -> UART_CLK_EN_W {
        UART_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi01_clk_en(&mut self) -> SPI01_CLK_EN_W {
        SPI01_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timers_clk_en(&mut self) -> TIMERS_CLK_EN_W {
        TIMERS_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTEM_PERIP_CLK_EN0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en0](index.html) module"]
pub struct PERIP_CLK_EN0_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en0::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en0::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_CLK_EN0 to value 0"]
impl crate::Resettable for PERIP_CLK_EN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
