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
#[doc = "Field `DETECT_START_INT_ST` reader - "]
pub struct DETECT_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl DETECT_START_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DETECT_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DETECT_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DATA_INT_ST` reader - "]
pub struct TX_DATA_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_DATA_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DATA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DATA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_INT_ST` reader - "]
pub struct RX_DATA_INT_ST_R(crate::FieldReader<bool, bool>);
impl RX_DATA_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_ERR_INT_ST` reader - "]
pub struct ACK_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl ACK_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT_INT_ST` reader - "]
pub struct TIMEOUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_COMPLETE_INT_ST` reader - "]
pub struct TRANS_COMPLETE_INT_ST_R(crate::FieldReader<bool, bool>);
impl TRANS_COMPLETE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_COMPLETE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_COMPLETE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_TRAN_COMP_INT_ST` reader - "]
pub struct MASTER_TRAN_COMP_INT_ST_R(crate::FieldReader<bool, bool>);
impl MASTER_TRAN_COMP_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_TRAN_COMP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_TRAN_COMP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBITRATION_LOST_INT_ST` reader - "]
pub struct ARBITRATION_LOST_INT_ST_R(crate::FieldReader<bool, bool>);
impl ARBITRATION_LOST_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBITRATION_LOST_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARBITRATION_LOST_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_TRAN_COMP_INT_ST` reader - "]
pub struct SLAVE_TRAN_COMP_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLAVE_TRAN_COMP_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_TRAN_COMP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_TRAN_COMP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn detect_start_int_st(&self) -> DETECT_START_INT_ST_R {
        DETECT_START_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_data_int_st(&self) -> TX_DATA_INT_ST_R {
        TX_DATA_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_data_int_st(&self) -> RX_DATA_INT_ST_R {
        RX_DATA_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ack_err_int_st(&self) -> ACK_ERR_INT_ST_R {
        ACK_ERR_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timeout_int_st(&self) -> TIMEOUT_INT_ST_R {
        TIMEOUT_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn master_tran_comp_int_st(&self) -> MASTER_TRAN_COMP_INT_ST_R {
        MASTER_TRAN_COMP_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slave_tran_comp_int_st(&self) -> SLAVE_TRAN_COMP_INT_ST_R {
        SLAVE_TRAN_COMP_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "RTC_I2C_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
