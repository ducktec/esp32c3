#[doc = "Register `POSPULSE` reader"]
pub struct R(crate::R<POSPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POSPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POSPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POSPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POSEDGE_MIN_CNT` reader - "]
pub struct POSEDGE_MIN_CNT_R(crate::FieldReader<u16, u16>);
impl POSEDGE_MIN_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        POSEDGE_MIN_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSEDGE_MIN_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn posedge_min_cnt(&self) -> POSEDGE_MIN_CNT_R {
        POSEDGE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "UART_POSPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pospulse](index.html) module"]
pub struct POSPULSE_SPEC;
impl crate::RegisterSpec for POSPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pospulse::R](R) reader structure"]
impl crate::Readable for POSPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets POSPULSE to value 0"]
impl crate::Resettable for POSPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
