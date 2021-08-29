#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTMEM_ICACHE_CTRL"]
    pub icache_ctrl: crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>,
    #[doc = "0x04 - EXTMEM_ICACHE_CTRL1"]
    pub icache_ctrl1: crate::Reg<icache_ctrl1::ICACHE_CTRL1_SPEC>,
    #[doc = "0x08 - EXTMEM_ICACHE_TAG_POWER_CTRL"]
    pub icache_tag_power_ctrl: crate::Reg<icache_tag_power_ctrl::ICACHE_TAG_POWER_CTRL_SPEC>,
    #[doc = "0x0c - EXTMEM_ICACHE_PRELOCK_CTRL"]
    pub icache_prelock_ctrl: crate::Reg<icache_prelock_ctrl::ICACHE_PRELOCK_CTRL_SPEC>,
    #[doc = "0x10 - EXTMEM_ICACHE_PRELOCK_SCT0_ADDR"]
    pub icache_prelock_sct0_addr:
        crate::Reg<icache_prelock_sct0_addr::ICACHE_PRELOCK_SCT0_ADDR_SPEC>,
    #[doc = "0x14 - EXTMEM_ICACHE_PRELOCK_SCT1_ADDR"]
    pub icache_prelock_sct1_addr:
        crate::Reg<icache_prelock_sct1_addr::ICACHE_PRELOCK_SCT1_ADDR_SPEC>,
    #[doc = "0x18 - EXTMEM_ICACHE_PRELOCK_SCT_SIZE"]
    pub icache_prelock_sct_size: crate::Reg<icache_prelock_sct_size::ICACHE_PRELOCK_SCT_SIZE_SPEC>,
    #[doc = "0x1c - EXTMEM_ICACHE_LOCK_CTRL"]
    pub icache_lock_ctrl: crate::Reg<icache_lock_ctrl::ICACHE_LOCK_CTRL_SPEC>,
    #[doc = "0x20 - EXTMEM_ICACHE_LOCK_ADDR"]
    pub icache_lock_addr: crate::Reg<icache_lock_addr::ICACHE_LOCK_ADDR_SPEC>,
    #[doc = "0x24 - EXTMEM_ICACHE_LOCK_SIZE"]
    pub icache_lock_size: crate::Reg<icache_lock_size::ICACHE_LOCK_SIZE_SPEC>,
    #[doc = "0x28 - EXTMEM_ICACHE_SYNC_CTRL"]
    pub icache_sync_ctrl: crate::Reg<icache_sync_ctrl::ICACHE_SYNC_CTRL_SPEC>,
    #[doc = "0x2c - EXTMEM_ICACHE_SYNC_ADDR"]
    pub icache_sync_addr: crate::Reg<icache_sync_addr::ICACHE_SYNC_ADDR_SPEC>,
    #[doc = "0x30 - EXTMEM_ICACHE_SYNC_SIZE"]
    pub icache_sync_size: crate::Reg<icache_sync_size::ICACHE_SYNC_SIZE_SPEC>,
    #[doc = "0x34 - EXTMEM_ICACHE_PRELOAD_CTRL"]
    pub icache_preload_ctrl: crate::Reg<icache_preload_ctrl::ICACHE_PRELOAD_CTRL_SPEC>,
    #[doc = "0x38 - EXTMEM_ICACHE_PRELOAD_ADDR"]
    pub icache_preload_addr: crate::Reg<icache_preload_addr::ICACHE_PRELOAD_ADDR_SPEC>,
    #[doc = "0x3c - EXTMEM_ICACHE_PRELOAD_SIZE"]
    pub icache_preload_size: crate::Reg<icache_preload_size::ICACHE_PRELOAD_SIZE_SPEC>,
    #[doc = "0x40 - EXTMEM_ICACHE_AUTOLOAD_CTRL"]
    pub icache_autoload_ctrl: crate::Reg<icache_autoload_ctrl::ICACHE_AUTOLOAD_CTRL_SPEC>,
    #[doc = "0x44 - EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR"]
    pub icache_autoload_sct0_addr:
        crate::Reg<icache_autoload_sct0_addr::ICACHE_AUTOLOAD_SCT0_ADDR_SPEC>,
    #[doc = "0x48 - EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE"]
    pub icache_autoload_sct0_size:
        crate::Reg<icache_autoload_sct0_size::ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>,
    #[doc = "0x4c - EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR"]
    pub icache_autoload_sct1_addr:
        crate::Reg<icache_autoload_sct1_addr::ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>,
    #[doc = "0x50 - EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE"]
    pub icache_autoload_sct1_size:
        crate::Reg<icache_autoload_sct1_size::ICACHE_AUTOLOAD_SCT1_SIZE_SPEC>,
    #[doc = "0x54 - EXTMEM_IBUS_TO_FLASH_START_VADDR"]
    pub ibus_to_flash_start_vaddr:
        crate::Reg<ibus_to_flash_start_vaddr::IBUS_TO_FLASH_START_VADDR_SPEC>,
    #[doc = "0x58 - EXTMEM_IBUS_TO_FLASH_END_VADDR"]
    pub ibus_to_flash_end_vaddr: crate::Reg<ibus_to_flash_end_vaddr::IBUS_TO_FLASH_END_VADDR_SPEC>,
    #[doc = "0x5c - EXTMEM_DBUS_TO_FLASH_START_VADDR"]
    pub dbus_to_flash_start_vaddr:
        crate::Reg<dbus_to_flash_start_vaddr::DBUS_TO_FLASH_START_VADDR_SPEC>,
    #[doc = "0x60 - EXTMEM_DBUS_TO_FLASH_END_VADDR"]
    pub dbus_to_flash_end_vaddr: crate::Reg<dbus_to_flash_end_vaddr::DBUS_TO_FLASH_END_VADDR_SPEC>,
    #[doc = "0x64 - EXTMEM_CACHE_ACS_CNT_CLR"]
    pub cache_acs_cnt_clr: crate::Reg<cache_acs_cnt_clr::CACHE_ACS_CNT_CLR_SPEC>,
    #[doc = "0x68 - EXTMEM_IBUS_ACS_MISS_CNT"]
    pub ibus_acs_miss_cnt: crate::Reg<ibus_acs_miss_cnt::IBUS_ACS_MISS_CNT_SPEC>,
    #[doc = "0x6c - EXTMEM_IBUS_ACS_CNT"]
    pub ibus_acs_cnt: crate::Reg<ibus_acs_cnt::IBUS_ACS_CNT_SPEC>,
    #[doc = "0x70 - EXTMEM_DBUS_ACS_FLASH_MISS_CNT"]
    pub dbus_acs_flash_miss_cnt: crate::Reg<dbus_acs_flash_miss_cnt::DBUS_ACS_FLASH_MISS_CNT_SPEC>,
    #[doc = "0x74 - EXTMEM_DBUS_ACS_CNT"]
    pub dbus_acs_cnt: crate::Reg<dbus_acs_cnt::DBUS_ACS_CNT_SPEC>,
    #[doc = "0x78 - EXTMEM_CACHE_ILG_INT_ENA"]
    pub cache_ilg_int_ena: crate::Reg<cache_ilg_int_ena::CACHE_ILG_INT_ENA_SPEC>,
    #[doc = "0x7c - EXTMEM_CACHE_ILG_INT_CLR"]
    pub cache_ilg_int_clr: crate::Reg<cache_ilg_int_clr::CACHE_ILG_INT_CLR_SPEC>,
    #[doc = "0x80 - EXTMEM_CACHE_ILG_INT_ST"]
    pub cache_ilg_int_st: crate::Reg<cache_ilg_int_st::CACHE_ILG_INT_ST_SPEC>,
    #[doc = "0x84 - EXTMEM_CORE0_ACS_CACHE_INT_ENA"]
    pub core0_acs_cache_int_ena: crate::Reg<core0_acs_cache_int_ena::CORE0_ACS_CACHE_INT_ENA_SPEC>,
    #[doc = "0x88 - EXTMEM_CORE0_ACS_CACHE_INT_CLR"]
    pub core0_acs_cache_int_clr: crate::Reg<core0_acs_cache_int_clr::CORE0_ACS_CACHE_INT_CLR_SPEC>,
    #[doc = "0x8c - EXTMEM_CORE0_ACS_CACHE_INT_ST"]
    pub core0_acs_cache_int_st: crate::Reg<core0_acs_cache_int_st::CORE0_ACS_CACHE_INT_ST_SPEC>,
    #[doc = "0x90 - EXTMEM_CORE0_DBUS_REJECT_ST"]
    pub core0_dbus_reject_st: crate::Reg<core0_dbus_reject_st::CORE0_DBUS_REJECT_ST_SPEC>,
    #[doc = "0x94 - EXTMEM_CORE0_DBUS_REJECT_VADDR"]
    pub core0_dbus_reject_vaddr: crate::Reg<core0_dbus_reject_vaddr::CORE0_DBUS_REJECT_VADDR_SPEC>,
    #[doc = "0x98 - EXTMEM_CORE0_IBUS_REJECT_ST"]
    pub core0_ibus_reject_st: crate::Reg<core0_ibus_reject_st::CORE0_IBUS_REJECT_ST_SPEC>,
    #[doc = "0x9c - EXTMEM_CORE0_IBUS_REJECT_VADDR"]
    pub core0_ibus_reject_vaddr: crate::Reg<core0_ibus_reject_vaddr::CORE0_IBUS_REJECT_VADDR_SPEC>,
    #[doc = "0xa0 - EXTMEM_CACHE_MMU_FAULT_CONTENT"]
    pub cache_mmu_fault_content: crate::Reg<cache_mmu_fault_content::CACHE_MMU_FAULT_CONTENT_SPEC>,
    #[doc = "0xa4 - EXTMEM_CACHE_MMU_FAULT_VADDR"]
    pub cache_mmu_fault_vaddr: crate::Reg<cache_mmu_fault_vaddr::CACHE_MMU_FAULT_VADDR_SPEC>,
    #[doc = "0xa8 - EXTMEM_CACHE_WRAP_AROUND_CTRL"]
    pub cache_wrap_around_ctrl: crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>,
    #[doc = "0xac - EXTMEM_CACHE_MMU_POWER_CTRL"]
    pub cache_mmu_power_ctrl: crate::Reg<cache_mmu_power_ctrl::CACHE_MMU_POWER_CTRL_SPEC>,
    #[doc = "0xb0 - EXTMEM_CACHE_STATE"]
    pub cache_state: crate::Reg<cache_state::CACHE_STATE_SPEC>,
    #[doc = "0xb4 - EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE"]
    pub cache_encrypt_decrypt_record_disable:
        crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>,
    #[doc = "0xb8 - EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON"]
    pub cache_encrypt_decrypt_clk_force_on:
        crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>,
    #[doc = "0xbc - EXTMEM_CACHE_PRELOAD_INT_CTRL"]
    pub cache_preload_int_ctrl: crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>,
    #[doc = "0xc0 - EXTMEM_CACHE_SYNC_INT_CTRL"]
    pub cache_sync_int_ctrl: crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>,
    #[doc = "0xc4 - EXTMEM_CACHE_MMU_OWNER"]
    pub cache_mmu_owner: crate::Reg<cache_mmu_owner::CACHE_MMU_OWNER_SPEC>,
    #[doc = "0xc8 - EXTMEM_CACHE_CONF_MISC"]
    pub cache_conf_misc: crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>,
    #[doc = "0xcc - EXTMEM_ICACHE_FREEZE"]
    pub icache_freeze: crate::Reg<icache_freeze::ICACHE_FREEZE_SPEC>,
    #[doc = "0xd0 - EXTMEM_ICACHE_ATOMIC_OPERATE_ENA"]
    pub icache_atomic_operate_ena:
        crate::Reg<icache_atomic_operate_ena::ICACHE_ATOMIC_OPERATE_ENA_SPEC>,
    #[doc = "0xd4 - EXTMEM_CACHE_REQUEST"]
    pub cache_request: crate::Reg<cache_request::CACHE_REQUEST_SPEC>,
    #[doc = "0xd8 - EXTMEM_IBUS_PMS_TBL_LOCK"]
    pub ibus_pms_tbl_lock: crate::Reg<ibus_pms_tbl_lock::IBUS_PMS_TBL_LOCK_SPEC>,
    #[doc = "0xdc - EXTMEM_IBUS_PMS_TBL_BOUNDARY0"]
    pub ibus_pms_tbl_boundary0: crate::Reg<ibus_pms_tbl_boundary0::IBUS_PMS_TBL_BOUNDARY0_SPEC>,
    #[doc = "0xe0 - EXTMEM_IBUS_PMS_TBL_BOUNDARY1"]
    pub ibus_pms_tbl_boundary1: crate::Reg<ibus_pms_tbl_boundary1::IBUS_PMS_TBL_BOUNDARY1_SPEC>,
    #[doc = "0xe4 - EXTMEM_IBUS_PMS_TBL_BOUNDARY2"]
    pub ibus_pms_tbl_boundary2: crate::Reg<ibus_pms_tbl_boundary2::IBUS_PMS_TBL_BOUNDARY2_SPEC>,
    #[doc = "0xe8 - EXTMEM_IBUS_PMS_TBL_ATTR"]
    pub ibus_pms_tbl_attr: crate::Reg<ibus_pms_tbl_attr::IBUS_PMS_TBL_ATTR_SPEC>,
    #[doc = "0xec - EXTMEM_DBUS_PMS_TBL_LOCK"]
    pub dbus_pms_tbl_lock: crate::Reg<dbus_pms_tbl_lock::DBUS_PMS_TBL_LOCK_SPEC>,
    #[doc = "0xf0 - EXTMEM_DBUS_PMS_TBL_BOUNDARY0"]
    pub dbus_pms_tbl_boundary0: crate::Reg<dbus_pms_tbl_boundary0::DBUS_PMS_TBL_BOUNDARY0_SPEC>,
    #[doc = "0xf4 - EXTMEM_DBUS_PMS_TBL_BOUNDARY1"]
    pub dbus_pms_tbl_boundary1: crate::Reg<dbus_pms_tbl_boundary1::DBUS_PMS_TBL_BOUNDARY1_SPEC>,
    #[doc = "0xf8 - EXTMEM_DBUS_PMS_TBL_BOUNDARY2"]
    pub dbus_pms_tbl_boundary2: crate::Reg<dbus_pms_tbl_boundary2::DBUS_PMS_TBL_BOUNDARY2_SPEC>,
    #[doc = "0xfc - EXTMEM_DBUS_PMS_TBL_ATTR"]
    pub dbus_pms_tbl_attr: crate::Reg<dbus_pms_tbl_attr::DBUS_PMS_TBL_ATTR_SPEC>,
    #[doc = "0x100 - EXTMEM_CLOCK_GATE"]
    pub clock_gate: crate::Reg<clock_gate::CLOCK_GATE_SPEC>,
    _reserved65: [u8; 0x02f8],
    #[doc = "0x3fc - EXTMEM_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "ICACHE_CTRL register accessor: an alias for `Reg<ICACHE_CTRL_SPEC>`"]
