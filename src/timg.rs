#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMG_T0CONFIG"]
    pub t0config: crate::Reg<t0config::T0CONFIG_SPEC>,
    #[doc = "0x04 - TIMG_T0LO"]
    pub t0lo: crate::Reg<t0lo::T0LO_SPEC>,
    #[doc = "0x08 - TIMG_T0HI"]
    pub t0hi: crate::Reg<t0hi::T0HI_SPEC>,
    #[doc = "0x0c - TIMG_T0UPDATE"]
    pub t0update: crate::Reg<t0update::T0UPDATE_SPEC>,
    #[doc = "0x10 - TIMG_T0ALARMLO"]
    pub t0alarmlo: crate::Reg<t0alarmlo::T0ALARMLO_SPEC>,
    #[doc = "0x14 - TIMG_T0ALARMHI"]
    pub t0alarmhi: crate::Reg<t0alarmhi::T0ALARMHI_SPEC>,
    #[doc = "0x18 - TIMG_T0LOADLO"]
    pub t0loadlo: crate::Reg<t0loadlo::T0LOADLO_SPEC>,
    #[doc = "0x1c - TIMG_T0LOADHI"]
    pub t0loadhi: crate::Reg<t0loadhi::T0LOADHI_SPEC>,
    #[doc = "0x20 - TIMG_T0LOAD"]
    pub t0load: crate::Reg<t0load::T0LOAD_SPEC>,
    _reserved9: [u8; 0x24],
    #[doc = "0x48 - TIMG_WDTCONFIG0"]
    pub wdtconfig0: crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>,
    #[doc = "0x4c - TIMG_WDTCONFIG1"]
    pub wdtconfig1: crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>,
    #[doc = "0x50 - TIMG_WDTCONFIG2"]
    pub wdtconfig2: crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>,
    #[doc = "0x54 - TIMG_WDTCONFIG3"]
    pub wdtconfig3: crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>,
    #[doc = "0x58 - TIMG_WDTCONFIG4"]
    pub wdtconfig4: crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>,
    #[doc = "0x5c - TIMG_WDTCONFIG5"]
    pub wdtconfig5: crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>,
    #[doc = "0x60 - TIMG_WDTFEED"]
    pub wdtfeed: crate::Reg<wdtfeed::WDTFEED_SPEC>,
    #[doc = "0x64 - TIMG_WDTWPROTECT"]
    pub wdtwprotect: crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>,
    #[doc = "0x68 - TIMG_RTCCALICFG"]
    pub rtccalicfg: crate::Reg<rtccalicfg::RTCCALICFG_SPEC>,
    #[doc = "0x6c - TIMG_RTCCALICFG1"]
    pub rtccalicfg1: crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>,
    #[doc = "0x70 - TIMG_INT_ENA_TIMERS"]
    pub int_ena_timers: crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>,
    #[doc = "0x74 - TIMG_INT_RAW_TIMERS"]
    pub int_raw_timers: crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>,
    #[doc = "0x78 - TIMG_INT_ST_TIMERS"]
    pub int_st_timers: crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>,
    #[doc = "0x7c - TIMG_INT_CLR_TIMERS"]
    pub int_clr_timers: crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>,
    #[doc = "0x80 - TIMG_RTCCALICFG2"]
    pub rtccalicfg2: crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>,
    _reserved24: [u8; 0x74],
    #[doc = "0xf8 - TIMG_NTIMERS_DATE"]
    pub ntimers_date: crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>,
    #[doc = "0xfc - TIMG_CLK"]
    pub clk: crate::Reg<clk::CLK_SPEC>,
}
#[doc = "T0CONFIG register accessor: an alias for `Reg<T0CONFIG_SPEC>`"]
pub type T0CONFIG = crate::Reg<t0config::T0CONFIG_SPEC>;
#[doc = "TIMG_T0CONFIG"]
pub mod t0config;
#[doc = "T0LO register accessor: an alias for `Reg<T0LO_SPEC>`"]
pub type T0LO = crate::Reg<t0lo::T0LO_SPEC>;
#[doc = "TIMG_T0LO"]
pub mod t0lo;
#[doc = "T0HI register accessor: an alias for `Reg<T0HI_SPEC>`"]
pub type T0HI = crate::Reg<t0hi::T0HI_SPEC>;
#[doc = "TIMG_T0HI"]
pub mod t0hi;
#[doc = "T0UPDATE register accessor: an alias for `Reg<T0UPDATE_SPEC>`"]
pub type T0UPDATE = crate::Reg<t0update::T0UPDATE_SPEC>;
#[doc = "TIMG_T0UPDATE"]
pub mod t0update;
#[doc = "T0ALARMLO register accessor: an alias for `Reg<T0ALARMLO_SPEC>`"]
pub type T0ALARMLO = crate::Reg<t0alarmlo::T0ALARMLO_SPEC>;
#[doc = "TIMG_T0ALARMLO"]
pub mod t0alarmlo;
#[doc = "T0ALARMHI register accessor: an alias for `Reg<T0ALARMHI_SPEC>`"]
pub type T0ALARMHI = crate::Reg<t0alarmhi::T0ALARMHI_SPEC>;
#[doc = "TIMG_T0ALARMHI"]
pub mod t0alarmhi;
#[doc = "T0LOADLO register accessor: an alias for `Reg<T0LOADLO_SPEC>`"]
pub type T0LOADLO = crate::Reg<t0loadlo::T0LOADLO_SPEC>;
#[doc = "TIMG_T0LOADLO"]
pub mod t0loadlo;
#[doc = "T0LOADHI register accessor: an alias for `Reg<T0LOADHI_SPEC>`"]
pub type T0LOADHI = crate::Reg<t0loadhi::T0LOADHI_SPEC>;
#[doc = "TIMG_T0LOADHI"]
pub mod t0loadhi;
#[doc = "T0LOAD register accessor: an alias for `Reg<T0LOAD_SPEC>`"]
pub type T0LOAD = crate::Reg<t0load::T0LOAD_SPEC>;
#[doc = "TIMG_T0LOAD"]
pub mod t0load;
#[doc = "WDTCONFIG0 register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "TIMG_WDTCONFIG0"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "TIMG_WDTCONFIG1"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "TIMG_WDTCONFIG2"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "TIMG_WDTCONFIG3"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "TIMG_WDTCONFIG4"]
pub mod wdtconfig4;
#[doc = "WDTCONFIG5 register accessor: an alias for `Reg<WDTCONFIG5_SPEC>`"]
pub type WDTCONFIG5 = crate::Reg<wdtconfig5::WDTCONFIG5_SPEC>;
#[doc = "TIMG_WDTCONFIG5"]
pub mod wdtconfig5;
#[doc = "WDTFEED register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "TIMG_WDTFEED"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "TIMG_WDTWPROTECT"]
pub mod wdtwprotect;
#[doc = "RTCCALICFG register accessor: an alias for `Reg<RTCCALICFG_SPEC>`"]
pub type RTCCALICFG = crate::Reg<rtccalicfg::RTCCALICFG_SPEC>;
#[doc = "TIMG_RTCCALICFG"]
pub mod rtccalicfg;
#[doc = "RTCCALICFG1 register accessor: an alias for `Reg<RTCCALICFG1_SPEC>`"]
pub type RTCCALICFG1 = crate::Reg<rtccalicfg1::RTCCALICFG1_SPEC>;
#[doc = "TIMG_RTCCALICFG1"]
pub mod rtccalicfg1;
#[doc = "INT_ENA_TIMERS register accessor: an alias for `Reg<INT_ENA_TIMERS_SPEC>`"]
pub type INT_ENA_TIMERS = crate::Reg<int_ena_timers::INT_ENA_TIMERS_SPEC>;
#[doc = "TIMG_INT_ENA_TIMERS"]
pub mod int_ena_timers;
#[doc = "INT_RAW_TIMERS register accessor: an alias for `Reg<INT_RAW_TIMERS_SPEC>`"]
pub type INT_RAW_TIMERS = crate::Reg<int_raw_timers::INT_RAW_TIMERS_SPEC>;
#[doc = "TIMG_INT_RAW_TIMERS"]
pub mod int_raw_timers;
#[doc = "INT_ST_TIMERS register accessor: an alias for `Reg<INT_ST_TIMERS_SPEC>`"]
pub type INT_ST_TIMERS = crate::Reg<int_st_timers::INT_ST_TIMERS_SPEC>;
#[doc = "TIMG_INT_ST_TIMERS"]
pub mod int_st_timers;
#[doc = "INT_CLR_TIMERS register accessor: an alias for `Reg<INT_CLR_TIMERS_SPEC>`"]
pub type INT_CLR_TIMERS = crate::Reg<int_clr_timers::INT_CLR_TIMERS_SPEC>;
#[doc = "TIMG_INT_CLR_TIMERS"]
pub mod int_clr_timers;
#[doc = "RTCCALICFG2 register accessor: an alias for `Reg<RTCCALICFG2_SPEC>`"]
pub type RTCCALICFG2 = crate::Reg<rtccalicfg2::RTCCALICFG2_SPEC>;
#[doc = "TIMG_RTCCALICFG2"]
pub mod rtccalicfg2;
#[doc = "NTIMERS_DATE register accessor: an alias for `Reg<NTIMERS_DATE_SPEC>`"]
pub type NTIMERS_DATE = crate::Reg<ntimers_date::NTIMERS_DATE_SPEC>;
#[doc = "TIMG_NTIMERS_DATE"]
pub mod ntimers_date;
#[doc = "CLK register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "TIMG_CLK"]
pub mod clk;
