#[doc = "Register `MEM_RX_STATUS` reader"]
pub struct R(crate::R<MEM_RX_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_RX_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_RX_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_RX_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_WADDR` reader - "]
pub struct RX_WADDR_R(crate::FieldReader<u16, u16>);
impl RX_WADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_WADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_RX_RADDR` reader - "]
pub struct APB_RX_RADDR_R(crate::FieldReader<u16, u16>);
impl APB_RX_RADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        APB_RX_RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_RX_RADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 11:20"]
    #[inline(always)]
    pub fn rx_waddr(&self) -> RX_WADDR_R {
        RX_WADDR_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn apb_rx_raddr(&self) -> APB_RX_RADDR_R {
        APB_RX_RADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "UART_MEM_RX_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_rx_status](index.html) module"]
pub struct MEM_RX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_RX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_rx_status::R](R) reader structure"]
impl crate::Readable for MEM_RX_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_RX_STATUS to value 0"]
impl crate::Resettable for MEM_RX_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
