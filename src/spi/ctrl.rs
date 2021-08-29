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
#[doc = "Field `WR_BIT_ORDER` reader - "]
pub struct WR_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl WR_BIT_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_BIT_ORDER` writer - "]
pub struct WR_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_BIT_ORDER_W<'a> {
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
#[doc = "Field `RD_BIT_ORDER` reader - "]
pub struct RD_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl RD_BIT_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_BIT_ORDER` writer - "]
pub struct RD_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_BIT_ORDER_W<'a> {
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
#[doc = "Field `WP_POL` reader - "]
pub struct WP_POL_R(crate::FieldReader<bool, bool>);
impl WP_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WP_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WP_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WP_POL` writer - "]
pub struct WP_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_POL_W<'a> {
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
#[doc = "Field `HOLD_POL` reader - "]
pub struct HOLD_POL_R(crate::FieldReader<bool, bool>);
impl HOLD_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOLD_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOLD_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOLD_POL` writer - "]
pub struct HOLD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_POL_W<'a> {
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
#[doc = "Field `D_POL` reader - "]
pub struct D_POL_R(crate::FieldReader<bool, bool>);
impl D_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        D_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_POL` writer - "]
pub struct D_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> D_POL_W<'a> {
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
#[doc = "Field `Q_POL` reader - "]
pub struct Q_POL_R(crate::FieldReader<bool, bool>);
impl Q_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Q_POL` writer - "]
pub struct Q_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_POL_W<'a> {
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
#[doc = "Field `FREAD_QUAD` reader - "]
pub struct FREAD_QUAD_R(crate::FieldReader<bool, bool>);
impl FREAD_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_QUAD` writer - "]
pub struct FREAD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_QUAD_W<'a> {
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
#[doc = "Field `FREAD_DUAL` reader - "]
pub struct FREAD_DUAL_R(crate::FieldReader<bool, bool>);
impl FREAD_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREAD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREAD_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREAD_DUAL` writer - "]
pub struct FREAD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREAD_DUAL_W<'a> {
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
#[doc = "Field `FCMD_QUAD` reader - "]
pub struct FCMD_QUAD_R(crate::FieldReader<bool, bool>);
impl FCMD_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCMD_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMD_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMD_QUAD` writer - "]
pub struct FCMD_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_QUAD_W<'a> {
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
#[doc = "Field `FCMD_DUAL` reader - "]
pub struct FCMD_DUAL_R(crate::FieldReader<bool, bool>);
impl FCMD_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCMD_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCMD_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCMD_DUAL` writer - "]
pub struct FCMD_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_DUAL_W<'a> {
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
#[doc = "Field `FADDR_QUAD` reader - "]
pub struct FADDR_QUAD_R(crate::FieldReader<bool, bool>);
impl FADDR_QUAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_QUAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_QUAD` writer - "]
pub struct FADDR_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_QUAD_W<'a> {
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
#[doc = "Field `FADDR_DUAL` reader - "]
pub struct FADDR_DUAL_R(crate::FieldReader<bool, bool>);
impl FADDR_DUAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FADDR_DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADDR_DUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADDR_DUAL` writer - "]
pub struct FADDR_DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_DUAL_W<'a> {
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
#[doc = "Field `DUMMY_OUT` reader - "]
pub struct DUMMY_OUT_R(crate::FieldReader<bool, bool>);
impl DUMMY_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUMMY_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUMMY_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUMMY_OUT` writer - "]
pub struct DUMMY_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMMY_OUT_W<'a> {
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
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wr_bit_order(&self) -> WR_BIT_ORDER_R {
        WR_BIT_ORDER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_bit_order(&self) -> RD_BIT_ORDER_R {
        RD_BIT_ORDER_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wp_pol(&self) -> WP_POL_R {
        WP_POL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn hold_pol(&self) -> HOLD_POL_R {
        HOLD_POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dummy_out(&self) -> DUMMY_OUT_R {
        DUMMY_OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wr_bit_order(&mut self) -> WR_BIT_ORDER_W {
        WR_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_bit_order(&mut self) -> RD_BIT_ORDER_W {
        RD_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wp_pol(&mut self) -> WP_POL_W {
        WP_POL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn hold_pol(&mut self) -> HOLD_POL_W {
        HOLD_POL_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn d_pol(&mut self) -> D_POL_W {
        D_POL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn q_pol(&mut self) -> Q_POL_W {
        Q_POL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W {
        FREAD_QUAD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W {
        FREAD_DUAL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W {
        FCMD_QUAD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W {
        FCMD_DUAL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W {
        FADDR_QUAD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W {
        FADDR_DUAL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dummy_out(&mut self) -> DUMMY_OUT_W {
        DUMMY_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