pub type ICACHE_CTRL = crate::Reg<icache_ctrl::ICACHE_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_CTRL"]
pub mod icache_ctrl;
#[doc = "ICACHE_CTRL1 register accessor: an alias for `Reg<ICACHE_CTRL1_SPEC>`"]
pub type ICACHE_CTRL1 = crate::Reg<icache_ctrl1::ICACHE_CTRL1_SPEC>;
#[doc = "EXTMEM_ICACHE_CTRL1"]
pub mod icache_ctrl1;
#[doc = "ICACHE_TAG_POWER_CTRL register accessor: an alias for `Reg<ICACHE_TAG_POWER_CTRL_SPEC>`"]
pub type ICACHE_TAG_POWER_CTRL = crate::Reg<icache_tag_power_ctrl::ICACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_TAG_POWER_CTRL"]
pub mod icache_tag_power_ctrl;
#[doc = "ICACHE_PRELOCK_CTRL register accessor: an alias for `Reg<ICACHE_PRELOCK_CTRL_SPEC>`"]
pub type ICACHE_PRELOCK_CTRL = crate::Reg<icache_prelock_ctrl::ICACHE_PRELOCK_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOCK_CTRL"]
pub mod icache_prelock_ctrl;
#[doc = "ICACHE_PRELOCK_SCT0_ADDR register accessor: an alias for `Reg<ICACHE_PRELOCK_SCT0_ADDR_SPEC>`"]
pub type ICACHE_PRELOCK_SCT0_ADDR =
    crate::Reg<icache_prelock_sct0_addr::ICACHE_PRELOCK_SCT0_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT0_ADDR"]
