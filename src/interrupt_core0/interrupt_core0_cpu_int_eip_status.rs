#[doc = "Register `INTERRUPT_CORE0_CPU_INT_EIP_STATUS` reader"]
pub struct R(crate::R<INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPU_INT_EIP_STATUS` reader - "]
pub struct CPU_INT_EIP_STATUS_R(crate::FieldReader<u32, u32>);
impl CPU_INT_EIP_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        CPU_INT_EIP_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_INT_EIP_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_int_eip_status(&self) -> CPU_INT_EIP_STATUS_R {
        CPU_INT_EIP_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "INTERRUPT_CORE0_CPU_INT_EIP_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_cpu_int_eip_status](index.html) module"]
pub struct INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC;
impl crate::RegisterSpec for INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_core0_cpu_int_eip_status::R](R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERRUPT_CORE0_CPU_INT_EIP_STATUS to value 0"]
impl crate::Resettable for INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
