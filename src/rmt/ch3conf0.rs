#[doc = "Register `CH3CONF0` reader"]
pub struct R(crate::R<CH3CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH3CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH3CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH3CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH3CONF0` writer"]
pub struct W(crate::W<CH3CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH3CONF0_SPEC>;
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
impl From<crate::W<CH3CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH3CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARRIER_OUT_LV` reader - "]
pub struct CARRIER_OUT_LV_R(crate::FieldReader<bool, bool>);
impl CARRIER_OUT_LV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_OUT_LV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_OUT_LV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_OUT_LV` writer - "]
pub struct CARRIER_OUT_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_OUT_LV_W<'a> {
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
#[doc = "Field `CARRIER_EN` reader - "]
pub struct CARRIER_EN_R(crate::FieldReader<bool, bool>);
impl CARRIER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARRIER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARRIER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARRIER_EN` writer - "]
pub struct CARRIER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CARRIER_EN_W<'a> {
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
#[doc = "Field `MEM_SIZE` reader - "]
pub struct MEM_SIZE_R(crate::FieldReader<u8, u8>);
impl MEM_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_SIZE` writer - "]
pub struct MEM_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `IDLE_THRES` reader - "]
pub struct IDLE_THRES_R(crate::FieldReader<u16, u16>);
impl IDLE_THRES_R {
    pub(crate) fn new(bits: u16) -> Self {
        IDLE_THRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLE_THRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLE_THRES` writer - "]
pub struct IDLE_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 8)) | ((value as u32 & 0x7fff) << 8);
        self.w
    }
}
#[doc = "Field `DIV_CNT` reader - "]
pub struct DIV_CNT_R(crate::FieldReader<u8, u8>);
impl DIV_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIV_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV_CNT` writer - "]
pub struct DIV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 8:22"]
    #[inline(always)]
    pub fn idle_thres(&self) -> IDLE_THRES_R {
        IDLE_THRES_R::new(((self.bits >> 8) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W {
        CARRIER_OUT_LV_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W {
        CARRIER_EN_W { w: self }
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn mem_size(&mut self) -> MEM_SIZE_W {
        MEM_SIZE_W { w: self }
    }
    #[doc = "Bits 8:22"]
    #[inline(always)]
    pub fn idle_thres(&mut self) -> IDLE_THRES_W {
        IDLE_THRES_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn div_cnt(&mut self) -> DIV_CNT_W {
        DIV_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH3CONF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3conf0](index.html) module"]
pub struct CH3CONF0_SPEC;
impl crate::RegisterSpec for CH3CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch3conf0::R](R) reader structure"]
impl crate::Readable for CH3CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch3conf0::W](W) writer structure"]
impl crate::Writable for CH3CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH3CONF0 to value 0"]
impl crate::Resettable for CH3CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
