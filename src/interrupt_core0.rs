#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock { # [ doc = "0x00 - INTERRUPT_CORE0_MAC_INTR_MAP" ] pub interrupt_core0_mac_intr_map : crate :: Reg < interrupt_core0_mac_intr_map :: INTERRUPT_CORE0_MAC_INTR_MAP_SPEC > , # [ doc = "0x04 - INTERRUPT_CORE0_MAC_NMI_MAP" ] pub interrupt_core0_mac_nmi_map : crate :: Reg < interrupt_core0_mac_nmi_map :: INTERRUPT_CORE0_MAC_NMI_MAP_SPEC > , # [ doc = "0x08 - INTERRUPT_CORE0_PWR_INTR_MAP" ] pub interrupt_core0_pwr_intr_map : crate :: Reg < interrupt_core0_pwr_intr_map :: INTERRUPT_CORE0_PWR_INTR_MAP_SPEC > , # [ doc = "0x0c - INTERRUPT_CORE0_BB_INT_MAP" ] pub interrupt_core0_bb_int_map : crate :: Reg < interrupt_core0_bb_int_map :: INTERRUPT_CORE0_BB_INT_MAP_SPEC > , # [ doc = "0x10 - INTERRUPT_CORE0_BT_MAC_INT_MAP" ] pub interrupt_core0_bt_mac_int_map : crate :: Reg < interrupt_core0_bt_mac_int_map :: INTERRUPT_CORE0_BT_MAC_INT_MAP_SPEC > , # [ doc = "0x14 - INTERRUPT_CORE0_BT_BB_INT_MAP" ] pub interrupt_core0_bt_bb_int_map : crate :: Reg < interrupt_core0_bt_bb_int_map :: INTERRUPT_CORE0_BT_BB_INT_MAP_SPEC > , # [ doc = "0x18 - INTERRUPT_CORE0_BT_BB_NMI_MAP" ] pub interrupt_core0_bt_bb_nmi_map : crate :: Reg < interrupt_core0_bt_bb_nmi_map :: INTERRUPT_CORE0_BT_BB_NMI_MAP_SPEC > , # [ doc = "0x1c - INTERRUPT_CORE0_RWBT_IRQ_MAP" ] pub interrupt_core0_rwbt_irq_map : crate :: Reg < interrupt_core0_rwbt_irq_map :: INTERRUPT_CORE0_RWBT_IRQ_MAP_SPEC > , # [ doc = "0x20 - INTERRUPT_CORE0_RWBLE_IRQ_MAP" ] pub interrupt_core0_rwble_irq_map : crate :: Reg < interrupt_core0_rwble_irq_map :: INTERRUPT_CORE0_RWBLE_IRQ_MAP_SPEC > , # [ doc = "0x24 - INTERRUPT_CORE0_RWBT_NMI_MAP" ] pub interrupt_core0_rwbt_nmi_map : crate :: Reg < interrupt_core0_rwbt_nmi_map :: INTERRUPT_CORE0_RWBT_NMI_MAP_SPEC > , # [ doc = "0x28 - INTERRUPT_CORE0_RWBLE_NMI_MAP" ] pub interrupt_core0_rwble_nmi_map : crate :: Reg < interrupt_core0_rwble_nmi_map :: INTERRUPT_CORE0_RWBLE_NMI_MAP_SPEC > , # [ doc = "0x2c - INTERRUPT_CORE0_I2C_MST_INT_MAP" ] pub interrupt_core0_i2c_mst_int_map : crate :: Reg < interrupt_core0_i2c_mst_int_map :: INTERRUPT_CORE0_I2C_MST_INT_MAP_SPEC > , # [ doc = "0x30 - INTERRUPT_CORE0_SLC0_INTR_MAP" ] pub interrupt_core0_slc0_intr_map : crate :: Reg < interrupt_core0_slc0_intr_map :: INTERRUPT_CORE0_SLC0_INTR_MAP_SPEC > , # [ doc = "0x34 - INTERRUPT_CORE0_SLC1_INTR_MAP" ] pub interrupt_core0_slc1_intr_map : crate :: Reg < interrupt_core0_slc1_intr_map :: INTERRUPT_CORE0_SLC1_INTR_MAP_SPEC > , # [ doc = "0x38 - INTERRUPT_CORE0_APB_CTRL_INTR_MAP" ] pub interrupt_core0_apb_ctrl_intr_map : crate :: Reg < interrupt_core0_apb_ctrl_intr_map :: INTERRUPT_CORE0_APB_CTRL_INTR_MAP_SPEC > , # [ doc = "0x3c - INTERRUPT_CORE0_UHCI0_INTR_MAP" ] pub interrupt_core0_uhci0_intr_map : crate :: Reg < interrupt_core0_uhci0_intr_map :: INTERRUPT_CORE0_UHCI0_INTR_MAP_SPEC > , # [ doc = "0x40 - INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP" ] pub interrupt_core0_gpio_interrupt_pro_map : crate :: Reg < interrupt_core0_gpio_interrupt_pro_map :: INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP_SPEC > , # [ doc = "0x44 - INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP" ] pub interrupt_core0_gpio_interrupt_pro_nmi_map : crate :: Reg < interrupt_core0_gpio_interrupt_pro_nmi_map :: INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP_SPEC > , # [ doc = "0x48 - INTERRUPT_CORE0_SPI_INTR_1_MAP" ] pub interrupt_core0_spi_intr_1_map : crate :: Reg < interrupt_core0_spi_intr_1_map :: INTERRUPT_CORE0_SPI_INTR_1_MAP_SPEC > , # [ doc = "0x4c - INTERRUPT_CORE0_SPI_INTR_2_MAP" ] pub interrupt_core0_spi_intr_2_map : crate :: Reg < interrupt_core0_spi_intr_2_map :: INTERRUPT_CORE0_SPI_INTR_2_MAP_SPEC > , # [ doc = "0x50 - INTERRUPT_CORE0_I2S1_INT_MAP" ] pub interrupt_core0_i2s1_int_map : crate :: Reg < interrupt_core0_i2s1_int_map :: INTERRUPT_CORE0_I2S1_INT_MAP_SPEC > , # [ doc = "0x54 - INTERRUPT_CORE0_UART_INTR_MAP" ] pub interrupt_core0_uart_intr_map : crate :: Reg < interrupt_core0_uart_intr_map :: INTERRUPT_CORE0_UART_INTR_MAP_SPEC > , # [ doc = "0x58 - INTERRUPT_CORE0_UART1_INTR_MAP" ] pub interrupt_core0_uart1_intr_map : crate :: Reg < interrupt_core0_uart1_intr_map :: INTERRUPT_CORE0_UART1_INTR_MAP_SPEC > , # [ doc = "0x5c - INTERRUPT_CORE0_LEDC_INT_MAP" ] pub interrupt_core0_ledc_int_map : crate :: Reg < interrupt_core0_ledc_int_map :: INTERRUPT_CORE0_LEDC_INT_MAP_SPEC > , # [ doc = "0x60 - INTERRUPT_CORE0_EFUSE_INT_MAP" ] pub interrupt_core0_efuse_int_map : crate :: Reg < interrupt_core0_efuse_int_map :: INTERRUPT_CORE0_EFUSE_INT_MAP_SPEC > , # [ doc = "0x64 - INTERRUPT_CORE0_CAN_INT_MAP" ] pub interrupt_core0_can_int_map : crate :: Reg < interrupt_core0_can_int_map :: INTERRUPT_CORE0_CAN_INT_MAP_SPEC > , # [ doc = "0x68 - INTERRUPT_CORE0_USB_INTR_MAP" ] pub interrupt_core0_usb_intr_map : crate :: Reg < interrupt_core0_usb_intr_map :: INTERRUPT_CORE0_USB_INTR_MAP_SPEC > , # [ doc = "0x6c - INTERRUPT_CORE0_RTC_CORE_INTR_MAP" ] pub interrupt_core0_rtc_core_intr_map : crate :: Reg < interrupt_core0_rtc_core_intr_map :: INTERRUPT_CORE0_RTC_CORE_INTR_MAP_SPEC > , # [ doc = "0x70 - INTERRUPT_CORE0_RMT_INTR_MAP" ] pub interrupt_core0_rmt_intr_map : crate :: Reg < interrupt_core0_rmt_intr_map :: INTERRUPT_CORE0_RMT_INTR_MAP_SPEC > , # [ doc = "0x74 - INTERRUPT_CORE0_I2C_EXT0_INTR_MAP" ] pub interrupt_core0_i2c_ext0_intr_map : crate :: Reg < interrupt_core0_i2c_ext0_intr_map :: INTERRUPT_CORE0_I2C_EXT0_INTR_MAP_SPEC > , # [ doc = "0x78 - INTERRUPT_CORE0_TIMER_INT1_MAP" ] pub interrupt_core0_timer_int1_map : crate :: Reg < interrupt_core0_timer_int1_map :: INTERRUPT_CORE0_TIMER_INT1_MAP_SPEC > , # [ doc = "0x7c - INTERRUPT_CORE0_TIMER_INT2_MAP" ] pub interrupt_core0_timer_int2_map : crate :: Reg < interrupt_core0_timer_int2_map :: INTERRUPT_CORE0_TIMER_INT2_MAP_SPEC > , # [ doc = "0x80 - INTERRUPT_CORE0_TG_T0_INT_MAP" ] pub interrupt_core0_tg_t0_int_map : crate :: Reg < interrupt_core0_tg_t0_int_map :: INTERRUPT_CORE0_TG_T0_INT_MAP_SPEC > , # [ doc = "0x84 - INTERRUPT_CORE0_TG_WDT_INT_MAP" ] pub interrupt_core0_tg_wdt_int_map : crate :: Reg < interrupt_core0_tg_wdt_int_map :: INTERRUPT_CORE0_TG_WDT_INT_MAP_SPEC > , # [ doc = "0x88 - INTERRUPT_CORE0_TG1_T0_INT_MAP" ] pub interrupt_core0_tg1_t0_int_map : crate :: Reg < interrupt_core0_tg1_t0_int_map :: INTERRUPT_CORE0_TG1_T0_INT_MAP_SPEC > , # [ doc = "0x8c - INTERRUPT_CORE0_TG1_WDT_INT_MAP" ] pub interrupt_core0_tg1_wdt_int_map : crate :: Reg < interrupt_core0_tg1_wdt_int_map :: INTERRUPT_CORE0_TG1_WDT_INT_MAP_SPEC > , # [ doc = "0x90 - INTERRUPT_CORE0_CACHE_IA_INT_MAP" ] pub interrupt_core0_cache_ia_int_map : crate :: Reg < interrupt_core0_cache_ia_int_map :: INTERRUPT_CORE0_CACHE_IA_INT_MAP_SPEC > , # [ doc = "0x94 - INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP" ] pub interrupt_core0_systimer_target0_int_map : crate :: Reg < interrupt_core0_systimer_target0_int_map :: INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP_SPEC > , # [ doc = "0x98 - INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP" ] pub interrupt_core0_systimer_target1_int_map : crate :: Reg < interrupt_core0_systimer_target1_int_map :: INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP_SPEC > , # [ doc = "0x9c - INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP" ] pub interrupt_core0_systimer_target2_int_map : crate :: Reg < interrupt_core0_systimer_target2_int_map :: INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP_SPEC > , # [ doc = "0xa0 - INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP" ] pub interrupt_core0_spi_mem_reject_intr_map : crate :: Reg < interrupt_core0_spi_mem_reject_intr_map :: INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP_SPEC > , # [ doc = "0xa4 - INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP" ] pub interrupt_core0_icache_preload_int_map : crate :: Reg < interrupt_core0_icache_preload_int_map :: INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP_SPEC > , # [ doc = "0xa8 - INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP" ] pub interrupt_core0_icache_sync_int_map : crate :: Reg < interrupt_core0_icache_sync_int_map :: INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP_SPEC > , # [ doc = "0xac - INTERRUPT_CORE0_APB_ADC_INT_MAP" ] pub interrupt_core0_apb_adc_int_map : crate :: Reg < interrupt_core0_apb_adc_int_map :: INTERRUPT_CORE0_APB_ADC_INT_MAP_SPEC > , # [ doc = "0xb0 - INTERRUPT_CORE0_DMA_CH0_INT_MAP" ] pub interrupt_core0_dma_ch0_int_map : crate :: Reg < interrupt_core0_dma_ch0_int_map :: INTERRUPT_CORE0_DMA_CH0_INT_MAP_SPEC > , # [ doc = "0xb4 - INTERRUPT_CORE0_DMA_CH1_INT_MAP" ] pub interrupt_core0_dma_ch1_int_map : crate :: Reg < interrupt_core0_dma_ch1_int_map :: INTERRUPT_CORE0_DMA_CH1_INT_MAP_SPEC > , # [ doc = "0xb8 - INTERRUPT_CORE0_DMA_CH2_INT_MAP" ] pub interrupt_core0_dma_ch2_int_map : crate :: Reg < interrupt_core0_dma_ch2_int_map :: INTERRUPT_CORE0_DMA_CH2_INT_MAP_SPEC > , # [ doc = "0xbc - INTERRUPT_CORE0_RSA_INT_MAP" ] pub interrupt_core0_rsa_int_map : crate :: Reg < interrupt_core0_rsa_int_map :: INTERRUPT_CORE0_RSA_INT_MAP_SPEC > , # [ doc = "0xc0 - INTERRUPT_CORE0_AES_INT_MAP" ] pub interrupt_core0_aes_int_map : crate :: Reg < interrupt_core0_aes_int_map :: INTERRUPT_CORE0_AES_INT_MAP_SPEC > , # [ doc = "0xc4 - INTERRUPT_CORE0_SHA_INT_MAP" ] pub interrupt_core0_sha_int_map : crate :: Reg < interrupt_core0_sha_int_map :: INTERRUPT_CORE0_SHA_INT_MAP_SPEC > , # [ doc = "0xc8 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP" ] pub interrupt_core0_cpu_intr_from_cpu_0_map : crate :: Reg < interrupt_core0_cpu_intr_from_cpu_0_map :: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP_SPEC > , # [ doc = "0xcc - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP" ] pub interrupt_core0_cpu_intr_from_cpu_1_map : crate :: Reg < interrupt_core0_cpu_intr_from_cpu_1_map :: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP_SPEC > , # [ doc = "0xd0 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP" ] pub interrupt_core0_cpu_intr_from_cpu_2_map : crate :: Reg < interrupt_core0_cpu_intr_from_cpu_2_map :: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP_SPEC > , # [ doc = "0xd4 - INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP" ] pub interrupt_core0_cpu_intr_from_cpu_3_map : crate :: Reg < interrupt_core0_cpu_intr_from_cpu_3_map :: INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP_SPEC > , # [ doc = "0xd8 - INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP" ] pub interrupt_core0_assist_debug_intr_map : crate :: Reg < interrupt_core0_assist_debug_intr_map :: INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP_SPEC > , # [ doc = "0xdc - INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map : crate :: Reg < interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xe0 - INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map : crate :: Reg < interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xe4 - INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map : crate :: Reg < interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xe8 - INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP" ] pub interrupt_core0_core_0_pif_pms_monitor_violate_intr_map : crate :: Reg < interrupt_core0_core_0_pif_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xec - INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP" ] pub interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map : crate :: Reg < interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map :: INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC > , # [ doc = "0xf0 - INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP" ] pub interrupt_core0_backup_pms_violate_intr_map : crate :: Reg < interrupt_core0_backup_pms_violate_intr_map :: INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP_SPEC > , # [ doc = "0xf4 - INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP" ] pub interrupt_core0_cache_core0_acs_int_map : crate :: Reg < interrupt_core0_cache_core0_acs_int_map :: INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP_SPEC > , # [ doc = "0xf8 - INTERRUPT_CORE0_INTR_STATUS_0" ] pub interrupt_core0_intr_status_0 : crate :: Reg < interrupt_core0_intr_status_0 :: INTERRUPT_CORE0_INTR_STATUS_0_SPEC > , # [ doc = "0xfc - INTERRUPT_CORE0_INTR_STATUS_1" ] pub interrupt_core0_intr_status_1 : crate :: Reg < interrupt_core0_intr_status_1 :: INTERRUPT_CORE0_INTR_STATUS_1_SPEC > , # [ doc = "0x100 - INTERRUPT_CORE0_CLOCK_GATE" ] pub interrupt_core0_clock_gate : crate :: Reg < interrupt_core0_clock_gate :: INTERRUPT_CORE0_CLOCK_GATE_SPEC > , # [ doc = "0x104 - INTERRUPT_CORE0_CPU_INT_ENABLE" ] pub interrupt_core0_cpu_int_enable : crate :: Reg < interrupt_core0_cpu_int_enable :: INTERRUPT_CORE0_CPU_INT_ENABLE_SPEC > , # [ doc = "0x108 - INTERRUPT_CORE0_CPU_INT_TYPE" ] pub interrupt_core0_cpu_int_type : crate :: Reg < interrupt_core0_cpu_int_type :: INTERRUPT_CORE0_CPU_INT_TYPE_SPEC > , # [ doc = "0x10c - INTERRUPT_CORE0_CPU_INT_CLEAR" ] pub interrupt_core0_cpu_int_clear : crate :: Reg < interrupt_core0_cpu_int_clear :: INTERRUPT_CORE0_CPU_INT_CLEAR_SPEC > , # [ doc = "0x110 - INTERRUPT_CORE0_CPU_INT_EIP_STATUS" ] pub interrupt_core0_cpu_int_eip_status : crate :: Reg < interrupt_core0_cpu_int_eip_status :: INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC > , # [ doc = "0x114 - INTERRUPT_CORE0_CPU_INT_PRI_0" ] pub interrupt_core0_cpu_int_pri_0 : crate :: Reg < interrupt_core0_cpu_int_pri_0 :: INTERRUPT_CORE0_CPU_INT_PRI_0_SPEC > , # [ doc = "0x118 - INTERRUPT_CORE0_CPU_INT_PRI_1" ] pub interrupt_core0_cpu_int_pri_1 : crate :: Reg < interrupt_core0_cpu_int_pri_1 :: INTERRUPT_CORE0_CPU_INT_PRI_1_SPEC > , # [ doc = "0x11c - INTERRUPT_CORE0_CPU_INT_PRI_2" ] pub interrupt_core0_cpu_int_pri_2 : crate :: Reg < interrupt_core0_cpu_int_pri_2 :: INTERRUPT_CORE0_CPU_INT_PRI_2_SPEC > , # [ doc = "0x120 - INTERRUPT_CORE0_CPU_INT_PRI_3" ] pub interrupt_core0_cpu_int_pri_3 : crate :: Reg < interrupt_core0_cpu_int_pri_3 :: INTERRUPT_CORE0_CPU_INT_PRI_3_SPEC > , # [ doc = "0x124 - INTERRUPT_CORE0_CPU_INT_PRI_4" ] pub interrupt_core0_cpu_int_pri_4 : crate :: Reg < interrupt_core0_cpu_int_pri_4 :: INTERRUPT_CORE0_CPU_INT_PRI_4_SPEC > , # [ doc = "0x128 - INTERRUPT_CORE0_CPU_INT_PRI_5" ] pub interrupt_core0_cpu_int_pri_5 : crate :: Reg < interrupt_core0_cpu_int_pri_5 :: INTERRUPT_CORE0_CPU_INT_PRI_5_SPEC > , # [ doc = "0x12c - INTERRUPT_CORE0_CPU_INT_PRI_6" ] pub interrupt_core0_cpu_int_pri_6 : crate :: Reg < interrupt_core0_cpu_int_pri_6 :: INTERRUPT_CORE0_CPU_INT_PRI_6_SPEC > , # [ doc = "0x130 - INTERRUPT_CORE0_CPU_INT_PRI_7" ] pub interrupt_core0_cpu_int_pri_7 : crate :: Reg < interrupt_core0_cpu_int_pri_7 :: INTERRUPT_CORE0_CPU_INT_PRI_7_SPEC > , # [ doc = "0x134 - INTERRUPT_CORE0_CPU_INT_PRI_8" ] pub interrupt_core0_cpu_int_pri_8 : crate :: Reg < interrupt_core0_cpu_int_pri_8 :: INTERRUPT_CORE0_CPU_INT_PRI_8_SPEC > , # [ doc = "0x138 - INTERRUPT_CORE0_CPU_INT_PRI_9" ] pub interrupt_core0_cpu_int_pri_9 : crate :: Reg < interrupt_core0_cpu_int_pri_9 :: INTERRUPT_CORE0_CPU_INT_PRI_9_SPEC > , # [ doc = "0x13c - INTERRUPT_CORE0_CPU_INT_PRI_10" ] pub interrupt_core0_cpu_int_pri_10 : crate :: Reg < interrupt_core0_cpu_int_pri_10 :: INTERRUPT_CORE0_CPU_INT_PRI_10_SPEC > , # [ doc = "0x140 - INTERRUPT_CORE0_CPU_INT_PRI_11" ] pub interrupt_core0_cpu_int_pri_11 : crate :: Reg < interrupt_core0_cpu_int_pri_11 :: INTERRUPT_CORE0_CPU_INT_PRI_11_SPEC > , # [ doc = "0x144 - INTERRUPT_CORE0_CPU_INT_PRI_12" ] pub interrupt_core0_cpu_int_pri_12 : crate :: Reg < interrupt_core0_cpu_int_pri_12 :: INTERRUPT_CORE0_CPU_INT_PRI_12_SPEC > , # [ doc = "0x148 - INTERRUPT_CORE0_CPU_INT_PRI_13" ] pub interrupt_core0_cpu_int_pri_13 : crate :: Reg < interrupt_core0_cpu_int_pri_13 :: INTERRUPT_CORE0_CPU_INT_PRI_13_SPEC > , # [ doc = "0x14c - INTERRUPT_CORE0_CPU_INT_PRI_14" ] pub interrupt_core0_cpu_int_pri_14 : crate :: Reg < interrupt_core0_cpu_int_pri_14 :: INTERRUPT_CORE0_CPU_INT_PRI_14_SPEC > , # [ doc = "0x150 - INTERRUPT_CORE0_CPU_INT_PRI_15" ] pub interrupt_core0_cpu_int_pri_15 : crate :: Reg < interrupt_core0_cpu_int_pri_15 :: INTERRUPT_CORE0_CPU_INT_PRI_15_SPEC > , # [ doc = "0x154 - INTERRUPT_CORE0_CPU_INT_PRI_16" ] pub interrupt_core0_cpu_int_pri_16 : crate :: Reg < interrupt_core0_cpu_int_pri_16 :: INTERRUPT_CORE0_CPU_INT_PRI_16_SPEC > , # [ doc = "0x158 - INTERRUPT_CORE0_CPU_INT_PRI_17" ] pub interrupt_core0_cpu_int_pri_17 : crate :: Reg < interrupt_core0_cpu_int_pri_17 :: INTERRUPT_CORE0_CPU_INT_PRI_17_SPEC > , # [ doc = "0x15c - INTERRUPT_CORE0_CPU_INT_PRI_18" ] pub interrupt_core0_cpu_int_pri_18 : crate :: Reg < interrupt_core0_cpu_int_pri_18 :: INTERRUPT_CORE0_CPU_INT_PRI_18_SPEC > , # [ doc = "0x160 - INTERRUPT_CORE0_CPU_INT_PRI_19" ] pub interrupt_core0_cpu_int_pri_19 : crate :: Reg < interrupt_core0_cpu_int_pri_19 :: INTERRUPT_CORE0_CPU_INT_PRI_19_SPEC > , # [ doc = "0x164 - INTERRUPT_CORE0_CPU_INT_PRI_20" ] pub interrupt_core0_cpu_int_pri_20 : crate :: Reg < interrupt_core0_cpu_int_pri_20 :: INTERRUPT_CORE0_CPU_INT_PRI_20_SPEC > , # [ doc = "0x168 - INTERRUPT_CORE0_CPU_INT_PRI_21" ] pub interrupt_core0_cpu_int_pri_21 : crate :: Reg < interrupt_core0_cpu_int_pri_21 :: INTERRUPT_CORE0_CPU_INT_PRI_21_SPEC > , # [ doc = "0x16c - INTERRUPT_CORE0_CPU_INT_PRI_22" ] pub interrupt_core0_cpu_int_pri_22 : crate :: Reg < interrupt_core0_cpu_int_pri_22 :: INTERRUPT_CORE0_CPU_INT_PRI_22_SPEC > , # [ doc = "0x170 - INTERRUPT_CORE0_CPU_INT_PRI_23" ] pub interrupt_core0_cpu_int_pri_23 : crate :: Reg < interrupt_core0_cpu_int_pri_23 :: INTERRUPT_CORE0_CPU_INT_PRI_23_SPEC > , # [ doc = "0x174 - INTERRUPT_CORE0_CPU_INT_PRI_24" ] pub interrupt_core0_cpu_int_pri_24 : crate :: Reg < interrupt_core0_cpu_int_pri_24 :: INTERRUPT_CORE0_CPU_INT_PRI_24_SPEC > , # [ doc = "0x178 - INTERRUPT_CORE0_CPU_INT_PRI_25" ] pub interrupt_core0_cpu_int_pri_25 : crate :: Reg < interrupt_core0_cpu_int_pri_25 :: INTERRUPT_CORE0_CPU_INT_PRI_25_SPEC > , # [ doc = "0x17c - INTERRUPT_CORE0_CPU_INT_PRI_26" ] pub interrupt_core0_cpu_int_pri_26 : crate :: Reg < interrupt_core0_cpu_int_pri_26 :: INTERRUPT_CORE0_CPU_INT_PRI_26_SPEC > , # [ doc = "0x180 - INTERRUPT_CORE0_CPU_INT_PRI_27" ] pub interrupt_core0_cpu_int_pri_27 : crate :: Reg < interrupt_core0_cpu_int_pri_27 :: INTERRUPT_CORE0_CPU_INT_PRI_27_SPEC > , # [ doc = "0x184 - INTERRUPT_CORE0_CPU_INT_PRI_28" ] pub interrupt_core0_cpu_int_pri_28 : crate :: Reg < interrupt_core0_cpu_int_pri_28 :: INTERRUPT_CORE0_CPU_INT_PRI_28_SPEC > , # [ doc = "0x188 - INTERRUPT_CORE0_CPU_INT_PRI_29" ] pub interrupt_core0_cpu_int_pri_29 : crate :: Reg < interrupt_core0_cpu_int_pri_29 :: INTERRUPT_CORE0_CPU_INT_PRI_29_SPEC > , # [ doc = "0x18c - INTERRUPT_CORE0_CPU_INT_PRI_30" ] pub interrupt_core0_cpu_int_pri_30 : crate :: Reg < interrupt_core0_cpu_int_pri_30 :: INTERRUPT_CORE0_CPU_INT_PRI_30_SPEC > , # [ doc = "0x190 - INTERRUPT_CORE0_CPU_INT_PRI_31" ] pub interrupt_core0_cpu_int_pri_31 : crate :: Reg < interrupt_core0_cpu_int_pri_31 :: INTERRUPT_CORE0_CPU_INT_PRI_31_SPEC > , # [ doc = "0x194 - INTERRUPT_CORE0_CPU_INT_THRESH" ] pub interrupt_core0_cpu_int_thresh : crate :: Reg < interrupt_core0_cpu_int_thresh :: INTERRUPT_CORE0_CPU_INT_THRESH_SPEC > , _reserved102 : [ u8 ; 0x0664 ] , # [ doc = "0x7fc - INTERRUPT_CORE0_INTERRUPT_DATE" ] pub interrupt_core0_interrupt_date : crate :: Reg < interrupt_core0_interrupt_date :: INTERRUPT_CORE0_INTERRUPT_DATE_SPEC > , }
#[doc = "INTERRUPT_CORE0_MAC_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_MAC_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_MAC_INTR_MAP =
    crate::Reg<interrupt_core0_mac_intr_map::INTERRUPT_CORE0_MAC_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_MAC_INTR_MAP"]
