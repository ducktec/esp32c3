#[doc = "Register `DIG_PWC` reader"]
pub struct R(crate::R<DIG_PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIG_PWC` writer"]
pub struct W(crate::W<DIG_PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_PWC_SPEC>;
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
impl From<crate::W<DIG_PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DG_WRAP_PD_EN` reader - "]
pub struct DG_WRAP_PD_EN_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_PD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_PD_EN` writer - "]
pub struct DG_WRAP_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_PD_EN_W<'a> {
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
#[doc = "Field `WIFI_PD_EN` reader - "]
pub struct WIFI_PD_EN_R(crate::FieldReader<bool, bool>);
impl WIFI_PD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_PD_EN` writer - "]
pub struct WIFI_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_PD_EN_W<'a> {
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
#[doc = "Field `CPU_TOP_PD_EN` reader - "]
pub struct CPU_TOP_PD_EN_R(crate::FieldReader<bool, bool>);
impl CPU_TOP_PD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_PD_EN` writer - "]
pub struct CPU_TOP_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_PD_EN_W<'a> {
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
#[doc = "Field `DG_PERI_PD_EN` reader - "]
pub struct DG_PERI_PD_EN_R(crate::FieldReader<bool, bool>);
impl DG_PERI_PD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_PD_EN` writer - "]
pub struct DG_PERI_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_PD_EN_W<'a> {
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
#[doc = "Field `BT_PD_EN` reader - "]
pub struct BT_PD_EN_R(crate::FieldReader<bool, bool>);
impl BT_PD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BT_PD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_PD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_PD_EN` writer - "]
pub struct BT_PD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_PD_EN_W<'a> {
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
#[doc = "Field `CPU_TOP_FORCE_PU` reader - "]
pub struct CPU_TOP_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl CPU_TOP_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_FORCE_PU` writer - "]
pub struct CPU_TOP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_PU_W<'a> {
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
#[doc = "Field `CPU_TOP_FORCE_PD` reader - "]
pub struct CPU_TOP_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl CPU_TOP_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_TOP_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TOP_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TOP_FORCE_PD` writer - "]
pub struct CPU_TOP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TOP_FORCE_PD_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_PU` reader - "]
pub struct DG_WRAP_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_PU` writer - "]
pub struct DG_WRAP_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_PU_W<'a> {
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
#[doc = "Field `DG_WRAP_FORCE_PD` reader - "]
pub struct DG_WRAP_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DG_WRAP_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_WRAP_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_WRAP_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_WRAP_FORCE_PD` writer - "]
pub struct DG_WRAP_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_WRAP_FORCE_PD_W<'a> {
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
#[doc = "Field `WIFI_FORCE_PU` reader - "]
pub struct WIFI_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_PU` writer - "]
pub struct WIFI_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_PU_W<'a> {
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
#[doc = "Field `WIFI_FORCE_PD` reader - "]
pub struct WIFI_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl WIFI_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIFI_FORCE_PD` writer - "]
pub struct WIFI_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_FORCE_PD_W<'a> {
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
#[doc = "Field `FASTMEM_FORCE_LPU` reader - "]
pub struct FASTMEM_FORCE_LPU_R(crate::FieldReader<bool, bool>);
impl FASTMEM_FORCE_LPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTMEM_FORCE_LPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTMEM_FORCE_LPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTMEM_FORCE_LPU` writer - "]
pub struct FASTMEM_FORCE_LPU_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_LPU_W<'a> {
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
#[doc = "Field `FASTMEM_FORCE_LPD` reader - "]
pub struct FASTMEM_FORCE_LPD_R(crate::FieldReader<bool, bool>);
impl FASTMEM_FORCE_LPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTMEM_FORCE_LPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTMEM_FORCE_LPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTMEM_FORCE_LPD` writer - "]
pub struct FASTMEM_FORCE_LPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTMEM_FORCE_LPD_W<'a> {
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
#[doc = "Field `DG_PERI_FORCE_PU` reader - "]
pub struct DG_PERI_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl DG_PERI_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_FORCE_PU` writer - "]
pub struct DG_PERI_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_PU_W<'a> {
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
#[doc = "Field `DG_PERI_FORCE_PD` reader - "]
pub struct DG_PERI_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl DG_PERI_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DG_PERI_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DG_PERI_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DG_PERI_FORCE_PD` writer - "]
pub struct DG_PERI_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DG_PERI_FORCE_PD_W<'a> {
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
#[doc = "Field `BT_FORCE_PU` reader - "]
pub struct BT_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl BT_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        BT_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_FORCE_PU` writer - "]
pub struct BT_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_PU_W<'a> {
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
#[doc = "Field `BT_FORCE_PD` reader - "]
pub struct BT_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl BT_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BT_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_FORCE_PD` writer - "]
pub struct BT_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_FORCE_PD_W<'a> {
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
#[doc = "Field `LSLP_MEM_FORCE_PU` reader - "]
pub struct LSLP_MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl LSLP_MEM_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSLP_MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSLP_MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSLP_MEM_FORCE_PU` writer - "]
pub struct LSLP_MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> LSLP_MEM_FORCE_PU_W<'a> {
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
#[doc = "Field `LSLP_MEM_FORCE_PD` reader - "]
pub struct LSLP_MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl LSLP_MEM_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSLP_MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSLP_MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSLP_MEM_FORCE_PD` writer - "]
pub struct LSLP_MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> LSLP_MEM_FORCE_PD_W<'a> {
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
#[doc = "Field `VDD_SPI_PWR_FORCE` reader - "]
pub struct VDD_SPI_PWR_FORCE_R(crate::FieldReader<bool, bool>);
impl VDD_SPI_PWR_FORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDD_SPI_PWR_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_SPI_PWR_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_SPI_PWR_FORCE` writer - "]
pub struct VDD_SPI_PWR_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_SPI_PWR_FORCE_W<'a> {
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
#[doc = "Field `VDD_SPI_PWR_DRV` reader - "]
pub struct VDD_SPI_PWR_DRV_R(crate::FieldReader<u8, u8>);
impl VDD_SPI_PWR_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDD_SPI_PWR_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_SPI_PWR_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_SPI_PWR_DRV` writer - "]
pub struct VDD_SPI_PWR_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_SPI_PWR_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&self) -> DG_WRAP_PD_EN_R {
        DG_WRAP_PD_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wifi_pd_en(&self) -> WIFI_PD_EN_R {
        WIFI_PD_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cpu_top_pd_en(&self) -> CPU_TOP_PD_EN_R {
        CPU_TOP_PD_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dg_peri_pd_en(&self) -> DG_PERI_PD_EN_R {
        DG_PERI_PD_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bt_pd_en(&self) -> BT_PD_EN_R {
        BT_PD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cpu_top_force_pu(&self) -> CPU_TOP_FORCE_PU_R {
        CPU_TOP_FORCE_PU_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cpu_top_force_pd(&self) -> CPU_TOP_FORCE_PD_R {
        CPU_TOP_FORCE_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&self) -> DG_WRAP_FORCE_PU_R {
        DG_WRAP_FORCE_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&self) -> DG_WRAP_FORCE_PD_R {
        DG_WRAP_FORCE_PD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wifi_force_pu(&self) -> WIFI_FORCE_PU_R {
        WIFI_FORCE_PU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wifi_force_pd(&self) -> WIFI_FORCE_PD_R {
        WIFI_FORCE_PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fastmem_force_lpu(&self) -> FASTMEM_FORCE_LPU_R {
        FASTMEM_FORCE_LPU_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fastmem_force_lpd(&self) -> FASTMEM_FORCE_LPD_R {
        FASTMEM_FORCE_LPD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_peri_force_pu(&self) -> DG_PERI_FORCE_PU_R {
        DG_PERI_FORCE_PU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_peri_force_pd(&self) -> DG_PERI_FORCE_PD_R {
        DG_PERI_FORCE_PD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bt_force_pu(&self) -> BT_FORCE_PU_R {
        BT_FORCE_PU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bt_force_pd(&self) -> BT_FORCE_PD_R {
        BT_FORCE_PD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&self) -> LSLP_MEM_FORCE_PU_R {
        LSLP_MEM_FORCE_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&self) -> LSLP_MEM_FORCE_PD_R {
        LSLP_MEM_FORCE_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdd_spi_pwr_force(&self) -> VDD_SPI_PWR_FORCE_R {
        VDD_SPI_PWR_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vdd_spi_pwr_drv(&self) -> VDD_SPI_PWR_DRV_R {
        VDD_SPI_PWR_DRV_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dg_wrap_pd_en(&mut self) -> DG_WRAP_PD_EN_W {
        DG_WRAP_PD_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wifi_pd_en(&mut self) -> WIFI_PD_EN_W {
        WIFI_PD_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cpu_top_pd_en(&mut self) -> CPU_TOP_PD_EN_W {
        CPU_TOP_PD_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dg_peri_pd_en(&mut self) -> DG_PERI_PD_EN_W {
        DG_PERI_PD_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bt_pd_en(&mut self) -> BT_PD_EN_W {
        BT_PD_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cpu_top_force_pu(&mut self) -> CPU_TOP_FORCE_PU_W {
        CPU_TOP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cpu_top_force_pd(&mut self) -> CPU_TOP_FORCE_PD_W {
        CPU_TOP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn dg_wrap_force_pu(&mut self) -> DG_WRAP_FORCE_PU_W {
        DG_WRAP_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dg_wrap_force_pd(&mut self) -> DG_WRAP_FORCE_PD_W {
        DG_WRAP_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wifi_force_pu(&mut self) -> WIFI_FORCE_PU_W {
        WIFI_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wifi_force_pd(&mut self) -> WIFI_FORCE_PD_W {
        WIFI_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fastmem_force_lpu(&mut self) -> FASTMEM_FORCE_LPU_W {
        FASTMEM_FORCE_LPU_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fastmem_force_lpd(&mut self) -> FASTMEM_FORCE_LPD_W {
        FASTMEM_FORCE_LPD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dg_peri_force_pu(&mut self) -> DG_PERI_FORCE_PU_W {
        DG_PERI_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dg_peri_force_pd(&mut self) -> DG_PERI_FORCE_PD_W {
        DG_PERI_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bt_force_pu(&mut self) -> BT_FORCE_PU_W {
        BT_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bt_force_pd(&mut self) -> BT_FORCE_PD_W {
        BT_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lslp_mem_force_pu(&mut self) -> LSLP_MEM_FORCE_PU_W {
        LSLP_MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lslp_mem_force_pd(&mut self) -> LSLP_MEM_FORCE_PD_W {
        LSLP_MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vdd_spi_pwr_force(&mut self) -> VDD_SPI_PWR_FORCE_W {
        VDD_SPI_PWR_FORCE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vdd_spi_pwr_drv(&mut self) -> VDD_SPI_PWR_DRV_W {
        VDD_SPI_PWR_DRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_DIG_PWC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_pwc](index.html) module"]
pub struct DIG_PWC_SPEC;
impl crate::RegisterSpec for DIG_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_pwc::R](R) reader structure"]
impl crate::Readable for DIG_PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_pwc::W](W) writer structure"]
impl crate::Writable for DIG_PWC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIG_PWC to value 0"]
impl crate::Resettable for DIG_PWC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
