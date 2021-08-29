#[doc = "Register `BIAS_CONF` reader"]
pub struct R(crate::R<BIAS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIAS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIAS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIAS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIAS_CONF` writer"]
pub struct W(crate::W<BIAS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIAS_CONF_SPEC>;
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
impl From<crate::W<BIAS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIAS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_ATTEN_MONITOR` reader - "]
pub struct DBG_ATTEN_MONITOR_R(crate::FieldReader<u8, u8>);
impl DBG_ATTEN_MONITOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBG_ATTEN_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_ATTEN_MONITOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_ATTEN_MONITOR` writer - "]
pub struct DBG_ATTEN_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_MONITOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `DBG_ATTEN_DEEP_SLP` reader - "]
pub struct DBG_ATTEN_DEEP_SLP_R(crate::FieldReader<u8, u8>);
impl DBG_ATTEN_DEEP_SLP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBG_ATTEN_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_ATTEN_DEEP_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_ATTEN_DEEP_SLP` writer - "]
pub struct DBG_ATTEN_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_ATTEN_DEEP_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `BIAS_SLEEP_MONITOR` reader - "]
pub struct BIAS_SLEEP_MONITOR_R(crate::FieldReader<bool, bool>);
impl BIAS_SLEEP_MONITOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_SLEEP_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_SLEEP_MONITOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_SLEEP_MONITOR` writer - "]
pub struct BIAS_SLEEP_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_MONITOR_W<'a> {
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
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` reader - "]
pub struct BIAS_SLEEP_DEEP_SLP_R(crate::FieldReader<bool, bool>);
impl BIAS_SLEEP_DEEP_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_SLEEP_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_SLEEP_DEEP_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` writer - "]
pub struct BIAS_SLEEP_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_SLEEP_DEEP_SLP_W<'a> {
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
#[doc = "Field `PD_CUR_MONITOR` reader - "]
pub struct PD_CUR_MONITOR_R(crate::FieldReader<bool, bool>);
impl PD_CUR_MONITOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_CUR_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_CUR_MONITOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_CUR_MONITOR` writer - "]
pub struct PD_CUR_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CUR_MONITOR_W<'a> {
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
#[doc = "Field `PD_CUR_DEEP_SLP` reader - "]
pub struct PD_CUR_DEEP_SLP_R(crate::FieldReader<bool, bool>);
impl PD_CUR_DEEP_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_CUR_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_CUR_DEEP_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_CUR_DEEP_SLP` writer - "]
pub struct PD_CUR_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_CUR_DEEP_SLP_W<'a> {
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
#[doc = "Field `BIAS_BUF_MONITOR` reader - "]
pub struct BIAS_BUF_MONITOR_R(crate::FieldReader<bool, bool>);
impl BIAS_BUF_MONITOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_MONITOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_MONITOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_MONITOR` writer - "]
pub struct BIAS_BUF_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_MONITOR_W<'a> {
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
#[doc = "Field `BIAS_BUF_DEEP_SLP` reader - "]
pub struct BIAS_BUF_DEEP_SLP_R(crate::FieldReader<bool, bool>);
impl BIAS_BUF_DEEP_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_DEEP_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_DEEP_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_DEEP_SLP` writer - "]
pub struct BIAS_BUF_DEEP_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_DEEP_SLP_W<'a> {
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
#[doc = "Field `BIAS_BUF_WAKE` reader - "]
pub struct BIAS_BUF_WAKE_R(crate::FieldReader<bool, bool>);
impl BIAS_BUF_WAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_WAKE` writer - "]
pub struct BIAS_BUF_WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_WAKE_W<'a> {
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
#[doc = "Field `BIAS_BUF_IDLE` reader - "]
pub struct BIAS_BUF_IDLE_R(crate::FieldReader<bool, bool>);
impl BIAS_BUF_IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIAS_BUF_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_BUF_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS_BUF_IDLE` writer - "]
pub struct BIAS_BUF_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_BUF_IDLE_W<'a> {
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
#[doc = "Field `DG_VDD_DRV_B_SLP_EN` reader - "]
pub struct DG_VDD_DRV_B_SLP_EN_R(crate::FieldReader<bool, bool>);
impl DG_VDD_DRV_B_SLP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_VDD_DRV_B_SLP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_VDD_DRV_B_SLP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_VDD_DRV_B_SLP_EN` writer - "]
pub struct DG_VDD_DRV_B_SLP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_VDD_DRV_B_SLP_EN_W<'a> {
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
#[doc = "Field `DG_VDD_DRV_B_SLP` reader - "]
pub struct DG_VDD_DRV_B_SLP_R(crate::FieldReader<u8, u8>);
impl DG_VDD_DRV_B_SLP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DG_VDD_DRV_B_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_VDD_DRV_B_SLP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_VDD_DRV_B_SLP` writer - "]
pub struct DG_VDD_DRV_B_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_VDD_DRV_B_SLP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&self) -> DBG_ATTEN_DEEP_SLP_R {
        DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&self) -> BIAS_SLEEP_MONITOR_R {
        BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&self) -> BIAS_SLEEP_DEEP_SLP_R {
        BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_cur_monitor(&self) -> PD_CUR_MONITOR_R {
        PD_CUR_MONITOR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&self) -> PD_CUR_DEEP_SLP_R {
        PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bias_buf_monitor(&self) -> BIAS_BUF_MONITOR_R {
        BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp_en(&self) -> DG_VDD_DRV_B_SLP_EN_R {
        DG_VDD_DRV_B_SLP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&self) -> DG_VDD_DRV_B_SLP_R {
        DG_VDD_DRV_B_SLP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W {
        DBG_ATTEN_MONITOR_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W {
        DBG_ATTEN_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W {
        BIAS_SLEEP_MONITOR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W {
        BIAS_SLEEP_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W {
        PD_CUR_MONITOR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W {
        PD_CUR_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W {
        BIAS_BUF_MONITOR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W {
        BIAS_BUF_DEEP_SLP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W {
        BIAS_BUF_WAKE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W {
        BIAS_BUF_IDLE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp_en(&mut self) -> DG_VDD_DRV_B_SLP_EN_W {
        DG_VDD_DRV_B_SLP_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W {
        DG_VDD_DRV_B_SLP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_BIAS_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bias_conf](index.html) module"]
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bias_conf::R](R) reader structure"]
impl crate::Readable for BIAS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bias_conf::W](W) writer structure"]
impl crate::Writable for BIAS_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIAS_CONF to value 0"]
impl crate::Resettable for BIAS_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
