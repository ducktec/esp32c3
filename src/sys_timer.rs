#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYS_TIMER_SYSTIMER_CONF"]
    pub systimer_conf: crate::Reg<systimer_conf::SYSTIMER_CONF_SPEC>,
    #[doc = "0x04 - SYS_TIMER_SYSTIMER_UNIT0_OP"]
    pub systimer_unit0_op: crate::Reg<systimer_unit0_op::SYSTIMER_UNIT0_OP_SPEC>,
    #[doc = "0x08 - SYS_TIMER_SYSTIMER_UNIT1_OP"]
    pub systimer_unit1_op: crate::Reg<systimer_unit1_op::SYSTIMER_UNIT1_OP_SPEC>,
    #[doc = "0x0c - SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI"]
    pub systimer_unit0_load_hi: crate::Reg<systimer_unit0_load_hi::SYSTIMER_UNIT0_LOAD_HI_SPEC>,
    #[doc = "0x10 - SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO"]
    pub systimer_unit0_load_lo: crate::Reg<systimer_unit0_load_lo::SYSTIMER_UNIT0_LOAD_LO_SPEC>,
    #[doc = "0x14 - SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI"]
    pub systimer_unit1_load_hi: crate::Reg<systimer_unit1_load_hi::SYSTIMER_UNIT1_LOAD_HI_SPEC>,
    #[doc = "0x18 - SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO"]
    pub systimer_unit1_load_lo: crate::Reg<systimer_unit1_load_lo::SYSTIMER_UNIT1_LOAD_LO_SPEC>,
    #[doc = "0x1c - SYS_TIMER_SYSTIMER_TARGET0_HI"]
    pub systimer_target0_hi: crate::Reg<systimer_target0_hi::SYSTIMER_TARGET0_HI_SPEC>,
    #[doc = "0x20 - SYS_TIMER_SYSTIMER_TARGET0_LO"]
    pub systimer_target0_lo: crate::Reg<systimer_target0_lo::SYSTIMER_TARGET0_LO_SPEC>,
    #[doc = "0x24 - SYS_TIMER_SYSTIMER_TARGET1_HI"]
    pub systimer_target1_hi: crate::Reg<systimer_target1_hi::SYSTIMER_TARGET1_HI_SPEC>,
    #[doc = "0x28 - SYS_TIMER_SYSTIMER_TARGET1_LO"]
    pub systimer_target1_lo: crate::Reg<systimer_target1_lo::SYSTIMER_TARGET1_LO_SPEC>,
    #[doc = "0x2c - SYS_TIMER_SYSTIMER_TARGET2_HI"]
    pub systimer_target2_hi: crate::Reg<systimer_target2_hi::SYSTIMER_TARGET2_HI_SPEC>,
    #[doc = "0x30 - SYS_TIMER_SYSTIMER_TARGET2_LO"]
    pub systimer_target2_lo: crate::Reg<systimer_target2_lo::SYSTIMER_TARGET2_LO_SPEC>,
    #[doc = "0x34 - SYS_TIMER_SYSTIMER_TARGET0_CONF"]
    pub systimer_target0_conf: crate::Reg<systimer_target0_conf::SYSTIMER_TARGET0_CONF_SPEC>,
    #[doc = "0x38 - SYS_TIMER_SYSTIMER_TARGET1_CONF"]
    pub systimer_target1_conf: crate::Reg<systimer_target1_conf::SYSTIMER_TARGET1_CONF_SPEC>,
    #[doc = "0x3c - SYS_TIMER_SYSTIMER_TARGET2_CONF"]
    pub systimer_target2_conf: crate::Reg<systimer_target2_conf::SYSTIMER_TARGET2_CONF_SPEC>,
    #[doc = "0x40 - SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI"]
    pub systimer_unit0_value_hi: crate::Reg<systimer_unit0_value_hi::SYSTIMER_UNIT0_VALUE_HI_SPEC>,
    #[doc = "0x44 - SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO"]
    pub systimer_unit0_value_lo: crate::Reg<systimer_unit0_value_lo::SYSTIMER_UNIT0_VALUE_LO_SPEC>,
    #[doc = "0x48 - SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI"]
    pub systimer_unit1_value_hi: crate::Reg<systimer_unit1_value_hi::SYSTIMER_UNIT1_VALUE_HI_SPEC>,
    #[doc = "0x4c - SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO"]
    pub systimer_unit1_value_lo: crate::Reg<systimer_unit1_value_lo::SYSTIMER_UNIT1_VALUE_LO_SPEC>,
    #[doc = "0x50 - SYS_TIMER_SYSTIMER_COMP0_LOAD"]
    pub systimer_comp0_load: crate::Reg<systimer_comp0_load::SYSTIMER_COMP0_LOAD_SPEC>,
    #[doc = "0x54 - SYS_TIMER_SYSTIMER_COMP1_LOAD"]
    pub systimer_comp1_load: crate::Reg<systimer_comp1_load::SYSTIMER_COMP1_LOAD_SPEC>,
    #[doc = "0x58 - SYS_TIMER_SYSTIMER_COMP2_LOAD"]
    pub systimer_comp2_load: crate::Reg<systimer_comp2_load::SYSTIMER_COMP2_LOAD_SPEC>,
    #[doc = "0x5c - SYS_TIMER_SYSTIMER_UNIT0_LOAD"]
    pub systimer_unit0_load: crate::Reg<systimer_unit0_load::SYSTIMER_UNIT0_LOAD_SPEC>,
    #[doc = "0x60 - SYS_TIMER_SYSTIMER_UNIT1_LOAD"]
    pub systimer_unit1_load: crate::Reg<systimer_unit1_load::SYSTIMER_UNIT1_LOAD_SPEC>,
    #[doc = "0x64 - SYS_TIMER_SYSTIMER_INT_ENA"]
    pub systimer_int_ena: crate::Reg<systimer_int_ena::SYSTIMER_INT_ENA_SPEC>,
    #[doc = "0x68 - SYS_TIMER_SYSTIMER_INT_RAW"]
    pub systimer_int_raw: crate::Reg<systimer_int_raw::SYSTIMER_INT_RAW_SPEC>,
    #[doc = "0x6c - SYS_TIMER_SYSTIMER_INT_CLR"]
    pub systimer_int_clr: crate::Reg<systimer_int_clr::SYSTIMER_INT_CLR_SPEC>,
    #[doc = "0x70 - SYS_TIMER_SYSTIMER_INT_ST"]
    pub systimer_int_st: crate::Reg<systimer_int_st::SYSTIMER_INT_ST_SPEC>,
    _reserved29: [u8; 0x88],
    #[doc = "0xfc - SYS_TIMER_SYSTIMER_DATE"]
    pub systimer_date: crate::Reg<systimer_date::SYSTIMER_DATE_SPEC>,
}
#[doc = "SYSTIMER_CONF register accessor: an alias for `Reg<SYSTIMER_CONF_SPEC>`"]
pub type SYSTIMER_CONF = crate::Reg<systimer_conf::SYSTIMER_CONF_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_CONF"]
pub mod systimer_conf;
#[doc = "SYSTIMER_UNIT0_OP register accessor: an alias for `Reg<SYSTIMER_UNIT0_OP_SPEC>`"]
pub type SYSTIMER_UNIT0_OP = crate::Reg<systimer_unit0_op::SYSTIMER_UNIT0_OP_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_OP"]
pub mod systimer_unit0_op;
#[doc = "SYSTIMER_UNIT1_OP register accessor: an alias for `Reg<SYSTIMER_UNIT1_OP_SPEC>`"]
pub type SYSTIMER_UNIT1_OP = crate::Reg<systimer_unit1_op::SYSTIMER_UNIT1_OP_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_OP"]
pub mod systimer_unit1_op;
#[doc = "SYSTIMER_UNIT0_LOAD_HI register accessor: an alias for `Reg<SYSTIMER_UNIT0_LOAD_HI_SPEC>`"]
pub type SYSTIMER_UNIT0_LOAD_HI = crate::Reg<systimer_unit0_load_hi::SYSTIMER_UNIT0_LOAD_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD_HI"]
pub mod systimer_unit0_load_hi;
#[doc = "SYSTIMER_UNIT0_LOAD_LO register accessor: an alias for `Reg<SYSTIMER_UNIT0_LOAD_LO_SPEC>`"]
pub type SYSTIMER_UNIT0_LOAD_LO = crate::Reg<systimer_unit0_load_lo::SYSTIMER_UNIT0_LOAD_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD_LO"]
pub mod systimer_unit0_load_lo;
#[doc = "SYSTIMER_UNIT1_LOAD_HI register accessor: an alias for `Reg<SYSTIMER_UNIT1_LOAD_HI_SPEC>`"]
pub type SYSTIMER_UNIT1_LOAD_HI = crate::Reg<systimer_unit1_load_hi::SYSTIMER_UNIT1_LOAD_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_HI"]
pub mod systimer_unit1_load_hi;
#[doc = "SYSTIMER_UNIT1_LOAD_LO register accessor: an alias for `Reg<SYSTIMER_UNIT1_LOAD_LO_SPEC>`"]
pub type SYSTIMER_UNIT1_LOAD_LO = crate::Reg<systimer_unit1_load_lo::SYSTIMER_UNIT1_LOAD_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD_LO"]
pub mod systimer_unit1_load_lo;
#[doc = "SYSTIMER_TARGET0_HI register accessor: an alias for `Reg<SYSTIMER_TARGET0_HI_SPEC>`"]
pub type SYSTIMER_TARGET0_HI = crate::Reg<systimer_target0_hi::SYSTIMER_TARGET0_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_HI"]
pub mod systimer_target0_hi;
#[doc = "SYSTIMER_TARGET0_LO register accessor: an alias for `Reg<SYSTIMER_TARGET0_LO_SPEC>`"]
pub type SYSTIMER_TARGET0_LO = crate::Reg<systimer_target0_lo::SYSTIMER_TARGET0_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_LO"]
pub mod systimer_target0_lo;
#[doc = "SYSTIMER_TARGET1_HI register accessor: an alias for `Reg<SYSTIMER_TARGET1_HI_SPEC>`"]
pub type SYSTIMER_TARGET1_HI = crate::Reg<systimer_target1_hi::SYSTIMER_TARGET1_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_HI"]
pub mod systimer_target1_hi;
#[doc = "SYSTIMER_TARGET1_LO register accessor: an alias for `Reg<SYSTIMER_TARGET1_LO_SPEC>`"]
pub type SYSTIMER_TARGET1_LO = crate::Reg<systimer_target1_lo::SYSTIMER_TARGET1_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_LO"]
pub mod systimer_target1_lo;
#[doc = "SYSTIMER_TARGET2_HI register accessor: an alias for `Reg<SYSTIMER_TARGET2_HI_SPEC>`"]
pub type SYSTIMER_TARGET2_HI = crate::Reg<systimer_target2_hi::SYSTIMER_TARGET2_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_HI"]
pub mod systimer_target2_hi;
#[doc = "SYSTIMER_TARGET2_LO register accessor: an alias for `Reg<SYSTIMER_TARGET2_LO_SPEC>`"]
pub type SYSTIMER_TARGET2_LO = crate::Reg<systimer_target2_lo::SYSTIMER_TARGET2_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_LO"]
pub mod systimer_target2_lo;
#[doc = "SYSTIMER_TARGET0_CONF register accessor: an alias for `Reg<SYSTIMER_TARGET0_CONF_SPEC>`"]
pub type SYSTIMER_TARGET0_CONF = crate::Reg<systimer_target0_conf::SYSTIMER_TARGET0_CONF_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET0_CONF"]
pub mod systimer_target0_conf;
#[doc = "SYSTIMER_TARGET1_CONF register accessor: an alias for `Reg<SYSTIMER_TARGET1_CONF_SPEC>`"]
pub type SYSTIMER_TARGET1_CONF = crate::Reg<systimer_target1_conf::SYSTIMER_TARGET1_CONF_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET1_CONF"]
pub mod systimer_target1_conf;
#[doc = "SYSTIMER_TARGET2_CONF register accessor: an alias for `Reg<SYSTIMER_TARGET2_CONF_SPEC>`"]
pub type SYSTIMER_TARGET2_CONF = crate::Reg<systimer_target2_conf::SYSTIMER_TARGET2_CONF_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_TARGET2_CONF"]
pub mod systimer_target2_conf;
#[doc = "SYSTIMER_UNIT0_VALUE_HI register accessor: an alias for `Reg<SYSTIMER_UNIT0_VALUE_HI_SPEC>`"]
pub type SYSTIMER_UNIT0_VALUE_HI =
    crate::Reg<systimer_unit0_value_hi::SYSTIMER_UNIT0_VALUE_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_HI"]