pub mod interrupt_core0_mac_intr_map;
#[doc = "INTERRUPT_CORE0_MAC_NMI_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_MAC_NMI_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_MAC_NMI_MAP =
    crate::Reg<interrupt_core0_mac_nmi_map::INTERRUPT_CORE0_MAC_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_MAC_NMI_MAP"]
pub mod interrupt_core0_mac_nmi_map;
#[doc = "INTERRUPT_CORE0_PWR_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_PWR_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_PWR_INTR_MAP =
    crate::Reg<interrupt_core0_pwr_intr_map::INTERRUPT_CORE0_PWR_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_PWR_INTR_MAP"]
pub mod interrupt_core0_pwr_intr_map;
#[doc = "INTERRUPT_CORE0_BB_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_BB_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_BB_INT_MAP =
    crate::Reg<interrupt_core0_bb_int_map::INTERRUPT_CORE0_BB_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BB_INT_MAP"]
pub mod interrupt_core0_bb_int_map;
#[doc = "INTERRUPT_CORE0_BT_MAC_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_BT_MAC_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_BT_MAC_INT_MAP =
    crate::Reg<interrupt_core0_bt_mac_int_map::INTERRUPT_CORE0_BT_MAC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BT_MAC_INT_MAP"]
pub mod interrupt_core0_bt_mac_int_map;
#[doc = "INTERRUPT_CORE0_BT_BB_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_BT_BB_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_BT_BB_INT_MAP =
    crate::Reg<interrupt_core0_bt_bb_int_map::INTERRUPT_CORE0_BT_BB_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BT_BB_INT_MAP"]
