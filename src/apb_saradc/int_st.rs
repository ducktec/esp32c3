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
#[doc = "Field `ADC1_DONE_INT_ST` reader - "]
pub struct ADC1_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl ADC1_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_DONE_INT_ST` reader - "]
pub struct ADC2_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl ADC2_DONE_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_HIGH_INT_ST` reader - "]
pub struct THRES0_HIGH_INT_ST_R(crate::FieldReader<bool, bool>);
impl THRES0_HIGH_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        THRES0_HIGH_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_HIGH_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES1_HIGH_INT_ST` reader - "]
pub struct THRES1_HIGH_INT_ST_R(crate::FieldReader<bool, bool>);
impl THRES1_HIGH_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        THRES1_HIGH_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES1_HIGH_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES0_LOW_INT_ST` reader - "]
pub struct THRES0_LOW_INT_ST_R(crate::FieldReader<bool, bool>);
impl THRES0_LOW_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        THRES0_LOW_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES0_LOW_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRES1_LOW_INT_ST` reader - "]
pub struct THRES1_LOW_INT_ST_R(crate::FieldReader<bool, bool>);
impl THRES1_LOW_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        THRES1_LOW_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRES1_LOW_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adc1_done_int_st(&self) -> ADC1_DONE_INT_ST_R {
        ADC1_DONE_INT_ST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn adc2_done_int_st(&self) -> ADC2_DONE_INT_ST_R {
        ADC2_DONE_INT_ST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn thres0_high_int_st(&self) -> THRES0_HIGH_INT_ST_R {
        THRES0_HIGH_INT_ST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn thres1_high_int_st(&self) -> THRES1_HIGH_INT_ST_R {
        THRES1_HIGH_INT_ST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn thres0_low_int_st(&self) -> THRES0_LOW_INT_ST_R {
        THRES0_LOW_INT_ST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn thres1_low_int_st(&self) -> THRES1_LOW_INT_ST_R {
        THRES1_LOW_INT_ST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
#[doc = "APB_SARADC_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
