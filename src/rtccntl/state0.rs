#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATE0` writer"]
pub struct W(crate::W<STATE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATE0_SPEC>;
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
impl From<crate::W<STATE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_EN` reader - "]
pub struct SLEEP_EN_R(crate::FieldReader<bool, bool>);
impl SLEEP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP_EN` writer - "]
pub struct SLEEP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_EN_W<'a> {
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
#[doc = "Field `SLP_REJECT` reader - "]
pub struct SLP_REJECT_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT` writer - "]
pub struct SLP_REJECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_W<'a> {
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
#[doc = "Field `SLP_WAKEUP` reader - "]
pub struct SLP_WAKEUP_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_WAKEUP` writer - "]
pub struct SLP_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_W<'a> {
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
#[doc = "Field `SDIO_ACTIVE_IND` reader - "]
pub struct SDIO_ACTIVE_IND_R(crate::FieldReader<bool, bool>);
impl SDIO_ACTIVE_IND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_ACTIVE_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_ACTIVE_IND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB2RTC_BRIDGE_SEL` reader - "]
pub struct APB2RTC_BRIDGE_SEL_R(crate::FieldReader<bool, bool>);
impl APB2RTC_BRIDGE_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB2RTC_BRIDGE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB2RTC_BRIDGE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB2RTC_BRIDGE_SEL` writer - "]
pub struct APB2RTC_BRIDGE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2RTC_BRIDGE_SEL_W<'a> {
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
#[doc = "Field `SLP_REJECT_CAUSE_CLR` writer - "]
pub struct SLP_REJECT_CAUSE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_CAUSE_CLR_W<'a> {
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
#[doc = "Field `SW_CPU_INT` writer - "]
pub struct SW_CPU_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_CPU_INT_W<'a> {
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
    pub fn sleep_en(&self) -> SLEEP_EN_R {
        SLEEP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slp_reject(&self) -> SLP_REJECT_R {
        SLP_REJECT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slp_wakeup(&self) -> SLP_WAKEUP_R {
        SLP_WAKEUP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sdio_active_ind(&self) -> SDIO_ACTIVE_IND_R {
        SDIO_ACTIVE_IND_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&self) -> APB2RTC_BRIDGE_SEL_R {
        APB2RTC_BRIDGE_SEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sleep_en(&mut self) -> SLEEP_EN_W {
        SLEEP_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn slp_reject(&mut self) -> SLP_REJECT_W {
        SLP_REJECT_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn slp_wakeup(&mut self) -> SLP_WAKEUP_W {
        SLP_WAKEUP_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn apb2rtc_bridge_sel(&mut self) -> APB2RTC_BRIDGE_SEL_W {
        APB2RTC_BRIDGE_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_reject_cause_clr(&mut self) -> SLP_REJECT_CAUSE_CLR_W {
        SLP_REJECT_CAUSE_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_cpu_int(&mut self) -> SW_CPU_INT_W {
        SW_CPU_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [state0::W](W) writer structure"]
impl crate::Writable for STATE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
