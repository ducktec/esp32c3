#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_MEM_CMD"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x04 - SPI_MEM_ADDR"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x08 - SPI_MEM_CTRL"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x0c - SPI_MEM_CTRL1"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x10 - SPI_MEM_CTRL2"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x14 - SPI_MEM_CLOCK"]
    pub clock: crate::Reg<clock::CLOCK_SPEC>,
    #[doc = "0x18 - SPI_MEM_USER"]
    pub user: crate::Reg<user::USER_SPEC>,
    #[doc = "0x1c - SPI_MEM_USER1"]
    pub user1: crate::Reg<user1::USER1_SPEC>,
    #[doc = "0x20 - SPI_MEM_USER2"]
    pub user2: crate::Reg<user2::USER2_SPEC>,
    #[doc = "0x24 - SPI_MEM_MOSI_DLEN"]
    pub mosi_dlen: crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>,
    #[doc = "0x28 - SPI_MEM_MISO_DLEN"]
    pub miso_dlen: crate::Reg<miso_dlen::MISO_DLEN_SPEC>,
    #[doc = "0x2c - SPI_MEM_RD_STATUS"]
    pub rd_status: crate::Reg<rd_status::RD_STATUS_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - SPI_MEM_MISC"]
    pub misc: crate::Reg<misc::MISC_SPEC>,
    #[doc = "0x38 - SPI_MEM_TX_CRC"]
    pub tx_crc: crate::Reg<tx_crc::TX_CRC_SPEC>,
    #[doc = "0x3c - SPI_MEM_CACHE_FCTRL"]
    pub cache_fctrl: crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>,
    _reserved15: [u8; 0x14],
    #[doc = "0x54 - SPI_MEM_FSM"]
    pub fsm: crate::Reg<fsm::FSM_SPEC>,
    #[doc = "0x58 - SPI_MEM_W0"]
    pub w0: crate::Reg<w0::W0_SPEC>,
    #[doc = "0x5c - SPI_MEM_W1"]
    pub w1: crate::Reg<w1::W1_SPEC>,
    #[doc = "0x60 - SPI_MEM_W2"]
    pub w2: crate::Reg<w2::W2_SPEC>,
    #[doc = "0x64 - SPI_MEM_W3"]
    pub w3: crate::Reg<w3::W3_SPEC>,
    #[doc = "0x68 - SPI_MEM_W4"]
    pub w4: crate::Reg<w4::W4_SPEC>,
    #[doc = "0x6c - SPI_MEM_W5"]
    pub w5: crate::Reg<w5::W5_SPEC>,
    #[doc = "0x70 - SPI_MEM_W6"]
    pub w6: crate::Reg<w6::W6_SPEC>,
    #[doc = "0x74 - SPI_MEM_W7"]
    pub w7: crate::Reg<w7::W7_SPEC>,
    #[doc = "0x78 - SPI_MEM_W8"]
    pub w8: crate::Reg<w8::W8_SPEC>,
    #[doc = "0x7c - SPI_MEM_W9"]
    pub w9: crate::Reg<w9::W9_SPEC>,
    #[doc = "0x80 - SPI_MEM_W10"]
    pub w10: crate::Reg<w10::W10_SPEC>,
    #[doc = "0x84 - SPI_MEM_W11"]
    pub w11: crate::Reg<w11::W11_SPEC>,
    #[doc = "0x88 - SPI_MEM_W12"]
    pub w12: crate::Reg<w12::W12_SPEC>,
    #[doc = "0x8c - SPI_MEM_W13"]
    pub w13: crate::Reg<w13::W13_SPEC>,
    #[doc = "0x90 - SPI_MEM_W14"]
    pub w14: crate::Reg<w14::W14_SPEC>,
    #[doc = "0x94 - SPI_MEM_W15"]
    pub w15: crate::Reg<w15::W15_SPEC>,
    #[doc = "0x98 - SPI_MEM_FLASH_WAITI_CTRL"]
    pub flash_waiti_ctrl: crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>,
    #[doc = "0x9c - SPI_MEM_FLASH_SUS_CTRL"]
    pub flash_sus_ctrl: crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>,
    #[doc = "0xa0 - SPI_MEM_FLASH_SUS_CMD"]
    pub flash_sus_cmd: crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>,
    #[doc = "0xa4 - SPI_MEM_SUS_STATUS"]
    pub sus_status: crate::Reg<sus_status::SUS_STATUS_SPEC>,
    #[doc = "0xa8 - SPI_MEM_TIMING_CALI"]
    pub timing_cali: crate::Reg<timing_cali::TIMING_CALI_SPEC>,
    #[doc = "0xac - SPI_MEM_DIN_MODE"]
    pub din_mode: crate::Reg<din_mode::DIN_MODE_SPEC>,
    #[doc = "0xb0 - SPI_MEM_DIN_NUM"]
    pub din_num: crate::Reg<din_num::DIN_NUM_SPEC>,
    #[doc = "0xb4 - SPI_MEM_DOUT_MODE"]
    pub dout_mode: crate::Reg<dout_mode::DOUT_MODE_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0xc0 - SPI_MEM_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0xc4 - SPI_MEM_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0xc8 - SPI_MEM_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0xcc - SPI_MEM_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0xdc - SPI_MEM_CLOCK_GATE"]
    pub clock_gate: crate::Reg<clock_gate::CLOCK_GATE_SPEC>,
    #[doc = "0xe0 - SPI_MEM_CORE_CLK_SEL"]
    pub core_clk_sel: crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>,
    _reserved46: [u8; 0x0318],
    #[doc = "0x3fc - SPI_MEM_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "SPI_MEM_CMD"]
