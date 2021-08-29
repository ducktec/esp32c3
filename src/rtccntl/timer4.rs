#[doc = "Register `TIMER4` reader"]
pub struct R(crate::R<TIMER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER4` writer"]
pub struct W(crate::W<TIMER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER4_SPEC>;
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
impl From<crate::W<TIMER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - "]
pub struct DG_WRAP_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl DG_WRAP_POWERUP_TIMER_R {
    pub(crate) fn new(bits: u8) -> Self {
        DG_WRAP_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - "]
pub struct DG_WRAP_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - "]
pub struct DG_WRAP_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl DG_WRAP_WAIT_TIMER_R {
    pub(crate) fn new(bits: u16) -> Self {
        DG_WRAP_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - "]
pub struct DG_WRAP_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `CPU_TOP_POWERUP_TIMER` reader - "]
pub struct CPU_TOP_POWERUP_TIMER_R(crate::FieldReader<u8, u8>);
impl CPU_TOP_POWERUP_TIMER_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TOP_POWERUP_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_POWERUP_TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_POWERUP_TIMER` writer - "]
pub struct CPU_TOP_POWERUP_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_POWERUP_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `CPU_TOP_WAIT_TIMER` reader - "]
pub struct CPU_TOP_WAIT_TIMER_R(crate::FieldReader<u16, u16>);
impl CPU_TOP_WAIT_TIMER_R {
    pub(crate) fn new(bits: u16) -> Self {
        CPU_TOP_WAIT_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_WAIT_TIMER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_WAIT_TIMER` writer - "]
pub struct CPU_TOP_WAIT_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_WAIT_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn cpu_top_powerup_timer(&self) -> CPU_TOP_POWERUP_TIMER_R {
        CPU_TOP_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn cpu_top_wait_timer(&self) -> CPU_TOP_WAIT_TIMER_R {
        CPU_TOP_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W {
        DG_WRAP_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W {
        DG_WRAP_WAIT_TIMER_W { w: self }
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn cpu_top_powerup_timer(&mut self) -> CPU_TOP_POWERUP_TIMER_W {
        CPU_TOP_POWERUP_TIMER_W { w: self }
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn cpu_top_wait_timer(&mut self) -> CPU_TOP_WAIT_TIMER_W {
        CPU_TOP_WAIT_TIMER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_TIMER4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer4](index.html) module"]
pub struct TIMER4_SPEC;
impl crate::RegisterSpec for TIMER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer4::R](R) reader structure"]
impl crate::Readable for TIMER4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer4::W](W) writer structure"]
impl crate::Writable for TIMER4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER4 to value 0"]
impl crate::Resettable for TIMER4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
