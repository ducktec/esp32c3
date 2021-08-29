#[doc = "Register `FIFO_ST` reader"]
pub struct R(crate::R<FIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLAVE_RW_POINT` reader - "]
pub struct SLAVE_RW_POINT_R(crate::FieldReader<u8, u8>);
impl SLAVE_RW_POINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_RW_POINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_RW_POINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_WADDR` reader - "]
pub struct TXFIFO_WADDR_R(crate::FieldReader<u8, u8>);
impl TXFIFO_WADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_WADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_RADDR` reader - "]
pub struct TXFIFO_RADDR_R(crate::FieldReader<u8, u8>);
impl TXFIFO_RADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFO_RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_RADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_WADDR` reader - "]
pub struct RXFIFO_WADDR_R(crate::FieldReader<u8, u8>);
impl RXFIFO_WADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_WADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_RADDR` reader - "]
pub struct RXFIFO_RADDR_R(crate::FieldReader<u8, u8>);
impl RXFIFO_RADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFO_RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_RADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn slave_rw_point(&self) -> SLAVE_RW_POINT_R {
        SLAVE_RW_POINT_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn txfifo_waddr(&self) -> TXFIFO_WADDR_R {
        TXFIFO_WADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn txfifo_raddr(&self) -> TXFIFO_RADDR_R {
        TXFIFO_RADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rxfifo_waddr(&self) -> RXFIFO_WADDR_R {
        RXFIFO_WADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rxfifo_raddr(&self) -> RXFIFO_RADDR_R {
        RXFIFO_RADDR_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "I2C_FIFO_ST\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_st](index.html) module"]
pub struct FIFO_ST_SPEC;
impl crate::RegisterSpec for FIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_st::R](R) reader structure"]
impl crate::Readable for FIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_ST to value 0"]
impl crate::Resettable for FIFO_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
