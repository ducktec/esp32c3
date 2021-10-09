#[doc = "Register `CORE0_DBUS_REJECT_ST` reader"]
pub struct R(crate::R<CORE0_DBUS_REJECT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_DBUS_REJECT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_DBUS_REJECT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_DBUS_REJECT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE0_DBUS_WORLD` reader - "]
pub struct CORE0_DBUS_WORLD_R(crate::FieldReader<bool, bool>);
impl CORE0_DBUS_WORLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORE0_DBUS_WORLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_WORLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORE0_DBUS_ATTR` reader - "]
pub struct CORE0_DBUS_ATTR_R(crate::FieldReader<u8, u8>);
impl CORE0_DBUS_ATTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORE0_DBUS_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_ATTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn core0_dbus_world(&self) -> CORE0_DBUS_WORLD_R {
        CORE0_DBUS_WORLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn core0_dbus_attr(&self) -> CORE0_DBUS_ATTR_R {
        CORE0_DBUS_ATTR_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "EXTMEM_CORE0_DBUS_REJECT_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_dbus_reject_st](index.html) module"]
pub struct CORE0_DBUS_REJECT_ST_SPEC;
impl crate::RegisterSpec for CORE0_DBUS_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_dbus_reject_st::R](R) reader structure"]
impl crate::Readable for CORE0_DBUS_REJECT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE0_DBUS_REJECT_ST to value 0"]
impl crate::Resettable for CORE0_DBUS_REJECT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}