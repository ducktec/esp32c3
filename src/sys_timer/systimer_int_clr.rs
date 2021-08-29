#[doc = "Register `SYSTIMER_INT_CLR` writer"]
pub struct W(crate::W<SYSTIMER_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTIMER_INT_CLR_SPEC>;
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
impl From<crate::W<SYSTIMER_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTIMER_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET2_INT_CLR` writer - "]
pub struct TARGET2_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TARGET1_INT_CLR` writer - "]
pub struct TARGET1_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET1_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TARGET0_INT_CLR` writer - "]
pub struct TARGET0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET0_INT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn target2_int_clr(&mut self) -> TARGET2_INT_CLR_W {
        TARGET2_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn target1_int_clr(&mut self) -> TARGET1_INT_CLR_W {
        TARGET1_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn target0_int_clr(&mut self) -> TARGET0_INT_CLR_W {
        TARGET0_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYS_TIMER_SYSTIMER_INT_CLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_int_clr](index.html) module"]
pub struct SYSTIMER_INT_CLR_SPEC;
impl crate::RegisterSpec for SYSTIMER_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [systimer_int_clr::W](W) writer structure"]
impl crate::Writable for SYSTIMER_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTIMER_INT_CLR to value 0"]
impl crate::Resettable for SYSTIMER_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
