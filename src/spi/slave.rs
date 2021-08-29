#[doc = "Register `SLAVE` reader"]
pub struct R(crate::R<SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE` writer"]
pub struct W(crate::W<SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPEC>;
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
impl From<crate::W<SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USR_CONF` reader - "]
pub struct USR_CONF_R(crate::FieldReader<bool, bool>);
impl USR_CONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        USR_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_CONF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USR_CONF` writer - "]
pub struct USR_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_CONF_W<'a> {
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
#[doc = "Field `SOFT_RESET` writer - "]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
#[doc = "Field `SLAVE_MODE` reader - "]
pub struct SLAVE_MODE_R(crate::FieldReader<bool, bool>);
impl SLAVE_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAVE_MODE` writer - "]
pub struct SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_MODE_W<'a> {
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
#[doc = "Field `DMA_SEG_MAGIC_VALUE` reader - "]
pub struct DMA_SEG_MAGIC_VALUE_R(crate::FieldReader<u8, u8>);
impl DMA_SEG_MAGIC_VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA_SEG_MAGIC_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_SEG_MAGIC_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_SEG_MAGIC_VALUE` writer - "]
pub struct DMA_SEG_MAGIC_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEG_MAGIC_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | ((value as u32 & 0x0f) << 22);
        self.w
    }
}
#[doc = "Field `SLV_WRBUF_BITLEN_EN` reader - "]
pub struct SLV_WRBUF_BITLEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WRBUF_BITLEN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WRBUF_BITLEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRBUF_BITLEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRBUF_BITLEN_EN` writer - "]
pub struct SLV_WRBUF_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRBUF_BITLEN_EN_W<'a> {
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
#[doc = "Field `SLV_RDBUF_BITLEN_EN` reader - "]
pub struct SLV_RDBUF_BITLEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RDBUF_BITLEN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RDBUF_BITLEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDBUF_BITLEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDBUF_BITLEN_EN` writer - "]
pub struct SLV_RDBUF_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDBUF_BITLEN_EN_W<'a> {
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
#[doc = "Field `SLV_WRDMA_BITLEN_EN` reader - "]
pub struct SLV_WRDMA_BITLEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_WRDMA_BITLEN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_WRDMA_BITLEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_WRDMA_BITLEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_WRDMA_BITLEN_EN` writer - "]
pub struct SLV_WRDMA_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WRDMA_BITLEN_EN_W<'a> {
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
#[doc = "Field `SLV_RDDMA_BITLEN_EN` reader - "]
pub struct SLV_RDDMA_BITLEN_EN_R(crate::FieldReader<bool, bool>);
impl SLV_RDDMA_BITLEN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLV_RDDMA_BITLEN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_RDDMA_BITLEN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_RDDMA_BITLEN_EN` writer - "]
pub struct SLV_RDDMA_BITLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RDDMA_BITLEN_EN_W<'a> {
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
#[doc = "Field `RSCK_DATA_OUT` reader - "]
pub struct RSCK_DATA_OUT_R(crate::FieldReader<bool, bool>);
impl RSCK_DATA_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSCK_DATA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSCK_DATA_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSCK_DATA_OUT` writer - "]
pub struct RSCK_DATA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSCK_DATA_OUT_W<'a> {
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
#[doc = "Field `CLK_MODE_13` reader - "]
pub struct CLK_MODE_13_R(crate::FieldReader<bool, bool>);
impl CLK_MODE_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK_MODE_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_MODE_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_MODE_13` writer - "]
pub struct CLK_MODE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_13_W<'a> {
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
#[doc = "Field `CLK_MODE` reader - "]
pub struct CLK_MODE_R(crate::FieldReader<u8, u8>);
impl CLK_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_MODE` writer - "]
pub struct CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_conf(&self) -> USR_CONF_R {
        USR_CONF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dma_seg_magic_value(&self) -> DMA_SEG_MAGIC_VALUE_R {
        DMA_SEG_MAGIC_VALUE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&self) -> SLV_WRBUF_BITLEN_EN_R {
        SLV_WRBUF_BITLEN_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&self) -> SLV_RDBUF_BITLEN_EN_R {
        SLV_RDBUF_BITLEN_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&self) -> SLV_WRDMA_BITLEN_EN_R {
        SLV_WRDMA_BITLEN_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&self) -> SLV_RDDMA_BITLEN_EN_R {
        SLV_RDDMA_BITLEN_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsck_data_out(&self) -> RSCK_DATA_OUT_R {
        RSCK_DATA_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_mode_13(&self) -> CLK_MODE_13_R {
        CLK_MODE_13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usr_conf(&mut self) -> USR_CONF_W {
        USR_CONF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W { w: self }
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn dma_seg_magic_value(&mut self) -> DMA_SEG_MAGIC_VALUE_W {
        DMA_SEG_MAGIC_VALUE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wrbuf_bitlen_en(&mut self) -> SLV_WRBUF_BITLEN_EN_W {
        SLV_WRBUF_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rdbuf_bitlen_en(&mut self) -> SLV_RDBUF_BITLEN_EN_W {
        SLV_RDBUF_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slv_wrdma_bitlen_en(&mut self) -> SLV_WRDMA_BITLEN_EN_W {
        SLV_WRDMA_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slv_rddma_bitlen_en(&mut self) -> SLV_RDDMA_BITLEN_EN_W {
        SLV_RDDMA_BITLEN_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rsck_data_out(&mut self) -> RSCK_DATA_OUT_W {
        RSCK_DATA_OUT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_mode_13(&mut self) -> CLK_MODE_13_W {
        CLK_MODE_13_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W {
        CLK_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_SLAVE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave](index.html) module"]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave::R](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave::W](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE to value 0"]
impl crate::Resettable for SLAVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