pub mod icache_prelock_sct0_addr;
#[doc = "ICACHE_PRELOCK_SCT1_ADDR register accessor: an alias for `Reg<ICACHE_PRELOCK_SCT1_ADDR_SPEC>`"]
pub type ICACHE_PRELOCK_SCT1_ADDR =
    crate::Reg<icache_prelock_sct1_addr::ICACHE_PRELOCK_SCT1_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT1_ADDR"]
pub mod icache_prelock_sct1_addr;
#[doc = "ICACHE_PRELOCK_SCT_SIZE register accessor: an alias for `Reg<ICACHE_PRELOCK_SCT_SIZE_SPEC>`"]
pub type ICACHE_PRELOCK_SCT_SIZE =
    crate::Reg<icache_prelock_sct_size::ICACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOCK_SCT_SIZE"]
pub mod icache_prelock_sct_size;
#[doc = "ICACHE_LOCK_CTRL register accessor: an alias for `Reg<ICACHE_LOCK_CTRL_SPEC>`"]
pub type ICACHE_LOCK_CTRL = crate::Reg<icache_lock_ctrl::ICACHE_LOCK_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_LOCK_CTRL"]
pub mod icache_lock_ctrl;
#[doc = "ICACHE_LOCK_ADDR register accessor: an alias for `Reg<ICACHE_LOCK_ADDR_SPEC>`"]
pub type ICACHE_LOCK_ADDR = crate::Reg<icache_lock_addr::ICACHE_LOCK_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_LOCK_ADDR"]
pub mod icache_lock_addr;
#[doc = "ICACHE_LOCK_SIZE register accessor: an alias for `Reg<ICACHE_LOCK_SIZE_SPEC>`"]
pub type ICACHE_LOCK_SIZE = crate::Reg<icache_lock_size::ICACHE_LOCK_SIZE_SPEC>;
#[doc = "EXTMEM_ICACHE_LOCK_SIZE"]
pub mod icache_lock_size;
#[doc = "ICACHE_SYNC_CTRL register accessor: an alias for `Reg<ICACHE_SYNC_CTRL_SPEC>`"]
pub type ICACHE_SYNC_CTRL = crate::Reg<icache_sync_ctrl::ICACHE_SYNC_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_SYNC_CTRL"]
pub mod icache_sync_ctrl;
#[doc = "ICACHE_SYNC_ADDR register accessor: an alias for `Reg<ICACHE_SYNC_ADDR_SPEC>`"]
pub type ICACHE_SYNC_ADDR = crate::Reg<icache_sync_addr::ICACHE_SYNC_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_SYNC_ADDR"]
pub mod icache_sync_addr;
#[doc = "ICACHE_SYNC_SIZE register accessor: an alias for `Reg<ICACHE_SYNC_SIZE_SPEC>`"]
pub type ICACHE_SYNC_SIZE = crate::Reg<icache_sync_size::ICACHE_SYNC_SIZE_SPEC>;
#[doc = "EXTMEM_ICACHE_SYNC_SIZE"]
pub mod icache_sync_size;
#[doc = "ICACHE_PRELOAD_CTRL register accessor: an alias for `Reg<ICACHE_PRELOAD_CTRL_SPEC>`"]
pub type ICACHE_PRELOAD_CTRL = crate::Reg<icache_preload_ctrl::ICACHE_PRELOAD_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOAD_CTRL"]
pub mod icache_preload_ctrl;
#[doc = "ICACHE_PRELOAD_ADDR register accessor: an alias for `Reg<ICACHE_PRELOAD_ADDR_SPEC>`"]
pub type ICACHE_PRELOAD_ADDR = crate::Reg<icache_preload_addr::ICACHE_PRELOAD_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOAD_ADDR"]
pub mod icache_preload_addr;
#[doc = "ICACHE_PRELOAD_SIZE register accessor: an alias for `Reg<ICACHE_PRELOAD_SIZE_SPEC>`"]
pub type ICACHE_PRELOAD_SIZE = crate::Reg<icache_preload_size::ICACHE_PRELOAD_SIZE_SPEC>;
#[doc = "EXTMEM_ICACHE_PRELOAD_SIZE"]
pub mod icache_preload_size;
#[doc = "ICACHE_AUTOLOAD_CTRL register accessor: an alias for `Reg<ICACHE_AUTOLOAD_CTRL_SPEC>`"]
pub type ICACHE_AUTOLOAD_CTRL = crate::Reg<icache_autoload_ctrl::ICACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_CTRL"]
pub mod icache_autoload_ctrl;
#[doc = "ICACHE_AUTOLOAD_SCT0_ADDR register accessor: an alias for `Reg<ICACHE_AUTOLOAD_SCT0_ADDR_SPEC>`"]
pub type ICACHE_AUTOLOAD_SCT0_ADDR =
    crate::Reg<icache_autoload_sct0_addr::ICACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT0_ADDR"]
