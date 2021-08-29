#[doc = "Register `PERI_BACKUP_CONFIG` reader"]
pub struct R(crate::R<PERI_BACKUP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_BACKUP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_BACKUP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_BACKUP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_BACKUP_CONFIG` writer"]
pub struct W(crate::W<PERI_BACKUP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_BACKUP_CONFIG_SPEC>;
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
impl From<crate::W<PERI_BACKUP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_BACKUP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_ENA` reader - "]
pub struct PERI_BACKUP_ENA_R(crate::FieldReader<bool, bool>);
impl PERI_BACKUP_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_ENA` writer - "]
pub struct PERI_BACKUP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_ENA_W<'a> {
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
#[doc = "Field `PERI_BACKUP_TO_MEM` reader - "]
pub struct PERI_BACKUP_TO_MEM_R(crate::FieldReader<bool, bool>);
impl PERI_BACKUP_TO_MEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERI_BACKUP_TO_MEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_TO_MEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_TO_MEM` writer - "]
pub struct PERI_BACKUP_TO_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_TO_MEM_W<'a> {
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
#[doc = "Field `PERI_BACKUP_START` writer - "]
pub struct PERI_BACKUP_START_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PERI_BACKUP_SIZE` reader - "]
pub struct PERI_BACKUP_SIZE_R(crate::FieldReader<u16, u16>);
impl PERI_BACKUP_SIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        PERI_BACKUP_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_SIZE` writer - "]
pub struct PERI_BACKUP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | ((value as u32 & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Field `PERI_BACKUP_TOUT_THRES` reader - "]
pub struct PERI_BACKUP_TOUT_THRES_R(crate::FieldReader<u16, u16>);
impl PERI_BACKUP_TOUT_THRES_R {
    pub(crate) fn new(bits: u16) -> Self {
        PERI_BACKUP_TOUT_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_TOUT_THRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_TOUT_THRES` writer - "]
pub struct PERI_BACKUP_TOUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_TOUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 9)) | ((value as u32 & 0x03ff) << 9);
        self.w
    }
}
#[doc = "Field `PERI_BACKUP_BURST_LIMIT` reader - "]
pub struct PERI_BACKUP_BURST_LIMIT_R(crate::FieldReader<u8, u8>);
impl PERI_BACKUP_BURST_LIMIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERI_BACKUP_BURST_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_BURST_LIMIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERI_BACKUP_BURST_LIMIT` writer - "]
pub struct PERI_BACKUP_BURST_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERI_BACKUP_BURST_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Field `PERI_BACKUP_FLOW_ERR` reader - "]
pub struct PERI_BACKUP_FLOW_ERR_R(crate::FieldReader<u8, u8>);
impl PERI_BACKUP_FLOW_ERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERI_BACKUP_FLOW_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERI_BACKUP_FLOW_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn peri_backup_ena(&self) -> PERI_BACKUP_ENA_R {
        PERI_BACKUP_ENA_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn peri_backup_to_mem(&self) -> PERI_BACKUP_TO_MEM_R {
        PERI_BACKUP_TO_MEM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn peri_backup_size(&self) -> PERI_BACKUP_SIZE_R {
        PERI_BACKUP_SIZE_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn peri_backup_tout_thres(&self) -> PERI_BACKUP_TOUT_THRES_R {
        PERI_BACKUP_TOUT_THRES_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn peri_backup_burst_limit(&self) -> PERI_BACKUP_BURST_LIMIT_R {
        PERI_BACKUP_BURST_LIMIT_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn peri_backup_flow_err(&self) -> PERI_BACKUP_FLOW_ERR_R {
        PERI_BACKUP_FLOW_ERR_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn peri_backup_ena(&mut self) -> PERI_BACKUP_ENA_W {
        PERI_BACKUP_ENA_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn peri_backup_to_mem(&mut self) -> PERI_BACKUP_TO_MEM_W {
        PERI_BACKUP_TO_MEM_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn peri_backup_start(&mut self) -> PERI_BACKUP_START_W {
        PERI_BACKUP_START_W { w: self }
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn peri_backup_size(&mut self) -> PERI_BACKUP_SIZE_W {
        PERI_BACKUP_SIZE_W { w: self }
    }
    #[doc = "Bits 9:18"]
    #[inline(always)]
    pub fn peri_backup_tout_thres(&mut self) -> PERI_BACKUP_TOUT_THRES_W {
        PERI_BACKUP_TOUT_THRES_W { w: self }
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn peri_backup_burst_limit(&mut self) -> PERI_BACKUP_BURST_LIMIT_W {
        PERI_BACKUP_BURST_LIMIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCON_PERI_BACKUP_CONFIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_backup_config](index.html) module"]
pub struct PERI_BACKUP_CONFIG_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_backup_config::R](R) reader structure"]
impl crate::Readable for PERI_BACKUP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_backup_config::W](W) writer structure"]
impl crate::Writable for PERI_BACKUP_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERI_BACKUP_CONFIG to value 0"]
impl crate::Resettable for PERI_BACKUP_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
