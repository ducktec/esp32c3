#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BBPLL_CAL_INT_ST` reader - "]
pub struct BBPLL_CAL_INT_ST_R(crate::FieldReader<bool, bool>);
impl BBPLL_CAL_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBPLL_CAL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBPLL_CAL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_ST` reader - "]
pub struct GLITCH_DET_INT_ST_R(crate::FieldReader<bool, bool>);
impl GLITCH_DET_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_DEAD_INT_ST` reader - "]
pub struct XTAL32K_DEAD_INT_ST_R(crate::FieldReader<bool, bool>);
impl XTAL32K_DEAD_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_DEAD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_DEAD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWD_INT_ST` reader - "]
pub struct SWD_INT_ST_R(crate::FieldReader<bool, bool>);
impl SWD_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWD_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWD_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_TIMER_INT_ST` reader - "]
pub struct MAIN_TIMER_INT_ST_R(crate::FieldReader<bool, bool>);
impl MAIN_TIMER_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_TIMER_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_TIMER_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BROWN_OUT_INT_ST` reader - "]
pub struct BROWN_OUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl BROWN_OUT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BROWN_OUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BROWN_OUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_INT_ST` reader - "]
pub struct WDT_INT_ST_R(crate::FieldReader<bool, bool>);
impl WDT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_REJECT_INT_ST` reader - "]
pub struct SLP_REJECT_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLP_REJECT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLP_REJECT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_REJECT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ST` reader - "]
pub struct SLP_WAKEUP_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLP_WAKEUP_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLP_WAKEUP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP_WAKEUP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bbpll_cal_int_st(&self) -> BBPLL_CAL_INT_ST_R {
        BBPLL_CAL_INT_ST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> GLITCH_DET_INT_ST_R {
        GLITCH_DET_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xtal32k_dead_int_st(&self) -> XTAL32K_DEAD_INT_ST_R {
        XTAL32K_DEAD_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swd_int_st(&self) -> SWD_INT_ST_R {
        SWD_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn main_timer_int_st(&self) -> MAIN_TIMER_INT_ST_R {
        MAIN_TIMER_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn brown_out_int_st(&self) -> BROWN_OUT_INT_ST_R {
        BROWN_OUT_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt_int_st(&self) -> WDT_INT_ST_R {
        WDT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_reject_int_st(&self) -> SLP_REJECT_INT_ST_R {
        SLP_REJECT_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slp_wakeup_int_st(&self) -> SLP_WAKEUP_INT_ST_R {
        SLP_WAKEUP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RTC_CNTL_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