pub mod icache_autoload_sct0_addr;
#[doc = "ICACHE_AUTOLOAD_SCT0_SIZE register accessor: an alias for `Reg<ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>`"]
pub type ICACHE_AUTOLOAD_SCT0_SIZE =
    crate::Reg<icache_autoload_sct0_size::ICACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT0_SIZE"]
pub mod icache_autoload_sct0_size;
#[doc = "ICACHE_AUTOLOAD_SCT1_ADDR register accessor: an alias for `Reg<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>`"]
pub type ICACHE_AUTOLOAD_SCT1_ADDR =
    crate::Reg<icache_autoload_sct1_addr::ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT1_ADDR"]
pub mod icache_autoload_sct1_addr;
#[doc = "ICACHE_AUTOLOAD_SCT1_SIZE register accessor: an alias for `Reg<ICACHE_AUTOLOAD_SCT1_SIZE_SPEC>`"]
pub type ICACHE_AUTOLOAD_SCT1_SIZE =
    crate::Reg<icache_autoload_sct1_size::ICACHE_AUTOLOAD_SCT1_SIZE_SPEC>;
#[doc = "EXTMEM_ICACHE_AUTOLOAD_SCT1_SIZE"]
pub mod icache_autoload_sct1_size;
#[doc = "IBUS_TO_FLASH_START_VADDR register accessor: an alias for `Reg<IBUS_TO_FLASH_START_VADDR_SPEC>`"]
pub type IBUS_TO_FLASH_START_VADDR =
    crate::Reg<ibus_to_flash_start_vaddr::IBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "EXTMEM_IBUS_TO_FLASH_START_VADDR"]
