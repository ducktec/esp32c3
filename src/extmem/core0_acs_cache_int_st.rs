#[doc = "Register `CORE0_ACS_CACHE_INT_ST` reader"]
pub struct R(crate::R<CORE0_ACS_CACHE_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_ACS_CACHE_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_ACS_CACHE_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_ACS_CACHE_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE0_DBUS_WR_ICACHE_ST` reader - "]
pub struct CORE0_DBUS_WR_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl CORE0_DBUS_WR_ICACHE_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_DBUS_WR_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_WR_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_DBUS_REJECT_ST` reader - "]
pub struct CORE0_DBUS_REJECT_ST_R(crate::FieldReader<bool, bool>);
impl CORE0_DBUS_REJECT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_DBUS_REJECT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_REJECT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_DBUS_ACS_MSK_ICACHE_ST` reader - "]
pub struct CORE0_DBUS_ACS_MSK_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl CORE0_DBUS_ACS_MSK_ICACHE_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_DBUS_ACS_MSK_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_ACS_MSK_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_IBUS_REJECT_ST` reader - "]
pub struct CORE0_IBUS_REJECT_ST_R(crate::FieldReader<bool, bool>);
impl CORE0_IBUS_REJECT_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_IBUS_REJECT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_IBUS_REJECT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_IBUS_WR_ICACHE_ST` reader - "]
pub struct CORE0_IBUS_WR_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl CORE0_IBUS_WR_ICACHE_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_IBUS_WR_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_IBUS_WR_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_IBUS_ACS_MSK_ICACHE_ST` reader - "]
pub struct CORE0_IBUS_ACS_MSK_ICACHE_ST_R(crate::FieldReader<bool, bool>);
impl CORE0_IBUS_ACS_MSK_ICACHE_ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_IBUS_ACS_MSK_ICACHE_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_IBUS_ACS_MSK_ICACHE_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn core0_dbus_wr_icache_st(&self) -> CORE0_DBUS_WR_ICACHE_ST_R {
        CORE0_DBUS_WR_ICACHE_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn core0_dbus_reject_st(&self) -> CORE0_DBUS_REJECT_ST_R {
        CORE0_DBUS_REJECT_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn core0_dbus_acs_msk_icache_st(&self) -> CORE0_DBUS_ACS_MSK_ICACHE_ST_R {
        CORE0_DBUS_ACS_MSK_ICACHE_ST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn core0_ibus_reject_st(&self) -> CORE0_IBUS_REJECT_ST_R {
        CORE0_IBUS_REJECT_ST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn core0_ibus_wr_icache_st(&self) -> CORE0_IBUS_WR_ICACHE_ST_R {
        CORE0_IBUS_WR_ICACHE_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn core0_ibus_acs_msk_icache_st(&self) -> CORE0_IBUS_ACS_MSK_ICACHE_ST_R {
        CORE0_IBUS_ACS_MSK_ICACHE_ST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_acs_cache_int_st](index.html) module"]
pub struct CORE0_ACS_CACHE_INT_ST_SPEC;
impl crate::RegisterSpec for CORE0_ACS_CACHE_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_acs_cache_int_st::R](R) reader structure"]
impl crate::Readable for CORE0_ACS_CACHE_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE0_ACS_CACHE_INT_ST to value 0"]
impl crate::Resettable for CORE0_ACS_CACHE_INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
