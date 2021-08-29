#[doc = "Register `SYSTIMER_UNIT1_OP` reader"]
pub struct R(crate::R<SYSTIMER_UNIT1_OP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_UNIT1_OP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_UNIT1_OP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_UNIT1_OP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTIMER_UNIT1_OP` writer"]
pub struct W(crate::W<SYSTIMER_UNIT1_OP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTIMER_UNIT1_OP_SPEC>;
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
impl From<crate::W<SYSTIMER_UNIT1_OP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTIMER_UNIT1_OP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT1_UPDATE` writer - "]
pub struct TIMER_UNIT1_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT1_UPDATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `TIMER_UNIT1_VALUE_VALID` reader - "]
pub struct TIMER_UNIT1_VALUE_VALID_R(crate::FieldReader<bool, bool>);
impl TIMER_UNIT1_VALUE_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER_UNIT1_VALUE_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_UNIT1_VALUE_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_UNIT1_VALUE_VALID` writer - "]
pub struct TIMER_UNIT1_VALUE_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_UNIT1_VALUE_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timer_unit1_value_valid(&self) -> TIMER_UNIT1_VALUE_VALID_R {
        TIMER_UNIT1_VALUE_VALID_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn timer_unit1_update(&mut self) -> TIMER_UNIT1_UPDATE_W {
        TIMER_UNIT1_UPDATE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn timer_unit1_value_valid(&mut self) -> TIMER_UNIT1_VALUE_VALID_W {
        TIMER_UNIT1_VALUE_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_OP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_unit1_op](index.html) module"]
pub struct SYSTIMER_UNIT1_OP_SPEC;
impl crate::RegisterSpec for SYSTIMER_UNIT1_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_unit1_op::R](R) reader structure"]
impl crate::Readable for SYSTIMER_UNIT1_OP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systimer_unit1_op::W](W) writer structure"]
impl crate::Writable for SYSTIMER_UNIT1_OP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTIMER_UNIT1_OP to value 0"]
impl crate::Resettable for SYSTIMER_UNIT1_OP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
