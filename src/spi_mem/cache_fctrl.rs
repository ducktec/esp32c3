#[doc = "Register `CACHE_FCTRL` reader"]
pub struct R(crate::R<CACHE_FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_FCTRL` writer"]
pub struct W(crate::W<CACHE_FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_FCTRL_SPEC>;
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
impl From<crate::W<CACHE_FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADDR_QUAD` reader - "]
pub struct FADDR_QUAD_R(crate::FieldReader<bool, bool>);
impl FADDR_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_QUAD` writer - "]
pub struct FADDR_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `FDOUT_QUAD` reader - "]
pub struct FDOUT_QUAD_R(crate::FieldReader<bool, bool>);
impl FDOUT_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDOUT_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDOUT_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDOUT_QUAD` writer - "]
pub struct FDOUT_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOUT_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FDIN_QUAD` reader - "]
pub struct FDIN_QUAD_R(crate::FieldReader<bool, bool>);
impl FDIN_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDIN_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDIN_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIN_QUAD` writer - "]
pub struct FDIN_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIN_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FADDR_DUAL` reader - "]
pub struct FADDR_DUAL_R(crate::FieldReader<bool, bool>);
impl FADDR_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_DUAL` writer - "]
pub struct FADDR_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `FDOUT_DUAL` reader - "]
pub struct FDOUT_DUAL_R(crate::FieldReader<bool, bool>);
impl FDOUT_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDOUT_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDOUT_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDOUT_DUAL` writer - "]
pub struct FDOUT_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOUT_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FDIN_DUAL` reader - "]
pub struct FDIN_DUAL_R(crate::FieldReader<bool, bool>);
impl FDIN_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDIN_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDIN_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDIN_DUAL` writer - "]
pub struct FDIN_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIN_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - "]
pub struct CACHE_FLASH_USR_CMD_R(crate::FieldReader<bool, bool>);
impl CACHE_FLASH_USR_CMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_FLASH_USR_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_FLASH_USR_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - "]
pub struct CACHE_FLASH_USR_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_FLASH_USR_CMD_W<'a> {
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
#[doc = "Field `CACHE_USR_ADDR_4BYTE` reader - "]
pub struct CACHE_USR_ADDR_4BYTE_R(crate::FieldReader<bool, bool>);
impl CACHE_USR_ADDR_4BYTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_USR_ADDR_4BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_USR_ADDR_4BYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_USR_ADDR_4BYTE` writer - "]
pub struct CACHE_USR_ADDR_4BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_USR_ADDR_4BYTE_W<'a> {
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
#[doc = "Field `CACHE_REQ_EN` reader - "]
pub struct CACHE_REQ_EN_R(crate::FieldReader<bool, bool>);
impl CACHE_REQ_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CACHE_REQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHE_REQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CACHE_REQ_EN` writer - "]
pub struct CACHE_REQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHE_REQ_EN_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FDOUT_QUAD_R {
        FDOUT_QUAD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FDIN_QUAD_R {
        FDIN_QUAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FDOUT_DUAL_R {
        FDOUT_DUAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FDIN_DUAL_R {
        FDIN_DUAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&self) -> CACHE_USR_ADDR_4BYTE_R {
        CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W {
        FADDR_QUAD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fdout_quad(&mut self) -> FDOUT_QUAD_W {
        FDOUT_QUAD_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn fdin_quad(&mut self) -> FDIN_QUAD_W {
        FDIN_QUAD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W {
        FADDR_DUAL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fdout_dual(&mut self) -> FDOUT_DUAL_W {
        FDOUT_DUAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdin_dual(&mut self) -> FDIN_DUAL_W {
        FDIN_DUAL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W {
        CACHE_FLASH_USR_CMD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cache_usr_addr_4byte(&mut self) -> CACHE_USR_ADDR_4BYTE_W {
        CACHE_USR_ADDR_4BYTE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W {
        CACHE_REQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_CACHE_FCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_fctrl](index.html) module"]
pub struct CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_fctrl::R](R) reader structure"]
impl crate::Readable for CACHE_FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_fctrl::W](W) writer structure"]
impl crate::Writable for CACHE_FCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0"]
impl crate::Resettable for CACHE_FCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
