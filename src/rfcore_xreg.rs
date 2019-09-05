#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    pub frmfilt0: FRMFILT0,
    #[doc = "0x04 - The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
    pub frmfilt1: FRMFILT1,
    #[doc = "0x08 - Source address matching and pending bits"]
    pub srcmatch: SRCMATCH,
    #[doc = "0x0c - Short address matching"]
    pub srcshorten0: SRCSHORTEN0,
    #[doc = "0x10 - Short address matching"]
    pub srcshorten1: SRCSHORTEN1,
    #[doc = "0x14 - Short address matching"]
    pub srcshorten2: SRCSHORTEN2,
    #[doc = "0x18 - Extended address matching"]
    pub srcexten0: SRCEXTEN0,
    #[doc = "0x1c - Extended address matching"]
    pub srcexten1: SRCEXTEN1,
    #[doc = "0x20 - Extended address matching"]
    pub srcexten2: SRCEXTEN2,
    #[doc = "0x24 - Frame handling"]
    pub frmctrl0: FRMCTRL0,
    #[doc = "0x28 - Frame handling"]
    pub frmctrl1: FRMCTRL1,
    #[doc = "0x2c - RX enabling"]
    pub rxenable: RXENABLE,
    #[doc = "0x30 - RX enabling"]
    pub rxmaskset: RXMASKSET,
    #[doc = "0x34 - RX disabling"]
    pub rxmaskclr: RXMASKCLR,
    #[doc = "0x38 - Crystal oscillator frequency tuning"]
    pub freqtune: FREQTUNE,
    #[doc = "0x3c - Controls the RF frequency"]
    pub freqctrl: FREQCTRL,
    #[doc = "0x40 - Controls the output power"]
    pub txpower: TXPOWER,
    #[doc = "0x44 - Controls the TX settings"]
    pub txctrl: TXCTRL,
    #[doc = "0x48 - Radio status register"]
    pub fsmstat0: FSMSTAT0,
    #[doc = "0x4c - Radio status register"]
    pub fsmstat1: FSMSTAT1,
    #[doc = "0x50 - FIFOP threshold"]
    pub fifopctrl: FIFOPCTRL,
    #[doc = "0x54 - FSM options"]
    pub fsmctrl: FSMCTRL,
    #[doc = "0x58 - CCA threshold"]
    pub ccactrl0: CCACTRL0,
    #[doc = "0x5c - Other CCA Options"]
    pub ccactrl1: CCACTRL1,
    #[doc = "0x60 - RSSI status register"]
    pub rssi: RSSI,
    #[doc = "0x64 - RSSI valid status register"]
    pub rssistat: RSSISTAT,
    #[doc = "0x68 - First byte in RX FIFO"]
    pub rxfirst: RXFIRST,
    #[doc = "0x6c - Number of bytes in RX FIFO"]
    pub rxfifocnt: RXFIFOCNT,
    #[doc = "0x70 - Number of bytes in TX FIFO"]
    pub txfifocnt: TXFIFOCNT,
    #[doc = "0x74 - RX FIFO pointer"]
    pub rxfirst_ptr: RXFIRST_PTR,
    #[doc = "0x78 - RX FIFO pointer"]
    pub rxlast_ptr: RXLAST_PTR,
    #[doc = "0x7c - RX FIFO pointer"]
    pub rxp1_ptr: RXP1_PTR,
    _reserved32: [u8; 4usize],
    #[doc = "0x84 - TX FIFO pointer"]
    pub txfirst_ptr: TXFIRST_PTR,
    #[doc = "0x88 - TX FIFO pointer"]
    pub txlast_ptr: TXLAST_PTR,
    #[doc = "0x8c - RF interrupt masks"]
    pub rfirqm0: RFIRQM0,
    #[doc = "0x90 - RF interrupt masks"]
    pub rfirqm1: RFIRQM1,
    #[doc = "0x94 - RF error interrupt mask"]
    pub rferrm: RFERRM,
    _reserved37: [u8; 4usize],
    #[doc = "0x9c - Random data"]
    pub rfrnd: RFRND,
    #[doc = "0xa0 - Controls modem"]
    pub mdmctrl0: MDMCTRL0,
    #[doc = "0xa4 - Controls modem"]
    pub mdmctrl1: MDMCTRL1,
    #[doc = "0xa8 - Estimated RF frequency offset"]
    pub freqest: FREQEST,
    #[doc = "0xac - Tune receive section"]
    pub rxctrl: RXCTRL,
    #[doc = "0xb0 - Tune frequency synthesizer"]
    pub fsctrl: FSCTRL,
    #[doc = "0xb4 - Tune frequency calibration"]
    pub fscal0: FSCAL0,
    #[doc = "0xb8 - Tune frequency calibration"]
    pub fscal1: FSCAL1,
    #[doc = "0xbc - Tune frequency calibration"]
    pub fscal2: FSCAL2,
    #[doc = "0xc0 - Tune frequency calibration"]
    pub fscal3: FSCAL3,
    #[doc = "0xc4 - AGC dynamic range control"]
    pub agcctrl0: AGCCTRL0,
    #[doc = "0xc8 - AGC reference level"]
    pub agcctrl1: AGCCTRL1,
    #[doc = "0xcc - AGC gain override"]
    pub agcctrl2: AGCCTRL2,
    #[doc = "0xd0 - AGC control"]
    pub agcctrl3: AGCCTRL3,
    #[doc = "0xd4 - ADC tuning"]
    pub adctest0: ADCTEST0,
    #[doc = "0xd8 - ADC tuning"]
    pub adctest1: ADCTEST1,
    #[doc = "0xdc - ADC tuning"]
    pub adctest2: ADCTEST2,
    #[doc = "0xe0 - Test register for modem"]
    pub mdmtest0: MDMTEST0,
    #[doc = "0xe4 - Test Register for Modem"]
    pub mdmtest1: MDMTEST1,
    #[doc = "0xe8 - DAC override value"]
    pub dactest0: DACTEST0,
    #[doc = "0xec - DAC override value"]
    pub dactest1: DACTEST1,
    #[doc = "0xf0 - DAC test setting"]
    pub dactest2: DACTEST2,
    #[doc = "0xf4 - Analog test control"]
    pub atest: ATEST,
    #[doc = "0xf8 - Override power-down register"]
    pub ptest0: PTEST0,
    #[doc = "0xfc - Override power-down register"]
    pub ptest1: PTEST1,
    #[doc = "0x100 - CSP program"]
    pub cspprog_0: CSPPROG_0,
    #[doc = "0x104 - CSP program"]
    pub cspprog_1: CSPPROG_1,
    #[doc = "0x108 - CSP program"]
    pub cspprog_2: CSPPROG_2,
    #[doc = "0x10c - CSP program"]
    pub cspprog_3: CSPPROG_3,
    #[doc = "0x110 - CSP program"]
    pub cspprog_4: CSPPROG_4,
    #[doc = "0x114 - CSP program"]
    pub cspprog_5: CSPPROG_5,
    #[doc = "0x118 - CSP program"]
    pub cspprog_6: CSPPROG_6,
    #[doc = "0x11c - CSP program"]
    pub cspprog_7: CSPPROG_7,
    #[doc = "0x120 - CSP program"]
    pub cspprog_8: CSPPROG_8,
    #[doc = "0x124 - CSP program"]
    pub cspprog_9: CSPPROG_9,
    #[doc = "0x128 - CSP program"]
    pub cspprog_10: CSPPROG_10,
    #[doc = "0x12c - CSP program"]
    pub cspprog_11: CSPPROG_11,
    #[doc = "0x130 - CSP program"]
    pub cspprog_12: CSPPROG_12,
    #[doc = "0x134 - CSP program"]
    pub cspprog_13: CSPPROG_13,
    #[doc = "0x138 - CSP program"]
    pub cspprog_14: CSPPROG_14,
    #[doc = "0x13c - CSP program"]
    pub cspprog_15: CSPPROG_15,
    #[doc = "0x140 - CSP program"]
    pub cspprog_16: CSPPROG_16,
    #[doc = "0x144 - CSP program"]
    pub cspprog_17: CSPPROG_17,
    #[doc = "0x148 - CSP program"]
    pub cspprog_18: CSPPROG_18,
    #[doc = "0x14c - CSP program"]
    pub cspprog_19: CSPPROG_19,
    #[doc = "0x150 - CSP program"]
    pub cspprog_20: CSPPROG_20,
    #[doc = "0x154 - CSP program"]
    pub cspprog_21: CSPPROG_21,
    #[doc = "0x158 - CSP program"]
    pub cspprog_22: CSPPROG_22,
    #[doc = "0x15c - CSP program"]
    pub cspprog_23: CSPPROG_23,
    _reserved86: [u8; 32usize],
    #[doc = "0x180 - CSP control bit"]
    pub cspctrl: CSPCTRL,
    #[doc = "0x184 - CSP status register"]
    pub cspstat: CSPSTAT,
    #[doc = "0x188 - CSP X data register"]
    pub cspx: CSPX,
    #[doc = "0x18c - CSP Y data register"]
    pub cspy: CSPY,
    #[doc = "0x190 - CSP Z data register"]
    pub cspz: CSPZ,
    #[doc = "0x194 - CSP T data register"]
    pub cspt: CSPT,
    _reserved92: [u8; 20usize],
    #[doc = "0x1ac - RF observation mux control"]
    pub rfc_obs_ctrl0: RFC_OBS_CTRL0,
    #[doc = "0x1b0 - RF observation mux control"]
    pub rfc_obs_ctrl1: RFC_OBS_CTRL1,
    #[doc = "0x1b4 - RF observation mux control"]
    pub rfc_obs_ctrl2: RFC_OBS_CTRL2,
    _reserved95: [u8; 48usize],
    #[doc = "0x1e8 - TX filter configuration"]
    pub txfiltcfg: TXFILTCFG,
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmfilt0](frmfilt0) module"]
pub type FRMFILT0 = crate::Reg<u32, _FRMFILT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMFILT0;
#[doc = "`read()` method returns [frmfilt0::R](frmfilt0::R) reader structure"]
impl crate::Readable for FRMFILT0 {}
#[doc = "`write(|w| ..)` method takes [frmfilt0::W](frmfilt0::W) writer structure"]
impl crate::Writable for FRMFILT0 {}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt0;
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmfilt1](frmfilt1) module"]
pub type FRMFILT1 = crate::Reg<u32, _FRMFILT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMFILT1;
#[doc = "`read()` method returns [frmfilt1::R](frmfilt1::R) reader structure"]
impl crate::Readable for FRMFILT1 {}
#[doc = "`write(|w| ..)` method takes [frmfilt1::W](frmfilt1::W) writer structure"]
impl crate::Writable for FRMFILT1 {}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM"]
pub mod frmfilt1;
#[doc = "Source address matching and pending bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcmatch](srcmatch) module"]
pub type SRCMATCH = crate::Reg<u32, _SRCMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCMATCH;
#[doc = "`read()` method returns [srcmatch::R](srcmatch::R) reader structure"]
impl crate::Readable for SRCMATCH {}
#[doc = "`write(|w| ..)` method takes [srcmatch::W](srcmatch::W) writer structure"]
impl crate::Writable for SRCMATCH {}
#[doc = "Source address matching and pending bits"]
pub mod srcmatch;
#[doc = "Short address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcshorten0](srcshorten0) module"]
pub type SRCSHORTEN0 = crate::Reg<u32, _SRCSHORTEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSHORTEN0;
#[doc = "`read()` method returns [srcshorten0::R](srcshorten0::R) reader structure"]
impl crate::Readable for SRCSHORTEN0 {}
#[doc = "`write(|w| ..)` method takes [srcshorten0::W](srcshorten0::W) writer structure"]
impl crate::Writable for SRCSHORTEN0 {}
#[doc = "Short address matching"]
pub mod srcshorten0;
#[doc = "Short address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcshorten1](srcshorten1) module"]
pub type SRCSHORTEN1 = crate::Reg<u32, _SRCSHORTEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSHORTEN1;
#[doc = "`read()` method returns [srcshorten1::R](srcshorten1::R) reader structure"]
impl crate::Readable for SRCSHORTEN1 {}
#[doc = "`write(|w| ..)` method takes [srcshorten1::W](srcshorten1::W) writer structure"]
impl crate::Writable for SRCSHORTEN1 {}
#[doc = "Short address matching"]
pub mod srcshorten1;
#[doc = "Short address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcshorten2](srcshorten2) module"]
pub type SRCSHORTEN2 = crate::Reg<u32, _SRCSHORTEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSHORTEN2;
#[doc = "`read()` method returns [srcshorten2::R](srcshorten2::R) reader structure"]
impl crate::Readable for SRCSHORTEN2 {}
#[doc = "`write(|w| ..)` method takes [srcshorten2::W](srcshorten2::W) writer structure"]
impl crate::Writable for SRCSHORTEN2 {}
#[doc = "Short address matching"]
pub mod srcshorten2;
#[doc = "Extended address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcexten0](srcexten0) module"]
pub type SRCEXTEN0 = crate::Reg<u32, _SRCEXTEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCEXTEN0;
#[doc = "`read()` method returns [srcexten0::R](srcexten0::R) reader structure"]
impl crate::Readable for SRCEXTEN0 {}
#[doc = "`write(|w| ..)` method takes [srcexten0::W](srcexten0::W) writer structure"]
impl crate::Writable for SRCEXTEN0 {}
#[doc = "Extended address matching"]
pub mod srcexten0;
#[doc = "Extended address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcexten1](srcexten1) module"]
pub type SRCEXTEN1 = crate::Reg<u32, _SRCEXTEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCEXTEN1;
#[doc = "`read()` method returns [srcexten1::R](srcexten1::R) reader structure"]
impl crate::Readable for SRCEXTEN1 {}
#[doc = "`write(|w| ..)` method takes [srcexten1::W](srcexten1::W) writer structure"]
impl crate::Writable for SRCEXTEN1 {}
#[doc = "Extended address matching"]
pub mod srcexten1;
#[doc = "Extended address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcexten2](srcexten2) module"]
pub type SRCEXTEN2 = crate::Reg<u32, _SRCEXTEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCEXTEN2;
#[doc = "`read()` method returns [srcexten2::R](srcexten2::R) reader structure"]
impl crate::Readable for SRCEXTEN2 {}
#[doc = "`write(|w| ..)` method takes [srcexten2::W](srcexten2::W) writer structure"]
impl crate::Writable for SRCEXTEN2 {}
#[doc = "Extended address matching"]
pub mod srcexten2;
#[doc = "Frame handling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmctrl0](frmctrl0) module"]
pub type FRMCTRL0 = crate::Reg<u32, _FRMCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMCTRL0;
#[doc = "`read()` method returns [frmctrl0::R](frmctrl0::R) reader structure"]
impl crate::Readable for FRMCTRL0 {}
#[doc = "`write(|w| ..)` method takes [frmctrl0::W](frmctrl0::W) writer structure"]
impl crate::Writable for FRMCTRL0 {}
#[doc = "Frame handling"]
pub mod frmctrl0;
#[doc = "Frame handling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [frmctrl1](frmctrl1) module"]
pub type FRMCTRL1 = crate::Reg<u32, _FRMCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRMCTRL1;
#[doc = "`read()` method returns [frmctrl1::R](frmctrl1::R) reader structure"]
impl crate::Readable for FRMCTRL1 {}
#[doc = "`write(|w| ..)` method takes [frmctrl1::W](frmctrl1::W) writer structure"]
impl crate::Writable for FRMCTRL1 {}
#[doc = "Frame handling"]
pub mod frmctrl1;
#[doc = "RX enabling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxenable](rxenable) module"]
pub type RXENABLE = crate::Reg<u32, _RXENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXENABLE;
#[doc = "`read()` method returns [rxenable::R](rxenable::R) reader structure"]
impl crate::Readable for RXENABLE {}
#[doc = "`write(|w| ..)` method takes [rxenable::W](rxenable::W) writer structure"]
impl crate::Writable for RXENABLE {}
#[doc = "RX enabling"]
pub mod rxenable;
#[doc = "RX enabling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxmaskset](rxmaskset) module"]
pub type RXMASKSET = crate::Reg<u32, _RXMASKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMASKSET;
#[doc = "`read()` method returns [rxmaskset::R](rxmaskset::R) reader structure"]
impl crate::Readable for RXMASKSET {}
#[doc = "`write(|w| ..)` method takes [rxmaskset::W](rxmaskset::W) writer structure"]
impl crate::Writable for RXMASKSET {}
#[doc = "RX enabling"]
pub mod rxmaskset;
#[doc = "RX disabling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxmaskclr](rxmaskclr) module"]
pub type RXMASKCLR = crate::Reg<u32, _RXMASKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMASKCLR;
#[doc = "`read()` method returns [rxmaskclr::R](rxmaskclr::R) reader structure"]
impl crate::Readable for RXMASKCLR {}
#[doc = "`write(|w| ..)` method takes [rxmaskclr::W](rxmaskclr::W) writer structure"]
impl crate::Writable for RXMASKCLR {}
#[doc = "RX disabling"]
pub mod rxmaskclr;
#[doc = "Crystal oscillator frequency tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freqtune](freqtune) module"]
pub type FREQTUNE = crate::Reg<u32, _FREQTUNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQTUNE;
#[doc = "`read()` method returns [freqtune::R](freqtune::R) reader structure"]
impl crate::Readable for FREQTUNE {}
#[doc = "`write(|w| ..)` method takes [freqtune::W](freqtune::W) writer structure"]
impl crate::Writable for FREQTUNE {}
#[doc = "Crystal oscillator frequency tuning"]
pub mod freqtune;
#[doc = "Controls the RF frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freqctrl](freqctrl) module"]
pub type FREQCTRL = crate::Reg<u32, _FREQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQCTRL;
#[doc = "`read()` method returns [freqctrl::R](freqctrl::R) reader structure"]
impl crate::Readable for FREQCTRL {}
#[doc = "`write(|w| ..)` method takes [freqctrl::W](freqctrl::W) writer structure"]
impl crate::Writable for FREQCTRL {}
#[doc = "Controls the RF frequency"]
pub mod freqctrl;
#[doc = "Controls the output power\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpower](txpower) module"]
pub type TXPOWER = crate::Reg<u32, _TXPOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPOWER;
#[doc = "`read()` method returns [txpower::R](txpower::R) reader structure"]
impl crate::Readable for TXPOWER {}
#[doc = "`write(|w| ..)` method takes [txpower::W](txpower::W) writer structure"]
impl crate::Writable for TXPOWER {}
#[doc = "Controls the output power"]
pub mod txpower;
#[doc = "Controls the TX settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txctrl](txctrl) module"]
pub type TXCTRL = crate::Reg<u32, _TXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCTRL;
#[doc = "`read()` method returns [txctrl::R](txctrl::R) reader structure"]
impl crate::Readable for TXCTRL {}
#[doc = "`write(|w| ..)` method takes [txctrl::W](txctrl::W) writer structure"]
impl crate::Writable for TXCTRL {}
#[doc = "Controls the TX settings"]
pub mod txctrl;
#[doc = "Radio status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsmstat0](fsmstat0) module"]
pub type FSMSTAT0 = crate::Reg<u32, _FSMSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSMSTAT0;
#[doc = "`read()` method returns [fsmstat0::R](fsmstat0::R) reader structure"]
impl crate::Readable for FSMSTAT0 {}
#[doc = "`write(|w| ..)` method takes [fsmstat0::W](fsmstat0::W) writer structure"]
impl crate::Writable for FSMSTAT0 {}
#[doc = "Radio status register"]
pub mod fsmstat0;
#[doc = "Radio status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsmstat1](fsmstat1) module"]
pub type FSMSTAT1 = crate::Reg<u32, _FSMSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSMSTAT1;
#[doc = "`read()` method returns [fsmstat1::R](fsmstat1::R) reader structure"]
impl crate::Readable for FSMSTAT1 {}
#[doc = "`write(|w| ..)` method takes [fsmstat1::W](fsmstat1::W) writer structure"]
impl crate::Writable for FSMSTAT1 {}
#[doc = "Radio status register"]
pub mod fsmstat1;
#[doc = "FIFOP threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifopctrl](fifopctrl) module"]
pub type FIFOPCTRL = crate::Reg<u32, _FIFOPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOPCTRL;
#[doc = "`read()` method returns [fifopctrl::R](fifopctrl::R) reader structure"]
impl crate::Readable for FIFOPCTRL {}
#[doc = "`write(|w| ..)` method takes [fifopctrl::W](fifopctrl::W) writer structure"]
impl crate::Writable for FIFOPCTRL {}
#[doc = "FIFOP threshold"]
pub mod fifopctrl;
#[doc = "FSM options\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsmctrl](fsmctrl) module"]
pub type FSMCTRL = crate::Reg<u32, _FSMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSMCTRL;
#[doc = "`read()` method returns [fsmctrl::R](fsmctrl::R) reader structure"]
impl crate::Readable for FSMCTRL {}
#[doc = "`write(|w| ..)` method takes [fsmctrl::W](fsmctrl::W) writer structure"]
impl crate::Writable for FSMCTRL {}
#[doc = "FSM options"]
pub mod fsmctrl;
#[doc = "CCA threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccactrl0](ccactrl0) module"]
pub type CCACTRL0 = crate::Reg<u32, _CCACTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCACTRL0;
#[doc = "`read()` method returns [ccactrl0::R](ccactrl0::R) reader structure"]
impl crate::Readable for CCACTRL0 {}
#[doc = "`write(|w| ..)` method takes [ccactrl0::W](ccactrl0::W) writer structure"]
impl crate::Writable for CCACTRL0 {}
#[doc = "CCA threshold"]
pub mod ccactrl0;
#[doc = "Other CCA Options\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccactrl1](ccactrl1) module"]
pub type CCACTRL1 = crate::Reg<u32, _CCACTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCACTRL1;
#[doc = "`read()` method returns [ccactrl1::R](ccactrl1::R) reader structure"]
impl crate::Readable for CCACTRL1 {}
#[doc = "`write(|w| ..)` method takes [ccactrl1::W](ccactrl1::W) writer structure"]
impl crate::Writable for CCACTRL1 {}
#[doc = "Other CCA Options"]
pub mod ccactrl1;
#[doc = "RSSI status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rssi](rssi) module"]
pub type RSSI = crate::Reg<u32, _RSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSSI;
#[doc = "`read()` method returns [rssi::R](rssi::R) reader structure"]
impl crate::Readable for RSSI {}
#[doc = "`write(|w| ..)` method takes [rssi::W](rssi::W) writer structure"]
impl crate::Writable for RSSI {}
#[doc = "RSSI status register"]
pub mod rssi;
#[doc = "RSSI valid status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rssistat](rssistat) module"]
pub type RSSISTAT = crate::Reg<u32, _RSSISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSSISTAT;
#[doc = "`read()` method returns [rssistat::R](rssistat::R) reader structure"]
impl crate::Readable for RSSISTAT {}
#[doc = "`write(|w| ..)` method takes [rssistat::W](rssistat::W) writer structure"]
impl crate::Writable for RSSISTAT {}
#[doc = "RSSI valid status register"]
pub mod rssistat;
#[doc = "First byte in RX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfirst](rxfirst) module"]
pub type RXFIRST = crate::Reg<u32, _RXFIRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIRST;
#[doc = "`read()` method returns [rxfirst::R](rxfirst::R) reader structure"]
impl crate::Readable for RXFIRST {}
#[doc = "`write(|w| ..)` method takes [rxfirst::W](rxfirst::W) writer structure"]
impl crate::Writable for RXFIRST {}
#[doc = "First byte in RX FIFO"]
pub mod rxfirst;
#[doc = "Number of bytes in RX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfifocnt](rxfifocnt) module"]
pub type RXFIFOCNT = crate::Reg<u32, _RXFIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIFOCNT;
#[doc = "`read()` method returns [rxfifocnt::R](rxfifocnt::R) reader structure"]
impl crate::Readable for RXFIFOCNT {}
#[doc = "`write(|w| ..)` method takes [rxfifocnt::W](rxfifocnt::W) writer structure"]
impl crate::Writable for RXFIFOCNT {}
#[doc = "Number of bytes in RX FIFO"]
pub mod rxfifocnt;
#[doc = "Number of bytes in TX FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txfifocnt](txfifocnt) module"]
pub type TXFIFOCNT = crate::Reg<u32, _TXFIFOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFIFOCNT;
#[doc = "`read()` method returns [txfifocnt::R](txfifocnt::R) reader structure"]
impl crate::Readable for TXFIFOCNT {}
#[doc = "`write(|w| ..)` method takes [txfifocnt::W](txfifocnt::W) writer structure"]
impl crate::Writable for TXFIFOCNT {}
#[doc = "Number of bytes in TX FIFO"]
pub mod txfifocnt;
#[doc = "RX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxfirst_ptr](rxfirst_ptr) module"]
pub type RXFIRST_PTR = crate::Reg<u32, _RXFIRST_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIRST_PTR;
#[doc = "`read()` method returns [rxfirst_ptr::R](rxfirst_ptr::R) reader structure"]
impl crate::Readable for RXFIRST_PTR {}
#[doc = "`write(|w| ..)` method takes [rxfirst_ptr::W](rxfirst_ptr::W) writer structure"]
impl crate::Writable for RXFIRST_PTR {}
#[doc = "RX FIFO pointer"]
pub mod rxfirst_ptr;
#[doc = "RX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxlast_ptr](rxlast_ptr) module"]
pub type RXLAST_PTR = crate::Reg<u32, _RXLAST_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXLAST_PTR;
#[doc = "`read()` method returns [rxlast_ptr::R](rxlast_ptr::R) reader structure"]
impl crate::Readable for RXLAST_PTR {}
#[doc = "`write(|w| ..)` method takes [rxlast_ptr::W](rxlast_ptr::W) writer structure"]
impl crate::Writable for RXLAST_PTR {}
#[doc = "RX FIFO pointer"]
pub mod rxlast_ptr;
#[doc = "RX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxp1_ptr](rxp1_ptr) module"]
pub type RXP1_PTR = crate::Reg<u32, _RXP1_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXP1_PTR;
#[doc = "`read()` method returns [rxp1_ptr::R](rxp1_ptr::R) reader structure"]
impl crate::Readable for RXP1_PTR {}
#[doc = "`write(|w| ..)` method takes [rxp1_ptr::W](rxp1_ptr::W) writer structure"]
impl crate::Writable for RXP1_PTR {}
#[doc = "RX FIFO pointer"]
pub mod rxp1_ptr;
#[doc = "TX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txfirst_ptr](txfirst_ptr) module"]
pub type TXFIRST_PTR = crate::Reg<u32, _TXFIRST_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFIRST_PTR;
#[doc = "`read()` method returns [txfirst_ptr::R](txfirst_ptr::R) reader structure"]
impl crate::Readable for TXFIRST_PTR {}
#[doc = "`write(|w| ..)` method takes [txfirst_ptr::W](txfirst_ptr::W) writer structure"]
impl crate::Writable for TXFIRST_PTR {}
#[doc = "TX FIFO pointer"]
pub mod txfirst_ptr;
#[doc = "TX FIFO pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txlast_ptr](txlast_ptr) module"]
pub type TXLAST_PTR = crate::Reg<u32, _TXLAST_PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXLAST_PTR;
#[doc = "`read()` method returns [txlast_ptr::R](txlast_ptr::R) reader structure"]
impl crate::Readable for TXLAST_PTR {}
#[doc = "`write(|w| ..)` method takes [txlast_ptr::W](txlast_ptr::W) writer structure"]
impl crate::Writable for TXLAST_PTR {}
#[doc = "TX FIFO pointer"]
pub mod txlast_ptr;
#[doc = "RF interrupt masks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfirqm0](rfirqm0) module"]
pub type RFIRQM0 = crate::Reg<u32, _RFIRQM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIRQM0;
#[doc = "`read()` method returns [rfirqm0::R](rfirqm0::R) reader structure"]
impl crate::Readable for RFIRQM0 {}
#[doc = "`write(|w| ..)` method takes [rfirqm0::W](rfirqm0::W) writer structure"]
impl crate::Writable for RFIRQM0 {}
#[doc = "RF interrupt masks"]
pub mod rfirqm0;
#[doc = "RF interrupt masks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfirqm1](rfirqm1) module"]
pub type RFIRQM1 = crate::Reg<u32, _RFIRQM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIRQM1;
#[doc = "`read()` method returns [rfirqm1::R](rfirqm1::R) reader structure"]
impl crate::Readable for RFIRQM1 {}
#[doc = "`write(|w| ..)` method takes [rfirqm1::W](rfirqm1::W) writer structure"]
impl crate::Writable for RFIRQM1 {}
#[doc = "RF interrupt masks"]
pub mod rfirqm1;
#[doc = "RF error interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rferrm](rferrm) module"]
pub type RFERRM = crate::Reg<u32, _RFERRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFERRM;
#[doc = "`read()` method returns [rferrm::R](rferrm::R) reader structure"]
impl crate::Readable for RFERRM {}
#[doc = "`write(|w| ..)` method takes [rferrm::W](rferrm::W) writer structure"]
impl crate::Writable for RFERRM {}
#[doc = "RF error interrupt mask"]
pub mod rferrm;
#[doc = "Random data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfrnd](rfrnd) module"]
pub type RFRND = crate::Reg<u32, _RFRND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFRND;
#[doc = "`read()` method returns [rfrnd::R](rfrnd::R) reader structure"]
impl crate::Readable for RFRND {}
#[doc = "`write(|w| ..)` method takes [rfrnd::W](rfrnd::W) writer structure"]
impl crate::Writable for RFRND {}
#[doc = "Random data"]
pub mod rfrnd;
#[doc = "Controls modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdmctrl0](mdmctrl0) module"]
pub type MDMCTRL0 = crate::Reg<u32, _MDMCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMCTRL0;
#[doc = "`read()` method returns [mdmctrl0::R](mdmctrl0::R) reader structure"]
impl crate::Readable for MDMCTRL0 {}
#[doc = "`write(|w| ..)` method takes [mdmctrl0::W](mdmctrl0::W) writer structure"]
impl crate::Writable for MDMCTRL0 {}
#[doc = "Controls modem"]
pub mod mdmctrl0;
#[doc = "Controls modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdmctrl1](mdmctrl1) module"]
pub type MDMCTRL1 = crate::Reg<u32, _MDMCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMCTRL1;
#[doc = "`read()` method returns [mdmctrl1::R](mdmctrl1::R) reader structure"]
impl crate::Readable for MDMCTRL1 {}
#[doc = "`write(|w| ..)` method takes [mdmctrl1::W](mdmctrl1::W) writer structure"]
impl crate::Writable for MDMCTRL1 {}
#[doc = "Controls modem"]
pub mod mdmctrl1;
#[doc = "Estimated RF frequency offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freqest](freqest) module"]
pub type FREQEST = crate::Reg<u32, _FREQEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQEST;
#[doc = "`read()` method returns [freqest::R](freqest::R) reader structure"]
impl crate::Readable for FREQEST {}
#[doc = "`write(|w| ..)` method takes [freqest::W](freqest::W) writer structure"]
impl crate::Writable for FREQEST {}
#[doc = "Estimated RF frequency offset"]
pub mod freqest;
#[doc = "Tune receive section\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxctrl](rxctrl) module"]
pub type RXCTRL = crate::Reg<u32, _RXCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCTRL;
#[doc = "`read()` method returns [rxctrl::R](rxctrl::R) reader structure"]
impl crate::Readable for RXCTRL {}
#[doc = "`write(|w| ..)` method takes [rxctrl::W](rxctrl::W) writer structure"]
impl crate::Writable for RXCTRL {}
#[doc = "Tune receive section"]
pub mod rxctrl;
#[doc = "Tune frequency synthesizer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsctrl](fsctrl) module"]
pub type FSCTRL = crate::Reg<u32, _FSCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCTRL;
#[doc = "`read()` method returns [fsctrl::R](fsctrl::R) reader structure"]
impl crate::Readable for FSCTRL {}
#[doc = "`write(|w| ..)` method takes [fsctrl::W](fsctrl::W) writer structure"]
impl crate::Writable for FSCTRL {}
#[doc = "Tune frequency synthesizer"]
pub mod fsctrl;
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fscal0](fscal0) module"]
pub type FSCAL0 = crate::Reg<u32, _FSCAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCAL0;
#[doc = "`read()` method returns [fscal0::R](fscal0::R) reader structure"]
impl crate::Readable for FSCAL0 {}
#[doc = "`write(|w| ..)` method takes [fscal0::W](fscal0::W) writer structure"]
impl crate::Writable for FSCAL0 {}
#[doc = "Tune frequency calibration"]
pub mod fscal0;
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fscal1](fscal1) module"]
pub type FSCAL1 = crate::Reg<u32, _FSCAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCAL1;
#[doc = "`read()` method returns [fscal1::R](fscal1::R) reader structure"]
impl crate::Readable for FSCAL1 {}
#[doc = "`write(|w| ..)` method takes [fscal1::W](fscal1::W) writer structure"]
impl crate::Writable for FSCAL1 {}
#[doc = "Tune frequency calibration"]
pub mod fscal1;
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fscal2](fscal2) module"]
pub type FSCAL2 = crate::Reg<u32, _FSCAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCAL2;
#[doc = "`read()` method returns [fscal2::R](fscal2::R) reader structure"]
impl crate::Readable for FSCAL2 {}
#[doc = "`write(|w| ..)` method takes [fscal2::W](fscal2::W) writer structure"]
impl crate::Writable for FSCAL2 {}
#[doc = "Tune frequency calibration"]
pub mod fscal2;
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fscal3](fscal3) module"]
pub type FSCAL3 = crate::Reg<u32, _FSCAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCAL3;
#[doc = "`read()` method returns [fscal3::R](fscal3::R) reader structure"]
impl crate::Readable for FSCAL3 {}
#[doc = "`write(|w| ..)` method takes [fscal3::W](fscal3::W) writer structure"]
impl crate::Writable for FSCAL3 {}
#[doc = "Tune frequency calibration"]
pub mod fscal3;
#[doc = "AGC dynamic range control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [agcctrl0](agcctrl0) module"]
pub type AGCCTRL0 = crate::Reg<u32, _AGCCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AGCCTRL0;
#[doc = "`read()` method returns [agcctrl0::R](agcctrl0::R) reader structure"]
impl crate::Readable for AGCCTRL0 {}
#[doc = "`write(|w| ..)` method takes [agcctrl0::W](agcctrl0::W) writer structure"]
impl crate::Writable for AGCCTRL0 {}
#[doc = "AGC dynamic range control"]
pub mod agcctrl0;
#[doc = "AGC reference level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [agcctrl1](agcctrl1) module"]
pub type AGCCTRL1 = crate::Reg<u32, _AGCCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AGCCTRL1;
#[doc = "`read()` method returns [agcctrl1::R](agcctrl1::R) reader structure"]
impl crate::Readable for AGCCTRL1 {}
#[doc = "`write(|w| ..)` method takes [agcctrl1::W](agcctrl1::W) writer structure"]
impl crate::Writable for AGCCTRL1 {}
#[doc = "AGC reference level"]
pub mod agcctrl1;
#[doc = "AGC gain override\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [agcctrl2](agcctrl2) module"]
pub type AGCCTRL2 = crate::Reg<u32, _AGCCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AGCCTRL2;
#[doc = "`read()` method returns [agcctrl2::R](agcctrl2::R) reader structure"]
impl crate::Readable for AGCCTRL2 {}
#[doc = "`write(|w| ..)` method takes [agcctrl2::W](agcctrl2::W) writer structure"]
impl crate::Writable for AGCCTRL2 {}
#[doc = "AGC gain override"]
pub mod agcctrl2;
#[doc = "AGC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [agcctrl3](agcctrl3) module"]
pub type AGCCTRL3 = crate::Reg<u32, _AGCCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AGCCTRL3;
#[doc = "`read()` method returns [agcctrl3::R](agcctrl3::R) reader structure"]
impl crate::Readable for AGCCTRL3 {}
#[doc = "`write(|w| ..)` method takes [agcctrl3::W](agcctrl3::W) writer structure"]
impl crate::Writable for AGCCTRL3 {}
#[doc = "AGC control"]
pub mod agcctrl3;
#[doc = "ADC tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adctest0](adctest0) module"]
pub type ADCTEST0 = crate::Reg<u32, _ADCTEST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCTEST0;
#[doc = "`read()` method returns [adctest0::R](adctest0::R) reader structure"]
impl crate::Readable for ADCTEST0 {}
#[doc = "`write(|w| ..)` method takes [adctest0::W](adctest0::W) writer structure"]
impl crate::Writable for ADCTEST0 {}
#[doc = "ADC tuning"]
pub mod adctest0;
#[doc = "ADC tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adctest1](adctest1) module"]
pub type ADCTEST1 = crate::Reg<u32, _ADCTEST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCTEST1;
#[doc = "`read()` method returns [adctest1::R](adctest1::R) reader structure"]
impl crate::Readable for ADCTEST1 {}
#[doc = "`write(|w| ..)` method takes [adctest1::W](adctest1::W) writer structure"]
impl crate::Writable for ADCTEST1 {}
#[doc = "ADC tuning"]
pub mod adctest1;
#[doc = "ADC tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adctest2](adctest2) module"]
pub type ADCTEST2 = crate::Reg<u32, _ADCTEST2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCTEST2;
#[doc = "`read()` method returns [adctest2::R](adctest2::R) reader structure"]
impl crate::Readable for ADCTEST2 {}
#[doc = "`write(|w| ..)` method takes [adctest2::W](adctest2::W) writer structure"]
impl crate::Writable for ADCTEST2 {}
#[doc = "ADC tuning"]
pub mod adctest2;
#[doc = "Test register for modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdmtest0](mdmtest0) module"]
pub type MDMTEST0 = crate::Reg<u32, _MDMTEST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMTEST0;
#[doc = "`read()` method returns [mdmtest0::R](mdmtest0::R) reader structure"]
impl crate::Readable for MDMTEST0 {}
#[doc = "`write(|w| ..)` method takes [mdmtest0::W](mdmtest0::W) writer structure"]
impl crate::Writable for MDMTEST0 {}
#[doc = "Test register for modem"]
pub mod mdmtest0;
#[doc = "Test Register for Modem\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdmtest1](mdmtest1) module"]
pub type MDMTEST1 = crate::Reg<u32, _MDMTEST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDMTEST1;
#[doc = "`read()` method returns [mdmtest1::R](mdmtest1::R) reader structure"]
impl crate::Readable for MDMTEST1 {}
#[doc = "`write(|w| ..)` method takes [mdmtest1::W](mdmtest1::W) writer structure"]
impl crate::Writable for MDMTEST1 {}
#[doc = "Test Register for Modem"]
pub mod mdmtest1;
#[doc = "DAC override value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dactest0](dactest0) module"]
pub type DACTEST0 = crate::Reg<u32, _DACTEST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACTEST0;
#[doc = "`read()` method returns [dactest0::R](dactest0::R) reader structure"]
impl crate::Readable for DACTEST0 {}
#[doc = "`write(|w| ..)` method takes [dactest0::W](dactest0::W) writer structure"]
impl crate::Writable for DACTEST0 {}
#[doc = "DAC override value"]
pub mod dactest0;
#[doc = "DAC override value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dactest1](dactest1) module"]
pub type DACTEST1 = crate::Reg<u32, _DACTEST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACTEST1;
#[doc = "`read()` method returns [dactest1::R](dactest1::R) reader structure"]
impl crate::Readable for DACTEST1 {}
#[doc = "`write(|w| ..)` method takes [dactest1::W](dactest1::W) writer structure"]
impl crate::Writable for DACTEST1 {}
#[doc = "DAC override value"]
pub mod dactest1;
#[doc = "DAC test setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dactest2](dactest2) module"]
pub type DACTEST2 = crate::Reg<u32, _DACTEST2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACTEST2;
#[doc = "`read()` method returns [dactest2::R](dactest2::R) reader structure"]
impl crate::Readable for DACTEST2 {}
#[doc = "`write(|w| ..)` method takes [dactest2::W](dactest2::W) writer structure"]
impl crate::Writable for DACTEST2 {}
#[doc = "DAC test setting"]
pub mod dactest2;
#[doc = "Analog test control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [atest](atest) module"]
pub type ATEST = crate::Reg<u32, _ATEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATEST;
#[doc = "`read()` method returns [atest::R](atest::R) reader structure"]
impl crate::Readable for ATEST {}
#[doc = "`write(|w| ..)` method takes [atest::W](atest::W) writer structure"]
impl crate::Writable for ATEST {}
#[doc = "Analog test control"]
pub mod atest;
#[doc = "Override power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptest0](ptest0) module"]
pub type PTEST0 = crate::Reg<u32, _PTEST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTEST0;
#[doc = "`read()` method returns [ptest0::R](ptest0::R) reader structure"]
impl crate::Readable for PTEST0 {}
#[doc = "`write(|w| ..)` method takes [ptest0::W](ptest0::W) writer structure"]
impl crate::Writable for PTEST0 {}
#[doc = "Override power-down register"]
pub mod ptest0;
#[doc = "Override power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptest1](ptest1) module"]
pub type PTEST1 = crate::Reg<u32, _PTEST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTEST1;
#[doc = "`read()` method returns [ptest1::R](ptest1::R) reader structure"]
impl crate::Readable for PTEST1 {}
#[doc = "`write(|w| ..)` method takes [ptest1::W](ptest1::W) writer structure"]
impl crate::Writable for PTEST1 {}
#[doc = "Override power-down register"]
pub mod ptest1;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_0](cspprog_0) module"]
pub type CSPPROG_0 = crate::Reg<u32, _CSPPROG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_0;
#[doc = "`read()` method returns [cspprog_0::R](cspprog_0::R) reader structure"]
impl crate::Readable for CSPPROG_0 {}
#[doc = "`write(|w| ..)` method takes [cspprog_0::W](cspprog_0::W) writer structure"]
impl crate::Writable for CSPPROG_0 {}
#[doc = "CSP program"]
pub mod cspprog_0;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_1](cspprog_1) module"]
pub type CSPPROG_1 = crate::Reg<u32, _CSPPROG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_1;
#[doc = "`read()` method returns [cspprog_1::R](cspprog_1::R) reader structure"]
impl crate::Readable for CSPPROG_1 {}
#[doc = "`write(|w| ..)` method takes [cspprog_1::W](cspprog_1::W) writer structure"]
impl crate::Writable for CSPPROG_1 {}
#[doc = "CSP program"]
pub mod cspprog_1;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_2](cspprog_2) module"]
pub type CSPPROG_2 = crate::Reg<u32, _CSPPROG_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_2;
#[doc = "`read()` method returns [cspprog_2::R](cspprog_2::R) reader structure"]
impl crate::Readable for CSPPROG_2 {}
#[doc = "`write(|w| ..)` method takes [cspprog_2::W](cspprog_2::W) writer structure"]
impl crate::Writable for CSPPROG_2 {}
#[doc = "CSP program"]
pub mod cspprog_2;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_3](cspprog_3) module"]
pub type CSPPROG_3 = crate::Reg<u32, _CSPPROG_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_3;
#[doc = "`read()` method returns [cspprog_3::R](cspprog_3::R) reader structure"]
impl crate::Readable for CSPPROG_3 {}
#[doc = "`write(|w| ..)` method takes [cspprog_3::W](cspprog_3::W) writer structure"]
impl crate::Writable for CSPPROG_3 {}
#[doc = "CSP program"]
pub mod cspprog_3;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_4](cspprog_4) module"]
pub type CSPPROG_4 = crate::Reg<u32, _CSPPROG_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_4;
#[doc = "`read()` method returns [cspprog_4::R](cspprog_4::R) reader structure"]
impl crate::Readable for CSPPROG_4 {}
#[doc = "`write(|w| ..)` method takes [cspprog_4::W](cspprog_4::W) writer structure"]
impl crate::Writable for CSPPROG_4 {}
#[doc = "CSP program"]
pub mod cspprog_4;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_5](cspprog_5) module"]
pub type CSPPROG_5 = crate::Reg<u32, _CSPPROG_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_5;
#[doc = "`read()` method returns [cspprog_5::R](cspprog_5::R) reader structure"]
impl crate::Readable for CSPPROG_5 {}
#[doc = "`write(|w| ..)` method takes [cspprog_5::W](cspprog_5::W) writer structure"]
impl crate::Writable for CSPPROG_5 {}
#[doc = "CSP program"]
pub mod cspprog_5;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_6](cspprog_6) module"]
pub type CSPPROG_6 = crate::Reg<u32, _CSPPROG_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_6;
#[doc = "`read()` method returns [cspprog_6::R](cspprog_6::R) reader structure"]
impl crate::Readable for CSPPROG_6 {}
#[doc = "`write(|w| ..)` method takes [cspprog_6::W](cspprog_6::W) writer structure"]
impl crate::Writable for CSPPROG_6 {}
#[doc = "CSP program"]
pub mod cspprog_6;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_7](cspprog_7) module"]
pub type CSPPROG_7 = crate::Reg<u32, _CSPPROG_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_7;
#[doc = "`read()` method returns [cspprog_7::R](cspprog_7::R) reader structure"]
impl crate::Readable for CSPPROG_7 {}
#[doc = "`write(|w| ..)` method takes [cspprog_7::W](cspprog_7::W) writer structure"]
impl crate::Writable for CSPPROG_7 {}
#[doc = "CSP program"]
pub mod cspprog_7;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_8](cspprog_8) module"]
pub type CSPPROG_8 = crate::Reg<u32, _CSPPROG_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_8;
#[doc = "`read()` method returns [cspprog_8::R](cspprog_8::R) reader structure"]
impl crate::Readable for CSPPROG_8 {}
#[doc = "`write(|w| ..)` method takes [cspprog_8::W](cspprog_8::W) writer structure"]
impl crate::Writable for CSPPROG_8 {}
#[doc = "CSP program"]
pub mod cspprog_8;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_9](cspprog_9) module"]
pub type CSPPROG_9 = crate::Reg<u32, _CSPPROG_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_9;
#[doc = "`read()` method returns [cspprog_9::R](cspprog_9::R) reader structure"]
impl crate::Readable for CSPPROG_9 {}
#[doc = "`write(|w| ..)` method takes [cspprog_9::W](cspprog_9::W) writer structure"]
impl crate::Writable for CSPPROG_9 {}
#[doc = "CSP program"]
pub mod cspprog_9;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_10](cspprog_10) module"]
pub type CSPPROG_10 = crate::Reg<u32, _CSPPROG_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_10;
#[doc = "`read()` method returns [cspprog_10::R](cspprog_10::R) reader structure"]
impl crate::Readable for CSPPROG_10 {}
#[doc = "`write(|w| ..)` method takes [cspprog_10::W](cspprog_10::W) writer structure"]
impl crate::Writable for CSPPROG_10 {}
#[doc = "CSP program"]
pub mod cspprog_10;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_11](cspprog_11) module"]
pub type CSPPROG_11 = crate::Reg<u32, _CSPPROG_11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_11;
#[doc = "`read()` method returns [cspprog_11::R](cspprog_11::R) reader structure"]
impl crate::Readable for CSPPROG_11 {}
#[doc = "`write(|w| ..)` method takes [cspprog_11::W](cspprog_11::W) writer structure"]
impl crate::Writable for CSPPROG_11 {}
#[doc = "CSP program"]
pub mod cspprog_11;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_12](cspprog_12) module"]
pub type CSPPROG_12 = crate::Reg<u32, _CSPPROG_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_12;
#[doc = "`read()` method returns [cspprog_12::R](cspprog_12::R) reader structure"]
impl crate::Readable for CSPPROG_12 {}
#[doc = "`write(|w| ..)` method takes [cspprog_12::W](cspprog_12::W) writer structure"]
impl crate::Writable for CSPPROG_12 {}
#[doc = "CSP program"]
pub mod cspprog_12;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_13](cspprog_13) module"]
pub type CSPPROG_13 = crate::Reg<u32, _CSPPROG_13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_13;
#[doc = "`read()` method returns [cspprog_13::R](cspprog_13::R) reader structure"]
impl crate::Readable for CSPPROG_13 {}
#[doc = "`write(|w| ..)` method takes [cspprog_13::W](cspprog_13::W) writer structure"]
impl crate::Writable for CSPPROG_13 {}
#[doc = "CSP program"]
pub mod cspprog_13;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_14](cspprog_14) module"]
pub type CSPPROG_14 = crate::Reg<u32, _CSPPROG_14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_14;
#[doc = "`read()` method returns [cspprog_14::R](cspprog_14::R) reader structure"]
impl crate::Readable for CSPPROG_14 {}
#[doc = "`write(|w| ..)` method takes [cspprog_14::W](cspprog_14::W) writer structure"]
impl crate::Writable for CSPPROG_14 {}
#[doc = "CSP program"]
pub mod cspprog_14;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_15](cspprog_15) module"]
pub type CSPPROG_15 = crate::Reg<u32, _CSPPROG_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_15;
#[doc = "`read()` method returns [cspprog_15::R](cspprog_15::R) reader structure"]
impl crate::Readable for CSPPROG_15 {}
#[doc = "`write(|w| ..)` method takes [cspprog_15::W](cspprog_15::W) writer structure"]
impl crate::Writable for CSPPROG_15 {}
#[doc = "CSP program"]
pub mod cspprog_15;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_16](cspprog_16) module"]
pub type CSPPROG_16 = crate::Reg<u32, _CSPPROG_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_16;
#[doc = "`read()` method returns [cspprog_16::R](cspprog_16::R) reader structure"]
impl crate::Readable for CSPPROG_16 {}
#[doc = "`write(|w| ..)` method takes [cspprog_16::W](cspprog_16::W) writer structure"]
impl crate::Writable for CSPPROG_16 {}
#[doc = "CSP program"]
pub mod cspprog_16;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_17](cspprog_17) module"]
pub type CSPPROG_17 = crate::Reg<u32, _CSPPROG_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_17;
#[doc = "`read()` method returns [cspprog_17::R](cspprog_17::R) reader structure"]
impl crate::Readable for CSPPROG_17 {}
#[doc = "`write(|w| ..)` method takes [cspprog_17::W](cspprog_17::W) writer structure"]
impl crate::Writable for CSPPROG_17 {}
#[doc = "CSP program"]
pub mod cspprog_17;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_18](cspprog_18) module"]
pub type CSPPROG_18 = crate::Reg<u32, _CSPPROG_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_18;
#[doc = "`read()` method returns [cspprog_18::R](cspprog_18::R) reader structure"]
impl crate::Readable for CSPPROG_18 {}
#[doc = "`write(|w| ..)` method takes [cspprog_18::W](cspprog_18::W) writer structure"]
impl crate::Writable for CSPPROG_18 {}
#[doc = "CSP program"]
pub mod cspprog_18;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_19](cspprog_19) module"]
pub type CSPPROG_19 = crate::Reg<u32, _CSPPROG_19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_19;
#[doc = "`read()` method returns [cspprog_19::R](cspprog_19::R) reader structure"]
impl crate::Readable for CSPPROG_19 {}
#[doc = "`write(|w| ..)` method takes [cspprog_19::W](cspprog_19::W) writer structure"]
impl crate::Writable for CSPPROG_19 {}
#[doc = "CSP program"]
pub mod cspprog_19;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_20](cspprog_20) module"]
pub type CSPPROG_20 = crate::Reg<u32, _CSPPROG_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_20;
#[doc = "`read()` method returns [cspprog_20::R](cspprog_20::R) reader structure"]
impl crate::Readable for CSPPROG_20 {}
#[doc = "`write(|w| ..)` method takes [cspprog_20::W](cspprog_20::W) writer structure"]
impl crate::Writable for CSPPROG_20 {}
#[doc = "CSP program"]
pub mod cspprog_20;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_21](cspprog_21) module"]
pub type CSPPROG_21 = crate::Reg<u32, _CSPPROG_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_21;
#[doc = "`read()` method returns [cspprog_21::R](cspprog_21::R) reader structure"]
impl crate::Readable for CSPPROG_21 {}
#[doc = "`write(|w| ..)` method takes [cspprog_21::W](cspprog_21::W) writer structure"]
impl crate::Writable for CSPPROG_21 {}
#[doc = "CSP program"]
pub mod cspprog_21;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_22](cspprog_22) module"]
pub type CSPPROG_22 = crate::Reg<u32, _CSPPROG_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_22;
#[doc = "`read()` method returns [cspprog_22::R](cspprog_22::R) reader structure"]
impl crate::Readable for CSPPROG_22 {}
#[doc = "`write(|w| ..)` method takes [cspprog_22::W](cspprog_22::W) writer structure"]
impl crate::Writable for CSPPROG_22 {}
#[doc = "CSP program"]
pub mod cspprog_22;
#[doc = "CSP program\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspprog_23](cspprog_23) module"]
pub type CSPPROG_23 = crate::Reg<u32, _CSPPROG_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPPROG_23;
#[doc = "`read()` method returns [cspprog_23::R](cspprog_23::R) reader structure"]
impl crate::Readable for CSPPROG_23 {}
#[doc = "`write(|w| ..)` method takes [cspprog_23::W](cspprog_23::W) writer structure"]
impl crate::Writable for CSPPROG_23 {}
#[doc = "CSP program"]
pub mod cspprog_23;
#[doc = "CSP control bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspctrl](cspctrl) module"]
pub type CSPCTRL = crate::Reg<u32, _CSPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPCTRL;
#[doc = "`read()` method returns [cspctrl::R](cspctrl::R) reader structure"]
impl crate::Readable for CSPCTRL {}
#[doc = "`write(|w| ..)` method takes [cspctrl::W](cspctrl::W) writer structure"]
impl crate::Writable for CSPCTRL {}
#[doc = "CSP control bit"]
pub mod cspctrl;
#[doc = "CSP status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspstat](cspstat) module"]
pub type CSPSTAT = crate::Reg<u32, _CSPSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPSTAT;
#[doc = "`read()` method returns [cspstat::R](cspstat::R) reader structure"]
impl crate::Readable for CSPSTAT {}
#[doc = "`write(|w| ..)` method takes [cspstat::W](cspstat::W) writer structure"]
impl crate::Writable for CSPSTAT {}
#[doc = "CSP status register"]
pub mod cspstat;
#[doc = "CSP X data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspx](cspx) module"]
pub type CSPX = crate::Reg<u32, _CSPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPX;
#[doc = "`read()` method returns [cspx::R](cspx::R) reader structure"]
impl crate::Readable for CSPX {}
#[doc = "`write(|w| ..)` method takes [cspx::W](cspx::W) writer structure"]
impl crate::Writable for CSPX {}
#[doc = "CSP X data register"]
pub mod cspx;
#[doc = "CSP Y data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspy](cspy) module"]
pub type CSPY = crate::Reg<u32, _CSPY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPY;
#[doc = "`read()` method returns [cspy::R](cspy::R) reader structure"]
impl crate::Readable for CSPY {}
#[doc = "`write(|w| ..)` method takes [cspy::W](cspy::W) writer structure"]
impl crate::Writable for CSPY {}
#[doc = "CSP Y data register"]
pub mod cspy;
#[doc = "CSP Z data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspz](cspz) module"]
pub type CSPZ = crate::Reg<u32, _CSPZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPZ;
#[doc = "`read()` method returns [cspz::R](cspz::R) reader structure"]
impl crate::Readable for CSPZ {}
#[doc = "`write(|w| ..)` method takes [cspz::W](cspz::W) writer structure"]
impl crate::Writable for CSPZ {}
#[doc = "CSP Z data register"]
pub mod cspz;
#[doc = "CSP T data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cspt](cspt) module"]
pub type CSPT = crate::Reg<u32, _CSPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPT;
#[doc = "`read()` method returns [cspt::R](cspt::R) reader structure"]
impl crate::Readable for CSPT {}
#[doc = "`write(|w| ..)` method takes [cspt::W](cspt::W) writer structure"]
impl crate::Writable for CSPT {}
#[doc = "CSP T data register"]
pub mod cspt;
#[doc = "RF observation mux control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfc_obs_ctrl0](rfc_obs_ctrl0) module"]
pub type RFC_OBS_CTRL0 = crate::Reg<u32, _RFC_OBS_CTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFC_OBS_CTRL0;
#[doc = "`read()` method returns [rfc_obs_ctrl0::R](rfc_obs_ctrl0::R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL0 {}
#[doc = "`write(|w| ..)` method takes [rfc_obs_ctrl0::W](rfc_obs_ctrl0::W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL0 {}
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl0;
#[doc = "RF observation mux control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfc_obs_ctrl1](rfc_obs_ctrl1) module"]
pub type RFC_OBS_CTRL1 = crate::Reg<u32, _RFC_OBS_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFC_OBS_CTRL1;
#[doc = "`read()` method returns [rfc_obs_ctrl1::R](rfc_obs_ctrl1::R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL1 {}
#[doc = "`write(|w| ..)` method takes [rfc_obs_ctrl1::W](rfc_obs_ctrl1::W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL1 {}
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl1;
#[doc = "RF observation mux control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfc_obs_ctrl2](rfc_obs_ctrl2) module"]
pub type RFC_OBS_CTRL2 = crate::Reg<u32, _RFC_OBS_CTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFC_OBS_CTRL2;
#[doc = "`read()` method returns [rfc_obs_ctrl2::R](rfc_obs_ctrl2::R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL2 {}
#[doc = "`write(|w| ..)` method takes [rfc_obs_ctrl2::W](rfc_obs_ctrl2::W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL2 {}
#[doc = "RF observation mux control"]
pub mod rfc_obs_ctrl2;
#[doc = "TX filter configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txfiltcfg](txfiltcfg) module"]
pub type TXFILTCFG = crate::Reg<u32, _TXFILTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFILTCFG;
#[doc = "`read()` method returns [txfiltcfg::R](txfiltcfg::R) reader structure"]
impl crate::Readable for TXFILTCFG {}
#[doc = "`write(|w| ..)` method takes [txfiltcfg::W](txfiltcfg::W) writer structure"]
impl crate::Writable for TXFILTCFG {}
#[doc = "TX filter configuration"]
pub mod txfiltcfg;
