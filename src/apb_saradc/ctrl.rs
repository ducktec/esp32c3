#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT_ARB_CYCLE` reader - "]
pub struct WAIT_ARB_CYCLE_R(crate::FieldReader<u8, u8>);
impl WAIT_ARB_CYCLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAIT_ARB_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_ARB_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_ARB_CYCLE` writer - "]
pub struct WAIT_ARB_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_ARB_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `XPD_SAR_FORCE` reader - "]
pub struct XPD_SAR_FORCE_R(crate::FieldReader<u8, u8>);
impl XPD_SAR_FORCE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XPD_SAR_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XPD_SAR_FORCE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XPD_SAR_FORCE` writer - "]
pub struct XPD_SAR_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> XPD_SAR_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `SAR_PATT_P_CLEAR` reader - "]
pub struct SAR_PATT_P_CLEAR_R(crate::FieldReader<bool, bool>);
impl SAR_PATT_P_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAR_PATT_P_CLEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_PATT_P_CLEAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_PATT_P_CLEAR` writer - "]
pub struct SAR_PATT_P_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_PATT_P_CLEAR_W<'a> {
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
#[doc = "Field `SAR_PATT_LEN` reader - "]
pub struct SAR_PATT_LEN_R(crate::FieldReader<u8, u8>);
impl SAR_PATT_LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAR_PATT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_PATT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_PATT_LEN` writer - "]
pub struct SAR_PATT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_PATT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SAR_CLK_DIV` reader - "]
pub struct SAR_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SAR_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAR_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_CLK_DIV` writer - "]
pub struct SAR_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 7)) | ((value as u32 & 0xff) << 7);
        self.w
    }
}
#[doc = "Field `SAR_CLK_GATED` reader - "]
pub struct SAR_CLK_GATED_R(crate::FieldReader<bool, bool>);
impl SAR_CLK_GATED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAR_CLK_GATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAR_CLK_GATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAR_CLK_GATED` writer - "]
pub struct SAR_CLK_GATED_W<'a> {
    w: &'a mut W,
}
impl<'a> SAR_CLK_GATED_W<'a> {
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
#[doc = "Field `START` reader - "]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - "]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `START_FORCE` reader - "]
pub struct START_FORCE_R(crate::FieldReader<bool, bool>);
impl START_FORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_FORCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_FORCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START_FORCE` writer - "]
pub struct START_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_FORCE_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn wait_arb_cycle(&self) -> WAIT_ARB_CYCLE_R {
        WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn xpd_sar_force(&self) -> XPD_SAR_FORCE_R {
        XPD_SAR_FORCE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sar_patt_p_clear(&self) -> SAR_PATT_P_CLEAR_R {
        SAR_PATT_P_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn sar_patt_len(&self) -> SAR_PATT_LEN_R {
        SAR_PATT_LEN_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 7:14"]
    #[inline(always)]
    pub fn sar_clk_div(&self) -> SAR_CLK_DIV_R {
        SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_clk_gated(&self) -> SAR_CLK_GATED_R {
        SAR_CLK_GATED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start_force(&self) -> START_FORCE_R {
        START_FORCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn wait_arb_cycle(&mut self) -> WAIT_ARB_CYCLE_W {
        WAIT_ARB_CYCLE_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn xpd_sar_force(&mut self) -> XPD_SAR_FORCE_W {
        XPD_SAR_FORCE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sar_patt_p_clear(&mut self) -> SAR_PATT_P_CLEAR_W {
        SAR_PATT_P_CLEAR_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn sar_patt_len(&mut self) -> SAR_PATT_LEN_W {
        SAR_PATT_LEN_W { w: self }
    }
    #[doc = "Bits 7:14"]
    #[inline(always)]
    pub fn sar_clk_div(&mut self) -> SAR_CLK_DIV_W {
        SAR_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sar_clk_gated(&mut self) -> SAR_CLK_GATED_W {
        SAR_CLK_GATED_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start_force(&mut self) -> START_FORCE_W {
        START_FORCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_SARADC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