pub mod interrupt_core0_bt_bb_int_map;
#[doc = "INTERRUPT_CORE0_BT_BB_NMI_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_BT_BB_NMI_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_BT_BB_NMI_MAP =
    crate::Reg<interrupt_core0_bt_bb_nmi_map::INTERRUPT_CORE0_BT_BB_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_BT_BB_NMI_MAP"]
pub mod interrupt_core0_bt_bb_nmi_map;
#[doc = "INTERRUPT_CORE0_RWBT_IRQ_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RWBT_IRQ_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RWBT_IRQ_MAP =
    crate::Reg<interrupt_core0_rwbt_irq_map::INTERRUPT_CORE0_RWBT_IRQ_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBT_IRQ_MAP"]
pub mod interrupt_core0_rwbt_irq_map;
#[doc = "INTERRUPT_CORE0_RWBLE_IRQ_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RWBLE_IRQ_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RWBLE_IRQ_MAP =
    crate::Reg<interrupt_core0_rwble_irq_map::INTERRUPT_CORE0_RWBLE_IRQ_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBLE_IRQ_MAP"]
pub mod interrupt_core0_rwble_irq_map;
#[doc = "INTERRUPT_CORE0_RWBT_NMI_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RWBT_NMI_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RWBT_NMI_MAP =
    crate::Reg<interrupt_core0_rwbt_nmi_map::INTERRUPT_CORE0_RWBT_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBT_NMI_MAP"]