pub mod ibus_to_flash_start_vaddr;
#[doc = "IBUS_TO_FLASH_END_VADDR register accessor: an alias for `Reg<IBUS_TO_FLASH_END_VADDR_SPEC>`"]
pub type IBUS_TO_FLASH_END_VADDR =
    crate::Reg<ibus_to_flash_end_vaddr::IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "EXTMEM_IBUS_TO_FLASH_END_VADDR"]
pub mod ibus_to_flash_end_vaddr;
#[doc = "DBUS_TO_FLASH_START_VADDR register accessor: an alias for `Reg<DBUS_TO_FLASH_START_VADDR_SPEC>`"]
pub type DBUS_TO_FLASH_START_VADDR =
    crate::Reg<dbus_to_flash_start_vaddr::DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "EXTMEM_DBUS_TO_FLASH_START_VADDR"]
pub mod dbus_to_flash_start_vaddr;
#[doc = "DBUS_TO_FLASH_END_VADDR register accessor: an alias for `Reg<DBUS_TO_FLASH_END_VADDR_SPEC>`"]
pub type DBUS_TO_FLASH_END_VADDR =
    crate::Reg<dbus_to_flash_end_vaddr::DBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "EXTMEM_DBUS_TO_FLASH_END_VADDR"]
pub mod dbus_to_flash_end_vaddr;
#[doc = "CACHE_ACS_CNT_CLR register accessor: an alias for `Reg<CACHE_ACS_CNT_CLR_SPEC>`"]
pub type CACHE_ACS_CNT_CLR = crate::Reg<cache_acs_cnt_clr::CACHE_ACS_CNT_CLR_SPEC>;
#[doc = "EXTMEM_CACHE_ACS_CNT_CLR"]
pub mod cache_acs_cnt_clr;
#[doc = "IBUS_ACS_MISS_CNT register accessor: an alias for `Reg<IBUS_ACS_MISS_CNT_SPEC>`"]
pub type IBUS_ACS_MISS_CNT = crate::Reg<ibus_acs_miss_cnt::IBUS_ACS_MISS_CNT_SPEC>;
#[doc = "EXTMEM_IBUS_ACS_MISS_CNT"]
pub mod ibus_acs_miss_cnt;
#[doc = "IBUS_ACS_CNT register accessor: an alias for `Reg<IBUS_ACS_CNT_SPEC>`"]
pub type IBUS_ACS_CNT = crate::Reg<ibus_acs_cnt::IBUS_ACS_CNT_SPEC>;
#[doc = "EXTMEM_IBUS_ACS_CNT"]
pub mod ibus_acs_cnt;
#[doc = "DBUS_ACS_FLASH_MISS_CNT register accessor: an alias for `Reg<DBUS_ACS_FLASH_MISS_CNT_SPEC>`"]
pub type DBUS_ACS_FLASH_MISS_CNT =
    crate::Reg<dbus_acs_flash_miss_cnt::DBUS_ACS_FLASH_MISS_CNT_SPEC>;
