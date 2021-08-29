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
#[doc = "Field `CH1_TX_LOOP_INT_RAW` reader - "]
pub struct CH1_TX_LOOP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_LOOP_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_LOOP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_LOOP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_LOOP_INT_RAW` writer - "]
pub struct CH1_TX_LOOP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_LOOP_INT_RAW_W<'a> {
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
#[doc = "Field `CH0_TX_LOOP_INT_RAW` reader - "]
pub struct CH0_TX_LOOP_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_LOOP_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_LOOP_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_LOOP_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_LOOP_INT_RAW` writer - "]
pub struct CH0_TX_LOOP_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_LOOP_INT_RAW_W<'a> {
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
#[doc = "Field `CH3_RX_THR_EVENT_INT_RAW` reader - "]
pub struct CH3_RX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_RX_THR_EVENT_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_RX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_RX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_RX_THR_EVENT_INT_RAW` writer - "]
pub struct CH3_RX_THR_EVENT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RX_THR_EVENT_INT_RAW_W<'a> {
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
#[doc = "Field `CH2_RX_THR_EVENT_INT_RAW` reader - "]
pub struct CH2_RX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_RX_THR_EVENT_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_RX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_RX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_RX_THR_EVENT_INT_RAW` writer - "]
pub struct CH2_RX_THR_EVENT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RX_THR_EVENT_INT_RAW_W<'a> {
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
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` reader - "]
pub struct CH1_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_THR_EVENT_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` writer - "]
pub struct CH1_TX_THR_EVENT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_THR_EVENT_INT_RAW_W<'a> {
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
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` reader - "]
pub struct CH0_TX_THR_EVENT_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_THR_EVENT_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_THR_EVENT_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_THR_EVENT_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` writer - "]
pub struct CH0_TX_THR_EVENT_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_THR_EVENT_INT_RAW_W<'a> {
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
#[doc = "Field `CH3_ERR_INT_RAW` reader - "]
pub struct CH3_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_ERR_INT_RAW` writer - "]
pub struct CH3_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `CH2_ERR_INT_RAW` reader - "]
pub struct CH2_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_ERR_INT_RAW` writer - "]
pub struct CH2_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `CH1_ERR_INT_RAW` reader - "]
pub struct CH1_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_ERR_INT_RAW` writer - "]
pub struct CH1_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `CH0_ERR_INT_RAW` reader - "]
pub struct CH0_ERR_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_ERR_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_ERR_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_ERR_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_ERR_INT_RAW` writer - "]
pub struct CH0_ERR_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_ERR_INT_RAW_W<'a> {
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
#[doc = "Field `CH3_RX_END_INT_RAW` reader - "]
pub struct CH3_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH3_RX_END_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3_RX_END_INT_RAW` writer - "]
pub struct CH3_RX_END_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_RX_END_INT_RAW_W<'a> {
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
#[doc = "Field `CH2_RX_END_INT_RAW` reader - "]
pub struct CH2_RX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH2_RX_END_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_RX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_RX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2_RX_END_INT_RAW` writer - "]
pub struct CH2_RX_END_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_RX_END_INT_RAW_W<'a> {
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
#[doc = "Field `CH1_TX_END_INT_RAW` reader - "]
pub struct CH1_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH1_TX_END_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_TX_END_INT_RAW` writer - "]
pub struct CH1_TX_END_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_TX_END_INT_RAW_W<'a> {
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
#[doc = "Field `CH0_TX_END_INT_RAW` reader - "]
pub struct CH0_TX_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl CH0_TX_END_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_TX_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_TX_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_TX_END_INT_RAW` writer - "]
pub struct CH0_TX_END_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_TX_END_INT_RAW_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH1_TX_LOOP_INT_RAW_R {
        CH1_TX_LOOP_INT_RAW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH0_TX_LOOP_INT_RAW_R {
        CH0_TX_LOOP_INT_RAW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_raw(&self) -> CH3_RX_THR_EVENT_INT_RAW_R {
        CH3_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_raw(&self) -> CH2_RX_THR_EVENT_INT_RAW_R {
        CH2_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH1_TX_THR_EVENT_INT_RAW_R {
        CH1_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH0_TX_THR_EVENT_INT_RAW_R {
        CH0_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch3_err_int_raw(&self) -> CH3_ERR_INT_RAW_R {
        CH3_ERR_INT_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch2_err_int_raw(&self) -> CH2_ERR_INT_RAW_R {
        CH2_ERR_INT_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch1_err_int_raw(&self) -> CH1_ERR_INT_RAW_R {
        CH1_ERR_INT_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch0_err_int_raw(&self) -> CH0_ERR_INT_RAW_R {
        CH0_ERR_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3_rx_end_int_raw(&self) -> CH3_RX_END_INT_RAW_R {
        CH3_RX_END_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2_rx_end_int_raw(&self) -> CH2_RX_END_INT_RAW_R {
        CH2_RX_END_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH1_TX_END_INT_RAW_R {
        CH1_TX_END_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH0_TX_END_INT_RAW_R {
        CH0_TX_END_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&mut self) -> CH1_TX_LOOP_INT_RAW_W {
        CH1_TX_LOOP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&mut self) -> CH0_TX_LOOP_INT_RAW_W {
        CH0_TX_LOOP_INT_RAW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_raw(&mut self) -> CH3_RX_THR_EVENT_INT_RAW_W {
        CH3_RX_THR_EVENT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_raw(&mut self) -> CH2_RX_THR_EVENT_INT_RAW_W {
        CH2_RX_THR_EVENT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&mut self) -> CH1_TX_THR_EVENT_INT_RAW_W {
        CH1_TX_THR_EVENT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&mut self) -> CH0_TX_THR_EVENT_INT_RAW_W {
        CH0_TX_THR_EVENT_INT_RAW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch3_err_int_raw(&mut self) -> CH3_ERR_INT_RAW_W {
        CH3_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch2_err_int_raw(&mut self) -> CH2_ERR_INT_RAW_W {
        CH2_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch1_err_int_raw(&mut self) -> CH1_ERR_INT_RAW_W {
        CH1_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch0_err_int_raw(&mut self) -> CH0_ERR_INT_RAW_W {
        CH0_ERR_INT_RAW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3_rx_end_int_raw(&mut self) -> CH3_RX_END_INT_RAW_W {
        CH3_RX_END_INT_RAW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2_rx_end_int_raw(&mut self) -> CH2_RX_END_INT_RAW_W {
        CH2_RX_END_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&mut self) -> CH1_TX_END_INT_RAW_W {
        CH1_TX_END_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&mut self) -> CH0_TX_END_INT_RAW_W {
        CH0_TX_END_INT_RAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
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
