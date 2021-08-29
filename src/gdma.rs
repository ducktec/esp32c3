#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_INT_RAW_CH0"]
    pub dma_int_raw_ch0: crate::Reg<dma_int_raw_ch0::DMA_INT_RAW_CH0_SPEC>,
    #[doc = "0x04 - DMA_INT_ST_CH0"]
    pub dma_int_st_ch0: crate::Reg<dma_int_st_ch0::DMA_INT_ST_CH0_SPEC>,
    #[doc = "0x08 - DMA_INT_ENA_CH0"]
    pub dma_int_ena_ch0: crate::Reg<dma_int_ena_ch0::DMA_INT_ENA_CH0_SPEC>,
    #[doc = "0x0c - DMA_INT_CLR_CH0"]
    pub dma_int_clr_ch0: crate::Reg<dma_int_clr_ch0::DMA_INT_CLR_CH0_SPEC>,
    #[doc = "0x10 - DMA_INT_RAW_CH1"]
    pub dma_int_raw_ch1: crate::Reg<dma_int_raw_ch1::DMA_INT_RAW_CH1_SPEC>,
    #[doc = "0x14 - DMA_INT_ST_CH1"]
    pub dma_int_st_ch1: crate::Reg<dma_int_st_ch1::DMA_INT_ST_CH1_SPEC>,
    #[doc = "0x18 - DMA_INT_ENA_CH1"]
    pub dma_int_ena_ch1: crate::Reg<dma_int_ena_ch1::DMA_INT_ENA_CH1_SPEC>,
    #[doc = "0x1c - DMA_INT_CLR_CH1"]
    pub dma_int_clr_ch1: crate::Reg<dma_int_clr_ch1::DMA_INT_CLR_CH1_SPEC>,
    #[doc = "0x20 - DMA_INT_RAW_CH2"]
    pub dma_int_raw_ch2: crate::Reg<dma_int_raw_ch2::DMA_INT_RAW_CH2_SPEC>,
    #[doc = "0x24 - DMA_INT_ST_CH2"]
    pub dma_int_st_ch2: crate::Reg<dma_int_st_ch2::DMA_INT_ST_CH2_SPEC>,
    #[doc = "0x28 - DMA_INT_ENA_CH2"]
    pub dma_int_ena_ch2: crate::Reg<dma_int_ena_ch2::DMA_INT_ENA_CH2_SPEC>,
    #[doc = "0x2c - DMA_INT_CLR_CH2"]
    pub dma_int_clr_ch2: crate::Reg<dma_int_clr_ch2::DMA_INT_CLR_CH2_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - DMA_AHB_TEST"]
    pub dma_ahb_test: crate::Reg<dma_ahb_test::DMA_AHB_TEST_SPEC>,
    #[doc = "0x44 - DMA_MISC_CONF"]
    pub dma_misc_conf: crate::Reg<dma_misc_conf::DMA_MISC_CONF_SPEC>,
    #[doc = "0x48 - DMA_DATE"]
    pub dma_date: crate::Reg<dma_date::DMA_DATE_SPEC>,
    _reserved15: [u8; 0x24],
    #[doc = "0x70 - DMA_IN_CONF0_CH0"]
    pub dma_in_conf0_ch0: crate::Reg<dma_in_conf0_ch0::DMA_IN_CONF0_CH0_SPEC>,
    #[doc = "0x74 - DMA_IN_CONF1_CH0"]
    pub dma_in_conf1_ch0: crate::Reg<dma_in_conf1_ch0::DMA_IN_CONF1_CH0_SPEC>,
    #[doc = "0x78 - DMA_INFIFO_STATUS_CH0"]
    pub dma_infifo_status_ch0: crate::Reg<dma_infifo_status_ch0::DMA_INFIFO_STATUS_CH0_SPEC>,
    #[doc = "0x7c - DMA_IN_POP_CH0"]
    pub dma_in_pop_ch0: crate::Reg<dma_in_pop_ch0::DMA_IN_POP_CH0_SPEC>,
    #[doc = "0x80 - DMA_IN_LINK_CH0"]
    pub dma_in_link_ch0: crate::Reg<dma_in_link_ch0::DMA_IN_LINK_CH0_SPEC>,
    #[doc = "0x84 - DMA_IN_STATE_CH0"]
    pub dma_in_state_ch0: crate::Reg<dma_in_state_ch0::DMA_IN_STATE_CH0_SPEC>,
    #[doc = "0x88 - DMA_IN_SUC_EOF_DES_ADDR_CH0"]
    pub dma_in_suc_eof_des_addr_ch0:
        crate::Reg<dma_in_suc_eof_des_addr_ch0::DMA_IN_SUC_EOF_DES_ADDR_CH0_SPEC>,
    #[doc = "0x8c - DMA_IN_ERR_EOF_DES_ADDR_CH0"]
    pub dma_in_err_eof_des_addr_ch0:
        crate::Reg<dma_in_err_eof_des_addr_ch0::DMA_IN_ERR_EOF_DES_ADDR_CH0_SPEC>,
    #[doc = "0x90 - DMA_IN_DSCR_CH0"]
    pub dma_in_dscr_ch0: crate::Reg<dma_in_dscr_ch0::DMA_IN_DSCR_CH0_SPEC>,
    #[doc = "0x94 - DMA_IN_DSCR_BF0_CH0"]
    pub dma_in_dscr_bf0_ch0: crate::Reg<dma_in_dscr_bf0_ch0::DMA_IN_DSCR_BF0_CH0_SPEC>,
    #[doc = "0x98 - DMA_IN_DSCR_BF1_CH0"]
    pub dma_in_dscr_bf1_ch0: crate::Reg<dma_in_dscr_bf1_ch0::DMA_IN_DSCR_BF1_CH0_SPEC>,
    #[doc = "0x9c - DMA_IN_PRI_CH0"]
    pub dma_in_pri_ch0: crate::Reg<dma_in_pri_ch0::DMA_IN_PRI_CH0_SPEC>,
    #[doc = "0xa0 - DMA_IN_PERI_SEL_CH0"]
    pub dma_in_peri_sel_ch0: crate::Reg<dma_in_peri_sel_ch0::DMA_IN_PERI_SEL_CH0_SPEC>,
    _reserved28: [u8; 0x2c],
    #[doc = "0xd0 - DMA_OUT_CONF0_CH0"]
    pub dma_out_conf0_ch0: crate::Reg<dma_out_conf0_ch0::DMA_OUT_CONF0_CH0_SPEC>,
    #[doc = "0xd4 - DMA_OUT_CONF1_CH0"]
    pub dma_out_conf1_ch0: crate::Reg<dma_out_conf1_ch0::DMA_OUT_CONF1_CH0_SPEC>,
    #[doc = "0xd8 - DMA_OUTFIFO_STATUS_CH0"]
    pub dma_outfifo_status_ch0: crate::Reg<dma_outfifo_status_ch0::DMA_OUTFIFO_STATUS_CH0_SPEC>,
    #[doc = "0xdc - DMA_OUT_PUSH_CH0"]
    pub dma_out_push_ch0: crate::Reg<dma_out_push_ch0::DMA_OUT_PUSH_CH0_SPEC>,
    #[doc = "0xe0 - DMA_OUT_LINK_CH0"]
    pub dma_out_link_ch0: crate::Reg<dma_out_link_ch0::DMA_OUT_LINK_CH0_SPEC>,
    #[doc = "0xe4 - DMA_OUT_STATE_CH0"]
    pub dma_out_state_ch0: crate::Reg<dma_out_state_ch0::DMA_OUT_STATE_CH0_SPEC>,
    #[doc = "0xe8 - DMA_OUT_EOF_DES_ADDR_CH0"]
    pub dma_out_eof_des_addr_ch0:
        crate::Reg<dma_out_eof_des_addr_ch0::DMA_OUT_EOF_DES_ADDR_CH0_SPEC>,
    #[doc = "0xec - DMA_OUT_EOF_BFR_DES_ADDR_CH0"]
    pub dma_out_eof_bfr_des_addr_ch0:
        crate::Reg<dma_out_eof_bfr_des_addr_ch0::DMA_OUT_EOF_BFR_DES_ADDR_CH0_SPEC>,
    #[doc = "0xf0 - DMA_OUT_DSCR_CH0"]
    pub dma_out_dscr_ch0: crate::Reg<dma_out_dscr_ch0::DMA_OUT_DSCR_CH0_SPEC>,
    #[doc = "0xf4 - DMA_OUT_DSCR_BF0_CH0"]
    pub dma_out_dscr_bf0_ch0: crate::Reg<dma_out_dscr_bf0_ch0::DMA_OUT_DSCR_BF0_CH0_SPEC>,
    #[doc = "0xf8 - DMA_OUT_DSCR_BF1_CH0"]
    pub dma_out_dscr_bf1_ch0: crate::Reg<dma_out_dscr_bf1_ch0::DMA_OUT_DSCR_BF1_CH0_SPEC>,
    #[doc = "0xfc - DMA_OUT_PRI_CH0"]
    pub dma_out_pri_ch0: crate::Reg<dma_out_pri_ch0::DMA_OUT_PRI_CH0_SPEC>,
    #[doc = "0x100 - DMA_OUT_PERI_SEL_CH0"]
    pub dma_out_peri_sel_ch0: crate::Reg<dma_out_peri_sel_ch0::DMA_OUT_PERI_SEL_CH0_SPEC>,
    _reserved41: [u8; 0x2c],
    #[doc = "0x130 - DMA_IN_CONF0_CH1"]
    pub dma_in_conf0_ch1: crate::Reg<dma_in_conf0_ch1::DMA_IN_CONF0_CH1_SPEC>,
    #[doc = "0x134 - DMA_IN_CONF1_CH1"]
    pub dma_in_conf1_ch1: crate::Reg<dma_in_conf1_ch1::DMA_IN_CONF1_CH1_SPEC>,
    #[doc = "0x138 - DMA_INFIFO_STATUS_CH1"]
    pub dma_infifo_status_ch1: crate::Reg<dma_infifo_status_ch1::DMA_INFIFO_STATUS_CH1_SPEC>,
    #[doc = "0x13c - DMA_IN_POP_CH1"]
    pub dma_in_pop_ch1: crate::Reg<dma_in_pop_ch1::DMA_IN_POP_CH1_SPEC>,
    #[doc = "0x140 - DMA_IN_LINK_CH1"]
    pub dma_in_link_ch1: crate::Reg<dma_in_link_ch1::DMA_IN_LINK_CH1_SPEC>,
    #[doc = "0x144 - DMA_IN_STATE_CH1"]
    pub dma_in_state_ch1: crate::Reg<dma_in_state_ch1::DMA_IN_STATE_CH1_SPEC>,
    #[doc = "0x148 - DMA_IN_SUC_EOF_DES_ADDR_CH1"]
    pub dma_in_suc_eof_des_addr_ch1:
        crate::Reg<dma_in_suc_eof_des_addr_ch1::DMA_IN_SUC_EOF_DES_ADDR_CH1_SPEC>,
    #[doc = "0x14c - DMA_IN_ERR_EOF_DES_ADDR_CH1"]
    pub dma_in_err_eof_des_addr_ch1:
        crate::Reg<dma_in_err_eof_des_addr_ch1::DMA_IN_ERR_EOF_DES_ADDR_CH1_SPEC>,
    #[doc = "0x150 - DMA_IN_DSCR_CH1"]
    pub dma_in_dscr_ch1: crate::Reg<dma_in_dscr_ch1::DMA_IN_DSCR_CH1_SPEC>,
    #[doc = "0x154 - DMA_IN_DSCR_BF0_CH1"]
    pub dma_in_dscr_bf0_ch1: crate::Reg<dma_in_dscr_bf0_ch1::DMA_IN_DSCR_BF0_CH1_SPEC>,
    #[doc = "0x158 - DMA_IN_DSCR_BF1_CH1"]
    pub dma_in_dscr_bf1_ch1: crate::Reg<dma_in_dscr_bf1_ch1::DMA_IN_DSCR_BF1_CH1_SPEC>,
    #[doc = "0x15c - DMA_IN_PRI_CH1"]
    pub dma_in_pri_ch1: crate::Reg<dma_in_pri_ch1::DMA_IN_PRI_CH1_SPEC>,
    #[doc = "0x160 - DMA_IN_PERI_SEL_CH1"]
    pub dma_in_peri_sel_ch1: crate::Reg<dma_in_peri_sel_ch1::DMA_IN_PERI_SEL_CH1_SPEC>,
    _reserved54: [u8; 0x2c],
    #[doc = "0x190 - DMA_OUT_CONF0_CH1"]
    pub dma_out_conf0_ch1: crate::Reg<dma_out_conf0_ch1::DMA_OUT_CONF0_CH1_SPEC>,
    #[doc = "0x194 - DMA_OUT_CONF1_CH1"]
    pub dma_out_conf1_ch1: crate::Reg<dma_out_conf1_ch1::DMA_OUT_CONF1_CH1_SPEC>,
    #[doc = "0x198 - DMA_OUTFIFO_STATUS_CH1"]
    pub dma_outfifo_status_ch1: crate::Reg<dma_outfifo_status_ch1::DMA_OUTFIFO_STATUS_CH1_SPEC>,
    #[doc = "0x19c - DMA_OUT_PUSH_CH1"]
    pub dma_out_push_ch1: crate::Reg<dma_out_push_ch1::DMA_OUT_PUSH_CH1_SPEC>,
    #[doc = "0x1a0 - DMA_OUT_LINK_CH1"]
    pub dma_out_link_ch1: crate::Reg<dma_out_link_ch1::DMA_OUT_LINK_CH1_SPEC>,
    #[doc = "0x1a4 - DMA_OUT_STATE_CH1"]
    pub dma_out_state_ch1: crate::Reg<dma_out_state_ch1::DMA_OUT_STATE_CH1_SPEC>,
    #[doc = "0x1a8 - DMA_OUT_EOF_DES_ADDR_CH1"]
    pub dma_out_eof_des_addr_ch1:
        crate::Reg<dma_out_eof_des_addr_ch1::DMA_OUT_EOF_DES_ADDR_CH1_SPEC>,
    #[doc = "0x1ac - DMA_OUT_EOF_BFR_DES_ADDR_CH1"]
    pub dma_out_eof_bfr_des_addr_ch1:
        crate::Reg<dma_out_eof_bfr_des_addr_ch1::DMA_OUT_EOF_BFR_DES_ADDR_CH1_SPEC>,
    #[doc = "0x1b0 - DMA_OUT_DSCR_CH1"]
    pub dma_out_dscr_ch1: crate::Reg<dma_out_dscr_ch1::DMA_OUT_DSCR_CH1_SPEC>,
    #[doc = "0x1b4 - DMA_OUT_DSCR_BF0_CH1"]
    pub dma_out_dscr_bf0_ch1: crate::Reg<dma_out_dscr_bf0_ch1::DMA_OUT_DSCR_BF0_CH1_SPEC>,
    #[doc = "0x1b8 - DMA_OUT_DSCR_BF1_CH1"]
    pub dma_out_dscr_bf1_ch1: crate::Reg<dma_out_dscr_bf1_ch1::DMA_OUT_DSCR_BF1_CH1_SPEC>,
    #[doc = "0x1bc - DMA_OUT_PRI_CH1"]
    pub dma_out_pri_ch1: crate::Reg<dma_out_pri_ch1::DMA_OUT_PRI_CH1_SPEC>,
    #[doc = "0x1c0 - DMA_OUT_PERI_SEL_CH1"]
    pub dma_out_peri_sel_ch1: crate::Reg<dma_out_peri_sel_ch1::DMA_OUT_PERI_SEL_CH1_SPEC>,
    _reserved67: [u8; 0x2c],
    #[doc = "0x1f0 - DMA_IN_CONF0_CH2"]
    pub dma_in_conf0_ch2: crate::Reg<dma_in_conf0_ch2::DMA_IN_CONF0_CH2_SPEC>,
    #[doc = "0x1f4 - DMA_IN_CONF1_CH2"]
    pub dma_in_conf1_ch2: crate::Reg<dma_in_conf1_ch2::DMA_IN_CONF1_CH2_SPEC>,
    #[doc = "0x1f8 - DMA_INFIFO_STATUS_CH2"]
    pub dma_infifo_status_ch2: crate::Reg<dma_infifo_status_ch2::DMA_INFIFO_STATUS_CH2_SPEC>,
    #[doc = "0x1fc - DMA_IN_POP_CH2"]
    pub dma_in_pop_ch2: crate::Reg<dma_in_pop_ch2::DMA_IN_POP_CH2_SPEC>,
    #[doc = "0x200 - DMA_IN_LINK_CH2"]
    pub dma_in_link_ch2: crate::Reg<dma_in_link_ch2::DMA_IN_LINK_CH2_SPEC>,
    #[doc = "0x204 - DMA_IN_STATE_CH2"]
    pub dma_in_state_ch2: crate::Reg<dma_in_state_ch2::DMA_IN_STATE_CH2_SPEC>,
    #[doc = "0x208 - DMA_IN_SUC_EOF_DES_ADDR_CH2"]
    pub dma_in_suc_eof_des_addr_ch2:
        crate::Reg<dma_in_suc_eof_des_addr_ch2::DMA_IN_SUC_EOF_DES_ADDR_CH2_SPEC>,
    #[doc = "0x20c - DMA_IN_ERR_EOF_DES_ADDR_CH2"]
    pub dma_in_err_eof_des_addr_ch2:
        crate::Reg<dma_in_err_eof_des_addr_ch2::DMA_IN_ERR_EOF_DES_ADDR_CH2_SPEC>,
    #[doc = "0x210 - DMA_IN_DSCR_CH2"]
    pub dma_in_dscr_ch2: crate::Reg<dma_in_dscr_ch2::DMA_IN_DSCR_CH2_SPEC>,
    #[doc = "0x214 - DMA_IN_DSCR_BF0_CH2"]
    pub dma_in_dscr_bf0_ch2: crate::Reg<dma_in_dscr_bf0_ch2::DMA_IN_DSCR_BF0_CH2_SPEC>,
    #[doc = "0x218 - DMA_IN_DSCR_BF1_CH2"]
    pub dma_in_dscr_bf1_ch2: crate::Reg<dma_in_dscr_bf1_ch2::DMA_IN_DSCR_BF1_CH2_SPEC>,
    #[doc = "0x21c - DMA_IN_PRI_CH2"]
    pub dma_in_pri_ch2: crate::Reg<dma_in_pri_ch2::DMA_IN_PRI_CH2_SPEC>,
    #[doc = "0x220 - DMA_IN_PERI_SEL_CH2"]
    pub dma_in_peri_sel_ch2: crate::Reg<dma_in_peri_sel_ch2::DMA_IN_PERI_SEL_CH2_SPEC>,
    _reserved80: [u8; 0x2c],
    #[doc = "0x250 - DMA_OUT_CONF0_CH2"]
    pub dma_out_conf0_ch2: crate::Reg<dma_out_conf0_ch2::DMA_OUT_CONF0_CH2_SPEC>,
    #[doc = "0x254 - DMA_OUT_CONF1_CH2"]
    pub dma_out_conf1_ch2: crate::Reg<dma_out_conf1_ch2::DMA_OUT_CONF1_CH2_SPEC>,
    #[doc = "0x258 - DMA_OUTFIFO_STATUS_CH2"]
    pub dma_outfifo_status_ch2: crate::Reg<dma_outfifo_status_ch2::DMA_OUTFIFO_STATUS_CH2_SPEC>,
    #[doc = "0x25c - DMA_OUT_PUSH_CH2"]
    pub dma_out_push_ch2: crate::Reg<dma_out_push_ch2::DMA_OUT_PUSH_CH2_SPEC>,
    #[doc = "0x260 - DMA_OUT_LINK_CH2"]
    pub dma_out_link_ch2: crate::Reg<dma_out_link_ch2::DMA_OUT_LINK_CH2_SPEC>,
    #[doc = "0x264 - DMA_OUT_STATE_CH2"]
    pub dma_out_state_ch2: crate::Reg<dma_out_state_ch2::DMA_OUT_STATE_CH2_SPEC>,
    #[doc = "0x268 - DMA_OUT_EOF_DES_ADDR_CH2"]
    pub dma_out_eof_des_addr_ch2:
        crate::Reg<dma_out_eof_des_addr_ch2::DMA_OUT_EOF_DES_ADDR_CH2_SPEC>,
    #[doc = "0x26c - DMA_OUT_EOF_BFR_DES_ADDR_CH2"]
    pub dma_out_eof_bfr_des_addr_ch2:
        crate::Reg<dma_out_eof_bfr_des_addr_ch2::DMA_OUT_EOF_BFR_DES_ADDR_CH2_SPEC>,
    #[doc = "0x270 - DMA_OUT_DSCR_CH2"]
    pub dma_out_dscr_ch2: crate::Reg<dma_out_dscr_ch2::DMA_OUT_DSCR_CH2_SPEC>,
    #[doc = "0x274 - DMA_OUT_DSCR_BF0_CH2"]
    pub dma_out_dscr_bf0_ch2: crate::Reg<dma_out_dscr_bf0_ch2::DMA_OUT_DSCR_BF0_CH2_SPEC>,
    #[doc = "0x278 - DMA_OUT_DSCR_BF1_CH2"]
    pub dma_out_dscr_bf1_ch2: crate::Reg<dma_out_dscr_bf1_ch2::DMA_OUT_DSCR_BF1_CH2_SPEC>,
    #[doc = "0x27c - DMA_OUT_PRI_CH2"]
    pub dma_out_pri_ch2: crate::Reg<dma_out_pri_ch2::DMA_OUT_PRI_CH2_SPEC>,
    #[doc = "0x280 - DMA_OUT_PERI_SEL_CH2"]
    pub dma_out_peri_sel_ch2: crate::Reg<dma_out_peri_sel_ch2::DMA_OUT_PERI_SEL_CH2_SPEC>,
}
#[doc = "DMA_INT_RAW_CH0 register accessor: an alias for `Reg<DMA_INT_RAW_CH0_SPEC>`"]
pub type DMA_INT_RAW_CH0 = crate::Reg<dma_int_raw_ch0::DMA_INT_RAW_CH0_SPEC>;
#[doc = "DMA_INT_RAW_CH0"]
pub mod dma_int_raw_ch0;
#[doc = "DMA_INT_ST_CH0 register accessor: an alias for `Reg<DMA_INT_ST_CH0_SPEC>`"]
pub type DMA_INT_ST_CH0 = crate::Reg<dma_int_st_ch0::DMA_INT_ST_CH0_SPEC>;
#[doc = "DMA_INT_ST_CH0"]
pub mod dma_int_st_ch0;
#[doc = "DMA_INT_ENA_CH0 register accessor: an alias for `Reg<DMA_INT_ENA_CH0_SPEC>`"]
pub type DMA_INT_ENA_CH0 = crate::Reg<dma_int_ena_ch0::DMA_INT_ENA_CH0_SPEC>;
#[doc = "DMA_INT_ENA_CH0"]
pub mod dma_int_ena_ch0;
#[doc = "DMA_INT_CLR_CH0 register accessor: an alias for `Reg<DMA_INT_CLR_CH0_SPEC>`"]
pub type DMA_INT_CLR_CH0 = crate::Reg<dma_int_clr_ch0::DMA_INT_CLR_CH0_SPEC>;
#[doc = "DMA_INT_CLR_CH0"]
pub mod dma_int_clr_ch0;
#[doc = "DMA_INT_RAW_CH1 register accessor: an alias for `Reg<DMA_INT_RAW_CH1_SPEC>`"]
pub type DMA_INT_RAW_CH1 = crate::Reg<dma_int_raw_ch1::DMA_INT_RAW_CH1_SPEC>;
#[doc = "DMA_INT_RAW_CH1"]
pub mod dma_int_raw_ch1;
#[doc = "DMA_INT_ST_CH1 register accessor: an alias for `Reg<DMA_INT_ST_CH1_SPEC>`"]
pub type DMA_INT_ST_CH1 = crate::Reg<dma_int_st_ch1::DMA_INT_ST_CH1_SPEC>;
#[doc = "DMA_INT_ST_CH1"]
pub mod dma_int_st_ch1;
#[doc = "DMA_INT_ENA_CH1 register accessor: an alias for `Reg<DMA_INT_ENA_CH1_SPEC>`"]
pub type DMA_INT_ENA_CH1 = crate::Reg<dma_int_ena_ch1::DMA_INT_ENA_CH1_SPEC>;
#[doc = "DMA_INT_ENA_CH1"]
pub mod dma_int_ena_ch1;
#[doc = "DMA_INT_CLR_CH1 register accessor: an alias for `Reg<DMA_INT_CLR_CH1_SPEC>`"]
pub type DMA_INT_CLR_CH1 = crate::Reg<dma_int_clr_ch1::DMA_INT_CLR_CH1_SPEC>;
#[doc = "DMA_INT_CLR_CH1"]
pub mod dma_int_clr_ch1;
#[doc = "DMA_INT_RAW_CH2 register accessor: an alias for `Reg<DMA_INT_RAW_CH2_SPEC>`"]
pub type DMA_INT_RAW_CH2 = crate::Reg<dma_int_raw_ch2::DMA_INT_RAW_CH2_SPEC>;
#[doc = "DMA_INT_RAW_CH2"]
pub mod dma_int_raw_ch2;
#[doc = "DMA_INT_ST_CH2 register accessor: an alias for `Reg<DMA_INT_ST_CH2_SPEC>`"]
pub type DMA_INT_ST_CH2 = crate::Reg<dma_int_st_ch2::DMA_INT_ST_CH2_SPEC>;
#[doc = "DMA_INT_ST_CH2"]
pub mod dma_int_st_ch2;
#[doc = "DMA_INT_ENA_CH2 register accessor: an alias for `Reg<DMA_INT_ENA_CH2_SPEC>`"]
pub type DMA_INT_ENA_CH2 = crate::Reg<dma_int_ena_ch2::DMA_INT_ENA_CH2_SPEC>;
#[doc = "DMA_INT_ENA_CH2"]
pub mod dma_int_ena_ch2;
#[doc = "DMA_INT_CLR_CH2 register accessor: an alias for `Reg<DMA_INT_CLR_CH2_SPEC>`"]
pub type DMA_INT_CLR_CH2 = crate::Reg<dma_int_clr_ch2::DMA_INT_CLR_CH2_SPEC>;
#[doc = "DMA_INT_CLR_CH2"]
pub mod dma_int_clr_ch2;
#[doc = "DMA_AHB_TEST register accessor: an alias for `Reg<DMA_AHB_TEST_SPEC>`"]
pub type DMA_AHB_TEST = crate::Reg<dma_ahb_test::DMA_AHB_TEST_SPEC>;
#[doc = "DMA_AHB_TEST"]
pub mod dma_ahb_test;
#[doc = "DMA_MISC_CONF register accessor: an alias for `Reg<DMA_MISC_CONF_SPEC>`"]
pub type DMA_MISC_CONF = crate::Reg<dma_misc_conf::DMA_MISC_CONF_SPEC>;
#[doc = "DMA_MISC_CONF"]
pub mod dma_misc_conf;
#[doc = "DMA_DATE register accessor: an alias for `Reg<DMA_DATE_SPEC>`"]
pub type DMA_DATE = crate::Reg<dma_date::DMA_DATE_SPEC>;
#[doc = "DMA_DATE"]
pub mod dma_date;
#[doc = "DMA_IN_CONF0_CH0 register accessor: an alias for `Reg<DMA_IN_CONF0_CH0_SPEC>`"]
pub type DMA_IN_CONF0_CH0 = crate::Reg<dma_in_conf0_ch0::DMA_IN_CONF0_CH0_SPEC>;
#[doc = "DMA_IN_CONF0_CH0"]
pub mod dma_in_conf0_ch0;
#[doc = "DMA_IN_CONF1_CH0 register accessor: an alias for `Reg<DMA_IN_CONF1_CH0_SPEC>`"]
pub type DMA_IN_CONF1_CH0 = crate::Reg<dma_in_conf1_ch0::DMA_IN_CONF1_CH0_SPEC>;
#[doc = "DMA_IN_CONF1_CH0"]
pub mod dma_in_conf1_ch0;
#[doc = "DMA_INFIFO_STATUS_CH0 register accessor: an alias for `Reg<DMA_INFIFO_STATUS_CH0_SPEC>`"]
pub type DMA_INFIFO_STATUS_CH0 = crate::Reg<dma_infifo_status_ch0::DMA_INFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH0"]
pub mod dma_infifo_status_ch0;
#[doc = "DMA_IN_POP_CH0 register accessor: an alias for `Reg<DMA_IN_POP_CH0_SPEC>`"]
pub type DMA_IN_POP_CH0 = crate::Reg<dma_in_pop_ch0::DMA_IN_POP_CH0_SPEC>;
#[doc = "DMA_IN_POP_CH0"]
pub mod dma_in_pop_ch0;
#[doc = "DMA_IN_LINK_CH0 register accessor: an alias for `Reg<DMA_IN_LINK_CH0_SPEC>`"]
pub type DMA_IN_LINK_CH0 = crate::Reg<dma_in_link_ch0::DMA_IN_LINK_CH0_SPEC>;
#[doc = "DMA_IN_LINK_CH0"]
pub mod dma_in_link_ch0;
#[doc = "DMA_IN_STATE_CH0 register accessor: an alias for `Reg<DMA_IN_STATE_CH0_SPEC>`"]
pub type DMA_IN_STATE_CH0 = crate::Reg<dma_in_state_ch0::DMA_IN_STATE_CH0_SPEC>;
#[doc = "DMA_IN_STATE_CH0"]
pub mod dma_in_state_ch0;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0 register accessor: an alias for `Reg<DMA_IN_SUC_EOF_DES_ADDR_CH0_SPEC>`"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH0 =
    crate::Reg<dma_in_suc_eof_des_addr_ch0::DMA_IN_SUC_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH0"]
