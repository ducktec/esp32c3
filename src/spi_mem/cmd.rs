#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_READ` reader - "]
pub struct FLASH_READ_R(crate::FieldReader<bool, bool>);
impl FLASH_READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_READ` writer - "]
pub struct FLASH_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_READ_W<'a> {
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
#[doc = "Field `FLASH_WREN` reader - "]
pub struct FLASH_WREN_R(crate::FieldReader<bool, bool>);
impl FLASH_WREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_WREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_WREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_WREN` writer - "]
pub struct FLASH_WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_WREN_W<'a> {
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
#[doc = "Field `FLASH_WRDI` reader - "]
pub struct FLASH_WRDI_R(crate::FieldReader<bool, bool>);
impl FLASH_WRDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_WRDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_WRDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_WRDI` writer - "]
pub struct FLASH_WRDI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_WRDI_W<'a> {
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
#[doc = "Field `FLASH_RDID` reader - "]
pub struct FLASH_RDID_R(crate::FieldReader<bool, bool>);
impl FLASH_RDID_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RDID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_RDID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RDID` writer - "]
pub struct FLASH_RDID_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RDID_W<'a> {
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
#[doc = "Field `FLASH_RDSR` reader - "]
pub struct FLASH_RDSR_R(crate::FieldReader<bool, bool>);
impl FLASH_RDSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RDSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_RDSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RDSR` writer - "]
pub struct FLASH_RDSR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RDSR_W<'a> {
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
#[doc = "Field `FLASH_WRSR` reader - "]
pub struct FLASH_WRSR_R(crate::FieldReader<bool, bool>);
impl FLASH_WRSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_WRSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_WRSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_WRSR` writer - "]
pub struct FLASH_WRSR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_WRSR_W<'a> {
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
#[doc = "Field `FLASH_PP` reader - "]
pub struct FLASH_PP_R(crate::FieldReader<bool, bool>);
impl FLASH_PP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PP` writer - "]
pub struct FLASH_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PP_W<'a> {
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
#[doc = "Field `FLASH_SE` reader - "]
pub struct FLASH_SE_R(crate::FieldReader<bool, bool>);
impl FLASH_SE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_SE` writer - "]
pub struct FLASH_SE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_SE_W<'a> {
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
#[doc = "Field `FLASH_BE` reader - "]
pub struct FLASH_BE_R(crate::FieldReader<bool, bool>);
impl FLASH_BE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_BE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_BE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_BE` writer - "]
pub struct FLASH_BE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_BE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `FLASH_CE` reader - "]
pub struct FLASH_CE_R(crate::FieldReader<bool, bool>);
impl FLASH_CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_CE` writer - "]
pub struct FLASH_CE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `FLASH_DP` reader - "]
pub struct FLASH_DP_R(crate::FieldReader<bool, bool>);
impl FLASH_DP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_DP` writer - "]
pub struct FLASH_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `FLASH_RES` reader - "]
pub struct FLASH_RES_R(crate::FieldReader<bool, bool>);
impl FLASH_RES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RES` writer - "]
pub struct FLASH_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `FLASH_HPM` reader - "]
pub struct FLASH_HPM_R(crate::FieldReader<bool, bool>);
impl FLASH_HPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_HPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_HPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_HPM` writer - "]
pub struct FLASH_HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_HPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `USR` reader - "]
pub struct USR_R(crate::FieldReader<bool, bool>);
impl USR_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR` writer - "]
pub struct USR_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `FLASH_PE` reader - "]
pub struct FLASH_PE_R(crate::FieldReader<bool, bool>);
impl FLASH_PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PE` writer - "]
pub struct FLASH_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `SLV_ST` reader - "]
pub struct SLV_ST_R(crate::FieldReader<u8, u8>);
impl SLV_ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLV_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_ST` reader - "]
pub struct MST_ST_R(crate::FieldReader<u8, u8>);
impl MST_ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        MST_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn flash_read(&self) -> FLASH_READ_R {
        FLASH_READ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn flash_wren(&self) -> FLASH_WREN_R {
        FLASH_WREN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn flash_wrdi(&self) -> FLASH_WRDI_R {
        FLASH_WRDI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn flash_rdid(&self) -> FLASH_RDID_R {
        FLASH_RDID_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn flash_rdsr(&self) -> FLASH_RDSR_R {
        FLASH_RDSR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn flash_wrsr(&self) -> FLASH_WRSR_R {
        FLASH_WRSR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn flash_pp(&self) -> FLASH_PP_R {
        FLASH_PP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn flash_se(&self) -> FLASH_SE_R {
        FLASH_SE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn flash_be(&self) -> FLASH_BE_R {
        FLASH_BE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn flash_ce(&self) -> FLASH_CE_R {
        FLASH_CE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn flash_dp(&self) -> FLASH_DP_R {
        FLASH_DP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flash_res(&self) -> FLASH_RES_R {
        FLASH_RES_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flash_hpm(&self) -> FLASH_HPM_R {
        FLASH_HPM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flash_pe(&self) -> FLASH_PE_R {
        FLASH_PE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn slv_st(&self) -> SLV_ST_R {
        SLV_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mst_st(&self) -> MST_ST_R {
        MST_ST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn flash_read(&mut self) -> FLASH_READ_W {
        FLASH_READ_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn flash_wren(&mut self) -> FLASH_WREN_W {
        FLASH_WREN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn flash_wrdi(&mut self) -> FLASH_WRDI_W {
        FLASH_WRDI_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn flash_rdid(&mut self) -> FLASH_RDID_W {
        FLASH_RDID_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn flash_rdsr(&mut self) -> FLASH_RDSR_W {
        FLASH_RDSR_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn flash_wrsr(&mut self) -> FLASH_WRSR_W {
        FLASH_WRSR_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn flash_pp(&mut self) -> FLASH_PP_W {
        FLASH_PP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn flash_se(&mut self) -> FLASH_SE_W {
        FLASH_SE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn flash_be(&mut self) -> FLASH_BE_W {
        FLASH_BE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn flash_ce(&mut self) -> FLASH_CE_W {
        FLASH_CE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn flash_dp(&mut self) -> FLASH_DP_W {
        FLASH_DP_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flash_res(&mut self) -> FLASH_RES_W {
        FLASH_RES_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flash_hpm(&mut self) -> FLASH_HPM_W {
        FLASH_HPM_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usr(&mut self) -> USR_W {
        USR_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flash_pe(&mut self) -> FLASH_PE_W {
        FLASH_PE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_CMD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
