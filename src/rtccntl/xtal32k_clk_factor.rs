#[doc = "Register `XTAL32K_CLK_FACTOR` reader"]
pub struct R(crate::R<XTAL32K_CLK_FACTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_CLK_FACTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_CLK_FACTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_CLK_FACTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K_CLK_FACTOR` writer"]
pub struct W(crate::W<XTAL32K_CLK_FACTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_CLK_FACTOR_SPEC>;
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
impl From<crate::W<XTAL32K_CLK_FACTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_CLK_FACTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32K_CLK_FACTOR` reader - "]
pub struct XTAL32K_CLK_FACTOR_R(crate::FieldReader<u32, u32>);
impl XTAL32K_CLK_FACTOR_R {
    pub(crate) fn new(bits: u32) -> Self {
        XTAL32K_CLK_FACTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_CLK_FACTOR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_CLK_FACTOR` writer - "]
pub struct XTAL32K_CLK_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_CLK_FACTOR_W<'a> {
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
    pub fn xtal32k_clk_factor(&self) -> XTAL32K_CLK_FACTOR_R {
        XTAL32K_CLK_FACTOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn xtal32k_clk_factor(&mut self) -> XTAL32K_CLK_FACTOR_W {
        XTAL32K_CLK_FACTOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_XTAL32K_CLK_FACTOR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k_clk_factor](index.html) module"]
pub struct XTAL32K_CLK_FACTOR_SPEC;
impl crate::RegisterSpec for XTAL32K_CLK_FACTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k_clk_factor::R](R) reader structure"]
impl crate::Readable for XTAL32K_CLK_FACTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k_clk_factor::W](W) writer structure"]
impl crate::Writable for XTAL32K_CLK_FACTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32K_CLK_FACTOR to value 0"]
impl crate::Resettable for XTAL32K_CLK_FACTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
