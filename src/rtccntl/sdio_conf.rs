#[doc = "Register `SDIO_CONF` reader"]
pub struct R(crate::R<SDIO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CONF` writer"]
pub struct W(crate::W<SDIO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CONF_SPEC>;
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
impl From<crate::W<SDIO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_SDIO_REG` reader - "]
pub struct XPD_SDIO_REG_R(crate::FieldReader<bool, bool>);
impl XPD_SDIO_REG_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_SDIO_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SDIO_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SDIO_REG` writer - "]
pub struct XPD_SDIO_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SDIO_REG_W<'a> {
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
#[doc = "Field `DREFH_SDIO` reader - "]
pub struct DREFH_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFH_SDIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        DREFH_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFH_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFH_SDIO` writer - "]
pub struct DREFH_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFH_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `DREFM_SDIO` reader - "]
pub struct DREFM_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFM_SDIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        DREFM_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFM_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFM_SDIO` writer - "]
pub struct DREFM_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFM_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `DREFL_SDIO` reader - "]
pub struct DREFL_SDIO_R(crate::FieldReader<u8, u8>);
impl DREFL_SDIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        DREFL_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DREFL_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DREFL_SDIO` writer - "]
pub struct DREFL_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DREFL_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `REG1P8_READY` reader - "]
pub struct REG1P8_READY_R(crate::FieldReader<bool, bool>);
impl REG1P8_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG1P8_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG1P8_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIEH` reader - "]
pub struct SDIO_TIEH_R(crate::FieldReader<bool, bool>);
impl SDIO_TIEH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_TIEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_TIEH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIEH` writer - "]
pub struct SDIO_TIEH_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIEH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SDIO_FORCE` reader - "]
pub struct SDIO_FORCE_R(crate::FieldReader<bool, bool>);
impl SDIO_FORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_FORCE` writer - "]
pub struct SDIO_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_FORCE_W<'a> {
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
#[doc = "Field `SDIO_PD_EN` reader - "]
pub struct SDIO_PD_EN_R(crate::FieldReader<bool, bool>);
impl SDIO_PD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_PD_EN` writer - "]
pub struct SDIO_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_PD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SDIO_ENCURLIM` reader - "]
pub struct SDIO_ENCURLIM_R(crate::FieldReader<bool, bool>);
impl SDIO_ENCURLIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_ENCURLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_ENCURLIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_ENCURLIM` writer - "]
pub struct SDIO_ENCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_ENCURLIM_W<'a> {
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
#[doc = "Field `SDIO_MODECURLIM` reader - "]
pub struct SDIO_MODECURLIM_R(crate::FieldReader<bool, bool>);
impl SDIO_MODECURLIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_MODECURLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_MODECURLIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_MODECURLIM` writer - "]
pub struct SDIO_MODECURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_MODECURLIM_W<'a> {
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
#[doc = "Field `SDIO_DCURLIM` reader - "]
pub struct SDIO_DCURLIM_R(crate::FieldReader<u8, u8>);
impl SDIO_DCURLIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DCURLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DCURLIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DCURLIM` writer - "]
pub struct SDIO_DCURLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DCURLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `SDIO_EN_INITI` reader - "]
pub struct SDIO_EN_INITI_R(crate::FieldReader<bool, bool>);
impl SDIO_EN_INITI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIO_EN_INITI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_EN_INITI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_EN_INITI` writer - "]
pub struct SDIO_EN_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_EN_INITI_W<'a> {
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
#[doc = "Field `SDIO_INITI` reader - "]
pub struct SDIO_INITI_R(crate::FieldReader<u8, u8>);
impl SDIO_INITI_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_INITI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_INITI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_INITI` writer - "]
pub struct SDIO_INITI_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_INITI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `SDIO_DCAP` reader - "]
pub struct SDIO_DCAP_R(crate::FieldReader<u8, u8>);
impl SDIO_DCAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DCAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DCAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DCAP` writer - "]
pub struct SDIO_DCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DCAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `SDIO_DTHDRV` reader - "]
pub struct SDIO_DTHDRV_R(crate::FieldReader<u8, u8>);
impl SDIO_DTHDRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_DTHDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_DTHDRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_DTHDRV` writer - "]
pub struct SDIO_DTHDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_DTHDRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `SDIO_TIMER_TARGET` reader - "]
pub struct SDIO_TIMER_TARGET_R(crate::FieldReader<u8, u8>);
impl SDIO_TIMER_TARGET_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_TIMER_TARGET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_TIMER_TARGET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIO_TIMER_TARGET` writer - "]
pub struct SDIO_TIMER_TARGET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_TIMER_TARGET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xpd_sdio_reg(&self) -> XPD_SDIO_REG_R {
        XPD_SDIO_REG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn drefh_sdio(&self) -> DREFH_SDIO_R {
        DREFH_SDIO_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn drefm_sdio(&self) -> DREFM_SDIO_R {
        DREFM_SDIO_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn drefl_sdio(&self) -> DREFL_SDIO_R {
        DREFL_SDIO_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg1p8_ready(&self) -> REG1P8_READY_R {
        REG1P8_READY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sdio_pd_en(&self) -> SDIO_PD_EN_R {
        SDIO_PD_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sdio_encurlim(&self) -> SDIO_ENCURLIM_R {
        SDIO_ENCURLIM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdio_modecurlim(&self) -> SDIO_MODECURLIM_R {
        SDIO_MODECURLIM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn sdio_dcurlim(&self) -> SDIO_DCURLIM_R {
        SDIO_DCURLIM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sdio_en_initi(&self) -> SDIO_EN_INITI_R {
        SDIO_EN_INITI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sdio_initi(&self) -> SDIO_INITI_R {
        SDIO_INITI_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sdio_dcap(&self) -> SDIO_DCAP_R {
        SDIO_DCAP_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn sdio_dthdrv(&self) -> SDIO_DTHDRV_R {
        SDIO_DTHDRV_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sdio_timer_target(&self) -> SDIO_TIMER_TARGET_R {
        SDIO_TIMER_TARGET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xpd_sdio_reg(&mut self) -> XPD_SDIO_REG_W {
        XPD_SDIO_REG_W { w: self }
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn drefh_sdio(&mut self) -> DREFH_SDIO_W {
        DREFH_SDIO_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn drefm_sdio(&mut self) -> DREFM_SDIO_W {
        DREFM_SDIO_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn drefl_sdio(&mut self) -> DREFL_SDIO_W {
        DREFL_SDIO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W {
        SDIO_TIEH_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W {
        SDIO_FORCE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sdio_pd_en(&mut self) -> SDIO_PD_EN_W {
        SDIO_PD_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sdio_encurlim(&mut self) -> SDIO_ENCURLIM_W {
        SDIO_ENCURLIM_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sdio_modecurlim(&mut self) -> SDIO_MODECURLIM_W {
        SDIO_MODECURLIM_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn sdio_dcurlim(&mut self) -> SDIO_DCURLIM_W {
        SDIO_DCURLIM_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sdio_en_initi(&mut self) -> SDIO_EN_INITI_W {
        SDIO_EN_INITI_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sdio_initi(&mut self) -> SDIO_INITI_W {
        SDIO_INITI_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn sdio_dcap(&mut self) -> SDIO_DCAP_W {
        SDIO_DCAP_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn sdio_dthdrv(&mut self) -> SDIO_DTHDRV_W {
        SDIO_DTHDRV_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sdio_timer_target(&mut self) -> SDIO_TIMER_TARGET_W {
        SDIO_TIMER_TARGET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_SDIO_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_conf](index.html) module"]
pub struct SDIO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_conf::R](R) reader structure"]
impl crate::Readable for SDIO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_conf::W](W) writer structure"]
impl crate::Writable for SDIO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_CONF to value 0"]
impl crate::Resettable for SDIO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
