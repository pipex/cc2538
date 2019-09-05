#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTIMER Internal loopback"]
    pub lpbkgpt: LPBKGPT,
    #[doc = "0x04 - UART internal loopback"]
    pub lpbkuart: LPBKUART,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - I2C internal loopback"]
    pub lpbki2c: LPBKI2C,
    _reserved3: [u8; 52usize],
    #[doc = "0x44 - Peripheral test mode enable 1"]
    pub ptme1: PTME1,
    #[doc = "0x48 - Peripheral test mode enable 2"]
    pub ptme2: PTME2,
    _reserved5: [u8; 116usize],
    #[doc = "0xc0 - GPTIMER override values"]
    pub gpt: GPT,
}
#[doc = "GPTIMER Internal loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpbkgpt](lpbkgpt) module"]
pub type LPBKGPT = crate::Reg<u32, _LPBKGPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPBKGPT;
#[doc = "`read()` method returns [lpbkgpt::R](lpbkgpt::R) reader structure"]
impl crate::Readable for LPBKGPT {}
#[doc = "`write(|w| ..)` method takes [lpbkgpt::W](lpbkgpt::W) writer structure"]
impl crate::Writable for LPBKGPT {}
#[doc = "GPTIMER Internal loopback"]
pub mod lpbkgpt;
#[doc = "UART internal loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpbkuart](lpbkuart) module"]
pub type LPBKUART = crate::Reg<u32, _LPBKUART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPBKUART;
#[doc = "`read()` method returns [lpbkuart::R](lpbkuart::R) reader structure"]
impl crate::Readable for LPBKUART {}
#[doc = "`write(|w| ..)` method takes [lpbkuart::W](lpbkuart::W) writer structure"]
impl crate::Writable for LPBKUART {}
#[doc = "UART internal loopback"]
pub mod lpbkuart;
#[doc = "I2C internal loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpbki2c](lpbki2c) module"]
pub type LPBKI2C = crate::Reg<u32, _LPBKI2C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPBKI2C;
#[doc = "`read()` method returns [lpbki2c::R](lpbki2c::R) reader structure"]
impl crate::Readable for LPBKI2C {}
#[doc = "`write(|w| ..)` method takes [lpbki2c::W](lpbki2c::W) writer structure"]
impl crate::Writable for LPBKI2C {}
#[doc = "I2C internal loopback"]
pub mod lpbki2c;
#[doc = "Peripheral test mode enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptme1](ptme1) module"]
pub type PTME1 = crate::Reg<u32, _PTME1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTME1;
#[doc = "`read()` method returns [ptme1::R](ptme1::R) reader structure"]
impl crate::Readable for PTME1 {}
#[doc = "`write(|w| ..)` method takes [ptme1::W](ptme1::W) writer structure"]
impl crate::Writable for PTME1 {}
#[doc = "Peripheral test mode enable 1"]
pub mod ptme1;
#[doc = "Peripheral test mode enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptme2](ptme2) module"]
pub type PTME2 = crate::Reg<u32, _PTME2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTME2;
#[doc = "`read()` method returns [ptme2::R](ptme2::R) reader structure"]
impl crate::Readable for PTME2 {}
#[doc = "`write(|w| ..)` method takes [ptme2::W](ptme2::W) writer structure"]
impl crate::Writable for PTME2 {}
#[doc = "Peripheral test mode enable 2"]
pub mod ptme2;
#[doc = "GPTIMER override values\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt](gpt) module"]
pub type GPT = crate::Reg<u32, _GPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT;
#[doc = "`read()` method returns [gpt::R](gpt::R) reader structure"]
impl crate::Readable for GPT {}
#[doc = "`write(|w| ..)` method takes [gpt::W](gpt::W) writer structure"]
impl crate::Writable for GPT {}
#[doc = "GPTIMER override values"]
pub mod gpt;
