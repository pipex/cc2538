#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Flash control This register provides control and monitoring functions for the flash module."]
    pub fctl: FCTL,
    #[doc = "0x0c - Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
    pub faddr: FADDR,
    #[doc = "0x10 - Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
    pub fwdata: FWDATA,
    #[doc = "0x14 - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
    pub diecfg0: DIECFG0,
    #[doc = "0x18 - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
    pub diecfg1: DIECFG1,
    #[doc = "0x1c - These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
    pub diecfg2: DIECFG2,
}
#[doc = "Flash control This register provides control and monitoring functions for the flash module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fctl](fctl) module"]
pub type FCTL = crate::Reg<u32, _FCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL;
#[doc = "`read()` method returns [fctl::R](fctl::R) reader structure"]
impl crate::Readable for FCTL {}
#[doc = "`write(|w| ..)` method takes [fctl::W](fctl::W) writer structure"]
impl crate::Writable for FCTL {}
#[doc = "Flash control This register provides control and monitoring functions for the flash module."]
pub mod fctl;
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [faddr](faddr) module"]
pub type FADDR = crate::Reg<u32, _FADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FADDR;
#[doc = "`read()` method returns [faddr::R](faddr::R) reader structure"]
impl crate::Readable for FADDR {}
#[doc = "`write(|w| ..)` method takes [faddr::W](faddr::W) writer structure"]
impl crate::Writable for FADDR {}
#[doc = "Flash address The register sets the address to be written in flash memory. See the bitfield descriptions for formatting information."]
pub mod faddr;
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fwdata](fwdata) module"]
pub type FWDATA = crate::Reg<u32, _FWDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWDATA;
#[doc = "`read()` method returns [fwdata::R](fwdata::R) reader structure"]
impl crate::Readable for FWDATA {}
#[doc = "`write(|w| ..)` method takes [fwdata::W](fwdata::W) writer structure"]
impl crate::Writable for FWDATA {}
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR."]
pub mod fwdata;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diecfg0](diecfg0) module"]
pub type DIECFG0 = crate::Reg<u32, _DIECFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIECFG0;
#[doc = "`read()` method returns [diecfg0::R](diecfg0::R) reader structure"]
impl crate::Readable for DIECFG0 {}
#[doc = "`write(|w| ..)` method takes [diecfg0::W](diecfg0::W) writer structure"]
impl crate::Writable for DIECFG0 {}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg0;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diecfg1](diecfg1) module"]
pub type DIECFG1 = crate::Reg<u32, _DIECFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIECFG1;
#[doc = "`read()` method returns [diecfg1::R](diecfg1::R) reader structure"]
impl crate::Readable for DIECFG1 {}
#[doc = "`write(|w| ..)` method takes [diecfg1::W](diecfg1::W) writer structure"]
impl crate::Writable for DIECFG1 {}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538."]
pub mod diecfg1;
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diecfg2](diecfg2) module"]
pub type DIECFG2 = crate::Reg<u32, _DIECFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIECFG2;
#[doc = "`read()` method returns [diecfg2::R](diecfg2::R) reader structure"]
impl crate::Readable for DIECFG2 {}
#[doc = "`write(|w| ..)` method takes [diecfg2::W](diecfg2::W) writer structure"]
impl crate::Writable for DIECFG2 {}
#[doc = "These settings are a function of the FLASH information page bit settings, which are programmed during production test, and are subject for specific configuration for multiple device flavors of cc2538. The DIE_*_REVISION registers are an exeception to this, as they are hardwired and are not part of the FLASH information page."]
pub mod diecfg2;
