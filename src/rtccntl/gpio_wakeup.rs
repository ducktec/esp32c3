#[doc = "Register `GPIO_WAKEUP` reader"]
pub struct R(crate::R<GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_WAKEUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_WAKEUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_WAKEUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_WAKEUP` writer"]
pub struct W(crate::W<GPIO_WAKEUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_WAKEUP_SPEC>;
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
impl From<crate::W<GPIO_WAKEUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_WAKEUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - "]
pub struct GPIO_PIN0_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN0_WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN0_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN0_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - "]
pub struct GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` reader - "]
pub struct GPIO_PIN1_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN1_WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN1_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN1_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN1_WAKEUP_ENABLE` writer - "]
pub struct GPIO_PIN1_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN1_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` reader - "]
pub struct GPIO_PIN2_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN2_WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN2_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN2_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN2_WAKEUP_ENABLE` writer - "]
pub struct GPIO_PIN2_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN2_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` reader - "]
pub struct GPIO_PIN3_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN3_WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN3_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN3_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN3_WAKEUP_ENABLE` writer - "]
pub struct GPIO_PIN3_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN3_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` reader - "]
pub struct GPIO_PIN4_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN4_WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN4_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN4_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN4_WAKEUP_ENABLE` writer - "]
pub struct GPIO_PIN4_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN4_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` reader - "]
pub struct GPIO_PIN5_WAKEUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN5_WAKEUP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN5_WAKEUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN5_WAKEUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN5_WAKEUP_ENABLE` writer - "]
pub struct GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN5_WAKEUP_ENABLE_W<'a> {
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
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - "]
pub struct GPIO_PIN0_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN0_INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN0_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN0_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - "]
pub struct GPIO_PIN0_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN0_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `GPIO_PIN1_INT_TYPE` reader - "]
pub struct GPIO_PIN1_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN1_INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN1_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN1_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN1_INT_TYPE` writer - "]
pub struct GPIO_PIN1_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN1_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `GPIO_PIN2_INT_TYPE` reader - "]
pub struct GPIO_PIN2_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN2_INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN2_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN2_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN2_INT_TYPE` writer - "]
pub struct GPIO_PIN2_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN2_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `GPIO_PIN3_INT_TYPE` reader - "]
pub struct GPIO_PIN3_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN3_INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN3_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN3_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN3_INT_TYPE` writer - "]
pub struct GPIO_PIN3_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN3_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `GPIO_PIN4_INT_TYPE` reader - "]
pub struct GPIO_PIN4_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN4_INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN4_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN4_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN4_INT_TYPE` writer - "]
pub struct GPIO_PIN4_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN4_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `GPIO_PIN5_INT_TYPE` reader - "]
pub struct GPIO_PIN5_INT_TYPE_R(crate::FieldReader<u8, u8>);
impl GPIO_PIN5_INT_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_PIN5_INT_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN5_INT_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN5_INT_TYPE` writer - "]
pub struct GPIO_PIN5_INT_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN5_INT_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `GPIO_PIN_CLK_GATE` reader - "]
pub struct GPIO_PIN_CLK_GATE_R(crate::FieldReader<bool, bool>);
impl GPIO_PIN_CLK_GATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_PIN_CLK_GATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_PIN_CLK_GATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_PIN_CLK_GATE` writer - "]
pub struct GPIO_PIN_CLK_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_PIN_CLK_GATE_W<'a> {
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
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` reader - "]
pub struct GPIO_WAKEUP_STATUS_CLR_R(crate::FieldReader<bool, bool>);
impl GPIO_WAKEUP_STATUS_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_WAKEUP_STATUS_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_WAKEUP_STATUS_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_WAKEUP_STATUS_CLR` writer - "]
pub struct GPIO_WAKEUP_STATUS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WAKEUP_STATUS_CLR_W<'a> {
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
#[doc = "Field `GPIO_WAKEUP_STATUS` reader - "]
pub struct GPIO_WAKEUP_STATUS_R(crate::FieldReader<u8, u8>);
impl GPIO_WAKEUP_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_WAKEUP_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_WAKEUP_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&self) -> GPIO_PIN1_WAKEUP_ENABLE_R {
        GPIO_PIN1_WAKEUP_ENABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&self) -> GPIO_PIN2_WAKEUP_ENABLE_R {
        GPIO_PIN2_WAKEUP_ENABLE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&self) -> GPIO_PIN3_WAKEUP_ENABLE_R {
        GPIO_PIN3_WAKEUP_ENABLE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&self) -> GPIO_PIN4_WAKEUP_ENABLE_R {
        GPIO_PIN4_WAKEUP_ENABLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&self) -> GPIO_PIN5_WAKEUP_ENABLE_R {
        GPIO_PIN5_WAKEUP_ENABLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&self) -> GPIO_PIN1_INT_TYPE_R {
        GPIO_PIN1_INT_TYPE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&self) -> GPIO_PIN2_INT_TYPE_R {
        GPIO_PIN2_INT_TYPE_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&self) -> GPIO_PIN3_INT_TYPE_R {
        GPIO_PIN3_INT_TYPE_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&self) -> GPIO_PIN4_INT_TYPE_R {
        GPIO_PIN4_INT_TYPE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&self) -> GPIO_PIN5_INT_TYPE_R {
        GPIO_PIN5_INT_TYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&self) -> GPIO_PIN_CLK_GATE_R {
        GPIO_PIN_CLK_GATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&self) -> GPIO_WAKEUP_STATUS_CLR_R {
        GPIO_WAKEUP_STATUS_CLR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpio_wakeup_status(&self) -> GPIO_WAKEUP_STATUS_R {
        GPIO_WAKEUP_STATUS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W {
        GPIO_PIN0_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpio_pin1_wakeup_enable(&mut self) -> GPIO_PIN1_WAKEUP_ENABLE_W {
        GPIO_PIN1_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn gpio_pin2_wakeup_enable(&mut self) -> GPIO_PIN2_WAKEUP_ENABLE_W {
        GPIO_PIN2_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn gpio_pin3_wakeup_enable(&mut self) -> GPIO_PIN3_WAKEUP_ENABLE_W {
        GPIO_PIN3_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpio_pin4_wakeup_enable(&mut self) -> GPIO_PIN4_WAKEUP_ENABLE_W {
        GPIO_PIN4_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpio_pin5_wakeup_enable(&mut self) -> GPIO_PIN5_WAKEUP_ENABLE_W {
        GPIO_PIN5_WAKEUP_ENABLE_W { w: self }
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W {
        GPIO_PIN0_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn gpio_pin1_int_type(&mut self) -> GPIO_PIN1_INT_TYPE_W {
        GPIO_PIN1_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn gpio_pin2_int_type(&mut self) -> GPIO_PIN2_INT_TYPE_W {
        GPIO_PIN2_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn gpio_pin3_int_type(&mut self) -> GPIO_PIN3_INT_TYPE_W {
        GPIO_PIN3_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn gpio_pin4_int_type(&mut self) -> GPIO_PIN4_INT_TYPE_W {
        GPIO_PIN4_INT_TYPE_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn gpio_pin5_int_type(&mut self) -> GPIO_PIN5_INT_TYPE_W {
        GPIO_PIN5_INT_TYPE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpio_pin_clk_gate(&mut self) -> GPIO_PIN_CLK_GATE_W {
        GPIO_PIN_CLK_GATE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpio_wakeup_status_clr(&mut self) -> GPIO_WAKEUP_STATUS_CLR_W {
        GPIO_WAKEUP_STATUS_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_GPIO_WAKEUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_wakeup](index.html) module"]
pub struct GPIO_WAKEUP_SPEC;
impl crate::RegisterSpec for GPIO_WAKEUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_wakeup::R](R) reader structure"]
impl crate::Readable for GPIO_WAKEUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_wakeup::W](W) writer structure"]
impl crate::Writable for GPIO_WAKEUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_WAKEUP to value 0"]
impl crate::Resettable for GPIO_WAKEUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
