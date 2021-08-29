#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_SIGMADELTA0"]
    pub sigmadelta0: crate::Reg<sigmadelta0::SIGMADELTA0_SPEC>,
    #[doc = "0x04 - GPIO_SIGMADELTA1"]
    pub sigmadelta1: crate::Reg<sigmadelta1::SIGMADELTA1_SPEC>,
    #[doc = "0x08 - GPIO_SIGMADELTA2"]
    pub sigmadelta2: crate::Reg<sigmadelta2::SIGMADELTA2_SPEC>,
    #[doc = "0x0c - GPIO_SIGMADELTA3"]
    pub sigmadelta3: crate::Reg<sigmadelta3::SIGMADELTA3_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - GPIO_SIGMADELTA_CG"]
    pub sigmadelta_cg: crate::Reg<sigmadelta_cg::SIGMADELTA_CG_SPEC>,
    #[doc = "0x24 - GPIO_SIGMADELTA_MISC"]
    pub sigmadelta_misc: crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>,
    #[doc = "0x28 - GPIO_SIGMADELTA_VERSION"]
    pub sigmadelta_version: crate::Reg<sigmadelta_version::SIGMADELTA_VERSION_SPEC>,
}
#[doc = "SIGMADELTA0 register accessor: an alias for `Reg<SIGMADELTA0_SPEC>`"]
pub type SIGMADELTA0 = crate::Reg<sigmadelta0::SIGMADELTA0_SPEC>;
#[doc = "GPIO_SIGMADELTA0"]
pub mod sigmadelta0;
#[doc = "SIGMADELTA1 register accessor: an alias for `Reg<SIGMADELTA1_SPEC>`"]
pub type SIGMADELTA1 = crate::Reg<sigmadelta1::SIGMADELTA1_SPEC>;
#[doc = "GPIO_SIGMADELTA1"]
pub mod sigmadelta1;
#[doc = "SIGMADELTA2 register accessor: an alias for `Reg<SIGMADELTA2_SPEC>`"]
pub type SIGMADELTA2 = crate::Reg<sigmadelta2::SIGMADELTA2_SPEC>;
#[doc = "GPIO_SIGMADELTA2"]
pub mod sigmadelta2;
#[doc = "SIGMADELTA3 register accessor: an alias for `Reg<SIGMADELTA3_SPEC>`"]
pub type SIGMADELTA3 = crate::Reg<sigmadelta3::SIGMADELTA3_SPEC>;
#[doc = "GPIO_SIGMADELTA3"]
pub mod sigmadelta3;
#[doc = "SIGMADELTA_CG register accessor: an alias for `Reg<SIGMADELTA_CG_SPEC>`"]
pub type SIGMADELTA_CG = crate::Reg<sigmadelta_cg::SIGMADELTA_CG_SPEC>;
#[doc = "GPIO_SIGMADELTA_CG"]
pub mod sigmadelta_cg;
#[doc = "SIGMADELTA_MISC register accessor: an alias for `Reg<SIGMADELTA_MISC_SPEC>`"]
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
#[doc = "GPIO_SIGMADELTA_MISC"]
pub mod sigmadelta_misc;
#[doc = "SIGMADELTA_VERSION register accessor: an alias for `Reg<SIGMADELTA_VERSION_SPEC>`"]
pub type SIGMADELTA_VERSION = crate::Reg<sigmadelta_version::SIGMADELTA_VERSION_SPEC>;
#[doc = "GPIO_SIGMADELTA_VERSION"]
pub mod sigmadelta_version;
