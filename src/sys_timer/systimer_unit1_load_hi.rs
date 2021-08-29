#[doc = "Register `SYSTIMER_UNIT1_LOAD_HI` reader"]
pub struct R(crate::R<SYSTIMER_UNIT1_LOAD_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_UNIT1_LOAD_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_UNIT1_LOAD_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_UNIT1_LOAD_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTIMER_UNIT1_LOAD_HI` writer"]
pub struct W(crate::W<SYSTIMER_UNIT1_LOAD_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTIMER_UNIT1_LOAD_HI_SPEC>;
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
impl From<crate::W<SYSTIMER_UNIT1_LOAD_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTIMER_UNIT1_LOAD_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT1_LOAD_HI` reader - "]
pub struct TIMER_UNIT1_LOAD_HI_R(crate::FieldReader<u32, u32>);
impl TIMER_UNIT1_LOAD_HI_R {
    pub(crate) fn new(bits: u32) -> Self {
        TIMER_UNIT1_LOAD_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT1_LOAD_HI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT1_LOAD_HI` writer - "]
pub struct TIMER_UNIT1_LOAD_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT1_LOAD_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn timer_unit1_load_hi(&self) -> TIMER_UNIT1_LOAD_HI_R {
        TIMER_UNIT1_LOAD_HI_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn timer_unit1_load_hi(&mut self) -> TIMER_UNIT1_LOAD_HI_W {
        TIMER_UNIT1_LOAD_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_unit1_load_hi](index.html) module"]
pub struct SYSTIMER_UNIT1_LOAD_HI_SPEC;
impl crate::RegisterSpec for SYSTIMER_UNIT1_LOAD_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_unit1_load_hi::R](R) reader structure"]
impl crate::Readable for SYSTIMER_UNIT1_LOAD_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systimer_unit1_load_hi::W](W) writer structure"]
impl crate::Writable for SYSTIMER_UNIT1_LOAD_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTIMER_UNIT1_LOAD_HI to value 0"]
impl crate::Resettable for SYSTIMER_UNIT1_LOAD_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
