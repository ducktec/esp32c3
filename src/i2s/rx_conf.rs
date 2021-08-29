#[doc = "Register `RX_CONF` reader"]
pub struct R(crate::R<RX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CONF` writer"]
pub struct W(crate::W<RX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CONF_SPEC>;
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
impl From<crate::W<RX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_PDM_EN` reader - "]
pub struct RX_PDM_EN_R(crate::FieldReader<bool, bool>);
impl RX_PDM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_PDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PDM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PDM_EN` writer - "]
pub struct RX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PDM_EN_W<'a> {
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
#[doc = "Field `RX_TDM_EN` reader - "]
pub struct RX_TDM_EN_R(crate::FieldReader<bool, bool>);
impl RX_TDM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_TDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TDM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TDM_EN` writer - "]
pub struct RX_TDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TDM_EN_W<'a> {
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
#[doc = "Field `RX_BIT_ORDER` reader - "]
pub struct RX_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl RX_BIT_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BIT_ORDER` writer - "]
pub struct RX_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BIT_ORDER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RX_WS_IDLE_POL` reader - "]
pub struct RX_WS_IDLE_POL_R(crate::FieldReader<bool, bool>);
impl RX_WS_IDLE_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_WS_IDLE_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WS_IDLE_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_WS_IDLE_POL` writer - "]
pub struct RX_WS_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_WS_IDLE_POL_W<'a> {
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
#[doc = "Field `RX_24_FILL_EN` reader - "]
pub struct RX_24_FILL_EN_R(crate::FieldReader<bool, bool>);
impl RX_24_FILL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_24_FILL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_24_FILL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_24_FILL_EN` writer - "]
pub struct RX_24_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_24_FILL_EN_W<'a> {
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
#[doc = "Field `RX_LEFT_ALIGN` reader - "]
pub struct RX_LEFT_ALIGN_R(crate::FieldReader<bool, bool>);
impl RX_LEFT_ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_LEFT_ALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LEFT_ALIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_LEFT_ALIGN` writer - "]
pub struct RX_LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LEFT_ALIGN_W<'a> {
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
#[doc = "Field `RX_STOP_MODE` reader - "]
pub struct RX_STOP_MODE_R(crate::FieldReader<u8, u8>);
impl RX_STOP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_STOP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_STOP_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_STOP_MODE` writer - "]
pub struct RX_STOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_STOP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `RX_PCM_BYPASS` reader - "]
pub struct RX_PCM_BYPASS_R(crate::FieldReader<bool, bool>);
impl RX_PCM_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_PCM_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PCM_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PCM_BYPASS` writer - "]
pub struct RX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PCM_BYPASS_W<'a> {
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
#[doc = "Field `RX_PCM_CONF` reader - "]
pub struct RX_PCM_CONF_R(crate::FieldReader<u8, u8>);
impl RX_PCM_CONF_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_PCM_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PCM_CONF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PCM_CONF` writer - "]
pub struct RX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `RX_MONO_FST_VLD` reader - "]
pub struct RX_MONO_FST_VLD_R(crate::FieldReader<bool, bool>);
impl RX_MONO_FST_VLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_MONO_FST_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MONO_FST_VLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MONO_FST_VLD` writer - "]
pub struct RX_MONO_FST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MONO_FST_VLD_W<'a> {
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
#[doc = "Field `RX_UPDATE` reader - "]
pub struct RX_UPDATE_R(crate::FieldReader<bool, bool>);
impl RX_UPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_UPDATE` writer - "]
pub struct RX_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UPDATE_W<'a> {
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
#[doc = "Field `RX_BIG_ENDIAN` reader - "]
pub struct RX_BIG_ENDIAN_R(crate::FieldReader<bool, bool>);
impl RX_BIG_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_BIG_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BIG_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_BIG_ENDIAN` writer - "]
pub struct RX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BIG_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RX_MONO` reader - "]
pub struct RX_MONO_R(crate::FieldReader<bool, bool>);
impl RX_MONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_MONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_MONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_MONO` writer - "]
pub struct RX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MONO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RX_SLAVE_MOD` reader - "]
pub struct RX_SLAVE_MOD_R(crate::FieldReader<bool, bool>);
impl RX_SLAVE_MOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_SLAVE_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SLAVE_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SLAVE_MOD` writer - "]
pub struct RX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SLAVE_MOD_W<'a> {
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
#[doc = "Field `RX_START` reader - "]
pub struct RX_START_R(crate::FieldReader<bool, bool>);
impl RX_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_START` writer - "]
pub struct RX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RX_FIFO_RESET` writer - "]
pub struct RX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_RESET_W<'a> {
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
#[doc = "Field `RX_RESET` writer - "]
pub struct RX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_RESET_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pdm_en(&self) -> RX_PDM_EN_R {
        RX_PDM_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_tdm_en(&self) -> RX_TDM_EN_R {
        RX_TDM_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rx_bit_order(&self) -> RX_BIT_ORDER_R {
        RX_BIT_ORDER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&self) -> RX_WS_IDLE_POL_R {
        RX_WS_IDLE_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_24_fill_en(&self) -> RX_24_FILL_EN_R {
        RX_24_FILL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_left_align(&self) -> RX_LEFT_ALIGN_R {
        RX_LEFT_ALIGN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rx_stop_mode(&self) -> RX_STOP_MODE_R {
        RX_STOP_MODE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_pcm_bypass(&self) -> RX_PCM_BYPASS_R {
        RX_PCM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rx_pcm_conf(&self) -> RX_PCM_CONF_R {
        RX_PCM_CONF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&self) -> RX_MONO_FST_VLD_R {
        RX_MONO_FST_VLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_update(&self) -> RX_UPDATE_R {
        RX_UPDATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_big_endian(&self) -> RX_BIG_ENDIAN_R {
        RX_BIG_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_mono(&self) -> RX_MONO_R {
        RX_MONO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_slave_mod(&self) -> RX_SLAVE_MOD_R {
        RX_SLAVE_MOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_pdm_en(&mut self) -> RX_PDM_EN_W {
        RX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rx_tdm_en(&mut self) -> RX_TDM_EN_W {
        RX_TDM_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rx_bit_order(&mut self) -> RX_BIT_ORDER_W {
        RX_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_ws_idle_pol(&mut self) -> RX_WS_IDLE_POL_W {
        RX_WS_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_24_fill_en(&mut self) -> RX_24_FILL_EN_W {
        RX_24_FILL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rx_left_align(&mut self) -> RX_LEFT_ALIGN_W {
        RX_LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rx_stop_mode(&mut self) -> RX_STOP_MODE_W {
        RX_STOP_MODE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rx_pcm_bypass(&mut self) -> RX_PCM_BYPASS_W {
        RX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rx_pcm_conf(&mut self) -> RX_PCM_CONF_W {
        RX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_mono_fst_vld(&mut self) -> RX_MONO_FST_VLD_W {
        RX_MONO_FST_VLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rx_update(&mut self) -> RX_UPDATE_W {
        RX_UPDATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_big_endian(&mut self) -> RX_BIG_ENDIAN_W {
        RX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_mono(&mut self) -> RX_MONO_W {
        RX_MONO_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_slave_mod(&mut self) -> RX_SLAVE_MOD_W {
        RX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W {
        RX_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_fifo_reset(&mut self) -> RX_FIFO_RESET_W {
        RX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_reset(&mut self) -> RX_RESET_W {
        RX_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_RX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_conf](index.html) module"]
pub struct RX_CONF_SPEC;
impl crate::RegisterSpec for RX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_conf::R](R) reader structure"]
impl crate::Readable for RX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_conf::W](W) writer structure"]
impl crate::Writable for RX_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RX_CONF to value 0"]
impl crate::Resettable for RX_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
