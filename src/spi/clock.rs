#[doc = "Register `CLOCK` reader"]
pub struct R(crate::R<CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK` writer"]
pub struct W(crate::W<CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_SPEC>;
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
impl From<crate::W<CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EQU_SYSCLK` reader - "]
pub struct CLK_EQU_SYSCLK_R(crate::FieldReader<bool, bool>);
impl CLK_EQU_SYSCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_EQU_SYSCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_EQU_SYSCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_EQU_SYSCLK` writer - "]
pub struct CLK_EQU_SYSCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EQU_SYSCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `CLKDIV_PRE` reader - "]
pub struct CLKDIV_PRE_R(crate::FieldReader<u8, u8>);
impl CLKDIV_PRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV_PRE` writer - "]
pub struct CLKDIV_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Field `CLKCNT_N` reader - "]
pub struct CLKCNT_N_R(crate::FieldReader<u8, u8>);
impl CLKCNT_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKCNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCNT_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCNT_N` writer - "]
pub struct CLKCNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `CLKCNT_H` reader - "]
pub struct CLKCNT_H_R(crate::FieldReader<u8, u8>);
impl CLKCNT_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKCNT_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCNT_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCNT_H` writer - "]
pub struct CLKCNT_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `CLKCNT_L` reader - "]
pub struct CLKCNT_L_R(crate::FieldReader<u8, u8>);
impl CLKCNT_L_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKCNT_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCNT_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCNT_L` writer - "]
pub struct CLKCNT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCNT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_equ_sysclk(&self) -> CLK_EQU_SYSCLK_R {
        CLK_EQU_SYSCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn clkdiv_pre(&self) -> CLKDIV_PRE_R {
        CLKDIV_PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn clkcnt_n(&self) -> CLKCNT_N_R {
        CLKCNT_N_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn clkcnt_h(&self) -> CLKCNT_H_R {
        CLKCNT_H_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn clkcnt_l(&self) -> CLKCNT_L_R {
        CLKCNT_L_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn clk_equ_sysclk(&mut self) -> CLK_EQU_SYSCLK_W {
        CLK_EQU_SYSCLK_W { w: self }
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn clkdiv_pre(&mut self) -> CLKDIV_PRE_W {
        CLKDIV_PRE_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn clkcnt_n(&mut self) -> CLKCNT_N_W {
        CLKCNT_N_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn clkcnt_h(&mut self) -> CLKCNT_H_W {
        CLKCNT_H_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn clkcnt_l(&mut self) -> CLKCNT_L_W {
        CLKCNT_L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CLOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](index.html) module"]
pub struct CLOCK_SPEC;
impl crate::RegisterSpec for CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock::R](R) reader structure"]
impl crate::Readable for CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock::W](W) writer structure"]
impl crate::Writable for CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK to value 0"]
impl crate::Resettable for CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
