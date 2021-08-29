#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNTL_OPTIONS0"]
    pub options0: crate::Reg<options0::OPTIONS0_SPEC>,
    #[doc = "0x04 - RTC_CNTL_SLP_TIMER0"]
    pub slp_timer0: crate::Reg<slp_timer0::SLP_TIMER0_SPEC>,
    #[doc = "0x08 - RTC_CNTL_SLP_TIMER1"]
    pub slp_timer1: crate::Reg<slp_timer1::SLP_TIMER1_SPEC>,
    #[doc = "0x0c - RTC_CNTL_TIME_UPDATE"]
    pub time_update: crate::Reg<time_update::TIME_UPDATE_SPEC>,
    #[doc = "0x10 - RTC_CNTL_TIME_LOW0"]
    pub time_low0: crate::Reg<time_low0::TIME_LOW0_SPEC>,
    #[doc = "0x14 - RTC_CNTL_TIME_HIGH0"]
    pub time_high0: crate::Reg<time_high0::TIME_HIGH0_SPEC>,
    #[doc = "0x18 - RTC_CNTL_STATE0"]
    pub state0: crate::Reg<state0::STATE0_SPEC>,
    #[doc = "0x1c - RTC_CNTL_TIMER1"]
    pub timer1: crate::Reg<timer1::TIMER1_SPEC>,
    #[doc = "0x20 - RTC_CNTL_TIMER2"]
    pub timer2: crate::Reg<timer2::TIMER2_SPEC>,
    #[doc = "0x24 - RTC_CNTL_TIMER3"]
    pub timer3: crate::Reg<timer3::TIMER3_SPEC>,
    #[doc = "0x28 - RTC_CNTL_TIMER4"]
    pub timer4: crate::Reg<timer4::TIMER4_SPEC>,
    #[doc = "0x2c - RTC_CNTL_TIMER5"]
    pub timer5: crate::Reg<timer5::TIMER5_SPEC>,
    #[doc = "0x30 - RTC_CNTL_TIMER6"]
    pub timer6: crate::Reg<timer6::TIMER6_SPEC>,
    #[doc = "0x34 - RTC_CNTL_ANA_CONF"]
    pub ana_conf: crate::Reg<ana_conf::ANA_CONF_SPEC>,
    #[doc = "0x38 - RTC_CNTL_RESET_STATE"]
    pub reset_state: crate::Reg<reset_state::RESET_STATE_SPEC>,
    #[doc = "0x3c - RTC_CNTL_WAKEUP_STATE"]
    pub wakeup_state: crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>,
    #[doc = "0x40 - RTC_CNTL_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x44 - RTC_CNTL_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x48 - RTC_CNTL_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x4c - RTC_CNTL_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x50 - RTC_CNTL_STORE0"]
    pub store0: crate::Reg<store0::STORE0_SPEC>,
    #[doc = "0x54 - RTC_CNTL_STORE1"]
    pub store1: crate::Reg<store1::STORE1_SPEC>,
    #[doc = "0x58 - RTC_CNTL_STORE2"]
    pub store2: crate::Reg<store2::STORE2_SPEC>,
    #[doc = "0x5c - RTC_CNTL_STORE3"]
    pub store3: crate::Reg<store3::STORE3_SPEC>,
    #[doc = "0x60 - RTC_CNTL_EXT_XTL_CONF"]
    pub ext_xtl_conf: crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>,
    #[doc = "0x64 - RTC_CNTL_EXT_WAKEUP_CONF"]
    pub ext_wakeup_conf: crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>,
    #[doc = "0x68 - RTC_CNTL_SLP_REJECT_CONF"]
    pub slp_reject_conf: crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>,
    #[doc = "0x6c - RTC_CNTL_CPU_PERIOD_CONF"]
    pub cpu_period_conf: crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>,
    #[doc = "0x70 - RTC_CNTL_CLK_CONF"]
    pub clk_conf: crate::Reg<clk_conf::CLK_CONF_SPEC>,
    #[doc = "0x74 - RTC_CNTL_SLOW_CLK_CONF"]
    pub slow_clk_conf: crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>,
    #[doc = "0x78 - RTC_CNTL_SDIO_CONF"]
    pub sdio_conf: crate::Reg<sdio_conf::SDIO_CONF_SPEC>,
    #[doc = "0x7c - RTC_CNTL_BIAS_CONF"]
    pub bias_conf: crate::Reg<bias_conf::BIAS_CONF_SPEC>,
    #[doc = "0x80 - RTC_CNTL"]
    pub rtc_cntl: crate::Reg<rtc_cntl::RTC_CNTL_SPEC>,
    #[doc = "0x84 - RTC_CNTL_PWC"]
    pub pwc: crate::Reg<pwc::PWC_SPEC>,
    #[doc = "0x88 - RTC_CNTL_DIG_PWC"]
    pub dig_pwc: crate::Reg<dig_pwc::DIG_PWC_SPEC>,
    #[doc = "0x8c - RTC_CNTL_DIG_ISO"]
    pub dig_iso: crate::Reg<dig_iso::DIG_ISO_SPEC>,
    #[doc = "0x90 - RTC_CNTL_WDTCONFIG0"]
    pub wdtconfig0: crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>,
    #[doc = "0x94 - RTC_CNTL_WDTCONFIG1"]
    pub wdtconfig1: crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>,
    #[doc = "0x98 - RTC_CNTL_WDTCONFIG2"]
    pub wdtconfig2: crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>,
    #[doc = "0x9c - RTC_CNTL_WDTCONFIG3"]
    pub wdtconfig3: crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>,
    #[doc = "0xa0 - RTC_CNTL_WDTCONFIG4"]
    pub wdtconfig4: crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>,
    #[doc = "0xa4 - RTC_CNTL_WDTFEED"]
    pub wdtfeed: crate::Reg<wdtfeed::WDTFEED_SPEC>,
    #[doc = "0xa8 - RTC_CNTL_WDTWPROTECT"]
    pub wdtwprotect: crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>,
    #[doc = "0xac - RTC_CNTL_SWD_CONF"]
    pub swd_conf: crate::Reg<swd_conf::SWD_CONF_SPEC>,
    #[doc = "0xb0 - RTC_CNTL_SWD_WPROTECT"]
    pub swd_wprotect: crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>,
    #[doc = "0xb4 - RTC_CNTL_SW_CPU_STALL"]
    pub sw_cpu_stall: crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>,
    #[doc = "0xb8 - RTC_CNTL_STORE4"]
    pub store4: crate::Reg<store4::STORE4_SPEC>,
    #[doc = "0xbc - RTC_CNTL_STORE5"]
    pub store5: crate::Reg<store5::STORE5_SPEC>,
    #[doc = "0xc0 - RTC_CNTL_STORE6"]
    pub store6: crate::Reg<store6::STORE6_SPEC>,
    #[doc = "0xc4 - RTC_CNTL_STORE7"]
    pub store7: crate::Reg<store7::STORE7_SPEC>,
    #[doc = "0xc8 - RTC_CNTL_LOW_POWER_ST"]
    pub low_power_st: crate::Reg<low_power_st::LOW_POWER_ST_SPEC>,
    #[doc = "0xcc - RTC_CNTL_DIAG0"]
    pub diag0: crate::Reg<diag0::DIAG0_SPEC>,
    #[doc = "0xd0 - RTC_CNTL_PAD_HOLD"]
    pub pad_hold: crate::Reg<pad_hold::PAD_HOLD_SPEC>,
    #[doc = "0xd4 - RTC_CNTL_DIG_PAD_HOLD"]
    pub dig_pad_hold: crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>,
    #[doc = "0xd8 - RTC_CNTL_BROWN_OUT"]
    pub brown_out: crate::Reg<brown_out::BROWN_OUT_SPEC>,
    #[doc = "0xdc - RTC_CNTL_TIME_LOW1"]
    pub time_low1: crate::Reg<time_low1::TIME_LOW1_SPEC>,
    #[doc = "0xe0 - RTC_CNTL_TIME_HIGH1"]
    pub time_high1: crate::Reg<time_high1::TIME_HIGH1_SPEC>,
    #[doc = "0xe4 - RTC_CNTL_XTAL32K_CLK_FACTOR"]
    pub xtal32k_clk_factor: crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>,
    #[doc = "0xe8 - RTC_CNTL_XTAL32K_CONF"]
    pub xtal32k_conf: crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>,
    #[doc = "0xec - RTC_CNTL_USB_CONF"]
    pub usb_conf: crate::Reg<usb_conf::USB_CONF_SPEC>,
    #[doc = "0xf0 - RTC_CNTL_SLP_REJECT_CAUSE"]
    pub slp_reject_cause: crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>,
    #[doc = "0xf4 - RTC_CNTL_OPTION1"]
    pub option1: crate::Reg<option1::OPTION1_SPEC>,
    #[doc = "0xf8 - RTC_CNTL_SLP_WAKEUP_CAUSE"]
    pub slp_wakeup_cause: crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>,
    #[doc = "0xfc - RTC_CNTL_ULP_CP_TIMER_1"]
    pub ulp_cp_timer_1: crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>,
    #[doc = "0x100 - RTC_CNTL_INT_ENA_W1TS"]
    pub int_ena_w1ts: crate::Reg<int_ena_w1ts::INT_ENA_W1TS_SPEC>,
    #[doc = "0x104 - RTC_CNTL_INT_ENA_W1TC"]
    pub int_ena_w1tc: crate::Reg<int_ena_w1tc::INT_ENA_W1TC_SPEC>,
    #[doc = "0x108 - RTC_CNTL_RETENTION_CTRL"]
    pub retention_ctrl: crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>,
    #[doc = "0x10c - RTC_CNTL_FIB_SEL"]
    pub fib_sel: crate::Reg<fib_sel::FIB_SEL_SPEC>,
    #[doc = "0x110 - RTC_CNTL_GPIO_WAKEUP"]
    pub gpio_wakeup: crate::Reg<gpio_wakeup::GPIO_WAKEUP_SPEC>,
    #[doc = "0x114 - RTC_CNTL_DBG_SEL"]
    pub dbg_sel: crate::Reg<dbg_sel::DBG_SEL_SPEC>,
    #[doc = "0x118 - RTC_CNTL_DBG_MAP"]
    pub dbg_map: crate::Reg<dbg_map::DBG_MAP_SPEC>,
    #[doc = "0x11c - RTC_CNTL_SENSOR_CTRL"]
    pub sensor_ctrl: crate::Reg<sensor_ctrl::SENSOR_CTRL_SPEC>,
    #[doc = "0x120 - RTC_CNTL_DBG_SAR_SEL"]
    pub dbg_sar_sel: crate::Reg<dbg_sar_sel::DBG_SAR_SEL_SPEC>,
    #[doc = "0x124 - RTC_CNTL_PG_CTRL"]
    pub pg_ctrl: crate::Reg<pg_ctrl::PG_CTRL_SPEC>,
    _reserved74: [u8; 0xd4],
    #[doc = "0x1fc - RTC_CNTL_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "OPTIONS0 register accessor: an alias for `Reg<OPTIONS0_SPEC>`"]
