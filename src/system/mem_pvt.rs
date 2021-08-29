#[doc = "Register `MEM_PVT` reader"]
pub struct R(crate::R<MEM_PVT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_PVT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_PVT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_PVT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_PVT` writer"]
pub struct W(crate::W<MEM_PVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_PVT_SPEC>;
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
impl From<crate::W<MEM_PVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_PVT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_VT_SEL` reader - "]
pub struct MEM_VT_SEL_R(crate::FieldReader<u8, u8>);
impl MEM_VT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_VT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_VT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_VT_SEL` writer - "]
pub struct MEM_VT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_VT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `MEM_TIMING_ERR_CNT` reader - "]
pub struct MEM_TIMING_ERR_CNT_R(crate::FieldReader<u16, u16>);
impl MEM_TIMING_ERR_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        MEM_TIMING_ERR_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_TIMING_ERR_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_PVT_MONITOR_EN` reader - "]
pub struct MEM_PVT_MONITOR_EN_R(crate::FieldReader<bool, bool>);
impl MEM_PVT_MONITOR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_PVT_MONITOR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_PVT_MONITOR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_PVT_MONITOR_EN` writer - "]
pub struct MEM_PVT_MONITOR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PVT_MONITOR_EN_W<'a> {
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
#[doc = "Field `MEM_ERR_CNT_CLR` writer - "]
pub struct MEM_ERR_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_ERR_CNT_CLR_W<'a> {
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
#[doc = "Field `MEM_PATH_LEN` reader - "]
pub struct MEM_PATH_LEN_R(crate::FieldReader<u8, u8>);
impl MEM_PATH_LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_PATH_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_PATH_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_PATH_LEN` writer - "]
pub struct MEM_PATH_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_PATH_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mem_vt_sel(&self) -> MEM_VT_SEL_R {
        MEM_VT_SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 6:21"]
    #[inline(always)]
    pub fn mem_timing_err_cnt(&self) -> MEM_TIMING_ERR_CNT_R {
        MEM_TIMING_ERR_CNT_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mem_pvt_monitor_en(&self) -> MEM_PVT_MONITOR_EN_R {
        MEM_PVT_MONITOR_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mem_path_len(&self) -> MEM_PATH_LEN_R {
        MEM_PATH_LEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mem_vt_sel(&mut self) -> MEM_VT_SEL_W {
        MEM_VT_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn mem_pvt_monitor_en(&mut self) -> MEM_PVT_MONITOR_EN_W {
        MEM_PVT_MONITOR_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mem_err_cnt_clr(&mut self) -> MEM_ERR_CNT_CLR_W {
        MEM_ERR_CNT_CLR_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mem_path_len(&mut self) -> MEM_PATH_LEN_W {
        MEM_PATH_LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTEM_MEM_PVT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_pvt](index.html) module"]
pub struct MEM_PVT_SPEC;
impl crate::RegisterSpec for MEM_PVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_pvt::R](R) reader structure"]
impl crate::Readable for MEM_PVT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_pvt::W](W) writer structure"]
impl crate::Writable for MEM_PVT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_PVT to value 0"]
impl crate::Resettable for MEM_PVT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
