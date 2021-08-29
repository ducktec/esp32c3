#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SYSTEM_CPU_PERI_CLK_EN"]
    pub cpu_peri_clk_en: crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>,
    #[doc = "0x04 - SYSTEM_CPU_PERI_RST_EN"]
    pub cpu_peri_rst_en: crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>,
    #[doc = "0x08 - SYSTEM_CPU_PER_CONF"]
    pub cpu_per_conf: crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>,
    #[doc = "0x0c - SYSTEM_MEM_PD_MASK"]
    pub mem_pd_mask: crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>,
    #[doc = "0x10 - SYSTEM_PERIP_CLK_EN0"]
    pub perip_clk_en0: crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>,
    #[doc = "0x14 - SYSTEM_PERIP_CLK_EN1"]
    pub perip_clk_en1: crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>,
    #[doc = "0x18 - SYSTEM_PERIP_RST_EN0"]
    pub perip_rst_en0: crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>,
    #[doc = "0x1c - SYSTEM_PERIP_RST_EN1"]
    pub perip_rst_en1: crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>,
    #[doc = "0x20 - SYSTEM_BT_LPCK_DIV_INT"]
    pub bt_lpck_div_int: crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>,
    #[doc = "0x24 - SYSTEM_BT_LPCK_DIV_FRAC"]
    pub bt_lpck_div_frac: crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>,
    #[doc = "0x28 - SYSTEM_CPU_INTR_FROM_CPU_0"]
    pub cpu_intr_from_cpu_0: crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>,
    #[doc = "0x2c - SYSTEM_CPU_INTR_FROM_CPU_1"]
    pub cpu_intr_from_cpu_1: crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>,
    #[doc = "0x30 - SYSTEM_CPU_INTR_FROM_CPU_2"]
    pub cpu_intr_from_cpu_2: crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>,
    #[doc = "0x34 - SYSTEM_CPU_INTR_FROM_CPU_3"]
    pub cpu_intr_from_cpu_3: crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>,
    #[doc = "0x38 - SYSTEM_RSA_PD_CTRL"]
    pub rsa_pd_ctrl: crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>,
    #[doc = "0x3c - SYSTEM_EDMA_CTRL"]
    pub edma_ctrl: crate::Reg<edma_ctrl::EDMA_CTRL_SPEC>,
    #[doc = "0x40 - SYSTEM_CACHE_CONTROL"]
    pub cache_control: crate::Reg<cache_control::CACHE_CONTROL_SPEC>,
    #[doc = "0x44 - SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL"]
    pub external_device_encrypt_decrypt_control: crate::Reg<
        external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
    >,
    #[doc = "0x48 - SYSTEM_RTC_FASTMEM_CONFIG"]
    pub rtc_fastmem_config: crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>,
    #[doc = "0x4c - SYSTEM_RTC_FASTMEM_CRC"]
    pub rtc_fastmem_crc: crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>,
    #[doc = "0x50 - SYSTEM_REDUNDANT_ECO_CTRL"]
    pub redundant_eco_ctrl: crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>,
    #[doc = "0x54 - SYSTEM_CLOCK_GATE"]
    pub clock_gate: crate::Reg<clock_gate::CLOCK_GATE_SPEC>,
    #[doc = "0x58 - SYSTEM_SYSCLK_CONF"]
    pub sysclk_conf: crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>,
    #[doc = "0x5c - SYSTEM_MEM_PVT"]
    pub mem_pvt: crate::Reg<mem_pvt::MEM_PVT_SPEC>,
    #[doc = "0x60 - SYSTEM_COMB_PVT_LVT_CONF"]
    pub comb_pvt_lvt_conf: crate::Reg<comb_pvt_lvt_conf::COMB_PVT_LVT_CONF_SPEC>,
    #[doc = "0x64 - SYSTEM_COMB_PVT_NVT_CONF"]
    pub comb_pvt_nvt_conf: crate::Reg<comb_pvt_nvt_conf::COMB_PVT_NVT_CONF_SPEC>,
    #[doc = "0x68 - SYSTEM_COMB_PVT_HVT_CONF"]
    pub comb_pvt_hvt_conf: crate::Reg<comb_pvt_hvt_conf::COMB_PVT_HVT_CONF_SPEC>,
    #[doc = "0x6c - SYSTEM_COMB_PVT_ERR_LVT_SITE0"]
    pub comb_pvt_err_lvt_site0: crate::Reg<comb_pvt_err_lvt_site0::COMB_PVT_ERR_LVT_SITE0_SPEC>,
    #[doc = "0x70 - SYSTEM_COMB_PVT_ERR_NVT_SITE0"]
    pub comb_pvt_err_nvt_site0: crate::Reg<comb_pvt_err_nvt_site0::COMB_PVT_ERR_NVT_SITE0_SPEC>,
    #[doc = "0x74 - SYSTEM_COMB_PVT_ERR_HVT_SITE0"]
    pub comb_pvt_err_hvt_site0: crate::Reg<comb_pvt_err_hvt_site0::COMB_PVT_ERR_HVT_SITE0_SPEC>,
    #[doc = "0x78 - SYSTEM_COMB_PVT_ERR_LVT_SITE1"]
    pub comb_pvt_err_lvt_site1: crate::Reg<comb_pvt_err_lvt_site1::COMB_PVT_ERR_LVT_SITE1_SPEC>,
    #[doc = "0x7c - SYSTEM_COMB_PVT_ERR_NVT_SITE1"]
    pub comb_pvt_err_nvt_site1: crate::Reg<comb_pvt_err_nvt_site1::COMB_PVT_ERR_NVT_SITE1_SPEC>,
    #[doc = "0x80 - SYSTEM_COMB_PVT_ERR_HVT_SITE1"]
    pub comb_pvt_err_hvt_site1: crate::Reg<comb_pvt_err_hvt_site1::COMB_PVT_ERR_HVT_SITE1_SPEC>,
    #[doc = "0x84 - SYSTEM_COMB_PVT_ERR_LVT_SITE2"]
    pub comb_pvt_err_lvt_site2: crate::Reg<comb_pvt_err_lvt_site2::COMB_PVT_ERR_LVT_SITE2_SPEC>,
    #[doc = "0x88 - SYSTEM_COMB_PVT_ERR_NVT_SITE2"]
    pub comb_pvt_err_nvt_site2: crate::Reg<comb_pvt_err_nvt_site2::COMB_PVT_ERR_NVT_SITE2_SPEC>,
    #[doc = "0x8c - SYSTEM_COMB_PVT_ERR_HVT_SITE2"]
    pub comb_pvt_err_hvt_site2: crate::Reg<comb_pvt_err_hvt_site2::COMB_PVT_ERR_HVT_SITE2_SPEC>,
    #[doc = "0x90 - SYSTEM_COMB_PVT_ERR_LVT_SITE3"]
    pub comb_pvt_err_lvt_site3: crate::Reg<comb_pvt_err_lvt_site3::COMB_PVT_ERR_LVT_SITE3_SPEC>,
    #[doc = "0x94 - SYSTEM_COMB_PVT_ERR_NVT_SITE3"]
    pub comb_pvt_err_nvt_site3: crate::Reg<comb_pvt_err_nvt_site3::COMB_PVT_ERR_NVT_SITE3_SPEC>,
    #[doc = "0x98 - SYSTEM_COMB_PVT_ERR_HVT_SITE3"]
    pub comb_pvt_err_hvt_site3: crate::Reg<comb_pvt_err_hvt_site3::COMB_PVT_ERR_HVT_SITE3_SPEC>,
    _reserved39: [u8; 0x0f60],
    #[doc = "0xffc - SYSTEM_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "CPU_PERI_CLK_EN register accessor: an alias for `Reg<CPU_PERI_CLK_EN_SPEC>`"]
