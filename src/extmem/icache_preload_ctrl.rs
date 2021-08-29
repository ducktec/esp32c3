#[doc = "Register `ICACHE_PRELOAD_CTRL` reader"]
pub struct R(crate::R<ICACHE_PRELOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_PRELOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_PRELOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_PRELOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_PRELOAD_CTRL` writer"]
pub struct W(crate::W<ICACHE_PRELOAD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_PRELOAD_CTRL_SPEC>;
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
impl From<crate::W<ICACHE_PRELOAD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_PRELOAD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_PRELOAD_ORDER` reader - "]
pub struct ICACHE_PRELOAD_ORDER_R(crate::FieldReader<bool, bool>);
impl ICACHE_PRELOAD_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_PRELOAD_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_PRELOAD_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_PRELOAD_ORDER` writer - "]
pub struct ICACHE_PRELOAD_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_PRELOAD_ORDER_W<'a> {
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
#[doc = "Field `ICACHE_PRELOAD_DONE` reader - "]
pub struct ICACHE_PRELOAD_DONE_R(crate::FieldReader<bool, bool>);
impl ICACHE_PRELOAD_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_PRELOAD_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_PRELOAD_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_PRELOAD_ENA` reader - "]
pub struct ICACHE_PRELOAD_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_PRELOAD_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_PRELOAD_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_PRELOAD_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_PRELOAD_ENA` writer - "]
pub struct ICACHE_PRELOAD_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_PRELOAD_ENA_W<'a> {
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
    pub fn icache_preload_order(&self) -> ICACHE_PRELOAD_ORDER_R {
        ICACHE_PRELOAD_ORDER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icache_preload_done(&self) -> ICACHE_PRELOAD_DONE_R {
        ICACHE_PRELOAD_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_preload_ena(&self) -> ICACHE_PRELOAD_ENA_R {
        ICACHE_PRELOAD_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icache_preload_order(&mut self) -> ICACHE_PRELOAD_ORDER_W {
        ICACHE_PRELOAD_ORDER_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_preload_ena(&mut self) -> ICACHE_PRELOAD_ENA_W {
        ICACHE_PRELOAD_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_ICACHE_PRELOAD_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_preload_ctrl](index.html) module"]
pub struct ICACHE_PRELOAD_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_PRELOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_preload_ctrl::R](R) reader structure"]
impl crate::Readable for ICACHE_PRELOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_preload_ctrl::W](W) writer structure"]
impl crate::Writable for ICACHE_PRELOAD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_PRELOAD_CTRL to value 0"]
impl crate::Resettable for ICACHE_PRELOAD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
