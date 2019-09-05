#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC Timer event configuration"]
    pub mtcspcfg: MTCSPCFG,
    #[doc = "0x04 - MAC Timer control register"]
    pub mtctrl: MTCTRL,
    #[doc = "0x08 - MAC Timer interrupt mask"]
    pub mtirqm: MTIRQM,
    #[doc = "0x0c - MAC Timer interrupt flags"]
    pub mtirqf: MTIRQF,
    #[doc = "0x10 - MAC Timer multiplex select"]
    pub mtmsel: MTMSEL,
    #[doc = "0x14 - MAC Timer multiplexed register 0"]
    pub mtm0: MTM0,
    #[doc = "0x18 - MAC Timer multiplexed register 1"]
    pub mtm1: MTM1,
    #[doc = "0x1c - MAC Timer multiplexed overflow register 2"]
    pub mtmovf2: MTMOVF2,
    #[doc = "0x20 - MAC Timer multiplexed overflow register 1"]
    pub mtmovf1: MTMOVF1,
    #[doc = "0x24 - MAC Timer multiplexed overflow register 0"]
    pub mtmovf0: MTMOVF0,
    #[doc = "0x28 - The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
    pub rfdata: RFDATA,
    #[doc = "0x2c - RF error interrupt flags"]
    pub rferrf: RFERRF,
    #[doc = "0x30 - RF interrupt flags"]
    pub rfirqf1: RFIRQF1,
    #[doc = "0x34 - RF interrupt flags"]
    pub rfirqf0: RFIRQF0,
    #[doc = "0x38 - RF CSMA-CA/strobe processor"]
    pub rfst: RFST,
}
#[doc = "MAC Timer event configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtcspcfg](mtcspcfg) module"]
pub type MTCSPCFG = crate::Reg<u32, _MTCSPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTCSPCFG;
#[doc = "`read()` method returns [mtcspcfg::R](mtcspcfg::R) reader structure"]
impl crate::Readable for MTCSPCFG {}
#[doc = "`write(|w| ..)` method takes [mtcspcfg::W](mtcspcfg::W) writer structure"]
impl crate::Writable for MTCSPCFG {}
#[doc = "MAC Timer event configuration"]
pub mod mtcspcfg;
#[doc = "MAC Timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtctrl](mtctrl) module"]
pub type MTCTRL = crate::Reg<u32, _MTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTCTRL;
#[doc = "`read()` method returns [mtctrl::R](mtctrl::R) reader structure"]
impl crate::Readable for MTCTRL {}
#[doc = "`write(|w| ..)` method takes [mtctrl::W](mtctrl::W) writer structure"]
impl crate::Writable for MTCTRL {}
#[doc = "MAC Timer control register"]
pub mod mtctrl;
#[doc = "MAC Timer interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtirqm](mtirqm) module"]
pub type MTIRQM = crate::Reg<u32, _MTIRQM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIRQM;
#[doc = "`read()` method returns [mtirqm::R](mtirqm::R) reader structure"]
impl crate::Readable for MTIRQM {}
#[doc = "`write(|w| ..)` method takes [mtirqm::W](mtirqm::W) writer structure"]
impl crate::Writable for MTIRQM {}
#[doc = "MAC Timer interrupt mask"]
pub mod mtirqm;
#[doc = "MAC Timer interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtirqf](mtirqf) module"]
pub type MTIRQF = crate::Reg<u32, _MTIRQF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIRQF;
#[doc = "`read()` method returns [mtirqf::R](mtirqf::R) reader structure"]
impl crate::Readable for MTIRQF {}
#[doc = "`write(|w| ..)` method takes [mtirqf::W](mtirqf::W) writer structure"]
impl crate::Writable for MTIRQF {}
#[doc = "MAC Timer interrupt flags"]
pub mod mtirqf;
#[doc = "MAC Timer multiplex select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtmsel](mtmsel) module"]
pub type MTMSEL = crate::Reg<u32, _MTMSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTMSEL;
#[doc = "`read()` method returns [mtmsel::R](mtmsel::R) reader structure"]
impl crate::Readable for MTMSEL {}
#[doc = "`write(|w| ..)` method takes [mtmsel::W](mtmsel::W) writer structure"]
impl crate::Writable for MTMSEL {}
#[doc = "MAC Timer multiplex select"]
pub mod mtmsel;
#[doc = "MAC Timer multiplexed register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtm0](mtm0) module"]
pub type MTM0 = crate::Reg<u32, _MTM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTM0;
#[doc = "`read()` method returns [mtm0::R](mtm0::R) reader structure"]
impl crate::Readable for MTM0 {}
#[doc = "`write(|w| ..)` method takes [mtm0::W](mtm0::W) writer structure"]
impl crate::Writable for MTM0 {}
#[doc = "MAC Timer multiplexed register 0"]
pub mod mtm0;
#[doc = "MAC Timer multiplexed register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtm1](mtm1) module"]
pub type MTM1 = crate::Reg<u32, _MTM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTM1;
#[doc = "`read()` method returns [mtm1::R](mtm1::R) reader structure"]
impl crate::Readable for MTM1 {}
#[doc = "`write(|w| ..)` method takes [mtm1::W](mtm1::W) writer structure"]
impl crate::Writable for MTM1 {}
#[doc = "MAC Timer multiplexed register 1"]
pub mod mtm1;
#[doc = "MAC Timer multiplexed overflow register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtmovf2](mtmovf2) module"]
pub type MTMOVF2 = crate::Reg<u32, _MTMOVF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTMOVF2;
#[doc = "`read()` method returns [mtmovf2::R](mtmovf2::R) reader structure"]
impl crate::Readable for MTMOVF2 {}
#[doc = "`write(|w| ..)` method takes [mtmovf2::W](mtmovf2::W) writer structure"]
impl crate::Writable for MTMOVF2 {}
#[doc = "MAC Timer multiplexed overflow register 2"]
pub mod mtmovf2;
#[doc = "MAC Timer multiplexed overflow register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtmovf1](mtmovf1) module"]
pub type MTMOVF1 = crate::Reg<u32, _MTMOVF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTMOVF1;
#[doc = "`read()` method returns [mtmovf1::R](mtmovf1::R) reader structure"]
impl crate::Readable for MTMOVF1 {}
#[doc = "`write(|w| ..)` method takes [mtmovf1::W](mtmovf1::W) writer structure"]
impl crate::Writable for MTMOVF1 {}
#[doc = "MAC Timer multiplexed overflow register 1"]
pub mod mtmovf1;
#[doc = "MAC Timer multiplexed overflow register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mtmovf0](mtmovf0) module"]
pub type MTMOVF0 = crate::Reg<u32, _MTMOVF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTMOVF0;
#[doc = "`read()` method returns [mtmovf0::R](mtmovf0::R) reader structure"]
impl crate::Readable for MTMOVF0 {}
#[doc = "`write(|w| ..)` method takes [mtmovf0::W](mtmovf0::W) writer structure"]
impl crate::Writable for MTMOVF0 {}
#[doc = "MAC Timer multiplexed overflow register 0"]
pub mod mtmovf0;
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfdata](rfdata) module"]
pub type RFDATA = crate::Reg<u32, _RFDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFDATA;
#[doc = "`read()` method returns [rfdata::R](rfdata::R) reader structure"]
impl crate::Readable for RFDATA {}
#[doc = "`write(|w| ..)` method takes [rfdata::W](rfdata::W) writer structure"]
impl crate::Writable for RFDATA {}
#[doc = "The TX FIFO and RX FIFO may be accessed through this register. Data is written to the TX FIFO when writing to the RFD register. Data is read from the RX FIFO when the RFD register is read. The XREG registers RXFIFOCNT and TXFIFOCNT provide information on the amount of data in the FIFOs. The FIFO contents can be cleared by issuing SFLUSHRX and SFLUSHTX."]
pub mod rfdata;
#[doc = "RF error interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rferrf](rferrf) module"]
pub type RFERRF = crate::Reg<u32, _RFERRF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFERRF;
#[doc = "`read()` method returns [rferrf::R](rferrf::R) reader structure"]
impl crate::Readable for RFERRF {}
#[doc = "`write(|w| ..)` method takes [rferrf::W](rferrf::W) writer structure"]
impl crate::Writable for RFERRF {}
#[doc = "RF error interrupt flags"]
pub mod rferrf;
#[doc = "RF interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfirqf1](rfirqf1) module"]
pub type RFIRQF1 = crate::Reg<u32, _RFIRQF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIRQF1;
#[doc = "`read()` method returns [rfirqf1::R](rfirqf1::R) reader structure"]
impl crate::Readable for RFIRQF1 {}
#[doc = "`write(|w| ..)` method takes [rfirqf1::W](rfirqf1::W) writer structure"]
impl crate::Writable for RFIRQF1 {}
#[doc = "RF interrupt flags"]
pub mod rfirqf1;
#[doc = "RF interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfirqf0](rfirqf0) module"]
pub type RFIRQF0 = crate::Reg<u32, _RFIRQF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIRQF0;
#[doc = "`read()` method returns [rfirqf0::R](rfirqf0::R) reader structure"]
impl crate::Readable for RFIRQF0 {}
#[doc = "`write(|w| ..)` method takes [rfirqf0::W](rfirqf0::W) writer structure"]
impl crate::Writable for RFIRQF0 {}
#[doc = "RF interrupt flags"]
pub mod rfirqf0;
#[doc = "RF CSMA-CA/strobe processor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfst](rfst) module"]
pub type RFST = crate::Reg<u32, _RFST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFST;
#[doc = "`read()` method returns [rfst::R](rfst::R) reader structure"]
impl crate::Readable for RFST {}
#[doc = "`write(|w| ..)` method takes [rfst::W](rfst::W) writer structure"]
impl crate::Writable for RFST {}
#[doc = "RF CSMA-CA/strobe processor"]
pub mod rfst;
