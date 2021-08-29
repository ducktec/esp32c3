#[doc = "Register `LOG_MEM_FULL_FLAG` reader"]
pub struct R(crate::R<LOG_MEM_FULL_FLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MEM_FULL_FLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MEM_FULL_FLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MEM_FULL_FLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_MEM_FULL_FLAG` writer"]
pub struct W(crate::W<LOG_MEM_FULL_FLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MEM_FULL_FLAG_SPEC>;
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
impl From<crate::W<LOG_MEM_FULL_FLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MEM_FULL_FLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_LOG_MEM_FULL_FLAG` reader - "]
pub struct CLR_LOG_MEM_FULL_FLAG_R(crate::FieldReader<bool, bool>);
impl CLR_LOG_MEM_FULL_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLR_LOG_MEM_FULL_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_LOG_MEM_FULL_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLR_LOG_MEM_FULL_FLAG` writer - "]
pub struct CLR_LOG_MEM_FULL_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_LOG_MEM_FULL_FLAG_W<'a> {
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
#[doc = "Field `LOG_MEM_FULL_FLAG` reader - "]
pub struct LOG_MEM_FULL_FLAG_R(crate::FieldReader<bool, bool>);
impl LOG_MEM_FULL_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOG_MEM_FULL_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOG_MEM_FULL_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clr_log_mem_full_flag(&self) -> CLR_LOG_MEM_FULL_FLAG_R {
        CLR_LOG_MEM_FULL_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn log_mem_full_flag(&self) -> LOG_MEM_FULL_FLAG_R {
        LOG_MEM_FULL_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clr_log_mem_full_flag(&mut self) -> CLR_LOG_MEM_FULL_FLAG_W {
        CLR_LOG_MEM_FULL_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_mem_full_flag](index.html) module"]
pub struct LOG_MEM_FULL_FLAG_SPEC;
impl crate::RegisterSpec for LOG_MEM_FULL_FLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_mem_full_flag::R](R) reader structure"]
impl crate::Readable for LOG_MEM_FULL_FLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_mem_full_flag::W](W) writer structure"]
impl crate::Writable for LOG_MEM_FULL_FLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOG_MEM_FULL_FLAG to value 0"]
impl crate::Resettable for LOG_MEM_FULL_FLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
