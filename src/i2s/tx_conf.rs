#[doc = "Register `TX_CONF` reader"]
pub struct R(crate::R<TX_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CONF` writer"]
pub struct W(crate::W<TX_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CONF_SPEC>;
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
impl From<crate::W<TX_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_LOOPBACK` reader - "]
pub struct SIG_LOOPBACK_R(crate::FieldReader<bool, bool>);
impl SIG_LOOPBACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIG_LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_LOOPBACK` writer - "]
pub struct SIG_LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_LOOPBACK_W<'a> {
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
#[doc = "Field `TX_CHAN_MOD` reader - "]
pub struct TX_CHAN_MOD_R(crate::FieldReader<u8, u8>);
impl TX_CHAN_MOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_CHAN_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CHAN_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CHAN_MOD` writer - "]
pub struct TX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `TX_PDM_EN` reader - "]
pub struct TX_PDM_EN_R(crate::FieldReader<bool, bool>);
impl TX_PDM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_PDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_EN` writer - "]
pub struct TX_PDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_EN_W<'a> {
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
#[doc = "Field `TX_TDM_EN` reader - "]
pub struct TX_TDM_EN_R(crate::FieldReader<bool, bool>);
impl TX_TDM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_TDM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TDM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TDM_EN` writer - "]
pub struct TX_TDM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TDM_EN_W<'a> {
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
#[doc = "Field `TX_BIT_ORDER` reader - "]
pub struct TX_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl TX_BIT_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BIT_ORDER` writer - "]
pub struct TX_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BIT_ORDER_W<'a> {
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
#[doc = "Field `TX_WS_IDLE_POL` reader - "]
pub struct TX_WS_IDLE_POL_R(crate::FieldReader<bool, bool>);
impl TX_WS_IDLE_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_WS_IDLE_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WS_IDLE_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WS_IDLE_POL` writer - "]
pub struct TX_WS_IDLE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WS_IDLE_POL_W<'a> {
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
#[doc = "Field `TX_24_FILL_EN` reader - "]
pub struct TX_24_FILL_EN_R(crate::FieldReader<bool, bool>);
impl TX_24_FILL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_24_FILL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_24_FILL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_24_FILL_EN` writer - "]
pub struct TX_24_FILL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_24_FILL_EN_W<'a> {
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
#[doc = "Field `TX_LEFT_ALIGN` reader - "]
pub struct TX_LEFT_ALIGN_R(crate::FieldReader<bool, bool>);
impl TX_LEFT_ALIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_LEFT_ALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LEFT_ALIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_LEFT_ALIGN` writer - "]
pub struct TX_LEFT_ALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LEFT_ALIGN_W<'a> {
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
#[doc = "Field `TX_STOP_EN` reader - "]
pub struct TX_STOP_EN_R(crate::FieldReader<bool, bool>);
impl TX_STOP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_STOP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_STOP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_STOP_EN` writer - "]
pub struct TX_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STOP_EN_W<'a> {
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
#[doc = "Field `TX_PCM_BYPASS` reader - "]
pub struct TX_PCM_BYPASS_R(crate::FieldReader<bool, bool>);
impl TX_PCM_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_PCM_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PCM_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PCM_BYPASS` writer - "]
pub struct TX_PCM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PCM_BYPASS_W<'a> {
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
#[doc = "Field `TX_PCM_CONF` reader - "]
pub struct TX_PCM_CONF_R(crate::FieldReader<u8, u8>);
impl TX_PCM_CONF_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_PCM_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PCM_CONF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PCM_CONF` writer - "]
pub struct TX_PCM_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PCM_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `TX_MONO_FST_VLD` reader - "]
pub struct TX_MONO_FST_VLD_R(crate::FieldReader<bool, bool>);
impl TX_MONO_FST_VLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_MONO_FST_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MONO_FST_VLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MONO_FST_VLD` writer - "]
pub struct TX_MONO_FST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MONO_FST_VLD_W<'a> {
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
#[doc = "Field `TX_UPDATE` reader - "]
pub struct TX_UPDATE_R(crate::FieldReader<bool, bool>);
impl TX_UPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_UPDATE` writer - "]
pub struct TX_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UPDATE_W<'a> {
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
#[doc = "Field `TX_BIG_ENDIAN` reader - "]
pub struct TX_BIG_ENDIAN_R(crate::FieldReader<bool, bool>);
impl TX_BIG_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BIG_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BIG_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BIG_ENDIAN` writer - "]
pub struct TX_BIG_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BIG_ENDIAN_W<'a> {
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
#[doc = "Field `TX_CHAN_EQUAL` reader - "]
pub struct TX_CHAN_EQUAL_R(crate::FieldReader<bool, bool>);
impl TX_CHAN_EQUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CHAN_EQUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CHAN_EQUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CHAN_EQUAL` writer - "]
pub struct TX_CHAN_EQUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHAN_EQUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TX_MONO` reader - "]
pub struct TX_MONO_R(crate::FieldReader<bool, bool>);
impl TX_MONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_MONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_MONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_MONO` writer - "]
pub struct TX_MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MONO_W<'a> {
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
#[doc = "Field `TX_SLAVE_MOD` reader - "]
pub struct TX_SLAVE_MOD_R(crate::FieldReader<bool, bool>);
impl TX_SLAVE_MOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_SLAVE_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SLAVE_MOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SLAVE_MOD` writer - "]
pub struct TX_SLAVE_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SLAVE_MOD_W<'a> {
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
#[doc = "Field `TX_START` reader - "]
pub struct TX_START_R(crate::FieldReader<bool, bool>);
impl TX_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START` writer - "]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
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
#[doc = "Field `TX_FIFO_RESET` writer - "]
pub struct TX_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_RESET_W<'a> {
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
#[doc = "Field `TX_RESET` writer - "]
pub struct TX_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RESET_W<'a> {
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
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sig_loopback(&self) -> SIG_LOOPBACK_R {
        SIG_LOOPBACK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_pdm_en(&self) -> TX_PDM_EN_R {
        TX_PDM_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_tdm_en(&self) -> TX_TDM_EN_R {
        TX_TDM_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_bit_order(&self) -> TX_BIT_ORDER_R {
        TX_BIT_ORDER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tx_ws_idle_pol(&self) -> TX_WS_IDLE_POL_R {
        TX_WS_IDLE_POL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_24_fill_en(&self) -> TX_24_FILL_EN_R {
        TX_24_FILL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_left_align(&self) -> TX_LEFT_ALIGN_R {
        TX_LEFT_ALIGN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_stop_en(&self) -> TX_STOP_EN_R {
        TX_STOP_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_pcm_bypass(&self) -> TX_PCM_BYPASS_R {
        TX_PCM_BYPASS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_pcm_conf(&self) -> TX_PCM_CONF_R {
        TX_PCM_CONF_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_mono_fst_vld(&self) -> TX_MONO_FST_VLD_R {
        TX_MONO_FST_VLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_update(&self) -> TX_UPDATE_R {
        TX_UPDATE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_big_endian(&self) -> TX_BIG_ENDIAN_R {
        TX_BIG_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_chan_equal(&self) -> TX_CHAN_EQUAL_R {
        TX_CHAN_EQUAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_mono(&self) -> TX_MONO_R {
        TX_MONO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_slave_mod(&self) -> TX_SLAVE_MOD_R {
        TX_SLAVE_MOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sig_loopback(&mut self) -> SIG_LOOPBACK_W {
        SIG_LOOPBACK_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W {
        TX_CHAN_MOD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tx_pdm_en(&mut self) -> TX_PDM_EN_W {
        TX_PDM_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_tdm_en(&mut self) -> TX_TDM_EN_W {
        TX_TDM_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tx_bit_order(&mut self) -> TX_BIT_ORDER_W {
        TX_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tx_ws_idle_pol(&mut self) -> TX_WS_IDLE_POL_W {
        TX_WS_IDLE_POL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tx_24_fill_en(&mut self) -> TX_24_FILL_EN_W {
        TX_24_FILL_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_left_align(&mut self) -> TX_LEFT_ALIGN_W {
        TX_LEFT_ALIGN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_stop_en(&mut self) -> TX_STOP_EN_W {
        TX_STOP_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_pcm_bypass(&mut self) -> TX_PCM_BYPASS_W {
        TX_PCM_BYPASS_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tx_pcm_conf(&mut self) -> TX_PCM_CONF_W {
        TX_PCM_CONF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tx_mono_fst_vld(&mut self) -> TX_MONO_FST_VLD_W {
        TX_MONO_FST_VLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_update(&mut self) -> TX_UPDATE_W {
        TX_UPDATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_big_endian(&mut self) -> TX_BIG_ENDIAN_W {
        TX_BIG_ENDIAN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_chan_equal(&mut self) -> TX_CHAN_EQUAL_W {
        TX_CHAN_EQUAL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_mono(&mut self) -> TX_MONO_W {
        TX_MONO_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_slave_mod(&mut self) -> TX_SLAVE_MOD_W {
        TX_SLAVE_MOD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_reset(&mut self) -> TX_FIFO_RESET_W {
        TX_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_reset(&mut self) -> TX_RESET_W {
        TX_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_TX_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_conf](index.html) module"]
pub struct TX_CONF_SPEC;
impl crate::RegisterSpec for TX_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_conf::R](R) reader structure"]
impl crate::Readable for TX_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_conf::W](W) writer structure"]
impl crate::Writable for TX_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_CONF to value 0"]
impl crate::Resettable for TX_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
