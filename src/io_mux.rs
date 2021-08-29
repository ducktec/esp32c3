#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock output configuration Register"]
    pub pin_ctrl: crate::Reg<pin_ctrl::PIN_CTRL_SPEC>,
    #[doc = "0x04..0x5c - configures IO_MUX for GPIO%s"]
    pub gpio: [crate::Reg<gpio::GPIO_SPEC>; 22],
    _reserved2: [u8; 0xa0],
    #[doc = "0xfc - Clock output configuration Register"]
    pub date: crate::Reg<date::DATE_SPEC>,
}
#[doc = "PIN_CTRL register accessor: an alias for `Reg<PIN_CTRL_SPEC>`"]
pub type PIN_CTRL = crate::Reg<pin_ctrl::PIN_CTRL_SPEC>;
#[doc = "Clock output configuration Register"]
pub mod pin_ctrl;
#[doc = "DATE register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Clock output configuration Register"]
pub mod date;
#[doc = "GPIO register accessor: an alias for `Reg<GPIO_SPEC>`"]
pub type GPIO = crate::Reg<gpio::GPIO_SPEC>;
#[doc = "configures IO_MUX for GPIO%s"]
pub mod gpio;
