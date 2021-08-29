#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_WFULL_ERR` reader - "]
pub struct RXFIFO_WFULL_ERR_R(crate::FieldReader<bool, bool>);
impl RXFIFO_WFULL_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO_WFULL_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_WFULL_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_RST` writer - "]
pub struct RXFIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_RST_W<'a> {
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
#[doc = "Field `CS_HOLD_DLY_RES` reader - "]
pub struct CS_HOLD_DLY_RES_R(crate::FieldReader<u16, u16>);
impl CS_HOLD_DLY_RES_R {
    pub(crate) fn new(bits: u16) -> Self {
        CS_HOLD_DLY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_DLY_RES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_DLY_RES` writer - "]
pub struct CS_HOLD_DLY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_DLY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | ((value as u32 & 0x03ff) << 2);
        self.w
    }
}
#[doc = "Field `CLK_MODE` reader - "]
pub struct CLK_MODE_R(crate::FieldReader<u8, u8>);
impl CLK_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_MODE` writer - "]
pub struct CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rxfifo_wfull_err(&self) -> RXFIFO_WFULL_ERR_R {
        RXFIFO_WFULL_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CS_HOLD_DLY_RES_R {
        CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W {
        RXFIFO_RST_W { w: self }
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn cs_hold_dly_res(&mut self) -> CS_HOLD_DLY_RES_W {
        CS_HOLD_DLY_RES_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W {
        CLK_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
