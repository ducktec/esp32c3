#[doc = "Register `REGION_PMS_CONSTRAIN_2` reader"]
pub struct R(crate::R<REGION_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION_PMS_CONSTRAIN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION_PMS_CONSTRAIN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION_PMS_CONSTRAIN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION_PMS_CONSTRAIN_2` writer"]
pub struct W(crate::W<REGION_PMS_CONSTRAIN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION_PMS_CONSTRAIN_2_SPEC>;
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
impl From<crate::W<REGION_PMS_CONSTRAIN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION_PMS_CONSTRAIN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_6` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_6` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_5` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_5` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_4` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_4` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_3` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_3` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_2` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_2` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_1` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_1` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_0` reader - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R(crate::FieldReader<u8, u8>);
impl REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGION_PMS_CONSTRAIN_WORLD_1_AREA_0` writer - "]
pub struct REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_6(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_5(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_4(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_3(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_2(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_1(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_0(&self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_6(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_6_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_5(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_5_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_4(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_4_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_3(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_3_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_2(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_2_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_1(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_1_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn region_pms_constrain_world_1_area_0(&mut self) -> REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W {
        REGION_PMS_CONSTRAIN_WORLD_1_AREA_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_REGION_PMS_CONSTRAIN_2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_pms_constrain_2](index.html) module"]
pub struct REGION_PMS_CONSTRAIN_2_SPEC;
impl crate::RegisterSpec for REGION_PMS_CONSTRAIN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region_pms_constrain_2::R](R) reader structure"]
impl crate::Readable for REGION_PMS_CONSTRAIN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region_pms_constrain_2::W](W) writer structure"]
impl crate::Writable for REGION_PMS_CONSTRAIN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REGION_PMS_CONSTRAIN_2 to value 0"]
impl crate::Resettable for REGION_PMS_CONSTRAIN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
