#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock { # [ doc = "0x00 - INTERRUPT_CORE0_MAC_INTR_MAP" ] pub mac_intr_map : crate :: Reg < mac_intr_map :: MAC_INTR_MAP_SPEC > , # [ doc = "0x04 - INTERRUPT_CORE0_MAC_NMI_MAP" ] pub mac_nmi_map : crate :: Reg < mac_nmi_map :: MAC_NMI_MAP_SPEC > , # [ doc = "0x08 - INTERRUPT_CORE0_PWR_INTR_MAP" ] pub pwr_intr_map : crate :: Reg < pwr_intr_map :: PWR_INTR_MAP_SPEC > , # [ doc = "0x0c - INTERRUPT_CORE0_BB_INT_MAP" ] pub bb_int_map : crate :: Reg < bb_int_map :: BB_INT_MAP_SPEC > , # [ doc = "0x10 - INTERRUPT_CORE0_BT_MAC_INT_MAP" ] pub bt_mac_int_map : crate :: Reg < bt_mac_int_map :: BT_MAC_INT_MAP_SPEC > , # [ doc = "0x14 - INTERRUPT_CORE0_BT_BB_INT_MAP" ] pub bt_bb_int_map : crate :: Reg < bt_bb_int_map :: BT_BB_INT_MAP_SPEC > , # [ doc = "0x18 - INTERRUPT_CORE0_BT_BB_NMI_MAP" ] pub bt_bb_nmi_map : crate :: Reg < bt_bb_nmi_map :: BT_BB_NMI_MAP_SPEC > , # [ doc = "0x1c - INTERRUPT_CORE0_RWBT_IRQ_MAP" ] pub rwbt_irq_map : crate :: Reg < rwbt_irq_map :: RWBT_IRQ_MAP_SPEC > , # [ doc = "0x20 - INTERRUPT_CORE0_RWBLE_IRQ_MAP" ] pub rwble_irq_map : crate :: Reg < rwble_irq_map :: RWBLE_IRQ_MAP_SPEC > , # [ doc = "0x24 - INTERRUPT_CORE0_RWBT_NMI_MAP" ] pub rwbt_nmi_map : crate :: Reg < rwbt_nmi_map :: RWBT_NMI_MAP_SPEC > , # [ doc = "0x28 - INTERRUPT_CORE0_RWBLE_NMI_MAP" ] pub rwble_nmi_map : crate :: Reg < rwble_nmi_map :: RWBLE_NMI_MAP_SPEC > , # [ doc = "0x2c - INTERRUPT_CORE0_I2C_MST_INT_MAP" ] pub i2c_mst_int_map : crate :: Reg < i2c_mst_int_map :: I2C_MST_INT_MAP_SPEC > , # [ doc = "0x30 - INTERRUPT_CORE0_SLC0_INTR_MAP" ] pub slc0_intr_map : crate :: Reg < slc0_intr_map :: SLC0_INTR_MAP_SPEC > , # [ doc = "0x34 - INTERRUPT_CORE0_SLC1_INTR_MAP" ] pub slc1_intr_map : crate :: Reg < slc1_intr_map :: SLC1_INTR_MAP_SPEC > , # [ doc = "0x38 - INTERRUPT_CORE0_APB_CTRL_INTR_MAP" ] pub apb_ctrl_intr_map : crate :: Reg < apb_ctrl_intr_map :: APB_CTRL_INTR_MAP_SPEC > , # [ doc = "0x3c - INTERRUPT_CORE0_UHCI0_INTR_MAP" ] pub uhci0_intr_map : crate :: Reg < uhci0_intr_map :: UHCI0_INTR_MAP_SPEC > , # [ doc = "0x40 - INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP" ] pub gpio_interrupt_pro_map : crate :: Reg < gpio_interrupt_pro_map :: GPIO_INTERRUPT_PRO_MAP_SPEC > , # [ doc = "0x44 - INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP" ] pub gpio_interrupt_pro_nmi_map : crate :: Reg < gpio_interrupt_pro_nmi_map :: GPIO_INTERRUPT_PRO_NMI_MAP_SPEC > , # [ doc = "0x48 - INTERRUPT_CORE0_SPI_INTR_1_MAP" ] pub spi_intr_1_map : crate :: Reg < spi_intr_1_map :: SPI_INTR_1_MAP_SPEC > , # [ doc = "0x4c - INTERRUPT_CORE0_SPI_INTR_2_MAP" ] pub spi_intr_2_map : crate :: Reg < spi_intr_2_map :: SPI_INTR_2_MAP_SPEC > , # [ doc = "0x50 - INTERRUPT_CORE0_I2S1_INT_MAP" ] pub i2s1_int_map : crate :: Reg < i2s1_int_map :: I2S1_INT_MAP_SPEC > , # [ doc = "0x54 - INTERRUPT_CORE0_UART_INTR_MAP" ] pub uart_intr_map : crate :: Reg < uart_intr_map :: UART_INTR_MAP_SPEC > , # [ doc = "0x58 - INTERRUPT_CORE0_UART1_INTR_MAP" ] pub uart1_intr_map : crate :: Reg < uart1_intr_map :: UART1_INTR_MAP_SPEC > , # [ doc = "0x5c - INTERRUPT_CORE0_LEDC_INT_MAP" ] pub ledc_int_map : crate :: Reg < ledc_int_map :: LEDC_INT_MAP_SPEC > , # [ doc = "0x60 - INTERRUPT_CORE0_EFUSE_INT_MAP" ] pub efuse_int_map : crate :: Reg < efuse_int_map :: EFUSE_INT_MAP_SPEC > , # [ doc = "0x64 - INTERRUPT_CORE0_CAN_INT_MAP" ] pub can_int_map : crate :: Reg < can_int_map :: CAN_INT_MAP_SPEC > , # [ doc = "0x68 - INTERRUPT_CORE0_USB_INTR_MAP" ] pub usb_intr_map : crate :: Reg < usb_intr_map :: USB_INTR_MAP_SPEC > , # [ doc = "0x6c - INTERRUPT_CORE0_RTC_CORE_INTR_MAP" ] pub rtc_core_intr_map : crate :: Reg < rtc_core_intr_map :: RTC_CORE_INTR_MAP_SPEC > , # [ doc = "0x70 - INTERRUPT_CORE0_RMT_INTR_MAP" ] pub rmt_intr_map : crate :: Reg < rmt_intr_map :: RMT_INTR_MAP_SPEC > , # [ doc = "0x74 - INTERRUPT_CORE0_I2C_EXT0_INTR_MAP" ] pub i2c_ext0_intr_map : crate :: Reg < i2c_ext0_intr_map :: I2C_EXT0_INTR_MAP_SPEC > , # [ doc = "0x78 - INTERRUPT_CORE0_TIMER_INT1_MAP" ] pub timer_int1_map : crate :: Reg < timer_int1_map :: TIMER_INT1_MAP_SPEC > , # [ doc = "0x7c - INTERRUPT_CORE0_TIMER_INT2_MAP" ] pub timer_int2_map : crate :: Reg < timer_int2_map :: TIMER_INT2_MAP_SPEC > , # [ doc = "0x80 - INTERRUPT_CORE0_TG_T0_INT_MAP" ] pub tg_t0_int_map : crate :: Reg < tg_t0_int_map :: TG_T0_INT_MAP_SPEC > , # [ doc = "0x84 - INTERRUPT_CORE0_TG_WDT_INT_MAP" ] pub tg_wdt_int_map : crate :: Reg < tg_wdt_int_map :: TG_WDT_INT_MAP_SPEC > , # [ doc = "0x88 - INTERRUPT_CORE0_TG1_T0_INT_MAP" ] pub tg1_t0_int_map : crate :: Reg < tg1_t0_int_map :: TG1_T0_INT_MAP_SPEC > , # [ doc = "0x8c - INTERRUPT_CORE0_TG1_WDT_INT_MAP" ] pub tg1_wdt_int_map : crate :: Reg < tg1_wdt_int_map :: TG1_WDT_INT_MAP_SPEC > , # [ doc = "0x90 - INTERRUPT_CORE0_CACHE_IA_INT_MAP" ] pub cache_ia_int_map : crate :: Reg < cache_ia_int_map :: CACHE_IA_INT_MAP_SPEC > , # [ doc = "0x94 - INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP" ] pub systimer_target0_int_map : crate :: Reg < systimer_target0_int_map :: SYSTIMER_TARGET0_INT_MAP_SPEC > , # [ doc = "0x98 - INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP" ] pub systimer_target1_int_map : crate :: Reg < systimer_target1_int_map :: SYSTIMER_TARGET1_INT_MAP_SPEC > , # [ doc = "0x9c - INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP" ] pub systimer_target2_int_map : crate :: Reg < systimer_target2_int_map :: SYSTIMER_TARGET2_INT_MAP_SPEC > , # [ doc = "0xa0 - INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP" ] pub spi_mem_reject_intr_map : crate :: Reg < spi_mem_reject_intr_map :: SPI_MEM_REJECT_INTR_MAP_SPEC > , # [ doc = "0xa4 - INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP" ] pub icache_preload_int_map : crate :: Reg < icache_preload_int_map :: ICACHE_PRELOAD_INT_MAP_SPEC > , # [ doc = "0xa8 - INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP" ] pub icache_sync_int_map : crate :: Reg < icache_sync_int_map :: ICACHE_SYNC_INT_MAP_SPEC > , # [ doc = "0xac - INTERRUPT_CORE0_APB_ADC_INT_MAP" ] pub apb_adc_int_map : crate :: Reg < apb_adc_int_map :: APB_ADC_INT_MAP_SPEC > , # [ doc = "0xb0 - INTERRUPT_CORE0_DMA_CH0_INT_MAP" ] pub dma_ch0_int_map : crate :: Reg < dma_ch0_int_map :: DMA_CH0_INT_MAP_SPEC > , # [ doc = "0xb4 - INTERRUPT_CORE0_DMA_CH1_INT_MAP" ] pub dma_ch1_int_map : crate :: Reg < dma_ch1_int_map :: DMA_CH1_INT_MAP_SPEC > , # [ doc = "0xb8 - INTERRUPT_CORE0_DMA_CH2_INT_MAP" ] pub dma_ch2_int_map : crate :: Reg < dma_ch2_int_map :: DMA_CH2_INT_MAP_SPEC > , # [ doc = "0xbc - INTERRUPT_CORE0_RSA_INT_MAP" ] pub rsa_int_map : crate :: Reg < rsa_int_map :: RSA_INT_MAP_SPEC > , # [ doc = "0xc0 - INTERRUPT_CORE0_AES_INT_MAP" ] pub aes_int_map : crate :: Reg < aes_int_map :: AES_INT_MAP_SPEC > , # [ doc = "0xc4 - INTERRUPT_CORE0_SHA_INT_MAP" ] pub sha_int_map : crate :: Reg < sha_int_map :: SHA_INT_MAP_SPEC > , # [ doc = "0xc8 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP" ] pub cpu_intr_from_cpu_0_map : crate :: Reg < cpu_intr_from_cpu_0_map :: CPU_INTR_FROM_CPU_0_MAP_SPEC > , # [ doc = "0xcc - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP" ] pub cpu_intr_from_cpu_1_map : crate :: Reg < cpu_intr_from_cpu_1_map :: CPU_INTR_FROM_CPU_1_MAP_SPEC > , # [ doc = "0xd0 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP" ] pub cpu_intr_from_cpu_2_map : crate :: Reg < cpu_intr_from_cpu_2_map :: CPU_INTR_FROM_CPU_2_MAP_SPEC > , # [ doc = "0xd4 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP" ] pub cpu_intr_from_cpu_3_map : crate :: Reg < cpu_intr_from_cpu_3_map :: CPU_INTR_FROM_CPU_3_MAP_SPEC > , # [ doc = "0xd8 - INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP" ] pub assist_debug_intr_map : crate :: Reg < assist_debug_intr_map :: ASSIST_DEBUG_INTR_MAP_SPEC > , # [ doc = "0xdc - INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub dma_apbperi_pms_monitor_violate_intr_map : crate :: Reg < dma_apbperi_pms_monitor_violate_intr_map :: DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xe0 - INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub core_0_iram0_pms_monitor_violate_intr_map : crate :: Reg < core_0_iram0_pms_monitor_violate_intr_map :: CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xe4 - INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub core_0_dram0_pms_monitor_violate_intr_map : crate :: Reg < core_0_dram0_pms_monitor_violate_intr_map :: CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xe8 - INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub core_0_pif_pms_monitor_violate_intr_map : crate :: Reg < core_0_pif_pms_monitor_violate_intr_map :: CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xec - INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP" ] pub core_0_pif_pms_monitor_violate_size_intr_map : crate :: Reg < core_0_pif_pms_monitor_violate_size_intr_map :: CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC > , # [ doc = "0xf0 - INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP" ] pub backup_pms_violate_intr_map : crate :: Reg < backup_pms_violate_intr_map :: BACKUP_PMS_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xf4 - INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP" ] pub cache_core0_acs_int_map : crate :: Reg < cache_core0_acs_int_map :: CACHE_CORE0_ACS_INT_MAP_SPEC > , # [ doc = "0xf8 - INTERRUPT_CORE0_INTR_STATUS_0" ] pub intr_status_0 : crate :: Reg < intr_status_0 :: INTR_STATUS_0_SPEC > , # [ doc = "0xfc - INTERRUPT_CORE0_INTR_STATUS_1" ] pub intr_status_1 : crate :: Reg < intr_status_1 :: INTR_STATUS_1_SPEC > , # [ doc = "0x100 - INTERRUPT_CORE0_CLOCK_GATE" ] pub clock_gate : crate :: Reg < clock_gate :: CLOCK_GATE_SPEC > , # [ doc = "0x104 - INTERRUPT_CORE0_CPU_INT_ENABLE" ] pub cpu_int_enable : crate :: Reg < cpu_int_enable :: CPU_INT_ENABLE_SPEC > , # [ doc = "0x108 - INTERRUPT_CORE0_CPU_INT_TYPE" ] pub cpu_int_type : crate :: Reg < cpu_int_type :: CPU_INT_TYPE_SPEC > , # [ doc = "0x10c - INTERRUPT_CORE0_CPU_INT_CLEAR" ] pub cpu_int_clear : crate :: Reg < cpu_int_clear :: CPU_INT_CLEAR_SPEC > , # [ doc = "0x110 - INTERRUPT_CORE0_CPU_INT_EIP_STATUS" ] pub cpu_int_eip_status : crate :: Reg < cpu_int_eip_status :: CPU_INT_EIP_STATUS_SPEC > , # [ doc = "0x114 - INTERRUPT_CORE0_CPU_INT_PRI_0" ] pub cpu_int_pri_0 : crate :: Reg < cpu_int_pri_0 :: CPU_INT_PRI_0_SPEC > , # [ doc = "0x118 - INTERRUPT_CORE0_CPU_INT_PRI_1" ] pub cpu_int_pri_1 : crate :: Reg < cpu_int_pri_1 :: CPU_INT_PRI_1_SPEC > , # [ doc = "0x11c - INTERRUPT_CORE0_CPU_INT_PRI_2" ] pub cpu_int_pri_2 : crate :: Reg < cpu_int_pri_2 :: CPU_INT_PRI_2_SPEC > , # [ doc = "0x120 - INTERRUPT_CORE0_CPU_INT_PRI_3" ] pub cpu_int_pri_3 : crate :: Reg < cpu_int_pri_3 :: CPU_INT_PRI_3_SPEC > , # [ doc = "0x124 - INTERRUPT_CORE0_CPU_INT_PRI_4" ] pub cpu_int_pri_4 : crate :: Reg < cpu_int_pri_4 :: CPU_INT_PRI_4_SPEC > , # [ doc = "0x128 - INTERRUPT_CORE0_CPU_INT_PRI_5" ] pub cpu_int_pri_5 : crate :: Reg < cpu_int_pri_5 :: CPU_INT_PRI_5_SPEC > , # [ doc = "0x12c - INTERRUPT_CORE0_CPU_INT_PRI_6" ] pub cpu_int_pri_6 : crate :: Reg < cpu_int_pri_6 :: CPU_INT_PRI_6_SPEC > , # [ doc = "0x130 - INTERRUPT_CORE0_CPU_INT_PRI_7" ] pub cpu_int_pri_7 : crate :: Reg < cpu_int_pri_7 :: CPU_INT_PRI_7_SPEC > , # [ doc = "0x134 - INTERRUPT_CORE0_CPU_INT_PRI_8" ] pub cpu_int_pri_8 : crate :: Reg < cpu_int_pri_8 :: CPU_INT_PRI_8_SPEC > , # [ doc = "0x138 - INTERRUPT_CORE0_CPU_INT_PRI_9" ] pub cpu_int_pri_9 : crate :: Reg < cpu_int_pri_9 :: CPU_INT_PRI_9_SPEC > , # [ doc = "0x13c - INTERRUPT_CORE0_CPU_INT_PRI_10" ] pub cpu_int_pri_10 : crate :: Reg < cpu_int_pri_10 :: CPU_INT_PRI_10_SPEC > , # [ doc = "0x140 - INTERRUPT_CORE0_CPU_INT_PRI_11" ] pub cpu_int_pri_11 : crate :: Reg < cpu_int_pri_11 :: CPU_INT_PRI_11_SPEC > , # [ doc = "0x144 - INTERRUPT_CORE0_CPU_INT_PRI_12" ] pub cpu_int_pri_12 : crate :: Reg < cpu_int_pri_12 :: CPU_INT_PRI_12_SPEC > , # [ doc = "0x148 - INTERRUPT_CORE0_CPU_INT_PRI_13" ] pub cpu_int_pri_13 : crate :: Reg < cpu_int_pri_13 :: CPU_INT_PRI_13_SPEC > , # [ doc = "0x14c - INTERRUPT_CORE0_CPU_INT_PRI_14" ] pub cpu_int_pri_14 : crate :: Reg < cpu_int_pri_14 :: CPU_INT_PRI_14_SPEC > , # [ doc = "0x150 - INTERRUPT_CORE0_CPU_INT_PRI_15" ] pub cpu_int_pri_15 : crate :: Reg < cpu_int_pri_15 :: CPU_INT_PRI_15_SPEC > , # [ doc = "0x154 - INTERRUPT_CORE0_CPU_INT_PRI_16" ] pub cpu_int_pri_16 : crate :: Reg < cpu_int_pri_16 :: CPU_INT_PRI_16_SPEC > , # [ doc = "0x158 - INTERRUPT_CORE0_CPU_INT_PRI_17" ] pub cpu_int_pri_17 : crate :: Reg < cpu_int_pri_17 :: CPU_INT_PRI_17_SPEC > , # [ doc = "0x15c - INTERRUPT_CORE0_CPU_INT_PRI_18" ] pub cpu_int_pri_18 : crate :: Reg < cpu_int_pri_18 :: CPU_INT_PRI_18_SPEC > , # [ doc = "0x160 - INTERRUPT_CORE0_CPU_INT_PRI_19" ] pub cpu_int_pri_19 : crate :: Reg < cpu_int_pri_19 :: CPU_INT_PRI_19_SPEC > , # [ doc = "0x164 - INTERRUPT_CORE0_CPU_INT_PRI_20" ] pub cpu_int_pri_20 : crate :: Reg < cpu_int_pri_20 :: CPU_INT_PRI_20_SPEC > , # [ doc = "0x168 - INTERRUPT_CORE0_CPU_INT_PRI_21" ] pub cpu_int_pri_21 : crate :: Reg < cpu_int_pri_21 :: CPU_INT_PRI_21_SPEC > , # [ doc = "0x16c - INTERRUPT_CORE0_CPU_INT_PRI_22" ] pub cpu_int_pri_22 : crate :: Reg < cpu_int_pri_22 :: CPU_INT_PRI_22_SPEC > , # [ doc = "0x170 - INTERRUPT_CORE0_CPU_INT_PRI_23" ] pub cpu_int_pri_23 : crate :: Reg < cpu_int_pri_23 :: CPU_INT_PRI_23_SPEC > , # [ doc = "0x174 - INTERRUPT_CORE0_CPU_INT_PRI_24" ] pub cpu_int_pri_24 : crate :: Reg < cpu_int_pri_24 :: CPU_INT_PRI_24_SPEC > , # [ doc = "0x178 - INTERRUPT_CORE0_CPU_INT_PRI_25" ] pub cpu_int_pri_25 : crate :: Reg < cpu_int_pri_25 :: CPU_INT_PRI_25_SPEC > , # [ doc = "0x17c - INTERRUPT_CORE0_CPU_INT_PRI_26" ] pub cpu_int_pri_26 : crate :: Reg < cpu_int_pri_26 :: CPU_INT_PRI_26_SPEC > , # [ doc = "0x180 - INTERRUPT_CORE0_CPU_INT_PRI_27" ] pub cpu_int_pri_27 : crate :: Reg < cpu_int_pri_27 :: CPU_INT_PRI_27_SPEC > , # [ doc = "0x184 - INTERRUPT_CORE0_CPU_INT_PRI_28" ] pub cpu_int_pri_28 : crate :: Reg < cpu_int_pri_28 :: CPU_INT_PRI_28_SPEC > , # [ doc = "0x188 - INTERRUPT_CORE0_CPU_INT_PRI_29" ] pub cpu_int_pri_29 : crate :: Reg < cpu_int_pri_29 :: CPU_INT_PRI_29_SPEC > , # [ doc = "0x18c - INTERRUPT_CORE0_CPU_INT_PRI_30" ] pub cpu_int_pri_30 : crate :: Reg < cpu_int_pri_30 :: CPU_INT_PRI_30_SPEC > , # [ doc = "0x190 - INTERRUPT_CORE0_CPU_INT_PRI_31" ] pub cpu_int_pri_31 : crate :: Reg < cpu_int_pri_31 :: CPU_INT_PRI_31_SPEC > , # [ doc = "0x194 - INTERRUPT_CORE0_CPU_INT_THRESH" ] pub cpu_int_thresh : crate :: Reg < cpu_int_thresh :: CPU_INT_THRESH_SPEC > , _reserved102 : [ u8 ; 0x0664 ] , # [ doc = "0x7fc - INTERRUPT_CORE0_INTERRUPT_DATE" ] pub interrupt_date : crate :: Reg < interrupt_date :: INTERRUPT_DATE_SPEC > , }
#[doc = "MAC_INTR_MAP register accessor: an alias for `Reg<MAC_INTR_MAP_SPEC>`"]
pub type MAC_INTR_MAP = crate::Reg<mac_intr_map::MAC_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_MAC_INTR_MAP"]
pub mod mac_intr_map;
#[doc = "MAC_NMI_MAP register accessor: an alias for `Reg<MAC_NMI_MAP_SPEC>`"]
pub type MAC_NMI_MAP = crate::Reg<mac_nmi_map::MAC_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_MAC_NMI_MAP"]
pub mod mac_nmi_map;
#[doc = "PWR_INTR_MAP register accessor: an alias for `Reg<PWR_INTR_MAP_SPEC>`"]
pub type PWR_INTR_MAP = crate::Reg<pwr_intr_map::PWR_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_PWR_INTR_MAP"]
pub mod pwr_intr_map;
#[doc = "BB_INT_MAP register accessor: an alias for `Reg<BB_INT_MAP_SPEC>`"]
pub type BB_INT_MAP = crate::Reg<bb_int_map::BB_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BB_INT_MAP"]
pub mod bb_int_map;
#[doc = "BT_MAC_INT_MAP register accessor: an alias for `Reg<BT_MAC_INT_MAP_SPEC>`"]
pub type BT_MAC_INT_MAP = crate::Reg<bt_mac_int_map::BT_MAC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BT_MAC_INT_MAP"]
pub mod bt_mac_int_map;
#[doc = "BT_BB_INT_MAP register accessor: an alias for `Reg<BT_BB_INT_MAP_SPEC>`"]
pub type BT_BB_INT_MAP = crate::Reg<bt_bb_int_map::BT_BB_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BT_BB_INT_MAP"]
pub mod bt_bb_int_map;
#[doc = "BT_BB_NMI_MAP register accessor: an alias for `Reg<BT_BB_NMI_MAP_SPEC>`"]
pub type BT_BB_NMI_MAP = crate::Reg<bt_bb_nmi_map::BT_BB_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BT_BB_NMI_MAP"]
pub mod bt_bb_nmi_map;
#[doc = "RWBT_IRQ_MAP register accessor: an alias for `Reg<RWBT_IRQ_MAP_SPEC>`"]
pub type RWBT_IRQ_MAP = crate::Reg<rwbt_irq_map::RWBT_IRQ_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBT_IRQ_MAP"]
pub mod rwbt_irq_map;
#[doc = "RWBLE_IRQ_MAP register accessor: an alias for `Reg<RWBLE_IRQ_MAP_SPEC>`"]
pub type RWBLE_IRQ_MAP = crate::Reg<rwble_irq_map::RWBLE_IRQ_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBLE_IRQ_MAP"]
pub mod rwble_irq_map;
#[doc = "RWBT_NMI_MAP register accessor: an alias for `Reg<RWBT_NMI_MAP_SPEC>`"]
pub type RWBT_NMI_MAP = crate::Reg<rwbt_nmi_map::RWBT_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBT_NMI_MAP"]
pub mod rwbt_nmi_map;
#[doc = "RWBLE_NMI_MAP register accessor: an alias for `Reg<RWBLE_NMI_MAP_SPEC>`"]
pub type RWBLE_NMI_MAP = crate::Reg<rwble_nmi_map::RWBLE_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBLE_NMI_MAP"]
pub mod rwble_nmi_map;
#[doc = "I2C_MST_INT_MAP register accessor: an alias for `Reg<I2C_MST_INT_MAP_SPEC>`"]
pub type I2C_MST_INT_MAP = crate::Reg<i2c_mst_int_map::I2C_MST_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_I2C_MST_INT_MAP"]
pub mod i2c_mst_int_map;
#[doc = "SLC0_INTR_MAP register accessor: an alias for `Reg<SLC0_INTR_MAP_SPEC>`"]
pub type SLC0_INTR_MAP = crate::Reg<slc0_intr_map::SLC0_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SLC0_INTR_MAP"]
pub mod slc0_intr_map;
#[doc = "SLC1_INTR_MAP register accessor: an alias for `Reg<SLC1_INTR_MAP_SPEC>`"]
pub type SLC1_INTR_MAP = crate::Reg<slc1_intr_map::SLC1_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SLC1_INTR_MAP"]
pub mod slc1_intr_map;
#[doc = "APB_CTRL_INTR_MAP register accessor: an alias for `Reg<APB_CTRL_INTR_MAP_SPEC>`"]
pub type APB_CTRL_INTR_MAP = crate::Reg<apb_ctrl_intr_map::APB_CTRL_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_APB_CTRL_INTR_MAP"]
pub mod apb_ctrl_intr_map;
#[doc = "UHCI0_INTR_MAP register accessor: an alias for `Reg<UHCI0_INTR_MAP_SPEC>`"]
pub type UHCI0_INTR_MAP = crate::Reg<uhci0_intr_map::UHCI0_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_UHCI0_INTR_MAP"]
pub mod uhci0_intr_map;
#[doc = "GPIO_INTERRUPT_PRO_MAP register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_MAP = crate::Reg<gpio_interrupt_pro_map::GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP"]
pub mod gpio_interrupt_pro_map;
#[doc = "GPIO_INTERRUPT_PRO_NMI_MAP register accessor: an alias for `Reg<GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>`"]
pub type GPIO_INTERRUPT_PRO_NMI_MAP =
    crate::Reg<gpio_interrupt_pro_nmi_map::GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP"]
