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
#[doc = "Field `APP_CTRL1_INT_ST` reader - "]
pub struct APP_CTRL1_INT_ST_R(crate::FieldReader<bool, bool>);
impl APP_CTRL1_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APP_CTRL0_INT_ST` reader - "]
pub struct APP_CTRL0_INT_ST_R(crate::FieldReader<bool, bool>);
impl APP_CTRL0_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        APP_CTRL0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APP_CTRL0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTLINK_EOF_ERR_INT_ST` reader - "]
pub struct OUTLINK_EOF_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl OUTLINK_EOF_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTLINK_EOF_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTLINK_EOF_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_A_Q_INT_ST` reader - "]
pub struct SEND_A_Q_INT_ST_R(crate::FieldReader<bool, bool>);
impl SEND_A_Q_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_A_Q_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_A_Q_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEND_S_Q_INT_ST` reader - "]
pub struct SEND_S_Q_INT_ST_R(crate::FieldReader<bool, bool>);
impl SEND_S_Q_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEND_S_Q_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEND_S_Q_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_HUNG_INT_ST` reader - "]
pub struct TX_HUNG_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_HUNG_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_HUNG_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_HUNG_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_HUNG_INT_ST` reader - "]
pub struct RX_HUNG_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_HUNG_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_HUNG_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_HUNG_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_START_INT_ST` reader - "]
pub struct TX_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_START_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_START_INT_ST` reader - "]
pub struct RX_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_START_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn app_ctrl1_int_st(&self) -> APP_CTRL1_INT_ST_R {
        APP_CTRL1_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn app_ctrl0_int_st(&self) -> APP_CTRL0_INT_ST_R {
        APP_CTRL0_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn outlink_eof_err_int_st(&self) -> OUTLINK_EOF_ERR_INT_ST_R {
        OUTLINK_EOF_ERR_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn send_a_q_int_st(&self) -> SEND_A_Q_INT_ST_R {
        SEND_A_Q_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn send_s_q_int_st(&self) -> SEND_S_Q_INT_ST_R {
        SEND_S_Q_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> TX_HUNG_INT_ST_R {
        TX_HUNG_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start_int_st(&self) -> TX_START_INT_ST_R {
        TX_START_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start_int_st(&self) -> RX_START_INT_ST_R {
        RX_START_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "UHCI_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