pub mod dma_in_suc_eof_des_addr_ch0;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0 register accessor: an alias for `Reg<DMA_IN_ERR_EOF_DES_ADDR_CH0_SPEC>`"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH0 =
    crate::Reg<dma_in_err_eof_des_addr_ch0::DMA_IN_ERR_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0"]
pub mod dma_in_err_eof_des_addr_ch0;
#[doc = "DMA_IN_DSCR_CH0 register accessor: an alias for `Reg<DMA_IN_DSCR_CH0_SPEC>`"]
pub type DMA_IN_DSCR_CH0 = crate::Reg<dma_in_dscr_ch0::DMA_IN_DSCR_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_CH0"]
pub mod dma_in_dscr_ch0;
#[doc = "DMA_IN_DSCR_BF0_CH0 register accessor: an alias for `Reg<DMA_IN_DSCR_BF0_CH0_SPEC>`"]
pub type DMA_IN_DSCR_BF0_CH0 = crate::Reg<dma_in_dscr_bf0_ch0::DMA_IN_DSCR_BF0_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH0"]
pub mod dma_in_dscr_bf0_ch0;
#[doc = "DMA_IN_DSCR_BF1_CH0 register accessor: an alias for `Reg<DMA_IN_DSCR_BF1_CH0_SPEC>`"]
pub type DMA_IN_DSCR_BF1_CH0 = crate::Reg<dma_in_dscr_bf1_ch0::DMA_IN_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH0"]
pub mod dma_in_dscr_bf1_ch0;
#[doc = "DMA_IN_PRI_CH0 register accessor: an alias for `Reg<DMA_IN_PRI_CH0_SPEC>`"]
pub type DMA_IN_PRI_CH0 = crate::Reg<dma_in_pri_ch0::DMA_IN_PRI_CH0_SPEC>;
#[doc = "DMA_IN_PRI_CH0"]
pub mod dma_in_pri_ch0;
#[doc = "DMA_IN_PERI_SEL_CH0 register accessor: an alias for `Reg<DMA_IN_PERI_SEL_CH0_SPEC>`"]
pub type DMA_IN_PERI_SEL_CH0 = crate::Reg<dma_in_peri_sel_ch0::DMA_IN_PERI_SEL_CH0_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH0"]
pub mod dma_in_peri_sel_ch0;
#[doc = "DMA_OUT_CONF0_CH0 register accessor: an alias for `Reg<DMA_OUT_CONF0_CH0_SPEC>`"]
pub type DMA_OUT_CONF0_CH0 = crate::Reg<dma_out_conf0_ch0::DMA_OUT_CONF0_CH0_SPEC>;
#[doc = "DMA_OUT_CONF0_CH0"]
pub mod dma_out_conf0_ch0;
#[doc = "DMA_OUT_CONF1_CH0 register accessor: an alias for `Reg<DMA_OUT_CONF1_CH0_SPEC>`"]
pub type DMA_OUT_CONF1_CH0 = crate::Reg<dma_out_conf1_ch0::DMA_OUT_CONF1_CH0_SPEC>;
#[doc = "DMA_OUT_CONF1_CH0"]
pub mod dma_out_conf1_ch0;
#[doc = "DMA_OUTFIFO_STATUS_CH0 register accessor: an alias for `Reg<DMA_OUTFIFO_STATUS_CH0_SPEC>`"]
pub type DMA_OUTFIFO_STATUS_CH0 = crate::Reg<dma_outfifo_status_ch0::DMA_OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH0"]
pub mod dma_outfifo_status_ch0;
#[doc = "DMA_OUT_PUSH_CH0 register accessor: an alias for `Reg<DMA_OUT_PUSH_CH0_SPEC>`"]
pub type DMA_OUT_PUSH_CH0 = crate::Reg<dma_out_push_ch0::DMA_OUT_PUSH_CH0_SPEC>;
#[doc = "DMA_OUT_PUSH_CH0"]
pub mod dma_out_push_ch0;
#[doc = "DMA_OUT_LINK_CH0 register accessor: an alias for `Reg<DMA_OUT_LINK_CH0_SPEC>`"]
pub type DMA_OUT_LINK_CH0 = crate::Reg<dma_out_link_ch0::DMA_OUT_LINK_CH0_SPEC>;
#[doc = "DMA_OUT_LINK_CH0"]
pub mod dma_out_link_ch0;
#[doc = "DMA_OUT_STATE_CH0 register accessor: an alias for `Reg<DMA_OUT_STATE_CH0_SPEC>`"]
pub type DMA_OUT_STATE_CH0 = crate::Reg<dma_out_state_ch0::DMA_OUT_STATE_CH0_SPEC>;
#[doc = "DMA_OUT_STATE_CH0"]
pub mod dma_out_state_ch0;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0 register accessor: an alias for `Reg<DMA_OUT_EOF_DES_ADDR_CH0_SPEC>`"]
pub type DMA_OUT_EOF_DES_ADDR_CH0 =
    crate::Reg<dma_out_eof_des_addr_ch0::DMA_OUT_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH0"]
