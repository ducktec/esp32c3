#[doc = "Register `STATE0` reader"]
pub struct R(crate::R<STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DECODE_STATE` reader - "]
pub struct DECODE_STATE_R(crate::FieldReader<u8, u8>);
impl DECODE_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DECODE_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECODE_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERR_CAUSE` reader - "]
pub struct RX_ERR_CAUSE_R(crate::FieldReader<u8, u8>);
impl RX_ERR_CAUSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_ERR_CAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERR_CAUSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn decode_state(&self) -> DECODE_STATE_R {
        DECODE_STATE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rx_err_cause(&self) -> RX_ERR_CAUSE_R {
        RX_ERR_CAUSE_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "UHCI_STATE0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state0](index.html) module"]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state0::R](R) reader structure"]
impl crate::Readable for STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}