pub mod cmd;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "SPI_MEM_ADDR"]
pub mod addr;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SPI_MEM_CTRL"]
pub mod ctrl;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "SPI_MEM_CTRL1"]
pub mod ctrl1;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "SPI_MEM_CTRL2"]
pub mod ctrl2;
#[doc = "CLOCK register accessor: an alias for `Reg<CLOCK_SPEC>`"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "SPI_MEM_CLOCK"]
pub mod clock;
#[doc = "USER register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "SPI_MEM_USER"]
pub mod user;
#[doc = "USER1 register accessor: an alias for `Reg<USER1_SPEC>`"]
pub type USER1 = crate::Reg<user1::USER1_SPEC>;
#[doc = "SPI_MEM_USER1"]
pub mod user1;
#[doc = "USER2 register accessor: an alias for `Reg<USER2_SPEC>`"]
pub type USER2 = crate::Reg<user2::USER2_SPEC>;
#[doc = "SPI_MEM_USER2"]
pub mod user2;
#[doc = "MOSI_DLEN register accessor: an alias for `Reg<MOSI_DLEN_SPEC>`"]
pub type MOSI_DLEN = crate::Reg<mosi_dlen::MOSI_DLEN_SPEC>;
#[doc = "SPI_MEM_MOSI_DLEN"]
pub mod mosi_dlen;
#[doc = "MISO_DLEN register accessor: an alias for `Reg<MISO_DLEN_SPEC>`"]
pub type MISO_DLEN = crate::Reg<miso_dlen::MISO_DLEN_SPEC>;
#[doc = "SPI_MEM_MISO_DLEN"]
pub mod miso_dlen;
#[doc = "RD_STATUS register accessor: an alias for `Reg<RD_STATUS_SPEC>`"]
pub type RD_STATUS = crate::Reg<rd_status::RD_STATUS_SPEC>;
#[doc = "SPI_MEM_RD_STATUS"]
pub mod rd_status;
#[doc = "MISC register accessor: an alias for `Reg<MISC_SPEC>`"]
pub type MISC = crate::Reg<misc::MISC_SPEC>;
#[doc = "SPI_MEM_MISC"]
pub mod misc;
#[doc = "TX_CRC register accessor: an alias for `Reg<TX_CRC_SPEC>`"]
pub type TX_CRC = crate::Reg<tx_crc::TX_CRC_SPEC>;
#[doc = "SPI_MEM_TX_CRC"]
pub mod tx_crc;
#[doc = "CACHE_FCTRL register accessor: an alias for `Reg<CACHE_FCTRL_SPEC>`"]
pub type CACHE_FCTRL = crate::Reg<cache_fctrl::CACHE_FCTRL_SPEC>;
#[doc = "SPI_MEM_CACHE_FCTRL"]
pub mod cache_fctrl;
#[doc = "FSM register accessor: an alias for `Reg<FSM_SPEC>`"]
pub type FSM = crate::Reg<fsm::FSM_SPEC>;
#[doc = "SPI_MEM_FSM"]
pub mod fsm;
#[doc = "W0 register accessor: an alias for `Reg<W0_SPEC>`"]
pub type W0 = crate::Reg<w0::W0_SPEC>;
#[doc = "SPI_MEM_W0"]
pub mod w0;
#[doc = "W1 register accessor: an alias for `Reg<W1_SPEC>`"]
pub type W1 = crate::Reg<w1::W1_SPEC>;
#[doc = "SPI_MEM_W1"]
pub mod w1;
#[doc = "W2 register accessor: an alias for `Reg<W2_SPEC>`"]
pub type W2 = crate::Reg<w2::W2_SPEC>;
#[doc = "SPI_MEM_W2"]
pub mod w2;
#[doc = "W3 register accessor: an alias for `Reg<W3_SPEC>`"]
pub type W3 = crate::Reg<w3::W3_SPEC>;
#[doc = "SPI_MEM_W3"]
pub mod w3;
#[doc = "W4 register accessor: an alias for `Reg<W4_SPEC>`"]
pub type W4 = crate::Reg<w4::W4_SPEC>;
#[doc = "SPI_MEM_W4"]
pub mod w4;
#[doc = "W5 register accessor: an alias for `Reg<W5_SPEC>`"]
pub type W5 = crate::Reg<w5::W5_SPEC>;
#[doc = "SPI_MEM_W5"]
pub mod w5;
#[doc = "W6 register accessor: an alias for `Reg<W6_SPEC>`"]
pub type W6 = crate::Reg<w6::W6_SPEC>;
#[doc = "SPI_MEM_W6"]
pub mod w6;
#[doc = "W7 register accessor: an alias for `Reg<W7_SPEC>`"]
pub type W7 = crate::Reg<w7::W7_SPEC>;
#[doc = "SPI_MEM_W7"]
pub mod w7;
#[doc = "W8 register accessor: an alias for `Reg<W8_SPEC>`"]
pub type W8 = crate::Reg<w8::W8_SPEC>;
#[doc = "SPI_MEM_W8"]
pub mod w8;
#[doc = "W9 register accessor: an alias for `Reg<W9_SPEC>`"]
pub type W9 = crate::Reg<w9::W9_SPEC>;
#[doc = "SPI_MEM_W9"]
pub mod w9;
#[doc = "W10 register accessor: an alias for `Reg<W10_SPEC>`"]
pub type W10 = crate::Reg<w10::W10_SPEC>;
#[doc = "SPI_MEM_W10"]
pub mod w10;
#[doc = "W11 register accessor: an alias for `Reg<W11_SPEC>`"]
pub type W11 = crate::Reg<w11::W11_SPEC>;
#[doc = "SPI_MEM_W11"]
pub mod w11;
#[doc = "W12 register accessor: an alias for `Reg<W12_SPEC>`"]
pub type W12 = crate::Reg<w12::W12_SPEC>;
#[doc = "SPI_MEM_W12"]
pub mod w12;
#[doc = "W13 register accessor: an alias for `Reg<W13_SPEC>`"]
pub type W13 = crate::Reg<w13::W13_SPEC>;
#[doc = "SPI_MEM_W13"]
pub mod w13;
#[doc = "W14 register accessor: an alias for `Reg<W14_SPEC>`"]
pub type W14 = crate::Reg<w14::W14_SPEC>;
#[doc = "SPI_MEM_W14"]
pub mod w14;
#[doc = "W15 register accessor: an alias for `Reg<W15_SPEC>`"]
pub type W15 = crate::Reg<w15::W15_SPEC>;
#[doc = "SPI_MEM_W15"]
pub mod w15;
#[doc = "FLASH_WAITI_CTRL register accessor: an alias for `Reg<FLASH_WAITI_CTRL_SPEC>`"]
pub type FLASH_WAITI_CTRL = crate::Reg<flash_waiti_ctrl::FLASH_WAITI_CTRL_SPEC>;
#[doc = "SPI_MEM_FLASH_WAITI_CTRL"]
pub mod flash_waiti_ctrl;
#[doc = "FLASH_SUS_CTRL register accessor: an alias for `Reg<FLASH_SUS_CTRL_SPEC>`"]
pub type FLASH_SUS_CTRL = crate::Reg<flash_sus_ctrl::FLASH_SUS_CTRL_SPEC>;
#[doc = "SPI_MEM_FLASH_SUS_CTRL"]
pub mod flash_sus_ctrl;
#[doc = "FLASH_SUS_CMD register accessor: an alias for `Reg<FLASH_SUS_CMD_SPEC>`"]
pub type FLASH_SUS_CMD = crate::Reg<flash_sus_cmd::FLASH_SUS_CMD_SPEC>;
#[doc = "SPI_MEM_FLASH_SUS_CMD"]
pub mod flash_sus_cmd;
#[doc = "SUS_STATUS register accessor: an alias for `Reg<SUS_STATUS_SPEC>`"]
pub type SUS_STATUS = crate::Reg<sus_status::SUS_STATUS_SPEC>;
#[doc = "SPI_MEM_SUS_STATUS"]
pub mod sus_status;
#[doc = "TIMING_CALI register accessor: an alias for `Reg<TIMING_CALI_SPEC>`"]
pub type TIMING_CALI = crate::Reg<timing_cali::TIMING_CALI_SPEC>;
#[doc = "SPI_MEM_TIMING_CALI"]
pub mod timing_cali;
#[doc = "DIN_MODE register accessor: an alias for `Reg<DIN_MODE_SPEC>`"]
pub type DIN_MODE = crate::Reg<din_mode::DIN_MODE_SPEC>;
#[doc = "SPI_MEM_DIN_MODE"]
pub mod din_mode;
#[doc = "DIN_NUM register accessor: an alias for `Reg<DIN_NUM_SPEC>`"]
pub type DIN_NUM = crate::Reg<din_num::DIN_NUM_SPEC>;
#[doc = "SPI_MEM_DIN_NUM"]
pub mod din_num;
#[doc = "DOUT_MODE register accessor: an alias for `Reg<DOUT_MODE_SPEC>`"]
pub type DOUT_MODE = crate::Reg<dout_mode::DOUT_MODE_SPEC>;
#[doc = "SPI_MEM_DOUT_MODE"]
pub mod dout_mode;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "SPI_MEM_INT_ENA"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "SPI_MEM_INT_CLR"]
pub mod int_clr;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "SPI_MEM_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "SPI_MEM_INT_ST"]
pub mod int_st;
#[doc = "CLOCK_GATE register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SPI_MEM_CLOCK_GATE"]
pub mod clock_gate;
#[doc = "CORE_CLK_SEL register accessor: an alias for `Reg<CORE_CLK_SEL_SPEC>`"]
pub type CORE_CLK_SEL = crate::Reg<core_clk_sel::CORE_CLK_SEL_SPEC>;
#[doc = "SPI_MEM_CORE_CLK_SEL"]
pub mod core_clk_sel;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SPI_MEM_DATE"]
pub mod date;