#[doc = "EXTMEM_DBUS_ACS_FLASH_MISS_CNT"]
pub mod dbus_acs_flash_miss_cnt;
#[doc = "DBUS_ACS_CNT register accessor: an alias for `Reg<DBUS_ACS_CNT_SPEC>`"]
pub type DBUS_ACS_CNT = crate::Reg<dbus_acs_cnt::DBUS_ACS_CNT_SPEC>;
#[doc = "EXTMEM_DBUS_ACS_CNT"]
pub mod dbus_acs_cnt;
#[doc = "CACHE_ILG_INT_ENA register accessor: an alias for `Reg<CACHE_ILG_INT_ENA_SPEC>`"]
pub type CACHE_ILG_INT_ENA = crate::Reg<cache_ilg_int_ena::CACHE_ILG_INT_ENA_SPEC>;
#[doc = "EXTMEM_CACHE_ILG_INT_ENA"]
pub mod cache_ilg_int_ena;
#[doc = "CACHE_ILG_INT_CLR register accessor: an alias for `Reg<CACHE_ILG_INT_CLR_SPEC>`"]
pub type CACHE_ILG_INT_CLR = crate::Reg<cache_ilg_int_clr::CACHE_ILG_INT_CLR_SPEC>;
#[doc = "EXTMEM_CACHE_ILG_INT_CLR"]
pub mod cache_ilg_int_clr;
#[doc = "CACHE_ILG_INT_ST register accessor: an alias for `Reg<CACHE_ILG_INT_ST_SPEC>`"]
pub type CACHE_ILG_INT_ST = crate::Reg<cache_ilg_int_st::CACHE_ILG_INT_ST_SPEC>;
#[doc = "EXTMEM_CACHE_ILG_INT_ST"]
pub mod cache_ilg_int_st;
#[doc = "CORE0_ACS_CACHE_INT_ENA register accessor: an alias for `Reg<CORE0_ACS_CACHE_INT_ENA_SPEC>`"]
pub type CORE0_ACS_CACHE_INT_ENA =
    crate::Reg<core0_acs_cache_int_ena::CORE0_ACS_CACHE_INT_ENA_SPEC>;
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ENA"]
pub mod core0_acs_cache_int_ena;
#[doc = "CORE0_ACS_CACHE_INT_CLR register accessor: an alias for `Reg<CORE0_ACS_CACHE_INT_CLR_SPEC>`"]
pub type CORE0_ACS_CACHE_INT_CLR =
    crate::Reg<core0_acs_cache_int_clr::CORE0_ACS_CACHE_INT_CLR_SPEC>;
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_CLR"]
pub mod core0_acs_cache_int_clr;
#[doc = "CORE0_ACS_CACHE_INT_ST register accessor: an alias for `Reg<CORE0_ACS_CACHE_INT_ST_SPEC>`"]
pub type CORE0_ACS_CACHE_INT_ST = crate::Reg<core0_acs_cache_int_st::CORE0_ACS_CACHE_INT_ST_SPEC>;
#[doc = "EXTMEM_CORE0_ACS_CACHE_INT_ST"]
pub mod core0_acs_cache_int_st;
#[doc = "CORE0_DBUS_REJECT_ST register accessor: an alias for `Reg<CORE0_DBUS_REJECT_ST_SPEC>`"]
pub type CORE0_DBUS_REJECT_ST = crate::Reg<core0_dbus_reject_st::CORE0_DBUS_REJECT_ST_SPEC>;
#[doc = "EXTMEM_CORE0_DBUS_REJECT_ST"]
pub mod core0_dbus_reject_st;
#[doc = "CORE0_DBUS_REJECT_VADDR register accessor: an alias for `Reg<CORE0_DBUS_REJECT_VADDR_SPEC>`"]
pub type CORE0_DBUS_REJECT_VADDR =
    crate::Reg<core0_dbus_reject_vaddr::CORE0_DBUS_REJECT_VADDR_SPEC>;
#[doc = "EXTMEM_CORE0_DBUS_REJECT_VADDR"]
pub mod core0_dbus_reject_vaddr;
#[doc = "CORE0_IBUS_REJECT_ST register accessor: an alias for `Reg<CORE0_IBUS_REJECT_ST_SPEC>`"]
pub type CORE0_IBUS_REJECT_ST = crate::Reg<core0_ibus_reject_st::CORE0_IBUS_REJECT_ST_SPEC>;
#[doc = "EXTMEM_CORE0_IBUS_REJECT_ST"]
pub mod core0_ibus_reject_st;
#[doc = "CORE0_IBUS_REJECT_VADDR register accessor: an alias for `Reg<CORE0_IBUS_REJECT_VADDR_SPEC>`"]
pub type CORE0_IBUS_REJECT_VADDR =
    crate::Reg<core0_ibus_reject_vaddr::CORE0_IBUS_REJECT_VADDR_SPEC>;
