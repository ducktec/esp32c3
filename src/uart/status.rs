#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXD` reader - "]
pub struct TXD_R(crate::FieldReader<bool, bool>);
impl TXD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSN` reader - "]
pub struct RTSN_R(crate::FieldReader<bool, bool>);
impl RTSN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTSN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRN` reader - "]
pub struct DTRN_R(crate::FieldReader<bool, bool>);
impl DTRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_CNT` reader - "]
pub struct TXFIFO_CNT_R(crate::FieldReader<u16, u16>);
impl TXFIFO_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXD` reader - "]
pub struct RXD_R(crate::FieldReader<bool, bool>);
impl RXD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSN` reader - "]
pub struct CTSN_R(crate::FieldReader<bool, bool>);
impl CTSN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRN` reader - "]
pub struct DSRN_R(crate::FieldReader<bool, bool>);
impl DSRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_CNT` reader - "]
pub struct RXFIFO_CNT_R(crate::FieldReader<u16, u16>);
impl RXFIFO_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXFIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn txd(&self) -> TXD_R {
        TXD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rtsn(&self) -> RTSN_R {
        RTSN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dtrn(&self) -> DTRN_R {
        DTRN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ctsn(&self) -> CTSN_R {
        CTSN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dsrn(&self) -> DSRN_R {
        DSRN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UART_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
