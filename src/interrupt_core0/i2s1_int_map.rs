#[doc = "Register `I2S1_INT_MAP` reader"]
pub struct R(crate::R<I2S1_INT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S1_INT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S1_INT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S1_INT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S1_INT_MAP` writer"]
pub struct W(crate::W<I2S1_INT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S1_INT_MAP_SPEC>;
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
impl From<crate::W<I2S1_INT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S1_INT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S1_INT_MAP` reader - "]
pub struct I2S1_INT_MAP_R(crate::FieldReader<u8, u8>);
impl I2S1_INT_MAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2S1_INT_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S1_INT_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S1_INT_MAP` writer - "]
pub struct I2S1_INT_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_INT_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2s1_int_map(&self) -> I2S1_INT_MAP_R {
        I2S1_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn i2s1_int_map(&mut self) -> I2S1_INT_MAP_W {
        I2S1_INT_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INTERRUPT_CORE0_I2S1_INT_MAP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s1_int_map](index.html) module"]
pub struct I2S1_INT_MAP_SPEC;
impl crate::RegisterSpec for I2S1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s1_int_map::R](R) reader structure"]
impl crate::Readable for I2S1_INT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s1_int_map::W](W) writer structure"]
impl crate::Writable for I2S1_INT_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S1_INT_MAP to value 0"]
impl crate::Resettable for I2S1_INT_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
