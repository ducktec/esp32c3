#[doc = "Register `CLK_GATE` reader"]
pub struct R(crate::R<CLK_GATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GATE` writer"]
pub struct W(crate::W<CLK_GATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GATE_SPEC>;
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
impl From<crate::W<CLK_GATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MST_CLK_SEL` reader - "]
pub struct MST_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl MST_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_CLK_SEL` writer - "]
pub struct MST_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_CLK_SEL_W<'a> {
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
#[doc = "Field `MST_CLK_ACTIVE` reader - "]
pub struct MST_CLK_ACTIVE_R(crate::FieldReader<bool, bool>);
impl MST_CLK_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_CLK_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_CLK_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_CLK_ACTIVE` writer - "]
pub struct MST_CLK_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_CLK_ACTIVE_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mst_clk_sel(&self) -> MST_CLK_SEL_R {
        MST_CLK_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mst_clk_active(&self) -> MST_CLK_ACTIVE_R {
        MST_CLK_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mst_clk_sel(&mut self) -> MST_CLK_SEL_W {
        MST_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mst_clk_active(&mut self) -> MST_CLK_ACTIVE_W {
        MST_CLK_ACTIVE_W { w: self }
    }
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
#[doc = "SPI_CLK_GATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gate](index.html) module"]
pub struct CLK_GATE_SPEC;
impl crate::RegisterSpec for CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gate::R](R) reader structure"]
impl crate::Readable for CLK_GATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gate::W](W) writer structure"]
impl crate::Writable for CLK_GATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_GATE to value 0"]
impl crate::Resettable for CLK_GATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
