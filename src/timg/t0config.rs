#[doc = "Register `T0CONFIG` reader"]
pub struct R(crate::R<T0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CONFIG` writer"]
pub struct W(crate::W<T0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CONFIG_SPEC>;
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
impl From<crate::W<T0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_EN` reader - "]
pub struct T0_EN_R(crate::FieldReader<bool, bool>);
impl T0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        T0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_EN` writer - "]
pub struct T0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_EN_W<'a> {
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
#[doc = "Field `T0_INCREASE` reader - "]
pub struct T0_INCREASE_R(crate::FieldReader<bool, bool>);
impl T0_INCREASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T0_INCREASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_INCREASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_INCREASE` writer - "]
pub struct T0_INCREASE_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_INCREASE_W<'a> {
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
#[doc = "Field `T0_AUTORELOAD` reader - "]
pub struct T0_AUTORELOAD_R(crate::FieldReader<bool, bool>);
impl T0_AUTORELOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        T0_AUTORELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_AUTORELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_AUTORELOAD` writer - "]
pub struct T0_AUTORELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_AUTORELOAD_W<'a> {
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
#[doc = "Field `T0_DIVIDER` reader - "]
pub struct T0_DIVIDER_R(crate::FieldReader<u16, u16>);
impl T0_DIVIDER_R {
    pub(crate) fn new(bits: u16) -> Self {
        T0_DIVIDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_DIVIDER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_DIVIDER` writer - "]
pub struct T0_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 13)) | ((value as u32 & 0xffff) << 13);
        self.w
    }
}
#[doc = "Field `T0_DIVCNT_RST` writer - "]
pub struct T0_DIVCNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_DIVCNT_RST_W<'a> {
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
#[doc = "Field `T0_ALARM_EN` reader - "]
pub struct T0_ALARM_EN_R(crate::FieldReader<bool, bool>);
impl T0_ALARM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        T0_ALARM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_ALARM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_ALARM_EN` writer - "]
pub struct T0_ALARM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_ALARM_EN_W<'a> {
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
#[doc = "Field `T0_USE_XTAL` reader - "]
pub struct T0_USE_XTAL_R(crate::FieldReader<bool, bool>);
impl T0_USE_XTAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        T0_USE_XTAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T0_USE_XTAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T0_USE_XTAL` writer - "]
pub struct T0_USE_XTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_USE_XTAL_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn t0_en(&self) -> T0_EN_R {
        T0_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn t0_increase(&self) -> T0_INCREASE_R {
        T0_INCREASE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn t0_autoreload(&self) -> T0_AUTORELOAD_R {
        T0_AUTORELOAD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn t0_divider(&self) -> T0_DIVIDER_R {
        T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn t0_alarm_en(&self) -> T0_ALARM_EN_R {
        T0_ALARM_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t0_use_xtal(&self) -> T0_USE_XTAL_R {
        T0_USE_XTAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn t0_en(&mut self) -> T0_EN_W {
        T0_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn t0_increase(&mut self) -> T0_INCREASE_W {
        T0_INCREASE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn t0_autoreload(&mut self) -> T0_AUTORELOAD_W {
        T0_AUTORELOAD_W { w: self }
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn t0_divider(&mut self) -> T0_DIVIDER_W {
        T0_DIVIDER_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn t0_divcnt_rst(&mut self) -> T0_DIVCNT_RST_W {
        T0_DIVCNT_RST_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn t0_alarm_en(&mut self) -> T0_ALARM_EN_W {
        T0_ALARM_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn t0_use_xtal(&mut self) -> T0_USE_XTAL_W {
        T0_USE_XTAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_T0CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](index.html) module"]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0config::R](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0config::W](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0CONFIG to value 0"]
impl crate::Resettable for T0CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
