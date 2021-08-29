#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_COMMAND` reader - "]
pub struct USR_COMMAND_R(crate::FieldReader<bool, bool>);
impl USR_COMMAND_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_COMMAND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_COMMAND` writer - "]
pub struct USR_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_COMMAND_W<'a> {
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
#[doc = "Field `USR_ADDR` reader - "]
pub struct USR_ADDR_R(crate::FieldReader<bool, bool>);
impl USR_ADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_ADDR` writer - "]
pub struct USR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_ADDR_W<'a> {
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
#[doc = "Field `USR_DUMMY` reader - "]
pub struct USR_DUMMY_R(crate::FieldReader<bool, bool>);
impl USR_DUMMY_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_DUMMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY` writer - "]
pub struct USR_DUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_W<'a> {
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
#[doc = "Field `USR_MISO` reader - "]
pub struct USR_MISO_R(crate::FieldReader<bool, bool>);
impl USR_MISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_MISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MISO` writer - "]
pub struct USR_MISO_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `USR_MOSI` reader - "]
pub struct USR_MOSI_R(crate::FieldReader<bool, bool>);
impl USR_MOSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_MOSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MOSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MOSI` writer - "]
pub struct USR_MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `USR_DUMMY_IDLE` reader - "]
pub struct USR_DUMMY_IDLE_R(crate::FieldReader<bool, bool>);
impl USR_DUMMY_IDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_DUMMY_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DUMMY_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_DUMMY_IDLE` writer - "]
pub struct USR_DUMMY_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_DUMMY_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `USR_MOSI_HIGHPART` reader - "]
pub struct USR_MOSI_HIGHPART_R(crate::FieldReader<bool, bool>);
impl USR_MOSI_HIGHPART_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_MOSI_HIGHPART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MOSI_HIGHPART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MOSI_HIGHPART` writer - "]
pub struct USR_MOSI_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MOSI_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `USR_MISO_HIGHPART` reader - "]
pub struct USR_MISO_HIGHPART_R(crate::FieldReader<bool, bool>);
impl USR_MISO_HIGHPART_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_MISO_HIGHPART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_MISO_HIGHPART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_MISO_HIGHPART` writer - "]
pub struct USR_MISO_HIGHPART_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_MISO_HIGHPART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `FWRITE_QIO` reader - "]
pub struct FWRITE_QIO_R(crate::FieldReader<bool, bool>);
impl FWRITE_QIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_QIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_QIO` writer - "]
pub struct FWRITE_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_QIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `FWRITE_DIO` reader - "]
pub struct FWRITE_DIO_R(crate::FieldReader<bool, bool>);
impl FWRITE_DIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_DIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_DIO` writer - "]
pub struct FWRITE_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_DIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `FWRITE_QUAD` reader - "]
pub struct FWRITE_QUAD_R(crate::FieldReader<bool, bool>);
impl FWRITE_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_QUAD` writer - "]
pub struct FWRITE_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_QUAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FWRITE_DUAL` reader - "]
pub struct FWRITE_DUAL_R(crate::FieldReader<bool, bool>);
impl FWRITE_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWRITE_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWRITE_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWRITE_DUAL` writer - "]
pub struct FWRITE_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FWRITE_DUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CK_OUT_EDGE` reader - "]
pub struct CK_OUT_EDGE_R(crate::FieldReader<bool, bool>);
impl CK_OUT_EDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CK_OUT_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK_OUT_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK_OUT_EDGE` writer - "]
pub struct CK_OUT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CK_OUT_EDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CS_SETUP` reader - "]
pub struct CS_SETUP_R(crate::FieldReader<bool, bool>);
impl CS_SETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SETUP` writer - "]
pub struct CS_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SETUP_W<'a> {
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
#[doc = "Field `CS_HOLD` reader - "]
pub struct CS_HOLD_R(crate::FieldReader<bool, bool>);
impl CS_HOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD` writer - "]
pub struct CS_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fwrite_qio(&self) -> FWRITE_QIO_R {
        FWRITE_QIO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fwrite_dio(&self) -> FWRITE_DIO_R {
        FWRITE_DIO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usr_command(&mut self) -> USR_COMMAND_W {
        USR_COMMAND_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn usr_addr(&mut self) -> USR_ADDR_W {
        USR_ADDR_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W {
        USR_DUMMY_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_miso(&mut self) -> USR_MISO_W {
        USR_MISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W {
        USR_MOSI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W {
        USR_DUMMY_IDLE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W {
        USR_MOSI_HIGHPART_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W {
        USR_MISO_HIGHPART_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fwrite_qio(&mut self) -> FWRITE_QIO_W {
        FWRITE_QIO_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fwrite_dio(&mut self) -> FWRITE_DIO_W {
        FWRITE_DIO_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W {
        FWRITE_QUAD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W {
        FWRITE_DUAL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W {
        CK_OUT_EDGE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cs_setup(&mut self) -> CS_SETUP_W {
        CS_SETUP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cs_hold(&mut self) -> CS_HOLD_W {
        CS_HOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_USER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER to value 0"]
impl crate::Resettable for USER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