pub mod interrupt_core0_rwbt_nmi_map;
#[doc = "INTERRUPT_CORE0_RWBLE_NMI_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RWBLE_NMI_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RWBLE_NMI_MAP =
    crate::Reg<interrupt_core0_rwble_nmi_map::INTERRUPT_CORE0_RWBLE_NMI_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RWBLE_NMI_MAP"]
pub mod interrupt_core0_rwble_nmi_map;
#[doc = "INTERRUPT_CORE0_I2C_MST_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_I2C_MST_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_I2C_MST_INT_MAP =
    crate::Reg<interrupt_core0_i2c_mst_int_map::INTERRUPT_CORE0_I2C_MST_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_I2C_MST_INT_MAP"]
pub mod interrupt_core0_i2c_mst_int_map;
#[doc = "INTERRUPT_CORE0_SLC0_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SLC0_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SLC0_INTR_MAP =
    crate::Reg<interrupt_core0_slc0_intr_map::INTERRUPT_CORE0_SLC0_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SLC0_INTR_MAP"]
pub mod interrupt_core0_slc0_intr_map;
#[doc = "INTERRUPT_CORE0_SLC1_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SLC1_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SLC1_INTR_MAP =
    crate::Reg<interrupt_core0_slc1_intr_map::INTERRUPT_CORE0_SLC1_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SLC1_INTR_MAP"]