pub mod gpio_interrupt_pro_nmi_map;
#[doc = "SPI_INTR_1_MAP register accessor: an alias for `Reg<SPI_INTR_1_MAP_SPEC>`"]
pub type SPI_INTR_1_MAP = crate::Reg<spi_intr_1_map::SPI_INTR_1_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SPI_INTR_1_MAP"]
pub mod spi_intr_1_map;
#[doc = "SPI_INTR_2_MAP register accessor: an alias for `Reg<SPI_INTR_2_MAP_SPEC>`"]
pub type SPI_INTR_2_MAP = crate::Reg<spi_intr_2_map::SPI_INTR_2_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SPI_INTR_2_MAP"]
pub mod spi_intr_2_map;
#[doc = "I2S1_INT_MAP register accessor: an alias for `Reg<I2S1_INT_MAP_SPEC>`"]
pub type I2S1_INT_MAP = crate::Reg<i2s1_int_map::I2S1_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_I2S1_INT_MAP"]
pub mod i2s1_int_map;
#[doc = "UART_INTR_MAP register accessor: an alias for `Reg<UART_INTR_MAP_SPEC>`"]
pub type UART_INTR_MAP = crate::Reg<uart_intr_map::UART_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_UART_INTR_MAP"]
pub mod uart_intr_map;
#[doc = "UART1_INTR_MAP register accessor: an alias for `Reg<UART1_INTR_MAP_SPEC>`"]
pub type UART1_INTR_MAP = crate::Reg<uart1_intr_map::UART1_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_UART1_INTR_MAP"]
pub mod uart1_intr_map;
#[doc = "LEDC_INT_MAP register accessor: an alias for `Reg<LEDC_INT_MAP_SPEC>`"]
pub type LEDC_INT_MAP = crate::Reg<ledc_int_map::LEDC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_LEDC_INT_MAP"]
pub mod ledc_int_map;
#[doc = "EFUSE_INT_MAP register accessor: an alias for `Reg<EFUSE_INT_MAP_SPEC>`"]
pub type EFUSE_INT_MAP = crate::Reg<efuse_int_map::EFUSE_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_EFUSE_INT_MAP"]
pub mod efuse_int_map;
#[doc = "CAN_INT_MAP register accessor: an alias for `Reg<CAN_INT_MAP_SPEC>`"]
pub type CAN_INT_MAP = crate::Reg<can_int_map::CAN_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CAN_INT_MAP"]
pub mod can_int_map;
#[doc = "USB_INTR_MAP register accessor: an alias for `Reg<USB_INTR_MAP_SPEC>`"]
pub type USB_INTR_MAP = crate::Reg<usb_intr_map::USB_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_USB_INTR_MAP"]
pub mod usb_intr_map;
#[doc = "RTC_CORE_INTR_MAP register accessor: an alias for `Reg<RTC_CORE_INTR_MAP_SPEC>`"]
pub type RTC_CORE_INTR_MAP = crate::Reg<rtc_core_intr_map::RTC_CORE_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RTC_CORE_INTR_MAP"]
pub mod rtc_core_intr_map;
#[doc = "RMT_INTR_MAP register accessor: an alias for `Reg<RMT_INTR_MAP_SPEC>`"]
pub type RMT_INTR_MAP = crate::Reg<rmt_intr_map::RMT_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RMT_INTR_MAP"]
pub mod rmt_intr_map;
#[doc = "I2C_EXT0_INTR_MAP register accessor: an alias for `Reg<I2C_EXT0_INTR_MAP_SPEC>`"]
pub type I2C_EXT0_INTR_MAP = crate::Reg<i2c_ext0_intr_map::I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_I2C_EXT0_INTR_MAP"]
pub mod i2c_ext0_intr_map;
#[doc = "TIMER_INT1_MAP register accessor: an alias for `Reg<TIMER_INT1_MAP_SPEC>`"]
pub type TIMER_INT1_MAP = crate::Reg<timer_int1_map::TIMER_INT1_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TIMER_INT1_MAP"]
pub mod timer_int1_map;
#[doc = "TIMER_INT2_MAP register accessor: an alias for `Reg<TIMER_INT2_MAP_SPEC>`"]
pub type TIMER_INT2_MAP = crate::Reg<timer_int2_map::TIMER_INT2_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TIMER_INT2_MAP"]
pub mod timer_int2_map;
#[doc = "TG_T0_INT_MAP register accessor: an alias for `Reg<TG_T0_INT_MAP_SPEC>`"]
pub type TG_T0_INT_MAP = crate::Reg<tg_t0_int_map::TG_T0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG_T0_INT_MAP"]
pub mod tg_t0_int_map;
#[doc = "TG_WDT_INT_MAP register accessor: an alias for `Reg<TG_WDT_INT_MAP_SPEC>`"]
pub type TG_WDT_INT_MAP = crate::Reg<tg_wdt_int_map::TG_WDT_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG_WDT_INT_MAP"]
pub mod tg_wdt_int_map;
#[doc = "TG1_T0_INT_MAP register accessor: an alias for `Reg<TG1_T0_INT_MAP_SPEC>`"]
pub type TG1_T0_INT_MAP = crate::Reg<tg1_t0_int_map::TG1_T0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG1_T0_INT_MAP"]
pub mod tg1_t0_int_map;
#[doc = "TG1_WDT_INT_MAP register accessor: an alias for `Reg<TG1_WDT_INT_MAP_SPEC>`"]
pub type TG1_WDT_INT_MAP = crate::Reg<tg1_wdt_int_map::TG1_WDT_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG1_WDT_INT_MAP"]
pub mod tg1_wdt_int_map;
#[doc = "CACHE_IA_INT_MAP register accessor: an alias for `Reg<CACHE_IA_INT_MAP_SPEC>`"]
pub type CACHE_IA_INT_MAP = crate::Reg<cache_ia_int_map::CACHE_IA_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CACHE_IA_INT_MAP"]
pub mod cache_ia_int_map;
#[doc = "SYSTIMER_TARGET0_INT_MAP register accessor: an alias for `Reg<SYSTIMER_TARGET0_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET0_INT_MAP =
    crate::Reg<systimer_target0_int_map::SYSTIMER_TARGET0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP"]
