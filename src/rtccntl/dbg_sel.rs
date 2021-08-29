#[doc = "Register `DBG_SEL` reader"]
pub struct R(crate::R<DBG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_SEL` writer"]
pub struct W(crate::W<DBG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SEL_SPEC>;
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
impl From<crate::W<DBG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUG_SEL4` reader - "]
pub struct DEBUG_SEL4_R(crate::FieldReader<u8, u8>);
impl DEBUG_SEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_SEL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_SEL4` writer - "]
pub struct DEBUG_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
#[doc = "Field `DEBUG_SEL3` reader - "]
pub struct DEBUG_SEL3_R(crate::FieldReader<u8, u8>);
impl DEBUG_SEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_SEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_SEL3` writer - "]
pub struct DEBUG_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 22)) | ((value as u32 & 0x1f) << 22);
        self.w
    }
}
#[doc = "Field `DEBUG_SEL2` reader - "]
pub struct DEBUG_SEL2_R(crate::FieldReader<u8, u8>);
impl DEBUG_SEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_SEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_SEL2` writer - "]
pub struct DEBUG_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Field `DEBUG_SEL1` reader - "]
pub struct DEBUG_SEL1_R(crate::FieldReader<u8, u8>);
impl DEBUG_SEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_SEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_SEL1` writer - "]
pub struct DEBUG_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
#[doc = "Field `DEBUG_SEL0` reader - "]
pub struct DEBUG_SEL0_R(crate::FieldReader<u8, u8>);
impl DEBUG_SEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_SEL0` writer - "]
pub struct DEBUG_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | ((value as u32 & 0x1f) << 7);
        self.w
    }
}
#[doc = "Field `DEBUG_BIT_SEL` reader - "]
pub struct DEBUG_BIT_SEL_R(crate::FieldReader<u8, u8>);
impl DEBUG_BIT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_BIT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_BIT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_BIT_SEL` writer - "]
pub struct DEBUG_BIT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_BIT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u32 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `DEBUG_12M_NO_GATING` reader - "]
pub struct DEBUG_12M_NO_GATING_R(crate::FieldReader<bool, bool>);
impl DEBUG_12M_NO_GATING_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBUG_12M_NO_GATING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_12M_NO_GATING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEBUG_12M_NO_GATING` writer - "]
pub struct DEBUG_12M_NO_GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_12M_NO_GATING_W<'a> {
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
impl R {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&self) -> DEBUG_12M_NO_GATING_R {
        DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W {
        DEBUG_SEL4_W { w: self }
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W {
        DEBUG_SEL3_W { w: self }
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W {
        DEBUG_SEL2_W { w: self }
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W {
        DEBUG_SEL1_W { w: self }
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W {
        DEBUG_SEL0_W { w: self }
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W {
        DEBUG_BIT_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&mut self) -> DEBUG_12M_NO_GATING_W {
        DEBUG_12M_NO_GATING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CNTL_DBG_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel](index.html) module"]
pub struct DBG_SEL_SPEC;
impl crate::RegisterSpec for DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_sel::R](R) reader structure"]
impl crate::Readable for DBG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_sel::W](W) writer structure"]
impl crate::Writable for DBG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_SEL to value 0"]
impl crate::Resettable for DBG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
