#[doc = "Register `WR_TIM_CONF2` reader"]
pub struct R(crate::R<WR_TIM_CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_TIM_CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_TIM_CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_TIM_CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_TIM_CONF2` writer"]
pub struct W(crate::W<WR_TIM_CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_TIM_CONF2_SPEC>;
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
impl From<crate::W<WR_TIM_CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_TIM_CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_OFF_NUM` reader - "]
pub struct PWR_OFF_NUM_R(crate::FieldReader<u16, u16>);
impl PWR_OFF_NUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWR_OFF_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_OFF_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_OFF_NUM` writer - "]
pub struct PWR_OFF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_OFF_NUM_W<'a> {
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
    pub fn pwr_off_num(&self) -> PWR_OFF_NUM_R {
        PWR_OFF_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwr_off_num(&mut self) -> PWR_OFF_NUM_W {
        PWR_OFF_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EFUSE_WR_TIM_CONF2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_tim_conf2](index.html) module"]
pub struct WR_TIM_CONF2_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_tim_conf2::R](R) reader structure"]
impl crate::Readable for WR_TIM_CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_tim_conf2::W](W) writer structure"]
impl crate::Writable for WR_TIM_CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR_TIM_CONF2 to value 0"]
impl crate::Resettable for WR_TIM_CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
