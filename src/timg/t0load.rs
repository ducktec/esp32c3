#[doc = "Register `T0LOAD` writer"]
pub struct W(crate::W<T0LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOAD_SPEC>;
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
impl From<crate::W<T0LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_LOAD` writer - "]
pub struct T0_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn t0_load(&mut self) -> T0_LOAD_W {
        T0_LOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_T0LOAD\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0load](index.html) module"]
pub struct T0LOAD_SPEC;
impl crate::RegisterSpec for T0LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [t0load::W](W) writer structure"]
impl crate::Writable for T0LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0LOAD to value 0"]
impl crate::Resettable for T0LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
