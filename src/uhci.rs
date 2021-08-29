#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UHCI_CONF0"]
    pub conf0: crate::Reg<conf0::CONF0_SPEC>,
    #[doc = "0x04 - UHCI_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x08 - UHCI_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x0c - UHCI_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x10 - UHCI_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x14 - UHCI_CONF1"]
    pub conf1: crate::Reg<conf1::CONF1_SPEC>,
    #[doc = "0x18 - UHCI_STATE0"]
    pub state0: crate::Reg<state0::STATE0_SPEC>,
    #[doc = "0x1c - UHCI_STATE1"]
    pub state1: crate::Reg<state1::STATE1_SPEC>,
    #[doc = "0x20 - UHCI_ESCAPE_CONF"]
    pub escape_conf: crate::Reg<escape_conf::ESCAPE_CONF_SPEC>,
    #[doc = "0x24 - UHCI_HUNG_CONF"]
    pub hung_conf: crate::Reg<hung_conf::HUNG_CONF_SPEC>,
    #[doc = "0x28 - UHCI_ACK_NUM"]
    pub ack_num: crate::Reg<ack_num::ACK_NUM_SPEC>,
    #[doc = "0x2c - UHCI_RX_HEAD"]
    pub rx_head: crate::Reg<rx_head::RX_HEAD_SPEC>,
    #[doc = "0x30 - UHCI_QUICK_SENT"]
    pub quick_sent: crate::Reg<quick_sent::QUICK_SENT_SPEC>,
    #[doc = "0x34 - UHCI_Q0_WORD0"]
    pub q0_word0: crate::Reg<q0_word0::Q0_WORD0_SPEC>,
    #[doc = "0x38 - UHCI_Q0_WORD1"]
    pub q0_word1: crate::Reg<q0_word1::Q0_WORD1_SPEC>,
    #[doc = "0x3c - UHCI_Q1_WORD0"]
    pub q1_word0: crate::Reg<q1_word0::Q1_WORD0_SPEC>,
    #[doc = "0x40 - UHCI_Q1_WORD1"]
    pub q1_word1: crate::Reg<q1_word1::Q1_WORD1_SPEC>,
    #[doc = "0x44 - UHCI_Q2_WORD0"]
    pub q2_word0: crate::Reg<q2_word0::Q2_WORD0_SPEC>,
    #[doc = "0x48 - UHCI_Q2_WORD1"]
    pub q2_word1: crate::Reg<q2_word1::Q2_WORD1_SPEC>,
    #[doc = "0x4c - UHCI_Q3_WORD0"]
    pub q3_word0: crate::Reg<q3_word0::Q3_WORD0_SPEC>,
    #[doc = "0x50 - UHCI_Q3_WORD1"]
    pub q3_word1: crate::Reg<q3_word1::Q3_WORD1_SPEC>,
    #[doc = "0x54 - UHCI_Q4_WORD0"]
    pub q4_word0: crate::Reg<q4_word0::Q4_WORD0_SPEC>,
    #[doc = "0x58 - UHCI_Q4_WORD1"]
    pub q4_word1: crate::Reg<q4_word1::Q4_WORD1_SPEC>,
    #[doc = "0x5c - UHCI_Q5_WORD0"]
    pub q5_word0: crate::Reg<q5_word0::Q5_WORD0_SPEC>,
    #[doc = "0x60 - UHCI_Q5_WORD1"]
    pub q5_word1: crate::Reg<q5_word1::Q5_WORD1_SPEC>,
    #[doc = "0x64 - UHCI_Q6_WORD0"]
    pub q6_word0: crate::Reg<q6_word0::Q6_WORD0_SPEC>,
    #[doc = "0x68 - UHCI_Q6_WORD1"]
    pub q6_word1: crate::Reg<q6_word1::Q6_WORD1_SPEC>,
    #[doc = "0x6c - UHCI_ESC_CONF0"]
    pub esc_conf0: crate::Reg<esc_conf0::ESC_CONF0_SPEC>,
    #[doc = "0x70 - UHCI_ESC_CONF1"]
    pub esc_conf1: crate::Reg<esc_conf1::ESC_CONF1_SPEC>,
    #[doc = "0x74 - UHCI_ESC_CONF2"]
    pub esc_conf2: crate::Reg<esc_conf2::ESC_CONF2_SPEC>,
    #[doc = "0x78 - UHCI_ESC_CONF3"]
    pub esc_conf3: crate::Reg<esc_conf3::ESC_CONF3_SPEC>,
    #[doc = "0x7c - UHCI_PKT_THRES"]
    pub pkt_thres: crate::Reg<pkt_thres::PKT_THRES_SPEC>,
    #[doc = "0x80 - UHCI_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CONF0 register accessor: an alias for `Reg<CONF0_SPEC>`"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "UHCI_CONF0"]