pub mod interrupt_core0_slc1_intr_map;
#[doc = "INTERRUPT_CORE0_APB_CTRL_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_APB_CTRL_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_APB_CTRL_INTR_MAP =
    crate::Reg<interrupt_core0_apb_ctrl_intr_map::INTERRUPT_CORE0_APB_CTRL_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_APB_CTRL_INTR_MAP"]
pub mod interrupt_core0_apb_ctrl_intr_map;
#[doc = "INTERRUPT_CORE0_UHCI0_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_UHCI0_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_UHCI0_INTR_MAP =
    crate::Reg<interrupt_core0_uhci0_intr_map::INTERRUPT_CORE0_UHCI0_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_UHCI0_INTR_MAP"]
pub mod interrupt_core0_uhci0_intr_map;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP =
    crate::Reg<interrupt_core0_gpio_interrupt_pro_map::INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_MAP"]
pub mod interrupt_core0_gpio_interrupt_pro_map;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP = crate::Reg<
    interrupt_core0_gpio_interrupt_pro_nmi_map::INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_GPIO_INTERRUPT_PRO_NMI_MAP"]
pub mod interrupt_core0_gpio_interrupt_pro_nmi_map;
#[doc = "INTERRUPT_CORE0_SPI_INTR_1_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SPI_INTR_1_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SPI_INTR_1_MAP =
    crate::Reg<interrupt_core0_spi_intr_1_map::INTERRUPT_CORE0_SPI_INTR_1_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SPI_INTR_1_MAP"]
pub mod interrupt_core0_spi_intr_1_map;
#[doc = "INTERRUPT_CORE0_SPI_INTR_2_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SPI_INTR_2_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SPI_INTR_2_MAP =
    crate::Reg<interrupt_core0_spi_intr_2_map::INTERRUPT_CORE0_SPI_INTR_2_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SPI_INTR_2_MAP"]
pub mod interrupt_core0_spi_intr_2_map;
#[doc = "INTERRUPT_CORE0_I2S1_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_I2S1_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_I2S1_INT_MAP =
    crate::Reg<interrupt_core0_i2s1_int_map::INTERRUPT_CORE0_I2S1_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_I2S1_INT_MAP"]
pub mod interrupt_core0_i2s1_int_map;
#[doc = "INTERRUPT_CORE0_UART_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_UART_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_UART_INTR_MAP =
    crate::Reg<interrupt_core0_uart_intr_map::INTERRUPT_CORE0_UART_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_UART_INTR_MAP"]
pub mod interrupt_core0_uart_intr_map;
#[doc = "INTERRUPT_CORE0_UART1_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_UART1_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_UART1_INTR_MAP =
    crate::Reg<interrupt_core0_uart1_intr_map::INTERRUPT_CORE0_UART1_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_UART1_INTR_MAP"]
pub mod interrupt_core0_uart1_intr_map;
#[doc = "INTERRUPT_CORE0_LEDC_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_LEDC_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_LEDC_INT_MAP =
    crate::Reg<interrupt_core0_ledc_int_map::INTERRUPT_CORE0_LEDC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_LEDC_INT_MAP"]
