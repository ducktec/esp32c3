#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSCON_SYSCLK_CONF"]
    pub sysclk_conf: crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>,
    #[doc = "0x04 - SYSCON_TICK_CONF"]
    pub tick_conf: crate::Reg<tick_conf::TICK_CONF_SPEC>,
    #[doc = "0x08 - SYSCON_CLK_OUT_EN"]
    pub clk_out_en: crate::Reg<clk_out_en::CLK_OUT_EN_SPEC>,
    #[doc = "0x0c - SYSCON_WIFI_BB_CFG"]
    pub wifi_bb_cfg: crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>,
    #[doc = "0x10 - SYSCON_WIFI_BB_CFG_2"]
    pub wifi_bb_cfg_2: crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>,
    #[doc = "0x14 - SYSCON_WIFI_CLK_EN"]
    pub wifi_clk_en: crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>,
    #[doc = "0x18 - SYSCON_WIFI_RST_EN"]
    pub wifi_rst_en: crate::Reg<wifi_rst_en::WIFI_RST_EN_SPEC>,
    #[doc = "0x1c - SYSCON_HOST_INF_SEL"]
    pub host_inf_sel: crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>,
    #[doc = "0x20 - SYSCON_EXT_MEM_PMS_LOCK"]
    pub ext_mem_pms_lock: crate::Reg<ext_mem_pms_lock::EXT_MEM_PMS_LOCK_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x28 - SYSCON_FLASH_ACE0_ATTR"]
    pub flash_ace0_attr: crate::Reg<flash_ace0_attr::FLASH_ACE0_ATTR_SPEC>,
    #[doc = "0x2c - SYSCON_FLASH_ACE1_ATTR"]
    pub flash_ace1_attr: crate::Reg<flash_ace1_attr::FLASH_ACE1_ATTR_SPEC>,
    #[doc = "0x30 - SYSCON_FLASH_ACE2_ATTR"]
    pub flash_ace2_attr: crate::Reg<flash_ace2_attr::FLASH_ACE2_ATTR_SPEC>,
    #[doc = "0x34 - SYSCON_FLASH_ACE3_ATTR"]
    pub flash_ace3_attr: crate::Reg<flash_ace3_attr::FLASH_ACE3_ATTR_SPEC>,
    #[doc = "0x38 - SYSCON_FLASH_ACE0_ADDR"]
    pub flash_ace0_addr: crate::Reg<flash_ace0_addr::FLASH_ACE0_ADDR_SPEC>,
    #[doc = "0x3c - SYSCON_FLASH_ACE1_ADDR"]
    pub flash_ace1_addr: crate::Reg<flash_ace1_addr::FLASH_ACE1_ADDR_SPEC>,
    #[doc = "0x40 - SYSCON_FLASH_ACE2_ADDR"]
    pub flash_ace2_addr: crate::Reg<flash_ace2_addr::FLASH_ACE2_ADDR_SPEC>,
    #[doc = "0x44 - SYSCON_FLASH_ACE3_ADDR"]
    pub flash_ace3_addr: crate::Reg<flash_ace3_addr::FLASH_ACE3_ADDR_SPEC>,
    #[doc = "0x48 - SYSCON_FLASH_ACE0_SIZE"]
    pub flash_ace0_size: crate::Reg<flash_ace0_size::FLASH_ACE0_SIZE_SPEC>,
    #[doc = "0x4c - SYSCON_FLASH_ACE1_SIZE"]
    pub flash_ace1_size: crate::Reg<flash_ace1_size::FLASH_ACE1_SIZE_SPEC>,
    #[doc = "0x50 - SYSCON_FLASH_ACE2_SIZE"]
    pub flash_ace2_size: crate::Reg<flash_ace2_size::FLASH_ACE2_SIZE_SPEC>,
    #[doc = "0x54 - SYSCON_FLASH_ACE3_SIZE"]
    pub flash_ace3_size: crate::Reg<flash_ace3_size::FLASH_ACE3_SIZE_SPEC>,
    _reserved21: [u8; 0x30],
    #[doc = "0x88 - SYSCON_SPI_MEM_PMS_CTRL"]
    pub spi_mem_pms_ctrl: crate::Reg<spi_mem_pms_ctrl::SPI_MEM_PMS_CTRL_SPEC>,
    #[doc = "0x8c - SYSCON_SPI_MEM_REJECT_ADDR"]
    pub spi_mem_reject_addr: crate::Reg<spi_mem_reject_addr::SPI_MEM_REJECT_ADDR_SPEC>,
    #[doc = "0x90 - SYSCON_SDIO_CTRL"]
    pub sdio_ctrl: crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>,
    #[doc = "0x94 - SYSCON_REDCY_SIG0"]
    pub redcy_sig0: crate::Reg<redcy_sig0::REDCY_SIG0_SPEC>,
    #[doc = "0x98 - SYSCON_REDCY_SIG1"]
    pub redcy_sig1: crate::Reg<redcy_sig1::REDCY_SIG1_SPEC>,
    #[doc = "0x9c - SYSCON_FRONT_END_MEM_PD"]
    pub front_end_mem_pd: crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>,
    #[doc = "0xa0 - SYSCON_RETENTION_CTRL"]
    pub retention_ctrl: crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>,
    #[doc = "0xa4 - SYSCON_CLKGATE_FORCE_ON"]
    pub clkgate_force_on: crate::Reg<clkgate_force_on::CLKGATE_FORCE_ON_SPEC>,
    #[doc = "0xa8 - SYSCON_MEM_POWER_DOWN"]
    pub mem_power_down: crate::Reg<mem_power_down::MEM_POWER_DOWN_SPEC>,
    #[doc = "0xac - SYSCON_MEM_POWER_UP"]
    pub mem_power_up: crate::Reg<mem_power_up::MEM_POWER_UP_SPEC>,
    #[doc = "0xb0 - SYSCON_RND_DATA"]
    pub rnd_data: crate::Reg<rnd_data::RND_DATA_SPEC>,
    #[doc = "0xb4 - SYSCON_PERI_BACKUP_CONFIG"]
    pub peri_backup_config: crate::Reg<peri_backup_config::PERI_BACKUP_CONFIG_SPEC>,
    #[doc = "0xb8 - SYSCON_PERI_BACKUP_APB_ADDR"]
    pub peri_backup_apb_addr: crate::Reg<peri_backup_apb_addr::PERI_BACKUP_APB_ADDR_SPEC>,
    #[doc = "0xbc - SYSCON_PERI_BACKUP_MEM_ADDR"]
    pub peri_backup_mem_addr: crate::Reg<peri_backup_mem_addr::PERI_BACKUP_MEM_ADDR_SPEC>,
    #[doc = "0xc0 - SYSCON_PERI_BACKUP_INT_RAW"]
    pub peri_backup_int_raw: crate::Reg<peri_backup_int_raw::PERI_BACKUP_INT_RAW_SPEC>,
    #[doc = "0xc4 - SYSCON_PERI_BACKUP_INT_ST"]
    pub peri_backup_int_st: crate::Reg<peri_backup_int_st::PERI_BACKUP_INT_ST_SPEC>,
    #[doc = "0xc8 - SYSCON_PERI_BACKUP_INT_ENA"]
    pub peri_backup_int_ena: crate::Reg<peri_backup_int_ena::PERI_BACKUP_INT_ENA_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0xd0 - SYSCON_PERI_BACKUP_INT_CLR"]
    pub peri_backup_int_clr: crate::Reg<peri_backup_int_clr::PERI_BACKUP_INT_CLR_SPEC>,
    _reserved39: [u8; 0x0328],
    #[doc = "0x3fc - SYSCON_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "SYSCLK_CONF register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "SYSCON_SYSCLK_CONF"]