pub type OPTIONS0 = crate::Reg<options0::OPTIONS0_SPEC>;
#[doc = "RTC_CNTL_OPTIONS0"]
pub mod options0;
#[doc = "SLP_TIMER0 register accessor: an alias for `Reg<SLP_TIMER0_SPEC>`"]
pub type SLP_TIMER0 = crate::Reg<slp_timer0::SLP_TIMER0_SPEC>;
#[doc = "RTC_CNTL_SLP_TIMER0"]
pub mod slp_timer0;
#[doc = "SLP_TIMER1 register accessor: an alias for `Reg<SLP_TIMER1_SPEC>`"]
pub type SLP_TIMER1 = crate::Reg<slp_timer1::SLP_TIMER1_SPEC>;
#[doc = "RTC_CNTL_SLP_TIMER1"]
pub mod slp_timer1;
#[doc = "TIME_UPDATE register accessor: an alias for `Reg<TIME_UPDATE_SPEC>`"]
pub type TIME_UPDATE = crate::Reg<time_update::TIME_UPDATE_SPEC>;
#[doc = "RTC_CNTL_TIME_UPDATE"]
pub mod time_update;
#[doc = "TIME_LOW0 register accessor: an alias for `Reg<TIME_LOW0_SPEC>`"]
pub type TIME_LOW0 = crate::Reg<time_low0::TIME_LOW0_SPEC>;
#[doc = "RTC_CNTL_TIME_LOW0"]
pub mod time_low0;
#[doc = "TIME_HIGH0 register accessor: an alias for `Reg<TIME_HIGH0_SPEC>`"]
pub type TIME_HIGH0 = crate::Reg<time_high0::TIME_HIGH0_SPEC>;
#[doc = "RTC_CNTL_TIME_HIGH0"]
pub mod time_high0;
#[doc = "STATE0 register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "RTC_CNTL_STATE0"]
pub mod state0;
#[doc = "TIMER1 register accessor: an alias for `Reg<TIMER1_SPEC>`"]
pub type TIMER1 = crate::Reg<timer1::TIMER1_SPEC>;
#[doc = "RTC_CNTL_TIMER1"]
pub mod timer1;
#[doc = "TIMER2 register accessor: an alias for `Reg<TIMER2_SPEC>`"]
pub type TIMER2 = crate::Reg<timer2::TIMER2_SPEC>;
#[doc = "RTC_CNTL_TIMER2"]
pub mod timer2;
#[doc = "TIMER3 register accessor: an alias for `Reg<TIMER3_SPEC>`"]
pub type TIMER3 = crate::Reg<timer3::TIMER3_SPEC>;
#[doc = "RTC_CNTL_TIMER3"]
pub mod timer3;
#[doc = "TIMER4 register accessor: an alias for `Reg<TIMER4_SPEC>`"]
pub type TIMER4 = crate::Reg<timer4::TIMER4_SPEC>;
#[doc = "RTC_CNTL_TIMER4"]
pub mod timer4;
#[doc = "TIMER5 register accessor: an alias for `Reg<TIMER5_SPEC>`"]
pub type TIMER5 = crate::Reg<timer5::TIMER5_SPEC>;
#[doc = "RTC_CNTL_TIMER5"]
pub mod timer5;
#[doc = "TIMER6 register accessor: an alias for `Reg<TIMER6_SPEC>`"]
pub type TIMER6 = crate::Reg<timer6::TIMER6_SPEC>;
#[doc = "RTC_CNTL_TIMER6"]
pub mod timer6;
#[doc = "ANA_CONF register accessor: an alias for `Reg<ANA_CONF_SPEC>`"]
pub type ANA_CONF = crate::Reg<ana_conf::ANA_CONF_SPEC>;
#[doc = "RTC_CNTL_ANA_CONF"]
pub mod ana_conf;
#[doc = "RESET_STATE register accessor: an alias for `Reg<RESET_STATE_SPEC>`"]
pub type RESET_STATE = crate::Reg<reset_state::RESET_STATE_SPEC>;
#[doc = "RTC_CNTL_RESET_STATE"]
pub mod reset_state;
#[doc = "WAKEUP_STATE register accessor: an alias for `Reg<WAKEUP_STATE_SPEC>`"]
pub type WAKEUP_STATE = crate::Reg<wakeup_state::WAKEUP_STATE_SPEC>;
#[doc = "RTC_CNTL_WAKEUP_STATE"]
pub mod wakeup_state;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RTC_CNTL_INT_ENA"]
pub mod int_ena;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RTC_CNTL_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RTC_CNTL_INT_ST"]
pub mod int_st;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RTC_CNTL_INT_CLR"]
pub mod int_clr;
#[doc = "STORE0 register accessor: an alias for `Reg<STORE0_SPEC>`"]
pub type STORE0 = crate::Reg<store0::STORE0_SPEC>;
#[doc = "RTC_CNTL_STORE0"]
pub mod store0;
#[doc = "STORE1 register accessor: an alias for `Reg<STORE1_SPEC>`"]
pub type STORE1 = crate::Reg<store1::STORE1_SPEC>;
#[doc = "RTC_CNTL_STORE1"]
pub mod store1;
#[doc = "STORE2 register accessor: an alias for `Reg<STORE2_SPEC>`"]
pub type STORE2 = crate::Reg<store2::STORE2_SPEC>;
#[doc = "RTC_CNTL_STORE2"]
pub mod store2;
#[doc = "STORE3 register accessor: an alias for `Reg<STORE3_SPEC>`"]
pub type STORE3 = crate::Reg<store3::STORE3_SPEC>;
#[doc = "RTC_CNTL_STORE3"]
pub mod store3;
#[doc = "EXT_XTL_CONF register accessor: an alias for `Reg<EXT_XTL_CONF_SPEC>`"]
pub type EXT_XTL_CONF = crate::Reg<ext_xtl_conf::EXT_XTL_CONF_SPEC>;
#[doc = "RTC_CNTL_EXT_XTL_CONF"]
pub mod ext_xtl_conf;
#[doc = "EXT_WAKEUP_CONF register accessor: an alias for `Reg<EXT_WAKEUP_CONF_SPEC>`"]
pub type EXT_WAKEUP_CONF = crate::Reg<ext_wakeup_conf::EXT_WAKEUP_CONF_SPEC>;
#[doc = "RTC_CNTL_EXT_WAKEUP_CONF"]
pub mod ext_wakeup_conf;
#[doc = "SLP_REJECT_CONF register accessor: an alias for `Reg<SLP_REJECT_CONF_SPEC>`"]
pub type SLP_REJECT_CONF = crate::Reg<slp_reject_conf::SLP_REJECT_CONF_SPEC>;
#[doc = "RTC_CNTL_SLP_REJECT_CONF"]
pub mod slp_reject_conf;
#[doc = "CPU_PERIOD_CONF register accessor: an alias for `Reg<CPU_PERIOD_CONF_SPEC>`"]
pub type CPU_PERIOD_CONF = crate::Reg<cpu_period_conf::CPU_PERIOD_CONF_SPEC>;
#[doc = "RTC_CNTL_CPU_PERIOD_CONF"]
pub mod cpu_period_conf;
#[doc = "CLK_CONF register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "RTC_CNTL_CLK_CONF"]
pub mod clk_conf;
#[doc = "SLOW_CLK_CONF register accessor: an alias for `Reg<SLOW_CLK_CONF_SPEC>`"]
pub type SLOW_CLK_CONF = crate::Reg<slow_clk_conf::SLOW_CLK_CONF_SPEC>;
#[doc = "RTC_CNTL_SLOW_CLK_CONF"]
pub mod slow_clk_conf;
#[doc = "SDIO_CONF register accessor: an alias for `Reg<SDIO_CONF_SPEC>`"]
pub type SDIO_CONF = crate::Reg<sdio_conf::SDIO_CONF_SPEC>;
#[doc = "RTC_CNTL_SDIO_CONF"]
pub mod sdio_conf;
#[doc = "BIAS_CONF register accessor: an alias for `Reg<BIAS_CONF_SPEC>`"]
pub type BIAS_CONF = crate::Reg<bias_conf::BIAS_CONF_SPEC>;
#[doc = "RTC_CNTL_BIAS_CONF"]
pub mod bias_conf;
#[doc = "RTC_CNTL register accessor: an alias for `Reg<RTC_CNTL_SPEC>`"]
pub type RTC_CNTL = crate::Reg<rtc_cntl::RTC_CNTL_SPEC>;
#[doc = "RTC_CNTL"]
pub mod rtc_cntl;
#[doc = "PWC register accessor: an alias for `Reg<PWC_SPEC>`"]
pub type PWC = crate::Reg<pwc::PWC_SPEC>;
#[doc = "RTC_CNTL_PWC"]
pub mod pwc;
#[doc = "DIG_PWC register accessor: an alias for `Reg<DIG_PWC_SPEC>`"]
pub type DIG_PWC = crate::Reg<dig_pwc::DIG_PWC_SPEC>;
#[doc = "RTC_CNTL_DIG_PWC"]
pub mod dig_pwc;
#[doc = "DIG_ISO register accessor: an alias for `Reg<DIG_ISO_SPEC>`"]
pub type DIG_ISO = crate::Reg<dig_iso::DIG_ISO_SPEC>;
#[doc = "RTC_CNTL_DIG_ISO"]
pub mod dig_iso;
#[doc = "WDTCONFIG0 register accessor: an alias for `Reg<WDTCONFIG0_SPEC>`"]
pub type WDTCONFIG0 = crate::Reg<wdtconfig0::WDTCONFIG0_SPEC>;
#[doc = "RTC_CNTL_WDTCONFIG0"]
pub mod wdtconfig0;
#[doc = "WDTCONFIG1 register accessor: an alias for `Reg<WDTCONFIG1_SPEC>`"]
pub type WDTCONFIG1 = crate::Reg<wdtconfig1::WDTCONFIG1_SPEC>;
#[doc = "RTC_CNTL_WDTCONFIG1"]
pub mod wdtconfig1;
#[doc = "WDTCONFIG2 register accessor: an alias for `Reg<WDTCONFIG2_SPEC>`"]
pub type WDTCONFIG2 = crate::Reg<wdtconfig2::WDTCONFIG2_SPEC>;
#[doc = "RTC_CNTL_WDTCONFIG2"]
pub mod wdtconfig2;
#[doc = "WDTCONFIG3 register accessor: an alias for `Reg<WDTCONFIG3_SPEC>`"]
pub type WDTCONFIG3 = crate::Reg<wdtconfig3::WDTCONFIG3_SPEC>;
#[doc = "RTC_CNTL_WDTCONFIG3"]
pub mod wdtconfig3;
#[doc = "WDTCONFIG4 register accessor: an alias for `Reg<WDTCONFIG4_SPEC>`"]
pub type WDTCONFIG4 = crate::Reg<wdtconfig4::WDTCONFIG4_SPEC>;
#[doc = "RTC_CNTL_WDTCONFIG4"]
pub mod wdtconfig4;
#[doc = "WDTFEED register accessor: an alias for `Reg<WDTFEED_SPEC>`"]
pub type WDTFEED = crate::Reg<wdtfeed::WDTFEED_SPEC>;
#[doc = "RTC_CNTL_WDTFEED"]
pub mod wdtfeed;
#[doc = "WDTWPROTECT register accessor: an alias for `Reg<WDTWPROTECT_SPEC>`"]
pub type WDTWPROTECT = crate::Reg<wdtwprotect::WDTWPROTECT_SPEC>;
#[doc = "RTC_CNTL_WDTWPROTECT"]
pub mod wdtwprotect;
#[doc = "SWD_CONF register accessor: an alias for `Reg<SWD_CONF_SPEC>`"]
pub type SWD_CONF = crate::Reg<swd_conf::SWD_CONF_SPEC>;
#[doc = "RTC_CNTL_SWD_CONF"]
pub mod swd_conf;
#[doc = "SWD_WPROTECT register accessor: an alias for `Reg<SWD_WPROTECT_SPEC>`"]
pub type SWD_WPROTECT = crate::Reg<swd_wprotect::SWD_WPROTECT_SPEC>;
#[doc = "RTC_CNTL_SWD_WPROTECT"]
pub mod swd_wprotect;
#[doc = "SW_CPU_STALL register accessor: an alias for `Reg<SW_CPU_STALL_SPEC>`"]
pub type SW_CPU_STALL = crate::Reg<sw_cpu_stall::SW_CPU_STALL_SPEC>;
#[doc = "RTC_CNTL_SW_CPU_STALL"]
pub mod sw_cpu_stall;
#[doc = "STORE4 register accessor: an alias for `Reg<STORE4_SPEC>`"]
pub type STORE4 = crate::Reg<store4::STORE4_SPEC>;
#[doc = "RTC_CNTL_STORE4"]
pub mod store4;
#[doc = "STORE5 register accessor: an alias for `Reg<STORE5_SPEC>`"]
pub type STORE5 = crate::Reg<store5::STORE5_SPEC>;
#[doc = "RTC_CNTL_STORE5"]
pub mod store5;
#[doc = "STORE6 register accessor: an alias for `Reg<STORE6_SPEC>`"]
pub type STORE6 = crate::Reg<store6::STORE6_SPEC>;
#[doc = "RTC_CNTL_STORE6"]
pub mod store6;
#[doc = "STORE7 register accessor: an alias for `Reg<STORE7_SPEC>`"]
pub type STORE7 = crate::Reg<store7::STORE7_SPEC>;
#[doc = "RTC_CNTL_STORE7"]
pub mod store7;
#[doc = "LOW_POWER_ST register accessor: an alias for `Reg<LOW_POWER_ST_SPEC>`"]
pub type LOW_POWER_ST = crate::Reg<low_power_st::LOW_POWER_ST_SPEC>;
#[doc = "RTC_CNTL_LOW_POWER_ST"]
pub mod low_power_st;
#[doc = "DIAG0 register accessor: an alias for `Reg<DIAG0_SPEC>`"]
pub type DIAG0 = crate::Reg<diag0::DIAG0_SPEC>;
#[doc = "RTC_CNTL_DIAG0"]
pub mod diag0;
#[doc = "PAD_HOLD register accessor: an alias for `Reg<PAD_HOLD_SPEC>`"]
pub type PAD_HOLD = crate::Reg<pad_hold::PAD_HOLD_SPEC>;
#[doc = "RTC_CNTL_PAD_HOLD"]
pub mod pad_hold;
#[doc = "DIG_PAD_HOLD register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = "RTC_CNTL_DIG_PAD_HOLD"]
pub mod dig_pad_hold;
#[doc = "BROWN_OUT register accessor: an alias for `Reg<BROWN_OUT_SPEC>`"]
pub type BROWN_OUT = crate::Reg<brown_out::BROWN_OUT_SPEC>;
#[doc = "RTC_CNTL_BROWN_OUT"]
pub mod brown_out;
#[doc = "TIME_LOW1 register accessor: an alias for `Reg<TIME_LOW1_SPEC>`"]
pub type TIME_LOW1 = crate::Reg<time_low1::TIME_LOW1_SPEC>;
#[doc = "RTC_CNTL_TIME_LOW1"]
pub mod time_low1;
#[doc = "TIME_HIGH1 register accessor: an alias for `Reg<TIME_HIGH1_SPEC>`"]
pub type TIME_HIGH1 = crate::Reg<time_high1::TIME_HIGH1_SPEC>;
#[doc = "RTC_CNTL_TIME_HIGH1"]
pub mod time_high1;
#[doc = "XTAL32K_CLK_FACTOR register accessor: an alias for `Reg<XTAL32K_CLK_FACTOR_SPEC>`"]
pub type XTAL32K_CLK_FACTOR = crate::Reg<xtal32k_clk_factor::XTAL32K_CLK_FACTOR_SPEC>;
#[doc = "RTC_CNTL_XTAL32K_CLK_FACTOR"]
pub mod xtal32k_clk_factor;
#[doc = "XTAL32K_CONF register accessor: an alias for `Reg<XTAL32K_CONF_SPEC>`"]
pub type XTAL32K_CONF = crate::Reg<xtal32k_conf::XTAL32K_CONF_SPEC>;
#[doc = "RTC_CNTL_XTAL32K_CONF"]
pub mod xtal32k_conf;
#[doc = "USB_CONF register accessor: an alias for `Reg<USB_CONF_SPEC>`"]
pub type USB_CONF = crate::Reg<usb_conf::USB_CONF_SPEC>;
#[doc = "RTC_CNTL_USB_CONF"]
pub mod usb_conf;
#[doc = "SLP_REJECT_CAUSE register accessor: an alias for `Reg<SLP_REJECT_CAUSE_SPEC>`"]
pub type SLP_REJECT_CAUSE = crate::Reg<slp_reject_cause::SLP_REJECT_CAUSE_SPEC>;
#[doc = "RTC_CNTL_SLP_REJECT_CAUSE"]
pub mod slp_reject_cause;
#[doc = "OPTION1 register accessor: an alias for `Reg<OPTION1_SPEC>`"]
pub type OPTION1 = crate::Reg<option1::OPTION1_SPEC>;
#[doc = "RTC_CNTL_OPTION1"]
pub mod option1;
#[doc = "SLP_WAKEUP_CAUSE register accessor: an alias for `Reg<SLP_WAKEUP_CAUSE_SPEC>`"]
pub type SLP_WAKEUP_CAUSE = crate::Reg<slp_wakeup_cause::SLP_WAKEUP_CAUSE_SPEC>;
#[doc = "RTC_CNTL_SLP_WAKEUP_CAUSE"]
pub mod slp_wakeup_cause;
#[doc = "ULP_CP_TIMER_1 register accessor: an alias for `Reg<ULP_CP_TIMER_1_SPEC>`"]
pub type ULP_CP_TIMER_1 = crate::Reg<ulp_cp_timer_1::ULP_CP_TIMER_1_SPEC>;
#[doc = "RTC_CNTL_ULP_CP_TIMER_1"]
pub mod ulp_cp_timer_1;
#[doc = "INT_ENA_W1TS register accessor: an alias for `Reg<INT_ENA_W1TS_SPEC>`"]
pub type INT_ENA_W1TS = crate::Reg<int_ena_w1ts::INT_ENA_W1TS_SPEC>;
#[doc = "RTC_CNTL_INT_ENA_W1TS"]
pub mod int_ena_w1ts;
#[doc = "INT_ENA_W1TC register accessor: an alias for `Reg<INT_ENA_W1TC_SPEC>`"]
pub type INT_ENA_W1TC = crate::Reg<int_ena_w1tc::INT_ENA_W1TC_SPEC>;
#[doc = "RTC_CNTL_INT_ENA_W1TC"]
pub mod int_ena_w1tc;
#[doc = "RETENTION_CTRL register accessor: an alias for `Reg<RETENTION_CTRL_SPEC>`"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "RTC_CNTL_RETENTION_CTRL"]
pub mod retention_ctrl;
#[doc = "FIB_SEL register accessor: an alias for `Reg<FIB_SEL_SPEC>`"]
pub type FIB_SEL = crate::Reg<fib_sel::FIB_SEL_SPEC>;
#[doc = "RTC_CNTL_FIB_SEL"]
pub mod fib_sel;
#[doc = "GPIO_WAKEUP register accessor: an alias for `Reg<GPIO_WAKEUP_SPEC>`"]
pub type GPIO_WAKEUP = crate::Reg<gpio_wakeup::GPIO_WAKEUP_SPEC>;
#[doc = "RTC_CNTL_GPIO_WAKEUP"]
pub mod gpio_wakeup;
#[doc = "DBG_SEL register accessor: an alias for `Reg<DBG_SEL_SPEC>`"]
pub type DBG_SEL = crate::Reg<dbg_sel::DBG_SEL_SPEC>;
#[doc = "RTC_CNTL_DBG_SEL"]
pub mod dbg_sel;
#[doc = "DBG_MAP register accessor: an alias for `Reg<DBG_MAP_SPEC>`"]
pub type DBG_MAP = crate::Reg<dbg_map::DBG_MAP_SPEC>;
#[doc = "RTC_CNTL_DBG_MAP"]
pub mod dbg_map;
#[doc = "SENSOR_CTRL register accessor: an alias for `Reg<SENSOR_CTRL_SPEC>`"]
pub type SENSOR_CTRL = crate::Reg<sensor_ctrl::SENSOR_CTRL_SPEC>;
#[doc = "RTC_CNTL_SENSOR_CTRL"]
pub mod sensor_ctrl;
#[doc = "DBG_SAR_SEL register accessor: an alias for `Reg<DBG_SAR_SEL_SPEC>`"]
pub type DBG_SAR_SEL = crate::Reg<dbg_sar_sel::DBG_SAR_SEL_SPEC>;
#[doc = "RTC_CNTL_DBG_SAR_SEL"]
pub mod dbg_sar_sel;
#[doc = "PG_CTRL register accessor: an alias for `Reg<PG_CTRL_SPEC>`"]
pub type PG_CTRL = crate::Reg<pg_ctrl::PG_CTRL_SPEC>;
#[doc = "RTC_CNTL_PG_CTRL"]
pub mod pg_ctrl;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RTC_CNTL_DATE"]
pub mod date;
