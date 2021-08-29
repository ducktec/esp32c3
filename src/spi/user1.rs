#[doc = "Register `USER1` reader"]
pub struct R(crate::R<USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER1` writer"]
pub struct W(crate::W<USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER1_SPEC>;
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
impl From<crate::W<USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_ADDR_BITLEN` reader - "]
pub struct USR_ADDR_BITLEN_R(crate::FieldReader<u8, u8>);
impl USR_ADDR_BITLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        USR_ADDR_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_ADDR_BITLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_ADDR_BITLEN` writer - "]
pub struct USR_ADDR_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
#[doc = "Field `CS_HOLD_TIME` reader - "]
pub struct CS_HOLD_TIME_R(crate::FieldReader<u8, u8>);
impl CS_HOLD_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_HOLD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_TIME` writer - "]
pub struct CS_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | ((value as u32 & 0x1f) << 22);
        self.w
    }
}
#[doc = "Field `CS_SETUP_TIME` reader - "]
pub struct CS_SETUP_TIME_R(crate::FieldReader<u8, u8>);
impl CS_SETUP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_SETUP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SETUP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SETUP_TIME` writer - "]
pub struct CS_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `MST_WFULL_ERR_END_EN` reader - "]
pub struct MST_WFULL_ERR_END_EN_R(crate::FieldReader<bool, bool>);
impl MST_WFULL_ERR_END_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MST_WFULL_ERR_END_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_WFULL_ERR_END_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_WFULL_ERR_END_EN` writer - "]
pub struct MST_WFULL_ERR_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_WFULL_ERR_END_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - "]
pub struct USR_DUMMY_CYCLELEN_R(crate::FieldReader<u8, u8>);
impl USR_DUMMY_CYCLELEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        USR_DUMMY_CYCLELEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_CYCLELEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - "]
pub struct USR_DUMMY_CYCLELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_CYCLELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&self) -> MST_WFULL_ERR_END_EN_R {
        MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W {
        USR_ADDR_BITLEN_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W {
        CS_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W {
        CS_SETUP_TIME_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&mut self) -> MST_WFULL_ERR_END_EN_W {
        MST_WFULL_ERR_END_EN_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W {
        USR_DUMMY_CYCLELEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_USER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1](index.html) module"]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user1::R](R) reader structure"]
impl crate::Readable for USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user1::W](W) writer structure"]
impl crate::Writable for USER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER1 to value 0"]
impl crate::Resettable for USER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
