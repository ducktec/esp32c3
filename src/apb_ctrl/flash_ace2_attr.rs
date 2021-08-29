#[doc = "Register `FLASH_ACE2_ATTR` reader"]
pub struct R(crate::R<FLASH_ACE2_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ACE2_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ACE2_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ACE2_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ACE2_ATTR` writer"]
pub struct W(crate::W<FLASH_ACE2_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ACE2_ATTR_SPEC>;
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
impl From<crate::W<FLASH_ACE2_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ACE2_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_ACE2_ATTR` reader - "]
pub struct FLASH_ACE2_ATTR_R(crate::FieldReader<u8, u8>);
impl FLASH_ACE2_ATTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLASH_ACE2_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_ACE2_ATTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_ACE2_ATTR` writer - "]
pub struct FLASH_ACE2_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_ACE2_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn flash_ace2_attr(&self) -> FLASH_ACE2_ATTR_R {
        FLASH_ACE2_ATTR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn flash_ace2_attr(&mut self) -> FLASH_ACE2_ATTR_W {
        FLASH_ACE2_ATTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_FLASH_ACE2_ATTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ace2_attr](index.html) module"]
pub struct FLASH_ACE2_ATTR_SPEC;
impl crate::RegisterSpec for FLASH_ACE2_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ace2_attr::R](R) reader structure"]
impl crate::Readable for FLASH_ACE2_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ace2_attr::W](W) writer structure"]
impl crate::Writable for FLASH_ACE2_ATTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_ACE2_ATTR to value 0"]
impl crate::Resettable for FLASH_ACE2_ATTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
