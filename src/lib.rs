#![doc = "Peripheral access API for CC2538 microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {}
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Synchronous Serial Interface"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Synchronous Serial Interface"]
pub mod ssi0;
#[doc = "Synchronous Serial Interface"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
impl SSI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi1::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for SSI1 {
    type Target = ssi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "Synchronous Serial Interface"]
pub mod ssi1;
#[doc = "Register fields should be considered static unless otherwise noted as dynamic."]
#[doc = "UART 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART 0"]
pub mod uart0;
#[doc = "UART 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART 1"]
pub mod uart1;
#[doc = "I2C Master"]
pub struct I2CM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CM {}
impl I2CM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2cm::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for I2CM {
    type Target = i2cm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2CM::ptr() }
    }
}
#[doc = "I2C Master"]
pub mod i2cm;
#[doc = "I2C Slave"]
pub struct I2CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CS {}
impl I2CS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2cs::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for I2CS {
    type Target = i2cs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2CS::ptr() }
    }
}
#[doc = "I2C Slave."]
pub mod i2cs;
#[doc = "General Purpose Timers"]
pub struct GPTIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMER0 {}
impl GPTIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimer0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for GPTIMER0 {
    type Target = gptimer0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTIMER0::ptr() }
    }
}
#[doc = "General Purpose Timers"]
pub mod gptimer0;
#[doc = "General Purpose Timers"]
pub struct GPTIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMER1 {}
impl GPTIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimer1::RegisterBlock {
        0x4003_1000 as *const _
    }
}
impl Deref for GPTIMER1 {
    type Target = gptimer1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTIMER1::ptr() }
    }
}
#[doc = "General Purpose Timers"]
pub mod gptimer1;
#[doc = "General Purpose Timers"]
pub struct GPTIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMER2 {}
impl GPTIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimer2::RegisterBlock {
        0x4003_2000 as *const _
    }
}
impl Deref for GPTIMER2 {
    type Target = gptimer2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTIMER2::ptr() }
    }
}
#[doc = "General Purpose Timers"]
pub mod gptimer2;
#[doc = "General Purpose Timers"]
pub struct GPTIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTIMER3 {}
impl GPTIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptimer3::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for GPTIMER3 {
    type Target = gptimer3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTIMER3::ptr() }
    }
}
#[doc = "General Purpose Timers"]
pub mod gptimer3;
#[doc = "RF Core FFSM"]
pub struct RFCORE_FFSM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFCORE_FFSM {}
impl RFCORE_FFSM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfcore_ffsm::RegisterBlock {
        0x4008_8500 as *const _
    }
}
impl Deref for RFCORE_FFSM {
    type Target = rfcore_ffsm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFCORE_FFSM::ptr() }
    }
}
#[doc = "RF Core FFSM"]
pub mod rfcore_ffsm;
#[doc = "RF Core XREG"]
pub struct RFCORE_XREG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFCORE_XREG {}
impl RFCORE_XREG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfcore_xreg::RegisterBlock {
        0x4008_8600 as *const _
    }
}
impl Deref for RFCORE_XREG {
    type Target = rfcore_xreg::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFCORE_XREG::ptr() }
    }
}
#[doc = "RF Core XREG"]
pub mod rfcore_xreg;
#[doc = "RF Core SFR"]
pub struct RFCORE_SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFCORE_SFR {}
impl RFCORE_SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfcore_sfr::RegisterBlock {
        0x4008_8800 as *const _
    }
}
impl Deref for RFCORE_SFR {
    type Target = rfcore_sfr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFCORE_SFR::ptr() }
    }
}
#[doc = "RF Core SFR"]
pub mod rfcore_sfr;
#[doc = "USB Module"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4008_9000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB Module"]
pub mod usb;
#[doc = "AES Module"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4008_b000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "AES Module"]
pub mod aes;
#[doc = "Register fields should be considered static unless otherwise noted as dynamic."]
#[doc = "System Control"]
pub struct SYS_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS_CTRL {}
impl SYS_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_ctrl::RegisterBlock {
        0x400d_2000 as *const _
    }
}
impl Deref for SYS_CTRL {
    type Target = sys_ctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS_CTRL::ptr() }
    }
}
#[doc = "System Control"]
pub mod sys_ctrl;
#[doc = "Flash Control"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_ctrl::RegisterBlock {
        0x400d_3000 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "Flash Control"]
