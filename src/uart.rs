#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART_FIFO"]
    pub fifo: crate::Reg<fifo::FIFO_SPEC>,
    #[doc = "0x04 - UART_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x08 - UART_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x0c - UART_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x10 - UART_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x14 - UART_CLKDIV"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x18 - UART_RX_FILT"]
    pub rx_filt: crate::Reg<rx_filt::RX_FILT_SPEC>,
    #[doc = "0x1c - UART_STATUS"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x20 - UART_CONF0"]
    pub conf0: crate::Reg<conf0::CONF0_SPEC>,
    #[doc = "0x24 - UART_CONF1"]
    pub conf1: crate::Reg<conf1::CONF1_SPEC>,
    #[doc = "0x28 - UART_LOWPULSE"]
    pub lowpulse: crate::Reg<lowpulse::LOWPULSE_SPEC>,
    #[doc = "0x2c - UART_HIGHPULSE"]
    pub highpulse: crate::Reg<highpulse::HIGHPULSE_SPEC>,
    #[doc = "0x30 - UART_RXD_CNT"]
    pub rxd_cnt: crate::Reg<rxd_cnt::RXD_CNT_SPEC>,
    #[doc = "0x34 - UART_FLOW_CONF"]
    pub flow_conf: crate::Reg<flow_conf::FLOW_CONF_SPEC>,
    #[doc = "0x38 - UART_SLEEP_CONF"]
    pub sleep_conf: crate::Reg<sleep_conf::SLEEP_CONF_SPEC>,
    #[doc = "0x3c - UART_SWFC_CONF0"]
    pub swfc_conf0: crate::Reg<swfc_conf0::SWFC_CONF0_SPEC>,
    #[doc = "0x40 - UART_SWFC_CONF1"]
    pub swfc_conf1: crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>,
    #[doc = "0x44 - UART_TXBRK_CONF"]
    pub txbrk_conf: crate::Reg<txbrk_conf::TXBRK_CONF_SPEC>,
    #[doc = "0x48 - UART_IDLE_CONF"]
    pub idle_conf: crate::Reg<idle_conf::IDLE_CONF_SPEC>,
    #[doc = "0x4c - UART_RS485_CONF"]
    pub rs485_conf: crate::Reg<rs485_conf::RS485_CONF_SPEC>,
    #[doc = "0x50 - UART_AT_CMD_PRECNT"]
    pub at_cmd_precnt: crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>,
    #[doc = "0x54 - UART_AT_CMD_POSTCNT"]
    pub at_cmd_postcnt: crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>,
    #[doc = "0x58 - UART_AT_CMD_GAPTOUT"]
    pub at_cmd_gaptout: crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>,
    #[doc = "0x5c - UART_AT_CMD_CHAR"]
    pub at_cmd_char: crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>,
    #[doc = "0x60 - UART_MEM_CONF"]
    pub mem_conf: crate::Reg<mem_conf::MEM_CONF_SPEC>,
    #[doc = "0x64 - UART_MEM_TX_STATUS"]
    pub mem_tx_status: crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>,
    #[doc = "0x68 - UART_MEM_RX_STATUS"]
    pub mem_rx_status: crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>,
    #[doc = "0x6c - UART_FSM_STATUS"]
    pub fsm_status: crate::Reg<fsm_status::FSM_STATUS_SPEC>,
    #[doc = "0x70 - UART_POSPULSE"]
    pub pospulse: crate::Reg<pospulse::POSPULSE_SPEC>,
    #[doc = "0x74 - UART_NEGPULSE"]
    pub negpulse: crate::Reg<negpulse::NEGPULSE_SPEC>,
    #[doc = "0x78 - UART_CLK_CONF"]
    pub clk_conf: crate::Reg<clk_conf::CLK_CONF_SPEC>,
    #[doc = "0x7c - UART_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
    #[doc = "0x80 - UART_ID"]
    pub id: crate::Reg<id::ID_SPEC>,
}
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "UART_FIFO"]
pub mod fifo;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "UART_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "UART_INT_ST"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "UART_INT_ENA"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "UART_INT_CLR"]
pub mod int_clr;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "UART_CLKDIV"]
pub mod clkdiv;
#[doc = "RX_FILT register accessor: an alias for `Reg<RX_FILT_SPEC>`"]
pub type RX_FILT = crate::Reg<rx_filt::RX_FILT_SPEC>;
#[doc = "UART_RX_FILT"]
pub mod rx_filt;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "UART_STATUS"]
pub mod status;
#[doc = "CONF0 register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "UART_CONF0"]
pub mod conf0;
#[doc = "CONF1 register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "UART_CONF1"]
pub mod conf1;
#[doc = "LOWPULSE register accessor: an alias for `Reg<LOWPULSE_SPEC>`"]
pub type LOWPULSE = crate::Reg<lowpulse::LOWPULSE_SPEC>;
#[doc = "UART_LOWPULSE"]
pub mod lowpulse;
#[doc = "HIGHPULSE register accessor: an alias for `Reg<HIGHPULSE_SPEC>`"]
pub type HIGHPULSE = crate::Reg<highpulse::HIGHPULSE_SPEC>;
#[doc = "UART_HIGHPULSE"]
pub mod highpulse;
#[doc = "RXD_CNT register accessor: an alias for `Reg<RXD_CNT_SPEC>`"]
pub type RXD_CNT = crate::Reg<rxd_cnt::RXD_CNT_SPEC>;
#[doc = "UART_RXD_CNT"]
pub mod rxd_cnt;
#[doc = "FLOW_CONF register accessor: an alias for `Reg<FLOW_CONF_SPEC>`"]
pub type FLOW_CONF = crate::Reg<flow_conf::FLOW_CONF_SPEC>;
#[doc = "UART_FLOW_CONF"]
pub mod flow_conf;
#[doc = "SLEEP_CONF register accessor: an alias for `Reg<SLEEP_CONF_SPEC>`"]
pub type SLEEP_CONF = crate::Reg<sleep_conf::SLEEP_CONF_SPEC>;
#[doc = "UART_SLEEP_CONF"]
pub mod sleep_conf;
#[doc = "SWFC_CONF0 register accessor: an alias for `Reg<SWFC_CONF0_SPEC>`"]
pub type SWFC_CONF0 = crate::Reg<swfc_conf0::SWFC_CONF0_SPEC>;
#[doc = "UART_SWFC_CONF0"]
pub mod swfc_conf0;
#[doc = "SWFC_CONF1 register accessor: an alias for `Reg<SWFC_CONF1_SPEC>`"]
pub type SWFC_CONF1 = crate::Reg<swfc_conf1::SWFC_CONF1_SPEC>;
#[doc = "UART_SWFC_CONF1"]
pub mod swfc_conf1;
#[doc = "TXBRK_CONF register accessor: an alias for `Reg<TXBRK_CONF_SPEC>`"]
pub type TXBRK_CONF = crate::Reg<txbrk_conf::TXBRK_CONF_SPEC>;
#[doc = "UART_TXBRK_CONF"]
pub mod txbrk_conf;
#[doc = "IDLE_CONF register accessor: an alias for `Reg<IDLE_CONF_SPEC>`"]
pub type IDLE_CONF = crate::Reg<idle_conf::IDLE_CONF_SPEC>;
#[doc = "UART_IDLE_CONF"]
pub mod idle_conf;
#[doc = "RS485_CONF register accessor: an alias for `Reg<RS485_CONF_SPEC>`"]
pub type RS485_CONF = crate::Reg<rs485_conf::RS485_CONF_SPEC>;
#[doc = "UART_RS485_CONF"]
pub mod rs485_conf;
#[doc = "AT_CMD_PRECNT register accessor: an alias for `Reg<AT_CMD_PRECNT_SPEC>`"]
pub type AT_CMD_PRECNT = crate::Reg<at_cmd_precnt::AT_CMD_PRECNT_SPEC>;
#[doc = "UART_AT_CMD_PRECNT"]
pub mod at_cmd_precnt;
#[doc = "AT_CMD_POSTCNT register accessor: an alias for `Reg<AT_CMD_POSTCNT_SPEC>`"]
pub type AT_CMD_POSTCNT = crate::Reg<at_cmd_postcnt::AT_CMD_POSTCNT_SPEC>;
#[doc = "UART_AT_CMD_POSTCNT"]
pub mod at_cmd_postcnt;
#[doc = "AT_CMD_GAPTOUT register accessor: an alias for `Reg<AT_CMD_GAPTOUT_SPEC>`"]
pub type AT_CMD_GAPTOUT = crate::Reg<at_cmd_gaptout::AT_CMD_GAPTOUT_SPEC>;
#[doc = "UART_AT_CMD_GAPTOUT"]
pub mod at_cmd_gaptout;
#[doc = "AT_CMD_CHAR register accessor: an alias for `Reg<AT_CMD_CHAR_SPEC>`"]
pub type AT_CMD_CHAR = crate::Reg<at_cmd_char::AT_CMD_CHAR_SPEC>;
#[doc = "UART_AT_CMD_CHAR"]
pub mod at_cmd_char;
#[doc = "MEM_CONF register accessor: an alias for `Reg<MEM_CONF_SPEC>`"]
pub type MEM_CONF = crate::Reg<mem_conf::MEM_CONF_SPEC>;
#[doc = "UART_MEM_CONF"]
pub mod mem_conf;
#[doc = "MEM_TX_STATUS register accessor: an alias for `Reg<MEM_TX_STATUS_SPEC>`"]
pub type MEM_TX_STATUS = crate::Reg<mem_tx_status::MEM_TX_STATUS_SPEC>;
#[doc = "UART_MEM_TX_STATUS"]
pub mod mem_tx_status;
#[doc = "MEM_RX_STATUS register accessor: an alias for `Reg<MEM_RX_STATUS_SPEC>`"]
pub type MEM_RX_STATUS = crate::Reg<mem_rx_status::MEM_RX_STATUS_SPEC>;
#[doc = "UART_MEM_RX_STATUS"]
pub mod mem_rx_status;
#[doc = "FSM_STATUS register accessor: an alias for `Reg<FSM_STATUS_SPEC>`"]
pub type FSM_STATUS = crate::Reg<fsm_status::FSM_STATUS_SPEC>;
#[doc = "UART_FSM_STATUS"]
pub mod fsm_status;
#[doc = "POSPULSE register accessor: an alias for `Reg<POSPULSE_SPEC>`"]
pub type POSPULSE = crate::Reg<pospulse::POSPULSE_SPEC>;
#[doc = "UART_POSPULSE"]
pub mod pospulse;
#[doc = "NEGPULSE register accessor: an alias for `Reg<NEGPULSE_SPEC>`"]
pub type NEGPULSE = crate::Reg<negpulse::NEGPULSE_SPEC>;
#[doc = "UART_NEGPULSE"]
pub mod negpulse;
#[doc = "CLK_CONF register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "UART_CLK_CONF"]
pub mod clk_conf;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UART_DATE"]
pub mod date;
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "UART_ID"]
pub mod id;
