#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EFUSE_PGM_DATA0"]
    pub pgm_data0: crate::Reg<pgm_data0::PGM_DATA0_SPEC>,
    #[doc = "0x04 - EFUSE_PGM_DATA1"]
    pub pgm_data1: crate::Reg<pgm_data1::PGM_DATA1_SPEC>,
    #[doc = "0x08 - EFUSE_PGM_DATA2"]
    pub pgm_data2: crate::Reg<pgm_data2::PGM_DATA2_SPEC>,
    #[doc = "0x0c - EFUSE_PGM_DATA3"]
    pub pgm_data3: crate::Reg<pgm_data3::PGM_DATA3_SPEC>,
    #[doc = "0x10 - EFUSE_PGM_DATA4"]
    pub pgm_data4: crate::Reg<pgm_data4::PGM_DATA4_SPEC>,
    #[doc = "0x14 - EFUSE_PGM_DATA5"]
    pub pgm_data5: crate::Reg<pgm_data5::PGM_DATA5_SPEC>,
    #[doc = "0x18 - EFUSE_PGM_DATA6"]
    pub pgm_data6: crate::Reg<pgm_data6::PGM_DATA6_SPEC>,
    #[doc = "0x1c - EFUSE_PGM_DATA7"]
    pub pgm_data7: crate::Reg<pgm_data7::PGM_DATA7_SPEC>,
    #[doc = "0x20 - EFUSE_PGM_CHECK_VALUE0"]
    pub pgm_check_value0: crate::Reg<pgm_check_value0::PGM_CHECK_VALUE0_SPEC>,
    #[doc = "0x24 - EFUSE_PGM_CHECK_VALUE1"]
    pub pgm_check_value1: crate::Reg<pgm_check_value1::PGM_CHECK_VALUE1_SPEC>,
    #[doc = "0x28 - EFUSE_PGM_CHECK_VALUE2"]
    pub pgm_check_value2: crate::Reg<pgm_check_value2::PGM_CHECK_VALUE2_SPEC>,
    #[doc = "0x2c - EFUSE_RD_WR_DIS"]
    pub rd_wr_dis: crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>,
    #[doc = "0x30 - EFUSE_RD_REPEAT_DATA0"]
    pub rd_repeat_data0: crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>,
    #[doc = "0x34 - EFUSE_RD_REPEAT_DATA1"]
    pub rd_repeat_data1: crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>,
    #[doc = "0x38 - EFUSE_RD_REPEAT_DATA2"]
    pub rd_repeat_data2: crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>,
    #[doc = "0x3c - EFUSE_RD_REPEAT_DATA3"]
    pub rd_repeat_data3: crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>,
    #[doc = "0x40 - EFUSE_RD_REPEAT_DATA4"]
    pub rd_repeat_data4: crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>,
    #[doc = "0x44 - EFUSE_RD_MAC_SPI_SYS_0"]
    pub rd_mac_spi_sys_0: crate::Reg<rd_mac_spi_sys_0::RD_MAC_SPI_SYS_0_SPEC>,
    #[doc = "0x48 - EFUSE_RD_MAC_SPI_SYS_1"]
    pub rd_mac_spi_sys_1: crate::Reg<rd_mac_spi_sys_1::RD_MAC_SPI_SYS_1_SPEC>,
    #[doc = "0x4c - EFUSE_RD_MAC_SPI_SYS_2"]
    pub rd_mac_spi_sys_2: crate::Reg<rd_mac_spi_sys_2::RD_MAC_SPI_SYS_2_SPEC>,
    #[doc = "0x50 - EFUSE_RD_MAC_SPI_SYS_3"]
    pub rd_mac_spi_sys_3: crate::Reg<rd_mac_spi_sys_3::RD_MAC_SPI_SYS_3_SPEC>,
    #[doc = "0x54 - EFUSE_RD_MAC_SPI_SYS_4"]
    pub rd_mac_spi_sys_4: crate::Reg<rd_mac_spi_sys_4::RD_MAC_SPI_SYS_4_SPEC>,
    #[doc = "0x58 - EFUSE_RD_MAC_SPI_SYS_5"]
    pub rd_mac_spi_sys_5: crate::Reg<rd_mac_spi_sys_5::RD_MAC_SPI_SYS_5_SPEC>,
    #[doc = "0x5c - EFUSE_RD_SYS_PART1_DATA0"]
    pub rd_sys_part1_data0: crate::Reg<rd_sys_part1_data0::RD_SYS_PART1_DATA0_SPEC>,
    #[doc = "0x60 - EFUSE_RD_SYS_PART1_DATA1"]
    pub rd_sys_part1_data1: crate::Reg<rd_sys_part1_data1::RD_SYS_PART1_DATA1_SPEC>,
    #[doc = "0x64 - EFUSE_RD_SYS_PART1_DATA2"]
    pub rd_sys_part1_data2: crate::Reg<rd_sys_part1_data2::RD_SYS_PART1_DATA2_SPEC>,
    #[doc = "0x68 - EFUSE_RD_SYS_PART1_DATA3"]
    pub rd_sys_part1_data3: crate::Reg<rd_sys_part1_data3::RD_SYS_PART1_DATA3_SPEC>,
    #[doc = "0x6c - EFUSE_RD_SYS_PART1_DATA4"]
    pub rd_sys_part1_data4: crate::Reg<rd_sys_part1_data4::RD_SYS_PART1_DATA4_SPEC>,
    #[doc = "0x70 - EFUSE_RD_SYS_PART1_DATA5"]
    pub rd_sys_part1_data5: crate::Reg<rd_sys_part1_data5::RD_SYS_PART1_DATA5_SPEC>,
    #[doc = "0x74 - EFUSE_RD_SYS_PART1_DATA6"]
    pub rd_sys_part1_data6: crate::Reg<rd_sys_part1_data6::RD_SYS_PART1_DATA6_SPEC>,
    #[doc = "0x78 - EFUSE_RD_SYS_PART1_DATA7"]
    pub rd_sys_part1_data7: crate::Reg<rd_sys_part1_data7::RD_SYS_PART1_DATA7_SPEC>,
    #[doc = "0x7c - EFUSE_RD_USR_DATA0"]
    pub rd_usr_data0: crate::Reg<rd_usr_data0::RD_USR_DATA0_SPEC>,
    #[doc = "0x80 - EFUSE_RD_USR_DATA1"]
    pub rd_usr_data1: crate::Reg<rd_usr_data1::RD_USR_DATA1_SPEC>,
    #[doc = "0x84 - EFUSE_RD_USR_DATA2"]
    pub rd_usr_data2: crate::Reg<rd_usr_data2::RD_USR_DATA2_SPEC>,
    #[doc = "0x88 - EFUSE_RD_USR_DATA3"]
    pub rd_usr_data3: crate::Reg<rd_usr_data3::RD_USR_DATA3_SPEC>,
    #[doc = "0x8c - EFUSE_RD_USR_DATA4"]
    pub rd_usr_data4: crate::Reg<rd_usr_data4::RD_USR_DATA4_SPEC>,
    #[doc = "0x90 - EFUSE_RD_USR_DATA5"]
    pub rd_usr_data5: crate::Reg<rd_usr_data5::RD_USR_DATA5_SPEC>,
    #[doc = "0x94 - EFUSE_RD_USR_DATA6"]
    pub rd_usr_data6: crate::Reg<rd_usr_data6::RD_USR_DATA6_SPEC>,
    #[doc = "0x98 - EFUSE_RD_USR_DATA7"]
    pub rd_usr_data7: crate::Reg<rd_usr_data7::RD_USR_DATA7_SPEC>,
    #[doc = "0x9c - EFUSE_RD_KEY0_DATA0"]
    pub rd_key0_data0: crate::Reg<rd_key0_data0::RD_KEY0_DATA0_SPEC>,
    #[doc = "0xa0 - EFUSE_RD_KEY0_DATA1"]
    pub rd_key0_data1: crate::Reg<rd_key0_data1::RD_KEY0_DATA1_SPEC>,
    #[doc = "0xa4 - EFUSE_RD_KEY0_DATA2"]
    pub rd_key0_data2: crate::Reg<rd_key0_data2::RD_KEY0_DATA2_SPEC>,
    #[doc = "0xa8 - EFUSE_RD_KEY0_DATA3"]
    pub rd_key0_data3: crate::Reg<rd_key0_data3::RD_KEY0_DATA3_SPEC>,
    #[doc = "0xac - EFUSE_RD_KEY0_DATA4"]
    pub rd_key0_data4: crate::Reg<rd_key0_data4::RD_KEY0_DATA4_SPEC>,
    #[doc = "0xb0 - EFUSE_RD_KEY0_DATA5"]
    pub rd_key0_data5: crate::Reg<rd_key0_data5::RD_KEY0_DATA5_SPEC>,
    #[doc = "0xb4 - EFUSE_RD_KEY0_DATA6"]
    pub rd_key0_data6: crate::Reg<rd_key0_data6::RD_KEY0_DATA6_SPEC>,
    #[doc = "0xb8 - EFUSE_RD_KEY0_DATA7"]
    pub rd_key0_data7: crate::Reg<rd_key0_data7::RD_KEY0_DATA7_SPEC>,
    #[doc = "0xbc - EFUSE_RD_KEY1_DATA0"]
    pub rd_key1_data0: crate::Reg<rd_key1_data0::RD_KEY1_DATA0_SPEC>,
    #[doc = "0xc0 - EFUSE_RD_KEY1_DATA1"]
    pub rd_key1_data1: crate::Reg<rd_key1_data1::RD_KEY1_DATA1_SPEC>,
    #[doc = "0xc4 - EFUSE_RD_KEY1_DATA2"]
    pub rd_key1_data2: crate::Reg<rd_key1_data2::RD_KEY1_DATA2_SPEC>,
    #[doc = "0xc8 - EFUSE_RD_KEY1_DATA3"]
    pub rd_key1_data3: crate::Reg<rd_key1_data3::RD_KEY1_DATA3_SPEC>,
    #[doc = "0xcc - EFUSE_RD_KEY1_DATA4"]
    pub rd_key1_data4: crate::Reg<rd_key1_data4::RD_KEY1_DATA4_SPEC>,
    #[doc = "0xd0 - EFUSE_RD_KEY1_DATA5"]
    pub rd_key1_data5: crate::Reg<rd_key1_data5::RD_KEY1_DATA5_SPEC>,
    #[doc = "0xd4 - EFUSE_RD_KEY1_DATA6"]
    pub rd_key1_data6: crate::Reg<rd_key1_data6::RD_KEY1_DATA6_SPEC>,
    #[doc = "0xd8 - EFUSE_RD_KEY1_DATA7"]
    pub rd_key1_data7: crate::Reg<rd_key1_data7::RD_KEY1_DATA7_SPEC>,
    #[doc = "0xdc - EFUSE_RD_KEY2_DATA0"]
    pub rd_key2_data0: crate::Reg<rd_key2_data0::RD_KEY2_DATA0_SPEC>,
    #[doc = "0xe0 - EFUSE_RD_KEY2_DATA1"]
    pub rd_key2_data1: crate::Reg<rd_key2_data1::RD_KEY2_DATA1_SPEC>,
    #[doc = "0xe4 - EFUSE_RD_KEY2_DATA2"]
    pub rd_key2_data2: crate::Reg<rd_key2_data2::RD_KEY2_DATA2_SPEC>,
    #[doc = "0xe8 - EFUSE_RD_KEY2_DATA3"]
    pub rd_key2_data3: crate::Reg<rd_key2_data3::RD_KEY2_DATA3_SPEC>,
    #[doc = "0xec - EFUSE_RD_KEY2_DATA4"]
    pub rd_key2_data4: crate::Reg<rd_key2_data4::RD_KEY2_DATA4_SPEC>,
    #[doc = "0xf0 - EFUSE_RD_KEY2_DATA5"]
    pub rd_key2_data5: crate::Reg<rd_key2_data5::RD_KEY2_DATA5_SPEC>,
    #[doc = "0xf4 - EFUSE_RD_KEY2_DATA6"]
    pub rd_key2_data6: crate::Reg<rd_key2_data6::RD_KEY2_DATA6_SPEC>,
    #[doc = "0xf8 - EFUSE_RD_KEY2_DATA7"]
    pub rd_key2_data7: crate::Reg<rd_key2_data7::RD_KEY2_DATA7_SPEC>,
    #[doc = "0xfc - EFUSE_RD_KEY3_DATA0"]
    pub rd_key3_data0: crate::Reg<rd_key3_data0::RD_KEY3_DATA0_SPEC>,
    #[doc = "0x100 - EFUSE_RD_KEY3_DATA1"]
    pub rd_key3_data1: crate::Reg<rd_key3_data1::RD_KEY3_DATA1_SPEC>,
    #[doc = "0x104 - EFUSE_RD_KEY3_DATA2"]
    pub rd_key3_data2: crate::Reg<rd_key3_data2::RD_KEY3_DATA2_SPEC>,
    #[doc = "0x108 - EFUSE_RD_KEY3_DATA3"]
    pub rd_key3_data3: crate::Reg<rd_key3_data3::RD_KEY3_DATA3_SPEC>,
    #[doc = "0x10c - EFUSE_RD_KEY3_DATA4"]
    pub rd_key3_data4: crate::Reg<rd_key3_data4::RD_KEY3_DATA4_SPEC>,
    #[doc = "0x110 - EFUSE_RD_KEY3_DATA5"]
    pub rd_key3_data5: crate::Reg<rd_key3_data5::RD_KEY3_DATA5_SPEC>,
    #[doc = "0x114 - EFUSE_RD_KEY3_DATA6"]
    pub rd_key3_data6: crate::Reg<rd_key3_data6::RD_KEY3_DATA6_SPEC>,
    #[doc = "0x118 - EFUSE_RD_KEY3_DATA7"]
    pub rd_key3_data7: crate::Reg<rd_key3_data7::RD_KEY3_DATA7_SPEC>,
    #[doc = "0x11c - EFUSE_RD_KEY4_DATA0"]
    pub rd_key4_data0: crate::Reg<rd_key4_data0::RD_KEY4_DATA0_SPEC>,
    #[doc = "0x120 - EFUSE_RD_KEY4_DATA1"]
    pub rd_key4_data1: crate::Reg<rd_key4_data1::RD_KEY4_DATA1_SPEC>,
    #[doc = "0x124 - EFUSE_RD_KEY4_DATA2"]
    pub rd_key4_data2: crate::Reg<rd_key4_data2::RD_KEY4_DATA2_SPEC>,
    #[doc = "0x128 - EFUSE_RD_KEY4_DATA3"]
    pub rd_key4_data3: crate::Reg<rd_key4_data3::RD_KEY4_DATA3_SPEC>,
    #[doc = "0x12c - EFUSE_RD_KEY4_DATA4"]
    pub rd_key4_data4: crate::Reg<rd_key4_data4::RD_KEY4_DATA4_SPEC>,
    #[doc = "0x130 - EFUSE_RD_KEY4_DATA5"]
    pub rd_key4_data5: crate::Reg<rd_key4_data5::RD_KEY4_DATA5_SPEC>,
    #[doc = "0x134 - EFUSE_RD_KEY4_DATA6"]
    pub rd_key4_data6: crate::Reg<rd_key4_data6::RD_KEY4_DATA6_SPEC>,
    #[doc = "0x138 - EFUSE_RD_KEY4_DATA7"]
    pub rd_key4_data7: crate::Reg<rd_key4_data7::RD_KEY4_DATA7_SPEC>,
    #[doc = "0x13c - EFUSE_RD_KEY5_DATA0"]
    pub rd_key5_data0: crate::Reg<rd_key5_data0::RD_KEY5_DATA0_SPEC>,
    #[doc = "0x140 - EFUSE_RD_KEY5_DATA1"]
    pub rd_key5_data1: crate::Reg<rd_key5_data1::RD_KEY5_DATA1_SPEC>,
    #[doc = "0x144 - EFUSE_RD_KEY5_DATA2"]
    pub rd_key5_data2: crate::Reg<rd_key5_data2::RD_KEY5_DATA2_SPEC>,
    #[doc = "0x148 - EFUSE_RD_KEY5_DATA3"]
    pub rd_key5_data3: crate::Reg<rd_key5_data3::RD_KEY5_DATA3_SPEC>,
    #[doc = "0x14c - EFUSE_RD_KEY5_DATA4"]
    pub rd_key5_data4: crate::Reg<rd_key5_data4::RD_KEY5_DATA4_SPEC>,
    #[doc = "0x150 - EFUSE_RD_KEY5_DATA5"]
    pub rd_key5_data5: crate::Reg<rd_key5_data5::RD_KEY5_DATA5_SPEC>,
    #[doc = "0x154 - EFUSE_RD_KEY5_DATA6"]
    pub rd_key5_data6: crate::Reg<rd_key5_data6::RD_KEY5_DATA6_SPEC>,
    #[doc = "0x158 - EFUSE_RD_KEY5_DATA7"]
    pub rd_key5_data7: crate::Reg<rd_key5_data7::RD_KEY5_DATA7_SPEC>,
    #[doc = "0x15c - EFUSE_RD_SYS_PART2_DATA0"]
    pub rd_sys_part2_data0: crate::Reg<rd_sys_part2_data0::RD_SYS_PART2_DATA0_SPEC>,
    #[doc = "0x160 - EFUSE_RD_SYS_PART2_DATA1"]
    pub rd_sys_part2_data1: crate::Reg<rd_sys_part2_data1::RD_SYS_PART2_DATA1_SPEC>,
    #[doc = "0x164 - EFUSE_RD_SYS_PART2_DATA2"]
    pub rd_sys_part2_data2: crate::Reg<rd_sys_part2_data2::RD_SYS_PART2_DATA2_SPEC>,
    #[doc = "0x168 - EFUSE_RD_SYS_PART2_DATA3"]
    pub rd_sys_part2_data3: crate::Reg<rd_sys_part2_data3::RD_SYS_PART2_DATA3_SPEC>,
    #[doc = "0x16c - EFUSE_RD_SYS_PART2_DATA4"]
    pub rd_sys_part2_data4: crate::Reg<rd_sys_part2_data4::RD_SYS_PART2_DATA4_SPEC>,
    #[doc = "0x170 - EFUSE_RD_SYS_PART2_DATA5"]
    pub rd_sys_part2_data5: crate::Reg<rd_sys_part2_data5::RD_SYS_PART2_DATA5_SPEC>,
    #[doc = "0x174 - EFUSE_RD_SYS_PART2_DATA6"]
    pub rd_sys_part2_data6: crate::Reg<rd_sys_part2_data6::RD_SYS_PART2_DATA6_SPEC>,
    #[doc = "0x178 - EFUSE_RD_SYS_PART2_DATA7"]
    pub rd_sys_part2_data7: crate::Reg<rd_sys_part2_data7::RD_SYS_PART2_DATA7_SPEC>,
    #[doc = "0x17c - EFUSE_RD_REPEAT_ERR0"]
    pub rd_repeat_err0: crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>,
    #[doc = "0x180 - EFUSE_RD_REPEAT_ERR1"]
    pub rd_repeat_err1: crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>,
    #[doc = "0x184 - EFUSE_RD_REPEAT_ERR2"]
    pub rd_repeat_err2: crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>,
    #[doc = "0x188 - EFUSE_RD_REPEAT_ERR3"]
    pub rd_repeat_err3: crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>,
    _reserved99: [u8; 0x04],
    #[doc = "0x190 - EFUSE_RD_REPEAT_ERR4"]
    pub rd_repeat_err4: crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>,
    _reserved100: [u8; 0x2c],
    #[doc = "0x1c0 - EFUSE_RD_RS_ERR0"]
    pub rd_rs_err0: crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>,
    #[doc = "0x1c4 - EFUSE_RD_RS_ERR1"]
    pub rd_rs_err1: crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>,
    #[doc = "0x1c8 - EFUSE_CLK"]
    pub clk: crate::Reg<clk::CLK_SPEC>,
    #[doc = "0x1cc - EFUSE_CONF"]
    pub conf: crate::Reg<conf::CONF_SPEC>,
    #[doc = "0x1d0 - EFUSE_STATUS"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1d4 - EFUSE_CMD"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x1d8 - EFUSE_INT_RAW"]
    pub int_raw: crate::Reg<int_raw::INT_RAW_SPEC>,
    #[doc = "0x1dc - EFUSE_INT_ST"]
    pub int_st: crate::Reg<int_st::INT_ST_SPEC>,
    #[doc = "0x1e0 - EFUSE_INT_ENA"]
    pub int_ena: crate::Reg<int_ena::INT_ENA_SPEC>,
    #[doc = "0x1e4 - EFUSE_INT_CLR"]
    pub int_clr: crate::Reg<int_clr::INT_CLR_SPEC>,
    #[doc = "0x1e8 - EFUSE_DAC_CONF"]
    pub dac_conf: crate::Reg<dac_conf::DAC_CONF_SPEC>,
    #[doc = "0x1ec - EFUSE_RD_TIM_CONF"]
    pub rd_tim_conf: crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>,
    #[doc = "0x1f0 - EFUSE_WR_TIM_CONF1"]
    pub wr_tim_conf1: crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>,
    #[doc = "0x1f4 - EFUSE_WR_TIM_CONF2"]
    pub wr_tim_conf2: crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>,
    _reserved114: [u8; 0x04],
    #[doc = "0x1fc - EFUSE_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "PGM_DATA0 register accessor: an alias for `Reg<PGM_DATA0_SPEC>`"]
