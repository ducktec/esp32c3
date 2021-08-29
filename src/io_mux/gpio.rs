#[doc = "Register `GPIO%s` reader"]
pub struct R(crate::R<GPIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO%s` writer"]
pub struct W(crate::W<GPIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_SPEC>;
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
impl From<crate::W<GPIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER_EN` reader - enables filter for pin input signal"]
pub struct FILTER_EN_R(crate::FieldReader<bool, bool>);
impl FILTER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FILTER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILTER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTER_EN` writer - enables filter for pin input signal"]
pub struct FILTER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_EN_W<'a> {
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
#[doc = "Field `MCU_SEL` reader - configures IO_MUX function"]
pub struct MCU_SEL_R(crate::FieldReader<u8, u8>);
impl MCU_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCU_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_SEL` writer - configures IO_MUX function"]
pub struct MCU_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `FUN_DRV` reader - configures drive strength"]
pub struct FUN_DRV_R(crate::FieldReader<u8, u8>);
impl FUN_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FUN_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_DRV` writer - configures drive strength"]
pub struct FUN_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `FUN_IE` reader - configures input enable"]
pub struct FUN_IE_R(crate::FieldReader<bool, bool>);
impl FUN_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUN_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_IE` writer - configures input enable"]
pub struct FUN_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_IE_W<'a> {
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
#[doc = "Field `FUN_WPU` reader - configures pull up"]
pub struct FUN_WPU_R(crate::FieldReader<bool, bool>);
impl FUN_WPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUN_WPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_WPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_WPU` writer - configures pull up"]
pub struct FUN_WPU_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_WPU_W<'a> {
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
#[doc = "Field `FUN_WPD` reader - configures pull down"]
pub struct FUN_WPD_R(crate::FieldReader<bool, bool>);
impl FUN_WPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUN_WPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUN_WPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUN_WPD` writer - configures pull down"]
pub struct FUN_WPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FUN_WPD_W<'a> {
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
#[doc = "Field `MCU_IE` reader - configures input enable during sleep mode"]
pub struct MCU_IE_R(crate::FieldReader<bool, bool>);
impl MCU_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_IE` writer - configures input enable during sleep mode"]
pub struct MCU_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_IE_W<'a> {
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
#[doc = "Field `MCU_WPU` reader - configures pull up during sleep mode"]
pub struct MCU_WPU_R(crate::FieldReader<bool, bool>);
impl MCU_WPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_WPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_WPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_WPU` writer - configures pull up during sleep mode"]
pub struct MCU_WPU_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WPU_W<'a> {
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
#[doc = "Field `MCU_WPD` reader - configures pull down during sleep mode"]
pub struct MCU_WPD_R(crate::FieldReader<bool, bool>);
impl MCU_WPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_WPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_WPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_WPD` writer - configures pull down during sleep mode"]
pub struct MCU_WPD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WPD_W<'a> {
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
#[doc = "Field `SLP_SEL` reader - configures sleep mode selection"]
pub struct SLP_SEL_R(crate::FieldReader<bool, bool>);
impl SLP_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_SEL` writer - configures sleep mode selection"]
pub struct SLP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_SEL_W<'a> {
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
#[doc = "Field `MCU_OE` reader - configures output enable during sleep mode"]
pub struct MCU_OE_R(crate::FieldReader<bool, bool>);
impl MCU_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_OE` writer - configures output enable during sleep mode"]
pub struct MCU_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_OE_W<'a> {
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
    #[doc = "Bit 15 - enables filter for pin input signal"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - configures IO_MUX function"]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 10:11 - configures drive strength"]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - configures input enable"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - configures pull up"]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - configures pull down"]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 4 - configures input enable during sleep mode"]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - configures pull up during sleep mode"]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - configures pull down during sleep mode"]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - configures sleep mode selection"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - configures output enable during sleep mode"]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - enables filter for pin input signal"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W {
        FILTER_EN_W { w: self }
    }
    #[doc = "Bits 12:14 - configures IO_MUX function"]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W {
        MCU_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - configures drive strength"]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W {
        FUN_DRV_W { w: self }
    }
    #[doc = "Bit 9 - configures input enable"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W {
        FUN_IE_W { w: self }
    }
    #[doc = "Bit 8 - configures pull up"]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W {
        FUN_WPU_W { w: self }
    }
    #[doc = "Bit 7 - configures pull down"]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W {
        FUN_WPD_W { w: self }
    }
    #[doc = "Bit 4 - configures input enable during sleep mode"]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> MCU_IE_W {
        MCU_IE_W { w: self }
    }
    #[doc = "Bit 3 - configures pull up during sleep mode"]
    #[inline(always)]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W {
        MCU_WPU_W { w: self }
    }
    #[doc = "Bit 2 - configures pull down during sleep mode"]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W {
        MCU_WPD_W { w: self }
    }
    #[doc = "Bit 1 - configures sleep mode selection"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W {
        SLP_SEL_W { w: self }
    }
    #[doc = "Bit 0 - configures output enable during sleep mode"]
    #[inline(always)]
    pub fn mcu_oe(&mut self) -> MCU_OE_W {
        MCU_OE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configures IO_MUX for GPIO%s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio](index.html) module"]
pub struct GPIO_SPEC;
impl crate::RegisterSpec for GPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio::R](R) reader structure"]
impl crate::Readable for GPIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio::W](W) writer structure"]
impl crate::Writable for GPIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO%s to value 0x0580"]
impl crate::Resettable for GPIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0580
    }
}