pub type CPU_PERI_CLK_EN = crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>;
#[doc = "SYSTEM_CPU_PERI_CLK_EN"]
pub mod cpu_peri_clk_en;
#[doc = "CPU_PERI_RST_EN register accessor: an alias for `Reg<CPU_PERI_RST_EN_SPEC>`"]
pub type CPU_PERI_RST_EN = crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>;
#[doc = "SYSTEM_CPU_PERI_RST_EN"]
pub mod cpu_peri_rst_en;
#[doc = "CPU_PER_CONF register accessor: an alias for `Reg<CPU_PER_CONF_SPEC>`"]
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
#[doc = "SYSTEM_CPU_PER_CONF"]
pub mod cpu_per_conf;
#[doc = "MEM_PD_MASK register accessor: an alias for `Reg<MEM_PD_MASK_SPEC>`"]
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
#[doc = "SYSTEM_MEM_PD_MASK"]
pub mod mem_pd_mask;
#[doc = "PERIP_CLK_EN0 register accessor: an alias for `Reg<PERIP_CLK_EN0_SPEC>`"]
pub type PERIP_CLK_EN0 = crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>;
#[doc = "SYSTEM_PERIP_CLK_EN0"]
pub mod perip_clk_en0;
#[doc = "PERIP_CLK_EN1 register accessor: an alias for `Reg<PERIP_CLK_EN1_SPEC>`"]
pub type PERIP_CLK_EN1 = crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>;
#[doc = "SYSTEM_PERIP_CLK_EN1"]
pub mod perip_clk_en1;
#[doc = "PERIP_RST_EN0 register accessor: an alias for `Reg<PERIP_RST_EN0_SPEC>`"]
pub type PERIP_RST_EN0 = crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>;
#[doc = "SYSTEM_PERIP_RST_EN0"]
pub mod perip_rst_en0;
#[doc = "PERIP_RST_EN1 register accessor: an alias for `Reg<PERIP_RST_EN1_SPEC>`"]
pub type PERIP_RST_EN1 = crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>;
#[doc = "SYSTEM_PERIP_RST_EN1"]
pub mod perip_rst_en1;
#[doc = "BT_LPCK_DIV_INT register accessor: an alias for `Reg<BT_LPCK_DIV_INT_SPEC>`"]
pub type BT_LPCK_DIV_INT = crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>;
#[doc = "SYSTEM_BT_LPCK_DIV_INT"]
pub mod bt_lpck_div_int;
#[doc = "BT_LPCK_DIV_FRAC register accessor: an alias for `Reg<BT_LPCK_DIV_FRAC_SPEC>`"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "SYSTEM_BT_LPCK_DIV_FRAC"]
pub mod bt_lpck_div_frac;
#[doc = "CPU_INTR_FROM_CPU_0 register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_0"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_1"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_2"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "SYSTEM_CPU_INTR_FROM_CPU_3"]
pub mod cpu_intr_from_cpu_3;
#[doc = "RSA_PD_CTRL register accessor: an alias for `Reg<RSA_PD_CTRL_SPEC>`"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "SYSTEM_RSA_PD_CTRL"]
pub mod rsa_pd_ctrl;
#[doc = "EDMA_CTRL register accessor: an alias for `Reg<EDMA_CTRL_SPEC>`"]
pub type EDMA_CTRL = crate::Reg<edma_ctrl::EDMA_CTRL_SPEC>;
#[doc = "SYSTEM_EDMA_CTRL"]
pub mod edma_ctrl;
#[doc = "CACHE_CONTROL register accessor: an alias for `Reg<CACHE_CONTROL_SPEC>`"]
pub type CACHE_CONTROL = crate::Reg<cache_control::CACHE_CONTROL_SPEC>;
#[doc = "SYSTEM_CACHE_CONTROL"]
pub mod cache_control;
#[doc = "EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL register accessor: an alias for `Reg<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>`"]
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
#[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL"]
pub mod external_device_encrypt_decrypt_control;
#[doc = "RTC_FASTMEM_CONFIG register accessor: an alias for `Reg<RTC_FASTMEM_CONFIG_SPEC>`"]
pub type RTC_FASTMEM_CONFIG = crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>;
#[doc = "SYSTEM_RTC_FASTMEM_CONFIG"]
pub mod rtc_fastmem_config;
#[doc = "RTC_FASTMEM_CRC register accessor: an alias for `Reg<RTC_FASTMEM_CRC_SPEC>`"]
pub type RTC_FASTMEM_CRC = crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>;
#[doc = "SYSTEM_RTC_FASTMEM_CRC"]
pub mod rtc_fastmem_crc;
#[doc = "REDUNDANT_ECO_CTRL register accessor: an alias for `Reg<REDUNDANT_ECO_CTRL_SPEC>`"]
pub type REDUNDANT_ECO_CTRL = crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>;
#[doc = "SYSTEM_REDUNDANT_ECO_CTRL"]
pub mod redundant_eco_ctrl;
#[doc = "CLOCK_GATE register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "SYSTEM_CLOCK_GATE"]
pub mod clock_gate;
#[doc = "SYSCLK_CONF register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "SYSTEM_SYSCLK_CONF"]
pub mod sysclk_conf;
#[doc = "MEM_PVT register accessor: an alias for `Reg<MEM_PVT_SPEC>`"]
pub type MEM_PVT = crate::Reg<mem_pvt::MEM_PVT_SPEC>;
#[doc = "SYSTEM_MEM_PVT"]
pub mod mem_pvt;
#[doc = "COMB_PVT_LVT_CONF register accessor: an alias for `Reg<COMB_PVT_LVT_CONF_SPEC>`"]
pub type COMB_PVT_LVT_CONF = crate::Reg<comb_pvt_lvt_conf::COMB_PVT_LVT_CONF_SPEC>;
#[doc = "SYSTEM_COMB_PVT_LVT_CONF"]
pub mod comb_pvt_lvt_conf;
#[doc = "COMB_PVT_NVT_CONF register accessor: an alias for `Reg<COMB_PVT_NVT_CONF_SPEC>`"]
pub type COMB_PVT_NVT_CONF = crate::Reg<comb_pvt_nvt_conf::COMB_PVT_NVT_CONF_SPEC>;
#[doc = "SYSTEM_COMB_PVT_NVT_CONF"]
pub mod comb_pvt_nvt_conf;
#[doc = "COMB_PVT_HVT_CONF register accessor: an alias for `Reg<COMB_PVT_HVT_CONF_SPEC>`"]
pub type COMB_PVT_HVT_CONF = crate::Reg<comb_pvt_hvt_conf::COMB_PVT_HVT_CONF_SPEC>;
#[doc = "SYSTEM_COMB_PVT_HVT_CONF"]
pub mod comb_pvt_hvt_conf;
#[doc = "COMB_PVT_ERR_LVT_SITE0 register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE0 = crate::Reg<comb_pvt_err_lvt_site0::COMB_PVT_ERR_LVT_SITE0_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE0"]
pub mod comb_pvt_err_lvt_site0;
#[doc = "COMB_PVT_ERR_NVT_SITE0 register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE0 = crate::Reg<comb_pvt_err_nvt_site0::COMB_PVT_ERR_NVT_SITE0_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE0"]
pub mod comb_pvt_err_nvt_site0;
#[doc = "COMB_PVT_ERR_HVT_SITE0 register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE0 = crate::Reg<comb_pvt_err_hvt_site0::COMB_PVT_ERR_HVT_SITE0_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE0"]
pub mod comb_pvt_err_hvt_site0;
#[doc = "COMB_PVT_ERR_LVT_SITE1 register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE1 = crate::Reg<comb_pvt_err_lvt_site1::COMB_PVT_ERR_LVT_SITE1_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE1"]
pub mod comb_pvt_err_lvt_site1;
#[doc = "COMB_PVT_ERR_NVT_SITE1 register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE1 = crate::Reg<comb_pvt_err_nvt_site1::COMB_PVT_ERR_NVT_SITE1_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE1"]
pub mod comb_pvt_err_nvt_site1;
#[doc = "COMB_PVT_ERR_HVT_SITE1 register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE1 = crate::Reg<comb_pvt_err_hvt_site1::COMB_PVT_ERR_HVT_SITE1_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE1"]
pub mod comb_pvt_err_hvt_site1;
#[doc = "COMB_PVT_ERR_LVT_SITE2 register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE2 = crate::Reg<comb_pvt_err_lvt_site2::COMB_PVT_ERR_LVT_SITE2_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE2"]
pub mod comb_pvt_err_lvt_site2;
#[doc = "COMB_PVT_ERR_NVT_SITE2 register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE2 = crate::Reg<comb_pvt_err_nvt_site2::COMB_PVT_ERR_NVT_SITE2_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE2"]
pub mod comb_pvt_err_nvt_site2;
#[doc = "COMB_PVT_ERR_HVT_SITE2 register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE2 = crate::Reg<comb_pvt_err_hvt_site2::COMB_PVT_ERR_HVT_SITE2_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE2"]
pub mod comb_pvt_err_hvt_site2;
#[doc = "COMB_PVT_ERR_LVT_SITE3 register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE3 = crate::Reg<comb_pvt_err_lvt_site3::COMB_PVT_ERR_LVT_SITE3_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_LVT_SITE3"]
pub mod comb_pvt_err_lvt_site3;
#[doc = "COMB_PVT_ERR_NVT_SITE3 register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE3 = crate::Reg<comb_pvt_err_nvt_site3::COMB_PVT_ERR_NVT_SITE3_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_NVT_SITE3"]
pub mod comb_pvt_err_nvt_site3;
#[doc = "COMB_PVT_ERR_HVT_SITE3 register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE3 = crate::Reg<comb_pvt_err_hvt_site3::COMB_PVT_ERR_HVT_SITE3_SPEC>;
#[doc = "SYSTEM_COMB_PVT_ERR_HVT_SITE3"]
pub mod comb_pvt_err_hvt_site3;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "SYSTEM_DATE"]
pub mod date;
