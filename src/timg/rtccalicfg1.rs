#[doc = "Register `RTCCALICFG1` reader"]
pub struct R(crate::R<RTCCALICFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - "]
pub struct VALUE_R(crate::FieldReader<u32, u32>);
impl VALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCLING_DATA_VLD` reader - "]
pub struct CYCLING_DATA_VLD_R(crate::FieldReader<bool, bool>);
impl CYCLING_DATA_VLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CYCLING_DATA_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLING_DATA_VLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 7:31"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cycling_data_vld(&self) -> CYCLING_DATA_VLD_R {
        CYCLING_DATA_VLD_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "TIMG_RTCCALICFG1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg1](index.html) module"]
pub struct RTCCALICFG1_SPEC;
impl crate::RegisterSpec for RTCCALICFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg1::R](R) reader structure"]
impl crate::Readable for RTCCALICFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTCCALICFG1 to value 0"]
impl crate::Resettable for RTCCALICFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
