#[doc = "Register `WDTCONFIG2` reader"]
pub struct R(crate::R<WDTCONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG2` writer"]
pub struct W(crate::W<WDTCONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG2_SPEC>;
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
impl From<crate::W<WDTCONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG1_HOLD` reader - "]
pub struct WDT_STG1_HOLD_R(crate::FieldReader<u32, u32>);
impl WDT_STG1_HOLD_R {
    pub(crate) fn new(bits: u32) -> Self {
        WDT_STG1_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STG1_HOLD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_STG1_HOLD` writer - "]
pub struct WDT_STG1_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STG1_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> WDT_STG1_HOLD_R {
        WDT_STG1_HOLD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wdt_stg1_hold(&mut self) -> WDT_STG1_HOLD_W {
        WDT_STG1_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_WDTCONFIG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig2](index.html) module"]
pub struct WDTCONFIG2_SPEC;
impl crate::RegisterSpec for WDTCONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig2::R](R) reader structure"]
impl crate::Readable for WDTCONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig2::W](W) writer structure"]
impl crate::Writable for WDTCONFIG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCONFIG2 to value 0"]
impl crate::Resettable for WDTCONFIG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
