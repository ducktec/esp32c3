#[doc = "Register `REDUNDANT_ECO_CTRL` reader"]
pub struct R(crate::R<REDUNDANT_ECO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDUNDANT_ECO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDUNDANT_ECO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDUNDANT_ECO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDUNDANT_ECO_CTRL` writer"]
pub struct W(crate::W<REDUNDANT_ECO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDUNDANT_ECO_CTRL_SPEC>;
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
impl From<crate::W<REDUNDANT_ECO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDUNDANT_ECO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDUNDANT_ECO_RESULT` reader - "]
pub struct REDUNDANT_ECO_RESULT_R(crate::FieldReader<bool, bool>);
impl REDUNDANT_ECO_RESULT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REDUNDANT_ECO_RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REDUNDANT_ECO_RESULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REDUNDANT_ECO_DRIVE` reader - "]
pub struct REDUNDANT_ECO_DRIVE_R(crate::FieldReader<bool, bool>);
impl REDUNDANT_ECO_DRIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REDUNDANT_ECO_DRIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REDUNDANT_ECO_DRIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REDUNDANT_ECO_DRIVE` writer - "]
pub struct REDUNDANT_ECO_DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> REDUNDANT_ECO_DRIVE_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn redundant_eco_result(&self) -> REDUNDANT_ECO_RESULT_R {
        REDUNDANT_ECO_RESULT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn redundant_eco_drive(&self) -> REDUNDANT_ECO_DRIVE_R {
        REDUNDANT_ECO_DRIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn redundant_eco_drive(&mut self) -> REDUNDANT_ECO_DRIVE_W {
        REDUNDANT_ECO_DRIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSTEM_REDUNDANT_ECO_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redundant_eco_ctrl](index.html) module"]
pub struct REDUNDANT_ECO_CTRL_SPEC;
impl crate::RegisterSpec for REDUNDANT_ECO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redundant_eco_ctrl::R](R) reader structure"]
impl crate::Readable for REDUNDANT_ECO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redundant_eco_ctrl::W](W) writer structure"]
impl crate::Writable for REDUNDANT_ECO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REDUNDANT_ECO_CTRL to value 0"]
impl crate::Resettable for REDUNDANT_ECO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
