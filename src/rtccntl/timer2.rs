#[doc = "Register `TIMER2` reader"]
pub struct R(crate::R<TIMER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2` writer"]
pub struct W(crate::W<TIMER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER2_SPEC>;
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
impl From<crate::W<TIMER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_TIME_CK8M_OFF` reader - "]
pub struct MIN_TIME_CK8M_OFF_R(crate::FieldReader<u8, u8>);
impl MIN_TIME_CK8M_OFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN_TIME_CK8M_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_TIME_CK8M_OFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_TIME_CK8M_OFF` writer - "]
pub struct MIN_TIME_CK8M_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_TIME_CK8M_OFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&self) -> MIN_TIME_CK8M_OFF_R {
        MIN_TIME_CK8M_OFF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn min_time_ck8m_off(&mut self) -> MIN_TIME_CK8M_OFF_W {
        MIN_TIME_CK8M_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_TIMER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2](index.html) module"]
pub struct TIMER2_SPEC;
impl crate::RegisterSpec for TIMER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2::R](R) reader structure"]
impl crate::Readable for TIMER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer2::W](W) writer structure"]
impl crate::Writable for TIMER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER2 to value 0"]
impl crate::Resettable for TIMER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
