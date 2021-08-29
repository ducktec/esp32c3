#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_CLK_EN` reader - "]
pub struct MEM_CLK_EN_R(crate::FieldReader<bool, bool>);
impl MEM_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_CLK_EN` writer - "]
pub struct MEM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `AUTOBAUD_EN` reader - "]
pub struct AUTOBAUD_EN_R(crate::FieldReader<bool, bool>);
impl AUTOBAUD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOBAUD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOBAUD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOBAUD_EN` writer - "]
pub struct AUTOBAUD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOBAUD_EN_W<'a> {
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
#[doc = "Field `ERR_WR_MASK` reader - "]
pub struct ERR_WR_MASK_R(crate::FieldReader<bool, bool>);
impl ERR_WR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_WR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_WR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_WR_MASK` writer - "]
pub struct ERR_WR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_WR_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CLK_EN` reader - "]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - "]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DTR_INV` reader - "]
pub struct DTR_INV_R(crate::FieldReader<bool, bool>);
impl DTR_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTR_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTR_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTR_INV` writer - "]
pub struct DTR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RTS_INV` reader - "]
pub struct RTS_INV_R(crate::FieldReader<bool, bool>);
impl RTS_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTS_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTS_INV` writer - "]
pub struct RTS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_INV_W<'a> {
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
#[doc = "Field `TXD_INV` reader - "]
pub struct TXD_INV_R(crate::FieldReader<bool, bool>);
impl TXD_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXD_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXD_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXD_INV` writer - "]
pub struct TXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_INV_W<'a> {
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
#[doc = "Field `DSR_INV` reader - "]
pub struct DSR_INV_R(crate::FieldReader<bool, bool>);
impl DSR_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSR_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSR_INV` writer - "]
pub struct DSR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DSR_INV_W<'a> {
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
#[doc = "Field `CTS_INV` reader - "]
pub struct CTS_INV_R(crate::FieldReader<bool, bool>);
impl CTS_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTS_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTS_INV` writer - "]
pub struct CTS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_INV_W<'a> {
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
#[doc = "Field `RXD_INV` reader - "]
pub struct RXD_INV_R(crate::FieldReader<bool, bool>);
impl RXD_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXD_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXD_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXD_INV` writer - "]
pub struct RXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD_INV_W<'a> {
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
#[doc = "Field `TXFIFO_RST` reader - "]
pub struct TXFIFO_RST_R(crate::FieldReader<bool, bool>);
impl TXFIFO_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_RST` writer - "]
pub struct TXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_RST_W<'a> {
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
#[doc = "Field `RXFIFO_RST` reader - "]
pub struct RXFIFO_RST_R(crate::FieldReader<bool, bool>);
impl RXFIFO_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_RST` writer - "]
pub struct RXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_RST_W<'a> {
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
#[doc = "Field `IRDA_EN` reader - "]
pub struct IRDA_EN_R(crate::FieldReader<bool, bool>);
impl IRDA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_EN` writer - "]
pub struct IRDA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_EN_W<'a> {
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
#[doc = "Field `TX_FLOW_EN` reader - "]
pub struct TX_FLOW_EN_R(crate::FieldReader<bool, bool>);
impl TX_FLOW_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FLOW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FLOW_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_FLOW_EN` writer - "]
pub struct TX_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FLOW_EN_W<'a> {
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
#[doc = "Field `LOOPBACK` reader - "]
pub struct LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LOOPBACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACK` writer - "]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Field `IRDA_RX_INV` reader - "]
pub struct IRDA_RX_INV_R(crate::FieldReader<bool, bool>);
impl IRDA_RX_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDA_RX_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_RX_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_RX_INV` writer - "]
pub struct IRDA_RX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_RX_INV_W<'a> {
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
#[doc = "Field `IRDA_TX_INV` reader - "]
pub struct IRDA_TX_INV_R(crate::FieldReader<bool, bool>);
impl IRDA_TX_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDA_TX_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_TX_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_TX_INV` writer - "]
pub struct IRDA_TX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_TX_INV_W<'a> {
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
#[doc = "Field `IRDA_WCTL` reader - "]
pub struct IRDA_WCTL_R(crate::FieldReader<bool, bool>);
impl IRDA_WCTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDA_WCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_WCTL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_WCTL` writer - "]
pub struct IRDA_WCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_WCTL_W<'a> {
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
#[doc = "Field `IRDA_TX_EN` reader - "]
pub struct IRDA_TX_EN_R(crate::FieldReader<bool, bool>);
impl IRDA_TX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDA_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_TX_EN` writer - "]
pub struct IRDA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_TX_EN_W<'a> {
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
#[doc = "Field `IRDA_DPLX` reader - "]
pub struct IRDA_DPLX_R(crate::FieldReader<bool, bool>);
impl IRDA_DPLX_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRDA_DPLX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRDA_DPLX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRDA_DPLX` writer - "]
pub struct IRDA_DPLX_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_DPLX_W<'a> {
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
#[doc = "Field `TXD_BRK` reader - "]
pub struct TXD_BRK_R(crate::FieldReader<bool, bool>);
impl TXD_BRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXD_BRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXD_BRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXD_BRK` writer - "]
pub struct TXD_BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_BRK_W<'a> {
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
#[doc = "Field `SW_DTR` reader - "]
pub struct SW_DTR_R(crate::FieldReader<bool, bool>);
impl SW_DTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_DTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_DTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_DTR` writer - "]
pub struct SW_DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_DTR_W<'a> {
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
#[doc = "Field `SW_RTS` reader - "]
pub struct SW_RTS_R(crate::FieldReader<bool, bool>);
impl SW_RTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_RTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_RTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_RTS` writer - "]
pub struct SW_RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RTS_W<'a> {
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
#[doc = "Field `STOP_BIT_NUM` reader - "]
pub struct STOP_BIT_NUM_R(crate::FieldReader<u8, u8>);
impl STOP_BIT_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        STOP_BIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_BIT_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP_BIT_NUM` writer - "]
pub struct STOP_BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `BIT_NUM` reader - "]
pub struct BIT_NUM_R(crate::FieldReader<u8, u8>);
impl BIT_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIT_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_NUM` writer - "]
pub struct BIT_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PARITY_EN` reader - "]
pub struct PARITY_EN_R(crate::FieldReader<bool, bool>);
impl PARITY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY_EN` writer - "]
pub struct PARITY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_EN_W<'a> {
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
#[doc = "Field `PARITY` reader - "]
pub struct PARITY_R(crate::FieldReader<bool, bool>);
impl PARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARITY` writer - "]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn autobaud_en(&self) -> AUTOBAUD_EN_R {
        AUTOBAUD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DTR_INV_R {
        DTR_INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rts_inv(&self) -> RTS_INV_R {
        RTS_INV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DSR_INV_R {
        DSR_INV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cts_inv(&self) -> CTS_INV_R {
        CTS_INV_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irda_en(&self) -> IRDA_EN_R {
        IRDA_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn irda_rx_inv(&self) -> IRDA_RX_INV_R {
        IRDA_RX_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irda_tx_inv(&self) -> IRDA_TX_INV_R {
        IRDA_TX_INV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn irda_wctl(&self) -> IRDA_WCTL_R {
        IRDA_WCTL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn irda_tx_en(&self) -> IRDA_TX_EN_R {
        IRDA_TX_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn irda_dplx(&self) -> IRDA_DPLX_R {
        IRDA_DPLX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W {
        MEM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn autobaud_en(&mut self) -> AUTOBAUD_EN_W {
        AUTOBAUD_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W {
        ERR_WR_MASK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dtr_inv(&mut self) -> DTR_INV_W {
        DTR_INV_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rts_inv(&mut self) -> RTS_INV_W {
        RTS_INV_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn txd_inv(&mut self) -> TXD_INV_W {
        TXD_INV_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn dsr_inv(&mut self) -> DSR_INV_W {
        DSR_INV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cts_inv(&mut self) -> CTS_INV_W {
        CTS_INV_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rxd_inv(&mut self) -> RXD_INV_W {
        RXD_INV_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W {
        TXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W {
        RXFIFO_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irda_en(&mut self) -> IRDA_EN_W {
        IRDA_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W {
        TX_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn irda_rx_inv(&mut self) -> IRDA_RX_INV_W {
        IRDA_RX_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irda_tx_inv(&mut self) -> IRDA_TX_INV_W {
        IRDA_TX_INV_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn irda_wctl(&mut self) -> IRDA_WCTL_W {
        IRDA_WCTL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn irda_tx_en(&mut self) -> IRDA_TX_EN_W {
        IRDA_TX_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn irda_dplx(&mut self) -> IRDA_DPLX_W {
        IRDA_DPLX_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txd_brk(&mut self) -> TXD_BRK_W {
        TXD_BRK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sw_dtr(&mut self) -> SW_DTR_W {
        SW_DTR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sw_rts(&mut self) -> SW_RTS_W {
        SW_RTS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W {
        STOP_BIT_NUM_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn bit_num(&mut self) -> BIT_NUM_W {
        BIT_NUM_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn parity_en(&mut self) -> PARITY_EN_W {
        PARITY_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF0 to value 0"]
impl crate::Resettable for CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