pub mod dma_out_eof_des_addr_ch0;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0 register accessor: an alias for `Reg<DMA_OUT_EOF_BFR_DES_ADDR_CH0_SPEC>`"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH0 =
    crate::Reg<dma_out_eof_bfr_des_addr_ch0::DMA_OUT_EOF_BFR_DES_ADDR_CH0_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH0"]
pub mod dma_out_eof_bfr_des_addr_ch0;
#[doc = "DMA_OUT_DSCR_CH0 register accessor: an alias for `Reg<DMA_OUT_DSCR_CH0_SPEC>`"]
pub type DMA_OUT_DSCR_CH0 = crate::Reg<dma_out_dscr_ch0::DMA_OUT_DSCR_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_CH0"]
pub mod dma_out_dscr_ch0;
#[doc = "DMA_OUT_DSCR_BF0_CH0 register accessor: an alias for `Reg<DMA_OUT_DSCR_BF0_CH0_SPEC>`"]
pub type DMA_OUT_DSCR_BF0_CH0 = crate::Reg<dma_out_dscr_bf0_ch0::DMA_OUT_DSCR_BF0_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH0"]
pub mod dma_out_dscr_bf0_ch0;
#[doc = "DMA_OUT_DSCR_BF1_CH0 register accessor: an alias for `Reg<DMA_OUT_DSCR_BF1_CH0_SPEC>`"]
pub type DMA_OUT_DSCR_BF1_CH0 = crate::Reg<dma_out_dscr_bf1_ch0::DMA_OUT_DSCR_BF1_CH0_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH0"]
pub mod dma_out_dscr_bf1_ch0;
#[doc = "DMA_OUT_PRI_CH0 register accessor: an alias for `Reg<DMA_OUT_PRI_CH0_SPEC>`"]
pub type DMA_OUT_PRI_CH0 = crate::Reg<dma_out_pri_ch0::DMA_OUT_PRI_CH0_SPEC>;
#[doc = "DMA_OUT_PRI_CH0"]
pub mod dma_out_pri_ch0;
#[doc = "DMA_OUT_PERI_SEL_CH0 register accessor: an alias for `Reg<DMA_OUT_PERI_SEL_CH0_SPEC>`"]
pub type DMA_OUT_PERI_SEL_CH0 = crate::Reg<dma_out_peri_sel_ch0::DMA_OUT_PERI_SEL_CH0_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH0"]
pub mod dma_out_peri_sel_ch0;
#[doc = "DMA_IN_CONF0_CH1 register accessor: an alias for `Reg<DMA_IN_CONF0_CH1_SPEC>`"]
pub type DMA_IN_CONF0_CH1 = crate::Reg<dma_in_conf0_ch1::DMA_IN_CONF0_CH1_SPEC>;
#[doc = "DMA_IN_CONF0_CH1"]
pub mod dma_in_conf0_ch1;
#[doc = "DMA_IN_CONF1_CH1 register accessor: an alias for `Reg<DMA_IN_CONF1_CH1_SPEC>`"]
pub type DMA_IN_CONF1_CH1 = crate::Reg<dma_in_conf1_ch1::DMA_IN_CONF1_CH1_SPEC>;
#[doc = "DMA_IN_CONF1_CH1"]
pub mod dma_in_conf1_ch1;
#[doc = "DMA_INFIFO_STATUS_CH1 register accessor: an alias for `Reg<DMA_INFIFO_STATUS_CH1_SPEC>`"]
pub type DMA_INFIFO_STATUS_CH1 = crate::Reg<dma_infifo_status_ch1::DMA_INFIFO_STATUS_CH1_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH1"]
pub mod dma_infifo_status_ch1;
#[doc = "DMA_IN_POP_CH1 register accessor: an alias for `Reg<DMA_IN_POP_CH1_SPEC>`"]
pub type DMA_IN_POP_CH1 = crate::Reg<dma_in_pop_ch1::DMA_IN_POP_CH1_SPEC>;
#[doc = "DMA_IN_POP_CH1"]
pub mod dma_in_pop_ch1;
#[doc = "DMA_IN_LINK_CH1 register accessor: an alias for `Reg<DMA_IN_LINK_CH1_SPEC>`"]
pub type DMA_IN_LINK_CH1 = crate::Reg<dma_in_link_ch1::DMA_IN_LINK_CH1_SPEC>;
#[doc = "DMA_IN_LINK_CH1"]
pub mod dma_in_link_ch1;
#[doc = "DMA_IN_STATE_CH1 register accessor: an alias for `Reg<DMA_IN_STATE_CH1_SPEC>`"]
pub type DMA_IN_STATE_CH1 = crate::Reg<dma_in_state_ch1::DMA_IN_STATE_CH1_SPEC>;
#[doc = "DMA_IN_STATE_CH1"]
pub mod dma_in_state_ch1;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1 register accessor: an alias for `Reg<DMA_IN_SUC_EOF_DES_ADDR_CH1_SPEC>`"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH1 =
    crate::Reg<dma_in_suc_eof_des_addr_ch1::DMA_IN_SUC_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH1"]
