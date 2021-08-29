#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - APB_SARADC_CTRL"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - APB_SARADC_CTRL2"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x08 - APB_SARADC_FILTER_CTRL1"]
    pub filter_ctrl1: crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>,
    #[doc = "0x0c - APB_SARADC_FSM_WAIT"]
    pub fsm_wait: crate::Reg<fsm_wait::FSM_WAIT_SPEC>,
    #[doc = "0x10 - APB_SARADC_SAR1_STATUS"]
    pub sar1_status: crate::Reg<sar1_status::SAR1_STATUS_SPEC>,
    #[doc = "0x14 - APB_SARADC_SAR2_STATUS"]
    pub sar2_status: crate::Reg<sar2_status::SAR2_STATUS_SPEC>,
    #[doc = "0x18 - APB_SARADC_SAR_PATT_TAB1"]
    pub sar_patt_tab1: crate::Reg<sar_patt_tab1::SAR_PATT_TAB1_SPEC>,
    #[doc = "0x1c - APB_SARADC_SAR_PATT_TAB2"]
    pub sar_patt_tab2: crate::Reg<sar_patt_tab2::SAR_PATT_TAB2_SPEC>,
    #[doc = "0x20 - APB_SARADC_ONETIME_SAMPLE"]
    pub onetime_sample: crate::Reg<onetime_sample::ONETIME_SAMPLE_SPEC>,
    #[doc = "0x24 - APB_SARADC_APB_ADC_ARB_CTRL"]
    pub apb_adc_arb_ctrl: crate::Reg<apb_adc_arb_ctrl::APB_ADC_ARB_CTRL_SPEC>,
    #[doc = "0x28 - APB_SARADC_FILTER_CTRL0"]
    pub filter_ctrl0: crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>,
    #[doc = "0x2c - APB_SARADC_1_DATA_STATUS"]
    pub _1_data_status: crate::Reg<_1_data_status::_1_DATA_STATUS_SPEC>,
    #[doc = "0x30 - APB_SARADC_2_DATA_STATUS"]
    pub _2_data_status: crate::Reg<_2_data_status::_2_DATA_STATUS_SPEC>,
    #[doc = "0x34 - APB_SARADC_THRES0_CTRL"]
    pub thres0_ctrl: crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>,
    #[doc = "0x38 - APB_SARADC_THRES1_CTRL"]
    pub thres1_ctrl: crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>,
    #[doc = "0x3c - APB_SARADC_THRES_CTRL"]
    pub thres_ctrl: crate::Reg<thres_ctrl::THRES_CTRL_SPEC>,
    #[doc = "0x40 - APB_SARADC_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x44 - APB_SARADC_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x48 - APB_SARADC_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x4c - APB_SARADC_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x50 - APB_SARADC_DMA_CONF"]
    pub dma_conf: crate::Reg<dma_conf::DMA_CONF_SPEC>,
    #[doc = "0x54 - APB_SARADC_APB_ADC_CLKM_CONF"]
    pub apb_adc_clkm_conf: crate::Reg<apb_adc_clkm_conf::APB_ADC_CLKM_CONF_SPEC>,
    #[doc = "0x58 - APB_SARADC_APB_TSENS_CTRL"]
    pub apb_tsens_ctrl: crate::Reg<apb_tsens_ctrl::APB_TSENS_CTRL_SPEC>,
    #[doc = "0x5c - APB_SARADC_APB_TSENS_CTRL2"]
    pub apb_tsens_ctrl2: crate::Reg<apb_tsens_ctrl2::APB_TSENS_CTRL2_SPEC>,
    #[doc = "0x60 - APB_SARADC_CALI"]
    pub cali: crate::Reg<cali::CALI_SPEC>,
    _reserved25: [u8; 0x0398],
    #[doc = "0x3fc - APB_SARADC_APB_CTRL_DATE"]
    pub apb_ctrl_date: crate::Reg<apb_ctrl_date::APB_CTRL_DATE_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "APB_SARADC_CTRL"]
