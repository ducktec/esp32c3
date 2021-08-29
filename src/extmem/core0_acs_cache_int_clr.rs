#[doc = "Register `CORE0_ACS_CACHE_INT_CLR` writer"]
pub struct W(crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>;
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
impl From<crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE0_ACS_CACHE_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE0_DBUS_WR_IC_INT_CLR` writer - "]
pub struct CORE0_DBUS_WR_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_DBUS_WR_IC_INT_CLR_W<'a> {
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
#[doc = "Field `CORE0_DBUS_REJECT_INT_CLR` writer - "]
pub struct CORE0_DBUS_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_DBUS_REJECT_INT_CLR_W<'a> {
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
#[doc = "Field `CORE0_DBUS_ACS_MSK_IC_INT_CLR` writer - "]
pub struct CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_DBUS_ACS_MSK_IC_INT_CLR_W<'a> {
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
#[doc = "Field `CORE0_IBUS_REJECT_INT_CLR` writer - "]
pub struct CORE0_IBUS_REJECT_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_IBUS_REJECT_INT_CLR_W<'a> {
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
#[doc = "Field `CORE0_IBUS_WR_IC_INT_CLR` writer - "]
pub struct CORE0_IBUS_WR_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_IBUS_WR_IC_INT_CLR_W<'a> {
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
#[doc = "Field `CORE0_IBUS_ACS_MSK_IC_INT_CLR` writer - "]
pub struct CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE0_IBUS_ACS_MSK_IC_INT_CLR_W<'a> {
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
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn core0_dbus_wr_ic_int_clr(&mut self) -> CORE0_DBUS_WR_IC_INT_CLR_W {
        CORE0_DBUS_WR_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn core0_dbus_reject_int_clr(&mut self) -> CORE0_DBUS_REJECT_INT_CLR_W {
        CORE0_DBUS_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn core0_dbus_acs_msk_ic_int_clr(&mut self) -> CORE0_DBUS_ACS_MSK_IC_INT_CLR_W {
        CORE0_DBUS_ACS_MSK_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn core0_ibus_reject_int_clr(&mut self) -> CORE0_IBUS_REJECT_INT_CLR_W {
        CORE0_IBUS_REJECT_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn core0_ibus_wr_ic_int_clr(&mut self) -> CORE0_IBUS_WR_IC_INT_CLR_W {
        CORE0_IBUS_WR_IC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn core0_ibus_acs_msk_ic_int_clr(&mut self) -> CORE0_IBUS_ACS_MSK_IC_INT_CLR_W {
        CORE0_IBUS_ACS_MSK_IC_INT_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_CLR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_acs_cache_int_clr](index.html) module"]
pub struct CORE0_ACS_CACHE_INT_CLR_SPEC;
impl crate::RegisterSpec for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core0_acs_cache_int_clr::W](W) writer structure"]
impl crate::Writable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE0_ACS_CACHE_INT_CLR to value 0"]
impl crate::Resettable for CORE0_ACS_CACHE_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
