#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_CTRL1_INT_ENA` reader - "]
pub struct APP_CTRL1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APP_CTRL1_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL1_INT_ENA` writer - "]
pub struct APP_CTRL1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL1_INT_ENA_W<'a> {
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
#[doc = "Field `APP_CTRL0_INT_ENA` reader - "]
pub struct APP_CTRL0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl APP_CTRL0_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL0_INT_ENA` writer - "]
pub struct APP_CTRL0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CTRL0_INT_ENA_W<'a> {
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
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` reader - "]
pub struct OUTLINK_EOF_ERR_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OUTLINK_EOF_ERR_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_EOF_ERR_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_EOF_ERR_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_EOF_ERR_INT_ENA` writer - "]
pub struct OUTLINK_EOF_ERR_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTLINK_EOF_ERR_INT_ENA_W<'a> {
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
#[doc = "Field `SEND_A_Q_INT_ENA` reader - "]
pub struct SEND_A_Q_INT_ENA_R(crate::FieldReader<bool, bool>);
impl SEND_A_Q_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_A_Q_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_A_Q_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_A_Q_INT_ENA` writer - "]
pub struct SEND_A_Q_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_A_Q_INT_ENA_W<'a> {
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
#[doc = "Field `SEND_S_Q_INT_ENA` reader - "]
pub struct SEND_S_Q_INT_ENA_R(crate::FieldReader<bool, bool>);
impl SEND_S_Q_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_S_Q_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_S_Q_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_S_Q_INT_ENA` writer - "]
pub struct SEND_S_Q_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SEND_S_Q_INT_ENA_W<'a> {
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
#[doc = "Field `TX_HUNG_INT_ENA` reader - "]
pub struct TX_HUNG_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_HUNG_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_HUNG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HUNG_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HUNG_INT_ENA` writer - "]
pub struct TX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Field `RX_HUNG_INT_ENA` reader - "]
pub struct RX_HUNG_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RX_HUNG_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_HUNG_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HUNG_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_HUNG_INT_ENA` writer - "]
pub struct RX_HUNG_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_HUNG_INT_ENA_W<'a> {
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
#[doc = "Field `TX_START_INT_ENA` reader - "]
pub struct TX_START_INT_ENA_R(crate::FieldReader<bool, bool>);
impl TX_START_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START_INT_ENA` writer - "]
pub struct TX_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_INT_ENA_W<'a> {
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
#[doc = "Field `RX_START_INT_ENA` reader - "]
pub struct RX_START_INT_ENA_R(crate::FieldReader<bool, bool>);
impl RX_START_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_START_INT_ENA` writer - "]
pub struct RX_START_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_START_INT_ENA_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&self) -> APP_CTRL1_INT_ENA_R {
        APP_CTRL1_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&self) -> APP_CTRL0_INT_ENA_R {
        APP_CTRL0_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&self) -> OUTLINK_EOF_ERR_INT_ENA_R {
        OUTLINK_EOF_ERR_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_a_q_int_ena(&self) -> SEND_A_Q_INT_ENA_R {
        SEND_A_Q_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_s_q_int_ena(&self) -> SEND_S_Q_INT_ENA_R {
        SEND_S_Q_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&self) -> TX_HUNG_INT_ENA_R {
        TX_HUNG_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&self) -> RX_HUNG_INT_ENA_R {
        RX_HUNG_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_ena(&self) -> TX_START_INT_ENA_R {
        TX_START_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_ena(&self) -> RX_START_INT_ENA_R {
        RX_START_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_ctrl1_int_ena(&mut self) -> APP_CTRL1_INT_ENA_W {
        APP_CTRL1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_ctrl0_int_ena(&mut self) -> APP_CTRL0_INT_ENA_W {
        APP_CTRL0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn outlink_eof_err_int_ena(&mut self) -> OUTLINK_EOF_ERR_INT_ENA_W {
        OUTLINK_EOF_ERR_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_a_q_int_ena(&mut self) -> SEND_A_Q_INT_ENA_W {
        SEND_A_Q_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_s_q_int_ena(&mut self) -> SEND_S_Q_INT_ENA_W {
        SEND_S_Q_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_ena(&mut self) -> TX_HUNG_INT_ENA_W {
        TX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_ena(&mut self) -> RX_HUNG_INT_ENA_W {
        RX_HUNG_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_ena(&mut self) -> TX_START_INT_ENA_W {
        TX_START_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_ena(&mut self) -> RX_START_INT_ENA_W {
        RX_START_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