pub mod systimer_target0_int_map;
#[doc = "SYSTIMER_TARGET1_INT_MAP register accessor: an alias for `Reg<SYSTIMER_TARGET1_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET1_INT_MAP =
    crate::Reg<systimer_target1_int_map::SYSTIMER_TARGET1_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP"]
pub mod systimer_target1_int_map;
#[doc = "SYSTIMER_TARGET2_INT_MAP register accessor: an alias for `Reg<SYSTIMER_TARGET2_INT_MAP_SPEC>`"]
pub type SYSTIMER_TARGET2_INT_MAP =
    crate::Reg<systimer_target2_int_map::SYSTIMER_TARGET2_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP"]
pub mod systimer_target2_int_map;
#[doc = "SPI_MEM_REJECT_INTR_MAP register accessor: an alias for `Reg<SPI_MEM_REJECT_INTR_MAP_SPEC>`"]
pub type SPI_MEM_REJECT_INTR_MAP =
    crate::Reg<spi_mem_reject_intr_map::SPI_MEM_REJECT_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP"]
pub mod spi_mem_reject_intr_map;
#[doc = "ICACHE_PRELOAD_INT_MAP register accessor: an alias for `Reg<ICACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type ICACHE_PRELOAD_INT_MAP = crate::Reg<icache_preload_int_map::ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP"]
pub mod icache_preload_int_map;
#[doc = "ICACHE_SYNC_INT_MAP register accessor: an alias for `Reg<ICACHE_SYNC_INT_MAP_SPEC>`"]
pub type ICACHE_SYNC_INT_MAP = crate::Reg<icache_sync_int_map::ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP"]
pub mod icache_sync_int_map;
#[doc = "APB_ADC_INT_MAP register accessor: an alias for `Reg<APB_ADC_INT_MAP_SPEC>`"]
pub type APB_ADC_INT_MAP = crate::Reg<apb_adc_int_map::APB_ADC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_APB_ADC_INT_MAP"]
pub mod apb_adc_int_map;
#[doc = "DMA_CH0_INT_MAP register accessor: an alias for `Reg<DMA_CH0_INT_MAP_SPEC>`"]
pub type DMA_CH0_INT_MAP = crate::Reg<dma_ch0_int_map::DMA_CH0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_DMA_CH0_INT_MAP"]
pub mod dma_ch0_int_map;
#[doc = "DMA_CH1_INT_MAP register accessor: an alias for `Reg<DMA_CH1_INT_MAP_SPEC>`"]
pub type DMA_CH1_INT_MAP = crate::Reg<dma_ch1_int_map::DMA_CH1_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_DMA_CH1_INT_MAP"]
pub mod dma_ch1_int_map;
#[doc = "DMA_CH2_INT_MAP register accessor: an alias for `Reg<DMA_CH2_INT_MAP_SPEC>`"]
pub type DMA_CH2_INT_MAP = crate::Reg<dma_ch2_int_map::DMA_CH2_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_DMA_CH2_INT_MAP"]
pub mod dma_ch2_int_map;
#[doc = "RSA_INT_MAP register accessor: an alias for `Reg<RSA_INT_MAP_SPEC>`"]
pub type RSA_INT_MAP = crate::Reg<rsa_int_map::RSA_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RSA_INT_MAP"]
pub mod rsa_int_map;
#[doc = "AES_INT_MAP register accessor: an alias for `Reg<AES_INT_MAP_SPEC>`"]
pub type AES_INT_MAP = crate::Reg<aes_int_map::AES_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_AES_INT_MAP"]
pub mod aes_int_map;
#[doc = "SHA_INT_MAP register accessor: an alias for `Reg<SHA_INT_MAP_SPEC>`"]
pub type SHA_INT_MAP = crate::Reg<sha_int_map::SHA_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SHA_INT_MAP"]
pub mod sha_int_map;
#[doc = "CPU_INTR_FROM_CPU_0_MAP register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0_MAP =
    crate::Reg<cpu_intr_from_cpu_0_map::CPU_INTR_FROM_CPU_0_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP"]
