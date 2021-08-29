#[doc = "Register `RESET_STATE` reader"]
pub struct R(crate::R<RESET_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_STATE` writer"]
pub struct W(crate::W<RESET_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_STATE_SPEC>;
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
impl From<crate::W<RESET_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRESET_MASK_PROCPU` reader - "]
pub struct DRESET_MASK_PROCPU_R(crate::FieldReader<bool, bool>);
impl DRESET_MASK_PROCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRESET_MASK_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRESET_MASK_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRESET_MASK_PROCPU` writer - "]
pub struct DRESET_MASK_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DRESET_MASK_PROCPU_W<'a> {
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
#[doc = "Field `DRESET_MASK_APPCPU` reader - "]
pub struct DRESET_MASK_APPCPU_R(crate::FieldReader<bool, bool>);
impl DRESET_MASK_APPCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRESET_MASK_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRESET_MASK_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRESET_MASK_APPCPU` writer - "]
pub struct DRESET_MASK_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> DRESET_MASK_APPCPU_W<'a> {
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
#[doc = "Field `JTAG_RESET_FLAG_CLR_APPCPU` writer - "]
pub struct JTAG_RESET_FLAG_CLR_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_RESET_FLAG_CLR_APPCPU_W<'a> {
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
#[doc = "Field `JTAG_RESET_FLAG_CLR_PROCPU` writer - "]
pub struct JTAG_RESET_FLAG_CLR_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_RESET_FLAG_CLR_PROCPU_W<'a> {
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
#[doc = "Field `JTAG_RESET_FLAG_APPCPU` reader - "]
pub struct JTAG_RESET_FLAG_APPCPU_R(crate::FieldReader<bool, bool>);
impl JTAG_RESET_FLAG_APPCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_RESET_FLAG_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_RESET_FLAG_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_RESET_FLAG_PROCPU` reader - "]
pub struct JTAG_RESET_FLAG_PROCPU_R(crate::FieldReader<bool, bool>);
impl JTAG_RESET_FLAG_PROCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_RESET_FLAG_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_RESET_FLAG_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` reader - "]
pub struct OCD_HALT_ON_RESET_PROCPU_R(crate::FieldReader<bool, bool>);
impl OCD_HALT_ON_RESET_PROCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCD_HALT_ON_RESET_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCD_HALT_ON_RESET_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCD_HALT_ON_RESET_PROCPU` writer - "]
pub struct OCD_HALT_ON_RESET_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> OCD_HALT_ON_RESET_PROCPU_W<'a> {
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
#[doc = "Field `OCD_HALT_ON_RESET_APPCPU` reader - "]
pub struct OCD_HALT_ON_RESET_APPCPU_R(crate::FieldReader<bool, bool>);
impl OCD_HALT_ON_RESET_APPCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCD_HALT_ON_RESET_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCD_HALT_ON_RESET_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCD_HALT_ON_RESET_APPCPU` writer - "]
pub struct OCD_HALT_ON_RESET_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> OCD_HALT_ON_RESET_APPCPU_W<'a> {
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
#[doc = "Field `ALL_RESET_FLAG_CLR_APPCPU` writer - "]
pub struct ALL_RESET_FLAG_CLR_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_RESET_FLAG_CLR_APPCPU_W<'a> {
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
#[doc = "Field `ALL_RESET_FLAG_CLR_PROCPU` writer - "]
pub struct ALL_RESET_FLAG_CLR_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_RESET_FLAG_CLR_PROCPU_W<'a> {
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
#[doc = "Field `ALL_RESET_FLAG_APPCPU` reader - "]
pub struct ALL_RESET_FLAG_APPCPU_R(crate::FieldReader<bool, bool>);
impl ALL_RESET_FLAG_APPCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALL_RESET_FLAG_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALL_RESET_FLAG_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALL_RESET_FLAG_PROCPU` reader - "]
pub struct ALL_RESET_FLAG_PROCPU_R(crate::FieldReader<bool, bool>);
impl ALL_RESET_FLAG_PROCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALL_RESET_FLAG_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALL_RESET_FLAG_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` reader - "]
pub struct STAT_VECTOR_SEL_PROCPU_R(crate::FieldReader<bool, bool>);
impl STAT_VECTOR_SEL_PROCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        STAT_VECTOR_SEL_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAT_VECTOR_SEL_PROCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_VECTOR_SEL_PROCPU` writer - "]
pub struct STAT_VECTOR_SEL_PROCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_VECTOR_SEL_PROCPU_W<'a> {
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
#[doc = "Field `STAT_VECTOR_SEL_APPCPU` reader - "]
pub struct STAT_VECTOR_SEL_APPCPU_R(crate::FieldReader<bool, bool>);
impl STAT_VECTOR_SEL_APPCPU_R {
    pub(crate) fn new(bits: bool) -> Self {
        STAT_VECTOR_SEL_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAT_VECTOR_SEL_APPCPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_VECTOR_SEL_APPCPU` writer - "]
pub struct STAT_VECTOR_SEL_APPCPU_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_VECTOR_SEL_APPCPU_W<'a> {
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
#[doc = "Field `RESET_CAUSE_APPCPU` reader - "]
pub struct RESET_CAUSE_APPCPU_R(crate::FieldReader<u8, u8>);
impl RESET_CAUSE_APPCPU_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESET_CAUSE_APPCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_CAUSE_APPCPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_CAUSE_PROCPU` reader - "]
pub struct RESET_CAUSE_PROCPU_R(crate::FieldReader<u8, u8>);
impl RESET_CAUSE_PROCPU_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESET_CAUSE_PROCPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_CAUSE_PROCPU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dreset_mask_procpu(&self) -> DRESET_MASK_PROCPU_R {
        DRESET_MASK_PROCPU_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dreset_mask_appcpu(&self) -> DRESET_MASK_APPCPU_R {
        DRESET_MASK_APPCPU_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn jtag_reset_flag_appcpu(&self) -> JTAG_RESET_FLAG_APPCPU_R {
        JTAG_RESET_FLAG_APPCPU_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn jtag_reset_flag_procpu(&self) -> JTAG_RESET_FLAG_PROCPU_R {
        JTAG_RESET_FLAG_PROCPU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&self) -> OCD_HALT_ON_RESET_PROCPU_R {
        OCD_HALT_ON_RESET_PROCPU_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_appcpu(&self) -> OCD_HALT_ON_RESET_APPCPU_R {
        OCD_HALT_ON_RESET_APPCPU_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn all_reset_flag_appcpu(&self) -> ALL_RESET_FLAG_APPCPU_R {
        ALL_RESET_FLAG_APPCPU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn all_reset_flag_procpu(&self) -> ALL_RESET_FLAG_PROCPU_R {
        ALL_RESET_FLAG_PROCPU_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&self) -> STAT_VECTOR_SEL_PROCPU_R {
        STAT_VECTOR_SEL_PROCPU_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn stat_vector_sel_appcpu(&self) -> STAT_VECTOR_SEL_APPCPU_R {
        STAT_VECTOR_SEL_APPCPU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn reset_cause_appcpu(&self) -> RESET_CAUSE_APPCPU_R {
        RESET_CAUSE_APPCPU_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reset_cause_procpu(&self) -> RESET_CAUSE_PROCPU_R {
        RESET_CAUSE_PROCPU_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dreset_mask_procpu(&mut self) -> DRESET_MASK_PROCPU_W {
        DRESET_MASK_PROCPU_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dreset_mask_appcpu(&mut self) -> DRESET_MASK_APPCPU_W {
        DRESET_MASK_APPCPU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn jtag_reset_flag_clr_appcpu(&mut self) -> JTAG_RESET_FLAG_CLR_APPCPU_W {
        JTAG_RESET_FLAG_CLR_APPCPU_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn jtag_reset_flag_clr_procpu(&mut self) -> JTAG_RESET_FLAG_CLR_PROCPU_W {
        JTAG_RESET_FLAG_CLR_PROCPU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_procpu(&mut self) -> OCD_HALT_ON_RESET_PROCPU_W {
        OCD_HALT_ON_RESET_PROCPU_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ocd_halt_on_reset_appcpu(&mut self) -> OCD_HALT_ON_RESET_APPCPU_W {
        OCD_HALT_ON_RESET_APPCPU_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn all_reset_flag_clr_appcpu(&mut self) -> ALL_RESET_FLAG_CLR_APPCPU_W {
        ALL_RESET_FLAG_CLR_APPCPU_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn all_reset_flag_clr_procpu(&mut self) -> ALL_RESET_FLAG_CLR_PROCPU_W {
        ALL_RESET_FLAG_CLR_PROCPU_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn stat_vector_sel_procpu(&mut self) -> STAT_VECTOR_SEL_PROCPU_W {
        STAT_VECTOR_SEL_PROCPU_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn stat_vector_sel_appcpu(&mut self) -> STAT_VECTOR_SEL_APPCPU_W {
        STAT_VECTOR_SEL_APPCPU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_RESET_STATE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_state](index.html) module"]
pub struct RESET_STATE_SPEC;
impl crate::RegisterSpec for RESET_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_state::R](R) reader structure"]
impl crate::Readable for RESET_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_state::W](W) writer structure"]
impl crate::Writable for RESET_STATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET_STATE to value 0"]
impl crate::Resettable for RESET_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
