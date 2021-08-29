#[doc = "Register `MEM_CONF` reader"]
pub struct R(crate::R<MEM_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CONF` writer"]
pub struct W(crate::W<MEM_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CONF_SPEC>;
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
impl From<crate::W<MEM_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_FORCE_PU` reader - "]
pub struct MEM_FORCE_PU_R(crate::FieldReader<bool, bool>);
impl MEM_FORCE_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FORCE_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FORCE_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FORCE_PU` writer - "]
pub struct MEM_FORCE_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PU_W<'a> {
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
#[doc = "Field `MEM_FORCE_PD` reader - "]
pub struct MEM_FORCE_PD_R(crate::FieldReader<bool, bool>);
impl MEM_FORCE_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_FORCE_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_FORCE_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_FORCE_PD` writer - "]
pub struct MEM_FORCE_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_FORCE_PD_W<'a> {
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
#[doc = "Field `RX_TOUT_THRHD` reader - "]
pub struct RX_TOUT_THRHD_R(crate::FieldReader<u16, u16>);
impl RX_TOUT_THRHD_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_TOUT_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TOUT_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TOUT_THRHD` writer - "]
pub struct RX_TOUT_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `RX_FLOW_THRHD` reader - "]
pub struct RX_FLOW_THRHD_R(crate::FieldReader<u16, u16>);
impl RX_FLOW_THRHD_R {
    pub(crate) fn new(bits: u16) -> Self {
        RX_FLOW_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FLOW_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_THRHD` writer - "]
pub struct RX_FLOW_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 7)) | ((value as u32 & 0x01ff) << 7);
        self.w
    }
}
#[doc = "Field `TX_SIZE` reader - "]
pub struct TX_SIZE_R(crate::FieldReader<u8, u8>);
impl TX_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_SIZE` writer - "]
pub struct TX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `RX_SIZE` reader - "]
pub struct RX_SIZE_R(crate::FieldReader<u8, u8>);
impl RX_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SIZE` writer - "]
pub struct RX_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&self) -> RX_TOUT_THRHD_R {
        RX_TOUT_THRHD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&self) -> RX_FLOW_THRHD_R {
        RX_FLOW_THRHD_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_size(&self) -> TX_SIZE_R {
        TX_SIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn rx_size(&self) -> RX_SIZE_R {
        RX_SIZE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W {
        MEM_FORCE_PU_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W {
        MEM_FORCE_PD_W { w: self }
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rx_tout_thrhd(&mut self) -> RX_TOUT_THRHD_W {
        RX_TOUT_THRHD_W { w: self }
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn rx_flow_thrhd(&mut self) -> RX_FLOW_THRHD_W {
        RX_FLOW_THRHD_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_size(&mut self) -> TX_SIZE_W {
        TX_SIZE_W { w: self }
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn rx_size(&mut self) -> RX_SIZE_W {
        RX_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_MEM_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_conf](index.html) module"]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_conf::R](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_conf::W](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_CONF to value 0"]
impl crate::Resettable for MEM_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