pub mod dma_in_suc_eof_des_addr_ch1;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1 register accessor: an alias for `Reg<DMA_IN_ERR_EOF_DES_ADDR_CH1_SPEC>`"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH1 =
    crate::Reg<dma_in_err_eof_des_addr_ch1::DMA_IN_ERR_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH1"]
pub mod dma_in_err_eof_des_addr_ch1;
#[doc = "DMA_IN_DSCR_CH1 register accessor: an alias for `Reg<DMA_IN_DSCR_CH1_SPEC>`"]
pub type DMA_IN_DSCR_CH1 = crate::Reg<dma_in_dscr_ch1::DMA_IN_DSCR_CH1_SPEC>;
#[doc = "DMA_IN_DSCR_CH1"]
pub mod dma_in_dscr_ch1;
#[doc = "DMA_IN_DSCR_BF0_CH1 register accessor: an alias for `Reg<DMA_IN_DSCR_BF0_CH1_SPEC>`"]
pub type DMA_IN_DSCR_BF0_CH1 = crate::Reg<dma_in_dscr_bf0_ch1::DMA_IN_DSCR_BF0_CH1_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH1"]
pub mod dma_in_dscr_bf0_ch1;
#[doc = "DMA_IN_DSCR_BF1_CH1 register accessor: an alias for `Reg<DMA_IN_DSCR_BF1_CH1_SPEC>`"]
pub type DMA_IN_DSCR_BF1_CH1 = crate::Reg<dma_in_dscr_bf1_ch1::DMA_IN_DSCR_BF1_CH1_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH1"]
pub mod dma_in_dscr_bf1_ch1;
#[doc = "DMA_IN_PRI_CH1 register accessor: an alias for `Reg<DMA_IN_PRI_CH1_SPEC>`"]
pub type DMA_IN_PRI_CH1 = crate::Reg<dma_in_pri_ch1::DMA_IN_PRI_CH1_SPEC>;
#[doc = "DMA_IN_PRI_CH1"]
pub mod dma_in_pri_ch1;
#[doc = "DMA_IN_PERI_SEL_CH1 register accessor: an alias for `Reg<DMA_IN_PERI_SEL_CH1_SPEC>`"]
pub type DMA_IN_PERI_SEL_CH1 = crate::Reg<dma_in_peri_sel_ch1::DMA_IN_PERI_SEL_CH1_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH1"]
pub mod dma_in_peri_sel_ch1;
#[doc = "DMA_OUT_CONF0_CH1 register accessor: an alias for `Reg<DMA_OUT_CONF0_CH1_SPEC>`"]
pub type DMA_OUT_CONF0_CH1 = crate::Reg<dma_out_conf0_ch1::DMA_OUT_CONF0_CH1_SPEC>;
#[doc = "DMA_OUT_CONF0_CH1"]
pub mod dma_out_conf0_ch1;
#[doc = "DMA_OUT_CONF1_CH1 register accessor: an alias for `Reg<DMA_OUT_CONF1_CH1_SPEC>`"]
pub type DMA_OUT_CONF1_CH1 = crate::Reg<dma_out_conf1_ch1::DMA_OUT_CONF1_CH1_SPEC>;
#[doc = "DMA_OUT_CONF1_CH1"]
pub mod dma_out_conf1_ch1;
#[doc = "DMA_OUTFIFO_STATUS_CH1 register accessor: an alias for `Reg<DMA_OUTFIFO_STATUS_CH1_SPEC>`"]
pub type DMA_OUTFIFO_STATUS_CH1 = crate::Reg<dma_outfifo_status_ch1::DMA_OUTFIFO_STATUS_CH1_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH1"]
pub mod dma_outfifo_status_ch1;
#[doc = "DMA_OUT_PUSH_CH1 register accessor: an alias for `Reg<DMA_OUT_PUSH_CH1_SPEC>`"]
pub type DMA_OUT_PUSH_CH1 = crate::Reg<dma_out_push_ch1::DMA_OUT_PUSH_CH1_SPEC>;
#[doc = "DMA_OUT_PUSH_CH1"]
pub mod dma_out_push_ch1;
#[doc = "DMA_OUT_LINK_CH1 register accessor: an alias for `Reg<DMA_OUT_LINK_CH1_SPEC>`"]
pub type DMA_OUT_LINK_CH1 = crate::Reg<dma_out_link_ch1::DMA_OUT_LINK_CH1_SPEC>;
#[doc = "DMA_OUT_LINK_CH1"]
pub mod dma_out_link_ch1;
#[doc = "DMA_OUT_STATE_CH1 register accessor: an alias for `Reg<DMA_OUT_STATE_CH1_SPEC>`"]
pub type DMA_OUT_STATE_CH1 = crate::Reg<dma_out_state_ch1::DMA_OUT_STATE_CH1_SPEC>;
#[doc = "DMA_OUT_STATE_CH1"]
pub mod dma_out_state_ch1;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1 register accessor: an alias for `Reg<DMA_OUT_EOF_DES_ADDR_CH1_SPEC>`"]
pub type DMA_OUT_EOF_DES_ADDR_CH1 =
    crate::Reg<dma_out_eof_des_addr_ch1::DMA_OUT_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1"]