pub mod interrupt_core0_ledc_int_map;
#[doc = "INTERRUPT_CORE0_EFUSE_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_EFUSE_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_EFUSE_INT_MAP =
    crate::Reg<interrupt_core0_efuse_int_map::INTERRUPT_CORE0_EFUSE_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_EFUSE_INT_MAP"]
pub mod interrupt_core0_efuse_int_map;
#[doc = "INTERRUPT_CORE0_CAN_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CAN_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CAN_INT_MAP =
    crate::Reg<interrupt_core0_can_int_map::INTERRUPT_CORE0_CAN_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CAN_INT_MAP"]
pub mod interrupt_core0_can_int_map;
#[doc = "INTERRUPT_CORE0_USB_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_USB_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_USB_INTR_MAP =
    crate::Reg<interrupt_core0_usb_intr_map::INTERRUPT_CORE0_USB_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_USB_INTR_MAP"]
pub mod interrupt_core0_usb_intr_map;
#[doc = "INTERRUPT_CORE0_RTC_CORE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RTC_CORE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RTC_CORE_INTR_MAP =
    crate::Reg<interrupt_core0_rtc_core_intr_map::INTERRUPT_CORE0_RTC_CORE_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RTC_CORE_INTR_MAP"]
pub mod interrupt_core0_rtc_core_intr_map;
#[doc = "INTERRUPT_CORE0_RMT_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RMT_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RMT_INTR_MAP =
    crate::Reg<interrupt_core0_rmt_intr_map::INTERRUPT_CORE0_RMT_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RMT_INTR_MAP"]
pub mod interrupt_core0_rmt_intr_map;
#[doc = "INTERRUPT_CORE0_I2C_EXT0_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_I2C_EXT0_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_I2C_EXT0_INTR_MAP =
    crate::Reg<interrupt_core0_i2c_ext0_intr_map::INTERRUPT_CORE0_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_I2C_EXT0_INTR_MAP"]
pub mod interrupt_core0_i2c_ext0_intr_map;
#[doc = "INTERRUPT_CORE0_TIMER_INT1_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_TIMER_INT1_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_TIMER_INT1_MAP =
    crate::Reg<interrupt_core0_timer_int1_map::INTERRUPT_CORE0_TIMER_INT1_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TIMER_INT1_MAP"]
pub mod interrupt_core0_timer_int1_map;
#[doc = "INTERRUPT_CORE0_TIMER_INT2_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_TIMER_INT2_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_TIMER_INT2_MAP =
    crate::Reg<interrupt_core0_timer_int2_map::INTERRUPT_CORE0_TIMER_INT2_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TIMER_INT2_MAP"]
pub mod interrupt_core0_timer_int2_map;
#[doc = "INTERRUPT_CORE0_TG_T0_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_TG_T0_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_TG_T0_INT_MAP =
    crate::Reg<interrupt_core0_tg_t0_int_map::INTERRUPT_CORE0_TG_T0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG_T0_INT_MAP"]
pub mod interrupt_core0_tg_t0_int_map;
#[doc = "INTERRUPT_CORE0_TG_WDT_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_TG_WDT_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_TG_WDT_INT_MAP =
    crate::Reg<interrupt_core0_tg_wdt_int_map::INTERRUPT_CORE0_TG_WDT_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG_WDT_INT_MAP"]
pub mod interrupt_core0_tg_wdt_int_map;
#[doc = "INTERRUPT_CORE0_TG1_T0_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_TG1_T0_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_TG1_T0_INT_MAP =
    crate::Reg<interrupt_core0_tg1_t0_int_map::INTERRUPT_CORE0_TG1_T0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG1_T0_INT_MAP"]
pub mod interrupt_core0_tg1_t0_int_map;
#[doc = "INTERRUPT_CORE0_TG1_WDT_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_TG1_WDT_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_TG1_WDT_INT_MAP =
    crate::Reg<interrupt_core0_tg1_wdt_int_map::INTERRUPT_CORE0_TG1_WDT_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_TG1_WDT_INT_MAP"]
pub mod interrupt_core0_tg1_wdt_int_map;
#[doc = "INTERRUPT_CORE0_CACHE_IA_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CACHE_IA_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CACHE_IA_INT_MAP =
    crate::Reg<interrupt_core0_cache_ia_int_map::INTERRUPT_CORE0_CACHE_IA_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_CACHE_IA_INT_MAP"]
pub mod interrupt_core0_cache_ia_int_map;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP = crate::Reg<
    interrupt_core0_systimer_target0_int_map::INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET0_INT_MAP"]
pub mod interrupt_core0_systimer_target0_int_map;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP = crate::Reg<
    interrupt_core0_systimer_target1_int_map::INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET1_INT_MAP"]
pub mod interrupt_core0_systimer_target1_int_map;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP = crate::Reg<
    interrupt_core0_systimer_target2_int_map::INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_SYSTIMER_TARGET2_INT_MAP"]
pub mod interrupt_core0_systimer_target2_int_map;
#[doc = "INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP = crate::Reg<
    interrupt_core0_spi_mem_reject_intr_map::INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_SPI_MEM_REJECT_INTR_MAP"]
pub mod interrupt_core0_spi_mem_reject_intr_map;
#[doc = "INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP =
    crate::Reg<interrupt_core0_icache_preload_int_map::INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_ICACHE_PRELOAD_INT_MAP"]
pub mod interrupt_core0_icache_preload_int_map;
#[doc = "INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP =
    crate::Reg<interrupt_core0_icache_sync_int_map::INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_ICACHE_SYNC_INT_MAP"]
pub mod interrupt_core0_icache_sync_int_map;
#[doc = "INTERRUPT_CORE0_APB_ADC_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_APB_ADC_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_APB_ADC_INT_MAP =
    crate::Reg<interrupt_core0_apb_adc_int_map::INTERRUPT_CORE0_APB_ADC_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_APB_ADC_INT_MAP"]
pub mod interrupt_core0_apb_adc_int_map;
#[doc = "INTERRUPT_CORE0_DMA_CH0_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_DMA_CH0_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_DMA_CH0_INT_MAP =
    crate::Reg<interrupt_core0_dma_ch0_int_map::INTERRUPT_CORE0_DMA_CH0_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_DMA_CH0_INT_MAP"]
pub mod interrupt_core0_dma_ch0_int_map;
#[doc = "INTERRUPT_CORE0_DMA_CH1_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_DMA_CH1_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_DMA_CH1_INT_MAP =
    crate::Reg<interrupt_core0_dma_ch1_int_map::INTERRUPT_CORE0_DMA_CH1_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_DMA_CH1_INT_MAP"]
pub mod interrupt_core0_dma_ch1_int_map;
#[doc = "INTERRUPT_CORE0_DMA_CH2_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_DMA_CH2_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_DMA_CH2_INT_MAP =
    crate::Reg<interrupt_core0_dma_ch2_int_map::INTERRUPT_CORE0_DMA_CH2_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_DMA_CH2_INT_MAP"]
pub mod interrupt_core0_dma_ch2_int_map;
#[doc = "INTERRUPT_CORE0_RSA_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_RSA_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_RSA_INT_MAP =
    crate::Reg<interrupt_core0_rsa_int_map::INTERRUPT_CORE0_RSA_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_RSA_INT_MAP"]
