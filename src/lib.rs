#![doc = "Peripheral access API for ESP32C3 microcontrollers (generated using svd2rust v0.19.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.19.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "APB_CTRL"]
pub struct APB_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_CTRL {}
impl APB_CTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apb_ctrl::RegisterBlock = 0x6002_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_ctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APB_CTRL {
    type Target = apb_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APB_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CTRL").finish()
    }
}
#[doc = "APB_CTRL"]
pub mod apb_ctrl;
#[doc = "APB_SARADC"]
pub struct APB_SARADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for APB_SARADC {}
impl APB_SARADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const apb_saradc::RegisterBlock = 0x6004_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const apb_saradc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for APB_SARADC {
    type Target = apb_saradc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for APB_SARADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC").finish()
    }
}
#[doc = "APB_SARADC"]
pub mod apb_saradc;
#[doc = "ASSIST_DEBUG"]
pub struct ASSIST_DEBUG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ASSIST_DEBUG {}
impl ASSIST_DEBUG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const assist_debug::RegisterBlock = 0x600c_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const assist_debug::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ASSIST_DEBUG {
    type Target = assist_debug::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ASSIST_DEBUG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_DEBUG").finish()
    }
}
#[doc = "ASSIST_DEBUG"]
pub mod assist_debug;
#[doc = "EFUSE"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse::RegisterBlock = 0x6000_8800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE").finish()
    }
}
#[doc = "EFUSE"]
pub mod efuse;
#[doc = "EXTMEM"]
pub struct EXTMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTMEM {}
impl EXTMEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const extmem::RegisterBlock = 0x600c_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const extmem::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EXTMEM {
    type Target = extmem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EXTMEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM").finish()
    }
}
#[doc = "EXTMEM"]
pub mod extmem;
#[doc = "GDMA"]
pub struct GDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GDMA {}
impl GDMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gdma::RegisterBlock = 0x6003_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gdma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GDMA {
    type Target = gdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GDMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA").finish()
    }
}
#[doc = "GDMA"]
pub mod gdma;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x6000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "GPIO_SD"]
pub struct GPIO_SD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_SD {}
impl GPIO_SD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio_sd::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_sd::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO_SD {
    type Target = gpio_sd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO_SD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_SD").finish()
    }
}
#[doc = "GPIO_SD"]
pub mod gpio_sd;
#[doc = "I2C"]
pub struct I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C {}
impl I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C").finish()
    }
}
#[doc = "I2C"]
pub mod i2c;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c::RegisterBlock = 0x6001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "I2S"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2s::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S").finish()
    }
}
#[doc = "I2S"]
pub mod i2s;
#[doc = "INTERRUPT_CORE0"]
pub struct INTERRUPT_CORE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INTERRUPT_CORE0 {}
impl INTERRUPT_CORE0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const interrupt_core0::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const interrupt_core0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for INTERRUPT_CORE0 {
    type Target = interrupt_core0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for INTERRUPT_CORE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_CORE0").finish()
    }
}
#[doc = "INTERRUPT_CORE0"]
pub mod interrupt_core0;
#[doc = "LEDC"]
pub struct LEDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDC {}
impl LEDC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ledc::RegisterBlock = 0x6001_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ledc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LEDC {
    type Target = ledc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LEDC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC").finish()
    }
}
#[doc = "LEDC"]
pub mod ledc;
#[doc = "RMT"]
pub struct RMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMT {}
impl RMT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rmt::RegisterBlock = 0x6001_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RMT {
    type Target = rmt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RMT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT").finish()
    }
}
#[doc = "RMT"]
pub mod rmt;
#[doc = "RTCCNTL"]
pub struct RTCCNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCCNTL {}
impl RTCCNTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtccntl::RegisterBlock = 0x6000_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtccntl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTCCNTL {
    type Target = rtccntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTCCNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCNTL").finish()
    }
}
#[doc = "RTCCNTL"]
pub mod rtccntl;
#[doc = "RTC_I2C"]
pub struct RTC_I2C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_I2C {}
impl RTC_I2C {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc_i2c::RegisterBlock = 0x6000_e000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_i2c::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC_I2C {
    type Target = rtc_i2c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC_I2C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_I2C").finish()
    }
}
#[doc = "RTC_I2C"]
pub mod rtc_i2c;
#[doc = "SENSITIVE"]
pub struct SENSITIVE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SENSITIVE {}
impl SENSITIVE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sensitive::RegisterBlock = 0x600c_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sensitive::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SENSITIVE {
    type Target = sensitive::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SENSITIVE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSITIVE").finish()
    }
}
#[doc = "SENSITIVE"]
pub mod sensitive;
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI").finish()
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "SPI0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x6000_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x6000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi::RegisterBlock = 0x6002_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI2 {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
#[doc = "SPI_MEM"]
pub struct SPI_MEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI_MEM {}
impl SPI_MEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi_mem::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi_mem::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI_MEM {
    type Target = spi_mem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI_MEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM").finish()
    }
}
#[doc = "SPI_MEM"]
pub mod spi_mem;
#[doc = "SYSCON"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const syscon::RegisterBlock = 0x6002_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSCON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCON").finish()
    }
}
#[doc = "SYSCON"]
pub mod syscon;
#[doc = "SYSTEM"]
pub struct SYSTEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM {}
impl SYSTEM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const system::RegisterBlock = 0x600c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTEM {
    type Target = system::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM").finish()
    }
}
#[doc = "SYSTEM"]
pub mod system;
#[doc = "SYS_TIMER"]
pub struct SYS_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS_TIMER {}
impl SYS_TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sys_timer::RegisterBlock = 0x6002_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYS_TIMER {
    type Target = sys_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYS_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_TIMER").finish()
    }
}
#[doc = "SYS_TIMER"]
pub mod sys_timer;
#[doc = "TIMG0"]
pub struct TIMG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG0 {}
impl TIMG0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timg::RegisterBlock = 0x6001_f000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMG0 {
    type Target = timg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG0").finish()
    }
}
#[doc = "TIMG1"]
pub struct TIMG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG1 {}
impl TIMG1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timg::RegisterBlock = 0x6002_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMG1 {
    type Target = timg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMG1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG1").finish()
    }
}
#[doc = "TIMG"]
pub struct TIMG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMG {}
impl TIMG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timg::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMG {
    type Target = timg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG").finish()
    }
}
#[doc = "TIMG"]
pub mod timg;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0x6000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART").finish()
    }
}
#[doc = "UART"]
pub mod uart;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0x6001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UHCI"]
pub struct UHCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI {}
impl UHCI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uhci::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UHCI {
    type Target = uhci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UHCI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI").finish()
    }
}
#[doc = "UHCI"]
pub mod uhci;
#[doc = "UHCI0"]
pub struct UHCI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UHCI0 {}
impl UHCI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uhci::RegisterBlock = 0x6001_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uhci::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UHCI0 {
    type Target = uhci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UHCI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI0").finish()
    }
}
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart::RegisterBlock = 0x6000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "IO Multiplexer"]
pub struct IO_MUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IO_MUX {}
impl IO_MUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const io_mux::RegisterBlock = 0x6000_9000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const io_mux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IO_MUX {
    type Target = io_mux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IO_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX").finish()
    }
}
#[doc = "IO Multiplexer"]
pub mod io_mux;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "APB_CTRL"]
    pub APB_CTRL: APB_CTRL,
    #[doc = "APB_SARADC"]
    pub APB_SARADC: APB_SARADC,
    #[doc = "ASSIST_DEBUG"]
    pub ASSIST_DEBUG: ASSIST_DEBUG,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "EXTMEM"]
    pub EXTMEM: EXTMEM,
    #[doc = "GDMA"]
    pub GDMA: GDMA,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPIO_SD"]
    pub GPIO_SD: GPIO_SD,
    #[doc = "I2C"]
    pub I2C: I2C,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "INTERRUPT_CORE0"]
    pub INTERRUPT_CORE0: INTERRUPT_CORE0,
    #[doc = "LEDC"]
    pub LEDC: LEDC,
    #[doc = "RMT"]
    pub RMT: RMT,
    #[doc = "RTCCNTL"]
    pub RTCCNTL: RTCCNTL,
    #[doc = "RTC_I2C"]
    pub RTC_I2C: RTC_I2C,
    #[doc = "SENSITIVE"]
    pub SENSITIVE: SENSITIVE,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI_MEM"]
    pub SPI_MEM: SPI_MEM,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "SYSTEM"]
    pub SYSTEM: SYSTEM,
    #[doc = "SYS_TIMER"]
    pub SYS_TIMER: SYS_TIMER,
    #[doc = "TIMG0"]
    pub TIMG0: TIMG0,
    #[doc = "TIMG1"]
    pub TIMG1: TIMG1,
    #[doc = "TIMG"]
    pub TIMG: TIMG,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UHCI"]
    pub UHCI: UHCI,
    #[doc = "UHCI0"]
    pub UHCI0: UHCI0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "IO_MUX"]
    pub IO_MUX: IO_MUX,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            APB_CTRL: APB_CTRL {
                _marker: PhantomData,
            },
            APB_SARADC: APB_SARADC {
                _marker: PhantomData,
            },
            ASSIST_DEBUG: ASSIST_DEBUG {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            EXTMEM: EXTMEM {
                _marker: PhantomData,
            },
            GDMA: GDMA {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPIO_SD: GPIO_SD {
                _marker: PhantomData,
            },
            I2C: I2C {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            INTERRUPT_CORE0: INTERRUPT_CORE0 {
                _marker: PhantomData,
            },
            LEDC: LEDC {
                _marker: PhantomData,
            },
            RMT: RMT {
                _marker: PhantomData,
            },
            RTCCNTL: RTCCNTL {
                _marker: PhantomData,
            },
            RTC_I2C: RTC_I2C {
                _marker: PhantomData,
            },
            SENSITIVE: SENSITIVE {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI_MEM: SPI_MEM {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            SYSTEM: SYSTEM {
                _marker: PhantomData,
            },
            SYS_TIMER: SYS_TIMER {
                _marker: PhantomData,
            },
            TIMG0: TIMG0 {
                _marker: PhantomData,
            },
            TIMG1: TIMG1 {
                _marker: PhantomData,
            },
            TIMG: TIMG {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UHCI: UHCI {
                _marker: PhantomData,
            },
            UHCI0: UHCI0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            IO_MUX: IO_MUX {
                _marker: PhantomData,
            },
        }
    }
}