pub mod sysclk_conf;
#[doc = "TICK_CONF register accessor: an alias for `Reg<TICK_CONF_SPEC>`"]
pub type TICK_CONF = crate::Reg<tick_conf::TICK_CONF_SPEC>;
#[doc = "SYSCON_TICK_CONF"]
pub mod tick_conf;
#[doc = "CLK_OUT_EN register accessor: an alias for `Reg<CLK_OUT_EN_SPEC>`"]
pub type CLK_OUT_EN = crate::Reg<clk_out_en::CLK_OUT_EN_SPEC>;
#[doc = "SYSCON_CLK_OUT_EN"]
pub mod clk_out_en;
#[doc = "WIFI_BB_CFG register accessor: an alias for `Reg<WIFI_BB_CFG_SPEC>`"]
pub type WIFI_BB_CFG = crate::Reg<wifi_bb_cfg::WIFI_BB_CFG_SPEC>;
#[doc = "SYSCON_WIFI_BB_CFG"]
pub mod wifi_bb_cfg;
#[doc = "WIFI_BB_CFG_2 register accessor: an alias for `Reg<WIFI_BB_CFG_2_SPEC>`"]
pub type WIFI_BB_CFG_2 = crate::Reg<wifi_bb_cfg_2::WIFI_BB_CFG_2_SPEC>;
#[doc = "SYSCON_WIFI_BB_CFG_2"]
pub mod wifi_bb_cfg_2;
#[doc = "WIFI_CLK_EN register accessor: an alias for `Reg<WIFI_CLK_EN_SPEC>`"]
pub type WIFI_CLK_EN = crate::Reg<wifi_clk_en::WIFI_CLK_EN_SPEC>;
#[doc = "SYSCON_WIFI_CLK_EN"]
pub mod wifi_clk_en;
#[doc = "WIFI_RST_EN register accessor: an alias for `Reg<WIFI_RST_EN_SPEC>`"]
pub type WIFI_RST_EN = crate::Reg<wifi_rst_en::WIFI_RST_EN_SPEC>;
#[doc = "SYSCON_WIFI_RST_EN"]
pub mod wifi_rst_en;
#[doc = "HOST_INF_SEL register accessor: an alias for `Reg<HOST_INF_SEL_SPEC>`"]
pub type HOST_INF_SEL = crate::Reg<host_inf_sel::HOST_INF_SEL_SPEC>;
#[doc = "SYSCON_HOST_INF_SEL"]
pub mod host_inf_sel;
#[doc = "EXT_MEM_PMS_LOCK register accessor: an alias for `Reg<EXT_MEM_PMS_LOCK_SPEC>`"]
pub type EXT_MEM_PMS_LOCK = crate::Reg<ext_mem_pms_lock::EXT_MEM_PMS_LOCK_SPEC>;
#[doc = "SYSCON_EXT_MEM_PMS_LOCK"]
pub mod ext_mem_pms_lock;
#[doc = "FLASH_ACE0_ATTR register accessor: an alias for `Reg<FLASH_ACE0_ATTR_SPEC>`"]
pub type FLASH_ACE0_ATTR = crate::Reg<flash_ace0_attr::FLASH_ACE0_ATTR_SPEC>;
#[doc = "SYSCON_FLASH_ACE0_ATTR"]
pub mod flash_ace0_attr;
#[doc = "FLASH_ACE1_ATTR register accessor: an alias for `Reg<FLASH_ACE1_ATTR_SPEC>`"]
pub type FLASH_ACE1_ATTR = crate::Reg<flash_ace1_attr::FLASH_ACE1_ATTR_SPEC>;
#[doc = "SYSCON_FLASH_ACE1_ATTR"]
pub mod flash_ace1_attr;
#[doc = "FLASH_ACE2_ATTR register accessor: an alias for `Reg<FLASH_ACE2_ATTR_SPEC>`"]
pub type FLASH_ACE2_ATTR = crate::Reg<flash_ace2_attr::FLASH_ACE2_ATTR_SPEC>;
#[doc = "SYSCON_FLASH_ACE2_ATTR"]
pub mod flash_ace2_attr;
#[doc = "FLASH_ACE3_ATTR register accessor: an alias for `Reg<FLASH_ACE3_ATTR_SPEC>`"]
pub type FLASH_ACE3_ATTR = crate::Reg<flash_ace3_attr::FLASH_ACE3_ATTR_SPEC>;
#[doc = "SYSCON_FLASH_ACE3_ATTR"]
pub mod flash_ace3_attr;
#[doc = "FLASH_ACE0_ADDR register accessor: an alias for `Reg<FLASH_ACE0_ADDR_SPEC>`"]
pub type FLASH_ACE0_ADDR = crate::Reg<flash_ace0_addr::FLASH_ACE0_ADDR_SPEC>;
#[doc = "SYSCON_FLASH_ACE0_ADDR"]
pub mod flash_ace0_addr;
#[doc = "FLASH_ACE1_ADDR register accessor: an alias for `Reg<FLASH_ACE1_ADDR_SPEC>`"]
pub type FLASH_ACE1_ADDR = crate::Reg<flash_ace1_addr::FLASH_ACE1_ADDR_SPEC>;
#[doc = "SYSCON_FLASH_ACE1_ADDR"]
pub mod flash_ace1_addr;
#[doc = "FLASH_ACE2_ADDR register accessor: an alias for `Reg<FLASH_ACE2_ADDR_SPEC>`"]
pub type FLASH_ACE2_ADDR = crate::Reg<flash_ace2_addr::FLASH_ACE2_ADDR_SPEC>;
#[doc = "SYSCON_FLASH_ACE2_ADDR"]
pub mod flash_ace2_addr;
#[doc = "FLASH_ACE3_ADDR register accessor: an alias for `Reg<FLASH_ACE3_ADDR_SPEC>`"]
pub type FLASH_ACE3_ADDR = crate::Reg<flash_ace3_addr::FLASH_ACE3_ADDR_SPEC>;
#[doc = "SYSCON_FLASH_ACE3_ADDR"]
pub mod flash_ace3_addr;
#[doc = "FLASH_ACE0_SIZE register accessor: an alias for `Reg<FLASH_ACE0_SIZE_SPEC>`"]
pub type FLASH_ACE0_SIZE = crate::Reg<flash_ace0_size::FLASH_ACE0_SIZE_SPEC>;
#[doc = "SYSCON_FLASH_ACE0_SIZE"]
pub mod flash_ace0_size;
#[doc = "FLASH_ACE1_SIZE register accessor: an alias for `Reg<FLASH_ACE1_SIZE_SPEC>`"]
pub type FLASH_ACE1_SIZE = crate::Reg<flash_ace1_size::FLASH_ACE1_SIZE_SPEC>;
#[doc = "SYSCON_FLASH_ACE1_SIZE"]
pub mod flash_ace1_size;
#[doc = "FLASH_ACE2_SIZE register accessor: an alias for `Reg<FLASH_ACE2_SIZE_SPEC>`"]
pub type FLASH_ACE2_SIZE = crate::Reg<flash_ace2_size::FLASH_ACE2_SIZE_SPEC>;
#[doc = "SYSCON_FLASH_ACE2_SIZE"]
pub mod flash_ace2_size;
#[doc = "FLASH_ACE3_SIZE register accessor: an alias for `Reg<FLASH_ACE3_SIZE_SPEC>`"]
pub type FLASH_ACE3_SIZE = crate::Reg<flash_ace3_size::FLASH_ACE3_SIZE_SPEC>;
#[doc = "SYSCON_FLASH_ACE3_SIZE"]
pub mod flash_ace3_size;
#[doc = "SPI_MEM_PMS_CTRL register accessor: an alias for `Reg<SPI_MEM_PMS_CTRL_SPEC>`"]
pub type SPI_MEM_PMS_CTRL = crate::Reg<spi_mem_pms_ctrl::SPI_MEM_PMS_CTRL_SPEC>;
#[doc = "SYSCON_SPI_MEM_PMS_CTRL"]
pub mod spi_mem_pms_ctrl;
#[doc = "SPI_MEM_REJECT_ADDR register accessor: an alias for `Reg<SPI_MEM_REJECT_ADDR_SPEC>`"]
pub type SPI_MEM_REJECT_ADDR = crate::Reg<spi_mem_reject_addr::SPI_MEM_REJECT_ADDR_SPEC>;
#[doc = "SYSCON_SPI_MEM_REJECT_ADDR"]
pub mod spi_mem_reject_addr;
#[doc = "SDIO_CTRL register accessor: an alias for `Reg<SDIO_CTRL_SPEC>`"]
pub type SDIO_CTRL = crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>;
#[doc = "SYSCON_SDIO_CTRL"]
pub mod sdio_ctrl;
#[doc = "REDCY_SIG0 register accessor: an alias for `Reg<REDCY_SIG0_SPEC>`"]
pub type REDCY_SIG0 = crate::Reg<redcy_sig0::REDCY_SIG0_SPEC>;
#[doc = "SYSCON_REDCY_SIG0"]
pub mod redcy_sig0;
#[doc = "REDCY_SIG1 register accessor: an alias for `Reg<REDCY_SIG1_SPEC>`"]
pub type REDCY_SIG1 = crate::Reg<redcy_sig1::REDCY_SIG1_SPEC>;
#[doc = "SYSCON_REDCY_SIG1"]
pub mod redcy_sig1;
#[doc = "FRONT_END_MEM_PD register accessor: an alias for `Reg<FRONT_END_MEM_PD_SPEC>`"]
pub type FRONT_END_MEM_PD = crate::Reg<front_end_mem_pd::FRONT_END_MEM_PD_SPEC>;
#[doc = "SYSCON_FRONT_END_MEM_PD"]
pub mod front_end_mem_pd;
#[doc = "RETENTION_CTRL register accessor: an alias for `Reg<RETENTION_CTRL_SPEC>`"]
pub type RETENTION_CTRL = crate::Reg<retention_ctrl::RETENTION_CTRL_SPEC>;
#[doc = "SYSCON_RETENTION_CTRL"]
pub mod retention_ctrl;
#[doc = "CLKGATE_FORCE_ON register accessor: an alias for `Reg<CLKGATE_FORCE_ON_SPEC>`"]
pub type CLKGATE_FORCE_ON = crate::Reg<clkgate_force_on::CLKGATE_FORCE_ON_SPEC>;
#[doc = "SYSCON_CLKGATE_FORCE_ON"]
pub mod clkgate_force_on;
#[doc = "MEM_POWER_DOWN register accessor: an alias for `Reg<MEM_POWER_DOWN_SPEC>`"]
pub type MEM_POWER_DOWN = crate::Reg<mem_power_down::MEM_POWER_DOWN_SPEC>;
#[doc = "SYSCON_MEM_POWER_DOWN"]
pub mod mem_power_down;
#[doc = "MEM_POWER_UP register accessor: an alias for `Reg<MEM_POWER_UP_SPEC>`"]
pub type MEM_POWER_UP = crate::Reg<mem_power_up::MEM_POWER_UP_SPEC>;
#[doc = "SYSCON_MEM_POWER_UP"]
pub mod mem_power_up;
#[doc = "RND_DATA register accessor: an alias for `Reg<RND_DATA_SPEC>`"]
pub type RND_DATA = crate::Reg<rnd_data::RND_DATA_SPEC>;
#[doc = "SYSCON_RND_DATA"]
pub mod rnd_data;
#[doc = "PERI_BACKUP_CONFIG register accessor: an alias for `Reg<PERI_BACKUP_CONFIG_SPEC>`"]
pub type PERI_BACKUP_CONFIG = crate::Reg<peri_backup_config::PERI_BACKUP_CONFIG_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_CONFIG"]
pub mod peri_backup_config;
#[doc = "PERI_BACKUP_APB_ADDR register accessor: an alias for `Reg<PERI_BACKUP_APB_ADDR_SPEC>`"]
pub type PERI_BACKUP_APB_ADDR = crate::Reg<peri_backup_apb_addr::PERI_BACKUP_APB_ADDR_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_APB_ADDR"]
pub mod peri_backup_apb_addr;
#[doc = "PERI_BACKUP_MEM_ADDR register accessor: an alias for `Reg<PERI_BACKUP_MEM_ADDR_SPEC>`"]
pub type PERI_BACKUP_MEM_ADDR = crate::Reg<peri_backup_mem_addr::PERI_BACKUP_MEM_ADDR_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_MEM_ADDR"]
pub mod peri_backup_mem_addr;
#[doc = "PERI_BACKUP_INT_RAW register accessor: an alias for `Reg<PERI_BACKUP_INT_RAW_SPEC>`"]
pub type PERI_BACKUP_INT_RAW = crate::Reg<peri_backup_int_raw::PERI_BACKUP_INT_RAW_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_INT_RAW"]
pub mod peri_backup_int_raw;
#[doc = "PERI_BACKUP_INT_ST register accessor: an alias for `Reg<PERI_BACKUP_INT_ST_SPEC>`"]
pub type PERI_BACKUP_INT_ST = crate::Reg<peri_backup_int_st::PERI_BACKUP_INT_ST_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_INT_ST"]
pub mod peri_backup_int_st;
#[doc = "PERI_BACKUP_INT_ENA register accessor: an alias for `Reg<PERI_BACKUP_INT_ENA_SPEC>`"]
pub type PERI_BACKUP_INT_ENA = crate::Reg<peri_backup_int_ena::PERI_BACKUP_INT_ENA_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_INT_ENA"]
pub mod peri_backup_int_ena;
#[doc = "PERI_BACKUP_INT_CLR register accessor: an alias for `Reg<PERI_BACKUP_INT_CLR_SPEC>`"]
pub type PERI_BACKUP_INT_CLR = crate::Reg<peri_backup_int_clr::PERI_BACKUP_INT_CLR_SPEC>;
#[doc = "SYSCON_PERI_BACKUP_INT_CLR"]
pub mod peri_backup_int_clr;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SYSCON_DATE"]
pub mod date;