pub mod cpu_intr_from_cpu_0_map;
#[doc = "CPU_INTR_FROM_CPU_1_MAP register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1_MAP =
    crate::Reg<cpu_intr_from_cpu_1_map::CPU_INTR_FROM_CPU_1_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP"]
pub mod cpu_intr_from_cpu_1_map;
#[doc = "CPU_INTR_FROM_CPU_2_MAP register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2_MAP =
    crate::Reg<cpu_intr_from_cpu_2_map::CPU_INTR_FROM_CPU_2_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP"]
pub mod cpu_intr_from_cpu_2_map;
#[doc = "CPU_INTR_FROM_CPU_3_MAP register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3_MAP =
    crate::Reg<cpu_intr_from_cpu_3_map::CPU_INTR_FROM_CPU_3_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP"]
pub mod cpu_intr_from_cpu_3_map;
#[doc = "ASSIST_DEBUG_INTR_MAP register accessor: an alias for `Reg<ASSIST_DEBUG_INTR_MAP_SPEC>`"]
pub type ASSIST_DEBUG_INTR_MAP = crate::Reg<assist_debug_intr_map::ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP"]
pub mod assist_debug_intr_map;
#[doc = "DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    dma_apbperi_pms_monitor_violate_intr_map::DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_iram0_pms_monitor_violate_intr_map::CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_dram0_pms_monitor_violate_intr_map::CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod core_0_pif_pms_monitor_violate_intr_map;
#[doc = "CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP register accessor: an alias for `Reg<CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC>`"]
pub type CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate::Reg<
    core_0_pif_pms_monitor_violate_size_intr_map::CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP"]
