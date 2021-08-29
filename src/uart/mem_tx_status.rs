#[doc = "Register `MEM_TX_STATUS` reader"]
pub struct R(crate::R<MEM_TX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_TX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_TX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_TX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_RADDR` reader - "]
pub struct TX_RADDR_R(crate::FieldReader<u16, u16>);
impl TX_RADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_RADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_TX_WADDR` reader - "]
pub struct APB_TX_WADDR_R(crate::FieldReader<u16, u16>);
impl APB_TX_WADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        APB_TX_WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_TX_WADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn tx_raddr(&self) -> TX_RADDR_R {
        TX_RADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn apb_tx_waddr(&self) -> APB_TX_WADDR_R {
        APB_TX_WADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UART_MEM_TX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_tx_status](index.html) module"]
pub struct MEM_TX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_TX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_tx_status::R](R) reader structure"]
impl crate::Readable for MEM_TX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_TX_STATUS to value 0"]
impl crate::Resettable for MEM_TX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
