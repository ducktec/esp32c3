#[doc = "Register `TIME_UPDATE` reader"]
pub struct R(crate::R<TIME_UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME_UPDATE` writer"]
pub struct W(crate::W<TIME_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_UPDATE_SPEC>;
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
impl From<crate::W<TIME_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_UPDATE` writer - "]
pub struct TIME_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_UPDATE_W<'a> {
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
#[doc = "Field `TIMER_SYS_RST` reader - "]
pub struct TIMER_SYS_RST_R(crate::FieldReader<bool, bool>);
impl TIMER_SYS_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_SYS_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_SYS_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_SYS_RST` writer - "]
pub struct TIMER_SYS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SYS_RST_W<'a> {
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
#[doc = "Field `TIMER_XTL_OFF` reader - "]
pub struct TIMER_XTL_OFF_R(crate::FieldReader<bool, bool>);
impl TIMER_XTL_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_XTL_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_XTL_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_XTL_OFF` writer - "]
pub struct TIMER_XTL_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_XTL_OFF_W<'a> {
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
#[doc = "Field `TIMER_SYS_STALL` reader - "]
pub struct TIMER_SYS_STALL_R(crate::FieldReader<bool, bool>);
impl TIMER_SYS_STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_SYS_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_SYS_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_SYS_STALL` writer - "]
pub struct TIMER_SYS_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_SYS_STALL_W<'a> {
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
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timer_sys_rst(&self) -> TIMER_SYS_RST_R {
        TIMER_SYS_RST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn timer_xtl_off(&self) -> TIMER_XTL_OFF_R {
        TIMER_XTL_OFF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn timer_sys_stall(&self) -> TIMER_SYS_STALL_R {
        TIMER_SYS_STALL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn time_update(&mut self) -> TIME_UPDATE_W {
        TIME_UPDATE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timer_sys_rst(&mut self) -> TIMER_SYS_RST_W {
        TIMER_SYS_RST_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn timer_xtl_off(&mut self) -> TIMER_XTL_OFF_W {
        TIMER_XTL_OFF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn timer_sys_stall(&mut self) -> TIMER_SYS_STALL_W {
        TIMER_SYS_STALL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_TIME_UPDATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_update](index.html) module"]
pub struct TIME_UPDATE_SPEC;
impl crate::RegisterSpec for TIME_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_update::R](R) reader structure"]
impl crate::Readable for TIME_UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time_update::W](W) writer structure"]
impl crate::Writable for TIME_UPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIME_UPDATE to value 0"]
impl crate::Resettable for TIME_UPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
