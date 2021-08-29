#[doc = "Register `EXT_XTL_CONF` reader"]
pub struct R(crate::R<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_XTL_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_XTL_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_XTL_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_XTL_CONF` writer"]
pub struct W(crate::W<EXT_XTL_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_XTL_CONF_SPEC>;
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
impl From<crate::W<EXT_XTL_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_XTL_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTL_EXT_CTR_EN` reader - "]
pub struct XTL_EXT_CTR_EN_R(crate::FieldReader<bool, bool>);
impl XTL_EXT_CTR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTL_EXT_CTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_EXT_CTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_EXT_CTR_EN` writer - "]
pub struct XTL_EXT_CTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_EN_W<'a> {
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
#[doc = "Field `XTL_EXT_CTR_LV` reader - "]
pub struct XTL_EXT_CTR_LV_R(crate::FieldReader<bool, bool>);
impl XTL_EXT_CTR_LV_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTL_EXT_CTR_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTL_EXT_CTR_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTL_EXT_CTR_LV` writer - "]
pub struct XTL_EXT_CTR_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> XTL_EXT_CTR_LV_W<'a> {
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
#[doc = "Field `XTAL32K_GPIO_SEL` reader - "]
pub struct XTAL32K_GPIO_SEL_R(crate::FieldReader<bool, bool>);
impl XTAL32K_GPIO_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_GPIO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_GPIO_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_GPIO_SEL` writer - "]
pub struct XTAL32K_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_GPIO_SEL_W<'a> {
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
#[doc = "Field `WDT_STATE` reader - "]
pub struct WDT_STATE_R(crate::FieldReader<u8, u8>);
impl WDT_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDT_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_XTAL_32K` reader - "]
pub struct DAC_XTAL_32K_R(crate::FieldReader<u8, u8>);
impl DAC_XTAL_32K_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAC_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC_XTAL_32K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC_XTAL_32K` writer - "]
pub struct DAC_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `XPD_XTAL_32K` reader - "]
pub struct XPD_XTAL_32K_R(crate::FieldReader<bool, bool>);
impl XPD_XTAL_32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        XPD_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_XTAL_32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_XTAL_32K` writer - "]
pub struct XPD_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_XTAL_32K_W<'a> {
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
#[doc = "Field `DRES_XTAL_32K` reader - "]
pub struct DRES_XTAL_32K_R(crate::FieldReader<u8, u8>);
impl DRES_XTAL_32K_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRES_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRES_XTAL_32K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRES_XTAL_32K` writer - "]
pub struct DRES_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `DGM_XTAL_32K` reader - "]
pub struct DGM_XTAL_32K_R(crate::FieldReader<u8, u8>);
impl DGM_XTAL_32K_R {
    pub(crate) fn new(bits: u8) -> Self {
        DGM_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DGM_XTAL_32K_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DGM_XTAL_32K` writer - "]
pub struct DGM_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DGM_XTAL_32K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `DBUF_XTAL_32K` reader - "]
pub struct DBUF_XTAL_32K_R(crate::FieldReader<bool, bool>);
impl DBUF_XTAL_32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBUF_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBUF_XTAL_32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBUF_XTAL_32K` writer - "]
pub struct DBUF_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUF_XTAL_32K_W<'a> {
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
#[doc = "Field `ENCKINIT_XTAL_32K` reader - "]
pub struct ENCKINIT_XTAL_32K_R(crate::FieldReader<bool, bool>);
impl ENCKINIT_XTAL_32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENCKINIT_XTAL_32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCKINIT_XTAL_32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCKINIT_XTAL_32K` writer - "]
pub struct ENCKINIT_XTAL_32K_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCKINIT_XTAL_32K_W<'a> {
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
#[doc = "Field `XTAL32K_XPD_FORCE` reader - "]
pub struct XTAL32K_XPD_FORCE_R(crate::FieldReader<bool, bool>);
impl XTAL32K_XPD_FORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_XPD_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_XPD_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_XPD_FORCE` writer - "]
pub struct XTAL32K_XPD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_XPD_FORCE_W<'a> {
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
#[doc = "Field `XTAL32K_AUTO_RETURN` reader - "]
pub struct XTAL32K_AUTO_RETURN_R(crate::FieldReader<bool, bool>);
impl XTAL32K_AUTO_RETURN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AUTO_RETURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AUTO_RETURN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_AUTO_RETURN` writer - "]
pub struct XTAL32K_AUTO_RETURN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AUTO_RETURN_W<'a> {
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
#[doc = "Field `XTAL32K_AUTO_RESTART` reader - "]
pub struct XTAL32K_AUTO_RESTART_R(crate::FieldReader<bool, bool>);
impl XTAL32K_AUTO_RESTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AUTO_RESTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AUTO_RESTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_AUTO_RESTART` writer - "]
pub struct XTAL32K_AUTO_RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AUTO_RESTART_W<'a> {
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
#[doc = "Field `XTAL32K_AUTO_BACKUP` reader - "]
pub struct XTAL32K_AUTO_BACKUP_R(crate::FieldReader<bool, bool>);
impl XTAL32K_AUTO_BACKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_AUTO_BACKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_AUTO_BACKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_AUTO_BACKUP` writer - "]
pub struct XTAL32K_AUTO_BACKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_AUTO_BACKUP_W<'a> {
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
#[doc = "Field `XTAL32K_EXT_CLK_FO` reader - "]
pub struct XTAL32K_EXT_CLK_FO_R(crate::FieldReader<bool, bool>);
impl XTAL32K_EXT_CLK_FO_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_EXT_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_EXT_CLK_FO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_EXT_CLK_FO` writer - "]
pub struct XTAL32K_EXT_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_EXT_CLK_FO_W<'a> {
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
#[doc = "Field `XTAL32K_WDT_RESET` reader - "]
pub struct XTAL32K_WDT_RESET_R(crate::FieldReader<bool, bool>);
impl XTAL32K_WDT_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_WDT_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_RESET` writer - "]
pub struct XTAL32K_WDT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_RESET_W<'a> {
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
#[doc = "Field `XTAL32K_WDT_CLK_FO` reader - "]
pub struct XTAL32K_WDT_CLK_FO_R(crate::FieldReader<bool, bool>);
impl XTAL32K_WDT_CLK_FO_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_WDT_CLK_FO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_CLK_FO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_CLK_FO` writer - "]
pub struct XTAL32K_WDT_CLK_FO_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_CLK_FO_W<'a> {
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
#[doc = "Field `XTAL32K_WDT_EN` reader - "]
pub struct XTAL32K_WDT_EN_R(crate::FieldReader<bool, bool>);
impl XTAL32K_WDT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32K_WDT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32K_WDT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32K_WDT_EN` writer - "]
pub struct XTAL32K_WDT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32K_WDT_EN_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&self) -> XTL_EXT_CTR_EN_R {
        XTL_EXT_CTR_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&self) -> XTL_EXT_CTR_LV_R {
        XTL_EXT_CTR_LV_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtal32k_gpio_sel(&self) -> XTAL32K_GPIO_SEL_R {
        XTAL32K_GPIO_SEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn wdt_state(&self) -> WDT_STATE_R {
        WDT_STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn dac_xtal_32k(&self) -> DAC_XTAL_32K_R {
        DAC_XTAL_32K_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&self) -> XPD_XTAL_32K_R {
        XPD_XTAL_32K_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn dres_xtal_32k(&self) -> DRES_XTAL_32K_R {
        DRES_XTAL_32K_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&self) -> DGM_XTAL_32K_R {
        DGM_XTAL_32K_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&self) -> DBUF_XTAL_32K_R {
        DBUF_XTAL_32K_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&self) -> ENCKINIT_XTAL_32K_R {
        ENCKINIT_XTAL_32K_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&self) -> XTAL32K_XPD_FORCE_R {
        XTAL32K_XPD_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn xtal32k_auto_return(&self) -> XTAL32K_AUTO_RETURN_R {
        XTAL32K_AUTO_RETURN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&self) -> XTAL32K_AUTO_RESTART_R {
        XTAL32K_AUTO_RESTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&self) -> XTAL32K_AUTO_BACKUP_R {
        XTAL32K_AUTO_BACKUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&self) -> XTAL32K_EXT_CLK_FO_R {
        XTAL32K_EXT_CLK_FO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&self) -> XTAL32K_WDT_RESET_R {
        XTAL32K_WDT_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&self) -> XTAL32K_WDT_CLK_FO_R {
        XTAL32K_WDT_CLK_FO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&self) -> XTAL32K_WDT_EN_R {
        XTAL32K_WDT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn xtl_ext_ctr_en(&mut self) -> XTL_EXT_CTR_EN_W {
        XTL_EXT_CTR_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn xtl_ext_ctr_lv(&mut self) -> XTL_EXT_CTR_LV_W {
        XTL_EXT_CTR_LV_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn xtal32k_gpio_sel(&mut self) -> XTAL32K_GPIO_SEL_W {
        XTAL32K_GPIO_SEL_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn dac_xtal_32k(&mut self) -> DAC_XTAL_32K_W {
        DAC_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xpd_xtal_32k(&mut self) -> XPD_XTAL_32K_W {
        XPD_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn dres_xtal_32k(&mut self) -> DRES_XTAL_32K_W {
        DRES_XTAL_32K_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn dgm_xtal_32k(&mut self) -> DGM_XTAL_32K_W {
        DGM_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dbuf_xtal_32k(&mut self) -> DBUF_XTAL_32K_W {
        DBUF_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn enckinit_xtal_32k(&mut self) -> ENCKINIT_XTAL_32K_W {
        ENCKINIT_XTAL_32K_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xtal32k_xpd_force(&mut self) -> XTAL32K_XPD_FORCE_W {
        XTAL32K_XPD_FORCE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn xtal32k_auto_return(&mut self) -> XTAL32K_AUTO_RETURN_W {
        XTAL32K_AUTO_RETURN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xtal32k_auto_restart(&mut self) -> XTAL32K_AUTO_RESTART_W {
        XTAL32K_AUTO_RESTART_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn xtal32k_auto_backup(&mut self) -> XTAL32K_AUTO_BACKUP_W {
        XTAL32K_AUTO_BACKUP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xtal32k_ext_clk_fo(&mut self) -> XTAL32K_EXT_CLK_FO_W {
        XTAL32K_EXT_CLK_FO_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xtal32k_wdt_reset(&mut self) -> XTAL32K_WDT_RESET_W {
        XTAL32K_WDT_RESET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xtal32k_wdt_clk_fo(&mut self) -> XTAL32K_WDT_CLK_FO_W {
        XTAL32K_WDT_CLK_FO_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xtal32k_wdt_en(&mut self) -> XTAL32K_WDT_EN_W {
        XTAL32K_WDT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_EXT_XTL_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_xtl_conf](index.html) module"]
pub struct EXT_XTL_CONF_SPEC;
impl crate::RegisterSpec for EXT_XTL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_xtl_conf::R](R) reader structure"]
impl crate::Readable for EXT_XTL_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_xtl_conf::W](W) writer structure"]
impl crate::Writable for EXT_XTL_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXT_XTL_CONF to value 0"]
impl crate::Resettable for EXT_XTL_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