#[doc = "EXTMEM_CORE0_IBUS_REJECT_VADDR"]
pub mod core0_ibus_reject_vaddr;
#[doc = "CACHE_MMU_FAULT_CONTENT register accessor: an alias for `Reg<CACHE_MMU_FAULT_CONTENT_SPEC>`"]
pub type CACHE_MMU_FAULT_CONTENT =
    crate::Reg<cache_mmu_fault_content::CACHE_MMU_FAULT_CONTENT_SPEC>;
#[doc = "EXTMEM_CACHE_MMU_FAULT_CONTENT"]
pub mod cache_mmu_fault_content;
#[doc = "CACHE_MMU_FAULT_VADDR register accessor: an alias for `Reg<CACHE_MMU_FAULT_VADDR_SPEC>`"]
pub type CACHE_MMU_FAULT_VADDR = crate::Reg<cache_mmu_fault_vaddr::CACHE_MMU_FAULT_VADDR_SPEC>;
#[doc = "EXTMEM_CACHE_MMU_FAULT_VADDR"]
pub mod cache_mmu_fault_vaddr;
#[doc = "CACHE_WRAP_AROUND_CTRL register accessor: an alias for `Reg<CACHE_WRAP_AROUND_CTRL_SPEC>`"]
pub type CACHE_WRAP_AROUND_CTRL = crate::Reg<cache_wrap_around_ctrl::CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "EXTMEM_CACHE_WRAP_AROUND_CTRL"]
pub mod cache_wrap_around_ctrl;
#[doc = "CACHE_MMU_POWER_CTRL register accessor: an alias for `Reg<CACHE_MMU_POWER_CTRL_SPEC>`"]
pub type CACHE_MMU_POWER_CTRL = crate::Reg<cache_mmu_power_ctrl::CACHE_MMU_POWER_CTRL_SPEC>;
#[doc = "EXTMEM_CACHE_MMU_POWER_CTRL"]
pub mod cache_mmu_power_ctrl;
#[doc = "CACHE_STATE register accessor: an alias for `Reg<CACHE_STATE_SPEC>`"]
pub type CACHE_STATE = crate::Reg<cache_state::CACHE_STATE_SPEC>;
#[doc = "EXTMEM_CACHE_STATE"]
pub mod cache_state;
#[doc = "CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE register accessor: an alias for `Reg<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>`"]
pub type CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE =
    crate::Reg<cache_encrypt_decrypt_record_disable::CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
#[doc = "EXTMEM_CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE"]
pub mod cache_encrypt_decrypt_record_disable;
#[doc = "CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON register accessor: an alias for `Reg<CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>`"]
pub type CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON =
    crate::Reg<cache_encrypt_decrypt_clk_force_on::CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON_SPEC>;
#[doc = "EXTMEM_CACHE_ENCRYPT_DECRYPT_CLK_FORCE_ON"]
pub mod cache_encrypt_decrypt_clk_force_on;
#[doc = "CACHE_PRELOAD_INT_CTRL register accessor: an alias for `Reg<CACHE_PRELOAD_INT_CTRL_SPEC>`"]
pub type CACHE_PRELOAD_INT_CTRL = crate::Reg<cache_preload_int_ctrl::CACHE_PRELOAD_INT_CTRL_SPEC>;
#[doc = "EXTMEM_CACHE_PRELOAD_INT_CTRL"]
pub mod cache_preload_int_ctrl;
#[doc = "CACHE_SYNC_INT_CTRL register accessor: an alias for `Reg<CACHE_SYNC_INT_CTRL_SPEC>`"]
pub type CACHE_SYNC_INT_CTRL = crate::Reg<cache_sync_int_ctrl::CACHE_SYNC_INT_CTRL_SPEC>;
#[doc = "EXTMEM_CACHE_SYNC_INT_CTRL"]
pub mod cache_sync_int_ctrl;
#[doc = "CACHE_MMU_OWNER register accessor: an alias for `Reg<CACHE_MMU_OWNER_SPEC>`"]
pub type CACHE_MMU_OWNER = crate::Reg<cache_mmu_owner::CACHE_MMU_OWNER_SPEC>;
#[doc = "EXTMEM_CACHE_MMU_OWNER"]
pub mod cache_mmu_owner;
#[doc = "CACHE_CONF_MISC register accessor: an alias for `Reg<CACHE_CONF_MISC_SPEC>`"]
pub type CACHE_CONF_MISC = crate::Reg<cache_conf_misc::CACHE_CONF_MISC_SPEC>;
#[doc = "EXTMEM_CACHE_CONF_MISC"]
pub mod cache_conf_misc;
#[doc = "ICACHE_FREEZE register accessor: an alias for `Reg<ICACHE_FREEZE_SPEC>`"]
pub type ICACHE_FREEZE = crate::Reg<icache_freeze::ICACHE_FREEZE_SPEC>;
#[doc = "EXTMEM_ICACHE_FREEZE"]
pub mod icache_freeze;
#[doc = "ICACHE_ATOMIC_OPERATE_ENA register accessor: an alias for `Reg<ICACHE_ATOMIC_OPERATE_ENA_SPEC>`"]
pub type ICACHE_ATOMIC_OPERATE_ENA =
    crate::Reg<icache_atomic_operate_ena::ICACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "EXTMEM_ICACHE_ATOMIC_OPERATE_ENA"]
