#[doc = "Register `INTERRUPT_CORE0_CLOCK_GATE` reader"]
pub struct R(crate::R<INTERRUPT_CORE0_CLOCK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPT_CORE0_CLOCK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPT_CORE0_CLOCK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPT_CORE0_CLOCK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPT_CORE0_CLOCK_GATE` writer"]
pub struct W(crate::W<INTERRUPT_CORE0_CLOCK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPT_CORE0_CLOCK_GATE_SPEC>;
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
impl From<crate::W<INTERRUPT_CORE0_CLOCK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPT_CORE0_CLOCK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - "]
pub struct CLK_EN_R(crate::FieldReader<bool, bool>);
impl CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EN` writer - "]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTERRUPT_CORE0_CLOCK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrupt_core0_clock_gate](index.html) module"]
pub struct INTERRUPT_CORE0_CLOCK_GATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_CORE0_CLOCK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interrupt_core0_clock_gate::R](R) reader structure"]
impl crate::Readable for INTERRUPT_CORE0_CLOCK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interrupt_core0_clock_gate::W](W) writer structure"]
impl crate::Writable for INTERRUPT_CORE0_CLOCK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTERRUPT_CORE0_CLOCK_GATE to value 0"]
impl crate::Resettable for INTERRUPT_CORE0_CLOCK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
