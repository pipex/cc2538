#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
    pub clock_ctrl: CLOCK_CTRL,
    #[doc = "0x04 - Clock status register This register reflects the current chip status."]
    pub clock_sta: CLOCK_STA,
    #[doc = "0x08 - This register defines the module clocks for GPT\\[3:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcgpt: RCGCGPT,
    #[doc = "0x0c - This register defines the module clocks for GPT\\[3:0\\] when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcgpt: SCGCGPT,
    #[doc = "0x10 - This register defines the module clocks for GPT\\[3:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcgpt: DCGCGPT,
    #[doc = "0x14 - This register controls the reset for GPT\\[3:0\\]."]
    pub srgpt: SRGPT,
    #[doc = "0x18 - This register defines the module clocks for SSI\\[1:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcssi: RCGCSSI,
    #[doc = "0x1c - This register defines the module clocks for SSI\\[1:0\\] when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcssi: SCGCSSI,
    #[doc = "0x20 - This register defines the module clocks for SSI\\[1:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcssi: DCGCSSI,
    #[doc = "0x24 - This register controls the reset for SSI\\[1:0\\]."]
    pub srssi: SRSSI,
    #[doc = "0x28 - This register defines the module clocks for UART\\[1:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcuart: RCGCUART,
    #[doc = "0x2c - This register defines the module clocks for UART\\[1:0\\] when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcuart: SCGCUART,
    #[doc = "0x30 - This register defines the module clocks for UART\\[1:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcuart: DCGCUART,
    #[doc = "0x34 - This register controls the reset for UART\\[1:0\\]."]
    pub sruart: SRUART,
    #[doc = "0x38 - This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgci2c: RCGCI2C,
    #[doc = "0x3c - This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgci2c: SCGCI2C,
    #[doc = "0x40 - This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgci2c: DCGCI2C,
    #[doc = "0x44 - This register controls the reset for I2C."]
    pub sri2c: SRI2C,
    #[doc = "0x48 - This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcsec: RCGCSEC,
    #[doc = "0x4c - This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcsec: SCGCSEC,
    #[doc = "0x50 - This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcsec: DCGCSEC,
    #[doc = "0x54 - This register controls the reset for the security module."]
    pub srsec: SRSEC,
    #[doc = "0x58 - This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
    pub pmctl: PMCTL,
    #[doc = "0x5c - This register controls CRC on state retention."]
    pub srcrc: SRCRC,
    _reserved24: [u8; 20usize],
    #[doc = "0x74 - Power debug register"]
    pub pwrdbg: PWRDBG,
    _reserved25: [u8; 8usize],
    #[doc = "0x80 - This register controls the clock loss detection feature."]
    pub cld: CLD,
    _reserved26: [u8; 16usize],
    #[doc = "0x94 - This register controls interrupt wake-up."]
    pub iwe: IWE,
    #[doc = "0x98 - This register selects which interrupt map to be used."]
    pub i_map: I_MAP,
    _reserved28: [u8; 12usize],
    #[doc = "0xa8 - This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub rcgcrfc: RCGCRFC,
    #[doc = "0xac - This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub scgcrfc: SCGCRFC,
    #[doc = "0xb0 - This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
    pub dcgcrfc: DCGCRFC,
    #[doc = "0xb4 - This register defines the emulator override controls for power mode and peripheral clock gate."]
    pub emuovr: EMUOVR,
}
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock_ctrl](clock_ctrl) module"]
pub type CLOCK_CTRL = crate::Reg<u32, _CLOCK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_CTRL;
#[doc = "`read()` method returns [clock_ctrl::R](clock_ctrl::R) reader structure"]
impl crate::Readable for CLOCK_CTRL {}
#[doc = "`write(|w| ..)` method takes [clock_ctrl::W](clock_ctrl::W) writer structure"]
impl crate::Writable for CLOCK_CTRL {}
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register."]
pub mod clock_ctrl;
#[doc = "Clock status register This register reflects the current chip status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clock_sta](clock_sta) module"]
pub type CLOCK_STA = crate::Reg<u32, _CLOCK_STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK_STA;
#[doc = "`read()` method returns [clock_sta::R](clock_sta::R) reader structure"]
impl crate::Readable for CLOCK_STA {}
#[doc = "`write(|w| ..)` method takes [clock_sta::W](clock_sta::W) writer structure"]
impl crate::Writable for CLOCK_STA {}
#[doc = "Clock status register This register reflects the current chip status."]
pub mod clock_sta;
#[doc = "This register defines the module clocks for GPT\\[3:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcgpt](rcgcgpt) module"]
pub type RCGCGPT = crate::Reg<u32, _RCGCGPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCGPT;
#[doc = "`read()` method returns [rcgcgpt::R](rcgcgpt::R) reader structure"]
impl crate::Readable for RCGCGPT {}
#[doc = "`write(|w| ..)` method takes [rcgcgpt::W](rcgcgpt::W) writer structure"]
impl crate::Writable for RCGCGPT {}
#[doc = "This register defines the module clocks for GPT\\[3:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcgpt;
#[doc = "This register defines the module clocks for GPT\\[3:0\\] when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcgpt](scgcgpt) module"]
pub type SCGCGPT = crate::Reg<u32, _SCGCGPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCGPT;
#[doc = "`read()` method returns [scgcgpt::R](scgcgpt::R) reader structure"]
impl crate::Readable for SCGCGPT {}
#[doc = "`write(|w| ..)` method takes [scgcgpt::W](scgcgpt::W) writer structure"]
impl crate::Writable for SCGCGPT {}
#[doc = "This register defines the module clocks for GPT\\[3:0\\] when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcgpt;
#[doc = "This register defines the module clocks for GPT\\[3:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcgpt](dcgcgpt) module"]
pub type DCGCGPT = crate::Reg<u32, _DCGCGPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCGPT;
#[doc = "`read()` method returns [dcgcgpt::R](dcgcgpt::R) reader structure"]
impl crate::Readable for DCGCGPT {}
#[doc = "`write(|w| ..)` method takes [dcgcgpt::W](dcgcgpt::W) writer structure"]
impl crate::Writable for DCGCGPT {}
#[doc = "This register defines the module clocks for GPT\\[3:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcgpt;
#[doc = "This register controls the reset for GPT\\[3:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srgpt](srgpt) module"]
pub type SRGPT = crate::Reg<u32, _SRGPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRGPT;
#[doc = "`read()` method returns [srgpt::R](srgpt::R) reader structure"]
impl crate::Readable for SRGPT {}
#[doc = "`write(|w| ..)` method takes [srgpt::W](srgpt::W) writer structure"]
impl crate::Writable for SRGPT {}
#[doc = "This register controls the reset for GPT\\[3:0\\]."]
pub mod srgpt;
#[doc = "This register defines the module clocks for SSI\\[1:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcssi](rcgcssi) module"]
pub type RCGCSSI = crate::Reg<u32, _RCGCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCSSI;
#[doc = "`read()` method returns [rcgcssi::R](rcgcssi::R) reader structure"]
impl crate::Readable for RCGCSSI {}
#[doc = "`write(|w| ..)` method takes [rcgcssi::W](rcgcssi::W) writer structure"]
impl crate::Writable for RCGCSSI {}
#[doc = "This register defines the module clocks for SSI\\[1:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcssi;
#[doc = "This register defines the module clocks for SSI\\[1:0\\] when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcssi](scgcssi) module"]
pub type SCGCSSI = crate::Reg<u32, _SCGCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCSSI;
#[doc = "`read()` method returns [scgcssi::R](scgcssi::R) reader structure"]
impl crate::Readable for SCGCSSI {}
#[doc = "`write(|w| ..)` method takes [scgcssi::W](scgcssi::W) writer structure"]
impl crate::Writable for SCGCSSI {}
#[doc = "This register defines the module clocks for SSI\\[1:0\\] when the CPU is insSleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcssi;
#[doc = "This register defines the module clocks for SSI\\[1:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcssi](dcgcssi) module"]
pub type DCGCSSI = crate::Reg<u32, _DCGCSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCSSI;
#[doc = "`read()` method returns [dcgcssi::R](dcgcssi::R) reader structure"]
impl crate::Readable for DCGCSSI {}
#[doc = "`write(|w| ..)` method takes [dcgcssi::W](dcgcssi::W) writer structure"]
impl crate::Writable for DCGCSSI {}
#[doc = "This register defines the module clocks for SSI\\[1:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcssi;
#[doc = "This register controls the reset for SSI\\[1:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srssi](srssi) module"]
pub type SRSSI = crate::Reg<u32, _SRSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSSI;
#[doc = "`read()` method returns [srssi::R](srssi::R) reader structure"]
impl crate::Readable for SRSSI {}
#[doc = "`write(|w| ..)` method takes [srssi::W](srssi::W) writer structure"]
impl crate::Writable for SRSSI {}
#[doc = "This register controls the reset for SSI\\[1:0\\]."]
pub mod srssi;
#[doc = "This register defines the module clocks for UART\\[1:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcuart](rcgcuart) module"]
pub type RCGCUART = crate::Reg<u32, _RCGCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCUART;
#[doc = "`read()` method returns [rcgcuart::R](rcgcuart::R) reader structure"]
impl crate::Readable for RCGCUART {}
#[doc = "`write(|w| ..)` method takes [rcgcuart::W](rcgcuart::W) writer structure"]
impl crate::Writable for RCGCUART {}
#[doc = "This register defines the module clocks for UART\\[1:0\\] when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcuart;
#[doc = "This register defines the module clocks for UART\\[1:0\\] when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcuart](scgcuart) module"]
pub type SCGCUART = crate::Reg<u32, _SCGCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCUART;
#[doc = "`read()` method returns [scgcuart::R](scgcuart::R) reader structure"]
impl crate::Readable for SCGCUART {}
#[doc = "`write(|w| ..)` method takes [scgcuart::W](scgcuart::W) writer structure"]
impl crate::Writable for SCGCUART {}
#[doc = "This register defines the module clocks for UART\\[1:0\\] when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcuart;
#[doc = "This register defines the module clocks for UART\\[1:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcuart](dcgcuart) module"]
pub type DCGCUART = crate::Reg<u32, _DCGCUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCUART;
#[doc = "`read()` method returns [dcgcuart::R](dcgcuart::R) reader structure"]
impl crate::Readable for DCGCUART {}
#[doc = "`write(|w| ..)` method takes [dcgcuart::W](dcgcuart::W) writer structure"]
impl crate::Writable for DCGCUART {}
#[doc = "This register defines the module clocks for UART\\[1:0\\] when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcuart;
#[doc = "This register controls the reset for UART\\[1:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sruart](sruart) module"]
pub type SRUART = crate::Reg<u32, _SRUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRUART;
#[doc = "`read()` method returns [sruart::R](sruart::R) reader structure"]
impl crate::Readable for SRUART {}
#[doc = "`write(|w| ..)` method takes [sruart::W](sruart::W) writer structure"]
impl crate::Writable for SRUART {}
#[doc = "This register controls the reset for UART\\[1:0\\]."]
pub mod sruart;
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgci2c](rcgci2c) module"]
pub type RCGCI2C = crate::Reg<u32, _RCGCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCI2C;
#[doc = "`read()` method returns [rcgci2c::R](rcgci2c::R) reader structure"]
impl crate::Readable for RCGCI2C {}
#[doc = "`write(|w| ..)` method takes [rcgci2c::W](rcgci2c::W) writer structure"]
impl crate::Writable for RCGCI2C {}
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgci2c;
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgci2c](scgci2c) module"]
pub type SCGCI2C = crate::Reg<u32, _SCGCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCI2C;
#[doc = "`read()` method returns [scgci2c::R](scgci2c::R) reader structure"]
impl crate::Readable for SCGCI2C {}
#[doc = "`write(|w| ..)` method takes [scgci2c::W](scgci2c::W) writer structure"]
impl crate::Writable for SCGCI2C {}
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgci2c;
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgci2c](dcgci2c) module"]
pub type DCGCI2C = crate::Reg<u32, _DCGCI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCI2C;
#[doc = "`read()` method returns [dcgci2c::R](dcgci2c::R) reader structure"]
impl crate::Readable for DCGCI2C {}
#[doc = "`write(|w| ..)` method takes [dcgci2c::W](dcgci2c::W) writer structure"]
impl crate::Writable for DCGCI2C {}
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgci2c;
#[doc = "This register controls the reset for I2C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sri2c](sri2c) module"]
pub type SRI2C = crate::Reg<u32, _SRI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRI2C;
#[doc = "`read()` method returns [sri2c::R](sri2c::R) reader structure"]
impl crate::Readable for SRI2C {}
#[doc = "`write(|w| ..)` method takes [sri2c::W](sri2c::W) writer structure"]
impl crate::Writable for SRI2C {}
#[doc = "This register controls the reset for I2C."]
pub mod sri2c;
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcsec](rcgcsec) module"]
pub type RCGCSEC = crate::Reg<u32, _RCGCSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCSEC;
#[doc = "`read()` method returns [rcgcsec::R](rcgcsec::R) reader structure"]
impl crate::Readable for RCGCSEC {}
#[doc = "`write(|w| ..)` method takes [rcgcsec::W](rcgcsec::W) writer structure"]
impl crate::Writable for RCGCSEC {}
#[doc = "This register defines the module clocks for the security module when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcsec;
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcsec](scgcsec) module"]
pub type SCGCSEC = crate::Reg<u32, _SCGCSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCSEC;
#[doc = "`read()` method returns [scgcsec::R](scgcsec::R) reader structure"]
impl crate::Readable for SCGCSEC {}
#[doc = "`write(|w| ..)` method takes [scgcsec::W](scgcsec::W) writer structure"]
impl crate::Writable for SCGCSEC {}
#[doc = "This register defines the module clocks for the security module when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcsec;
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcsec](dcgcsec) module"]
pub type DCGCSEC = crate::Reg<u32, _DCGCSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCSEC;
#[doc = "`read()` method returns [dcgcsec::R](dcgcsec::R) reader structure"]
impl crate::Readable for DCGCSEC {}
#[doc = "`write(|w| ..)` method takes [dcgcsec::W](dcgcsec::W) writer structure"]
impl crate::Writable for DCGCSEC {}
#[doc = "This register defines the module clocks for the security module when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcsec;
#[doc = "This register controls the reset for the security module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srsec](srsec) module"]
pub type SRSEC = crate::Reg<u32, _SRSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRSEC;
#[doc = "`read()` method returns [srsec::R](srsec::R) reader structure"]
impl crate::Readable for SRSEC {}
#[doc = "`write(|w| ..)` method takes [srsec::W](srsec::W) writer structure"]
impl crate::Writable for SRSEC {}
#[doc = "This register controls the reset for the security module."]
pub mod srsec;
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmctl](pmctl) module"]
pub type PMCTL = crate::Reg<u32, _PMCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCTL;
#[doc = "`read()` method returns [pmctl::R](pmctl::R) reader structure"]
impl crate::Readable for PMCTL {}
#[doc = "`write(|w| ..)` method takes [pmctl::W](pmctl::W) writer structure"]
impl crate::Writable for PMCTL {}
#[doc = "This register controls the power mode. Note: The Corresponding PM is not entered before the WFI instruction is asserted. To enter PM1-3 the DEEPSLEEP bit in SYSCTRL must be 1."]
pub mod pmctl;
#[doc = "This register controls CRC on state retention.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcrc](srcrc) module"]
pub type SRCRC = crate::Reg<u32, _SRCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCRC;
#[doc = "`read()` method returns [srcrc::R](srcrc::R) reader structure"]
impl crate::Readable for SRCRC {}
#[doc = "`write(|w| ..)` method takes [srcrc::W](srcrc::W) writer structure"]
impl crate::Writable for SRCRC {}
#[doc = "This register controls CRC on state retention."]
pub mod srcrc;
#[doc = "Power debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pwrdbg](pwrdbg) module"]
pub type PWRDBG = crate::Reg<u32, _PWRDBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRDBG;
#[doc = "`read()` method returns [pwrdbg::R](pwrdbg::R) reader structure"]
impl crate::Readable for PWRDBG {}
#[doc = "`write(|w| ..)` method takes [pwrdbg::W](pwrdbg::W) writer structure"]
impl crate::Writable for PWRDBG {}
#[doc = "Power debug register"]
pub mod pwrdbg;
#[doc = "This register controls the clock loss detection feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cld](cld) module"]
pub type CLD = crate::Reg<u32, _CLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLD;
#[doc = "`read()` method returns [cld::R](cld::R) reader structure"]
impl crate::Readable for CLD {}
#[doc = "`write(|w| ..)` method takes [cld::W](cld::W) writer structure"]
impl crate::Writable for CLD {}
#[doc = "This register controls the clock loss detection feature."]
pub mod cld;
#[doc = "This register controls interrupt wake-up.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iwe](iwe) module"]
pub type IWE = crate::Reg<u32, _IWE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IWE;
#[doc = "`read()` method returns [iwe::R](iwe::R) reader structure"]
impl crate::Readable for IWE {}
#[doc = "`write(|w| ..)` method takes [iwe::W](iwe::W) writer structure"]
impl crate::Writable for IWE {}
#[doc = "This register controls interrupt wake-up."]
pub mod iwe;
#[doc = "This register selects which interrupt map to be used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i_map](i_map) module"]
pub type I_MAP = crate::Reg<u32, _I_MAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I_MAP;
#[doc = "`read()` method returns [i_map::R](i_map::R) reader structure"]
impl crate::Readable for I_MAP {}
#[doc = "`write(|w| ..)` method takes [i_map::W](i_map::W) writer structure"]
impl crate::Writable for I_MAP {}
#[doc = "This register selects which interrupt map to be used."]
pub mod i_map;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcgcrfc](rcgcrfc) module"]
pub type RCGCRFC = crate::Reg<u32, _RCGCRFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGCRFC;
#[doc = "`read()` method returns [rcgcrfc::R](rcgcrfc::R) reader structure"]
impl crate::Readable for RCGCRFC {}
#[doc = "`write(|w| ..)` method takes [rcgcrfc::W](rcgcrfc::W) writer structure"]
impl crate::Writable for RCGCRFC {}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod rcgcrfc;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scgcrfc](scgcrfc) module"]
pub type SCGCRFC = crate::Reg<u32, _SCGCRFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGCRFC;
#[doc = "`read()` method returns [scgcrfc::R](scgcrfc::R) reader structure"]
impl crate::Readable for SCGCRFC {}
#[doc = "`write(|w| ..)` method takes [scgcrfc::W](scgcrfc::W) writer structure"]
impl crate::Writable for SCGCRFC {}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod scgcrfc;
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcgcrfc](dcgcrfc) module"]
pub type DCGCRFC = crate::Reg<u32, _DCGCRFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGCRFC;
#[doc = "`read()` method returns [dcgcrfc::R](dcgcrfc::R) reader structure"]
impl crate::Readable for DCGCRFC {}
#[doc = "`write(|w| ..)` method takes [dcgcrfc::W](dcgcrfc::W) writer structure"]
impl crate::Writable for DCGCRFC {}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes."]
pub mod dcgcrfc;
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [emuovr](emuovr) module"]
pub type EMUOVR = crate::Reg<u32, _EMUOVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMUOVR;
#[doc = "`read()` method returns [emuovr::R](emuovr::R) reader structure"]
impl crate::Readable for EMUOVR {}
#[doc = "`write(|w| ..)` method takes [emuovr::W](emuovr::W) writer structure"]
impl crate::Writable for EMUOVR {}
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate."]
pub mod emuovr;