pub mod icache_atomic_operate_ena;
#[doc = "CACHE_REQUEST register accessor: an alias for `Reg<CACHE_REQUEST_SPEC>`"]
pub type CACHE_REQUEST = crate::Reg<cache_request::CACHE_REQUEST_SPEC>;
#[doc = "EXTMEM_CACHE_REQUEST"]
pub mod cache_request;
#[doc = "IBUS_PMS_TBL_LOCK register accessor: an alias for `Reg<IBUS_PMS_TBL_LOCK_SPEC>`"]
pub type IBUS_PMS_TBL_LOCK = crate::Reg<ibus_pms_tbl_lock::IBUS_PMS_TBL_LOCK_SPEC>;
#[doc = "EXTMEM_IBUS_PMS_TBL_LOCK"]
pub mod ibus_pms_tbl_lock;
#[doc = "IBUS_PMS_TBL_BOUNDARY0 register accessor: an alias for `Reg<IBUS_PMS_TBL_BOUNDARY0_SPEC>`"]
pub type IBUS_PMS_TBL_BOUNDARY0 = crate::Reg<ibus_pms_tbl_boundary0::IBUS_PMS_TBL_BOUNDARY0_SPEC>;
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY0"]
pub mod ibus_pms_tbl_boundary0;
#[doc = "IBUS_PMS_TBL_BOUNDARY1 register accessor: an alias for `Reg<IBUS_PMS_TBL_BOUNDARY1_SPEC>`"]
pub type IBUS_PMS_TBL_BOUNDARY1 = crate::Reg<ibus_pms_tbl_boundary1::IBUS_PMS_TBL_BOUNDARY1_SPEC>;
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY1"]
pub mod ibus_pms_tbl_boundary1;
#[doc = "IBUS_PMS_TBL_BOUNDARY2 register accessor: an alias for `Reg<IBUS_PMS_TBL_BOUNDARY2_SPEC>`"]
pub type IBUS_PMS_TBL_BOUNDARY2 = crate::Reg<ibus_pms_tbl_boundary2::IBUS_PMS_TBL_BOUNDARY2_SPEC>;
#[doc = "EXTMEM_IBUS_PMS_TBL_BOUNDARY2"]
pub mod ibus_pms_tbl_boundary2;
#[doc = "IBUS_PMS_TBL_ATTR register accessor: an alias for `Reg<IBUS_PMS_TBL_ATTR_SPEC>`"]
pub type IBUS_PMS_TBL_ATTR = crate::Reg<ibus_pms_tbl_attr::IBUS_PMS_TBL_ATTR_SPEC>;
#[doc = "EXTMEM_IBUS_PMS_TBL_ATTR"]
pub mod ibus_pms_tbl_attr;
#[doc = "DBUS_PMS_TBL_LOCK register accessor: an alias for `Reg<DBUS_PMS_TBL_LOCK_SPEC>`"]
pub type DBUS_PMS_TBL_LOCK = crate::Reg<dbus_pms_tbl_lock::DBUS_PMS_TBL_LOCK_SPEC>;
#[doc = "EXTMEM_DBUS_PMS_TBL_LOCK"]
pub mod dbus_pms_tbl_lock;
#[doc = "DBUS_PMS_TBL_BOUNDARY0 register accessor: an alias for `Reg<DBUS_PMS_TBL_BOUNDARY0_SPEC>`"]
pub type DBUS_PMS_TBL_BOUNDARY0 = crate::Reg<dbus_pms_tbl_boundary0::DBUS_PMS_TBL_BOUNDARY0_SPEC>;
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY0"]
pub mod dbus_pms_tbl_boundary0;
#[doc = "DBUS_PMS_TBL_BOUNDARY1 register accessor: an alias for `Reg<DBUS_PMS_TBL_BOUNDARY1_SPEC>`"]
pub type DBUS_PMS_TBL_BOUNDARY1 = crate::Reg<dbus_pms_tbl_boundary1::DBUS_PMS_TBL_BOUNDARY1_SPEC>;
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY1"]
pub mod dbus_pms_tbl_boundary1;
#[doc = "DBUS_PMS_TBL_BOUNDARY2 register accessor: an alias for `Reg<DBUS_PMS_TBL_BOUNDARY2_SPEC>`"]
pub type DBUS_PMS_TBL_BOUNDARY2 = crate::Reg<dbus_pms_tbl_boundary2::DBUS_PMS_TBL_BOUNDARY2_SPEC>;
#[doc = "EXTMEM_DBUS_PMS_TBL_BOUNDARY2"]
pub mod dbus_pms_tbl_boundary2;
#[doc = "DBUS_PMS_TBL_ATTR register accessor: an alias for `Reg<DBUS_PMS_TBL_ATTR_SPEC>`"]
pub type DBUS_PMS_TBL_ATTR = crate::Reg<dbus_pms_tbl_attr::DBUS_PMS_TBL_ATTR_SPEC>;
#[doc = "EXTMEM_DBUS_PMS_TBL_ATTR"]
pub mod dbus_pms_tbl_attr;
#[doc = "CLOCK_GATE register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "EXTMEM_CLOCK_GATE"]
pub mod clock_gate;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "EXTMEM_DATE"]
pub mod date;