pub type PGM_DATA0 = crate::Reg<pgm_data0::PGM_DATA0_SPEC>;
#[doc = "EFUSE_PGM_DATA0"]
pub mod pgm_data0;
#[doc = "PGM_DATA1 register accessor: an alias for `Reg<PGM_DATA1_SPEC>`"]
pub type PGM_DATA1 = crate::Reg<pgm_data1::PGM_DATA1_SPEC>;
#[doc = "EFUSE_PGM_DATA1"]
pub mod pgm_data1;
#[doc = "PGM_DATA2 register accessor: an alias for `Reg<PGM_DATA2_SPEC>`"]
pub type PGM_DATA2 = crate::Reg<pgm_data2::PGM_DATA2_SPEC>;
#[doc = "EFUSE_PGM_DATA2"]
pub mod pgm_data2;
#[doc = "PGM_DATA3 register accessor: an alias for `Reg<PGM_DATA3_SPEC>`"]
pub type PGM_DATA3 = crate::Reg<pgm_data3::PGM_DATA3_SPEC>;
#[doc = "EFUSE_PGM_DATA3"]
pub mod pgm_data3;
#[doc = "PGM_DATA4 register accessor: an alias for `Reg<PGM_DATA4_SPEC>`"]
pub type PGM_DATA4 = crate::Reg<pgm_data4::PGM_DATA4_SPEC>;
#[doc = "EFUSE_PGM_DATA4"]
pub mod pgm_data4;
#[doc = "PGM_DATA5 register accessor: an alias for `Reg<PGM_DATA5_SPEC>`"]
pub type PGM_DATA5 = crate::Reg<pgm_data5::PGM_DATA5_SPEC>;
#[doc = "EFUSE_PGM_DATA5"]
pub mod pgm_data5;
#[doc = "PGM_DATA6 register accessor: an alias for `Reg<PGM_DATA6_SPEC>`"]
pub type PGM_DATA6 = crate::Reg<pgm_data6::PGM_DATA6_SPEC>;
#[doc = "EFUSE_PGM_DATA6"]
pub mod pgm_data6;
#[doc = "PGM_DATA7 register accessor: an alias for `Reg<PGM_DATA7_SPEC>`"]
pub type PGM_DATA7 = crate::Reg<pgm_data7::PGM_DATA7_SPEC>;
#[doc = "EFUSE_PGM_DATA7"]
pub mod pgm_data7;
#[doc = "PGM_CHECK_VALUE0 register accessor: an alias for `Reg<PGM_CHECK_VALUE0_SPEC>`"]
pub type PGM_CHECK_VALUE0 = crate::Reg<pgm_check_value0::PGM_CHECK_VALUE0_SPEC>;
#[doc = "EFUSE_PGM_CHECK_VALUE0"]
pub mod pgm_check_value0;
#[doc = "PGM_CHECK_VALUE1 register accessor: an alias for `Reg<PGM_CHECK_VALUE1_SPEC>`"]
pub type PGM_CHECK_VALUE1 = crate::Reg<pgm_check_value1::PGM_CHECK_VALUE1_SPEC>;
#[doc = "EFUSE_PGM_CHECK_VALUE1"]
pub mod pgm_check_value1;
#[doc = "PGM_CHECK_VALUE2 register accessor: an alias for `Reg<PGM_CHECK_VALUE2_SPEC>`"]
pub type PGM_CHECK_VALUE2 = crate::Reg<pgm_check_value2::PGM_CHECK_VALUE2_SPEC>;
#[doc = "EFUSE_PGM_CHECK_VALUE2"]
pub mod pgm_check_value2;
#[doc = "RD_WR_DIS register accessor: an alias for `Reg<RD_WR_DIS_SPEC>`"]
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
#[doc = "EFUSE_RD_WR_DIS"]
pub mod rd_wr_dis;
#[doc = "RD_REPEAT_DATA0 register accessor: an alias for `Reg<RD_REPEAT_DATA0_SPEC>`"]
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
#[doc = "EFUSE_RD_REPEAT_DATA0"]
pub mod rd_repeat_data0;
#[doc = "RD_REPEAT_DATA1 register accessor: an alias for `Reg<RD_REPEAT_DATA1_SPEC>`"]
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
#[doc = "EFUSE_RD_REPEAT_DATA1"]
pub mod rd_repeat_data1;
#[doc = "RD_REPEAT_DATA2 register accessor: an alias for `Reg<RD_REPEAT_DATA2_SPEC>`"]
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
#[doc = "EFUSE_RD_REPEAT_DATA2"]
pub mod rd_repeat_data2;
#[doc = "RD_REPEAT_DATA3 register accessor: an alias for `Reg<RD_REPEAT_DATA3_SPEC>`"]
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
#[doc = "EFUSE_RD_REPEAT_DATA3"]
pub mod rd_repeat_data3;
#[doc = "RD_REPEAT_DATA4 register accessor: an alias for `Reg<RD_REPEAT_DATA4_SPEC>`"]
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
#[doc = "EFUSE_RD_REPEAT_DATA4"]
pub mod rd_repeat_data4;
#[doc = "RD_MAC_SPI_SYS_0 register accessor: an alias for `Reg<RD_MAC_SPI_SYS_0_SPEC>`"]
pub type RD_MAC_SPI_SYS_0 = crate::Reg<rd_mac_spi_sys_0::RD_MAC_SPI_SYS_0_SPEC>;
#[doc = "EFUSE_RD_MAC_SPI_SYS_0"]
pub mod rd_mac_spi_sys_0;
#[doc = "RD_MAC_SPI_SYS_1 register accessor: an alias for `Reg<RD_MAC_SPI_SYS_1_SPEC>`"]
pub type RD_MAC_SPI_SYS_1 = crate::Reg<rd_mac_spi_sys_1::RD_MAC_SPI_SYS_1_SPEC>;
#[doc = "EFUSE_RD_MAC_SPI_SYS_1"]
pub mod rd_mac_spi_sys_1;
#[doc = "RD_MAC_SPI_SYS_2 register accessor: an alias for `Reg<RD_MAC_SPI_SYS_2_SPEC>`"]
pub type RD_MAC_SPI_SYS_2 = crate::Reg<rd_mac_spi_sys_2::RD_MAC_SPI_SYS_2_SPEC>;
#[doc = "EFUSE_RD_MAC_SPI_SYS_2"]
pub mod rd_mac_spi_sys_2;
#[doc = "RD_MAC_SPI_SYS_3 register accessor: an alias for `Reg<RD_MAC_SPI_SYS_3_SPEC>`"]
pub type RD_MAC_SPI_SYS_3 = crate::Reg<rd_mac_spi_sys_3::RD_MAC_SPI_SYS_3_SPEC>;
#[doc = "EFUSE_RD_MAC_SPI_SYS_3"]
pub mod rd_mac_spi_sys_3;
#[doc = "RD_MAC_SPI_SYS_4 register accessor: an alias for `Reg<RD_MAC_SPI_SYS_4_SPEC>`"]
pub type RD_MAC_SPI_SYS_4 = crate::Reg<rd_mac_spi_sys_4::RD_MAC_SPI_SYS_4_SPEC>;
#[doc = "EFUSE_RD_MAC_SPI_SYS_4"]
pub mod rd_mac_spi_sys_4;
#[doc = "RD_MAC_SPI_SYS_5 register accessor: an alias for `Reg<RD_MAC_SPI_SYS_5_SPEC>`"]
pub type RD_MAC_SPI_SYS_5 = crate::Reg<rd_mac_spi_sys_5::RD_MAC_SPI_SYS_5_SPEC>;
#[doc = "EFUSE_RD_MAC_SPI_SYS_5"]
pub mod rd_mac_spi_sys_5;
#[doc = "RD_SYS_PART1_DATA0 register accessor: an alias for `Reg<RD_SYS_PART1_DATA0_SPEC>`"]
pub type RD_SYS_PART1_DATA0 = crate::Reg<rd_sys_part1_data0::RD_SYS_PART1_DATA0_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA0"]
pub mod rd_sys_part1_data0;
#[doc = "RD_SYS_PART1_DATA1 register accessor: an alias for `Reg<RD_SYS_PART1_DATA1_SPEC>`"]
pub type RD_SYS_PART1_DATA1 = crate::Reg<rd_sys_part1_data1::RD_SYS_PART1_DATA1_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA1"]
pub mod rd_sys_part1_data1;
#[doc = "RD_SYS_PART1_DATA2 register accessor: an alias for `Reg<RD_SYS_PART1_DATA2_SPEC>`"]
pub type RD_SYS_PART1_DATA2 = crate::Reg<rd_sys_part1_data2::RD_SYS_PART1_DATA2_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA2"]
pub mod rd_sys_part1_data2;
#[doc = "RD_SYS_PART1_DATA3 register accessor: an alias for `Reg<RD_SYS_PART1_DATA3_SPEC>`"]
pub type RD_SYS_PART1_DATA3 = crate::Reg<rd_sys_part1_data3::RD_SYS_PART1_DATA3_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA3"]
pub mod rd_sys_part1_data3;
#[doc = "RD_SYS_PART1_DATA4 register accessor: an alias for `Reg<RD_SYS_PART1_DATA4_SPEC>`"]
pub type RD_SYS_PART1_DATA4 = crate::Reg<rd_sys_part1_data4::RD_SYS_PART1_DATA4_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA4"]
pub mod rd_sys_part1_data4;
#[doc = "RD_SYS_PART1_DATA5 register accessor: an alias for `Reg<RD_SYS_PART1_DATA5_SPEC>`"]
pub type RD_SYS_PART1_DATA5 = crate::Reg<rd_sys_part1_data5::RD_SYS_PART1_DATA5_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA5"]
pub mod rd_sys_part1_data5;
#[doc = "RD_SYS_PART1_DATA6 register accessor: an alias for `Reg<RD_SYS_PART1_DATA6_SPEC>`"]
pub type RD_SYS_PART1_DATA6 = crate::Reg<rd_sys_part1_data6::RD_SYS_PART1_DATA6_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA6"]
pub mod rd_sys_part1_data6;
#[doc = "RD_SYS_PART1_DATA7 register accessor: an alias for `Reg<RD_SYS_PART1_DATA7_SPEC>`"]
pub type RD_SYS_PART1_DATA7 = crate::Reg<rd_sys_part1_data7::RD_SYS_PART1_DATA7_SPEC>;
#[doc = "EFUSE_RD_SYS_PART1_DATA7"]
pub mod rd_sys_part1_data7;
#[doc = "RD_USR_DATA0 register accessor: an alias for `Reg<RD_USR_DATA0_SPEC>`"]
pub type RD_USR_DATA0 = crate::Reg<rd_usr_data0::RD_USR_DATA0_SPEC>;
#[doc = "EFUSE_RD_USR_DATA0"]
pub mod rd_usr_data0;
#[doc = "RD_USR_DATA1 register accessor: an alias for `Reg<RD_USR_DATA1_SPEC>`"]
pub type RD_USR_DATA1 = crate::Reg<rd_usr_data1::RD_USR_DATA1_SPEC>;
#[doc = "EFUSE_RD_USR_DATA1"]
pub mod rd_usr_data1;
#[doc = "RD_USR_DATA2 register accessor: an alias for `Reg<RD_USR_DATA2_SPEC>`"]
pub type RD_USR_DATA2 = crate::Reg<rd_usr_data2::RD_USR_DATA2_SPEC>;
#[doc = "EFUSE_RD_USR_DATA2"]
pub mod rd_usr_data2;
#[doc = "RD_USR_DATA3 register accessor: an alias for `Reg<RD_USR_DATA3_SPEC>`"]
pub type RD_USR_DATA3 = crate::Reg<rd_usr_data3::RD_USR_DATA3_SPEC>;
#[doc = "EFUSE_RD_USR_DATA3"]
pub mod rd_usr_data3;
#[doc = "RD_USR_DATA4 register accessor: an alias for `Reg<RD_USR_DATA4_SPEC>`"]
pub type RD_USR_DATA4 = crate::Reg<rd_usr_data4::RD_USR_DATA4_SPEC>;
#[doc = "EFUSE_RD_USR_DATA4"]
pub mod rd_usr_data4;
#[doc = "RD_USR_DATA5 register accessor: an alias for `Reg<RD_USR_DATA5_SPEC>`"]
pub type RD_USR_DATA5 = crate::Reg<rd_usr_data5::RD_USR_DATA5_SPEC>;
#[doc = "EFUSE_RD_USR_DATA5"]
pub mod rd_usr_data5;
#[doc = "RD_USR_DATA6 register accessor: an alias for `Reg<RD_USR_DATA6_SPEC>`"]
pub type RD_USR_DATA6 = crate::Reg<rd_usr_data6::RD_USR_DATA6_SPEC>;
#[doc = "EFUSE_RD_USR_DATA6"]
pub mod rd_usr_data6;
#[doc = "RD_USR_DATA7 register accessor: an alias for `Reg<RD_USR_DATA7_SPEC>`"]
pub type RD_USR_DATA7 = crate::Reg<rd_usr_data7::RD_USR_DATA7_SPEC>;
#[doc = "EFUSE_RD_USR_DATA7"]
pub mod rd_usr_data7;
#[doc = "RD_KEY0_DATA0 register accessor: an alias for `Reg<RD_KEY0_DATA0_SPEC>`"]
pub type RD_KEY0_DATA0 = crate::Reg<rd_key0_data0::RD_KEY0_DATA0_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA0"]
pub mod rd_key0_data0;
#[doc = "RD_KEY0_DATA1 register accessor: an alias for `Reg<RD_KEY0_DATA1_SPEC>`"]
pub type RD_KEY0_DATA1 = crate::Reg<rd_key0_data1::RD_KEY0_DATA1_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA1"]
pub mod rd_key0_data1;
#[doc = "RD_KEY0_DATA2 register accessor: an alias for `Reg<RD_KEY0_DATA2_SPEC>`"]
pub type RD_KEY0_DATA2 = crate::Reg<rd_key0_data2::RD_KEY0_DATA2_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA2"]
pub mod rd_key0_data2;
#[doc = "RD_KEY0_DATA3 register accessor: an alias for `Reg<RD_KEY0_DATA3_SPEC>`"]
pub type RD_KEY0_DATA3 = crate::Reg<rd_key0_data3::RD_KEY0_DATA3_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA3"]
pub mod rd_key0_data3;
#[doc = "RD_KEY0_DATA4 register accessor: an alias for `Reg<RD_KEY0_DATA4_SPEC>`"]
pub type RD_KEY0_DATA4 = crate::Reg<rd_key0_data4::RD_KEY0_DATA4_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA4"]
pub mod rd_key0_data4;
#[doc = "RD_KEY0_DATA5 register accessor: an alias for `Reg<RD_KEY0_DATA5_SPEC>`"]
pub type RD_KEY0_DATA5 = crate::Reg<rd_key0_data5::RD_KEY0_DATA5_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA5"]
pub mod rd_key0_data5;
#[doc = "RD_KEY0_DATA6 register accessor: an alias for `Reg<RD_KEY0_DATA6_SPEC>`"]
pub type RD_KEY0_DATA6 = crate::Reg<rd_key0_data6::RD_KEY0_DATA6_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA6"]
pub mod rd_key0_data6;
#[doc = "RD_KEY0_DATA7 register accessor: an alias for `Reg<RD_KEY0_DATA7_SPEC>`"]
pub type RD_KEY0_DATA7 = crate::Reg<rd_key0_data7::RD_KEY0_DATA7_SPEC>;
#[doc = "EFUSE_RD_KEY0_DATA7"]
pub mod rd_key0_data7;
#[doc = "RD_KEY1_DATA0 register accessor: an alias for `Reg<RD_KEY1_DATA0_SPEC>`"]
pub type RD_KEY1_DATA0 = crate::Reg<rd_key1_data0::RD_KEY1_DATA0_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA0"]
pub mod rd_key1_data0;
#[doc = "RD_KEY1_DATA1 register accessor: an alias for `Reg<RD_KEY1_DATA1_SPEC>`"]
pub type RD_KEY1_DATA1 = crate::Reg<rd_key1_data1::RD_KEY1_DATA1_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA1"]
pub mod rd_key1_data1;
#[doc = "RD_KEY1_DATA2 register accessor: an alias for `Reg<RD_KEY1_DATA2_SPEC>`"]
pub type RD_KEY1_DATA2 = crate::Reg<rd_key1_data2::RD_KEY1_DATA2_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA2"]
pub mod rd_key1_data2;
#[doc = "RD_KEY1_DATA3 register accessor: an alias for `Reg<RD_KEY1_DATA3_SPEC>`"]
pub type RD_KEY1_DATA3 = crate::Reg<rd_key1_data3::RD_KEY1_DATA3_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA3"]
pub mod rd_key1_data3;
#[doc = "RD_KEY1_DATA4 register accessor: an alias for `Reg<RD_KEY1_DATA4_SPEC>`"]
pub type RD_KEY1_DATA4 = crate::Reg<rd_key1_data4::RD_KEY1_DATA4_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA4"]
pub mod rd_key1_data4;
#[doc = "RD_KEY1_DATA5 register accessor: an alias for `Reg<RD_KEY1_DATA5_SPEC>`"]
pub type RD_KEY1_DATA5 = crate::Reg<rd_key1_data5::RD_KEY1_DATA5_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA5"]
pub mod rd_key1_data5;
#[doc = "RD_KEY1_DATA6 register accessor: an alias for `Reg<RD_KEY1_DATA6_SPEC>`"]
pub type RD_KEY1_DATA6 = crate::Reg<rd_key1_data6::RD_KEY1_DATA6_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA6"]
pub mod rd_key1_data6;
#[doc = "RD_KEY1_DATA7 register accessor: an alias for `Reg<RD_KEY1_DATA7_SPEC>`"]
pub type RD_KEY1_DATA7 = crate::Reg<rd_key1_data7::RD_KEY1_DATA7_SPEC>;
#[doc = "EFUSE_RD_KEY1_DATA7"]
pub mod rd_key1_data7;
#[doc = "RD_KEY2_DATA0 register accessor: an alias for `Reg<RD_KEY2_DATA0_SPEC>`"]
pub type RD_KEY2_DATA0 = crate::Reg<rd_key2_data0::RD_KEY2_DATA0_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA0"]
pub mod rd_key2_data0;
#[doc = "RD_KEY2_DATA1 register accessor: an alias for `Reg<RD_KEY2_DATA1_SPEC>`"]
pub type RD_KEY2_DATA1 = crate::Reg<rd_key2_data1::RD_KEY2_DATA1_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA1"]
pub mod rd_key2_data1;
#[doc = "RD_KEY2_DATA2 register accessor: an alias for `Reg<RD_KEY2_DATA2_SPEC>`"]
pub type RD_KEY2_DATA2 = crate::Reg<rd_key2_data2::RD_KEY2_DATA2_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA2"]
pub mod rd_key2_data2;
#[doc = "RD_KEY2_DATA3 register accessor: an alias for `Reg<RD_KEY2_DATA3_SPEC>`"]
pub type RD_KEY2_DATA3 = crate::Reg<rd_key2_data3::RD_KEY2_DATA3_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA3"]
pub mod rd_key2_data3;
#[doc = "RD_KEY2_DATA4 register accessor: an alias for `Reg<RD_KEY2_DATA4_SPEC>`"]
pub type RD_KEY2_DATA4 = crate::Reg<rd_key2_data4::RD_KEY2_DATA4_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA4"]
pub mod rd_key2_data4;
#[doc = "RD_KEY2_DATA5 register accessor: an alias for `Reg<RD_KEY2_DATA5_SPEC>`"]
pub type RD_KEY2_DATA5 = crate::Reg<rd_key2_data5::RD_KEY2_DATA5_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA5"]
pub mod rd_key2_data5;
#[doc = "RD_KEY2_DATA6 register accessor: an alias for `Reg<RD_KEY2_DATA6_SPEC>`"]
pub type RD_KEY2_DATA6 = crate::Reg<rd_key2_data6::RD_KEY2_DATA6_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA6"]
pub mod rd_key2_data6;
#[doc = "RD_KEY2_DATA7 register accessor: an alias for `Reg<RD_KEY2_DATA7_SPEC>`"]
pub type RD_KEY2_DATA7 = crate::Reg<rd_key2_data7::RD_KEY2_DATA7_SPEC>;
#[doc = "EFUSE_RD_KEY2_DATA7"]
pub mod rd_key2_data7;
#[doc = "RD_KEY3_DATA0 register accessor: an alias for `Reg<RD_KEY3_DATA0_SPEC>`"]
pub type RD_KEY3_DATA0 = crate::Reg<rd_key3_data0::RD_KEY3_DATA0_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA0"]
pub mod rd_key3_data0;
#[doc = "RD_KEY3_DATA1 register accessor: an alias for `Reg<RD_KEY3_DATA1_SPEC>`"]
pub type RD_KEY3_DATA1 = crate::Reg<rd_key3_data1::RD_KEY3_DATA1_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA1"]
pub mod rd_key3_data1;
#[doc = "RD_KEY3_DATA2 register accessor: an alias for `Reg<RD_KEY3_DATA2_SPEC>`"]
pub type RD_KEY3_DATA2 = crate::Reg<rd_key3_data2::RD_KEY3_DATA2_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA2"]
pub mod rd_key3_data2;
#[doc = "RD_KEY3_DATA3 register accessor: an alias for `Reg<RD_KEY3_DATA3_SPEC>`"]
pub type RD_KEY3_DATA3 = crate::Reg<rd_key3_data3::RD_KEY3_DATA3_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA3"]
pub mod rd_key3_data3;
#[doc = "RD_KEY3_DATA4 register accessor: an alias for `Reg<RD_KEY3_DATA4_SPEC>`"]
pub type RD_KEY3_DATA4 = crate::Reg<rd_key3_data4::RD_KEY3_DATA4_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA4"]
pub mod rd_key3_data4;
#[doc = "RD_KEY3_DATA5 register accessor: an alias for `Reg<RD_KEY3_DATA5_SPEC>`"]
pub type RD_KEY3_DATA5 = crate::Reg<rd_key3_data5::RD_KEY3_DATA5_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA5"]
pub mod rd_key3_data5;
#[doc = "RD_KEY3_DATA6 register accessor: an alias for `Reg<RD_KEY3_DATA6_SPEC>`"]
pub type RD_KEY3_DATA6 = crate::Reg<rd_key3_data6::RD_KEY3_DATA6_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA6"]
pub mod rd_key3_data6;
#[doc = "RD_KEY3_DATA7 register accessor: an alias for `Reg<RD_KEY3_DATA7_SPEC>`"]
pub type RD_KEY3_DATA7 = crate::Reg<rd_key3_data7::RD_KEY3_DATA7_SPEC>;
#[doc = "EFUSE_RD_KEY3_DATA7"]
pub mod rd_key3_data7;
#[doc = "RD_KEY4_DATA0 register accessor: an alias for `Reg<RD_KEY4_DATA0_SPEC>`"]
pub type RD_KEY4_DATA0 = crate::Reg<rd_key4_data0::RD_KEY4_DATA0_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA0"]
pub mod rd_key4_data0;
#[doc = "RD_KEY4_DATA1 register accessor: an alias for `Reg<RD_KEY4_DATA1_SPEC>`"]
pub type RD_KEY4_DATA1 = crate::Reg<rd_key4_data1::RD_KEY4_DATA1_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA1"]
pub mod rd_key4_data1;
#[doc = "RD_KEY4_DATA2 register accessor: an alias for `Reg<RD_KEY4_DATA2_SPEC>`"]
pub type RD_KEY4_DATA2 = crate::Reg<rd_key4_data2::RD_KEY4_DATA2_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA2"]
pub mod rd_key4_data2;
#[doc = "RD_KEY4_DATA3 register accessor: an alias for `Reg<RD_KEY4_DATA3_SPEC>`"]
pub type RD_KEY4_DATA3 = crate::Reg<rd_key4_data3::RD_KEY4_DATA3_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA3"]
pub mod rd_key4_data3;
#[doc = "RD_KEY4_DATA4 register accessor: an alias for `Reg<RD_KEY4_DATA4_SPEC>`"]
pub type RD_KEY4_DATA4 = crate::Reg<rd_key4_data4::RD_KEY4_DATA4_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA4"]
pub mod rd_key4_data4;
#[doc = "RD_KEY4_DATA5 register accessor: an alias for `Reg<RD_KEY4_DATA5_SPEC>`"]
pub type RD_KEY4_DATA5 = crate::Reg<rd_key4_data5::RD_KEY4_DATA5_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA5"]
pub mod rd_key4_data5;
#[doc = "RD_KEY4_DATA6 register accessor: an alias for `Reg<RD_KEY4_DATA6_SPEC>`"]
pub type RD_KEY4_DATA6 = crate::Reg<rd_key4_data6::RD_KEY4_DATA6_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA6"]
pub mod rd_key4_data6;
#[doc = "RD_KEY4_DATA7 register accessor: an alias for `Reg<RD_KEY4_DATA7_SPEC>`"]
pub type RD_KEY4_DATA7 = crate::Reg<rd_key4_data7::RD_KEY4_DATA7_SPEC>;
#[doc = "EFUSE_RD_KEY4_DATA7"]
pub mod rd_key4_data7;
#[doc = "RD_KEY5_DATA0 register accessor: an alias for `Reg<RD_KEY5_DATA0_SPEC>`"]
pub type RD_KEY5_DATA0 = crate::Reg<rd_key5_data0::RD_KEY5_DATA0_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA0"]
pub mod rd_key5_data0;
#[doc = "RD_KEY5_DATA1 register accessor: an alias for `Reg<RD_KEY5_DATA1_SPEC>`"]
pub type RD_KEY5_DATA1 = crate::Reg<rd_key5_data1::RD_KEY5_DATA1_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA1"]
pub mod rd_key5_data1;
#[doc = "RD_KEY5_DATA2 register accessor: an alias for `Reg<RD_KEY5_DATA2_SPEC>`"]
pub type RD_KEY5_DATA2 = crate::Reg<rd_key5_data2::RD_KEY5_DATA2_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA2"]
pub mod rd_key5_data2;
#[doc = "RD_KEY5_DATA3 register accessor: an alias for `Reg<RD_KEY5_DATA3_SPEC>`"]
pub type RD_KEY5_DATA3 = crate::Reg<rd_key5_data3::RD_KEY5_DATA3_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA3"]
pub mod rd_key5_data3;
#[doc = "RD_KEY5_DATA4 register accessor: an alias for `Reg<RD_KEY5_DATA4_SPEC>`"]
pub type RD_KEY5_DATA4 = crate::Reg<rd_key5_data4::RD_KEY5_DATA4_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA4"]
pub mod rd_key5_data4;
#[doc = "RD_KEY5_DATA5 register accessor: an alias for `Reg<RD_KEY5_DATA5_SPEC>`"]
pub type RD_KEY5_DATA5 = crate::Reg<rd_key5_data5::RD_KEY5_DATA5_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA5"]
pub mod rd_key5_data5;
#[doc = "RD_KEY5_DATA6 register accessor: an alias for `Reg<RD_KEY5_DATA6_SPEC>`"]
pub type RD_KEY5_DATA6 = crate::Reg<rd_key5_data6::RD_KEY5_DATA6_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA6"]
pub mod rd_key5_data6;
#[doc = "RD_KEY5_DATA7 register accessor: an alias for `Reg<RD_KEY5_DATA7_SPEC>`"]
pub type RD_KEY5_DATA7 = crate::Reg<rd_key5_data7::RD_KEY5_DATA7_SPEC>;
#[doc = "EFUSE_RD_KEY5_DATA7"]
pub mod rd_key5_data7;
#[doc = "RD_SYS_PART2_DATA0 register accessor: an alias for `Reg<RD_SYS_PART2_DATA0_SPEC>`"]
pub type RD_SYS_PART2_DATA0 = crate::Reg<rd_sys_part2_data0::RD_SYS_PART2_DATA0_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA0"]
pub mod rd_sys_part2_data0;
#[doc = "RD_SYS_PART2_DATA1 register accessor: an alias for `Reg<RD_SYS_PART2_DATA1_SPEC>`"]
pub type RD_SYS_PART2_DATA1 = crate::Reg<rd_sys_part2_data1::RD_SYS_PART2_DATA1_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA1"]
pub mod rd_sys_part2_data1;
#[doc = "RD_SYS_PART2_DATA2 register accessor: an alias for `Reg<RD_SYS_PART2_DATA2_SPEC>`"]
pub type RD_SYS_PART2_DATA2 = crate::Reg<rd_sys_part2_data2::RD_SYS_PART2_DATA2_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA2"]
pub mod rd_sys_part2_data2;
#[doc = "RD_SYS_PART2_DATA3 register accessor: an alias for `Reg<RD_SYS_PART2_DATA3_SPEC>`"]
pub type RD_SYS_PART2_DATA3 = crate::Reg<rd_sys_part2_data3::RD_SYS_PART2_DATA3_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA3"]
pub mod rd_sys_part2_data3;
#[doc = "RD_SYS_PART2_DATA4 register accessor: an alias for `Reg<RD_SYS_PART2_DATA4_SPEC>`"]
pub type RD_SYS_PART2_DATA4 = crate::Reg<rd_sys_part2_data4::RD_SYS_PART2_DATA4_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA4"]
pub mod rd_sys_part2_data4;
#[doc = "RD_SYS_PART2_DATA5 register accessor: an alias for `Reg<RD_SYS_PART2_DATA5_SPEC>`"]
pub type RD_SYS_PART2_DATA5 = crate::Reg<rd_sys_part2_data5::RD_SYS_PART2_DATA5_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA5"]
pub mod rd_sys_part2_data5;
#[doc = "RD_SYS_PART2_DATA6 register accessor: an alias for `Reg<RD_SYS_PART2_DATA6_SPEC>`"]
pub type RD_SYS_PART2_DATA6 = crate::Reg<rd_sys_part2_data6::RD_SYS_PART2_DATA6_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA6"]
pub mod rd_sys_part2_data6;
#[doc = "RD_SYS_PART2_DATA7 register accessor: an alias for `Reg<RD_SYS_PART2_DATA7_SPEC>`"]
pub type RD_SYS_PART2_DATA7 = crate::Reg<rd_sys_part2_data7::RD_SYS_PART2_DATA7_SPEC>;
#[doc = "EFUSE_RD_SYS_PART2_DATA7"]
pub mod rd_sys_part2_data7;
#[doc = "RD_REPEAT_ERR0 register accessor: an alias for `Reg<RD_REPEAT_ERR0_SPEC>`"]
pub type RD_REPEAT_ERR0 = crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>;
#[doc = "EFUSE_RD_REPEAT_ERR0"]
pub mod rd_repeat_err0;
#[doc = "RD_REPEAT_ERR1 register accessor: an alias for `Reg<RD_REPEAT_ERR1_SPEC>`"]
pub type RD_REPEAT_ERR1 = crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>;
#[doc = "EFUSE_RD_REPEAT_ERR1"]
pub mod rd_repeat_err1;
#[doc = "RD_REPEAT_ERR2 register accessor: an alias for `Reg<RD_REPEAT_ERR2_SPEC>`"]
pub type RD_REPEAT_ERR2 = crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>;
#[doc = "EFUSE_RD_REPEAT_ERR2"]
pub mod rd_repeat_err2;
#[doc = "RD_REPEAT_ERR3 register accessor: an alias for `Reg<RD_REPEAT_ERR3_SPEC>`"]
pub type RD_REPEAT_ERR3 = crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>;
#[doc = "EFUSE_RD_REPEAT_ERR3"]
pub mod rd_repeat_err3;
#[doc = "RD_REPEAT_ERR4 register accessor: an alias for `Reg<RD_REPEAT_ERR4_SPEC>`"]
pub type RD_REPEAT_ERR4 = crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>;
#[doc = "EFUSE_RD_REPEAT_ERR4"]
pub mod rd_repeat_err4;
#[doc = "RD_RS_ERR0 register accessor: an alias for `Reg<RD_RS_ERR0_SPEC>`"]
pub type RD_RS_ERR0 = crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>;
#[doc = "EFUSE_RD_RS_ERR0"]
pub mod rd_rs_err0;
#[doc = "RD_RS_ERR1 register accessor: an alias for `Reg<RD_RS_ERR1_SPEC>`"]
pub type RD_RS_ERR1 = crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>;
#[doc = "EFUSE_RD_RS_ERR1"]
pub mod rd_rs_err1;
#[doc = "CLK register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "EFUSE_CLK"]
pub mod clk;
#[doc = "CONF register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "EFUSE_CONF"]
pub mod conf;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "EFUSE_STATUS"]
pub mod status;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "EFUSE_CMD"]
pub mod cmd;
#[doc = "INT_RAW register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "EFUSE_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "EFUSE_INT_ST"]
pub mod int_st;
#[doc = "INT_ENA register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "EFUSE_INT_ENA"]
pub mod int_ena;
#[doc = "INT_CLR register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "EFUSE_INT_CLR"]
pub mod int_clr;
#[doc = "DAC_CONF register accessor: an alias for `Reg<DAC_CONF_SPEC>`"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = "EFUSE_DAC_CONF"]
pub mod dac_conf;
#[doc = "RD_TIM_CONF register accessor: an alias for `Reg<RD_TIM_CONF_SPEC>`"]
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
#[doc = "EFUSE_RD_TIM_CONF"]
pub mod rd_tim_conf;
#[doc = "WR_TIM_CONF1 register accessor: an alias for `Reg<WR_TIM_CONF1_SPEC>`"]
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
#[doc = "EFUSE_WR_TIM_CONF1"]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 register accessor: an alias for `Reg<WR_TIM_CONF2_SPEC>`"]
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
#[doc = "EFUSE_WR_TIM_CONF2"]
pub mod wr_tim_conf2;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "EFUSE_DATE"]
pub mod date;
