#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO_BT_SELECT"]
    pub bt_select: crate::Reg<bt_select::BT_SELECT_SPEC>,
    #[doc = "0x04 - GPIO_OUT"]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x08 - GPIO_OUT_W1TS"]
    pub out_w1ts: crate::Reg<out_w1ts::OUT_W1TS_SPEC>,
    #[doc = "0x0c - GPIO_OUT_W1TC"]
    pub out_w1tc: crate::Reg<out_w1tc::OUT_W1TC_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - GPIO_SDIO_SELECT"]
    pub sdio_select: crate::Reg<sdio_select::SDIO_SELECT_SPEC>,
    #[doc = "0x20 - GPIO_ENABLE"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x24 - GPIO_ENABLE_W1TS"]
    pub enable_w1ts: crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>,
    #[doc = "0x28 - GPIO_ENABLE_W1TC"]
    pub enable_w1tc: crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x38 - GPIO_STRAP"]
    pub strap: crate::Reg<strap::STRAP_SPEC>,
    #[doc = "0x3c - GPIO_IN"]
    pub in_: crate::Reg<in_::IN_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - GPIO_STATUS"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x48 - GPIO_STATUS_W1TS"]
    pub status_w1ts: crate::Reg<status_w1ts::STATUS_W1TS_SPEC>,
    #[doc = "0x4c - GPIO_STATUS_W1TC"]
    pub status_w1tc: crate::Reg<status_w1tc::STATUS_W1TC_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x5c - GPIO_PCPU_INT"]
    pub pcpu_int: crate::Reg<pcpu_int::PCPU_INT_SPEC>,
    #[doc = "0x60 - GPIO_PCPU_NMI_INT"]
    pub pcpu_nmi_int: crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>,
    #[doc = "0x64 - GPIO_CPUSDIO_INT"]
    pub cpusdio_int: crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x74..0xdc - GPIO_PIN%s"]
    pub pin: [crate::Reg<pin::PIN_SPEC>; 26],
    _reserved17: [u8; 0x70],
    #[doc = "0x14c - GPIO_STATUS_NEXT"]
    pub status_next: crate::Reg<status_next::STATUS_NEXT_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x154..0x354 - GPIO_FUNC%s_IN_SEL_CFG"]
    pub func_in_sel_cfg: [crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>; 128],
    _reserved19: [u8; 0x0200],
    #[doc = "0x554..0x5bc - GPIO_FUNC%s_OUT_SEL_CFG"]
    pub func_out_sel_cfg: [crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>; 26],
    _reserved20: [u8; 0x70],
    #[doc = "0x62c - GPIO_CLOCK_GATE"]
    pub clock_gate: crate::Reg<clock_gate::CLOCK_GATE_SPEC>,
    _reserved21: [u8; 0xcc],
    #[doc = "0x6fc - GPIO_DATE"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "BT_SELECT register accessor: an alias for `Reg<BT_SELECT_SPEC>`"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = "GPIO_BT_SELECT"]
pub mod bt_select;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO_OUT"]
pub mod out;
#[doc = "OUT_W1TS register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO_OUT_W1TS"]
pub mod out_w1ts;
#[doc = "OUT_W1TC register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO_OUT_W1TC"]
pub mod out_w1tc;
#[doc = "SDIO_SELECT register accessor: an alias for `Reg<SDIO_SELECT_SPEC>`"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = "GPIO_SDIO_SELECT"]
pub mod sdio_select;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO_ENABLE"]
pub mod enable;
#[doc = "ENABLE_W1TS register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO_ENABLE_W1TS"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO_ENABLE_W1TC"]
pub mod enable_w1tc;
#[doc = "STRAP register accessor: an alias for `Reg<STRAP_SPEC>`"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "GPIO_STRAP"]
pub mod strap;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO_IN"]
pub mod in_;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO_STATUS"]
pub mod status;
#[doc = "STATUS_W1TS register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO_STATUS_W1TS"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO_STATUS_W1TC"]
pub mod status_w1tc;
#[doc = "PCPU_INT register accessor: an alias for `Reg<PCPU_INT_SPEC>`"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = "GPIO_PCPU_INT"]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT register accessor: an alias for `Reg<PCPU_NMI_INT_SPEC>`"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = "GPIO_PCPU_NMI_INT"]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT register accessor: an alias for `Reg<CPUSDIO_INT_SPEC>`"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = "GPIO_CPUSDIO_INT"]
pub mod cpusdio_int;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO_PIN%s"]
pub mod pin;
#[doc = "STATUS_NEXT register accessor: an alias for `Reg<STATUS_NEXT_SPEC>`"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO_STATUS_NEXT"]
pub mod status_next;
#[doc = "FUNC_IN_SEL_CFG register accessor: an alias for `Reg<FUNC_IN_SEL_CFG_SPEC>`"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = "GPIO_FUNC%s_IN_SEL_CFG"]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC_OUT_SEL_CFG_SPEC>`"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO_FUNC%s_OUT_SEL_CFG"]
pub mod func_out_sel_cfg;
#[doc = "CLOCK_GATE register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO_CLOCK_GATE"]
pub mod clock_gate;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO_DATE"]
pub mod date;