pub mod conf0;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "UHCI_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "UHCI_INT_ST"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "UHCI_INT_ENA"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "UHCI_INT_CLR"]
pub mod int_clr;
#[doc = "CONF1 register accessor: an alias for `Reg<CONF1_SPEC>`"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "UHCI_CONF1"]
pub mod conf1;
#[doc = "STATE0 register accessor: an alias for `Reg<STATE0_SPEC>`"]
pub type STATE0 = crate::Reg<state0::STATE0_SPEC>;
#[doc = "UHCI_STATE0"]
pub mod state0;
#[doc = "STATE1 register accessor: an alias for `Reg<STATE1_SPEC>`"]
pub type STATE1 = crate::Reg<state1::STATE1_SPEC>;
#[doc = "UHCI_STATE1"]
pub mod state1;
#[doc = "ESCAPE_CONF register accessor: an alias for `Reg<ESCAPE_CONF_SPEC>`"]
pub type ESCAPE_CONF = crate::Reg<escape_conf::ESCAPE_CONF_SPEC>;
#[doc = "UHCI_ESCAPE_CONF"]
pub mod escape_conf;
#[doc = "HUNG_CONF register accessor: an alias for `Reg<HUNG_CONF_SPEC>`"]
pub type HUNG_CONF = crate::Reg<hung_conf::HUNG_CONF_SPEC>;
#[doc = "UHCI_HUNG_CONF"]
pub mod hung_conf;
#[doc = "ACK_NUM register accessor: an alias for `Reg<ACK_NUM_SPEC>`"]
pub type ACK_NUM = crate::Reg<ack_num::ACK_NUM_SPEC>;
#[doc = "UHCI_ACK_NUM"]
pub mod ack_num;
#[doc = "RX_HEAD register accessor: an alias for `Reg<RX_HEAD_SPEC>`"]
pub type RX_HEAD = crate::Reg<rx_head::RX_HEAD_SPEC>;
#[doc = "UHCI_RX_HEAD"]
pub mod rx_head;
#[doc = "QUICK_SENT register accessor: an alias for `Reg<QUICK_SENT_SPEC>`"]
pub type QUICK_SENT = crate::Reg<quick_sent::QUICK_SENT_SPEC>;
#[doc = "UHCI_QUICK_SENT"]
pub mod quick_sent;
#[doc = "Q0_WORD0 register accessor: an alias for `Reg<Q0_WORD0_SPEC>`"]
pub type Q0_WORD0 = crate::Reg<q0_word0::Q0_WORD0_SPEC>;
#[doc = "UHCI_Q0_WORD0"]
pub mod q0_word0;
#[doc = "Q0_WORD1 register accessor: an alias for `Reg<Q0_WORD1_SPEC>`"]
pub type Q0_WORD1 = crate::Reg<q0_word1::Q0_WORD1_SPEC>;
#[doc = "UHCI_Q0_WORD1"]
pub mod q0_word1;
#[doc = "Q1_WORD0 register accessor: an alias for `Reg<Q1_WORD0_SPEC>`"]
pub type Q1_WORD0 = crate::Reg<q1_word0::Q1_WORD0_SPEC>;
#[doc = "UHCI_Q1_WORD0"]
pub mod q1_word0;
#[doc = "Q1_WORD1 register accessor: an alias for `Reg<Q1_WORD1_SPEC>`"]
pub type Q1_WORD1 = crate::Reg<q1_word1::Q1_WORD1_SPEC>;
#[doc = "UHCI_Q1_WORD1"]
pub mod q1_word1;
#[doc = "Q2_WORD0 register accessor: an alias for `Reg<Q2_WORD0_SPEC>`"]
pub type Q2_WORD0 = crate::Reg<q2_word0::Q2_WORD0_SPEC>;
#[doc = "UHCI_Q2_WORD0"]
pub mod q2_word0;
#[doc = "Q2_WORD1 register accessor: an alias for `Reg<Q2_WORD1_SPEC>`"]
pub type Q2_WORD1 = crate::Reg<q2_word1::Q2_WORD1_SPEC>;
#[doc = "UHCI_Q2_WORD1"]
pub mod q2_word1;
#[doc = "Q3_WORD0 register accessor: an alias for `Reg<Q3_WORD0_SPEC>`"]
pub type Q3_WORD0 = crate::Reg<q3_word0::Q3_WORD0_SPEC>;
#[doc = "UHCI_Q3_WORD0"]
pub mod q3_word0;
#[doc = "Q3_WORD1 register accessor: an alias for `Reg<Q3_WORD1_SPEC>`"]
pub type Q3_WORD1 = crate::Reg<q3_word1::Q3_WORD1_SPEC>;
#[doc = "UHCI_Q3_WORD1"]
pub mod q3_word1;
#[doc = "Q4_WORD0 register accessor: an alias for `Reg<Q4_WORD0_SPEC>`"]
pub type Q4_WORD0 = crate::Reg<q4_word0::Q4_WORD0_SPEC>;
#[doc = "UHCI_Q4_WORD0"]
pub mod q4_word0;
#[doc = "Q4_WORD1 register accessor: an alias for `Reg<Q4_WORD1_SPEC>`"]
pub type Q4_WORD1 = crate::Reg<q4_word1::Q4_WORD1_SPEC>;
#[doc = "UHCI_Q4_WORD1"]
pub mod q4_word1;
#[doc = "Q5_WORD0 register accessor: an alias for `Reg<Q5_WORD0_SPEC>`"]
pub type Q5_WORD0 = crate::Reg<q5_word0::Q5_WORD0_SPEC>;
#[doc = "UHCI_Q5_WORD0"]
pub mod q5_word0;
#[doc = "Q5_WORD1 register accessor: an alias for `Reg<Q5_WORD1_SPEC>`"]
pub type Q5_WORD1 = crate::Reg<q5_word1::Q5_WORD1_SPEC>;
#[doc = "UHCI_Q5_WORD1"]
pub mod q5_word1;
#[doc = "Q6_WORD0 register accessor: an alias for `Reg<Q6_WORD0_SPEC>`"]
pub type Q6_WORD0 = crate::Reg<q6_word0::Q6_WORD0_SPEC>;
#[doc = "UHCI_Q6_WORD0"]
pub mod q6_word0;
#[doc = "Q6_WORD1 register accessor: an alias for `Reg<Q6_WORD1_SPEC>`"]
pub type Q6_WORD1 = crate::Reg<q6_word1::Q6_WORD1_SPEC>;
#[doc = "UHCI_Q6_WORD1"]
pub mod q6_word1;
#[doc = "ESC_CONF0 register accessor: an alias for `Reg<ESC_CONF0_SPEC>`"]
pub type ESC_CONF0 = crate::Reg<esc_conf0::ESC_CONF0_SPEC>;
#[doc = "UHCI_ESC_CONF0"]
pub mod esc_conf0;
#[doc = "ESC_CONF1 register accessor: an alias for `Reg<ESC_CONF1_SPEC>`"]
pub type ESC_CONF1 = crate::Reg<esc_conf1::ESC_CONF1_SPEC>;
#[doc = "UHCI_ESC_CONF1"]
pub mod esc_conf1;
#[doc = "ESC_CONF2 register accessor: an alias for `Reg<ESC_CONF2_SPEC>`"]
pub type ESC_CONF2 = crate::Reg<esc_conf2::ESC_CONF2_SPEC>;
#[doc = "UHCI_ESC_CONF2"]
pub mod esc_conf2;
#[doc = "ESC_CONF3 register accessor: an alias for `Reg<ESC_CONF3_SPEC>`"]
pub type ESC_CONF3 = crate::Reg<esc_conf3::ESC_CONF3_SPEC>;
#[doc = "UHCI_ESC_CONF3"]
pub mod esc_conf3;
#[doc = "PKT_THRES register accessor: an alias for `Reg<PKT_THRES_SPEC>`"]
pub type PKT_THRES = crate::Reg<pkt_thres::PKT_THRES_SPEC>;
#[doc = "UHCI_PKT_THRES"]
pub mod pkt_thres;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "UHCI_DATE"]
pub mod date;
