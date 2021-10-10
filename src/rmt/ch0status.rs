#[doc = "Register `CH0STATUS` reader"]
pub struct R(crate::R<CH0STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_MEM_RADDR` reader - "]
pub struct APB_MEM_RADDR_R(crate::FieldReader<u8, u8>);
impl APB_MEM_RADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        APB_MEM_RADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WR_ERR` reader - "]
pub struct APB_MEM_WR_ERR_R(crate::FieldReader<bool, bool>);
impl APB_MEM_WR_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_WR_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WR_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_EMPTY` reader - "]
pub struct MEM_EMPTY_R(crate::FieldReader<bool, bool>);
impl MEM_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MEM_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RD_ERR` reader - "]
pub struct APB_MEM_RD_ERR_R(crate::FieldReader<bool, bool>);
impl APB_MEM_RD_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_RD_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RD_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WADDR` reader - "]
pub struct APB_MEM_WADDR_R(crate::FieldReader<u16, u16>);
impl APB_MEM_WADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        APB_MEM_WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` reader - "]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_RADDR_EX` reader - "]
pub struct MEM_RADDR_EX_R(crate::FieldReader<u16, u16>);
impl MEM_RADDR_EX_R {
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RADDR_EX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RADDR_EX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn apb_mem_wr_err(&self) -> APB_MEM_WR_ERR_R {
        APB_MEM_WR_ERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn mem_empty(&self) -> MEM_EMPTY_R {
        MEM_EMPTY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn apb_mem_waddr(&self) -> APB_MEM_WADDR_R {
        APB_MEM_WADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn mem_raddr_ex(&self) -> MEM_RADDR_EX_R {
        MEM_RADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "RMT_CH0STATUS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0status](index.html) module"]
pub struct CH0STATUS_SPEC;
impl crate::RegisterSpec for CH0STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0status::R](R) reader structure"]
impl crate::Readable for CH0STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH0STATUS to value 0"]
impl crate::Resettable for CH0STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