pub mod interrupt_core0_rsa_int_map;
#[doc = "INTERRUPT_CORE0_AES_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_AES_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_AES_INT_MAP =
    crate::Reg<interrupt_core0_aes_int_map::INTERRUPT_CORE0_AES_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_AES_INT_MAP"]
pub mod interrupt_core0_aes_int_map;
#[doc = "INTERRUPT_CORE0_SHA_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_SHA_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_SHA_INT_MAP =
    crate::Reg<interrupt_core0_sha_int_map::INTERRUPT_CORE0_SHA_INT_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_SHA_INT_MAP"]
pub mod interrupt_core0_sha_int_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP = crate::Reg<
    interrupt_core0_cpu_intr_from_cpu_0_map::INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_0_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_0_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP = crate::Reg<
    interrupt_core0_cpu_intr_from_cpu_1_map::INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_1_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_1_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP = crate::Reg<
    interrupt_core0_cpu_intr_from_cpu_2_map::INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_2_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_2_map;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP = crate::Reg<
    interrupt_core0_cpu_intr_from_cpu_3_map::INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CPU_INTR_FROM_CPU_3_MAP"]
pub mod interrupt_core0_cpu_intr_from_cpu_3_map;
#[doc = "INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP =
    crate::Reg<interrupt_core0_assist_debug_intr_map::INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP_SPEC>;
#[doc = "INTERRUPT_CORE0_ASSIST_DEBUG_INTR_MAP"]
pub mod interrupt_core0_assist_debug_intr_map;
#[doc = "INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP = crate :: Reg < interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > ;
#[doc = "INTERRUPT_CORE0_DMA_APBPERI_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_dma_apbperi_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate :: Reg < interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > ;
#[doc = "INTERRUPT_CORE0_CORE_0_IRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_core_0_iram0_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP = crate :: Reg < interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > ;
#[doc = "INTERRUPT_CORE0_CORE_0_DRAM0_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_core_0_dram0_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP = crate :: Reg < interrupt_core0_core_0_pif_pms_monitor_violate_intr_map :: INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP_SPEC > ;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_core_0_pif_pms_monitor_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP = crate :: Reg < interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map :: INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP_SPEC > ;
#[doc = "INTERRUPT_CORE0_CORE_0_PIF_PMS_MONITOR_VIOLATE_SIZE_INTR_MAP"]
pub mod interrupt_core0_core_0_pif_pms_monitor_violate_size_intr_map;
#[doc = "INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP = crate::Reg<
    interrupt_core0_backup_pms_violate_intr_map::INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_BACKUP_PMS_VIOLATE_INTR_MAP"]
pub mod interrupt_core0_backup_pms_violate_intr_map;
#[doc = "INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP register accessor: an alias for `Reg<INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP_SPEC>`"]
pub type INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP = crate::Reg<
    interrupt_core0_cache_core0_acs_int_map::INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP_SPEC,
>;
#[doc = "INTERRUPT_CORE0_CACHE_CORE0_ACS_INT_MAP"]
pub mod interrupt_core0_cache_core0_acs_int_map;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_0 register accessor: an alias for `Reg<INTERRUPT_CORE0_INTR_STATUS_0_SPEC>`"]
pub type INTERRUPT_CORE0_INTR_STATUS_0 =
    crate::Reg<interrupt_core0_intr_status_0::INTERRUPT_CORE0_INTR_STATUS_0_SPEC>;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_0"]
pub mod interrupt_core0_intr_status_0;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_1 register accessor: an alias for `Reg<INTERRUPT_CORE0_INTR_STATUS_1_SPEC>`"]
pub type INTERRUPT_CORE0_INTR_STATUS_1 =
    crate::Reg<interrupt_core0_intr_status_1::INTERRUPT_CORE0_INTR_STATUS_1_SPEC>;
#[doc = "INTERRUPT_CORE0_INTR_STATUS_1"]
pub mod interrupt_core0_intr_status_1;
#[doc = "INTERRUPT_CORE0_CLOCK_GATE register accessor: an alias for `Reg<INTERRUPT_CORE0_CLOCK_GATE_SPEC>`"]
pub type INTERRUPT_CORE0_CLOCK_GATE =
    crate::Reg<interrupt_core0_clock_gate::INTERRUPT_CORE0_CLOCK_GATE_SPEC>;
#[doc = "INTERRUPT_CORE0_CLOCK_GATE"]
pub mod interrupt_core0_clock_gate;
#[doc = "INTERRUPT_CORE0_CPU_INT_ENABLE register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_ENABLE_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_ENABLE =
    crate::Reg<interrupt_core0_cpu_int_enable::INTERRUPT_CORE0_CPU_INT_ENABLE_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_ENABLE"]
pub mod interrupt_core0_cpu_int_enable;
#[doc = "INTERRUPT_CORE0_CPU_INT_TYPE register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_TYPE_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_TYPE =
    crate::Reg<interrupt_core0_cpu_int_type::INTERRUPT_CORE0_CPU_INT_TYPE_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_TYPE"]
pub mod interrupt_core0_cpu_int_type;
#[doc = "INTERRUPT_CORE0_CPU_INT_CLEAR register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_CLEAR_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_CLEAR =
    crate::Reg<interrupt_core0_cpu_int_clear::INTERRUPT_CORE0_CPU_INT_CLEAR_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_CLEAR"]
pub mod interrupt_core0_cpu_int_clear;
#[doc = "INTERRUPT_CORE0_CPU_INT_EIP_STATUS register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_EIP_STATUS =
    crate::Reg<interrupt_core0_cpu_int_eip_status::INTERRUPT_CORE0_CPU_INT_EIP_STATUS_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_EIP_STATUS"]
pub mod interrupt_core0_cpu_int_eip_status;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_0 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_0_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_0 =
    crate::Reg<interrupt_core0_cpu_int_pri_0::INTERRUPT_CORE0_CPU_INT_PRI_0_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_0"]
pub mod interrupt_core0_cpu_int_pri_0;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_1 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_1_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_1 =
    crate::Reg<interrupt_core0_cpu_int_pri_1::INTERRUPT_CORE0_CPU_INT_PRI_1_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_1"]
pub mod interrupt_core0_cpu_int_pri_1;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_2 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_2_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_2 =
    crate::Reg<interrupt_core0_cpu_int_pri_2::INTERRUPT_CORE0_CPU_INT_PRI_2_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_2"]
pub mod interrupt_core0_cpu_int_pri_2;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_3 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_3_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_3 =
    crate::Reg<interrupt_core0_cpu_int_pri_3::INTERRUPT_CORE0_CPU_INT_PRI_3_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_3"]
pub mod interrupt_core0_cpu_int_pri_3;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_4 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_4_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_4 =
    crate::Reg<interrupt_core0_cpu_int_pri_4::INTERRUPT_CORE0_CPU_INT_PRI_4_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_4"]
pub mod interrupt_core0_cpu_int_pri_4;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_5 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_5_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_5 =
    crate::Reg<interrupt_core0_cpu_int_pri_5::INTERRUPT_CORE0_CPU_INT_PRI_5_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_5"]
pub mod interrupt_core0_cpu_int_pri_5;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_6 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_6_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_6 =
    crate::Reg<interrupt_core0_cpu_int_pri_6::INTERRUPT_CORE0_CPU_INT_PRI_6_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_6"]