pub mod dma_out_eof_des_addr_ch1;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1 register accessor: an alias for `Reg<DMA_OUT_EOF_BFR_DES_ADDR_CH1_SPEC>`"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH1 =
    crate::Reg<dma_out_eof_bfr_des_addr_ch1::DMA_OUT_EOF_BFR_DES_ADDR_CH1_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1"]
pub mod dma_out_eof_bfr_des_addr_ch1;
#[doc = "DMA_OUT_DSCR_CH1 register accessor: an alias for `Reg<DMA_OUT_DSCR_CH1_SPEC>`"]
pub type DMA_OUT_DSCR_CH1 = crate::Reg<dma_out_dscr_ch1::DMA_OUT_DSCR_CH1_SPEC>;
#[doc = "DMA_OUT_DSCR_CH1"]
pub mod dma_out_dscr_ch1;
#[doc = "DMA_OUT_DSCR_BF0_CH1 register accessor: an alias for `Reg<DMA_OUT_DSCR_BF0_CH1_SPEC>`"]
pub type DMA_OUT_DSCR_BF0_CH1 = crate::Reg<dma_out_dscr_bf0_ch1::DMA_OUT_DSCR_BF0_CH1_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH1"]
pub mod dma_out_dscr_bf0_ch1;
#[doc = "DMA_OUT_DSCR_BF1_CH1 register accessor: an alias for `Reg<DMA_OUT_DSCR_BF1_CH1_SPEC>`"]
pub type DMA_OUT_DSCR_BF1_CH1 = crate::Reg<dma_out_dscr_bf1_ch1::DMA_OUT_DSCR_BF1_CH1_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH1"]
pub mod dma_out_dscr_bf1_ch1;
#[doc = "DMA_OUT_PRI_CH1 register accessor: an alias for `Reg<DMA_OUT_PRI_CH1_SPEC>`"]
pub type DMA_OUT_PRI_CH1 = crate::Reg<dma_out_pri_ch1::DMA_OUT_PRI_CH1_SPEC>;
#[doc = "DMA_OUT_PRI_CH1"]
pub mod dma_out_pri_ch1;
#[doc = "DMA_OUT_PERI_SEL_CH1 register accessor: an alias for `Reg<DMA_OUT_PERI_SEL_CH1_SPEC>`"]
pub type DMA_OUT_PERI_SEL_CH1 = crate::Reg<dma_out_peri_sel_ch1::DMA_OUT_PERI_SEL_CH1_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH1"]
pub mod dma_out_peri_sel_ch1;
#[doc = "DMA_IN_CONF0_CH2 register accessor: an alias for `Reg<DMA_IN_CONF0_CH2_SPEC>`"]
pub type DMA_IN_CONF0_CH2 = crate::Reg<dma_in_conf0_ch2::DMA_IN_CONF0_CH2_SPEC>;
#[doc = "DMA_IN_CONF0_CH2"]
pub mod dma_in_conf0_ch2;
#[doc = "DMA_IN_CONF1_CH2 register accessor: an alias for `Reg<DMA_IN_CONF1_CH2_SPEC>`"]
pub type DMA_IN_CONF1_CH2 = crate::Reg<dma_in_conf1_ch2::DMA_IN_CONF1_CH2_SPEC>;
#[doc = "DMA_IN_CONF1_CH2"]
pub mod dma_in_conf1_ch2;
#[doc = "DMA_INFIFO_STATUS_CH2 register accessor: an alias for `Reg<DMA_INFIFO_STATUS_CH2_SPEC>`"]
pub type DMA_INFIFO_STATUS_CH2 = crate::Reg<dma_infifo_status_ch2::DMA_INFIFO_STATUS_CH2_SPEC>;
#[doc = "DMA_INFIFO_STATUS_CH2"]
pub mod dma_infifo_status_ch2;
#[doc = "DMA_IN_POP_CH2 register accessor: an alias for `Reg<DMA_IN_POP_CH2_SPEC>`"]
pub type DMA_IN_POP_CH2 = crate::Reg<dma_in_pop_ch2::DMA_IN_POP_CH2_SPEC>;
#[doc = "DMA_IN_POP_CH2"]
pub mod dma_in_pop_ch2;
#[doc = "DMA_IN_LINK_CH2 register accessor: an alias for `Reg<DMA_IN_LINK_CH2_SPEC>`"]
pub type DMA_IN_LINK_CH2 = crate::Reg<dma_in_link_ch2::DMA_IN_LINK_CH2_SPEC>;
#[doc = "DMA_IN_LINK_CH2"]
pub mod dma_in_link_ch2;
#[doc = "DMA_IN_STATE_CH2 register accessor: an alias for `Reg<DMA_IN_STATE_CH2_SPEC>`"]
pub type DMA_IN_STATE_CH2 = crate::Reg<dma_in_state_ch2::DMA_IN_STATE_CH2_SPEC>;
#[doc = "DMA_IN_STATE_CH2"]
pub mod dma_in_state_ch2;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2 register accessor: an alias for `Reg<DMA_IN_SUC_EOF_DES_ADDR_CH2_SPEC>`"]
pub type DMA_IN_SUC_EOF_DES_ADDR_CH2 =
    crate::Reg<dma_in_suc_eof_des_addr_ch2::DMA_IN_SUC_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_IN_SUC_EOF_DES_ADDR_CH2"]
