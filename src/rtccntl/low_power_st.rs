#[doc = "Register `LOW_POWER_ST` reader"]
pub struct R(crate::R<LOW_POWER_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_POWER_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_POWER_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_POWER_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAIN_STATE` reader - "]
pub struct MAIN_STATE_R(crate::FieldReader<u8, u8>);
impl MAIN_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAIN_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_IN_IDLE` reader - "]
pub struct MAIN_STATE_IN_IDLE_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_IN_IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_IN_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_IN_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_IN_SLP` reader - "]
pub struct MAIN_STATE_IN_SLP_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_IN_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_IN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_IN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_IN_WAIT_XTL` reader - "]
pub struct MAIN_STATE_IN_WAIT_XTL_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_IN_WAIT_XTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_IN_WAIT_XTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_IN_WAIT_XTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_IN_WAIT_PLL` reader - "]
pub struct MAIN_STATE_IN_WAIT_PLL_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_IN_WAIT_PLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_IN_WAIT_PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_IN_WAIT_PLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_IN_WAIT_8M` reader - "]
pub struct MAIN_STATE_IN_WAIT_8M_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_IN_WAIT_8M_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_IN_WAIT_8M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_IN_WAIT_8M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_LOW_POWER_STATE` reader - "]
pub struct IN_LOW_POWER_STATE_R(crate::FieldReader<bool, bool>);
impl IN_LOW_POWER_STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_LOW_POWER_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_LOW_POWER_STATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_WAKEUP_STATE` reader - "]
pub struct IN_WAKEUP_STATE_R(crate::FieldReader<bool, bool>);
impl IN_WAKEUP_STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN_WAKEUP_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_WAKEUP_STATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_WAIT_END` reader - "]
pub struct MAIN_STATE_WAIT_END_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_WAIT_END_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_WAIT_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_WAIT_END_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDY_FOR_WAKEUP` reader - "]
pub struct RDY_FOR_WAKEUP_R(crate::FieldReader<bool, bool>);
impl RDY_FOR_WAKEUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDY_FOR_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDY_FOR_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_PLL_ON` reader - "]
pub struct MAIN_STATE_PLL_ON_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_PLL_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_PLL_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_PLL_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAIN_STATE_XTAL_ISO` reader - "]
pub struct MAIN_STATE_XTAL_ISO_R(crate::FieldReader<bool, bool>);
impl MAIN_STATE_XTAL_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAIN_STATE_XTAL_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAIN_STATE_XTAL_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_STATE_DONE` reader - "]
pub struct COCPU_STATE_DONE_R(crate::FieldReader<bool, bool>);
impl COCPU_STATE_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_STATE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_STATE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_STATE_SLP` reader - "]
pub struct COCPU_STATE_SLP_R(crate::FieldReader<bool, bool>);
impl COCPU_STATE_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_STATE_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_STATE_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_STATE_SWITCH` reader - "]
pub struct COCPU_STATE_SWITCH_R(crate::FieldReader<bool, bool>);
impl COCPU_STATE_SWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_STATE_SWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_STATE_SWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COCPU_STATE_START` reader - "]
pub struct COCPU_STATE_START_R(crate::FieldReader<bool, bool>);
impl COCPU_STATE_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        COCPU_STATE_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COCPU_STATE_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_STATE_DONE` reader - "]
pub struct TOUCH_STATE_DONE_R(crate::FieldReader<bool, bool>);
impl TOUCH_STATE_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_STATE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_STATE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_STATE_SLP` reader - "]
pub struct TOUCH_STATE_SLP_R(crate::FieldReader<bool, bool>);
impl TOUCH_STATE_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_STATE_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_STATE_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_STATE_SWITCH` reader - "]
pub struct TOUCH_STATE_SWITCH_R(crate::FieldReader<bool, bool>);
impl TOUCH_STATE_SWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_STATE_SWITCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_STATE_SWITCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOUCH_STATE_START` reader - "]
pub struct TOUCH_STATE_START_R(crate::FieldReader<bool, bool>);
impl TOUCH_STATE_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOUCH_STATE_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOUCH_STATE_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_DIG` reader - "]
pub struct XPD_DIG_R(crate::FieldReader<bool, bool>);
impl XPD_DIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_DIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_DIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_ISO` reader - "]
pub struct DIG_ISO_R(crate::FieldReader<bool, bool>);
impl DIG_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_WIFI` reader - "]
pub struct XPD_WIFI_R(crate::FieldReader<bool, bool>);
impl XPD_WIFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_WIFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_WIFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_ISO` reader - "]
pub struct WIFI_ISO_R(crate::FieldReader<bool, bool>);
impl WIFI_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_RTC_PERI` reader - "]
pub struct XPD_RTC_PERI_R(crate::FieldReader<bool, bool>);
impl XPD_RTC_PERI_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_RTC_PERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_RTC_PERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_ISO` reader - "]
pub struct PERI_ISO_R(crate::FieldReader<bool, bool>);
impl PERI_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_DIG_DCDC` reader - "]
pub struct XPD_DIG_DCDC_R(crate::FieldReader<bool, bool>);
impl XPD_DIG_DCDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_DIG_DCDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_DIG_DCDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_ROM0` reader - "]
pub struct XPD_ROM0_R(crate::FieldReader<bool, bool>);
impl XPD_ROM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_ROM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_ROM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn main_state_in_idle(&self) -> MAIN_STATE_IN_IDLE_R {
        MAIN_STATE_IN_IDLE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn main_state_in_slp(&self) -> MAIN_STATE_IN_SLP_R {
        MAIN_STATE_IN_SLP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn main_state_in_wait_xtl(&self) -> MAIN_STATE_IN_WAIT_XTL_R {
        MAIN_STATE_IN_WAIT_XTL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn main_state_in_wait_pll(&self) -> MAIN_STATE_IN_WAIT_PLL_R {
        MAIN_STATE_IN_WAIT_PLL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn main_state_in_wait_8m(&self) -> MAIN_STATE_IN_WAIT_8M_R {
        MAIN_STATE_IN_WAIT_8M_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn in_low_power_state(&self) -> IN_LOW_POWER_STATE_R {
        IN_LOW_POWER_STATE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn in_wakeup_state(&self) -> IN_WAKEUP_STATE_R {
        IN_WAKEUP_STATE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn main_state_wait_end(&self) -> MAIN_STATE_WAIT_END_R {
        MAIN_STATE_WAIT_END_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&self) -> RDY_FOR_WAKEUP_R {
        RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn main_state_pll_on(&self) -> MAIN_STATE_PLL_ON_R {
        MAIN_STATE_PLL_ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn main_state_xtal_iso(&self) -> MAIN_STATE_XTAL_ISO_R {
        MAIN_STATE_XTAL_ISO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cocpu_state_done(&self) -> COCPU_STATE_DONE_R {
        COCPU_STATE_DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cocpu_state_slp(&self) -> COCPU_STATE_SLP_R {
        COCPU_STATE_SLP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cocpu_state_switch(&self) -> COCPU_STATE_SWITCH_R {
        COCPU_STATE_SWITCH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cocpu_state_start(&self) -> COCPU_STATE_START_R {
        COCPU_STATE_START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_state_done(&self) -> TOUCH_STATE_DONE_R {
        TOUCH_STATE_DONE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_state_slp(&self) -> TOUCH_STATE_SLP_R {
        TOUCH_STATE_SLP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_state_switch(&self) -> TOUCH_STATE_SWITCH_R {
        TOUCH_STATE_SWITCH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_state_start(&self) -> TOUCH_STATE_START_R {
        TOUCH_STATE_START_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn xpd_dig(&self) -> XPD_DIG_R {
        XPD_DIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dig_iso(&self) -> DIG_ISO_R {
        DIG_ISO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn xpd_wifi(&self) -> XPD_WIFI_R {
        XPD_WIFI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifi_iso(&self) -> WIFI_ISO_R {
        WIFI_ISO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn xpd_rtc_peri(&self) -> XPD_RTC_PERI_R {
        XPD_RTC_PERI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn peri_iso(&self) -> PERI_ISO_R {
        PERI_ISO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xpd_dig_dcdc(&self) -> XPD_DIG_DCDC_R {
        XPD_DIG_DCDC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xpd_rom0(&self) -> XPD_ROM0_R {
        XPD_ROM0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RTC_CNTL_LOW_POWER_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_power_st](index.html) module"]
pub struct LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [low_power_st::R](R) reader structure"]
impl crate::Readable for LOW_POWER_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOW_POWER_ST to value 0"]
impl crate::Resettable for LOW_POWER_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
