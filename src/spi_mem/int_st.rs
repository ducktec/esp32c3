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
#[doc = "Field `MST_ST_END_INT_ST` reader - "]
pub struct MST_ST_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl MST_ST_END_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_ST_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_ST_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_ST_END_INT_ST` reader - "]
pub struct SLV_ST_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl SLV_ST_END_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ST_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ST_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPE_END_INT_ST` reader - "]
pub struct WPE_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl WPE_END_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPE_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPE_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_END_INT_ST` reader - "]
pub struct PES_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl PES_END_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PES_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER_END_INT_ST` reader - "]
pub struct PER_END_INT_ST_R(crate::FieldReader<bool, bool>);
impl PER_END_INT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER_END_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_END_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mst_st_end_int_st(&self) -> MST_ST_END_INT_ST_R {
        MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_st_end_int_st(&self) -> SLV_ST_END_INT_ST_R {
        SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wpe_end_int_st(&self) -> WPE_END_INT_ST_R {
        WPE_END_INT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_end_int_st(&self) -> PES_END_INT_ST_R {
        PES_END_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn per_end_int_st(&self) -> PER_END_INT_ST_R {
        PER_END_INT_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SPI_MEM_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
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