pub mod dma_in_suc_eof_des_addr_ch2;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2 register accessor: an alias for `Reg<DMA_IN_ERR_EOF_DES_ADDR_CH2_SPEC>`"]
pub type DMA_IN_ERR_EOF_DES_ADDR_CH2 =
    crate::Reg<dma_in_err_eof_des_addr_ch2::DMA_IN_ERR_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH2"]
pub mod dma_in_err_eof_des_addr_ch2;
#[doc = "DMA_IN_DSCR_CH2 register accessor: an alias for `Reg<DMA_IN_DSCR_CH2_SPEC>`"]
pub type DMA_IN_DSCR_CH2 = crate::Reg<dma_in_dscr_ch2::DMA_IN_DSCR_CH2_SPEC>;
#[doc = "DMA_IN_DSCR_CH2"]
pub mod dma_in_dscr_ch2;
#[doc = "DMA_IN_DSCR_BF0_CH2 register accessor: an alias for `Reg<DMA_IN_DSCR_BF0_CH2_SPEC>`"]
pub type DMA_IN_DSCR_BF0_CH2 = crate::Reg<dma_in_dscr_bf0_ch2::DMA_IN_DSCR_BF0_CH2_SPEC>;
#[doc = "DMA_IN_DSCR_BF0_CH2"]
pub mod dma_in_dscr_bf0_ch2;
#[doc = "DMA_IN_DSCR_BF1_CH2 register accessor: an alias for `Reg<DMA_IN_DSCR_BF1_CH2_SPEC>`"]
pub type DMA_IN_DSCR_BF1_CH2 = crate::Reg<dma_in_dscr_bf1_ch2::DMA_IN_DSCR_BF1_CH2_SPEC>;
#[doc = "DMA_IN_DSCR_BF1_CH2"]
pub mod dma_in_dscr_bf1_ch2;
#[doc = "DMA_IN_PRI_CH2 register accessor: an alias for `Reg<DMA_IN_PRI_CH2_SPEC>`"]
pub type DMA_IN_PRI_CH2 = crate::Reg<dma_in_pri_ch2::DMA_IN_PRI_CH2_SPEC>;
#[doc = "DMA_IN_PRI_CH2"]
pub mod dma_in_pri_ch2;
#[doc = "DMA_IN_PERI_SEL_CH2 register accessor: an alias for `Reg<DMA_IN_PERI_SEL_CH2_SPEC>`"]
pub type DMA_IN_PERI_SEL_CH2 = crate::Reg<dma_in_peri_sel_ch2::DMA_IN_PERI_SEL_CH2_SPEC>;
#[doc = "DMA_IN_PERI_SEL_CH2"]
pub mod dma_in_peri_sel_ch2;
#[doc = "DMA_OUT_CONF0_CH2 register accessor: an alias for `Reg<DMA_OUT_CONF0_CH2_SPEC>`"]
pub type DMA_OUT_CONF0_CH2 = crate::Reg<dma_out_conf0_ch2::DMA_OUT_CONF0_CH2_SPEC>;
#[doc = "DMA_OUT_CONF0_CH2"]
pub mod dma_out_conf0_ch2;
#[doc = "DMA_OUT_CONF1_CH2 register accessor: an alias for `Reg<DMA_OUT_CONF1_CH2_SPEC>`"]
pub type DMA_OUT_CONF1_CH2 = crate::Reg<dma_out_conf1_ch2::DMA_OUT_CONF1_CH2_SPEC>;
#[doc = "DMA_OUT_CONF1_CH2"]
pub mod dma_out_conf1_ch2;
#[doc = "DMA_OUTFIFO_STATUS_CH2 register accessor: an alias for `Reg<DMA_OUTFIFO_STATUS_CH2_SPEC>`"]
pub type DMA_OUTFIFO_STATUS_CH2 = crate::Reg<dma_outfifo_status_ch2::DMA_OUTFIFO_STATUS_CH2_SPEC>;
#[doc = "DMA_OUTFIFO_STATUS_CH2"]
pub mod dma_outfifo_status_ch2;
#[doc = "DMA_OUT_PUSH_CH2 register accessor: an alias for `Reg<DMA_OUT_PUSH_CH2_SPEC>`"]
pub type DMA_OUT_PUSH_CH2 = crate::Reg<dma_out_push_ch2::DMA_OUT_PUSH_CH2_SPEC>;
#[doc = "DMA_OUT_PUSH_CH2"]
pub mod dma_out_push_ch2;
#[doc = "DMA_OUT_LINK_CH2 register accessor: an alias for `Reg<DMA_OUT_LINK_CH2_SPEC>`"]
pub type DMA_OUT_LINK_CH2 = crate::Reg<dma_out_link_ch2::DMA_OUT_LINK_CH2_SPEC>;
#[doc = "DMA_OUT_LINK_CH2"]
pub mod dma_out_link_ch2;
#[doc = "DMA_OUT_STATE_CH2 register accessor: an alias for `Reg<DMA_OUT_STATE_CH2_SPEC>`"]
pub type DMA_OUT_STATE_CH2 = crate::Reg<dma_out_state_ch2::DMA_OUT_STATE_CH2_SPEC>;
#[doc = "DMA_OUT_STATE_CH2"]
pub mod dma_out_state_ch2;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH2 register accessor: an alias for `Reg<DMA_OUT_EOF_DES_ADDR_CH2_SPEC>`"]
pub type DMA_OUT_EOF_DES_ADDR_CH2 =
    crate::Reg<dma_out_eof_des_addr_ch2::DMA_OUT_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_OUT_EOF_DES_ADDR_CH2"]
