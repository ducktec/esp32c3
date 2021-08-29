#[doc = "Register `CACHE_MMU_ACCESS_1` reader"]
pub struct R(crate::R<CACHE_MMU_ACCESS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_ACCESS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_ACCESS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_ACCESS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MMU_ACCESS_1` writer"]
pub struct W(crate::W<CACHE_MMU_ACCESS_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MMU_ACCESS_1_SPEC>;
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
impl From<crate::W<CACHE_MMU_ACCESS_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MMU_ACCESS_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_MMU_WR_ACS` reader - "]
pub struct PRO_MMU_WR_ACS_R(crate::FieldReader<bool, bool>);
impl PRO_MMU_WR_ACS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRO_MMU_WR_ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_MMU_WR_ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_MMU_WR_ACS` writer - "]
pub struct PRO_MMU_WR_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_MMU_WR_ACS_W<'a> {
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
#[doc = "Field `PRO_MMU_RD_ACS` reader - "]
pub struct PRO_MMU_RD_ACS_R(crate::FieldReader<bool, bool>);
impl PRO_MMU_RD_ACS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRO_MMU_RD_ACS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_MMU_RD_ACS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO_MMU_RD_ACS` writer - "]
pub struct PRO_MMU_RD_ACS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_MMU_RD_ACS_W<'a> {
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
    pub fn pro_mmu_wr_acs(&self) -> PRO_MMU_WR_ACS_R {
        PRO_MMU_WR_ACS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_mmu_rd_acs(&self) -> PRO_MMU_RD_ACS_R {
        PRO_MMU_RD_ACS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pro_mmu_wr_acs(&mut self) -> PRO_MMU_WR_ACS_W {
        PRO_MMU_WR_ACS_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_mmu_rd_acs(&mut self) -> PRO_MMU_RD_ACS_W {
        PRO_MMU_RD_ACS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_CACHE_MMU_ACCESS_1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_access_1](index.html) module"]
pub struct CACHE_MMU_ACCESS_1_SPEC;
impl crate::RegisterSpec for CACHE_MMU_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_access_1::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_ACCESS_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mmu_access_1::W](W) writer structure"]
impl crate::Writable for CACHE_MMU_ACCESS_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_MMU_ACCESS_1 to value 0"]
impl crate::Resettable for CACHE_MMU_ACCESS_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