pub mod interrupt_core0_cpu_int_pri_6;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_7 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_7_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_7 =
    crate::Reg<interrupt_core0_cpu_int_pri_7::INTERRUPT_CORE0_CPU_INT_PRI_7_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_7"]
pub mod interrupt_core0_cpu_int_pri_7;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_8 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_8_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_8 =
    crate::Reg<interrupt_core0_cpu_int_pri_8::INTERRUPT_CORE0_CPU_INT_PRI_8_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_8"]
pub mod interrupt_core0_cpu_int_pri_8;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_9 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_9_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_9 =
    crate::Reg<interrupt_core0_cpu_int_pri_9::INTERRUPT_CORE0_CPU_INT_PRI_9_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_9"]
pub mod interrupt_core0_cpu_int_pri_9;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_10 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_10_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_10 =
    crate::Reg<interrupt_core0_cpu_int_pri_10::INTERRUPT_CORE0_CPU_INT_PRI_10_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_10"]
pub mod interrupt_core0_cpu_int_pri_10;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_11 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_11_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_11 =
    crate::Reg<interrupt_core0_cpu_int_pri_11::INTERRUPT_CORE0_CPU_INT_PRI_11_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_11"]
pub mod interrupt_core0_cpu_int_pri_11;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_12 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_12_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_12 =
    crate::Reg<interrupt_core0_cpu_int_pri_12::INTERRUPT_CORE0_CPU_INT_PRI_12_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_12"]
pub mod interrupt_core0_cpu_int_pri_12;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_13 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_13_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_13 =
    crate::Reg<interrupt_core0_cpu_int_pri_13::INTERRUPT_CORE0_CPU_INT_PRI_13_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_13"]
pub mod interrupt_core0_cpu_int_pri_13;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_14 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_14_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_14 =
    crate::Reg<interrupt_core0_cpu_int_pri_14::INTERRUPT_CORE0_CPU_INT_PRI_14_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_14"]
pub mod interrupt_core0_cpu_int_pri_14;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_15 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_15_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_15 =
    crate::Reg<interrupt_core0_cpu_int_pri_15::INTERRUPT_CORE0_CPU_INT_PRI_15_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_15"]
pub mod interrupt_core0_cpu_int_pri_15;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_16 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_16_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_16 =
    crate::Reg<interrupt_core0_cpu_int_pri_16::INTERRUPT_CORE0_CPU_INT_PRI_16_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_16"]
pub mod interrupt_core0_cpu_int_pri_16;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_17 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_17_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_17 =
    crate::Reg<interrupt_core0_cpu_int_pri_17::INTERRUPT_CORE0_CPU_INT_PRI_17_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_17"]
pub mod interrupt_core0_cpu_int_pri_17;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_18 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_18_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_18 =
    crate::Reg<interrupt_core0_cpu_int_pri_18::INTERRUPT_CORE0_CPU_INT_PRI_18_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_18"]
pub mod interrupt_core0_cpu_int_pri_18;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_19 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_19_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_19 =
    crate::Reg<interrupt_core0_cpu_int_pri_19::INTERRUPT_CORE0_CPU_INT_PRI_19_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_19"]
pub mod interrupt_core0_cpu_int_pri_19;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_20 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_20_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_20 =
    crate::Reg<interrupt_core0_cpu_int_pri_20::INTERRUPT_CORE0_CPU_INT_PRI_20_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_20"]
pub mod interrupt_core0_cpu_int_pri_20;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_21 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_21_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_21 =
    crate::Reg<interrupt_core0_cpu_int_pri_21::INTERRUPT_CORE0_CPU_INT_PRI_21_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_21"]
pub mod interrupt_core0_cpu_int_pri_21;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_22 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_22_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_22 =
    crate::Reg<interrupt_core0_cpu_int_pri_22::INTERRUPT_CORE0_CPU_INT_PRI_22_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_22"]
pub mod interrupt_core0_cpu_int_pri_22;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_23 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_23_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_23 =
    crate::Reg<interrupt_core0_cpu_int_pri_23::INTERRUPT_CORE0_CPU_INT_PRI_23_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_23"]
pub mod interrupt_core0_cpu_int_pri_23;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_24 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_24_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_24 =
    crate::Reg<interrupt_core0_cpu_int_pri_24::INTERRUPT_CORE0_CPU_INT_PRI_24_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_24"]
pub mod interrupt_core0_cpu_int_pri_24;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_25 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_25_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_25 =
    crate::Reg<interrupt_core0_cpu_int_pri_25::INTERRUPT_CORE0_CPU_INT_PRI_25_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_25"]
pub mod interrupt_core0_cpu_int_pri_25;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_26 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_26_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_26 =
    crate::Reg<interrupt_core0_cpu_int_pri_26::INTERRUPT_CORE0_CPU_INT_PRI_26_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_26"]
pub mod interrupt_core0_cpu_int_pri_26;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_27 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_27_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_27 =
    crate::Reg<interrupt_core0_cpu_int_pri_27::INTERRUPT_CORE0_CPU_INT_PRI_27_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_27"]
pub mod interrupt_core0_cpu_int_pri_27;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_28 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_28_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_28 =
    crate::Reg<interrupt_core0_cpu_int_pri_28::INTERRUPT_CORE0_CPU_INT_PRI_28_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_28"]
pub mod interrupt_core0_cpu_int_pri_28;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_29 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_29_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_29 =
    crate::Reg<interrupt_core0_cpu_int_pri_29::INTERRUPT_CORE0_CPU_INT_PRI_29_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_29"]
pub mod interrupt_core0_cpu_int_pri_29;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_30 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_30_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_30 =
    crate::Reg<interrupt_core0_cpu_int_pri_30::INTERRUPT_CORE0_CPU_INT_PRI_30_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_30"]
pub mod interrupt_core0_cpu_int_pri_30;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_31 register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_PRI_31_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_PRI_31 =
    crate::Reg<interrupt_core0_cpu_int_pri_31::INTERRUPT_CORE0_CPU_INT_PRI_31_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_PRI_31"]
pub mod interrupt_core0_cpu_int_pri_31;
#[doc = "INTERRUPT_CORE0_CPU_INT_THRESH register accessor: an alias for `Reg<INTERRUPT_CORE0_CPU_INT_THRESH_SPEC>`"]
pub type INTERRUPT_CORE0_CPU_INT_THRESH =
    crate::Reg<interrupt_core0_cpu_int_thresh::INTERRUPT_CORE0_CPU_INT_THRESH_SPEC>;
#[doc = "INTERRUPT_CORE0_CPU_INT_THRESH"]
pub mod interrupt_core0_cpu_int_thresh;
#[doc = "INTERRUPT_CORE0_INTERRUPT_DATE register accessor: an alias for `Reg<INTERRUPT_CORE0_INTERRUPT_DATE_SPEC>`"]
pub type INTERRUPT_CORE0_INTERRUPT_DATE =
    crate::Reg<interrupt_core0_interrupt_date::INTERRUPT_CORE0_INTERRUPT_DATE_SPEC>;
#[doc = "INTERRUPT_CORE0_INTERRUPT_DATE"]
pub mod interrupt_core0_interrupt_date;
