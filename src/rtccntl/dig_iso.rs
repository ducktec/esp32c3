#[doc = "Register `DIG_ISO` reader"]
pub struct R(crate::R<DIG_ISO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_ISO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_ISO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_ISO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_ISO` writer"]
pub struct W(crate::W<DIG_ISO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_ISO_SPEC>;
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
impl From<crate::W<DIG_ISO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_ISO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DG_WRAP_FORCE_NOISO` reader - "]
pub struct DG_WRAP_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_NOISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_NOISO` writer - "]
pub struct DG_WRAP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_ISO` reader - "]
pub struct DG_WRAP_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_ISO` writer - "]
pub struct DG_WRAP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_ISO_W<'a> {
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
#[doc = "Field `WIFI_FORCE_NOISO` reader - "]
pub struct WIFI_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_NOISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_NOISO` writer - "]
pub struct WIFI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_NOISO_W<'a> {
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
#[doc = "Field `WIFI_FORCE_ISO` reader - "]
pub struct WIFI_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_ISO` writer - "]
pub struct WIFI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_ISO_W<'a> {
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
#[doc = "Field `CPU_TOP_FORCE_NOISO` reader - "]
pub struct CPU_TOP_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl CPU_TOP_FORCE_NOISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_FORCE_NOISO` writer - "]
pub struct CPU_TOP_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_NOISO_W<'a> {
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
#[doc = "Field `CPU_TOP_FORCE_ISO` reader - "]
pub struct CPU_TOP_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl CPU_TOP_FORCE_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_FORCE_ISO` writer - "]
pub struct CPU_TOP_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_ISO_W<'a> {
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
#[doc = "Field `DG_PERI_FORCE_NOISO` reader - "]
pub struct DG_PERI_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl DG_PERI_FORCE_NOISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_FORCE_NOISO` writer - "]
pub struct DG_PERI_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_PERI_FORCE_ISO` reader - "]
pub struct DG_PERI_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl DG_PERI_FORCE_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_FORCE_ISO` writer - "]
pub struct DG_PERI_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_ISO_W<'a> {
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
#[doc = "Field `BT_FORCE_NOISO` reader - "]
pub struct BT_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl BT_FORCE_NOISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BT_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_FORCE_NOISO` writer - "]
pub struct BT_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_NOISO_W<'a> {
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
#[doc = "Field `BT_FORCE_ISO` reader - "]
pub struct BT_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl BT_FORCE_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BT_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_FORCE_ISO` writer - "]
pub struct BT_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_ISO_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_HOLD` reader - "]
pub struct DG_PAD_FORCE_HOLD_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_HOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_HOLD` writer - "]
pub struct DG_PAD_FORCE_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_HOLD_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_UNHOLD` reader - "]
pub struct DG_PAD_FORCE_UNHOLD_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_UNHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_UNHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_UNHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_UNHOLD` writer - "]
pub struct DG_PAD_FORCE_UNHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_UNHOLD_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_ISO` reader - "]
pub struct DG_PAD_FORCE_ISO_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_ISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_ISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_ISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_ISO` writer - "]
pub struct DG_PAD_FORCE_ISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_ISO_W<'a> {
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
#[doc = "Field `DG_PAD_FORCE_NOISO` reader - "]
pub struct DG_PAD_FORCE_NOISO_R(crate::FieldReader<bool, bool>);
impl DG_PAD_FORCE_NOISO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_FORCE_NOISO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_FORCE_NOISO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_FORCE_NOISO` writer - "]
pub struct DG_PAD_FORCE_NOISO_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_FORCE_NOISO_W<'a> {
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
#[doc = "Field `DG_PAD_AUTOHOLD_EN` reader - "]
pub struct DG_PAD_AUTOHOLD_EN_R(crate::FieldReader<bool, bool>);
impl DG_PAD_AUTOHOLD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_AUTOHOLD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_AUTOHOLD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PAD_AUTOHOLD_EN` writer - "]
pub struct DG_PAD_AUTOHOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PAD_AUTOHOLD_EN_W<'a> {
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
#[doc = "Field `CLR_DG_PAD_AUTOHOLD` writer - "]
pub struct CLR_DG_PAD_AUTOHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DG_PAD_AUTOHOLD_W<'a> {
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
#[doc = "Field `DG_PAD_AUTOHOLD` reader - "]
pub struct DG_PAD_AUTOHOLD_R(crate::FieldReader<bool, bool>);
impl DG_PAD_AUTOHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PAD_AUTOHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PAD_AUTOHOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_ISO_FORCE_ON` reader - "]
pub struct DIG_ISO_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl DIG_ISO_FORCE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_ISO_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_ISO_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_ISO_FORCE_ON` writer - "]
pub struct DIG_ISO_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_ISO_FORCE_ON_W<'a> {
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
#[doc = "Field `DIG_ISO_FORCE_OFF` reader - "]
pub struct DIG_ISO_FORCE_OFF_R(crate::FieldReader<bool, bool>);
impl DIG_ISO_FORCE_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIG_ISO_FORCE_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIG_ISO_FORCE_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIG_ISO_FORCE_OFF` writer - "]
pub struct DIG_ISO_FORCE_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIG_ISO_FORCE_OFF_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&self) -> DG_WRAP_FORCE_NOISO_R {
        DG_WRAP_FORCE_NOISO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&self) -> DG_WRAP_FORCE_ISO_R {
        DG_WRAP_FORCE_ISO_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifi_force_noiso(&self) -> WIFI_FORCE_NOISO_R {
        WIFI_FORCE_NOISO_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifi_force_iso(&self) -> WIFI_FORCE_ISO_R {
        WIFI_FORCE_ISO_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cpu_top_force_noiso(&self) -> CPU_TOP_FORCE_NOISO_R {
        CPU_TOP_FORCE_NOISO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cpu_top_force_iso(&self) -> CPU_TOP_FORCE_ISO_R {
        CPU_TOP_FORCE_ISO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dg_peri_force_noiso(&self) -> DG_PERI_FORCE_NOISO_R {
        DG_PERI_FORCE_NOISO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dg_peri_force_iso(&self) -> DG_PERI_FORCE_ISO_R {
        DG_PERI_FORCE_ISO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bt_force_noiso(&self) -> BT_FORCE_NOISO_R {
        BT_FORCE_NOISO_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bt_force_iso(&self) -> BT_FORCE_ISO_R {
        BT_FORCE_ISO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&self) -> DG_PAD_FORCE_HOLD_R {
        DG_PAD_FORCE_HOLD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&self) -> DG_PAD_FORCE_UNHOLD_R {
        DG_PAD_FORCE_UNHOLD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&self) -> DG_PAD_FORCE_ISO_R {
        DG_PAD_FORCE_ISO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&self) -> DG_PAD_FORCE_NOISO_R {
        DG_PAD_FORCE_NOISO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&self) -> DG_PAD_AUTOHOLD_EN_R {
        DG_PAD_AUTOHOLD_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dg_pad_autohold(&self) -> DG_PAD_AUTOHOLD_R {
        DG_PAD_AUTOHOLD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dig_iso_force_on(&self) -> DIG_ISO_FORCE_ON_R {
        DIG_ISO_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dig_iso_force_off(&self) -> DIG_ISO_FORCE_OFF_R {
        DIG_ISO_FORCE_OFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dg_wrap_force_noiso(&mut self) -> DG_WRAP_FORCE_NOISO_W {
        DG_WRAP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dg_wrap_force_iso(&mut self) -> DG_WRAP_FORCE_ISO_W {
        DG_WRAP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wifi_force_noiso(&mut self) -> WIFI_FORCE_NOISO_W {
        WIFI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifi_force_iso(&mut self) -> WIFI_FORCE_ISO_W {
        WIFI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cpu_top_force_noiso(&mut self) -> CPU_TOP_FORCE_NOISO_W {
        CPU_TOP_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cpu_top_force_iso(&mut self) -> CPU_TOP_FORCE_ISO_W {
        CPU_TOP_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dg_peri_force_noiso(&mut self) -> DG_PERI_FORCE_NOISO_W {
        DG_PERI_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dg_peri_force_iso(&mut self) -> DG_PERI_FORCE_ISO_W {
        DG_PERI_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bt_force_noiso(&mut self) -> BT_FORCE_NOISO_W {
        BT_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn bt_force_iso(&mut self) -> BT_FORCE_ISO_W {
        BT_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dg_pad_force_hold(&mut self) -> DG_PAD_FORCE_HOLD_W {
        DG_PAD_FORCE_HOLD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_pad_force_unhold(&mut self) -> DG_PAD_FORCE_UNHOLD_W {
        DG_PAD_FORCE_UNHOLD_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_pad_force_iso(&mut self) -> DG_PAD_FORCE_ISO_W {
        DG_PAD_FORCE_ISO_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dg_pad_force_noiso(&mut self) -> DG_PAD_FORCE_NOISO_W {
        DG_PAD_FORCE_NOISO_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dg_pad_autohold_en(&mut self) -> DG_PAD_AUTOHOLD_EN_W {
        DG_PAD_AUTOHOLD_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clr_dg_pad_autohold(&mut self) -> CLR_DG_PAD_AUTOHOLD_W {
        CLR_DG_PAD_AUTOHOLD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dig_iso_force_on(&mut self) -> DIG_ISO_FORCE_ON_W {
        DIG_ISO_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dig_iso_force_off(&mut self) -> DIG_ISO_FORCE_OFF_W {
        DIG_ISO_FORCE_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_DIG_ISO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_iso](index.html) module"]
pub struct DIG_ISO_SPEC;
impl crate::RegisterSpec for DIG_ISO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_iso::R](R) reader structure"]
impl crate::Readable for DIG_ISO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_iso::W](W) writer structure"]
impl crate::Writable for DIG_ISO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG_ISO to value 0"]
impl crate::Resettable for DIG_ISO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
