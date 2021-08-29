#[doc = "Register `INT_ENA_W1TS` writer"]
pub struct W(crate::W<INT_ENA_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_W1TS_SPEC>;
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
impl From<crate::W<INT_ENA_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BBPLL_CAL_INT_ENA_W1TS` writer - "]
pub struct BBPLL_CAL_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> BBPLL_CAL_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `GLITCH_DET_INT_ENA_W1TS` writer - "]
pub struct GLITCH_DET_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `XTAL32K_DEAD_INT_ENA_W1TS` writer - "]
pub struct XTAL32K_DEAD_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_DEAD_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `SWD_INT_ENA_W1TS` writer - "]
pub struct SWD_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `MAIN_TIMER_INT_ENA_W1TS` writer - "]
pub struct MAIN_TIMER_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_TIMER_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `BROWN_OUT_INT_ENA_W1TS` writer - "]
pub struct BROWN_OUT_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `WDT_INT_ENA_W1TS` writer - "]
pub struct WDT_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `SLP_REJECT_INT_ENA_W1TS` writer - "]
pub struct SLP_REJECT_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_REJECT_INT_ENA_W1TS_W<'a> {
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
#[doc = "Field `SLP_WAKEUP_INT_ENA_W1TS` writer - "]
pub struct SLP_WAKEUP_INT_ENA_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP_WAKEUP_INT_ENA_W1TS_W<'a> {
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
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bbpll_cal_int_ena_w1ts(&mut self) -> BBPLL_CAL_INT_ENA_W1TS_W {
        BBPLL_CAL_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn glitch_det_int_ena_w1ts(&mut self) -> GLITCH_DET_INT_ENA_W1TS_W {
        GLITCH_DET_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xtal32k_dead_int_ena_w1ts(&mut self) -> XTAL32K_DEAD_INT_ENA_W1TS_W {
        XTAL32K_DEAD_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swd_int_ena_w1ts(&mut self) -> SWD_INT_ENA_W1TS_W {
        SWD_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn main_timer_int_ena_w1ts(&mut self) -> MAIN_TIMER_INT_ENA_W1TS_W {
        MAIN_TIMER_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn brown_out_int_ena_w1ts(&mut self) -> BROWN_OUT_INT_ENA_W1TS_W {
        BROWN_OUT_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt_int_ena_w1ts(&mut self) -> WDT_INT_ENA_W1TS_W {
        WDT_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_reject_int_ena_w1ts(&mut self) -> SLP_REJECT_INT_ENA_W1TS_W {
        SLP_REJECT_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena_w1ts(&mut self) -> SLP_WAKEUP_INT_ENA_W1TS_W {
        SLP_WAKEUP_INT_ENA_W1TS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_INT_ENA_W1TS\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_w1ts](index.html) module"]
pub struct INT_ENA_W1TS_SPEC;
impl crate::RegisterSpec for INT_ENA_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_ena_w1ts::W](W) writer structure"]
impl crate::Writable for INT_ENA_W1TS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_W1TS to value 0"]
impl crate::Resettable for INT_ENA_W1TS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
