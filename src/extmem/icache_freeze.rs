#[doc = "Register `ICACHE_FREEZE` reader"]
pub struct R(crate::R<ICACHE_FREEZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_FREEZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_FREEZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_FREEZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_FREEZE` writer"]
pub struct W(crate::W<ICACHE_FREEZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_FREEZE_SPEC>;
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
impl From<crate::W<ICACHE_FREEZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_FREEZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_FREEZE_DONE` reader - "]
pub struct ICACHE_FREEZE_DONE_R(crate::FieldReader<bool, bool>);
impl ICACHE_FREEZE_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_FREEZE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_FREEZE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_FREEZE_MODE` reader - "]
pub struct ICACHE_FREEZE_MODE_R(crate::FieldReader<bool, bool>);
impl ICACHE_FREEZE_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_FREEZE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_FREEZE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_FREEZE_MODE` writer - "]
pub struct ICACHE_FREEZE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_FREEZE_MODE_W<'a> {
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
#[doc = "Field `ICACHE_FREEZE_ENA` reader - "]
pub struct ICACHE_FREEZE_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_FREEZE_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_FREEZE_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_FREEZE_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_FREEZE_ENA` writer - "]
pub struct ICACHE_FREEZE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_FREEZE_ENA_W<'a> {
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
    pub fn icache_freeze_done(&self) -> ICACHE_FREEZE_DONE_R {
        ICACHE_FREEZE_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icache_freeze_mode(&self) -> ICACHE_FREEZE_MODE_R {
        ICACHE_FREEZE_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_freeze_ena(&self) -> ICACHE_FREEZE_ENA_R {
        ICACHE_FREEZE_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icache_freeze_mode(&mut self) -> ICACHE_FREEZE_MODE_W {
        ICACHE_FREEZE_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_freeze_ena(&mut self) -> ICACHE_FREEZE_ENA_W {
        ICACHE_FREEZE_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_ICACHE_FREEZE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_freeze](index.html) module"]
pub struct ICACHE_FREEZE_SPEC;
impl crate::RegisterSpec for ICACHE_FREEZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_freeze::R](R) reader structure"]
impl crate::Readable for ICACHE_FREEZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_freeze::W](W) writer structure"]
impl crate::Writable for ICACHE_FREEZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_FREEZE to value 0"]
impl crate::Resettable for ICACHE_FREEZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}