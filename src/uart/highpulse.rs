#[doc = "Register `HIGHPULSE` reader"]
pub struct R(crate::R<HIGHPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGHPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGHPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGHPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HIGHPULSE_MIN_CNT` reader - "]
pub struct HIGHPULSE_MIN_CNT_R(crate::FieldReader<u16, u16>);
impl HIGHPULSE_MIN_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        HIGHPULSE_MIN_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHPULSE_MIN_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn highpulse_min_cnt(&self) -> HIGHPULSE_MIN_CNT_R {
        HIGHPULSE_MIN_CNT_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "UART_HIGHPULSE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [highpulse](index.html) module"]
pub struct HIGHPULSE_SPEC;
impl crate::RegisterSpec for HIGHPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [highpulse::R](R) reader structure"]
impl crate::Readable for HIGHPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HIGHPULSE to value 0"]
impl crate::Resettable for HIGHPULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
