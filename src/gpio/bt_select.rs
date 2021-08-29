#[doc = "Register `BT_SELECT` reader"]
pub struct R(crate::R<BT_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_SELECT` writer"]
pub struct W(crate::W<BT_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_SELECT_SPEC>;
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
impl From<crate::W<BT_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BT_SEL` reader - "]
pub struct BT_SEL_R(crate::FieldReader<u32, u32>);
impl BT_SEL_R {
    pub(crate) fn new(bits: u32) -> Self {
        BT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BT_SEL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BT_SEL` writer - "]
pub struct BT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bt_sel(&self) -> BT_SEL_R {
        BT_SEL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bt_sel(&mut self) -> BT_SEL_W {
        BT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_BT_SELECT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_select](index.html) module"]
pub struct BT_SELECT_SPEC;
impl crate::RegisterSpec for BT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_select::R](R) reader structure"]
impl crate::Readable for BT_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_select::W](W) writer structure"]
impl crate::Writable for BT_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT_SELECT to value 0"]
impl crate::Resettable for BT_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