pub mod ctrl;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "APB_SARADC_CTRL2"]
pub mod ctrl2;
#[doc = "FILTER_CTRL1 register accessor: an alias for `Reg<FILTER_CTRL1_SPEC>`"]
pub type FILTER_CTRL1 = crate::Reg<filter_ctrl1::FILTER_CTRL1_SPEC>;
#[doc = "APB_SARADC_FILTER_CTRL1"]
pub mod filter_ctrl1;
#[doc = "FSM_WAIT register accessor: an alias for `Reg<FSM_WAIT_SPEC>`"]
pub type FSM_WAIT = crate::Reg<fsm_wait::FSM_WAIT_SPEC>;
#[doc = "APB_SARADC_FSM_WAIT"]
pub mod fsm_wait;
#[doc = "SAR1_STATUS register accessor: an alias for `Reg<SAR1_STATUS_SPEC>`"]
pub type SAR1_STATUS = crate::Reg<sar1_status::SAR1_STATUS_SPEC>;
#[doc = "APB_SARADC_SAR1_STATUS"]
pub mod sar1_status;
#[doc = "SAR2_STATUS register accessor: an alias for `Reg<SAR2_STATUS_SPEC>`"]
pub type SAR2_STATUS = crate::Reg<sar2_status::SAR2_STATUS_SPEC>;
#[doc = "APB_SARADC_SAR2_STATUS"]
pub mod sar2_status;
#[doc = "SAR_PATT_TAB1 register accessor: an alias for `Reg<SAR_PATT_TAB1_SPEC>`"]
pub type SAR_PATT_TAB1 = crate::Reg<sar_patt_tab1::SAR_PATT_TAB1_SPEC>;
#[doc = "APB_SARADC_SAR_PATT_TAB1"]
pub mod sar_patt_tab1;
#[doc = "SAR_PATT_TAB2 register accessor: an alias for `Reg<SAR_PATT_TAB2_SPEC>`"]
pub type SAR_PATT_TAB2 = crate::Reg<sar_patt_tab2::SAR_PATT_TAB2_SPEC>;
#[doc = "APB_SARADC_SAR_PATT_TAB2"]
pub mod sar_patt_tab2;
#[doc = "ONETIME_SAMPLE register accessor: an alias for `Reg<ONETIME_SAMPLE_SPEC>`"]
pub type ONETIME_SAMPLE = crate::Reg<onetime_sample::ONETIME_SAMPLE_SPEC>;
#[doc = "APB_SARADC_ONETIME_SAMPLE"]
pub mod onetime_sample;
#[doc = "APB_ADC_ARB_CTRL register accessor: an alias for `Reg<APB_ADC_ARB_CTRL_SPEC>`"]
pub type APB_ADC_ARB_CTRL = crate::Reg<apb_adc_arb_ctrl::APB_ADC_ARB_CTRL_SPEC>;
#[doc = "APB_SARADC_APB_ADC_ARB_CTRL"]
pub mod apb_adc_arb_ctrl;
#[doc = "FILTER_CTRL0 register accessor: an alias for `Reg<FILTER_CTRL0_SPEC>`"]
pub type FILTER_CTRL0 = crate::Reg<filter_ctrl0::FILTER_CTRL0_SPEC>;
#[doc = "APB_SARADC_FILTER_CTRL0"]
pub mod filter_ctrl0;
#[doc = "1_DATA_STATUS register accessor: an alias for `Reg<_1_DATA_STATUS_SPEC>`"]
pub type _1_DATA_STATUS = crate::Reg<_1_data_status::_1_DATA_STATUS_SPEC>;
#[doc = "APB_SARADC_1_DATA_STATUS"]
pub mod _1_data_status;
#[doc = "2_DATA_STATUS register accessor: an alias for `Reg<_2_DATA_STATUS_SPEC>`"]
pub type _2_DATA_STATUS = crate::Reg<_2_data_status::_2_DATA_STATUS_SPEC>;
#[doc = "APB_SARADC_2_DATA_STATUS"]
pub mod _2_data_status;
#[doc = "THRES0_CTRL register accessor: an alias for `Reg<THRES0_CTRL_SPEC>`"]
pub type THRES0_CTRL = crate::Reg<thres0_ctrl::THRES0_CTRL_SPEC>;
#[doc = "APB_SARADC_THRES0_CTRL"]
pub mod thres0_ctrl;
#[doc = "THRES1_CTRL register accessor: an alias for `Reg<THRES1_CTRL_SPEC>`"]
pub type THRES1_CTRL = crate::Reg<thres1_ctrl::THRES1_CTRL_SPEC>;
#[doc = "APB_SARADC_THRES1_CTRL"]
pub mod thres1_ctrl;
#[doc = "THRES_CTRL register accessor: an alias for `Reg<THRES_CTRL_SPEC>`"]
pub type THRES_CTRL = crate::Reg<thres_ctrl::THRES_CTRL_SPEC>;
#[doc = "APB_SARADC_THRES_CTRL"]
pub mod thres_ctrl;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "APB_SARADC_INT_ENA"]
pub mod int_ena;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "APB_SARADC_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "APB_SARADC_INT_ST"]
pub mod int_st;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "APB_SARADC_INT_CLR"]
pub mod int_clr;
#[doc = "DMA_CONF register accessor: an alias for `Reg<DMA_CONF_SPEC>`"]
pub type DMA_CONF = crate::Reg<dma_conf::DMA_CONF_SPEC>;
#[doc = "APB_SARADC_DMA_CONF"]
pub mod dma_conf;
#[doc = "APB_ADC_CLKM_CONF register accessor: an alias for `Reg<APB_ADC_CLKM_CONF_SPEC>`"]
pub type APB_ADC_CLKM_CONF = crate::Reg<apb_adc_clkm_conf::APB_ADC_CLKM_CONF_SPEC>;
#[doc = "APB_SARADC_APB_ADC_CLKM_CONF"]
pub mod apb_adc_clkm_conf;
#[doc = "APB_TSENS_CTRL register accessor: an alias for `Reg<APB_TSENS_CTRL_SPEC>`"]
pub type APB_TSENS_CTRL = crate::Reg<apb_tsens_ctrl::APB_TSENS_CTRL_SPEC>;
#[doc = "APB_SARADC_APB_TSENS_CTRL"]
pub mod apb_tsens_ctrl;
#[doc = "APB_TSENS_CTRL2 register accessor: an alias for `Reg<APB_TSENS_CTRL2_SPEC>`"]
pub type APB_TSENS_CTRL2 = crate::Reg<apb_tsens_ctrl2::APB_TSENS_CTRL2_SPEC>;
#[doc = "APB_SARADC_APB_TSENS_CTRL2"]
pub mod apb_tsens_ctrl2;
#[doc = "CALI register accessor: an alias for `Reg<CALI_SPEC>`"]
pub type CALI = crate::Reg<cali::CALI_SPEC>;
#[doc = "APB_SARADC_CALI"]
pub mod cali;
#[doc = "APB_CTRL_DATE register accessor: an alias for `Reg<APB_CTRL_DATE_SPEC>`"]
pub type APB_CTRL_DATE = crate::Reg<apb_ctrl_date::APB_CTRL_DATE_SPEC>;
#[doc = "APB_SARADC_APB_CTRL_DATE"]
pub mod apb_ctrl_date;
