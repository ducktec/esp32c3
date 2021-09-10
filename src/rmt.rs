#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Read and write data register for channel 0"]
    pub ch0data: crate::Reg<ch0data::CH0DATA_SPEC>,
    #[doc = "0x04 - Read and write data register for channel 1"]
    pub ch1data: crate::Reg<ch1data::CH1DATA_SPEC>,
    #[doc = "0x08 - Read and write data register for channel 2"]
    pub ch2data: crate::Reg<ch2data::CH2DATA_SPEC>,
    #[doc = "0x0c - Read and write data register for channel 3"]
    pub ch3data: crate::Reg<ch3data::CH3DATA_SPEC>,
    #[doc = "0x10 - RMT_CH0CONF0"]
    pub ch0conf0: crate::Reg<ch0conf0::CH0CONF0_SPEC>,
    #[doc = "0x14 - RMT_CH1CONF0"]
    pub ch1conf0: crate::Reg<ch1conf0::CH1CONF0_SPEC>,
    #[doc = "0x18 - RMT_CH2CONF0"]
    pub ch2conf0: crate::Reg<ch2conf0::CH2CONF0_SPEC>,
    #[doc = "0x1c - RMT_CH2CONF1"]
    pub ch2conf1: crate::Reg<ch2conf1::CH2CONF1_SPEC>,
    #[doc = "0x20 - RMT_CH3CONF0"]
    pub ch3conf0: crate::Reg<ch3conf0::CH3CONF0_SPEC>,
    #[doc = "0x24 - RMT_CH3CONF1"]
    pub ch3conf1: crate::Reg<ch3conf1::CH3CONF1_SPEC>,
    #[doc = "0x28 - RMT_CH0STATUS"]
    pub ch0status: crate::Reg<ch0status::CH0STATUS_SPEC>,
    #[doc = "0x2c - RMT_CH1STATUS"]
    pub ch1status: crate::Reg<ch1status::CH1STATUS_SPEC>,
    #[doc = "0x30 - RMT_CH2STATUS"]
    pub ch2status: crate::Reg<ch2status::CH2STATUS_SPEC>,
    #[doc = "0x34 - RMT_CH3STATUS"]
    pub ch3status: crate::Reg<ch3status::CH3STATUS_SPEC>,
    #[doc = "0x38 - RMT_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x3c - RMT_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x40 - RMT_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x44 - RMT_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x48 - RMT_CH0CARRIER_DUTY"]
    pub ch0carrier_duty: crate::Reg<ch0carrier_duty::CH0CARRIER_DUTY_SPEC>,
    #[doc = "0x4c - RMT_CH1CARRIER_DUTY"]
    pub ch1carrier_duty: crate::Reg<ch1carrier_duty::CH1CARRIER_DUTY_SPEC>,
    #[doc = "0x50 - RMT_CH2_RX_CARRIER_RM"]
    pub ch2_rx_carrier_rm: crate::Reg<ch2_rx_carrier_rm::CH2_RX_CARRIER_RM_SPEC>,
    #[doc = "0x54 - RMT_CH3_RX_CARRIER_RM"]
    pub ch3_rx_carrier_rm: crate::Reg<ch3_rx_carrier_rm::CH3_RX_CARRIER_RM_SPEC>,
    #[doc = "0x58 - RMT_CH0_TX_LIM"]
    pub ch0_tx_lim: crate::Reg<ch0_tx_lim::CH0_TX_LIM_SPEC>,
    #[doc = "0x5c - RMT_CH1_TX_LIM"]
    pub ch1_tx_lim: crate::Reg<ch1_tx_lim::CH1_TX_LIM_SPEC>,
    #[doc = "0x60 - RMT_CH2_RX_LIM"]
    pub ch2_rx_lim: crate::Reg<ch2_rx_lim::CH2_RX_LIM_SPEC>,
    #[doc = "0x64 - RMT_CH3_RX_LIM"]
    pub ch3_rx_lim: crate::Reg<ch3_rx_lim::CH3_RX_LIM_SPEC>,
    #[doc = "0x68 - RMT_SYS_CONF"]
    pub sys_conf: crate::Reg<sys_conf::SYS_CONF_SPEC>,
    #[doc = "0x6c - RMT_TX_SIM"]
    pub tx_sim: crate::Reg<tx_sim::TX_SIM_SPEC>,
    #[doc = "0x70 - RMT_REF_CNT_RST"]
    pub ref_cnt_rst: crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>,
    _reserved29: [u8; 0x58],
    #[doc = "0xcc - RMT_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CH0CONF0 register accessor: an alias for `Reg<CH0CONF0_SPEC>`"]
