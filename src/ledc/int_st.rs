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
#[doc = "Field `OVF_CNT_LSCH5_INT_ST` reader - "]
pub struct OVF_CNT_LSCH5_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH5_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH5_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH5_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH4_INT_ST` reader - "]
pub struct OVF_CNT_LSCH4_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH4_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH4_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH4_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH3_INT_ST` reader - "]
pub struct OVF_CNT_LSCH3_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH3_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH3_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH3_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH2_INT_ST` reader - "]
pub struct OVF_CNT_LSCH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH1_INT_ST` reader - "]
pub struct OVF_CNT_LSCH1_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH1_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH0_INT_ST` reader - "]
pub struct OVF_CNT_LSCH0_INT_ST_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH0_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ST` reader - "]
pub struct DUTY_CHNG_END_LSCH5_INT_ST_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH5_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH5_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH5_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ST` reader - "]
pub struct DUTY_CHNG_END_LSCH4_INT_ST_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH4_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH4_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH4_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ST` reader - "]
pub struct DUTY_CHNG_END_LSCH3_INT_ST_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH3_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH3_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH3_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ST` reader - "]
pub struct DUTY_CHNG_END_LSCH2_INT_ST_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH2_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH2_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH2_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ST` reader - "]
pub struct DUTY_CHNG_END_LSCH1_INT_ST_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH1_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH1_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH1_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ST` reader - "]
pub struct DUTY_CHNG_END_LSCH0_INT_ST_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH0_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH0_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH0_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_OVF_INT_ST` reader - "]
pub struct LSTIMER3_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl LSTIMER3_OVF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER3_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_OVF_INT_ST` reader - "]
pub struct LSTIMER2_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl LSTIMER2_OVF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_OVF_INT_ST` reader - "]
pub struct LSTIMER1_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl LSTIMER1_OVF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER1_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_OVF_INT_ST` reader - "]
pub struct LSTIMER0_OVF_INT_ST_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_OVF_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_OVF_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_OVF_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_st(&self) -> OVF_CNT_LSCH5_INT_ST_R {
        OVF_CNT_LSCH5_INT_ST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_st(&self) -> OVF_CNT_LSCH4_INT_ST_R {
        OVF_CNT_LSCH4_INT_ST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_st(&self) -> OVF_CNT_LSCH3_INT_ST_R {
        OVF_CNT_LSCH3_INT_ST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_st(&self) -> OVF_CNT_LSCH2_INT_ST_R {
        OVF_CNT_LSCH2_INT_ST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_st(&self) -> OVF_CNT_LSCH1_INT_ST_R {
        OVF_CNT_LSCH1_INT_ST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_st(&self) -> OVF_CNT_LSCH0_INT_ST_R {
        OVF_CNT_LSCH0_INT_ST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_st(&self) -> DUTY_CHNG_END_LSCH5_INT_ST_R {
        DUTY_CHNG_END_LSCH5_INT_ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_st(&self) -> DUTY_CHNG_END_LSCH4_INT_ST_R {
        DUTY_CHNG_END_LSCH4_INT_ST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_st(&self) -> DUTY_CHNG_END_LSCH3_INT_ST_R {
        DUTY_CHNG_END_LSCH3_INT_ST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_st(&self) -> DUTY_CHNG_END_LSCH2_INT_ST_R {
        DUTY_CHNG_END_LSCH2_INT_ST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_st(&self) -> DUTY_CHNG_END_LSCH1_INT_ST_R {
        DUTY_CHNG_END_LSCH1_INT_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_st(&self) -> DUTY_CHNG_END_LSCH0_INT_ST_R {
        DUTY_CHNG_END_LSCH0_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lstimer3_ovf_int_st(&self) -> LSTIMER3_OVF_INT_ST_R {
        LSTIMER3_OVF_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lstimer2_ovf_int_st(&self) -> LSTIMER2_OVF_INT_ST_R {
        LSTIMER2_OVF_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lstimer1_ovf_int_st(&self) -> LSTIMER1_OVF_INT_ST_R {
        LSTIMER1_OVF_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lstimer0_ovf_int_st(&self) -> LSTIMER0_OVF_INT_ST_R {
        LSTIMER0_OVF_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "LEDC_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
