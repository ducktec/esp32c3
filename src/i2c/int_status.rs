#[doc = "Register `INT_STATUS` reader"]
pub struct R(crate::R<INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GENERAL_CALL_INT_ST` reader - "]
pub struct GENERAL_CALL_INT_ST_R(crate::FieldReader<bool, bool>);
impl GENERAL_CALL_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENERAL_CALL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENERAL_CALL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_STRETCH_INT_ST` reader - "]
pub struct SLAVE_STRETCH_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLAVE_STRETCH_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_STRETCH_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_STRETCH_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DET_START_INT_ST` reader - "]
pub struct DET_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl DET_START_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DET_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DET_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_MAIN_ST_TO_INT_ST` reader - "]
pub struct SCL_MAIN_ST_TO_INT_ST_R(crate::FieldReader<bool, bool>);
impl SCL_MAIN_ST_TO_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCL_MAIN_ST_TO_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_MAIN_ST_TO_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_ST_TO_INT_ST` reader - "]
pub struct SCL_ST_TO_INT_ST_R(crate::FieldReader<bool, bool>);
impl SCL_ST_TO_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCL_ST_TO_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_ST_TO_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_UDF_INT_ST` reader - "]
pub struct RXFIFO_UDF_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_UDF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_UDF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_UDF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_OVF_INT_ST` reader - "]
pub struct TXFIFO_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl TXFIFO_OVF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK_INT_ST` reader - "]
pub struct NACK_INT_ST_R(crate::FieldReader<bool, bool>);
impl NACK_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACK_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACK_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANS_START_INT_ST` reader - "]
pub struct TRANS_START_INT_ST_R(crate::FieldReader<bool, bool>);
impl TRANS_START_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRANS_START_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANS_START_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_OUT_INT_ST` reader - "]
pub struct TIME_OUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl TIME_OUT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIME_OUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_OUT_INT_ST_R {
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
#[doc = "Field `MST_TXFIFO_UDF_INT_ST` reader - "]
pub struct MST_TXFIFO_UDF_INT_ST_R(crate::FieldReader<bool, bool>);
impl MST_TXFIFO_UDF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_TXFIFO_UDF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_TXFIFO_UDF_INT_ST_R {
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
#[doc = "Field `BYTE_TRANS_DONE_INT_ST` reader - "]
pub struct BYTE_TRANS_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl BYTE_TRANS_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYTE_TRANS_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE_TRANS_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `END_DETECT_INT_ST` reader - "]
pub struct END_DETECT_INT_ST_R(crate::FieldReader<bool, bool>);
impl END_DETECT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        END_DETECT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for END_DETECT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_OVF_INT_ST` reader - "]
pub struct RXFIFO_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_OVF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_WM_INT_ST` reader - "]
pub struct TXFIFO_WM_INT_ST_R(crate::FieldReader<bool, bool>);
impl TXFIFO_WM_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_WM_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_WM_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_WM_INT_ST` reader - "]
pub struct RXFIFO_WM_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_WM_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_WM_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_WM_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn general_call_int_st(&self) -> GENERAL_CALL_INT_ST_R {
        GENERAL_CALL_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slave_stretch_int_st(&self) -> SLAVE_STRETCH_INT_ST_R {
        SLAVE_STRETCH_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn det_start_int_st(&self) -> DET_START_INT_ST_R {
        DET_START_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn scl_main_st_to_int_st(&self) -> SCL_MAIN_ST_TO_INT_ST_R {
        SCL_MAIN_ST_TO_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn scl_st_to_int_st(&self) -> SCL_ST_TO_INT_ST_R {
        SCL_ST_TO_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxfifo_udf_int_st(&self) -> RXFIFO_UDF_INT_ST_R {
        RXFIFO_UDF_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn txfifo_ovf_int_st(&self) -> TXFIFO_OVF_INT_ST_R {
        TXFIFO_OVF_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn nack_int_st(&self) -> NACK_INT_ST_R {
        NACK_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trans_start_int_st(&self) -> TRANS_START_INT_ST_R {
        TRANS_START_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn time_out_int_st(&self) -> TIME_OUT_INT_ST_R {
        TIME_OUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trans_complete_int_st(&self) -> TRANS_COMPLETE_INT_ST_R {
        TRANS_COMPLETE_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_st(&self) -> MST_TXFIFO_UDF_INT_ST_R {
        MST_TXFIFO_UDF_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn arbitration_lost_int_st(&self) -> ARBITRATION_LOST_INT_ST_R {
        ARBITRATION_LOST_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn byte_trans_done_int_st(&self) -> BYTE_TRANS_DONE_INT_ST_R {
        BYTE_TRANS_DONE_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn end_detect_int_st(&self) -> END_DETECT_INT_ST_R {
        END_DETECT_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_wm_int_st(&self) -> TXFIFO_WM_INT_ST_R {
        TXFIFO_WM_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_wm_int_st(&self) -> RXFIFO_WM_INT_ST_R {
        RXFIFO_WM_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "I2C_INT_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](index.html) module"]
pub struct INT_STATUS_SPEC;
impl crate::RegisterSpec for INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status::R](R) reader structure"]
impl crate::Readable for INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
