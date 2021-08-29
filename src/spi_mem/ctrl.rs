#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREAD_QIO` reader - "]
pub struct FREAD_QIO_R(crate::FieldReader<bool, bool>);
impl FREAD_QIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_QIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_QIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_QIO` writer - "]
pub struct FREAD_QIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_QIO_W<'a> {
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
#[doc = "Field `FREAD_DIO` reader - "]
pub struct FREAD_DIO_R(crate::FieldReader<bool, bool>);
impl FREAD_DIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_DIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_DIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_DIO` writer - "]
pub struct FREAD_DIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_DIO_W<'a> {
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
#[doc = "Field `WRSR_2B` reader - "]
pub struct WRSR_2B_R(crate::FieldReader<bool, bool>);
impl WRSR_2B_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRSR_2B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRSR_2B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRSR_2B` writer - "]
pub struct WRSR_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSR_2B_W<'a> {
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
#[doc = "Field `WP_REG` reader - "]
pub struct WP_REG_R(crate::FieldReader<bool, bool>);
impl WP_REG_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP_REG` writer - "]
pub struct WP_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_REG_W<'a> {
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
#[doc = "Field `FREAD_QUAD` reader - "]
pub struct FREAD_QUAD_R(crate::FieldReader<bool, bool>);
impl FREAD_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_QUAD` writer - "]
pub struct FREAD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_QUAD_W<'a> {
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
#[doc = "Field `D_POL` reader - "]
pub struct D_POL_R(crate::FieldReader<bool, bool>);
impl D_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        D_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_POL` writer - "]
pub struct D_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_POL_W<'a> {
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
#[doc = "Field `Q_POL` reader - "]
pub struct Q_POL_R(crate::FieldReader<bool, bool>);
impl Q_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Q_POL` writer - "]
pub struct Q_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_POL_W<'a> {
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
#[doc = "Field `RESANDRES` reader - "]
pub struct RESANDRES_R(crate::FieldReader<bool, bool>);
impl RESANDRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESANDRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESANDRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESANDRES` writer - "]
pub struct RESANDRES_W<'a> {
    w: &'a mut W,
}
impl<'a> RESANDRES_W<'a> {
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
#[doc = "Field `FREAD_DUAL` reader - "]
pub struct FREAD_DUAL_R(crate::FieldReader<bool, bool>);
impl FREAD_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_DUAL` writer - "]
pub struct FREAD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_DUAL_W<'a> {
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
#[doc = "Field `FASTRD_MODE` reader - "]
pub struct FASTRD_MODE_R(crate::FieldReader<bool, bool>);
impl FASTRD_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTRD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTRD_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTRD_MODE` writer - "]
pub struct FASTRD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTRD_MODE_W<'a> {
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
#[doc = "Field `TX_CRC_EN` reader - "]
pub struct TX_CRC_EN_R(crate::FieldReader<bool, bool>);
impl TX_CRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_CRC_EN` writer - "]
pub struct TX_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CRC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `FCS_CRC_EN` reader - "]
pub struct FCS_CRC_EN_R(crate::FieldReader<bool, bool>);
impl FCS_CRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCS_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCS_CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCS_CRC_EN` writer - "]
pub struct FCS_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_CRC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FCMD_QUAD` reader - "]
pub struct FCMD_QUAD_R(crate::FieldReader<bool, bool>);
impl FCMD_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCMD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMD_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMD_QUAD` writer - "]
pub struct FCMD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_QUAD_W<'a> {
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
#[doc = "Field `FCMD_DUAL` reader - "]
pub struct FCMD_DUAL_R(crate::FieldReader<bool, bool>);
impl FCMD_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCMD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMD_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMD_DUAL` writer - "]
pub struct FCMD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_DUAL_W<'a> {
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
#[doc = "Field `FDUMMY_OUT` reader - "]
pub struct FDUMMY_OUT_R(crate::FieldReader<bool, bool>);
impl FDUMMY_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDUMMY_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDUMMY_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDUMMY_OUT` writer - "]
pub struct FDUMMY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FDUMMY_OUT_W<'a> {
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
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fread_qio(&self) -> FREAD_QIO_R {
        FREAD_QIO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fread_dio(&self) -> FREAD_DIO_R {
        FREAD_DIO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wrsr_2b(&self) -> WRSR_2B_R {
        WRSR_2B_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wp_reg(&self) -> WP_REG_R {
        WP_REG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn resandres(&self) -> RESANDRES_R {
        RESANDRES_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_crc_en(&self) -> TX_CRC_EN_R {
        TX_CRC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fcs_crc_en(&self) -> FCS_CRC_EN_R {
        FCS_CRC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdummy_out(&self) -> FDUMMY_OUT_R {
        FDUMMY_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fread_qio(&mut self) -> FREAD_QIO_W {
        FREAD_QIO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fread_dio(&mut self) -> FREAD_DIO_W {
        FREAD_DIO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wrsr_2b(&mut self) -> WRSR_2B_W {
        WRSR_2B_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wp_reg(&mut self) -> WP_REG_W {
        WP_REG_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W {
        FREAD_QUAD_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W {
        D_POL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W {
        Q_POL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn resandres(&mut self) -> RESANDRES_W {
        RESANDRES_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W {
        FREAD_DUAL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W {
        FASTRD_MODE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tx_crc_en(&mut self) -> TX_CRC_EN_W {
        TX_CRC_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fcs_crc_en(&mut self) -> FCS_CRC_EN_W {
        FCS_CRC_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W {
        FCMD_QUAD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W {
        FCMD_DUAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fdummy_out(&mut self) -> FDUMMY_OUT_W {
        FDUMMY_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
