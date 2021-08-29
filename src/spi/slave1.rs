#[doc = "Register `SLAVE1` reader"]
pub struct R(crate::R<SLAVE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE1` writer"]
pub struct W(crate::W<SLAVE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE1_SPEC>;
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
impl From<crate::W<SLAVE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV_LAST_ADDR` reader - "]
pub struct SLV_LAST_ADDR_R(crate::FieldReader<u8, u8>);
impl SLV_LAST_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLV_LAST_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_LAST_ADDR` writer - "]
pub struct SLV_LAST_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `SLV_LAST_COMMAND` reader - "]
pub struct SLV_LAST_COMMAND_R(crate::FieldReader<u8, u8>);
impl SLV_LAST_COMMAND_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLV_LAST_COMMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_LAST_COMMAND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_LAST_COMMAND` writer - "]
pub struct SLV_LAST_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 18)) | ((value as u32 & 0xff) << 18);
        self.w
    }
}
#[doc = "Field `SLV_DATA_BITLEN` reader - "]
pub struct SLV_DATA_BITLEN_R(crate::FieldReader<u32, u32>);
impl SLV_DATA_BITLEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        SLV_DATA_BITLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_DATA_BITLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_DATA_BITLEN` writer - "]
pub struct SLV_DATA_BITLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_DATA_BITLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn slv_last_addr(&self) -> SLV_LAST_ADDR_R {
        SLV_LAST_ADDR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn slv_data_bitlen(&self) -> SLV_DATA_BITLEN_R {
        SLV_DATA_BITLEN_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn slv_last_addr(&mut self) -> SLV_LAST_ADDR_W {
        SLV_LAST_ADDR_W { w: self }
    }
    #[doc = "Bits 18:25"]
    #[inline(always)]
    pub fn slv_last_command(&mut self) -> SLV_LAST_COMMAND_W {
        SLV_LAST_COMMAND_W { w: self }
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn slv_data_bitlen(&mut self) -> SLV_DATA_BITLEN_W {
        SLV_DATA_BITLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_SLAVE1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave1](index.html) module"]
pub struct SLAVE1_SPEC;
impl crate::RegisterSpec for SLAVE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave1::R](R) reader structure"]
impl crate::Readable for SLAVE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave1::W](W) writer structure"]
impl crate::Writable for SLAVE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLAVE1 to value 0"]
impl crate::Resettable for SLAVE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
