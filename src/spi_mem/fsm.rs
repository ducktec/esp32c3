#[doc = "Register `FSM` reader"]
pub struct R(crate::R<FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM` writer"]
pub struct W(crate::W<FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SPEC>;
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
impl From<crate::W<FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSPI_LOCK_DELAY_TIME` reader - "]
pub struct CSPI_LOCK_DELAY_TIME_R(crate::FieldReader<u8, u8>);
impl CSPI_LOCK_DELAY_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSPI_LOCK_DELAY_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSPI_LOCK_DELAY_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSPI_LOCK_DELAY_TIME` writer - "]
pub struct CSPI_LOCK_DELAY_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPI_LOCK_DELAY_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | ((value as u32 & 0x1f) << 7);
        self.w
    }
}
#[doc = "Field `EM_ST` reader - "]
pub struct EM_ST_R(crate::FieldReader<u8, u8>);
impl EM_ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        EM_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSPI_ST` reader - "]
pub struct CSPI_ST_R(crate::FieldReader<u8, u8>);
impl CSPI_ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSPI_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSPI_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn cspi_lock_delay_time(&self) -> CSPI_LOCK_DELAY_TIME_R {
        CSPI_LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn em_st(&self) -> EM_ST_R {
        EM_ST_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cspi_st(&self) -> CSPI_ST_R {
        CSPI_ST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn cspi_lock_delay_time(&mut self) -> CSPI_LOCK_DELAY_TIME_W {
        CSPI_LOCK_DELAY_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_MEM_FSM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm](index.html) module"]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm::R](R) reader structure"]
impl crate::Readable for FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm::W](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM to value 0"]
impl crate::Resettable for FSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
