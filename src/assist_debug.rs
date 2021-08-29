#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ASSIST_DEBUG_CORE_0_INTR_ENA"]
    pub core_0_intr_ena: crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>,
    #[doc = "0x04 - ASSIST_DEBUG_CORE_0_INTR_RAW"]
    pub core_0_intr_raw: crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>,
    #[doc = "0x08 - ASSIST_DEBUG_CORE_0_INTR_RLS"]
    pub core_0_intr_rls: crate::Reg<core_0_intr_rls::CORE_0_INTR_RLS_SPEC>,
    #[doc = "0x0c - ASSIST_DEBUG_CORE_0_INTR_CLR"]
    pub core_0_intr_clr: crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>,
    #[doc = "0x10 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN"]
    pub core_0_area_dram0_0_min: crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>,
    #[doc = "0x14 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX"]
    pub core_0_area_dram0_0_max: crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>,
    #[doc = "0x18 - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN"]
    pub core_0_area_dram0_1_min: crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>,
    #[doc = "0x1c - ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX"]
    pub core_0_area_dram0_1_max: crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>,
    #[doc = "0x20 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN"]
    pub core_0_area_pif_0_min: crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>,
    #[doc = "0x24 - ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX"]
    pub core_0_area_pif_0_max: crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>,
    #[doc = "0x28 - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN"]
    pub core_0_area_pif_1_min: crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>,
    #[doc = "0x2c - ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX"]
    pub core_0_area_pif_1_max: crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>,
    #[doc = "0x30 - ASSIST_DEBUG_CORE_0_AREA_PC"]
    pub core_0_area_pc: crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>,
    #[doc = "0x34 - ASSIST_DEBUG_CORE_0_AREA_SP"]
    pub core_0_area_sp: crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>,
    #[doc = "0x38 - ASSIST_DEBUG_CORE_0_SP_MIN"]
    pub core_0_sp_min: crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>,
    #[doc = "0x3c - ASSIST_DEBUG_CORE_0_SP_MAX"]
    pub core_0_sp_max: crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>,
    #[doc = "0x40 - ASSIST_DEBUG_CORE_0_SP_PC"]
    pub core_0_sp_pc: crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>,
    #[doc = "0x44 - ASSIST_DEBUG_CORE_0_RCD_EN"]
    pub core_0_rcd_en: crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>,
    #[doc = "0x48 - ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC"]
    pub core_0_rcd_pdebugpc: crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>,
    #[doc = "0x4c - ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP"]
    pub core_0_rcd_pdebugsp: crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>,
    #[doc = "0x50 - ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0"]
    pub core_0_iram0_exception_monitor_0:
        crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>,
    #[doc = "0x54 - ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1"]
    pub core_0_iram0_exception_monitor_1:
        crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>,
    #[doc = "0x58 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0"]
    pub core_0_dram0_exception_monitor_0:
        crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>,
    #[doc = "0x5c - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1"]
    pub core_0_dram0_exception_monitor_1:
        crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>,
    #[doc = "0x60 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2"]
    pub core_0_dram0_exception_monitor_2:
        crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>,
    #[doc = "0x64 - ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3"]
    pub core_0_dram0_exception_monitor_3:
        crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>,
    #[doc = "0x68 - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0"]
    pub core_x_iram0_dram0_exception_monitor_0: crate::Reg<
        core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC,
    >,
    #[doc = "0x6c - ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1"]
    pub core_x_iram0_dram0_exception_monitor_1: crate::Reg<
        core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC,
    >,
    #[doc = "0x70 - ASSIST_DEBUG_LOG_SETTING"]
    pub log_setting: crate::Reg<log_setting::LOG_SETTING_SPEC>,
    #[doc = "0x74 - ASSIST_DEBUG_LOG_DATA_0"]
    pub log_data_0: crate::Reg<log_data_0::LOG_DATA_0_SPEC>,
    #[doc = "0x78 - ASSIST_DEBUG_LOG_DATA_MASK"]
    pub log_data_mask: crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>,
    #[doc = "0x7c - ASSIST_DEBUG_LOG_MIN"]
    pub log_min: crate::Reg<log_min::LOG_MIN_SPEC>,
    #[doc = "0x80 - ASSIST_DEBUG_LOG_MAX"]
    pub log_max: crate::Reg<log_max::LOG_MAX_SPEC>,
    #[doc = "0x84 - ASSIST_DEBUG_LOG_MEM_START"]
    pub log_mem_start: crate::Reg<log_mem_start::LOG_MEM_START_SPEC>,
    #[doc = "0x88 - ASSIST_DEBUG_LOG_MEM_END"]
    pub log_mem_end: crate::Reg<log_mem_end::LOG_MEM_END_SPEC>,
    #[doc = "0x8c - ASSIST_DEBUG_LOG_MEM_WRITING_ADDR"]
    pub log_mem_writing_addr: crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>,
    #[doc = "0x90 - ASSIST_DEBUG_LOG_MEM_FULL_FLAG"]
    pub log_mem_full_flag: crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>,
    #[doc = "0x94 - ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
    pub c0re_0_lastpc_before_exception:
        crate::Reg<c0re_0_lastpc_before_exception::C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>,
    #[doc = "0x98 - ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
    pub c0re_0_debug_mode: crate::Reg<c0re_0_debug_mode::C0RE_0_DEBUG_MODE_SPEC>,
    _reserved39: [u8; 0x0160],
    #[doc = "0x1fc - ASSIST_DEBUG_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CORE_0_INTR_ENA register accessor: an alias for `Reg<CORE_0_INTR_ENA_SPEC>`"]
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_ENA"]
pub mod core_0_intr_ena;
#[doc = "CORE_0_INTR_RAW register accessor: an alias for `Reg<CORE_0_INTR_RAW_SPEC>`"]
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RAW"]
pub mod core_0_intr_raw;
#[doc = "CORE_0_INTR_RLS register accessor: an alias for `Reg<CORE_0_INTR_RLS_SPEC>`"]
pub type CORE_0_INTR_RLS = crate::Reg<core_0_intr_rls::CORE_0_INTR_RLS_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_RLS"]
pub mod core_0_intr_rls;
#[doc = "CORE_0_INTR_CLR register accessor: an alias for `Reg<CORE_0_INTR_CLR_SPEC>`"]
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_INTR_CLR"]
pub mod core_0_intr_clr;
#[doc = "CORE_0_AREA_DRAM0_0_MIN register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MIN"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_0_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_0_MAX"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MIN_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MIN"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX register accessor: an alias for `Reg<CORE_0_AREA_DRAM0_1_MAX_SPEC>`"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_DRAM0_1_MAX"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MIN"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX register accessor: an alias for `Reg<CORE_0_AREA_PIF_0_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_0_MAX"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MIN_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MIN"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX register accessor: an alias for `Reg<CORE_0_AREA_PIF_1_MAX_SPEC>`"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PIF_1_MAX"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_PC register accessor: an alias for `Reg<CORE_0_AREA_PC_SPEC>`"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC"]
pub mod core_0_area_pc;
#[doc = "CORE_0_AREA_SP register accessor: an alias for `Reg<CORE_0_AREA_SP_SPEC>`"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_AREA_SP"]
pub mod core_0_area_sp;
#[doc = "CORE_0_SP_MIN register accessor: an alias for `Reg<CORE_0_SP_MIN_SPEC>`"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MIN"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX register accessor: an alias for `Reg<CORE_0_SP_MAX_SPEC>`"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_MAX"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC register accessor: an alias for `Reg<CORE_0_SP_PC_SPEC>`"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_SP_PC"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_EN register accessor: an alias for `Reg<CORE_0_RCD_EN_SPEC>`"]
pub type CORE_0_RCD_EN = crate::Reg<core_0_rcd_en::CORE_0_RCD_EN_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_EN"]
pub mod core_0_rcd_en;
#[doc = "CORE_0_RCD_PDEBUGPC register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGPC_SPEC>`"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGPC"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGSP register accessor: an alias for `Reg<CORE_0_RCD_PDEBUGSP_SPEC>`"]
pub type CORE_0_RCD_PDEBUGSP = crate::Reg<core_0_rcd_pdebugsp::CORE_0_RCD_PDEBUGSP_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_RCD_PDEBUGSP"]
pub mod core_0_rcd_pdebugsp;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_0"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_IRAM0_EXCEPTION_MONITOR_1"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_0"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_2"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 register accessor: an alias for `Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>`"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_3"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 register accessor: an alias for `Reg<CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>`"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "ASSIST_DEBUG_CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "LOG_SETTING register accessor: an alias for `Reg<LOG_SETTING_SPEC>`"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_SETTING"]
pub mod log_setting;
#[doc = "LOG_DATA_0 register accessor: an alias for `Reg<LOG_DATA_0_SPEC>`"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_DATA_0"]
pub mod log_data_0;
#[doc = "LOG_DATA_MASK register accessor: an alias for `Reg<LOG_DATA_MASK_SPEC>`"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK"]
pub mod log_data_mask;
#[doc = "LOG_MIN register accessor: an alias for `Reg<LOG_MIN_SPEC>`"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MIN"]
pub mod log_min;
#[doc = "LOG_MAX register accessor: an alias for `Reg<LOG_MAX_SPEC>`"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MAX"]
pub mod log_max;
#[doc = "LOG_MEM_START register accessor: an alias for `Reg<LOG_MEM_START_SPEC>`"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_START"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END register accessor: an alias for `Reg<LOG_MEM_END_SPEC>`"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_END"]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR register accessor: an alias for `Reg<LOG_MEM_WRITING_ADDR_SPEC>`"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_WRITING_ADDR"]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG register accessor: an alias for `Reg<LOG_MEM_FULL_FLAG_SPEC>`"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "ASSIST_DEBUG_LOG_MEM_FULL_FLAG"]
pub mod log_mem_full_flag;
#[doc = "C0RE_0_LASTPC_BEFORE_EXCEPTION register accessor: an alias for `Reg<C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>`"]
pub type C0RE_0_LASTPC_BEFORE_EXCEPTION =
    crate::Reg<c0re_0_lastpc_before_exception::C0RE_0_LASTPC_BEFORE_EXCEPTION_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_LASTPC_BEFORE_EXCEPTION"]
pub mod c0re_0_lastpc_before_exception;
#[doc = "C0RE_0_DEBUG_MODE register accessor: an alias for `Reg<C0RE_0_DEBUG_MODE_SPEC>`"]
pub type C0RE_0_DEBUG_MODE = crate::Reg<c0re_0_debug_mode::C0RE_0_DEBUG_MODE_SPEC>;
#[doc = "ASSIST_DEBUG_C0RE_0_DEBUG_MODE"]
pub mod c0re_0_debug_mode;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "ASSIST_DEBUG_DATE"]
pub mod date;
