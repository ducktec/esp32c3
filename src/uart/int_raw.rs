#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAKEUP_INT_RAW` reader - "]
pub struct WAKEUP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl WAKEUP_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP_INT_RAW` writer - "]
pub struct WAKEUP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_INT_RAW_W<'a> {
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
#[doc = "Field `AT_CMD_CHAR_DET_INT_RAW` reader - "]
pub struct AT_CMD_CHAR_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl AT_CMD_CHAR_DET_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        AT_CMD_CHAR_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AT_CMD_CHAR_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AT_CMD_CHAR_DET_INT_RAW` writer - "]
pub struct AT_CMD_CHAR_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_CMD_CHAR_DET_INT_RAW_W<'a> {
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
#[doc = "Field `RS485_CLASH_INT_RAW` reader - "]
pub struct RS485_CLASH_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RS485_CLASH_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485_CLASH_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_CLASH_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_CLASH_INT_RAW` writer - "]
pub struct RS485_CLASH_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_CLASH_INT_RAW_W<'a> {
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
#[doc = "Field `RS485_FRM_ERR_INT_RAW` reader - "]
pub struct RS485_FRM_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RS485_FRM_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485_FRM_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_FRM_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_FRM_ERR_INT_RAW` writer - "]
pub struct RS485_FRM_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_FRM_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `RS485_PARITY_ERR_INT_RAW` reader - "]
pub struct RS485_PARITY_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RS485_PARITY_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485_PARITY_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_PARITY_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_PARITY_ERR_INT_RAW` writer - "]
pub struct RS485_PARITY_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RS485_PARITY_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `TX_DONE_INT_RAW` reader - "]
pub struct TX_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_DONE_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DONE_INT_RAW` writer - "]
pub struct TX_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_INT_RAW_W<'a> {
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
#[doc = "Field `TX_BRK_IDLE_DONE_INT_RAW` reader - "]
pub struct TX_BRK_IDLE_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_BRK_IDLE_DONE_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_IDLE_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_IDLE_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_IDLE_DONE_INT_RAW` writer - "]
pub struct TX_BRK_IDLE_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_IDLE_DONE_INT_RAW_W<'a> {
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
#[doc = "Field `TX_BRK_DONE_INT_RAW` reader - "]
pub struct TX_BRK_DONE_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TX_BRK_DONE_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_DONE_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_DONE_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_DONE_INT_RAW` writer - "]
pub struct TX_BRK_DONE_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BRK_DONE_INT_RAW_W<'a> {
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
#[doc = "Field `GLITCH_DET_INT_RAW` reader - "]
pub struct GLITCH_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl GLITCH_DET_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_RAW` writer - "]
pub struct GLITCH_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> GLITCH_DET_INT_RAW_W<'a> {
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
#[doc = "Field `SW_XOFF_INT_RAW` reader - "]
pub struct SW_XOFF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SW_XOFF_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_XOFF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XOFF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XOFF_INT_RAW` writer - "]
pub struct SW_XOFF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XOFF_INT_RAW_W<'a> {
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
#[doc = "Field `SW_XON_INT_RAW` reader - "]
pub struct SW_XON_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SW_XON_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_XON_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XON_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XON_INT_RAW` writer - "]
pub struct SW_XON_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_XON_INT_RAW_W<'a> {
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
#[doc = "Field `RXFIFO_TOUT_INT_RAW` reader - "]
pub struct RXFIFO_TOUT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_TOUT_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_TOUT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TOUT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TOUT_INT_RAW` writer - "]
pub struct RXFIFO_TOUT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_TOUT_INT_RAW_W<'a> {
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
#[doc = "Field `BRK_DET_INT_RAW` reader - "]
pub struct BRK_DET_INT_RAW_R(crate::FieldReader<bool, bool>);
impl BRK_DET_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK_DET_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_DET_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK_DET_INT_RAW` writer - "]
pub struct BRK_DET_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_DET_INT_RAW_W<'a> {
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
#[doc = "Field `CTS_CHG_INT_RAW` reader - "]
pub struct CTS_CHG_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CTS_CHG_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTS_CHG_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_CHG_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_CHG_INT_RAW` writer - "]
pub struct CTS_CHG_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_CHG_INT_RAW_W<'a> {
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
#[doc = "Field `DSR_CHG_INT_RAW` reader - "]
pub struct DSR_CHG_INT_RAW_R(crate::FieldReader<bool, bool>);
impl DSR_CHG_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR_CHG_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_CHG_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR_CHG_INT_RAW` writer - "]
pub struct DSR_CHG_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_CHG_INT_RAW_W<'a> {
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
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - "]
pub struct RXFIFO_OVF_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_OVF_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_RAW` writer - "]
pub struct RXFIFO_OVF_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_OVF_INT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FRM_ERR_INT_RAW` reader - "]
pub struct FRM_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl FRM_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRM_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRM_ERR_INT_RAW` writer - "]
pub struct FRM_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRM_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `PARITY_ERR_INT_RAW` reader - "]
pub struct PARITY_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl PARITY_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ERR_INT_RAW` writer - "]
pub struct PARITY_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` reader - "]
pub struct TXFIFO_EMPTY_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TXFIFO_EMPTY_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_EMPTY_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` writer - "]
pub struct TXFIFO_EMPTY_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_INT_RAW_W<'a> {
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
#[doc = "Field `RXFIFO_FULL_INT_RAW` reader - "]
pub struct RXFIFO_FULL_INT_RAW_R(crate::FieldReader<bool, bool>);
impl RXFIFO_FULL_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_FULL_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_FULL_INT_RAW` writer - "]
pub struct RXFIFO_FULL_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_INT_RAW_W<'a> {
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
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wakeup_int_raw(&self) -> WAKEUP_INT_RAW_R {
        WAKEUP_INT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&self) -> AT_CMD_CHAR_DET_INT_RAW_R {
        AT_CMD_CHAR_DET_INT_RAW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rs485_clash_int_raw(&self) -> RS485_CLASH_INT_RAW_R {
        RS485_CLASH_INT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rs485_frm_err_int_raw(&self) -> RS485_FRM_ERR_INT_RAW_R {
        RS485_FRM_ERR_INT_RAW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rs485_parity_err_int_raw(&self) -> RS485_PARITY_ERR_INT_RAW_R {
        RS485_PARITY_ERR_INT_RAW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> TX_DONE_INT_RAW_R {
        TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&self) -> TX_BRK_IDLE_DONE_INT_RAW_R {
        TX_BRK_IDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&self) -> TX_BRK_DONE_INT_RAW_R {
        TX_BRK_DONE_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&self) -> SW_XOFF_INT_RAW_R {
        SW_XOFF_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_xon_int_raw(&self) -> SW_XON_INT_RAW_R {
        SW_XON_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> RXFIFO_TOUT_INT_RAW_R {
        RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> BRK_DET_INT_RAW_R {
        BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> CTS_CHG_INT_RAW_R {
        CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> DSR_CHG_INT_RAW_R {
        DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> FRM_ERR_INT_RAW_R {
        FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> PARITY_ERR_INT_RAW_R {
        PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wakeup_int_raw(&mut self) -> WAKEUP_INT_RAW_W {
        WAKEUP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&mut self) -> AT_CMD_CHAR_DET_INT_RAW_W {
        AT_CMD_CHAR_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rs485_clash_int_raw(&mut self) -> RS485_CLASH_INT_RAW_W {
        RS485_CLASH_INT_RAW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rs485_frm_err_int_raw(&mut self) -> RS485_FRM_ERR_INT_RAW_W {
        RS485_FRM_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rs485_parity_err_int_raw(&mut self) -> RS485_PARITY_ERR_INT_RAW_W {
        RS485_PARITY_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_done_int_raw(&mut self) -> TX_DONE_INT_RAW_W {
        TX_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&mut self) -> TX_BRK_IDLE_DONE_INT_RAW_W {
        TX_BRK_IDLE_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&mut self) -> TX_BRK_DONE_INT_RAW_W {
        TX_BRK_DONE_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn glitch_det_int_raw(&mut self) -> GLITCH_DET_INT_RAW_W {
        GLITCH_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&mut self) -> SW_XOFF_INT_RAW_W {
        SW_XOFF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_xon_int_raw(&mut self) -> SW_XON_INT_RAW_W {
        SW_XON_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&mut self) -> RXFIFO_TOUT_INT_RAW_W {
        RXFIFO_TOUT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn brk_det_int_raw(&mut self) -> BRK_DET_INT_RAW_W {
        BRK_DET_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cts_chg_int_raw(&mut self) -> CTS_CHG_INT_RAW_W {
        CTS_CHG_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&mut self) -> DSR_CHG_INT_RAW_W {
        DSR_CHG_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&mut self) -> RXFIFO_OVF_INT_RAW_W {
        RXFIFO_OVF_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frm_err_int_raw(&mut self) -> FRM_ERR_INT_RAW_W {
        FRM_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn parity_err_int_raw(&mut self) -> PARITY_ERR_INT_RAW_W {
        PARITY_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&mut self) -> TXFIFO_EMPTY_INT_RAW_W {
        TXFIFO_EMPTY_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&mut self) -> RXFIFO_FULL_INT_RAW_W {
        RXFIFO_FULL_INT_RAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