pub mod systimer_unit0_value_hi;
#[doc = "SYSTIMER_UNIT0_VALUE_LO register accessor: an alias for `Reg<SYSTIMER_UNIT0_VALUE_LO_SPEC>`"]
pub type SYSTIMER_UNIT0_VALUE_LO =
    crate::Reg<systimer_unit0_value_lo::SYSTIMER_UNIT0_VALUE_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_VALUE_LO"]
pub mod systimer_unit0_value_lo;
#[doc = "SYSTIMER_UNIT1_VALUE_HI register accessor: an alias for `Reg<SYSTIMER_UNIT1_VALUE_HI_SPEC>`"]
pub type SYSTIMER_UNIT1_VALUE_HI =
    crate::Reg<systimer_unit1_value_hi::SYSTIMER_UNIT1_VALUE_HI_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_VALUE_HI"]
pub mod systimer_unit1_value_hi;
#[doc = "SYSTIMER_UNIT1_VALUE_LO register accessor: an alias for `Reg<SYSTIMER_UNIT1_VALUE_LO_SPEC>`"]
pub type SYSTIMER_UNIT1_VALUE_LO =
    crate::Reg<systimer_unit1_value_lo::SYSTIMER_UNIT1_VALUE_LO_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_VALUE_LO"]
pub mod systimer_unit1_value_lo;
#[doc = "SYSTIMER_COMP0_LOAD register accessor: an alias for `Reg<SYSTIMER_COMP0_LOAD_SPEC>`"]
pub type SYSTIMER_COMP0_LOAD = crate::Reg<systimer_comp0_load::SYSTIMER_COMP0_LOAD_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_COMP0_LOAD"]
pub mod systimer_comp0_load;
#[doc = "SYSTIMER_COMP1_LOAD register accessor: an alias for `Reg<SYSTIMER_COMP1_LOAD_SPEC>`"]
pub type SYSTIMER_COMP1_LOAD = crate::Reg<systimer_comp1_load::SYSTIMER_COMP1_LOAD_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_COMP1_LOAD"]
pub mod systimer_comp1_load;
#[doc = "SYSTIMER_COMP2_LOAD register accessor: an alias for `Reg<SYSTIMER_COMP2_LOAD_SPEC>`"]
pub type SYSTIMER_COMP2_LOAD = crate::Reg<systimer_comp2_load::SYSTIMER_COMP2_LOAD_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_COMP2_LOAD"]
pub mod systimer_comp2_load;
#[doc = "SYSTIMER_UNIT0_LOAD register accessor: an alias for `Reg<SYSTIMER_UNIT0_LOAD_SPEC>`"]
pub type SYSTIMER_UNIT0_LOAD = crate::Reg<systimer_unit0_load::SYSTIMER_UNIT0_LOAD_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT0_LOAD"]
pub mod systimer_unit0_load;
#[doc = "SYSTIMER_UNIT1_LOAD register accessor: an alias for `Reg<SYSTIMER_UNIT1_LOAD_SPEC>`"]
pub type SYSTIMER_UNIT1_LOAD = crate::Reg<systimer_unit1_load::SYSTIMER_UNIT1_LOAD_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_UNIT1_LOAD"]
pub mod systimer_unit1_load;
#[doc = "SYSTIMER_INT_ENA register accessor: an alias for `Reg<SYSTIMER_INT_ENA_SPEC>`"]
pub type SYSTIMER_INT_ENA = crate::Reg<systimer_int_ena::SYSTIMER_INT_ENA_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_INT_ENA"]
pub mod systimer_int_ena;
#[doc = "SYSTIMER_INT_RAW register accessor: an alias for `Reg<SYSTIMER_INT_RAW_SPEC>`"]
pub type SYSTIMER_INT_RAW = crate::Reg<systimer_int_raw::SYSTIMER_INT_RAW_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_INT_RAW"]
pub mod systimer_int_raw;
#[doc = "SYSTIMER_INT_CLR register accessor: an alias for `Reg<SYSTIMER_INT_CLR_SPEC>`"]
pub type SYSTIMER_INT_CLR = crate::Reg<systimer_int_clr::SYSTIMER_INT_CLR_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_INT_CLR"]
pub mod systimer_int_clr;
#[doc = "SYSTIMER_INT_ST register accessor: an alias for `Reg<SYSTIMER_INT_ST_SPEC>`"]
pub type SYSTIMER_INT_ST = crate::Reg<systimer_int_st::SYSTIMER_INT_ST_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_INT_ST"]
pub mod systimer_int_st;
#[doc = "SYSTIMER_DATE register accessor: an alias for `Reg<SYSTIMER_DATE_SPEC>`"]
pub type SYSTIMER_DATE = crate::Reg<systimer_date::SYSTIMER_DATE_SPEC>;
#[doc = "SYS_TIMER_SYSTIMER_DATE"]
pub mod systimer_date;