pub mod core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "BACKUP_PMS_VIOLATE_INTR_MAP register accessor: an alias for `Reg<BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>`"]
pub type BACKUP_PMS_VIOLATE_INTR_MAP =
    crate::Reg<backup_pms_violate_intr_map::BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP"]
pub mod backup_pms_violate_intr_map;
#[doc = "CACHE_CORE0_ACS_INT_MAP register accessor: an alias for `Reg<CACHE_CORE0_ACS_INT_MAP_SPEC>`"]
pub type CACHE_CORE0_ACS_INT_MAP =
    crate::Reg<cache_core0_acs_int_map::CACHE_CORE0_ACS_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP"]
pub mod cache_core0_acs_int_map;
#[doc = "INTR_STATUS_0 register accessor: an alias for `Reg<INTR_STATUS_0_SPEC>`"]
pub type INTR_STATUS_0 = crate::Reg<intr_status_0::INTR_STATUS_0_SPEC>;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_0"]
pub mod intr_status_0;
#[doc = "INTR_STATUS_1 register accessor: an alias for `Reg<INTR_STATUS_1_SPEC>`"]
pub type INTR_STATUS_1 = crate::Reg<intr_status_1::INTR_STATUS_1_SPEC>;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_1"]
pub mod intr_status_1;
#[doc = "CLOCK_GATE register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "INTERRUPT_CORE0_CLOCK_GATE"]
pub mod clock_gate;
#[doc = "CPU_INT_ENABLE register accessor: an alias for `Reg<CPU_INT_ENABLE_SPEC>`"]
pub type CPU_INT_ENABLE = crate::Reg<cpu_int_enable::CPU_INT_ENABLE_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_ENABLE"]
pub mod cpu_int_enable;
#[doc = "CPU_INT_TYPE register accessor: an alias for `Reg<CPU_INT_TYPE_SPEC>`"]
pub type CPU_INT_TYPE = crate::Reg<cpu_int_type::CPU_INT_TYPE_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_TYPE"]
pub mod cpu_int_type;
#[doc = "CPU_INT_CLEAR register accessor: an alias for `Reg<CPU_INT_CLEAR_SPEC>`"]
pub type CPU_INT_CLEAR = crate::Reg<cpu_int_clear::CPU_INT_CLEAR_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_CLEAR"]
pub mod cpu_int_clear;
#[doc = "CPU_INT_EIP_STATUS register accessor: an alias for `Reg<CPU_INT_EIP_STATUS_SPEC>`"]
pub type CPU_INT_EIP_STATUS = crate::Reg<cpu_int_eip_status::CPU_INT_EIP_STATUS_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_EIP_STATUS"]
pub mod cpu_int_eip_status;
#[doc = "CPU_INT_PRI_0 register accessor: an alias for `Reg<CPU_INT_PRI_0_SPEC>`"]
pub type CPU_INT_PRI_0 = crate::Reg<cpu_int_pri_0::CPU_INT_PRI_0_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_0"]
pub mod cpu_int_pri_0;
#[doc = "CPU_INT_PRI_1 register accessor: an alias for `Reg<CPU_INT_PRI_1_SPEC>`"]
pub type CPU_INT_PRI_1 = crate::Reg<cpu_int_pri_1::CPU_INT_PRI_1_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_1"]
pub mod cpu_int_pri_1;
#[doc = "CPU_INT_PRI_2 register accessor: an alias for `Reg<CPU_INT_PRI_2_SPEC>`"]
pub type CPU_INT_PRI_2 = crate::Reg<cpu_int_pri_2::CPU_INT_PRI_2_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_2"]
pub mod cpu_int_pri_2;
#[doc = "CPU_INT_PRI_3 register accessor: an alias for `Reg<CPU_INT_PRI_3_SPEC>`"]
pub type CPU_INT_PRI_3 = crate::Reg<cpu_int_pri_3::CPU_INT_PRI_3_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_3"]
pub mod cpu_int_pri_3;
#[doc = "CPU_INT_PRI_4 register accessor: an alias for `Reg<CPU_INT_PRI_4_SPEC>`"]
pub type CPU_INT_PRI_4 = crate::Reg<cpu_int_pri_4::CPU_INT_PRI_4_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_4"]
pub mod cpu_int_pri_4;
#[doc = "CPU_INT_PRI_5 register accessor: an alias for `Reg<CPU_INT_PRI_5_SPEC>`"]
pub type CPU_INT_PRI_5 = crate::Reg<cpu_int_pri_5::CPU_INT_PRI_5_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_5"]
pub mod cpu_int_pri_5;
#[doc = "CPU_INT_PRI_6 register accessor: an alias for `Reg<CPU_INT_PRI_6_SPEC>`"]
pub type CPU_INT_PRI_6 = crate::Reg<cpu_int_pri_6::CPU_INT_PRI_6_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_6"]
pub mod cpu_int_pri_6;
#[doc = "CPU_INT_PRI_7 register accessor: an alias for `Reg<CPU_INT_PRI_7_SPEC>`"]
pub type CPU_INT_PRI_7 = crate::Reg<cpu_int_pri_7::CPU_INT_PRI_7_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_7"]
pub mod cpu_int_pri_7;
#[doc = "CPU_INT_PRI_8 register accessor: an alias for `Reg<CPU_INT_PRI_8_SPEC>`"]
pub type CPU_INT_PRI_8 = crate::Reg<cpu_int_pri_8::CPU_INT_PRI_8_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_8"]
pub mod cpu_int_pri_8;
#[doc = "CPU_INT_PRI_9 register accessor: an alias for `Reg<CPU_INT_PRI_9_SPEC>`"]
pub type CPU_INT_PRI_9 = crate::Reg<cpu_int_pri_9::CPU_INT_PRI_9_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_9"]
pub mod cpu_int_pri_9;
#[doc = "CPU_INT_PRI_10 register accessor: an alias for `Reg<CPU_INT_PRI_10_SPEC>`"]
pub type CPU_INT_PRI_10 = crate::Reg<cpu_int_pri_10::CPU_INT_PRI_10_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_10"]
pub mod cpu_int_pri_10;
#[doc = "CPU_INT_PRI_11 register accessor: an alias for `Reg<CPU_INT_PRI_11_SPEC>`"]
pub type CPU_INT_PRI_11 = crate::Reg<cpu_int_pri_11::CPU_INT_PRI_11_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_11"]
pub mod cpu_int_pri_11;
#[doc = "CPU_INT_PRI_12 register accessor: an alias for `Reg<CPU_INT_PRI_12_SPEC>`"]
pub type CPU_INT_PRI_12 = crate::Reg<cpu_int_pri_12::CPU_INT_PRI_12_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_12"]
pub mod cpu_int_pri_12;
#[doc = "CPU_INT_PRI_13 register accessor: an alias for `Reg<CPU_INT_PRI_13_SPEC>`"]
pub type CPU_INT_PRI_13 = crate::Reg<cpu_int_pri_13::CPU_INT_PRI_13_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_13"]
pub mod cpu_int_pri_13;
#[doc = "CPU_INT_PRI_14 register accessor: an alias for `Reg<CPU_INT_PRI_14_SPEC>`"]
pub type CPU_INT_PRI_14 = crate::Reg<cpu_int_pri_14::CPU_INT_PRI_14_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_14"]
pub mod cpu_int_pri_14;
#[doc = "CPU_INT_PRI_15 register accessor: an alias for `Reg<CPU_INT_PRI_15_SPEC>`"]
pub type CPU_INT_PRI_15 = crate::Reg<cpu_int_pri_15::CPU_INT_PRI_15_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_15"]
pub mod cpu_int_pri_15;
#[doc = "CPU_INT_PRI_16 register accessor: an alias for `Reg<CPU_INT_PRI_16_SPEC>`"]
pub type CPU_INT_PRI_16 = crate::Reg<cpu_int_pri_16::CPU_INT_PRI_16_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_16"]
pub mod cpu_int_pri_16;
#[doc = "CPU_INT_PRI_17 register accessor: an alias for `Reg<CPU_INT_PRI_17_SPEC>`"]
pub type CPU_INT_PRI_17 = crate::Reg<cpu_int_pri_17::CPU_INT_PRI_17_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_17"]
pub mod cpu_int_pri_17;
#[doc = "CPU_INT_PRI_18 register accessor: an alias for `Reg<CPU_INT_PRI_18_SPEC>`"]
pub type CPU_INT_PRI_18 = crate::Reg<cpu_int_pri_18::CPU_INT_PRI_18_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_18"]
pub mod cpu_int_pri_18;
#[doc = "CPU_INT_PRI_19 register accessor: an alias for `Reg<CPU_INT_PRI_19_SPEC>`"]
pub type CPU_INT_PRI_19 = crate::Reg<cpu_int_pri_19::CPU_INT_PRI_19_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_19"]
pub mod cpu_int_pri_19;
#[doc = "CPU_INT_PRI_20 register accessor: an alias for `Reg<CPU_INT_PRI_20_SPEC>`"]
pub type CPU_INT_PRI_20 = crate::Reg<cpu_int_pri_20::CPU_INT_PRI_20_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_20"]
pub mod cpu_int_pri_20;
#[doc = "CPU_INT_PRI_21 register accessor: an alias for `Reg<CPU_INT_PRI_21_SPEC>`"]
pub type CPU_INT_PRI_21 = crate::Reg<cpu_int_pri_21::CPU_INT_PRI_21_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_21"]
pub mod cpu_int_pri_21;
#[doc = "CPU_INT_PRI_22 register accessor: an alias for `Reg<CPU_INT_PRI_22_SPEC>`"]
pub type CPU_INT_PRI_22 = crate::Reg<cpu_int_pri_22::CPU_INT_PRI_22_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_22"]
pub mod cpu_int_pri_22;
#[doc = "CPU_INT_PRI_23 register accessor: an alias for `Reg<CPU_INT_PRI_23_SPEC>`"]
pub type CPU_INT_PRI_23 = crate::Reg<cpu_int_pri_23::CPU_INT_PRI_23_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_23"]
pub mod cpu_int_pri_23;
#[doc = "CPU_INT_PRI_24 register accessor: an alias for `Reg<CPU_INT_PRI_24_SPEC>`"]
pub type CPU_INT_PRI_24 = crate::Reg<cpu_int_pri_24::CPU_INT_PRI_24_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_24"]
pub mod cpu_int_pri_24;
#[doc = "CPU_INT_PRI_25 register accessor: an alias for `Reg<CPU_INT_PRI_25_SPEC>`"]
pub type CPU_INT_PRI_25 = crate::Reg<cpu_int_pri_25::CPU_INT_PRI_25_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_25"]
pub mod cpu_int_pri_25;
#[doc = "CPU_INT_PRI_26 register accessor: an alias for `Reg<CPU_INT_PRI_26_SPEC>`"]
pub type CPU_INT_PRI_26 = crate::Reg<cpu_int_pri_26::CPU_INT_PRI_26_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_26"]
pub mod cpu_int_pri_26;
#[doc = "CPU_INT_PRI_27 register accessor: an alias for `Reg<CPU_INT_PRI_27_SPEC>`"]
pub type CPU_INT_PRI_27 = crate::Reg<cpu_int_pri_27::CPU_INT_PRI_27_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_27"]
pub mod cpu_int_pri_27;
#[doc = "CPU_INT_PRI_28 register accessor: an alias for `Reg<CPU_INT_PRI_28_SPEC>`"]
pub type CPU_INT_PRI_28 = crate::Reg<cpu_int_pri_28::CPU_INT_PRI_28_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_28"]
pub mod cpu_int_pri_28;
#[doc = "CPU_INT_PRI_29 register accessor: an alias for `Reg<CPU_INT_PRI_29_SPEC>`"]
pub type CPU_INT_PRI_29 = crate::Reg<cpu_int_pri_29::CPU_INT_PRI_29_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_29"]
pub mod cpu_int_pri_29;
#[doc = "CPU_INT_PRI_30 register accessor: an alias for `Reg<CPU_INT_PRI_30_SPEC>`"]
pub type CPU_INT_PRI_30 = crate::Reg<cpu_int_pri_30::CPU_INT_PRI_30_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_30"]
pub mod cpu_int_pri_30;
#[doc = "CPU_INT_PRI_31 register accessor: an alias for `Reg<CPU_INT_PRI_31_SPEC>`"]
pub type CPU_INT_PRI_31 = crate::Reg<cpu_int_pri_31::CPU_INT_PRI_31_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_31"]
pub mod cpu_int_pri_31;
#[doc = "CPU_INT_THRESH register accessor: an alias for `Reg<CPU_INT_THRESH_SPEC>`"]
pub type CPU_INT_THRESH = crate::Reg<cpu_int_thresh::CPU_INT_THRESH_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_THRESH"]
pub mod cpu_int_thresh;
#[doc = "INTERRUPT_DATE register accessor: an alias for `Reg<INTERRUPT_DATE_SPEC>`"]
pub type INTERRUPT_DATE = crate::Reg<interrupt_date::INTERRUPT_DATE_SPEC>;
#[doc = "INTERRUPT_CORE0_INTERRUPT_DATE"]
pub mod interrupt_date;
