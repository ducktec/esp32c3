#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_I2C_SCL_LOW_PERIOD"]
    pub scl_low_period: crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>,
    #[doc = "0x04 - RTC_I2C_CTRL"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x08 - RTC_I2C_STATUS"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x0c - RTC_I2C_TIMEOUT"]
    pub timeout: crate::Reg<timeout::TIMEOUT_SPEC>,
    #[doc = "0x10 - RTC_I2C_SLAVE_ADDR"]
    pub slave_addr: crate::Reg<slave_addr::SLAVE_ADDR_SPEC>,
    #[doc = "0x14 - RTC_I2C_SCL_HIGH"]
    pub scl_high: crate::Reg<scl_high::SCL_HIGH_SPEC>,
    #[doc = "0x18 - RTC_I2C_SDA_DUTY"]
    pub sda_duty: crate::Reg<sda_duty::SDA_DUTY_SPEC>,
    #[doc = "0x1c - RTC_I2C_SCL_START_PERIOD"]
    pub scl_start_period: crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>,
    #[doc = "0x20 - RTC_I2C_SCL_STOP_PERIOD"]
    pub scl_stop_period: crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>,
    #[doc = "0x24 - RTC_I2C_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x28 - RTC_I2C_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x2c - RTC_I2C_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x30 - RTC_I2C_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x34 - RTC_I2C_DATA"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x38 - RTC_I2C_CMD0"]
    pub cmd0: crate::Reg<cmd0::CMD0_SPEC>,
    #[doc = "0x3c - RTC_I2C_CMD1"]
    pub cmd1: crate::Reg<cmd1::CMD1_SPEC>,
    #[doc = "0x40 - RTC_I2C_CMD2"]
    pub cmd2: crate::Reg<cmd2::CMD2_SPEC>,
    #[doc = "0x44 - RTC_I2C_CMD3"]
    pub cmd3: crate::Reg<cmd3::CMD3_SPEC>,
    #[doc = "0x48 - RTC_I2C_CMD4"]
    pub cmd4: crate::Reg<cmd4::CMD4_SPEC>,
    #[doc = "0x4c - RTC_I2C_CMD5"]
    pub cmd5: crate::Reg<cmd5::CMD5_SPEC>,
    #[doc = "0x50 - RTC_I2C_CMD6"]
    pub cmd6: crate::Reg<cmd6::CMD6_SPEC>,
    #[doc = "0x54 - RTC_I2C_CMD7"]
    pub cmd7: crate::Reg<cmd7::CMD7_SPEC>,
    #[doc = "0x58 - RTC_I2C_CMD8"]
    pub cmd8: crate::Reg<cmd8::CMD8_SPEC>,
    #[doc = "0x5c - RTC_I2C_CMD9"]
    pub cmd9: crate::Reg<cmd9::CMD9_SPEC>,
    #[doc = "0x60 - RTC_I2C_CMD10"]
    pub cmd10: crate::Reg<cmd10::CMD10_SPEC>,
    #[doc = "0x64 - RTC_I2C_CMD11"]
    pub cmd11: crate::Reg<cmd11::CMD11_SPEC>,
    #[doc = "0x68 - RTC_I2C_CMD12"]
    pub cmd12: crate::Reg<cmd12::CMD12_SPEC>,
    #[doc = "0x6c - RTC_I2C_CMD13"]
    pub cmd13: crate::Reg<cmd13::CMD13_SPEC>,
    #[doc = "0x70 - RTC_I2C_CMD14"]
    pub cmd14: crate::Reg<cmd14::CMD14_SPEC>,
    #[doc = "0x74 - RTC_I2C_CMD15"]
    pub cmd15: crate::Reg<cmd15::CMD15_SPEC>,
    _reserved30: [u8; 0x84],
    #[doc = "0xfc - RTC_I2C_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SCL_LOW_PERIOD register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = "RTC_I2C_SCL_LOW_PERIOD"]