pub type CH0CONF0 = crate::Reg<ch0conf0::CH0CONF0_SPEC>;
#[doc = "RMT_CH0CONF0"]
pub mod ch0conf0;
#[doc = "CH1CONF0 register accessor: an alias for `Reg<CH1CONF0_SPEC>`"]
pub type CH1CONF0 = crate::Reg<ch1conf0::CH1CONF0_SPEC>;
#[doc = "RMT_CH1CONF0"]
pub mod ch1conf0;
#[doc = "CH2CONF0 register accessor: an alias for `Reg<CH2CONF0_SPEC>`"]
pub type CH2CONF0 = crate::Reg<ch2conf0::CH2CONF0_SPEC>;
#[doc = "RMT_CH2CONF0"]
pub mod ch2conf0;
#[doc = "CH2CONF1 register accessor: an alias for `Reg<CH2CONF1_SPEC>`"]
pub type CH2CONF1 = crate::Reg<ch2conf1::CH2CONF1_SPEC>;
#[doc = "RMT_CH2CONF1"]
pub mod ch2conf1;
#[doc = "CH3CONF0 register accessor: an alias for `Reg<CH3CONF0_SPEC>`"]
pub type CH3CONF0 = crate::Reg<ch3conf0::CH3CONF0_SPEC>;
#[doc = "RMT_CH3CONF0"]
pub mod ch3conf0;
#[doc = "CH3CONF1 register accessor: an alias for `Reg<CH3CONF1_SPEC>`"]
pub type CH3CONF1 = crate::Reg<ch3conf1::CH3CONF1_SPEC>;
#[doc = "RMT_CH3CONF1"]
pub mod ch3conf1;
#[doc = "CH0STATUS register accessor: an alias for `Reg<CH0STATUS_SPEC>`"]
pub type CH0STATUS = crate::Reg<ch0status::CH0STATUS_SPEC>;
#[doc = "RMT_CH0STATUS"]
pub mod ch0status;
#[doc = "CH1STATUS register accessor: an alias for `Reg<CH1STATUS_SPEC>`"]
pub type CH1STATUS = crate::Reg<ch1status::CH1STATUS_SPEC>;
#[doc = "RMT_CH1STATUS"]
pub mod ch1status;
#[doc = "CH2STATUS register accessor: an alias for `Reg<CH2STATUS_SPEC>`"]
pub type CH2STATUS = crate::Reg<ch2status::CH2STATUS_SPEC>;
#[doc = "RMT_CH2STATUS"]
pub mod ch2status;
#[doc = "CH3STATUS register accessor: an alias for `Reg<CH3STATUS_SPEC>`"]
pub type CH3STATUS = crate::Reg<ch3status::CH3STATUS_SPEC>;
#[doc = "RMT_CH3STATUS"]
pub mod ch3status;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "RMT_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "RMT_INT_ST"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "RMT_INT_ENA"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "RMT_INT_CLR"]
pub mod int_clr;
#[doc = "CH0CARRIER_DUTY register accessor: an alias for `Reg<CH0CARRIER_DUTY_SPEC>`"]
pub type CH0CARRIER_DUTY = crate::Reg<ch0carrier_duty::CH0CARRIER_DUTY_SPEC>;
#[doc = "RMT_CH0CARRIER_DUTY"]
pub mod ch0carrier_duty;
#[doc = "CH1CARRIER_DUTY register accessor: an alias for `Reg<CH1CARRIER_DUTY_SPEC>`"]
pub type CH1CARRIER_DUTY = crate::Reg<ch1carrier_duty::CH1CARRIER_DUTY_SPEC>;
#[doc = "RMT_CH1CARRIER_DUTY"]
pub mod ch1carrier_duty;
#[doc = "CH2_RX_CARRIER_RM register accessor: an alias for `Reg<CH2_RX_CARRIER_RM_SPEC>`"]
pub type CH2_RX_CARRIER_RM = crate::Reg<ch2_rx_carrier_rm::CH2_RX_CARRIER_RM_SPEC>;
#[doc = "RMT_CH2_RX_CARRIER_RM"]
pub mod ch2_rx_carrier_rm;
#[doc = "CH3_RX_CARRIER_RM register accessor: an alias for `Reg<CH3_RX_CARRIER_RM_SPEC>`"]
pub type CH3_RX_CARRIER_RM = crate::Reg<ch3_rx_carrier_rm::CH3_RX_CARRIER_RM_SPEC>;
#[doc = "RMT_CH3_RX_CARRIER_RM"]
pub mod ch3_rx_carrier_rm;
#[doc = "CH0_TX_LIM register accessor: an alias for `Reg<CH0_TX_LIM_SPEC>`"]
pub type CH0_TX_LIM = crate::Reg<ch0_tx_lim::CH0_TX_LIM_SPEC>;
#[doc = "RMT_CH0_TX_LIM"]
pub mod ch0_tx_lim;
#[doc = "CH1_TX_LIM register accessor: an alias for `Reg<CH1_TX_LIM_SPEC>`"]
pub type CH1_TX_LIM = crate::Reg<ch1_tx_lim::CH1_TX_LIM_SPEC>;
#[doc = "RMT_CH1_TX_LIM"]
pub mod ch1_tx_lim;
#[doc = "CH2_RX_LIM register accessor: an alias for `Reg<CH2_RX_LIM_SPEC>`"]
pub type CH2_RX_LIM = crate::Reg<ch2_rx_lim::CH2_RX_LIM_SPEC>;
#[doc = "RMT_CH2_RX_LIM"]
pub mod ch2_rx_lim;
#[doc = "CH3_RX_LIM register accessor: an alias for `Reg<CH3_RX_LIM_SPEC>`"]
pub type CH3_RX_LIM = crate::Reg<ch3_rx_lim::CH3_RX_LIM_SPEC>;
#[doc = "RMT_CH3_RX_LIM"]
pub mod ch3_rx_lim;
#[doc = "SYS_CONF register accessor: an alias for `Reg<SYS_CONF_SPEC>`"]
pub type SYS_CONF = crate::Reg<sys_conf::SYS_CONF_SPEC>;
#[doc = "RMT_SYS_CONF"]
pub mod sys_conf;
#[doc = "TX_SIM register accessor: an alias for `Reg<TX_SIM_SPEC>`"]
pub type TX_SIM = crate::Reg<tx_sim::TX_SIM_SPEC>;
#[doc = "RMT_TX_SIM"]
pub mod tx_sim;
#[doc = "REF_CNT_RST register accessor: an alias for `Reg<REF_CNT_RST_SPEC>`"]
pub type REF_CNT_RST = crate::Reg<ref_cnt_rst::REF_CNT_RST_SPEC>;
#[doc = "RMT_REF_CNT_RST"]
pub mod ref_cnt_rst;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "RMT_DATE"]
pub mod date;
#[doc = "CH0DATA register accessor: an alias for `Reg<CH0DATA_SPEC>`"]
pub type CH0DATA = crate::Reg<ch0data::CH0DATA_SPEC>;
#[doc = "Read and write data register for channel 0"]
pub mod ch0data;
#[doc = "CH1DATA register accessor: an alias for `Reg<CH1DATA_SPEC>`"]
pub type CH1DATA = crate::Reg<ch1data::CH1DATA_SPEC>;
#[doc = "Read and write data register for channel 1"]
pub mod ch1data;
#[doc = "CH2DATA register accessor: an alias for `Reg<CH2DATA_SPEC>`"]
pub type CH2DATA = crate::Reg<ch2data::CH2DATA_SPEC>;
#[doc = "Read and write data register for channel 2"]
pub mod ch2data;
#[doc = "CH3DATA register accessor: an alias for `Reg<CH3DATA_SPEC>`"]
pub type CH3DATA = crate::Reg<ch3data::CH3DATA_SPEC>;
#[doc = "Read and write data register for channel 3"]
pub mod ch3data;