pub mod dma_out_eof_des_addr_ch2;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2 register accessor: an alias for `Reg<DMA_OUT_EOF_BFR_DES_ADDR_CH2_SPEC>`"]
pub type DMA_OUT_EOF_BFR_DES_ADDR_CH2 =
    crate::Reg<dma_out_eof_bfr_des_addr_ch2::DMA_OUT_EOF_BFR_DES_ADDR_CH2_SPEC>;
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH2"]
pub mod dma_out_eof_bfr_des_addr_ch2;
#[doc = "DMA_OUT_DSCR_CH2 register accessor: an alias for `Reg<DMA_OUT_DSCR_CH2_SPEC>`"]
pub type DMA_OUT_DSCR_CH2 = crate::Reg<dma_out_dscr_ch2::DMA_OUT_DSCR_CH2_SPEC>;
#[doc = "DMA_OUT_DSCR_CH2"]
pub mod dma_out_dscr_ch2;
#[doc = "DMA_OUT_DSCR_BF0_CH2 register accessor: an alias for `Reg<DMA_OUT_DSCR_BF0_CH2_SPEC>`"]
pub type DMA_OUT_DSCR_BF0_CH2 = crate::Reg<dma_out_dscr_bf0_ch2::DMA_OUT_DSCR_BF0_CH2_SPEC>;
#[doc = "DMA_OUT_DSCR_BF0_CH2"]
pub mod dma_out_dscr_bf0_ch2;
#[doc = "DMA_OUT_DSCR_BF1_CH2 register accessor: an alias for `Reg<DMA_OUT_DSCR_BF1_CH2_SPEC>`"]
pub type DMA_OUT_DSCR_BF1_CH2 = crate::Reg<dma_out_dscr_bf1_ch2::DMA_OUT_DSCR_BF1_CH2_SPEC>;
#[doc = "DMA_OUT_DSCR_BF1_CH2"]
pub mod dma_out_dscr_bf1_ch2;
#[doc = "DMA_OUT_PRI_CH2 register accessor: an alias for `Reg<DMA_OUT_PRI_CH2_SPEC>`"]
pub type DMA_OUT_PRI_CH2 = crate::Reg<dma_out_pri_ch2::DMA_OUT_PRI_CH2_SPEC>;
#[doc = "DMA_OUT_PRI_CH2"]
pub mod dma_out_pri_ch2;
#[doc = "DMA_OUT_PERI_SEL_CH2 register accessor: an alias for `Reg<DMA_OUT_PERI_SEL_CH2_SPEC>`"]
pub type DMA_OUT_PERI_SEL_CH2 = crate::Reg<dma_out_peri_sel_ch2::DMA_OUT_PERI_SEL_CH2_SPEC>;
#[doc = "DMA_OUT_PERI_SEL_CH2"]
pub mod dma_out_peri_sel_ch2;