pub mod scl_low_period;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC_I2C_CTRL"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RTC_I2C_STATUS"]
pub mod status;
#[doc = "TIMEOUT register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "RTC_I2C_TIMEOUT"]
pub mod timeout;
#[doc = "SLAVE_ADDR register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "RTC_I2C_SLAVE_ADDR"]
pub mod slave_addr;
#[doc = "SCL_HIGH register accessor: an alias for `Reg<SCL_HIGH_SPEC>`"]
pub type SCL_HIGH = crate::Reg<scl_high::SCL_HIGH_SPEC>;
#[doc = "RTC_I2C_SCL_HIGH"]
pub mod scl_high;
#[doc = "SDA_DUTY register accessor: an alias for `Reg<SDA_DUTY_SPEC>`"]
pub type SDA_DUTY = crate::Reg<sda_duty::SDA_DUTY_SPEC>;
#[doc = "RTC_I2C_SDA_DUTY"]
pub mod sda_duty;
#[doc = "SCL_START_PERIOD register accessor: an alias for `Reg<SCL_START_PERIOD_SPEC>`"]
pub type SCL_START_PERIOD = crate::Reg<scl_start_period::SCL_START_PERIOD_SPEC>;
#[doc = "RTC_I2C_SCL_START_PERIOD"]
pub mod scl_start_period;
#[doc = "SCL_STOP_PERIOD register accessor: an alias for `Reg<SCL_STOP_PERIOD_SPEC>`"]
pub type SCL_STOP_PERIOD = crate::Reg<scl_stop_period::SCL_STOP_PERIOD_SPEC>;
#[doc = "RTC_I2C_SCL_STOP_PERIOD"]
pub mod scl_stop_period;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RTC_I2C_INT_CLR"]
pub mod int_clr;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RTC_I2C_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RTC_I2C_INT_ST"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RTC_I2C_INT_ENA"]
pub mod int_ena;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "RTC_I2C_DATA"]
pub mod data;
#[doc = "CMD0 register accessor: an alias for `Reg<CMD0_SPEC>`"]
pub type CMD0 = crate::Reg<cmd0::CMD0_SPEC>;
#[doc = "RTC_I2C_CMD0"]
pub mod cmd0;
#[doc = "CMD1 register accessor: an alias for `Reg<CMD1_SPEC>`"]
pub type CMD1 = crate::Reg<cmd1::CMD1_SPEC>;
#[doc = "RTC_I2C_CMD1"]
pub mod cmd1;
#[doc = "CMD2 register accessor: an alias for `Reg<CMD2_SPEC>`"]
pub type CMD2 = crate::Reg<cmd2::CMD2_SPEC>;
#[doc = "RTC_I2C_CMD2"]
pub mod cmd2;
#[doc = "CMD3 register accessor: an alias for `Reg<CMD3_SPEC>`"]
pub type CMD3 = crate::Reg<cmd3::CMD3_SPEC>;
#[doc = "RTC_I2C_CMD3"]
pub mod cmd3;
#[doc = "CMD4 register accessor: an alias for `Reg<CMD4_SPEC>`"]
pub type CMD4 = crate::Reg<cmd4::CMD4_SPEC>;
#[doc = "RTC_I2C_CMD4"]
pub mod cmd4;
#[doc = "CMD5 register accessor: an alias for `Reg<CMD5_SPEC>`"]
pub type CMD5 = crate::Reg<cmd5::CMD5_SPEC>;
#[doc = "RTC_I2C_CMD5"]
pub mod cmd5;
#[doc = "CMD6 register accessor: an alias for `Reg<CMD6_SPEC>`"]
pub type CMD6 = crate::Reg<cmd6::CMD6_SPEC>;
#[doc = "RTC_I2C_CMD6"]
pub mod cmd6;
#[doc = "CMD7 register accessor: an alias for `Reg<CMD7_SPEC>`"]
pub type CMD7 = crate::Reg<cmd7::CMD7_SPEC>;
#[doc = "RTC_I2C_CMD7"]
pub mod cmd7;
#[doc = "CMD8 register accessor: an alias for `Reg<CMD8_SPEC>`"]
pub type CMD8 = crate::Reg<cmd8::CMD8_SPEC>;
#[doc = "RTC_I2C_CMD8"]
pub mod cmd8;
#[doc = "CMD9 register accessor: an alias for `Reg<CMD9_SPEC>`"]
pub type CMD9 = crate::Reg<cmd9::CMD9_SPEC>;
#[doc = "RTC_I2C_CMD9"]
pub mod cmd9;
#[doc = "CMD10 register accessor: an alias for `Reg<CMD10_SPEC>`"]
pub type CMD10 = crate::Reg<cmd10::CMD10_SPEC>;
#[doc = "RTC_I2C_CMD10"]
pub mod cmd10;
#[doc = "CMD11 register accessor: an alias for `Reg<CMD11_SPEC>`"]
pub type CMD11 = crate::Reg<cmd11::CMD11_SPEC>;
#[doc = "RTC_I2C_CMD11"]
pub mod cmd11;
#[doc = "CMD12 register accessor: an alias for `Reg<CMD12_SPEC>`"]
pub type CMD12 = crate::Reg<cmd12::CMD12_SPEC>;
#[doc = "RTC_I2C_CMD12"]
pub mod cmd12;
#[doc = "CMD13 register accessor: an alias for `Reg<CMD13_SPEC>`"]
pub type CMD13 = crate::Reg<cmd13::CMD13_SPEC>;
#[doc = "RTC_I2C_CMD13"]
pub mod cmd13;
#[doc = "CMD14 register accessor: an alias for `Reg<CMD14_SPEC>`"]
pub type CMD14 = crate::Reg<cmd14::CMD14_SPEC>;
#[doc = "RTC_I2C_CMD14"]
pub mod cmd14;
#[doc = "CMD15 register accessor: an alias for `Reg<CMD15_SPEC>`"]
pub type CMD15 = crate::Reg<cmd15::CMD15_SPEC>;
#[doc = "RTC_I2C_CMD15"]
pub mod cmd15;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RTC_I2C_DATE"]
pub mod date;
