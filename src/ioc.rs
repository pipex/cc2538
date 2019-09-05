#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Peripheral select control for PA0"]
    pub pa0_sel: PA0_SEL,
    #[doc = "0x04 - Peripheral select control for PA1"]
    pub pa1_sel: PA1_SEL,
    #[doc = "0x08 - Peripheral select control for PA2"]
    pub pa2_sel: PA2_SEL,
    #[doc = "0x0c - Peripheral select control for PA3"]
    pub pa3_sel: PA3_SEL,
    #[doc = "0x10 - Peripheral select control for PA4"]
    pub pa4_sel: PA4_SEL,
    #[doc = "0x14 - Peripheral select control for PA5"]
    pub pa5_sel: PA5_SEL,
    #[doc = "0x18 - Peripheral select control for PA6"]
    pub pa6_sel: PA6_SEL,
    #[doc = "0x1c - Peripheral select control for PA7"]
    pub pa7_sel: PA7_SEL,
    #[doc = "0x20 - Peripheral select control for PB0"]
    pub pb0_sel: PB0_SEL,
    #[doc = "0x24 - Peripheral select control for PB1"]
    pub pb1_sel: PB1_SEL,
    #[doc = "0x28 - Peripheral select control for PB2"]
    pub pb2_sel: PB2_SEL,
    #[doc = "0x2c - Peripheral select control for PB3"]
    pub pb3_sel: PB3_SEL,
    #[doc = "0x30 - Peripheral select control for PB4"]
    pub pb4_sel: PB4_SEL,
    #[doc = "0x34 - Peripheral select control for PB5"]
    pub pb5_sel: PB5_SEL,
    #[doc = "0x38 - Peripheral select control for PB6"]
    pub pb6_sel: PB6_SEL,
    #[doc = "0x3c - Peripheral select control for PB7"]
    pub pb7_sel: PB7_SEL,
    #[doc = "0x40 - Peripheral select control for PC0"]
    pub pc0_sel: PC0_SEL,
    #[doc = "0x44 - Peripheral select control for PC1"]
    pub pc1_sel: PC1_SEL,
    #[doc = "0x48 - Peripheral select control for PC2"]
    pub pc2_sel: PC2_SEL,
    #[doc = "0x4c - Peripheral select control for PC3"]
    pub pc3_sel: PC3_SEL,
    #[doc = "0x50 - Peripheral select control for PC4"]
    pub pc4_sel: PC4_SEL,
    #[doc = "0x54 - Peripheral select control for PC5"]
    pub pc5_sel: PC5_SEL,
    #[doc = "0x58 - Peripheral select control for PC6"]
    pub pc6_sel: PC6_SEL,
    #[doc = "0x5c - Peripheral select control for PC7"]
    pub pc7_sel: PC7_SEL,
    #[doc = "0x60 - Peripheral select control for PD0"]
    pub pd0_sel: PD0_SEL,
    #[doc = "0x64 - Peripheral select control for PD1"]
    pub pd1_sel: PD1_SEL,
    #[doc = "0x68 - Peripheral select control for PD2"]
    pub pd2_sel: PD2_SEL,
    #[doc = "0x6c - Peripheral select control for PD3"]
    pub pd3_sel: PD3_SEL,
    #[doc = "0x70 - Peripheral select control for PD4"]
    pub pd4_sel: PD4_SEL,
    #[doc = "0x74 - Peripheral select control for PD5"]
    pub pd5_sel: PD5_SEL,
    #[doc = "0x78 - Peripheral select control for PD6"]
    pub pd6_sel: PD6_SEL,
    #[doc = "0x7c - Peripheral select control for PD7"]
    pub pd7_sel: PD7_SEL,
    #[doc = "0x80 - This is the overide configuration register for each pad."]
    pub pa0_over: PA0_OVER,
    #[doc = "0x84 - This is the overide configuration register for each pad."]
    pub pa1_over: PA1_OVER,
    #[doc = "0x88 - This is the overide configuration register for each pad."]
    pub pa2_over: PA2_OVER,
    #[doc = "0x8c - This is the overide configuration register for each pad."]
    pub pa3_over: PA3_OVER,
    #[doc = "0x90 - This is the overide configuration register for each pad."]
    pub pa4_over: PA4_OVER,
    #[doc = "0x94 - This is the overide configuration register for each pad."]
    pub pa5_over: PA5_OVER,
    #[doc = "0x98 - This is the overide configuration register for each pad."]
    pub pa6_over: PA6_OVER,
    #[doc = "0x9c - This is the overide configuration register for each pad."]
    pub pa7_over: PA7_OVER,
    #[doc = "0xa0 - This is the overide configuration register for each pad."]
    pub pb0_over: PB0_OVER,
    #[doc = "0xa4 - This is the overide configuration register for each pad."]
    pub pb1_over: PB1_OVER,
    #[doc = "0xa8 - This is the overide configuration register for each pad."]
    pub pb2_over: PB2_OVER,
    #[doc = "0xac - This is the overide configuration register for each pad."]
    pub pb3_over: PB3_OVER,
    #[doc = "0xb0 - This is the overide configuration register for each pad."]
    pub pb4_over: PB4_OVER,
    #[doc = "0xb4 - This is the overide configuration register for each pad."]
    pub pb5_over: PB5_OVER,
    #[doc = "0xb8 - This is the overide configuration register for each pad."]
    pub pb6_over: PB6_OVER,
    #[doc = "0xbc - This is the overide configuration register for each pad."]
    pub pb7_over: PB7_OVER,
    #[doc = "0xc0 - This is the overide configuration register for each pad. PC0 has high drive capability."]
    pub pc0_over: PC0_OVER,
    #[doc = "0xc4 - This is the overide configuration register for each pad. PC1 has high drive capability."]
    pub pc1_over: PC1_OVER,
    #[doc = "0xc8 - This is the overide configuration register for each pad. PC2 has high drive capability."]
    pub pc2_over: PC2_OVER,
    #[doc = "0xcc - This is the overide configuration register for each pad. PC3 has high drive capability."]
    pub pc3_over: PC3_OVER,
    #[doc = "0xd0 - This is the overide configuration register for each pad."]
    pub pc4_over: PC4_OVER,
    #[doc = "0xd4 - This is the overide configuration register for each pad."]
    pub pc5_over: PC5_OVER,
    #[doc = "0xd8 - This is the overide configuration register for each pad."]
    pub pc6_over: PC6_OVER,
    #[doc = "0xdc - This is the overide configuration register for each pad."]
    pub pc7_over: PC7_OVER,
    #[doc = "0xe0 - This is the overide configuration register for each pad."]
    pub pd0_over: PD0_OVER,
    #[doc = "0xe4 - This is the overide configuration register for each pad."]
    pub pd1_over: PD1_OVER,
    #[doc = "0xe8 - This is the overide configuration register for each pad."]
    pub pd2_over: PD2_OVER,
    #[doc = "0xec - This is the overide configuration register for each pad."]
    pub pd3_over: PD3_OVER,
    #[doc = "0xf0 - This is the overide configuration register for each pad."]
    pub pd4_over: PD4_OVER,
    #[doc = "0xf4 - This is the overide configuration register for each pad."]
    pub pd5_over: PD5_OVER,
    #[doc = "0xf8 - This is the overide configuration register for each pad."]
    pub pd6_over: PD6_OVER,
    #[doc = "0xfc - This is the overide configuration register for each pad."]
    pub pd7_over: PD7_OVER,
    #[doc = "0x100 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
    pub uartrxd_uart0: UARTRXD_UART0,
    #[doc = "0x104 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
    pub uartcts_uart1: UARTCTS_UART1,
    #[doc = "0x108 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
    pub uartrxd_uart1: UARTRXD_UART1,
    #[doc = "0x10c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
    pub clk_ssi_ssi0: CLK_SSI_SSI0,
    #[doc = "0x110 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
    pub ssirxd_ssi0: SSIRXD_SSI0,
    #[doc = "0x114 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
    pub ssifssin_ssi0: SSIFSSIN_SSI0,
    #[doc = "0x118 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
    pub clk_ssiin_ssi0: CLK_SSIIN_SSI0,
    #[doc = "0x11c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
    pub clk_ssi_ssi1: CLK_SSI_SSI1,
    #[doc = "0x120 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
    pub ssirxd_ssi1: SSIRXD_SSI1,
    #[doc = "0x124 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
    pub ssifssin_ssi1: SSIFSSIN_SSI1,
    #[doc = "0x128 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
    pub clk_ssiin_ssi1: CLK_SSIIN_SSI1,
    #[doc = "0x12c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
    pub i2cmssda: I2CMSSDA,
    #[doc = "0x130 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
    pub i2cmsscl: I2CMSSCL,
    #[doc = "0x134 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
    pub gpt0ocp1: GPT0OCP1,
    #[doc = "0x138 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
    pub gpt0ocp2: GPT0OCP2,
    #[doc = "0x13c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
    pub gpt1ocp1: GPT1OCP1,
    #[doc = "0x140 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
    pub gpt1ocp2: GPT1OCP2,
    #[doc = "0x144 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
    pub gpt2ocp1: GPT2OCP1,
    #[doc = "0x148 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
    pub gpt2ocp2: GPT2OCP2,
    #[doc = "0x14c - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
    pub gpt3ocp1: GPT3OCP1,
    #[doc = "0x150 - Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
    pub gpt3ocp2: GPT3OCP2,
}
#[doc = "Peripheral select control for PA0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa0_sel](pa0_sel) module"]
pub type PA0_SEL = crate::Reg<u32, _PA0_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA0_SEL;
#[doc = "`read()` method returns [pa0_sel::R](pa0_sel::R) reader structure"]
impl crate::Readable for PA0_SEL {}
#[doc = "`write(|w| ..)` method takes [pa0_sel::W](pa0_sel::W) writer structure"]
impl crate::Writable for PA0_SEL {}
#[doc = "Peripheral select control for PA0"]
pub mod pa0_sel;
#[doc = "Peripheral select control for PA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa1_sel](pa1_sel) module"]
pub type PA1_SEL = crate::Reg<u32, _PA1_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA1_SEL;
#[doc = "`read()` method returns [pa1_sel::R](pa1_sel::R) reader structure"]
impl crate::Readable for PA1_SEL {}
#[doc = "`write(|w| ..)` method takes [pa1_sel::W](pa1_sel::W) writer structure"]
impl crate::Writable for PA1_SEL {}
#[doc = "Peripheral select control for PA1"]
pub mod pa1_sel;
#[doc = "Peripheral select control for PA2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa2_sel](pa2_sel) module"]
pub type PA2_SEL = crate::Reg<u32, _PA2_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA2_SEL;
#[doc = "`read()` method returns [pa2_sel::R](pa2_sel::R) reader structure"]
impl crate::Readable for PA2_SEL {}
#[doc = "`write(|w| ..)` method takes [pa2_sel::W](pa2_sel::W) writer structure"]
impl crate::Writable for PA2_SEL {}
#[doc = "Peripheral select control for PA2"]
pub mod pa2_sel;
#[doc = "Peripheral select control for PA3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa3_sel](pa3_sel) module"]
pub type PA3_SEL = crate::Reg<u32, _PA3_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA3_SEL;
#[doc = "`read()` method returns [pa3_sel::R](pa3_sel::R) reader structure"]
impl crate::Readable for PA3_SEL {}
#[doc = "`write(|w| ..)` method takes [pa3_sel::W](pa3_sel::W) writer structure"]
impl crate::Writable for PA3_SEL {}
#[doc = "Peripheral select control for PA3"]
pub mod pa3_sel;
#[doc = "Peripheral select control for PA4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa4_sel](pa4_sel) module"]
pub type PA4_SEL = crate::Reg<u32, _PA4_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA4_SEL;
#[doc = "`read()` method returns [pa4_sel::R](pa4_sel::R) reader structure"]
impl crate::Readable for PA4_SEL {}
#[doc = "`write(|w| ..)` method takes [pa4_sel::W](pa4_sel::W) writer structure"]
impl crate::Writable for PA4_SEL {}
#[doc = "Peripheral select control for PA4"]
pub mod pa4_sel;
#[doc = "Peripheral select control for PA5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa5_sel](pa5_sel) module"]
pub type PA5_SEL = crate::Reg<u32, _PA5_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA5_SEL;
#[doc = "`read()` method returns [pa5_sel::R](pa5_sel::R) reader structure"]
impl crate::Readable for PA5_SEL {}
#[doc = "`write(|w| ..)` method takes [pa5_sel::W](pa5_sel::W) writer structure"]
impl crate::Writable for PA5_SEL {}
#[doc = "Peripheral select control for PA5"]
pub mod pa5_sel;
#[doc = "Peripheral select control for PA6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa6_sel](pa6_sel) module"]
pub type PA6_SEL = crate::Reg<u32, _PA6_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA6_SEL;
#[doc = "`read()` method returns [pa6_sel::R](pa6_sel::R) reader structure"]
impl crate::Readable for PA6_SEL {}
#[doc = "`write(|w| ..)` method takes [pa6_sel::W](pa6_sel::W) writer structure"]
impl crate::Writable for PA6_SEL {}
#[doc = "Peripheral select control for PA6"]
pub mod pa6_sel;
#[doc = "Peripheral select control for PA7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa7_sel](pa7_sel) module"]
pub type PA7_SEL = crate::Reg<u32, _PA7_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA7_SEL;
#[doc = "`read()` method returns [pa7_sel::R](pa7_sel::R) reader structure"]
impl crate::Readable for PA7_SEL {}
#[doc = "`write(|w| ..)` method takes [pa7_sel::W](pa7_sel::W) writer structure"]
impl crate::Writable for PA7_SEL {}
#[doc = "Peripheral select control for PA7"]
pub mod pa7_sel;
#[doc = "Peripheral select control for PB0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb0_sel](pb0_sel) module"]
pub type PB0_SEL = crate::Reg<u32, _PB0_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB0_SEL;
#[doc = "`read()` method returns [pb0_sel::R](pb0_sel::R) reader structure"]
impl crate::Readable for PB0_SEL {}
#[doc = "`write(|w| ..)` method takes [pb0_sel::W](pb0_sel::W) writer structure"]
impl crate::Writable for PB0_SEL {}
#[doc = "Peripheral select control for PB0"]
pub mod pb0_sel;
#[doc = "Peripheral select control for PB1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb1_sel](pb1_sel) module"]
pub type PB1_SEL = crate::Reg<u32, _PB1_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB1_SEL;
#[doc = "`read()` method returns [pb1_sel::R](pb1_sel::R) reader structure"]
impl crate::Readable for PB1_SEL {}
#[doc = "`write(|w| ..)` method takes [pb1_sel::W](pb1_sel::W) writer structure"]
impl crate::Writable for PB1_SEL {}
#[doc = "Peripheral select control for PB1"]
pub mod pb1_sel;
#[doc = "Peripheral select control for PB2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb2_sel](pb2_sel) module"]
pub type PB2_SEL = crate::Reg<u32, _PB2_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB2_SEL;
#[doc = "`read()` method returns [pb2_sel::R](pb2_sel::R) reader structure"]
impl crate::Readable for PB2_SEL {}
#[doc = "`write(|w| ..)` method takes [pb2_sel::W](pb2_sel::W) writer structure"]
impl crate::Writable for PB2_SEL {}
#[doc = "Peripheral select control for PB2"]
pub mod pb2_sel;
#[doc = "Peripheral select control for PB3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb3_sel](pb3_sel) module"]
pub type PB3_SEL = crate::Reg<u32, _PB3_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB3_SEL;
#[doc = "`read()` method returns [pb3_sel::R](pb3_sel::R) reader structure"]
impl crate::Readable for PB3_SEL {}
#[doc = "`write(|w| ..)` method takes [pb3_sel::W](pb3_sel::W) writer structure"]
impl crate::Writable for PB3_SEL {}
#[doc = "Peripheral select control for PB3"]
pub mod pb3_sel;
#[doc = "Peripheral select control for PB4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb4_sel](pb4_sel) module"]
pub type PB4_SEL = crate::Reg<u32, _PB4_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB4_SEL;
#[doc = "`read()` method returns [pb4_sel::R](pb4_sel::R) reader structure"]
impl crate::Readable for PB4_SEL {}
#[doc = "`write(|w| ..)` method takes [pb4_sel::W](pb4_sel::W) writer structure"]
impl crate::Writable for PB4_SEL {}
#[doc = "Peripheral select control for PB4"]
pub mod pb4_sel;
#[doc = "Peripheral select control for PB5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb5_sel](pb5_sel) module"]
pub type PB5_SEL = crate::Reg<u32, _PB5_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB5_SEL;
#[doc = "`read()` method returns [pb5_sel::R](pb5_sel::R) reader structure"]
impl crate::Readable for PB5_SEL {}
#[doc = "`write(|w| ..)` method takes [pb5_sel::W](pb5_sel::W) writer structure"]
impl crate::Writable for PB5_SEL {}
#[doc = "Peripheral select control for PB5"]
pub mod pb5_sel;
#[doc = "Peripheral select control for PB6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb6_sel](pb6_sel) module"]
pub type PB6_SEL = crate::Reg<u32, _PB6_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB6_SEL;
#[doc = "`read()` method returns [pb6_sel::R](pb6_sel::R) reader structure"]
impl crate::Readable for PB6_SEL {}
#[doc = "`write(|w| ..)` method takes [pb6_sel::W](pb6_sel::W) writer structure"]
impl crate::Writable for PB6_SEL {}
#[doc = "Peripheral select control for PB6"]
pub mod pb6_sel;
#[doc = "Peripheral select control for PB7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb7_sel](pb7_sel) module"]
pub type PB7_SEL = crate::Reg<u32, _PB7_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB7_SEL;
#[doc = "`read()` method returns [pb7_sel::R](pb7_sel::R) reader structure"]
impl crate::Readable for PB7_SEL {}
#[doc = "`write(|w| ..)` method takes [pb7_sel::W](pb7_sel::W) writer structure"]
impl crate::Writable for PB7_SEL {}
#[doc = "Peripheral select control for PB7"]
pub mod pb7_sel;
#[doc = "Peripheral select control for PC0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc0_sel](pc0_sel) module"]
pub type PC0_SEL = crate::Reg<u32, _PC0_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC0_SEL;
#[doc = "`read()` method returns [pc0_sel::R](pc0_sel::R) reader structure"]
impl crate::Readable for PC0_SEL {}
#[doc = "`write(|w| ..)` method takes [pc0_sel::W](pc0_sel::W) writer structure"]
impl crate::Writable for PC0_SEL {}
#[doc = "Peripheral select control for PC0"]
pub mod pc0_sel;
#[doc = "Peripheral select control for PC1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc1_sel](pc1_sel) module"]
pub type PC1_SEL = crate::Reg<u32, _PC1_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC1_SEL;
#[doc = "`read()` method returns [pc1_sel::R](pc1_sel::R) reader structure"]
impl crate::Readable for PC1_SEL {}
#[doc = "`write(|w| ..)` method takes [pc1_sel::W](pc1_sel::W) writer structure"]
impl crate::Writable for PC1_SEL {}
#[doc = "Peripheral select control for PC1"]
pub mod pc1_sel;
#[doc = "Peripheral select control for PC2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc2_sel](pc2_sel) module"]
pub type PC2_SEL = crate::Reg<u32, _PC2_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC2_SEL;
#[doc = "`read()` method returns [pc2_sel::R](pc2_sel::R) reader structure"]
impl crate::Readable for PC2_SEL {}
#[doc = "`write(|w| ..)` method takes [pc2_sel::W](pc2_sel::W) writer structure"]
impl crate::Writable for PC2_SEL {}
#[doc = "Peripheral select control for PC2"]
pub mod pc2_sel;
#[doc = "Peripheral select control for PC3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc3_sel](pc3_sel) module"]
pub type PC3_SEL = crate::Reg<u32, _PC3_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC3_SEL;
#[doc = "`read()` method returns [pc3_sel::R](pc3_sel::R) reader structure"]
impl crate::Readable for PC3_SEL {}
#[doc = "`write(|w| ..)` method takes [pc3_sel::W](pc3_sel::W) writer structure"]
impl crate::Writable for PC3_SEL {}
#[doc = "Peripheral select control for PC3"]
pub mod pc3_sel;
#[doc = "Peripheral select control for PC4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc4_sel](pc4_sel) module"]
pub type PC4_SEL = crate::Reg<u32, _PC4_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC4_SEL;
#[doc = "`read()` method returns [pc4_sel::R](pc4_sel::R) reader structure"]
impl crate::Readable for PC4_SEL {}
#[doc = "`write(|w| ..)` method takes [pc4_sel::W](pc4_sel::W) writer structure"]
impl crate::Writable for PC4_SEL {}
#[doc = "Peripheral select control for PC4"]
pub mod pc4_sel;
#[doc = "Peripheral select control for PC5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc5_sel](pc5_sel) module"]
pub type PC5_SEL = crate::Reg<u32, _PC5_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC5_SEL;
#[doc = "`read()` method returns [pc5_sel::R](pc5_sel::R) reader structure"]
impl crate::Readable for PC5_SEL {}
#[doc = "`write(|w| ..)` method takes [pc5_sel::W](pc5_sel::W) writer structure"]
impl crate::Writable for PC5_SEL {}
#[doc = "Peripheral select control for PC5"]
pub mod pc5_sel;
#[doc = "Peripheral select control for PC6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc6_sel](pc6_sel) module"]
pub type PC6_SEL = crate::Reg<u32, _PC6_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC6_SEL;
#[doc = "`read()` method returns [pc6_sel::R](pc6_sel::R) reader structure"]
impl crate::Readable for PC6_SEL {}
#[doc = "`write(|w| ..)` method takes [pc6_sel::W](pc6_sel::W) writer structure"]
impl crate::Writable for PC6_SEL {}
#[doc = "Peripheral select control for PC6"]
pub mod pc6_sel;
#[doc = "Peripheral select control for PC7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc7_sel](pc7_sel) module"]
pub type PC7_SEL = crate::Reg<u32, _PC7_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC7_SEL;
#[doc = "`read()` method returns [pc7_sel::R](pc7_sel::R) reader structure"]
impl crate::Readable for PC7_SEL {}
#[doc = "`write(|w| ..)` method takes [pc7_sel::W](pc7_sel::W) writer structure"]
impl crate::Writable for PC7_SEL {}
#[doc = "Peripheral select control for PC7"]
pub mod pc7_sel;
#[doc = "Peripheral select control for PD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd0_sel](pd0_sel) module"]
pub type PD0_SEL = crate::Reg<u32, _PD0_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD0_SEL;
#[doc = "`read()` method returns [pd0_sel::R](pd0_sel::R) reader structure"]
impl crate::Readable for PD0_SEL {}
#[doc = "`write(|w| ..)` method takes [pd0_sel::W](pd0_sel::W) writer structure"]
impl crate::Writable for PD0_SEL {}
#[doc = "Peripheral select control for PD0"]
pub mod pd0_sel;
#[doc = "Peripheral select control for PD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd1_sel](pd1_sel) module"]
pub type PD1_SEL = crate::Reg<u32, _PD1_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD1_SEL;
#[doc = "`read()` method returns [pd1_sel::R](pd1_sel::R) reader structure"]
impl crate::Readable for PD1_SEL {}
#[doc = "`write(|w| ..)` method takes [pd1_sel::W](pd1_sel::W) writer structure"]
impl crate::Writable for PD1_SEL {}
#[doc = "Peripheral select control for PD1"]
pub mod pd1_sel;
#[doc = "Peripheral select control for PD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd2_sel](pd2_sel) module"]
pub type PD2_SEL = crate::Reg<u32, _PD2_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD2_SEL;
#[doc = "`read()` method returns [pd2_sel::R](pd2_sel::R) reader structure"]
impl crate::Readable for PD2_SEL {}
#[doc = "`write(|w| ..)` method takes [pd2_sel::W](pd2_sel::W) writer structure"]
impl crate::Writable for PD2_SEL {}
#[doc = "Peripheral select control for PD2"]
pub mod pd2_sel;
#[doc = "Peripheral select control for PD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd3_sel](pd3_sel) module"]
pub type PD3_SEL = crate::Reg<u32, _PD3_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD3_SEL;
#[doc = "`read()` method returns [pd3_sel::R](pd3_sel::R) reader structure"]
impl crate::Readable for PD3_SEL {}
#[doc = "`write(|w| ..)` method takes [pd3_sel::W](pd3_sel::W) writer structure"]
impl crate::Writable for PD3_SEL {}
#[doc = "Peripheral select control for PD3"]
pub mod pd3_sel;
#[doc = "Peripheral select control for PD4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd4_sel](pd4_sel) module"]
pub type PD4_SEL = crate::Reg<u32, _PD4_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD4_SEL;
#[doc = "`read()` method returns [pd4_sel::R](pd4_sel::R) reader structure"]
impl crate::Readable for PD4_SEL {}
#[doc = "`write(|w| ..)` method takes [pd4_sel::W](pd4_sel::W) writer structure"]
impl crate::Writable for PD4_SEL {}
#[doc = "Peripheral select control for PD4"]
pub mod pd4_sel;
#[doc = "Peripheral select control for PD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd5_sel](pd5_sel) module"]
pub type PD5_SEL = crate::Reg<u32, _PD5_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD5_SEL;
#[doc = "`read()` method returns [pd5_sel::R](pd5_sel::R) reader structure"]
impl crate::Readable for PD5_SEL {}
#[doc = "`write(|w| ..)` method takes [pd5_sel::W](pd5_sel::W) writer structure"]
impl crate::Writable for PD5_SEL {}
#[doc = "Peripheral select control for PD5"]
pub mod pd5_sel;
#[doc = "Peripheral select control for PD6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd6_sel](pd6_sel) module"]
pub type PD6_SEL = crate::Reg<u32, _PD6_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD6_SEL;
#[doc = "`read()` method returns [pd6_sel::R](pd6_sel::R) reader structure"]
impl crate::Readable for PD6_SEL {}
#[doc = "`write(|w| ..)` method takes [pd6_sel::W](pd6_sel::W) writer structure"]
impl crate::Writable for PD6_SEL {}
#[doc = "Peripheral select control for PD6"]
pub mod pd6_sel;
#[doc = "Peripheral select control for PD7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd7_sel](pd7_sel) module"]
pub type PD7_SEL = crate::Reg<u32, _PD7_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD7_SEL;
#[doc = "`read()` method returns [pd7_sel::R](pd7_sel::R) reader structure"]
impl crate::Readable for PD7_SEL {}
#[doc = "`write(|w| ..)` method takes [pd7_sel::W](pd7_sel::W) writer structure"]
impl crate::Writable for PD7_SEL {}
#[doc = "Peripheral select control for PD7"]
pub mod pd7_sel;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa0_over](pa0_over) module"]
pub type PA0_OVER = crate::Reg<u32, _PA0_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA0_OVER;
#[doc = "`read()` method returns [pa0_over::R](pa0_over::R) reader structure"]
impl crate::Readable for PA0_OVER {}
#[doc = "`write(|w| ..)` method takes [pa0_over::W](pa0_over::W) writer structure"]
impl crate::Writable for PA0_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa0_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa1_over](pa1_over) module"]
pub type PA1_OVER = crate::Reg<u32, _PA1_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA1_OVER;
#[doc = "`read()` method returns [pa1_over::R](pa1_over::R) reader structure"]
impl crate::Readable for PA1_OVER {}
#[doc = "`write(|w| ..)` method takes [pa1_over::W](pa1_over::W) writer structure"]
impl crate::Writable for PA1_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa1_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa2_over](pa2_over) module"]
pub type PA2_OVER = crate::Reg<u32, _PA2_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA2_OVER;
#[doc = "`read()` method returns [pa2_over::R](pa2_over::R) reader structure"]
impl crate::Readable for PA2_OVER {}
#[doc = "`write(|w| ..)` method takes [pa2_over::W](pa2_over::W) writer structure"]
impl crate::Writable for PA2_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa2_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa3_over](pa3_over) module"]
pub type PA3_OVER = crate::Reg<u32, _PA3_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA3_OVER;
#[doc = "`read()` method returns [pa3_over::R](pa3_over::R) reader structure"]
impl crate::Readable for PA3_OVER {}
#[doc = "`write(|w| ..)` method takes [pa3_over::W](pa3_over::W) writer structure"]
impl crate::Writable for PA3_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa3_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa4_over](pa4_over) module"]
pub type PA4_OVER = crate::Reg<u32, _PA4_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA4_OVER;
#[doc = "`read()` method returns [pa4_over::R](pa4_over::R) reader structure"]
impl crate::Readable for PA4_OVER {}
#[doc = "`write(|w| ..)` method takes [pa4_over::W](pa4_over::W) writer structure"]
impl crate::Writable for PA4_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa4_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa5_over](pa5_over) module"]
pub type PA5_OVER = crate::Reg<u32, _PA5_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA5_OVER;
#[doc = "`read()` method returns [pa5_over::R](pa5_over::R) reader structure"]
impl crate::Readable for PA5_OVER {}
#[doc = "`write(|w| ..)` method takes [pa5_over::W](pa5_over::W) writer structure"]
impl crate::Writable for PA5_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa5_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa6_over](pa6_over) module"]
pub type PA6_OVER = crate::Reg<u32, _PA6_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA6_OVER;
#[doc = "`read()` method returns [pa6_over::R](pa6_over::R) reader structure"]
impl crate::Readable for PA6_OVER {}
#[doc = "`write(|w| ..)` method takes [pa6_over::W](pa6_over::W) writer structure"]
impl crate::Writable for PA6_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa6_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pa7_over](pa7_over) module"]
pub type PA7_OVER = crate::Reg<u32, _PA7_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA7_OVER;
#[doc = "`read()` method returns [pa7_over::R](pa7_over::R) reader structure"]
impl crate::Readable for PA7_OVER {}
#[doc = "`write(|w| ..)` method takes [pa7_over::W](pa7_over::W) writer structure"]
impl crate::Writable for PA7_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pa7_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb0_over](pb0_over) module"]
pub type PB0_OVER = crate::Reg<u32, _PB0_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB0_OVER;
#[doc = "`read()` method returns [pb0_over::R](pb0_over::R) reader structure"]
impl crate::Readable for PB0_OVER {}
#[doc = "`write(|w| ..)` method takes [pb0_over::W](pb0_over::W) writer structure"]
impl crate::Writable for PB0_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb0_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb1_over](pb1_over) module"]
pub type PB1_OVER = crate::Reg<u32, _PB1_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB1_OVER;
#[doc = "`read()` method returns [pb1_over::R](pb1_over::R) reader structure"]
impl crate::Readable for PB1_OVER {}
#[doc = "`write(|w| ..)` method takes [pb1_over::W](pb1_over::W) writer structure"]
impl crate::Writable for PB1_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb1_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb2_over](pb2_over) module"]
pub type PB2_OVER = crate::Reg<u32, _PB2_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB2_OVER;
#[doc = "`read()` method returns [pb2_over::R](pb2_over::R) reader structure"]
impl crate::Readable for PB2_OVER {}
#[doc = "`write(|w| ..)` method takes [pb2_over::W](pb2_over::W) writer structure"]
impl crate::Writable for PB2_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb2_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb3_over](pb3_over) module"]
pub type PB3_OVER = crate::Reg<u32, _PB3_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB3_OVER;
#[doc = "`read()` method returns [pb3_over::R](pb3_over::R) reader structure"]
impl crate::Readable for PB3_OVER {}
#[doc = "`write(|w| ..)` method takes [pb3_over::W](pb3_over::W) writer structure"]
impl crate::Writable for PB3_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb3_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb4_over](pb4_over) module"]
pub type PB4_OVER = crate::Reg<u32, _PB4_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB4_OVER;
#[doc = "`read()` method returns [pb4_over::R](pb4_over::R) reader structure"]
impl crate::Readable for PB4_OVER {}
#[doc = "`write(|w| ..)` method takes [pb4_over::W](pb4_over::W) writer structure"]
impl crate::Writable for PB4_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb4_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb5_over](pb5_over) module"]
pub type PB5_OVER = crate::Reg<u32, _PB5_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB5_OVER;
#[doc = "`read()` method returns [pb5_over::R](pb5_over::R) reader structure"]
impl crate::Readable for PB5_OVER {}
#[doc = "`write(|w| ..)` method takes [pb5_over::W](pb5_over::W) writer structure"]
impl crate::Writable for PB5_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb5_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb6_over](pb6_over) module"]
pub type PB6_OVER = crate::Reg<u32, _PB6_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB6_OVER;
#[doc = "`read()` method returns [pb6_over::R](pb6_over::R) reader structure"]
impl crate::Readable for PB6_OVER {}
#[doc = "`write(|w| ..)` method takes [pb6_over::W](pb6_over::W) writer structure"]
impl crate::Writable for PB6_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb6_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pb7_over](pb7_over) module"]
pub type PB7_OVER = crate::Reg<u32, _PB7_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB7_OVER;
#[doc = "`read()` method returns [pb7_over::R](pb7_over::R) reader structure"]
impl crate::Readable for PB7_OVER {}
#[doc = "`write(|w| ..)` method takes [pb7_over::W](pb7_over::W) writer structure"]
impl crate::Writable for PB7_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pb7_over;
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc0_over](pc0_over) module"]
pub type PC0_OVER = crate::Reg<u32, _PC0_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC0_OVER;
#[doc = "`read()` method returns [pc0_over::R](pc0_over::R) reader structure"]
impl crate::Readable for PC0_OVER {}
#[doc = "`write(|w| ..)` method takes [pc0_over::W](pc0_over::W) writer structure"]
impl crate::Writable for PC0_OVER {}
#[doc = "This is the overide configuration register for each pad. PC0 has high drive capability."]
pub mod pc0_over;
#[doc = "This is the overide configuration register for each pad. PC1 has high drive capability.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc1_over](pc1_over) module"]
pub type PC1_OVER = crate::Reg<u32, _PC1_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC1_OVER;
#[doc = "`read()` method returns [pc1_over::R](pc1_over::R) reader structure"]
impl crate::Readable for PC1_OVER {}
#[doc = "`write(|w| ..)` method takes [pc1_over::W](pc1_over::W) writer structure"]
impl crate::Writable for PC1_OVER {}
#[doc = "This is the overide configuration register for each pad. PC1 has high drive capability."]
pub mod pc1_over;
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc2_over](pc2_over) module"]
pub type PC2_OVER = crate::Reg<u32, _PC2_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC2_OVER;
#[doc = "`read()` method returns [pc2_over::R](pc2_over::R) reader structure"]
impl crate::Readable for PC2_OVER {}
#[doc = "`write(|w| ..)` method takes [pc2_over::W](pc2_over::W) writer structure"]
impl crate::Writable for PC2_OVER {}
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability."]
pub mod pc2_over;
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc3_over](pc3_over) module"]
pub type PC3_OVER = crate::Reg<u32, _PC3_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC3_OVER;
#[doc = "`read()` method returns [pc3_over::R](pc3_over::R) reader structure"]
impl crate::Readable for PC3_OVER {}
#[doc = "`write(|w| ..)` method takes [pc3_over::W](pc3_over::W) writer structure"]
impl crate::Writable for PC3_OVER {}
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability."]
pub mod pc3_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc4_over](pc4_over) module"]
pub type PC4_OVER = crate::Reg<u32, _PC4_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC4_OVER;
#[doc = "`read()` method returns [pc4_over::R](pc4_over::R) reader structure"]
impl crate::Readable for PC4_OVER {}
#[doc = "`write(|w| ..)` method takes [pc4_over::W](pc4_over::W) writer structure"]
impl crate::Writable for PC4_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pc4_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc5_over](pc5_over) module"]
pub type PC5_OVER = crate::Reg<u32, _PC5_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC5_OVER;
#[doc = "`read()` method returns [pc5_over::R](pc5_over::R) reader structure"]
impl crate::Readable for PC5_OVER {}
#[doc = "`write(|w| ..)` method takes [pc5_over::W](pc5_over::W) writer structure"]
impl crate::Writable for PC5_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pc5_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc6_over](pc6_over) module"]
pub type PC6_OVER = crate::Reg<u32, _PC6_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC6_OVER;
#[doc = "`read()` method returns [pc6_over::R](pc6_over::R) reader structure"]
impl crate::Readable for PC6_OVER {}
#[doc = "`write(|w| ..)` method takes [pc6_over::W](pc6_over::W) writer structure"]
impl crate::Writable for PC6_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pc6_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pc7_over](pc7_over) module"]
pub type PC7_OVER = crate::Reg<u32, _PC7_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC7_OVER;
#[doc = "`read()` method returns [pc7_over::R](pc7_over::R) reader structure"]
impl crate::Readable for PC7_OVER {}
#[doc = "`write(|w| ..)` method takes [pc7_over::W](pc7_over::W) writer structure"]
impl crate::Writable for PC7_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pc7_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd0_over](pd0_over) module"]
pub type PD0_OVER = crate::Reg<u32, _PD0_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD0_OVER;
#[doc = "`read()` method returns [pd0_over::R](pd0_over::R) reader structure"]
impl crate::Readable for PD0_OVER {}
#[doc = "`write(|w| ..)` method takes [pd0_over::W](pd0_over::W) writer structure"]
impl crate::Writable for PD0_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd0_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd1_over](pd1_over) module"]
pub type PD1_OVER = crate::Reg<u32, _PD1_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD1_OVER;
#[doc = "`read()` method returns [pd1_over::R](pd1_over::R) reader structure"]
impl crate::Readable for PD1_OVER {}
#[doc = "`write(|w| ..)` method takes [pd1_over::W](pd1_over::W) writer structure"]
impl crate::Writable for PD1_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd1_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd2_over](pd2_over) module"]
pub type PD2_OVER = crate::Reg<u32, _PD2_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD2_OVER;
#[doc = "`read()` method returns [pd2_over::R](pd2_over::R) reader structure"]
impl crate::Readable for PD2_OVER {}
#[doc = "`write(|w| ..)` method takes [pd2_over::W](pd2_over::W) writer structure"]
impl crate::Writable for PD2_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd2_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd3_over](pd3_over) module"]
pub type PD3_OVER = crate::Reg<u32, _PD3_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD3_OVER;
#[doc = "`read()` method returns [pd3_over::R](pd3_over::R) reader structure"]
impl crate::Readable for PD3_OVER {}
#[doc = "`write(|w| ..)` method takes [pd3_over::W](pd3_over::W) writer structure"]
impl crate::Writable for PD3_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd3_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd4_over](pd4_over) module"]
pub type PD4_OVER = crate::Reg<u32, _PD4_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD4_OVER;
#[doc = "`read()` method returns [pd4_over::R](pd4_over::R) reader structure"]
impl crate::Readable for PD4_OVER {}
#[doc = "`write(|w| ..)` method takes [pd4_over::W](pd4_over::W) writer structure"]
impl crate::Writable for PD4_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd4_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd5_over](pd5_over) module"]
pub type PD5_OVER = crate::Reg<u32, _PD5_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD5_OVER;
#[doc = "`read()` method returns [pd5_over::R](pd5_over::R) reader structure"]
impl crate::Readable for PD5_OVER {}
#[doc = "`write(|w| ..)` method takes [pd5_over::W](pd5_over::W) writer structure"]
impl crate::Writable for PD5_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd5_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd6_over](pd6_over) module"]
pub type PD6_OVER = crate::Reg<u32, _PD6_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD6_OVER;
#[doc = "`read()` method returns [pd6_over::R](pd6_over::R) reader structure"]
impl crate::Readable for PD6_OVER {}
#[doc = "`write(|w| ..)` method takes [pd6_over::W](pd6_over::W) writer structure"]
impl crate::Writable for PD6_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd6_over;
#[doc = "This is the overide configuration register for each pad.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pd7_over](pd7_over) module"]
pub type PD7_OVER = crate::Reg<u32, _PD7_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD7_OVER;
#[doc = "`read()` method returns [pd7_over::R](pd7_over::R) reader structure"]
impl crate::Readable for PD7_OVER {}
#[doc = "`write(|w| ..)` method takes [pd7_over::W](pd7_over::W) writer structure"]
impl crate::Writable for PD7_OVER {}
#[doc = "This is the overide configuration register for each pad."]
pub mod pd7_over;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartrxd_uart0](uartrxd_uart0) module"]
pub type UARTRXD_UART0 = crate::Reg<u32, _UARTRXD_UART0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRXD_UART0;
#[doc = "`read()` method returns [uartrxd_uart0::R](uartrxd_uart0::R) reader structure"]
impl crate::Readable for UARTRXD_UART0 {}
#[doc = "`write(|w| ..)` method takes [uartrxd_uart0::W](uartrxd_uart0::W) writer structure"]
impl crate::Writable for UARTRXD_UART0 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX."]
pub mod uartrxd_uart0;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartcts_uart1](uartcts_uart1) module"]
pub type UARTCTS_UART1 = crate::Reg<u32, _UARTCTS_UART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTCTS_UART1;
#[doc = "`read()` method returns [uartcts_uart1::R](uartcts_uart1::R) reader structure"]
impl crate::Readable for UARTCTS_UART1 {}
#[doc = "`write(|w| ..)` method takes [uartcts_uart1::W](uartcts_uart1::W) writer structure"]
impl crate::Writable for UARTCTS_UART1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS."]
pub mod uartcts_uart1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uartrxd_uart1](uartrxd_uart1) module"]
pub type UARTRXD_UART1 = crate::Reg<u32, _UARTRXD_UART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UARTRXD_UART1;
#[doc = "`read()` method returns [uartrxd_uart1::R](uartrxd_uart1::R) reader structure"]
impl crate::Readable for UARTRXD_UART1 {}
#[doc = "`write(|w| ..)` method takes [uartrxd_uart1::W](uartrxd_uart1::W) writer structure"]
impl crate::Writable for UARTRXD_UART1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 RX."]
pub mod uartrxd_uart1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_ssi_ssi0](clk_ssi_ssi0) module"]
pub type CLK_SSI_SSI0 = crate::Reg<u32, _CLK_SSI_SSI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SSI_SSI0;
#[doc = "`read()` method returns [clk_ssi_ssi0::R](clk_ssi_ssi0::R) reader structure"]
impl crate::Readable for CLK_SSI_SSI0 {}
#[doc = "`write(|w| ..)` method takes [clk_ssi_ssi0::W](clk_ssi_ssi0::W) writer structure"]
impl crate::Writable for CLK_SSI_SSI0 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK."]
pub mod clk_ssi_ssi0;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssirxd_ssi0](ssirxd_ssi0) module"]
pub type SSIRXD_SSI0 = crate::Reg<u32, _SSIRXD_SSI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIRXD_SSI0;
#[doc = "`read()` method returns [ssirxd_ssi0::R](ssirxd_ssi0::R) reader structure"]
impl crate::Readable for SSIRXD_SSI0 {}
#[doc = "`write(|w| ..)` method takes [ssirxd_ssi0::W](ssirxd_ssi0::W) writer structure"]
impl crate::Writable for SSIRXD_SSI0 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX."]
pub mod ssirxd_ssi0;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssifssin_ssi0](ssifssin_ssi0) module"]
pub type SSIFSSIN_SSI0 = crate::Reg<u32, _SSIFSSIN_SSI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIFSSIN_SSI0;
#[doc = "`read()` method returns [ssifssin_ssi0::R](ssifssin_ssi0::R) reader structure"]
impl crate::Readable for SSIFSSIN_SSI0 {}
#[doc = "`write(|w| ..)` method takes [ssifssin_ssi0::W](ssifssin_ssi0::W) writer structure"]
impl crate::Writable for SSIFSSIN_SSI0 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN."]
pub mod ssifssin_ssi0;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_ssiin_ssi0](clk_ssiin_ssi0) module"]
pub type CLK_SSIIN_SSI0 = crate::Reg<u32, _CLK_SSIIN_SSI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SSIIN_SSI0;
#[doc = "`read()` method returns [clk_ssiin_ssi0::R](clk_ssiin_ssi0::R) reader structure"]
impl crate::Readable for CLK_SSIIN_SSI0 {}
#[doc = "`write(|w| ..)` method takes [clk_ssiin_ssi0::W](clk_ssiin_ssi0::W) writer structure"]
impl crate::Writable for CLK_SSIIN_SSI0 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 CLK_SSIN."]
pub mod clk_ssiin_ssi0;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_ssi_ssi1](clk_ssi_ssi1) module"]
pub type CLK_SSI_SSI1 = crate::Reg<u32, _CLK_SSI_SSI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SSI_SSI1;
#[doc = "`read()` method returns [clk_ssi_ssi1::R](clk_ssi_ssi1::R) reader structure"]
impl crate::Readable for CLK_SSI_SSI1 {}
#[doc = "`write(|w| ..)` method takes [clk_ssi_ssi1::W](clk_ssi_ssi1::W) writer structure"]
impl crate::Writable for CLK_SSI_SSI1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK."]
pub mod clk_ssi_ssi1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssirxd_ssi1](ssirxd_ssi1) module"]
pub type SSIRXD_SSI1 = crate::Reg<u32, _SSIRXD_SSI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIRXD_SSI1;
#[doc = "`read()` method returns [ssirxd_ssi1::R](ssirxd_ssi1::R) reader structure"]
impl crate::Readable for SSIRXD_SSI1 {}
#[doc = "`write(|w| ..)` method takes [ssirxd_ssi1::W](ssirxd_ssi1::W) writer structure"]
impl crate::Writable for SSIRXD_SSI1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX."]
pub mod ssirxd_ssi1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssifssin_ssi1](ssifssin_ssi1) module"]
pub type SSIFSSIN_SSI1 = crate::Reg<u32, _SSIFSSIN_SSI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIFSSIN_SSI1;
#[doc = "`read()` method returns [ssifssin_ssi1::R](ssifssin_ssi1::R) reader structure"]
impl crate::Readable for SSIFSSIN_SSI1 {}
#[doc = "`write(|w| ..)` method takes [ssifssin_ssi1::W](ssifssin_ssi1::W) writer structure"]
impl crate::Writable for SSIFSSIN_SSI1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 FSSIN."]
pub mod ssifssin_ssi1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_ssiin_ssi1](clk_ssiin_ssi1) module"]
pub type CLK_SSIIN_SSI1 = crate::Reg<u32, _CLK_SSIIN_SSI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SSIIN_SSI1;
#[doc = "`read()` method returns [clk_ssiin_ssi1::R](clk_ssiin_ssi1::R) reader structure"]
impl crate::Readable for CLK_SSIIN_SSI1 {}
#[doc = "`write(|w| ..)` method takes [clk_ssiin_ssi1::W](clk_ssiin_ssi1::W) writer structure"]
impl crate::Writable for CLK_SSIIN_SSI1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN."]
pub mod clk_ssiin_ssi1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2cmssda](i2cmssda) module"]
pub type I2CMSSDA = crate::Reg<u32, _I2CMSSDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMSSDA;
#[doc = "`read()` method returns [i2cmssda::R](i2cmssda::R) reader structure"]
impl crate::Readable for I2CMSSDA {}
#[doc = "`write(|w| ..)` method takes [i2cmssda::W](i2cmssda::W) writer structure"]
impl crate::Writable for I2CMSSDA {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA."]
pub mod i2cmssda;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2cmsscl](i2cmsscl) module"]
pub type I2CMSSCL = crate::Reg<u32, _I2CMSSCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2CMSSCL;
#[doc = "`read()` method returns [i2cmsscl::R](i2cmsscl::R) reader structure"]
impl crate::Readable for I2CMSSCL {}
#[doc = "`write(|w| ..)` method takes [i2cmsscl::W](i2cmsscl::W) writer structure"]
impl crate::Writable for I2CMSSCL {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SCL."]
pub mod i2cmsscl;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt0ocp1](gpt0ocp1) module"]
pub type GPT0OCP1 = crate::Reg<u32, _GPT0OCP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT0OCP1;
#[doc = "`read()` method returns [gpt0ocp1::R](gpt0ocp1::R) reader structure"]
impl crate::Readable for GPT0OCP1 {}
#[doc = "`write(|w| ..)` method takes [gpt0ocp1::W](gpt0ocp1::W) writer structure"]
impl crate::Writable for GPT0OCP1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP1."]
pub mod gpt0ocp1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt0ocp2](gpt0ocp2) module"]
pub type GPT0OCP2 = crate::Reg<u32, _GPT0OCP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT0OCP2;
#[doc = "`read()` method returns [gpt0ocp2::R](gpt0ocp2::R) reader structure"]
impl crate::Readable for GPT0OCP2 {}
#[doc = "`write(|w| ..)` method takes [gpt0ocp2::W](gpt0ocp2::W) writer structure"]
impl crate::Writable for GPT0OCP2 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT0OCP2."]
pub mod gpt0ocp2;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt1ocp1](gpt1ocp1) module"]
pub type GPT1OCP1 = crate::Reg<u32, _GPT1OCP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1OCP1;
#[doc = "`read()` method returns [gpt1ocp1::R](gpt1ocp1::R) reader structure"]
impl crate::Readable for GPT1OCP1 {}
#[doc = "`write(|w| ..)` method takes [gpt1ocp1::W](gpt1ocp1::W) writer structure"]
impl crate::Writable for GPT1OCP1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1."]
pub mod gpt1ocp1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt1ocp2](gpt1ocp2) module"]
pub type GPT1OCP2 = crate::Reg<u32, _GPT1OCP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1OCP2;
#[doc = "`read()` method returns [gpt1ocp2::R](gpt1ocp2::R) reader structure"]
impl crate::Readable for GPT1OCP2 {}
#[doc = "`write(|w| ..)` method takes [gpt1ocp2::W](gpt1ocp2::W) writer structure"]
impl crate::Writable for GPT1OCP2 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP2."]
pub mod gpt1ocp2;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt2ocp1](gpt2ocp1) module"]
pub type GPT2OCP1 = crate::Reg<u32, _GPT2OCP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2OCP1;
#[doc = "`read()` method returns [gpt2ocp1::R](gpt2ocp1::R) reader structure"]
impl crate::Readable for GPT2OCP1 {}
#[doc = "`write(|w| ..)` method takes [gpt2ocp1::W](gpt2ocp1::W) writer structure"]
impl crate::Writable for GPT2OCP1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP1."]
pub mod gpt2ocp1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt2ocp2](gpt2ocp2) module"]
pub type GPT2OCP2 = crate::Reg<u32, _GPT2OCP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2OCP2;
#[doc = "`read()` method returns [gpt2ocp2::R](gpt2ocp2::R) reader structure"]
impl crate::Readable for GPT2OCP2 {}
#[doc = "`write(|w| ..)` method takes [gpt2ocp2::W](gpt2ocp2::W) writer structure"]
impl crate::Writable for GPT2OCP2 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT2OCP2."]
pub mod gpt2ocp2;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt3ocp1](gpt3ocp1) module"]
pub type GPT3OCP1 = crate::Reg<u32, _GPT3OCP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT3OCP1;
#[doc = "`read()` method returns [gpt3ocp1::R](gpt3ocp1::R) reader structure"]
impl crate::Readable for GPT3OCP1 {}
#[doc = "`write(|w| ..)` method takes [gpt3ocp1::W](gpt3ocp1::W) writer structure"]
impl crate::Writable for GPT3OCP1 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP1."]
pub mod gpt3ocp1;
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpt3ocp2](gpt3ocp2) module"]
pub type GPT3OCP2 = crate::Reg<u32, _GPT3OCP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT3OCP2;
#[doc = "`read()` method returns [gpt3ocp2::R](gpt3ocp2::R) reader structure"]
impl crate::Readable for GPT3OCP2 {}
#[doc = "`write(|w| ..)` method takes [gpt3ocp2::W](gpt3ocp2::W) writer structure"]
impl crate::Writable for GPT3OCP2 {}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT3OCP2."]
pub mod gpt3ocp2;
