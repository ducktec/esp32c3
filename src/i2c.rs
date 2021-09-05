#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_SCL_LOW_PERIOD"]
    pub scl_low_period: crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>,
    #[doc = "0x04 - I2C_CTR"]
    pub ctr: crate::Reg<ctr::CTR_SPEC>,
    #[doc = "0x08 - I2C_SR"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x0c - I2C_TO"]
    pub to: crate::Reg<to::TO_SPEC>,
    #[doc = "0x10 - I2C_SLAVE_ADDR"]
    pub slave_addr: crate::Reg<slave_addr::SLAVE_ADDR_SPEC>,
    #[doc = "0x14 - I2C_FIFO_ST"]
    pub fifo_st: crate::Reg<fifo_st::FIFO_ST_SPEC>,
    #[doc = "0x18 - I2C_FIFO_CONF"]
    pub fifo_conf: crate::Reg<fifo_conf::FIFO_CONF_SPEC>,
    #[doc = "0x1c - I2C_DATA"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x20 - I2C_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x24 - I2C_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x28 - I2C_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x2c - I2C_INT_STATUS"]
    pub int_status: crate::Reg<int_status::INT_STATUS_SPEC>,
    #[doc = "0x30 - I2C_SDA_HOLD"]
    pub sda_hold: crate::Reg<sda_hold::SDA_HOLD_SPEC>,
    #[doc = "0x34 - I2C_SDA_SAMPLE"]
    pub sda_sample: crate::Reg<sda_sample::SDA_SAMPLE_SPEC>,
    #[doc = "0x38 - I2C_SCL_HIGH_PERIOD"]
    pub scl_high_period: crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - I2C_SCL_START_HOLD"]
    pub scl_start_hold: crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>,
    #[doc = "0x44 - I2C_SCL_RSTART_SETUP"]
    pub scl_rstart_setup: crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>,
    #[doc = "0x48 - I2C_SCL_STOP_HOLD"]
    pub scl_stop_hold: crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>,
    #[doc = "0x4c - I2C_SCL_STOP_SETUP"]
    pub scl_stop_setup: crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>,
    #[doc = "0x50 - I2C_FILTER_CFG"]
    pub filter_cfg: crate::Reg<filter_cfg::FILTER_CFG_SPEC>,
    #[doc = "0x54 - I2C_CLK_CONF"]
    pub clk_conf: crate::Reg<clk_conf::CLK_CONF_SPEC>,
    #[doc = "0x58..0x78 - I2C_COMD%s"]
    pub comd: [crate::Reg<comd::COMD_SPEC>; 8],
    #[doc = "0x78 - I2C_SCL_ST_TIME_OUT"]
    pub scl_st_time_out: crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>,
    #[doc = "0x7c - I2C_SCL_MAIN_ST_TIME_OUT"]
    pub scl_main_st_time_out: crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>,
    #[doc = "0x80 - I2C_SCL_SP_CONF"]
    pub scl_sp_conf: crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>,
    #[doc = "0x84 - I2C_SCL_STRETCH_CONF"]
    pub scl_stretch_conf: crate::Reg<scl_stretch_conf::SCL_STRETCH_CONF_SPEC>,
    _reserved26: [u8; 0x70],
    #[doc = "0xf8 - I2C_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SCL_LOW_PERIOD register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = "I2C_SCL_LOW_PERIOD"]
pub mod scl_low_period;
#[doc = "CTR register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "I2C_CTR"]
pub mod ctr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "I2C_SR"]
pub mod sr;
#[doc = "TO register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "I2C_TO"]
pub mod to;
#[doc = "SLAVE_ADDR register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "I2C_SLAVE_ADDR"]
pub mod slave_addr;
#[doc = "FIFO_ST register accessor: an alias for `Reg<FIFO_ST_SPEC>`"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "I2C_FIFO_ST"]
pub mod fifo_st;
#[doc = "FIFO_CONF register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = "I2C_FIFO_CONF"]
pub mod fifo_conf;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "I2C_DATA"]
pub mod data;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "I2C_INT_RAW"]
pub mod int_raw;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "I2C_INT_CLR"]
pub mod int_clr;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "I2C_INT_ENA"]
pub mod int_ena;
#[doc = "INT_STATUS register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "I2C_INT_STATUS"]
pub mod int_status;
#[doc = "SDA_HOLD register accessor: an alias for `Reg<SDA_HOLD_SPEC>`"]
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
#[doc = "I2C_SDA_HOLD"]
pub mod sda_hold;
#[doc = "SDA_SAMPLE register accessor: an alias for `Reg<SDA_SAMPLE_SPEC>`"]
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
#[doc = "I2C_SDA_SAMPLE"]
pub mod sda_sample;
#[doc = "SCL_HIGH_PERIOD register accessor: an alias for `Reg<SCL_HIGH_PERIOD_SPEC>`"]
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
#[doc = "I2C_SCL_HIGH_PERIOD"]
pub mod scl_high_period;
#[doc = "SCL_START_HOLD register accessor: an alias for `Reg<SCL_START_HOLD_SPEC>`"]
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
#[doc = "I2C_SCL_START_HOLD"]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP register accessor: an alias for `Reg<SCL_RSTART_SETUP_SPEC>`"]
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
#[doc = "I2C_SCL_RSTART_SETUP"]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD register accessor: an alias for `Reg<SCL_STOP_HOLD_SPEC>`"]
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
#[doc = "I2C_SCL_STOP_HOLD"]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP register accessor: an alias for `Reg<SCL_STOP_SETUP_SPEC>`"]
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
#[doc = "I2C_SCL_STOP_SETUP"]
pub mod scl_stop_setup;
#[doc = "FILTER_CFG register accessor: an alias for `Reg<FILTER_CFG_SPEC>`"]
pub type FILTER_CFG = crate::Reg<filter_cfg::FILTER_CFG_SPEC>;
#[doc = "I2C_FILTER_CFG"]
pub mod filter_cfg;
#[doc = "CLK_CONF register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "I2C_CLK_CONF"]
pub mod clk_conf;
#[doc = "COMD register accessor: an alias for `Reg<COMD_SPEC>`"]
pub type COMD = crate::Reg<comd::COMD_SPEC>;
#[doc = "I2C_COMD%s"]
pub mod comd;
#[doc = "SCL_ST_TIME_OUT register accessor: an alias for `Reg<SCL_ST_TIME_OUT_SPEC>`"]
pub type SCL_ST_TIME_OUT = crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>;
#[doc = "I2C_SCL_ST_TIME_OUT"]
pub mod scl_st_time_out;
#[doc = "SCL_MAIN_ST_TIME_OUT register accessor: an alias for `Reg<SCL_MAIN_ST_TIME_OUT_SPEC>`"]
pub type SCL_MAIN_ST_TIME_OUT = crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT"]
pub mod scl_main_st_time_out;
#[doc = "SCL_SP_CONF register accessor: an alias for `Reg<SCL_SP_CONF_SPEC>`"]
pub type SCL_SP_CONF = crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>;
#[doc = "I2C_SCL_SP_CONF"]
pub mod scl_sp_conf;
#[doc = "SCL_STRETCH_CONF register accessor: an alias for `Reg<SCL_STRETCH_CONF_SPEC>`"]
pub type SCL_STRETCH_CONF = crate::Reg<scl_stretch_conf::SCL_STRETCH_CONF_SPEC>;
#[doc = "I2C_SCL_STRETCH_CONF"]
pub mod scl_stretch_conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "I2C_DATE"]
pub mod date;
