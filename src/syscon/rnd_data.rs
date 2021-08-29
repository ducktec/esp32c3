#[doc = "Register `RND_DATA` reader"]
pub struct R(crate::R<RND_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RND_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RND_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RND_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RND_DATA` reader - "]
pub struct RND_DATA_R(crate::FieldReader<u32, u32>);
impl RND_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RND_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RND_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rnd_data(&self) -> RND_DATA_R {
        RND_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "SYSCON_RND_DATA\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnd_data](index.html) module"]
pub struct RND_DATA_SPEC;
impl crate::RegisterSpec for RND_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnd_data::R](R) reader structure"]
impl crate::Readable for RND_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RND_DATA to value 0"]
impl crate::Resettable for RND_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
