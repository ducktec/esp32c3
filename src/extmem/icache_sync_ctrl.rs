#[doc = "Register `ICACHE_SYNC_CTRL` reader"]
pub struct R(crate::R<ICACHE_SYNC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_SYNC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_SYNC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_SYNC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_SYNC_CTRL` writer"]
pub struct W(crate::W<ICACHE_SYNC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_SYNC_CTRL_SPEC>;
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
impl From<crate::W<ICACHE_SYNC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_SYNC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_SYNC_DONE` reader - "]
pub struct ICACHE_SYNC_DONE_R(crate::FieldReader<bool, bool>);
impl ICACHE_SYNC_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_SYNC_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_SYNC_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_INVALIDATE_ENA` reader - "]
pub struct ICACHE_INVALIDATE_ENA_R(crate::FieldReader<bool, bool>);
impl ICACHE_INVALIDATE_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICACHE_INVALIDATE_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACHE_INVALIDATE_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACHE_INVALIDATE_ENA` writer - "]
pub struct ICACHE_INVALIDATE_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHE_INVALIDATE_ENA_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icache_sync_done(&self) -> ICACHE_SYNC_DONE_R {
        ICACHE_SYNC_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_invalidate_ena(&self) -> ICACHE_INVALIDATE_ENA_R {
        ICACHE_INVALIDATE_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icache_invalidate_ena(&mut self) -> ICACHE_INVALIDATE_ENA_W {
        ICACHE_INVALIDATE_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_ICACHE_SYNC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_sync_ctrl](index.html) module"]
pub struct ICACHE_SYNC_CTRL_SPEC;
impl crate::RegisterSpec for ICACHE_SYNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_sync_ctrl::R](R) reader structure"]
impl crate::Readable for ICACHE_SYNC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_sync_ctrl::W](W) writer structure"]
impl crate::Writable for ICACHE_SYNC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_SYNC_CTRL to value 0"]
impl crate::Resettable for ICACHE_SYNC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