pub mod flash_ctrl;
#[doc = "IOC Module"]
pub struct IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOC {}
impl IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ioc::RegisterBlock {
        0x400d_4000 as *const _
    }
}
impl Deref for IOC {
    type Target = ioc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOC::ptr() }
    }
}
#[doc = "IOC Module"]
pub mod ioc;
#[doc = "Sleep Timer and Watchdog"]
pub struct SMWDTHROSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMWDTHROSC {}
impl SMWDTHROSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smwdthrosc::RegisterBlock {
        0x400d_5000 as *const _
    }
}
impl Deref for SMWDTHROSC {
    type Target = smwdthrosc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMWDTHROSC::ptr() }
    }
}
#[doc = "Sleep Timer and Watchdog"]
pub mod smwdthrosc;
#[doc = "ANA_REGS Module"]
pub struct ANA_REGS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ANA_REGS {}
impl ANA_REGS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ana_regs::RegisterBlock {
        0x400d_6000 as *const _
    }
}
impl Deref for ANA_REGS {
    type Target = ana_regs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ANA_REGS::ptr() }
    }
}
#[doc = "ANA_REGS Module"]
pub mod ana_regs;
#[doc = "ADC Module"]
pub struct SOC_ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SOC_ADC {}
impl SOC_ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const soc_adc::RegisterBlock {
        0x400d_7000 as *const _
    }
}
impl Deref for SOC_ADC {
    type Target = soc_adc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SOC_ADC::ptr() }
    }
}
#[doc = "ADC Module"]
pub mod soc_adc;
#[doc = "General Purpose I/O"]
pub struct GPIO_A {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_A {}
impl GPIO_A {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_a::RegisterBlock {
        0x400d_9000 as *const _
    }
}
impl Deref for GPIO_A {
    type Target = gpio_a::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_A::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio_a;
#[doc = "General Purpose I/O"]
pub struct GPIO_B {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_B {}
impl GPIO_B {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_b::RegisterBlock {
        0x400d_a000 as *const _
    }
}
impl Deref for GPIO_B {
    type Target = gpio_b::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_B::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio_b;
#[doc = "General Purpose I/O"]
pub struct GPIO_C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_C {}
impl GPIO_C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_c::RegisterBlock {
        0x400d_b000 as *const _
    }
}
impl Deref for GPIO_C {
    type Target = gpio_c::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_C::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio_c;
#[doc = "General Purpose I/O"]
pub struct GPIO_D {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_D {}
impl GPIO_D {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_d::RegisterBlock {
        0x400d_c000 as *const _
    }
}
impl Deref for GPIO_D {
    type Target = gpio_d::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_D::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio_d;
#[doc = "Micro DMA Controller"]
pub struct UDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA {}
impl UDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for UDMA {
    type Target = udma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA::ptr() }
    }
}
#[doc = "Micro DMA Controller"]
pub mod udma;
#[doc = "STTEST"]
pub struct STTEST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STTEST {}
impl STTEST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sttest::RegisterBlock {
        0x4011_0000 as *const _
    }
}
impl Deref for STTEST {
    type Target = sttest::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*STTEST::ptr() }
    }
}
#[doc = "STTEST"]
pub mod sttest;
#[doc = "Public Key Accelerator Engine"]
pub struct PKA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PKA {}
impl PKA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pka::RegisterBlock {
        0x4400_4000 as *const _
    }
}
impl Deref for PKA {
    type Target = pka::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PKA::ptr() }
    }
}
#[doc = "Public Key Accelerator Engine"]
pub mod pka;
#[doc = "CCTEST"]
pub struct CCTEST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCTEST {}
impl CCTEST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cctest::RegisterBlock {
        0x4401_0000 as *const _
    }
}
impl Deref for CCTEST {
    type Target = cctest::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCTEST::ptr() }
    }
}
#[doc = "CCTEST"]
pub mod cctest;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "I2CM"]
    pub I2CM: I2CM,
    #[doc = "I2CS"]
    pub I2CS: I2CS,
    #[doc = "GPTIMER0"]
    pub GPTIMER0: GPTIMER0,
    #[doc = "GPTIMER1"]
    pub GPTIMER1: GPTIMER1,
    #[doc = "GPTIMER2"]
    pub GPTIMER2: GPTIMER2,
    #[doc = "GPTIMER3"]
    pub GPTIMER3: GPTIMER3,
    #[doc = "RFCORE_FFSM"]
    pub RFCORE_FFSM: RFCORE_FFSM,
    #[doc = "RFCORE_XREG"]
    pub RFCORE_XREG: RFCORE_XREG,
    #[doc = "RFCORE_SFR"]
    pub RFCORE_SFR: RFCORE_SFR,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "SYS_CTRL"]
    pub SYS_CTRL: SYS_CTRL,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "IOC"]
    pub IOC: IOC,
    #[doc = "SMWDTHROSC"]
    pub SMWDTHROSC: SMWDTHROSC,
    #[doc = "ANA_REGS"]
    pub ANA_REGS: ANA_REGS,
    #[doc = "SOC_ADC"]
    pub SOC_ADC: SOC_ADC,
    #[doc = "GPIO_A"]
    pub GPIO_A: GPIO_A,
    #[doc = "GPIO_B"]
    pub GPIO_B: GPIO_B,
    #[doc = "GPIO_C"]
    pub GPIO_C: GPIO_C,
    #[doc = "GPIO_D"]
    pub GPIO_D: GPIO_D,
    #[doc = "UDMA"]
    pub UDMA: UDMA,
    #[doc = "STTEST"]
    pub STTEST: STTEST,
    #[doc = "PKA"]
    pub PKA: PKA,
    #[doc = "CCTEST"]
    pub CCTEST: CCTEST,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            SSI1: SSI1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            I2CM: I2CM {
                _marker: PhantomData,
            },
            I2CS: I2CS {
                _marker: PhantomData,
            },
            GPTIMER0: GPTIMER0 {
                _marker: PhantomData,
            },
            GPTIMER1: GPTIMER1 {
                _marker: PhantomData,
            },
            GPTIMER2: GPTIMER2 {
                _marker: PhantomData,
            },
            GPTIMER3: GPTIMER3 {
                _marker: PhantomData,
            },
            RFCORE_FFSM: RFCORE_FFSM {
                _marker: PhantomData,
            },
            RFCORE_XREG: RFCORE_XREG {
                _marker: PhantomData,
            },
            RFCORE_SFR: RFCORE_SFR {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            SYS_CTRL: SYS_CTRL {
                _marker: PhantomData,
            },
            FLASH_CTRL: FLASH_CTRL {
                _marker: PhantomData,
            },
            IOC: IOC {
                _marker: PhantomData,
            },
            SMWDTHROSC: SMWDTHROSC {
                _marker: PhantomData,
            },
            ANA_REGS: ANA_REGS {
                _marker: PhantomData,
            },
            SOC_ADC: SOC_ADC {
                _marker: PhantomData,
            },
            GPIO_A: GPIO_A {
                _marker: PhantomData,
            },
            GPIO_B: GPIO_B {
                _marker: PhantomData,
            },
            GPIO_C: GPIO_C {
                _marker: PhantomData,
            },
            GPIO_D: GPIO_D {
                _marker: PhantomData,
            },
            UDMA: UDMA {
                _marker: PhantomData,
            },
            STTEST: STTEST {
                _marker: PhantomData,
            },
            PKA: PKA {
                _marker: PhantomData,
            },
            CCTEST: CCTEST {
                _marker: PhantomData,
            },
        }
    }
}
