#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register controls the ADC."]
    pub adccon1: ADCCON1,
    #[doc = "0x04 - This register controls the ADC."]
    pub adccon2: ADCCON2,
    #[doc = "0x08 - This register controls the ADC."]
    pub adccon3: ADCCON3,
    #[doc = "0x0c - This register contains the least-significant part of ADC conversion result."]
    pub adcl: ADCL,
    #[doc = "0x10 - This register contains the most-significant part of ADC conversion result."]
    pub adch: ADCH,
    #[doc = "0x14 - This registers contains random-number-generator data; low byte."]
    pub rndl: RNDL,
    #[doc = "0x18 - This register contains random-number-generator data; high byte."]
    pub rndh: RNDH,
    _reserved7: [u8; 8usize],
    #[doc = "0x24 - Analog comparator control and status register."]
    pub cmpctl: CMPCTL,
}
#[doc = "This register controls the ADC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adccon1](adccon1) module"]
pub type ADCCON1 = crate::Reg<u32, _ADCCON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCON1;
#[doc = "`read()` method returns [adccon1::R](adccon1::R) reader structure"]
impl crate::Readable for ADCCON1 {}
#[doc = "`write(|w| ..)` method takes [adccon1::W](adccon1::W) writer structure"]
impl crate::Writable for ADCCON1 {}
#[doc = "This register controls the ADC."]
pub mod adccon1;
#[doc = "This register controls the ADC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adccon2](adccon2) module"]
pub type ADCCON2 = crate::Reg<u32, _ADCCON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCON2;
#[doc = "`read()` method returns [adccon2::R](adccon2::R) reader structure"]
impl crate::Readable for ADCCON2 {}
#[doc = "`write(|w| ..)` method takes [adccon2::W](adccon2::W) writer structure"]
impl crate::Writable for ADCCON2 {}
#[doc = "This register controls the ADC."]
pub mod adccon2;
#[doc = "This register controls the ADC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adccon3](adccon3) module"]
pub type ADCCON3 = crate::Reg<u32, _ADCCON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCON3;
#[doc = "`read()` method returns [adccon3::R](adccon3::R) reader structure"]
impl crate::Readable for ADCCON3 {}
#[doc = "`write(|w| ..)` method takes [adccon3::W](adccon3::W) writer structure"]
impl crate::Writable for ADCCON3 {}
#[doc = "This register controls the ADC."]
pub mod adccon3;
#[doc = "This register contains the least-significant part of ADC conversion result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adcl](adcl) module"]
pub type ADCL = crate::Reg<u32, _ADCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCL;
#[doc = "`read()` method returns [adcl::R](adcl::R) reader structure"]
impl crate::Readable for ADCL {}
#[doc = "`write(|w| ..)` method takes [adcl::W](adcl::W) writer structure"]
impl crate::Writable for ADCL {}
#[doc = "This register contains the least-significant part of ADC conversion result."]
pub mod adcl;
#[doc = "This register contains the most-significant part of ADC conversion result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adch](adch) module"]
pub type ADCH = crate::Reg<u32, _ADCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCH;
#[doc = "`read()` method returns [adch::R](adch::R) reader structure"]
impl crate::Readable for ADCH {}
#[doc = "`write(|w| ..)` method takes [adch::W](adch::W) writer structure"]
impl crate::Writable for ADCH {}
#[doc = "This register contains the most-significant part of ADC conversion result."]
pub mod adch;
#[doc = "This registers contains random-number-generator data; low byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rndl](rndl) module"]
pub type RNDL = crate::Reg<u32, _RNDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNDL;
#[doc = "`read()` method returns [rndl::R](rndl::R) reader structure"]
impl crate::Readable for RNDL {}
#[doc = "`write(|w| ..)` method takes [rndl::W](rndl::W) writer structure"]
impl crate::Writable for RNDL {}
#[doc = "This registers contains random-number-generator data; low byte."]
pub mod rndl;
#[doc = "This register contains random-number-generator data; high byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rndh](rndh) module"]
pub type RNDH = crate::Reg<u32, _RNDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNDH;
#[doc = "`read()` method returns [rndh::R](rndh::R) reader structure"]
impl crate::Readable for RNDH {}
#[doc = "`write(|w| ..)` method takes [rndh::W](rndh::W) writer structure"]
impl crate::Writable for RNDH {}
#[doc = "This register contains random-number-generator data; high byte."]
pub mod rndh;
#[doc = "Analog comparator control and status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmpctl](cmpctl) module"]
pub type CMPCTL = crate::Reg<u32, _CMPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPCTL;
#[doc = "`read()` method returns [cmpctl::R](cmpctl::R) reader structure"]
impl crate::Readable for CMPCTL {}
#[doc = "`write(|w| ..)` method takes [cmpctl::W](cmpctl::W) writer structure"]
impl crate::Writable for CMPCTL {}
#[doc = "Analog comparator control and status register."]
pub mod cmpctl;
