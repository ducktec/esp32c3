#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF_CNT_LSCH5_INT_ENA` reader - "]
pub struct OVF_CNT_LSCH5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH5_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH5_INT_ENA` writer - "]
pub struct OVF_CNT_LSCH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH5_INT_ENA_W<'a> {
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
#[doc = "Field `OVF_CNT_LSCH4_INT_ENA` reader - "]
pub struct OVF_CNT_LSCH4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH4_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH4_INT_ENA` writer - "]
pub struct OVF_CNT_LSCH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH4_INT_ENA_W<'a> {
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
#[doc = "Field `OVF_CNT_LSCH3_INT_ENA` reader - "]
pub struct OVF_CNT_LSCH3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH3_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH3_INT_ENA` writer - "]
pub struct OVF_CNT_LSCH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH3_INT_ENA_W<'a> {
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
#[doc = "Field `OVF_CNT_LSCH2_INT_ENA` reader - "]
pub struct OVF_CNT_LSCH2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH2_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH2_INT_ENA` writer - "]
pub struct OVF_CNT_LSCH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH2_INT_ENA_W<'a> {
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
#[doc = "Field `OVF_CNT_LSCH1_INT_ENA` reader - "]
pub struct OVF_CNT_LSCH1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH1_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH1_INT_ENA` writer - "]
pub struct OVF_CNT_LSCH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH1_INT_ENA_W<'a> {
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
#[doc = "Field `OVF_CNT_LSCH0_INT_ENA` reader - "]
pub struct OVF_CNT_LSCH0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl OVF_CNT_LSCH0_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_CNT_LSCH0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_CNT_LSCH0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVF_CNT_LSCH0_INT_ENA` writer - "]
pub struct OVF_CNT_LSCH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_CNT_LSCH0_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` reader - "]
pub struct DUTY_CHNG_END_LSCH5_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH5_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH5_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH5_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH5_INT_ENA` writer - "]
pub struct DUTY_CHNG_END_LSCH5_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH5_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` reader - "]
pub struct DUTY_CHNG_END_LSCH4_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH4_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH4_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH4_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH4_INT_ENA` writer - "]
pub struct DUTY_CHNG_END_LSCH4_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH4_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` reader - "]
pub struct DUTY_CHNG_END_LSCH3_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH3_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH3_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH3_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH3_INT_ENA` writer - "]
pub struct DUTY_CHNG_END_LSCH3_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH3_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` reader - "]
pub struct DUTY_CHNG_END_LSCH2_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH2_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH2_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH2_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH2_INT_ENA` writer - "]
pub struct DUTY_CHNG_END_LSCH2_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH2_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` reader - "]
pub struct DUTY_CHNG_END_LSCH1_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH1_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH1_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH1_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH1_INT_ENA` writer - "]
pub struct DUTY_CHNG_END_LSCH1_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH1_INT_ENA_W<'a> {
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
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` reader - "]
pub struct DUTY_CHNG_END_LSCH0_INT_ENA_R(crate::FieldReader<bool, bool>);
impl DUTY_CHNG_END_LSCH0_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_CHNG_END_LSCH0_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CHNG_END_LSCH0_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CHNG_END_LSCH0_INT_ENA` writer - "]
pub struct DUTY_CHNG_END_LSCH0_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CHNG_END_LSCH0_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER3_OVF_INT_ENA` reader - "]
pub struct LSTIMER3_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER3_OVF_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER3_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER3_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER3_OVF_INT_ENA` writer - "]
pub struct LSTIMER3_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER3_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER2_OVF_INT_ENA` reader - "]
pub struct LSTIMER2_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER2_OVF_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER2_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER2_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER2_OVF_INT_ENA` writer - "]
pub struct LSTIMER2_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER2_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER1_OVF_INT_ENA` reader - "]
pub struct LSTIMER1_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER1_OVF_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER1_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER1_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER1_OVF_INT_ENA` writer - "]
pub struct LSTIMER1_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER1_OVF_INT_ENA_W<'a> {
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
#[doc = "Field `LSTIMER0_OVF_INT_ENA` reader - "]
pub struct LSTIMER0_OVF_INT_ENA_R(crate::FieldReader<bool, bool>);
impl LSTIMER0_OVF_INT_ENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTIMER0_OVF_INT_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTIMER0_OVF_INT_ENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTIMER0_OVF_INT_ENA` writer - "]
pub struct LSTIMER0_OVF_INT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTIMER0_OVF_INT_ENA_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_ena(&self) -> OVF_CNT_LSCH5_INT_ENA_R {
        OVF_CNT_LSCH5_INT_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_ena(&self) -> OVF_CNT_LSCH4_INT_ENA_R {
        OVF_CNT_LSCH4_INT_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_ena(&self) -> OVF_CNT_LSCH3_INT_ENA_R {
        OVF_CNT_LSCH3_INT_ENA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_ena(&self) -> OVF_CNT_LSCH2_INT_ENA_R {
        OVF_CNT_LSCH2_INT_ENA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_ena(&self) -> OVF_CNT_LSCH1_INT_ENA_R {
        OVF_CNT_LSCH1_INT_ENA_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_ena(&self) -> OVF_CNT_LSCH0_INT_ENA_R {
        OVF_CNT_LSCH0_INT_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_ena(&self) -> DUTY_CHNG_END_LSCH5_INT_ENA_R {
        DUTY_CHNG_END_LSCH5_INT_ENA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_ena(&self) -> DUTY_CHNG_END_LSCH4_INT_ENA_R {
        DUTY_CHNG_END_LSCH4_INT_ENA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_ena(&self) -> DUTY_CHNG_END_LSCH3_INT_ENA_R {
        DUTY_CHNG_END_LSCH3_INT_ENA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_ena(&self) -> DUTY_CHNG_END_LSCH2_INT_ENA_R {
        DUTY_CHNG_END_LSCH2_INT_ENA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_ena(&self) -> DUTY_CHNG_END_LSCH1_INT_ENA_R {
        DUTY_CHNG_END_LSCH1_INT_ENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_ena(&self) -> DUTY_CHNG_END_LSCH0_INT_ENA_R {
        DUTY_CHNG_END_LSCH0_INT_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lstimer3_ovf_int_ena(&self) -> LSTIMER3_OVF_INT_ENA_R {
        LSTIMER3_OVF_INT_ENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lstimer2_ovf_int_ena(&self) -> LSTIMER2_OVF_INT_ENA_R {
        LSTIMER2_OVF_INT_ENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lstimer1_ovf_int_ena(&self) -> LSTIMER1_OVF_INT_ENA_R {
        LSTIMER1_OVF_INT_ENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lstimer0_ovf_int_ena(&self) -> LSTIMER0_OVF_INT_ENA_R {
        LSTIMER0_OVF_INT_ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ovf_cnt_lsch5_int_ena(&mut self) -> OVF_CNT_LSCH5_INT_ENA_W {
        OVF_CNT_LSCH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ovf_cnt_lsch4_int_ena(&mut self) -> OVF_CNT_LSCH4_INT_ENA_W {
        OVF_CNT_LSCH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ovf_cnt_lsch3_int_ena(&mut self) -> OVF_CNT_LSCH3_INT_ENA_W {
        OVF_CNT_LSCH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ovf_cnt_lsch2_int_ena(&mut self) -> OVF_CNT_LSCH2_INT_ENA_W {
        OVF_CNT_LSCH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovf_cnt_lsch1_int_ena(&mut self) -> OVF_CNT_LSCH1_INT_ENA_W {
        OVF_CNT_LSCH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ovf_cnt_lsch0_int_ena(&mut self) -> OVF_CNT_LSCH0_INT_ENA_W {
        OVF_CNT_LSCH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn duty_chng_end_lsch5_int_ena(&mut self) -> DUTY_CHNG_END_LSCH5_INT_ENA_W {
        DUTY_CHNG_END_LSCH5_INT_ENA_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn duty_chng_end_lsch4_int_ena(&mut self) -> DUTY_CHNG_END_LSCH4_INT_ENA_W {
        DUTY_CHNG_END_LSCH4_INT_ENA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn duty_chng_end_lsch3_int_ena(&mut self) -> DUTY_CHNG_END_LSCH3_INT_ENA_W {
        DUTY_CHNG_END_LSCH3_INT_ENA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn duty_chng_end_lsch2_int_ena(&mut self) -> DUTY_CHNG_END_LSCH2_INT_ENA_W {
        DUTY_CHNG_END_LSCH2_INT_ENA_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn duty_chng_end_lsch1_int_ena(&mut self) -> DUTY_CHNG_END_LSCH1_INT_ENA_W {
        DUTY_CHNG_END_LSCH1_INT_ENA_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn duty_chng_end_lsch0_int_ena(&mut self) -> DUTY_CHNG_END_LSCH0_INT_ENA_W {
        DUTY_CHNG_END_LSCH0_INT_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lstimer3_ovf_int_ena(&mut self) -> LSTIMER3_OVF_INT_ENA_W {
        LSTIMER3_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lstimer2_ovf_int_ena(&mut self) -> LSTIMER2_OVF_INT_ENA_W {
        LSTIMER2_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lstimer1_ovf_int_ena(&mut self) -> LSTIMER1_OVF_INT_ENA_W {
        LSTIMER1_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lstimer0_ovf_int_ena(&mut self) -> LSTIMER0_OVF_INT_ENA_W {
        LSTIMER0_OVF_INT_ENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_INT_ENA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
