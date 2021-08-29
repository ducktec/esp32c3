#[doc = "Register `PIN_CTRL` reader"]
pub struct R(crate::R<PIN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN_CTRL` writer"]
pub struct W(crate::W<PIN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_CTRL_SPEC>;
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
impl From<crate::W<PIN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_CTRL_CLK3` reader - Controls I2S output clock to CLK_OUT_out3"]
pub struct PIN_CTRL_CLK3_R(crate::FieldReader<u8, u8>);
impl PIN_CTRL_CLK3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIN_CTRL_CLK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_CTRL_CLK3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_CTRL_CLK3` writer - Controls I2S output clock to CLK_OUT_out3"]
pub struct PIN_CTRL_CLK3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CTRL_CLK3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `PIN_CTRL_CLK2` reader - Controls I2S output clock to CLK_OUT_out2"]
pub struct PIN_CTRL_CLK2_R(crate::FieldReader<u8, u8>);
impl PIN_CTRL_CLK2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIN_CTRL_CLK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_CTRL_CLK2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_CTRL_CLK2` writer - Controls I2S output clock to CLK_OUT_out2"]
pub struct PIN_CTRL_CLK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CTRL_CLK2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PIN_CTRL_CLK1` reader - Controls I2S output clock to CLK_OUT_out1"]
pub struct PIN_CTRL_CLK1_R(crate::FieldReader<u8, u8>);
impl PIN_CTRL_CLK1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIN_CTRL_CLK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_CTRL_CLK1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_CTRL_CLK1` writer - Controls I2S output clock to CLK_OUT_out1"]
pub struct PIN_CTRL_CLK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_CTRL_CLK1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Controls I2S output clock to CLK_OUT_out3"]
    #[inline(always)]
    pub fn pin_ctrl_clk3(&self) -> PIN_CTRL_CLK3_R {
        PIN_CTRL_CLK3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Controls I2S output clock to CLK_OUT_out2"]
    #[inline(always)]
    pub fn pin_ctrl_clk2(&self) -> PIN_CTRL_CLK2_R {
        PIN_CTRL_CLK2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Controls I2S output clock to CLK_OUT_out1"]
    #[inline(always)]
    pub fn pin_ctrl_clk1(&self) -> PIN_CTRL_CLK1_R {
        PIN_CTRL_CLK1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Controls I2S output clock to CLK_OUT_out3"]
    #[inline(always)]
    pub fn pin_ctrl_clk3(&mut self) -> PIN_CTRL_CLK3_W {
        PIN_CTRL_CLK3_W { w: self }
    }
    #[doc = "Bits 4:7 - Controls I2S output clock to CLK_OUT_out2"]
    #[inline(always)]
    pub fn pin_ctrl_clk2(&mut self) -> PIN_CTRL_CLK2_W {
        PIN_CTRL_CLK2_W { w: self }
    }
    #[doc = "Bits 0:3 - Controls I2S output clock to CLK_OUT_out1"]
    #[inline(always)]
    pub fn pin_ctrl_clk1(&mut self) -> PIN_CTRL_CLK1_W {
        PIN_CTRL_CLK1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock output configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_ctrl](index.html) module"]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin_ctrl::R](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin_ctrl::W](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0x07ff"]
impl crate::Resettable for PIN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
