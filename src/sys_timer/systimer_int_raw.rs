#[doc = "Register `SYSTIMER_INT_RAW` reader"]
pub struct R(crate::R<SYSTIMER_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTIMER_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTIMER_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTIMER_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTIMER_INT_RAW` writer"]
pub struct W(crate::W<SYSTIMER_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTIMER_INT_RAW_SPEC>;
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
impl From<crate::W<SYSTIMER_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTIMER_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TARGET2_INT_RAW` reader - "]
pub struct TARGET2_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TARGET2_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET2_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET2_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET2_INT_RAW` writer - "]
pub struct TARGET2_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET2_INT_RAW_W<'a> {
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
#[doc = "Field `TARGET1_INT_RAW` reader - "]
pub struct TARGET1_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TARGET1_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET1_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET1_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET1_INT_RAW` writer - "]
pub struct TARGET1_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET1_INT_RAW_W<'a> {
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
#[doc = "Field `TARGET0_INT_RAW` reader - "]
pub struct TARGET0_INT_RAW_R(crate::FieldReader<bool, bool>);
impl TARGET0_INT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TARGET0_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TARGET0_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TARGET0_INT_RAW` writer - "]
pub struct TARGET0_INT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET0_INT_RAW_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn target2_int_raw(&self) -> TARGET2_INT_RAW_R {
        TARGET2_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn target1_int_raw(&self) -> TARGET1_INT_RAW_R {
        TARGET1_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn target0_int_raw(&self) -> TARGET0_INT_RAW_R {
        TARGET0_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn target2_int_raw(&mut self) -> TARGET2_INT_RAW_W {
        TARGET2_INT_RAW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn target1_int_raw(&mut self) -> TARGET1_INT_RAW_W {
        TARGET1_INT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn target0_int_raw(&mut self) -> TARGET0_INT_RAW_W {
        TARGET0_INT_RAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYS_TIMER_SYSTIMER_INT_RAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systimer_int_raw](index.html) module"]
pub struct SYSTIMER_INT_RAW_SPEC;
impl crate::RegisterSpec for SYSTIMER_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systimer_int_raw::R](R) reader structure"]
impl crate::Readable for SYSTIMER_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systimer_int_raw::W](W) writer structure"]
impl crate::Writable for SYSTIMER_INT_RAW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTIMER_INT_RAW to value 0"]
impl crate::Resettable for SYSTIMER_INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
