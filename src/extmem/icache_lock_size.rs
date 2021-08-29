#[doc = "Register `ICACHE_LOCK_SIZE` reader"]
pub struct R(crate::R<ICACHE_LOCK_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_LOCK_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_LOCK_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_LOCK_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_LOCK_SIZE` writer"]
pub struct W(crate::W<ICACHE_LOCK_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_LOCK_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICACHE_LOCK_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_LOCK_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_LOCK_SIZE` reader - "]
pub struct ICACHE_LOCK_SIZE_R(crate::FieldReader<u16, u16>);
impl ICACHE_LOCK_SIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        ICACHE_LOCK_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_LOCK_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_LOCK_SIZE` writer - "]
pub struct ICACHE_LOCK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_LOCK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn icache_lock_size(&self) -> ICACHE_LOCK_SIZE_R {
        ICACHE_LOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn icache_lock_size(&mut self) -> ICACHE_LOCK_SIZE_W {
        ICACHE_LOCK_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_ICACHE_LOCK_SIZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_lock_size](index.html) module"]
pub struct ICACHE_LOCK_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE_LOCK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_lock_size::R](R) reader structure"]
impl crate::Readable for ICACHE_LOCK_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_lock_size::W](W) writer structure"]
impl crate::Writable for ICACHE_LOCK_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_LOCK_SIZE to value 0"]
impl crate::Resettable for ICACHE_LOCK_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
