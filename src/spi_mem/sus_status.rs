#[doc = "Register `SUS_STATUS` reader"]
pub struct R(crate::R<SUS_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUS_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUS_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUS_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUS_STATUS` writer"]
pub struct W(crate::W<SUS_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUS_STATUS_SPEC>;
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
impl From<crate::W<SUS_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUS_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI0_LOCK_EN` reader - "]
pub struct SPI0_LOCK_EN_R(crate::FieldReader<bool, bool>);
impl SPI0_LOCK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_LOCK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI0_LOCK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0_LOCK_EN` writer - "]
pub struct SPI0_LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_LOCK_EN_W<'a> {
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
#[doc = "Field `FLASH_PES_DLY_128` reader - "]
pub struct FLASH_PES_DLY_128_R(crate::FieldReader<bool, bool>);
impl FLASH_PES_DLY_128_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PES_DLY_128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PES_DLY_128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PES_DLY_128` writer - "]
pub struct FLASH_PES_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PES_DLY_128_W<'a> {
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
#[doc = "Field `FLASH_PER_DLY_128` reader - "]
pub struct FLASH_PER_DLY_128_R(crate::FieldReader<bool, bool>);
impl FLASH_PER_DLY_128_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PER_DLY_128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_PER_DLY_128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PER_DLY_128` writer - "]
pub struct FLASH_PER_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PER_DLY_128_W<'a> {
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
#[doc = "Field `FLASH_DP_DLY_128` reader - "]
pub struct FLASH_DP_DLY_128_R(crate::FieldReader<bool, bool>);
impl FLASH_DP_DLY_128_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_DP_DLY_128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_DP_DLY_128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_DP_DLY_128` writer - "]
pub struct FLASH_DP_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_DP_DLY_128_W<'a> {
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
#[doc = "Field `FLASH_RES_DLY_128` reader - "]
pub struct FLASH_RES_DLY_128_R(crate::FieldReader<bool, bool>);
impl FLASH_RES_DLY_128_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_RES_DLY_128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_RES_DLY_128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_RES_DLY_128` writer - "]
pub struct FLASH_RES_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_RES_DLY_128_W<'a> {
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
#[doc = "Field `FLASH_HPM_DLY_128` reader - "]
pub struct FLASH_HPM_DLY_128_R(crate::FieldReader<bool, bool>);
impl FLASH_HPM_DLY_128_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_HPM_DLY_128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_HPM_DLY_128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_HPM_DLY_128` writer - "]
pub struct FLASH_HPM_DLY_128_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_HPM_DLY_128_W<'a> {
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
#[doc = "Field `WAIT_PESR_CMD_2B` reader - "]
pub struct WAIT_PESR_CMD_2B_R(crate::FieldReader<bool, bool>);
impl WAIT_PESR_CMD_2B_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_PESR_CMD_2B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_PESR_CMD_2B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_PESR_CMD_2B` writer - "]
pub struct WAIT_PESR_CMD_2B_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_PESR_CMD_2B_W<'a> {
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
#[doc = "Field `FLASH_SUS` reader - "]
pub struct FLASH_SUS_R(crate::FieldReader<bool, bool>);
impl FLASH_SUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_SUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_SUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_SUS` writer - "]
pub struct FLASH_SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_SUS_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi0_lock_en(&self) -> SPI0_LOCK_EN_R {
        SPI0_LOCK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn flash_pes_dly_128(&self) -> FLASH_PES_DLY_128_R {
        FLASH_PES_DLY_128_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn flash_per_dly_128(&self) -> FLASH_PER_DLY_128_R {
        FLASH_PER_DLY_128_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flash_dp_dly_128(&self) -> FLASH_DP_DLY_128_R {
        FLASH_DP_DLY_128_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_res_dly_128(&self) -> FLASH_RES_DLY_128_R {
        FLASH_RES_DLY_128_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flash_hpm_dly_128(&self) -> FLASH_HPM_DLY_128_R {
        FLASH_HPM_DLY_128_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&self) -> WAIT_PESR_CMD_2B_R {
        WAIT_PESR_CMD_2B_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_sus(&self) -> FLASH_SUS_R {
        FLASH_SUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi0_lock_en(&mut self) -> SPI0_LOCK_EN_W {
        SPI0_LOCK_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn flash_pes_dly_128(&mut self) -> FLASH_PES_DLY_128_W {
        FLASH_PES_DLY_128_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn flash_per_dly_128(&mut self) -> FLASH_PER_DLY_128_W {
        FLASH_PER_DLY_128_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flash_dp_dly_128(&mut self) -> FLASH_DP_DLY_128_W {
        FLASH_DP_DLY_128_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flash_res_dly_128(&mut self) -> FLASH_RES_DLY_128_W {
        FLASH_RES_DLY_128_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flash_hpm_dly_128(&mut self) -> FLASH_HPM_DLY_128_W {
        FLASH_HPM_DLY_128_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&mut self) -> WAIT_PESR_CMD_2B_W {
        WAIT_PESR_CMD_2B_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flash_sus(&mut self) -> FLASH_SUS_W {
        FLASH_SUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_SUS_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sus_status](index.html) module"]
pub struct SUS_STATUS_SPEC;
impl crate::RegisterSpec for SUS_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sus_status::R](R) reader structure"]
impl crate::Readable for SUS_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sus_status::W](W) writer structure"]
impl crate::Writable for SUS_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUS_STATUS to value 0"]
impl crate::Resettable for SUS_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
