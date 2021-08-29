#[doc = "Register `TX_CRC` reader"]
pub struct R(crate::R<TX_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_CRC_DATA` reader - "]
pub struct TX_CRC_DATA_R(crate::FieldReader<u32, u32>);
impl TX_CRC_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        TX_CRC_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CRC_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_crc_data(&self) -> TX_CRC_DATA_R {
        TX_CRC_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "SPI_MEM_TX_CRC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_crc](index.html) module"]
pub struct TX_CRC_SPEC;
impl crate::RegisterSpec for TX_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_crc::R](R) reader structure"]
impl crate::Readable for TX_CRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_CRC to value 0"]
impl crate::Resettable for TX_CRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
