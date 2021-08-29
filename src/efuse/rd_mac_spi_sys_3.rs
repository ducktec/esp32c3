#[doc = "Register `RD_MAC_SPI_SYS_3` reader"]
pub struct R(crate::R<RD_MAC_SPI_SYS_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_MAC_SPI_SYS_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_MAC_SPI_SYS_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_MAC_SPI_SYS_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DATA_PART0_0` reader - "]
pub struct SYS_DATA_PART0_0_R(crate::FieldReader<u8, u8>);
impl SYS_DATA_PART0_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYS_DATA_PART0_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_DATA_PART0_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKG_VERSION` reader - "]
pub struct PKG_VERSION_R(crate::FieldReader<u8, u8>);
impl PKG_VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKG_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKG_VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAFER_VERSION` reader - "]
pub struct WAFER_VERSION_R(crate::FieldReader<u8, u8>);
impl WAFER_VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAFER_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAFER_VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_PAD_CONF_2` reader - "]
pub struct SPI_PAD_CONF_2_R(crate::FieldReader<u32, u32>);
impl SPI_PAD_CONF_2_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPI_PAD_CONF_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_PAD_CONF_2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sys_data_part0_0(&self) -> SYS_DATA_PART0_0_R {
        SYS_DATA_PART0_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn pkg_version(&self) -> PKG_VERSION_R {
        PKG_VERSION_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn wafer_version(&self) -> WAFER_VERSION_R {
        WAFER_VERSION_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn spi_pad_conf_2(&self) -> SPI_PAD_CONF_2_R {
        SPI_PAD_CONF_2_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "EFUSE_RD_MAC_SPI_SYS_3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_mac_spi_sys_3](index.html) module"]
pub struct RD_MAC_SPI_SYS_3_SPEC;
impl crate::RegisterSpec for RD_MAC_SPI_SYS_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_mac_spi_sys_3::R](R) reader structure"]
impl crate::Readable for RD_MAC_SPI_SYS_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_MAC_SPI_SYS_3 to value 0"]
impl crate::Resettable for RD_MAC_SPI_SYS_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
