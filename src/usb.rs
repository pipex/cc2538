#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Function address"]
    pub addr: ADDR,
    #[doc = "0x04 - Power management and control register"]
    pub pow: POW,
    #[doc = "0x08 - Interrupt flags for endpoint 0 and IN endpoints 1-5"]
    pub iif: IIF,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Interrupt flags for OUT endpoints 1-5"]
    pub oif: OIF,
    _reserved4: [u8; 4usize],
    #[doc = "0x18 - Common USB interrupt flags"]
    pub cif: CIF,
    #[doc = "0x1c - Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
    pub iie: IIE,
    _reserved6: [u8; 4usize],
    #[doc = "0x24 - Interrupt enable mask for OUT endpoints 1-5"]
    pub oie: OIE,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c - Common USB interrupt enable mask"]
    pub cie: CIE,
    #[doc = "0x30 - Frame number (low byte)"]
    pub frml: FRML,
    #[doc = "0x34 - Frame number (high byte)"]
    pub frmh: FRMH,
    #[doc = "0x38 - Index register for selecting the endpoint status and control registers"]
    pub index: INDEX,
    #[doc = "0x3c - USB peripheral control register"]
    pub ctrl: CTRL,
    #[doc = "0x40 - Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
    pub maxi: MAXI,
    #[doc = "0x44 - Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
    pub cs0_csil: CS0_CSIL,
    #[doc = "0x48 - Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
    pub csih: CSIH,
    #[doc = "0x4c - Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
    pub maxo: MAXO,
    #[doc = "0x50 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
    pub csol: CSOL,
    #[doc = "0x54 - Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
    pub csoh: CSOH,
    #[doc = "0x58 - Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
    pub cnt0_cntl: CNT0_CNTL,
    #[doc = "0x5c - Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
    pub cnth: CNTH,
    _reserved20: [u8; 32usize],
    #[doc = "0x80 - Endpoint 0 FIFO"]
    pub f0: F0,
    _reserved21: [u8; 4usize],
    #[doc = "0x88 - IN/OUT endpoint 1 FIFO"]
    pub f1: F1,
    _reserved22: [u8; 4usize],
    #[doc = "0x90 - IN/OUT endpoint 2 FIFO"]
    pub f2: F2,
    _reserved23: [u8; 4usize],
    #[doc = "0x98 - IN/OUT endpoint 3 FIFO"]
    pub f3: F3,
    _reserved24: [u8; 4usize],
    #[doc = "0xa0 - IN/OUT endpoint 4 FIFO"]
    pub f4: F4,
    _reserved25: [u8; 4usize],
    #[doc = "0xa8 - IN/OUT endpoint 5 FIFO"]
    pub f5: F5,
}
#[doc = "Function address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addr](addr) module"]
pub type ADDR = crate::Reg<u32, _ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR;
#[doc = "`read()` method returns [addr::R](addr::R) reader structure"]
impl crate::Readable for ADDR {}
#[doc = "`write(|w| ..)` method takes [addr::W](addr::W) writer structure"]
impl crate::Writable for ADDR {}
#[doc = "Function address"]
pub mod addr;
#[doc = "Power management and control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pow](pow) module"]
pub type POW = crate::Reg<u32, _POW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POW;
#[doc = "`read()` method returns [pow::R](pow::R) reader structure"]
impl crate::Readable for POW {}
#[doc = "`write(|w| ..)` method takes [pow::W](pow::W) writer structure"]
impl crate::Writable for POW {}
#[doc = "Power management and control register"]
pub mod pow;
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iif](iif) module"]
pub type IIF = crate::Reg<u32, _IIF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IIF;
#[doc = "`read()` method returns [iif::R](iif::R) reader structure"]
impl crate::Readable for IIF {}
#[doc = "`write(|w| ..)` method takes [iif::W](iif::W) writer structure"]
impl crate::Writable for IIF {}
#[doc = "Interrupt flags for endpoint 0 and IN endpoints 1-5"]
pub mod iif;
#[doc = "Interrupt flags for OUT endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oif](oif) module"]
pub type OIF = crate::Reg<u32, _OIF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OIF;
#[doc = "`read()` method returns [oif::R](oif::R) reader structure"]
impl crate::Readable for OIF {}
#[doc = "`write(|w| ..)` method takes [oif::W](oif::W) writer structure"]
impl crate::Writable for OIF {}
#[doc = "Interrupt flags for OUT endpoints 1-5"]
pub mod oif;
#[doc = "Common USB interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cif](cif) module"]
pub type CIF = crate::Reg<u32, _CIF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIF;
#[doc = "`read()` method returns [cif::R](cif::R) reader structure"]
impl crate::Readable for CIF {}
#[doc = "`write(|w| ..)` method takes [cif::W](cif::W) writer structure"]
impl crate::Writable for CIF {}
#[doc = "Common USB interrupt flags"]
pub mod cif;
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iie](iie) module"]
pub type IIE = crate::Reg<u32, _IIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IIE;
#[doc = "`read()` method returns [iie::R](iie::R) reader structure"]
impl crate::Readable for IIE {}
#[doc = "`write(|w| ..)` method takes [iie::W](iie::W) writer structure"]
impl crate::Writable for IIE {}
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0"]
pub mod iie;
#[doc = "Interrupt enable mask for OUT endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oie](oie) module"]
pub type OIE = crate::Reg<u32, _OIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OIE;
#[doc = "`read()` method returns [oie::R](oie::R) reader structure"]
impl crate::Readable for OIE {}
#[doc = "`write(|w| ..)` method takes [oie::W](oie::W) writer structure"]
impl crate::Writable for OIE {}
#[doc = "Interrupt enable mask for OUT endpoints 1-5"]
pub mod oie;
#[doc = "Common USB interrupt enable mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cie](cie) module"]
pub type CIE = crate::Reg<u32, _CIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIE;
#[doc = "`read()` method returns [cie::R](cie::R) reader structure"]
impl crate::Readable for CIE {}
#[doc = "`write(|w| ..)` method takes [cie::W](cie::W) writer structure"]
impl crate::Writable for CIE {}
#[doc = "Common USB interrupt enable mask"]
pub mod cie;
#[doc = "Frame number (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frml](frml) module"]
pub type FRML = crate::Reg<u32, _FRML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRML;
#[doc = "`read()` method returns [frml::R](frml::R) reader structure"]
impl crate::Readable for FRML {}
#[doc = "`write(|w| ..)` method takes [frml::W](frml::W) writer structure"]
impl crate::Writable for FRML {}
#[doc = "Frame number (low byte)"]
pub mod frml;
#[doc = "Frame number (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmh](frmh) module"]
pub type FRMH = crate::Reg<u32, _FRMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMH;
#[doc = "`read()` method returns [frmh::R](frmh::R) reader structure"]
impl crate::Readable for FRMH {}
#[doc = "`write(|w| ..)` method takes [frmh::W](frmh::W) writer structure"]
impl crate::Writable for FRMH {}
#[doc = "Frame number (high byte)"]
pub mod frmh;
#[doc = "Index register for selecting the endpoint status and control registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [index](index) module"]
pub type INDEX = crate::Reg<u32, _INDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INDEX;
#[doc = "`read()` method returns [index::R](index::R) reader structure"]
impl crate::Readable for INDEX {}
#[doc = "`write(|w| ..)` method takes [index::W](index::W) writer structure"]
impl crate::Writable for INDEX {}
#[doc = "Index register for selecting the endpoint status and control registers"]
pub mod index;
#[doc = "USB peripheral control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "USB peripheral control register"]
pub mod ctrl;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxi](maxi) module"]
pub type MAXI = crate::Reg<u32, _MAXI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXI;
#[doc = "`read()` method returns [maxi::R](maxi::R) reader structure"]
impl crate::Readable for MAXI {}
#[doc = "`write(|w| ..)` method takes [maxi::W](maxi::W) writer structure"]
impl crate::Writable for MAXI {}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for IN endpoint {1-5}"]
pub mod maxi;
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cs0_csil](cs0_csil) module"]
pub type CS0_CSIL = crate::Reg<u32, _CS0_CSIL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS0_CSIL;
#[doc = "`read()` method returns [cs0_csil::R](cs0_csil::R) reader structure"]
impl crate::Readable for CS0_CSIL {}
#[doc = "`write(|w| ..)` method takes [cs0_csil::W](cs0_csil::W) writer structure"]
impl crate::Writable for CS0_CSIL {}
#[doc = "Indexed register: For USB_INDEX = 0: Endpoint 0 control and status For USB_INDEX = 1-5: IN endpoint {1-5} control and status (low byte)"]
pub mod cs0_csil;
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csih](csih) module"]
pub type CSIH = crate::Reg<u32, _CSIH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIH;
#[doc = "`read()` method returns [csih::R](csih::R) reader structure"]
impl crate::Readable for CSIH {}
#[doc = "`write(|w| ..)` method takes [csih::W](csih::W) writer structure"]
impl crate::Writable for CSIH {}
#[doc = "Indexed register: For USB_INDEX = 1-5: IN endpoint {1-5} control and status (high byte)"]
pub mod csih;
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxo](maxo) module"]
pub type MAXO = crate::Reg<u32, _MAXO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXO;
#[doc = "`read()` method returns [maxo::R](maxo::R) reader structure"]
impl crate::Readable for MAXO {}
#[doc = "`write(|w| ..)` method takes [maxo::W](maxo::W) writer structure"]
impl crate::Writable for MAXO {}
#[doc = "Indexed register: For USB_INDEX = 1-5: Maximum packet size for OUT endpoint {1-5}"]
pub mod maxo;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csol](csol) module"]
pub type CSOL = crate::Reg<u32, _CSOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSOL;
#[doc = "`read()` method returns [csol::R](csol::R) reader structure"]
impl crate::Readable for CSOL {}
#[doc = "`write(|w| ..)` method takes [csol::W](csol::W) writer structure"]
impl crate::Writable for CSOL {}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (low byte)"]
pub mod csol;
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csoh](csoh) module"]
pub type CSOH = crate::Reg<u32, _CSOH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSOH;
#[doc = "`read()` method returns [csoh::R](csoh::R) reader structure"]
impl crate::Readable for CSOH {}
#[doc = "`write(|w| ..)` method takes [csoh::W](csoh::W) writer structure"]
impl crate::Writable for CSOH {}
#[doc = "Indexed register: For USB_INDEX = 1-5: OUT endpoint {1-5} control and status (high byte)"]
pub mod csoh;
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt0_cntl](cnt0_cntl) module"]
pub type CNT0_CNTL = crate::Reg<u32, _CNT0_CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT0_CNTL;
#[doc = "`read()` method returns [cnt0_cntl::R](cnt0_cntl::R) reader structure"]
impl crate::Readable for CNT0_CNTL {}
#[doc = "`write(|w| ..)` method takes [cnt0_cntl::W](cnt0_cntl::W) writer structure"]
impl crate::Writable for CNT0_CNTL {}
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)"]
pub mod cnt0_cntl;
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnth](cnth) module"]
pub type CNTH = crate::Reg<u32, _CNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTH;
#[doc = "`read()` method returns [cnth::R](cnth::R) reader structure"]
impl crate::Readable for CNTH {}
#[doc = "`write(|w| ..)` method takes [cnth::W](cnth::W) writer structure"]
impl crate::Writable for CNTH {}
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)"]
pub mod cnth;
#[doc = "Endpoint 0 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f0](f0) module"]
pub type F0 = crate::Reg<u32, _F0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F0;
#[doc = "`read()` method returns [f0::R](f0::R) reader structure"]
impl crate::Readable for F0 {}
#[doc = "`write(|w| ..)` method takes [f0::W](f0::W) writer structure"]
impl crate::Writable for F0 {}
#[doc = "Endpoint 0 FIFO"]
pub mod f0;
#[doc = "IN/OUT endpoint 1 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f1](f1) module"]
pub type F1 = crate::Reg<u32, _F1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F1;
#[doc = "`read()` method returns [f1::R](f1::R) reader structure"]
impl crate::Readable for F1 {}
#[doc = "`write(|w| ..)` method takes [f1::W](f1::W) writer structure"]
impl crate::Writable for F1 {}
#[doc = "IN/OUT endpoint 1 FIFO"]
pub mod f1;
#[doc = "IN/OUT endpoint 2 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f2](f2) module"]
pub type F2 = crate::Reg<u32, _F2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F2;
#[doc = "`read()` method returns [f2::R](f2::R) reader structure"]
impl crate::Readable for F2 {}
#[doc = "`write(|w| ..)` method takes [f2::W](f2::W) writer structure"]
impl crate::Writable for F2 {}
#[doc = "IN/OUT endpoint 2 FIFO"]
pub mod f2;
#[doc = "IN/OUT endpoint 3 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f3](f3) module"]
pub type F3 = crate::Reg<u32, _F3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F3;
#[doc = "`read()` method returns [f3::R](f3::R) reader structure"]
impl crate::Readable for F3 {}
#[doc = "`write(|w| ..)` method takes [f3::W](f3::W) writer structure"]
impl crate::Writable for F3 {}
#[doc = "IN/OUT endpoint 3 FIFO"]
pub mod f3;
#[doc = "IN/OUT endpoint 4 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f4](f4) module"]
pub type F4 = crate::Reg<u32, _F4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F4;
#[doc = "`read()` method returns [f4::R](f4::R) reader structure"]
impl crate::Readable for F4 {}
#[doc = "`write(|w| ..)` method takes [f4::W](f4::W) writer structure"]
impl crate::Writable for F4 {}
#[doc = "IN/OUT endpoint 4 FIFO"]
pub mod f4;
#[doc = "IN/OUT endpoint 5 FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [f5](f5) module"]
pub type F5 = crate::Reg<u32, _F5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F5;
#[doc = "`read()` method returns [f5::R](f5::R) reader structure"]
impl crate::Readable for F5 {}
#[doc = "`write(|w| ..)` method takes [f5::W](f5::W) writer structure"]
impl crate::Writable for F5 {}
#[doc = "IN/OUT endpoint 5 FIFO"]
pub mod f5;
