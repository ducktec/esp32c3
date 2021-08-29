#[doc = "Register `CORE0_DBUS_REJECT_VADDR` reader"]
pub struct R(crate::R<CORE0_DBUS_REJECT_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_DBUS_REJECT_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_DBUS_REJECT_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_DBUS_REJECT_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE0_DBUS_VADDR` reader - "]
pub struct CORE0_DBUS_VADDR_R(crate::FieldReader<u32, u32>);
impl CORE0_DBUS_VADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CORE0_DBUS_VADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE0_DBUS_VADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn core0_dbus_vaddr(&self) -> CORE0_DBUS_VADDR_R {
        CORE0_DBUS_VADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "EXTMEM_CORE0_DBUS_REJECT_VADDR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_dbus_reject_vaddr](index.html) module"]
pub struct CORE0_DBUS_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for CORE0_DBUS_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_dbus_reject_vaddr::R](R) reader structure"]
impl crate::Readable for CORE0_DBUS_REJECT_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE0_DBUS_REJECT_VADDR to value 0"]
impl crate::Resettable for CORE0_DBUS_REJECT_VADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
