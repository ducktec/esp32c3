#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT` reader - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R(crate::FieldReader<u8, u8>);
impl CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT` writer - "]
pub struct CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_systimer(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bb(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_ledc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rmt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uhci0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_systimer(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SYSTIMER_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP1_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_timergroup(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_TIMERGROUP_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bb(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BB_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_ledc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_LEDC_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rmt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RMT_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uhci0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UHCI0_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT0_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_bt(&mut self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_BT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CORE_0_PIF_PMS_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_2](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_2 to value 0"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
