#[doc = "Register `SIGMADELTA_MISC` reader"]
pub struct R(crate::R<SIGMADELTA_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA_MISC` writer"]
pub struct W(crate::W<SIGMADELTA_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_MISC_SPEC>;
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
impl From<crate::W<SIGMADELTA_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SWAP` reader - "]
pub struct SPI_SWAP_R(crate::FieldReader<bool, bool>);
impl SPI_SWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SWAP` writer - "]
pub struct SPI_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SWAP_W<'a> {
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
#[doc = "Field `FUNCTION_CLK_EN` reader - "]
pub struct FUNCTION_CLK_EN_R(crate::FieldReader<bool, bool>);
impl FUNCTION_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUNCTION_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUNCTION_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNCTION_CLK_EN` writer - "]
pub struct FUNCTION_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNCTION_CLK_EN_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_swap(&self) -> SPI_SWAP_R {
        SPI_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn function_clk_en(&self) -> FUNCTION_CLK_EN_R {
        FUNCTION_CLK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn spi_swap(&mut self) -> SPI_SWAP_W {
        SPI_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn function_clk_en(&mut self) -> FUNCTION_CLK_EN_W {
        FUNCTION_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_SIGMADELTA_MISC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_misc](index.html) module"]
pub struct SIGMADELTA_MISC_SPEC;
impl crate::RegisterSpec for SIGMADELTA_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta_misc::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta_misc::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA_MISC to value 0"]
impl crate::Resettable for SIGMADELTA_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
