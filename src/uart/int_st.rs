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
#[doc = "Field `WAKEUP_INT_ST` reader - "]
pub struct WAKEUP_INT_ST_R(crate::FieldReader<bool, bool>);
impl WAKEUP_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AT_CMD_CHAR_DET_INT_ST` reader - "]
pub struct AT_CMD_CHAR_DET_INT_ST_R(crate::FieldReader<bool, bool>);
impl AT_CMD_CHAR_DET_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AT_CMD_CHAR_DET_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AT_CMD_CHAR_DET_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_CLASH_INT_ST` reader - "]
pub struct RS485_CLASH_INT_ST_R(crate::FieldReader<bool, bool>);
impl RS485_CLASH_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485_CLASH_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_CLASH_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_FRM_ERR_INT_ST` reader - "]
pub struct RS485_FRM_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl RS485_FRM_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485_FRM_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_FRM_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS485_PARITY_ERR_INT_ST` reader - "]
pub struct RS485_PARITY_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl RS485_PARITY_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS485_PARITY_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS485_PARITY_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_DONE_INT_ST` reader - "]
pub struct TX_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_IDLE_DONE_INT_ST` reader - "]
pub struct TX_BRK_IDLE_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_BRK_IDLE_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_IDLE_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_IDLE_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_BRK_DONE_INT_ST` reader - "]
pub struct TX_BRK_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl TX_BRK_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_BRK_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_BRK_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLITCH_DET_INT_ST` reader - "]
pub struct GLITCH_DET_INT_ST_R(crate::FieldReader<bool, bool>);
impl GLITCH_DET_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLITCH_DET_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLITCH_DET_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XOFF_INT_ST` reader - "]
pub struct SW_XOFF_INT_ST_R(crate::FieldReader<bool, bool>);
impl SW_XOFF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_XOFF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XOFF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_XON_INT_ST` reader - "]
pub struct SW_XON_INT_ST_R(crate::FieldReader<bool, bool>);
impl SW_XON_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_XON_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_XON_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_TOUT_INT_ST` reader - "]
pub struct RXFIFO_TOUT_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_TOUT_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_TOUT_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_TOUT_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK_DET_INT_ST` reader - "]
pub struct BRK_DET_INT_ST_R(crate::FieldReader<bool, bool>);
impl BRK_DET_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK_DET_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_DET_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_CHG_INT_ST` reader - "]
pub struct CTS_CHG_INT_ST_R(crate::FieldReader<bool, bool>);
impl CTS_CHG_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTS_CHG_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_CHG_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR_CHG_INT_ST` reader - "]
pub struct DSR_CHG_INT_ST_R(crate::FieldReader<bool, bool>);
impl DSR_CHG_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR_CHG_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_CHG_INT_ST_R {
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
#[doc = "Field `FRM_ERR_INT_ST` reader - "]
pub struct FRM_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl FRM_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRM_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRM_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_ERR_INT_ST` reader - "]
pub struct PARITY_ERR_INT_ST_R(crate::FieldReader<bool, bool>);
impl PARITY_ERR_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_ERR_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_ERR_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_INT_ST` reader - "]
pub struct TXFIFO_EMPTY_INT_ST_R(crate::FieldReader<bool, bool>);
impl TXFIFO_EMPTY_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_EMPTY_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_FULL_INT_ST` reader - "]
pub struct RXFIFO_FULL_INT_ST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_FULL_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_FULL_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wakeup_int_st(&self) -> WAKEUP_INT_ST_R {
        WAKEUP_INT_ST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn at_cmd_char_det_int_st(&self) -> AT_CMD_CHAR_DET_INT_ST_R {
        AT_CMD_CHAR_DET_INT_ST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rs485_clash_int_st(&self) -> RS485_CLASH_INT_ST_R {
        RS485_CLASH_INT_ST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rs485_frm_err_int_st(&self) -> RS485_FRM_ERR_INT_ST_R {
        RS485_FRM_ERR_INT_ST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rs485_parity_err_int_st(&self) -> RS485_PARITY_ERR_INT_ST_R {
        RS485_PARITY_ERR_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tx_done_int_st(&self) -> TX_DONE_INT_ST_R {
        TX_DONE_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_st(&self) -> TX_BRK_IDLE_DONE_INT_ST_R {
        TX_BRK_IDLE_DONE_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_brk_done_int_st(&self) -> TX_BRK_DONE_INT_ST_R {
        TX_BRK_DONE_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn glitch_det_int_st(&self) -> GLITCH_DET_INT_ST_R {
        GLITCH_DET_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_xoff_int_st(&self) -> SW_XOFF_INT_ST_R {
        SW_XOFF_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_xon_int_st(&self) -> SW_XON_INT_ST_R {
        SW_XON_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxfifo_tout_int_st(&self) -> RXFIFO_TOUT_INT_ST_R {
        RXFIFO_TOUT_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn brk_det_int_st(&self) -> BRK_DET_INT_ST_R {
        BRK_DET_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cts_chg_int_st(&self) -> CTS_CHG_INT_ST_R {
        CTS_CHG_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dsr_chg_int_st(&self) -> DSR_CHG_INT_ST_R {
        DSR_CHG_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_st(&self) -> RXFIFO_OVF_INT_ST_R {
        RXFIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frm_err_int_st(&self) -> FRM_ERR_INT_ST_R {
        FRM_ERR_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn parity_err_int_st(&self) -> PARITY_ERR_INT_ST_R {
        PARITY_ERR_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txfifo_empty_int_st(&self) -> TXFIFO_EMPTY_INT_ST_R {
        TXFIFO_EMPTY_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfifo_full_int_st(&self) -> RXFIFO_FULL_INT_ST_R {
        RXFIFO_FULL_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "UART_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
