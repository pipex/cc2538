#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    pub wdctl: WDCTL,
    _reserved1: [u8; 60usize],
    #[doc = "0x40 - Sleep Timer 0 count and compare"]
    pub st0: ST0,
    #[doc = "0x44 - Sleep Timer 1 count and compare"]
    pub st1: ST1,
    #[doc = "0x48 - Sleep Timer 2 count and compare"]
    pub st2: ST2,
    #[doc = "0x4c - Sleep Timer 3 count and compare"]
    pub st3: ST3,
    #[doc = "0x50 - Sleep Timer load status"]
    pub stload: STLOAD,
    #[doc = "0x54 - Sleep Timer Capture control"]
    pub stcc: STCC,
    #[doc = "0x58 - Sleep Timer Capture status"]
    pub stcs: STCS,
    #[doc = "0x5c - Sleep Timer Capture value byte 0"]
    pub stcv0: STCV0,
    #[doc = "0x60 - Sleep Timer Capture value byte 1"]
    pub stcv1: STCV1,
    #[doc = "0x64 - Sleep Timer Capture value byte 2"]
    pub stcv2: STCV2,
    #[doc = "0x68 - Sleep Timer Capture value byte 3"]
    pub stcv3: STCV3,
}
#[doc = "Watchdog Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wdctl](wdctl) module"]
pub type WDCTL = crate::Reg<u32, _WDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDCTL;
#[doc = "`read()` method returns [wdctl::R](wdctl::R) reader structure"]
impl crate::Readable for WDCTL {}
#[doc = "`write(|w| ..)` method takes [wdctl::W](wdctl::W) writer structure"]
impl crate::Writable for WDCTL {}
#[doc = "Watchdog Timer Control"]
pub mod wdctl;
#[doc = "Sleep Timer 0 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [st0](st0) module"]
pub type ST0 = crate::Reg<u32, _ST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST0;
#[doc = "`read()` method returns [st0::R](st0::R) reader structure"]
impl crate::Readable for ST0 {}
#[doc = "`write(|w| ..)` method takes [st0::W](st0::W) writer structure"]
impl crate::Writable for ST0 {}
#[doc = "Sleep Timer 0 count and compare"]
pub mod st0;
#[doc = "Sleep Timer 1 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [st1](st1) module"]
pub type ST1 = crate::Reg<u32, _ST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST1;
#[doc = "`read()` method returns [st1::R](st1::R) reader structure"]
impl crate::Readable for ST1 {}
#[doc = "`write(|w| ..)` method takes [st1::W](st1::W) writer structure"]
impl crate::Writable for ST1 {}
#[doc = "Sleep Timer 1 count and compare"]
pub mod st1;
#[doc = "Sleep Timer 2 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [st2](st2) module"]
pub type ST2 = crate::Reg<u32, _ST2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST2;
#[doc = "`read()` method returns [st2::R](st2::R) reader structure"]
impl crate::Readable for ST2 {}
#[doc = "`write(|w| ..)` method takes [st2::W](st2::W) writer structure"]
impl crate::Writable for ST2 {}
#[doc = "Sleep Timer 2 count and compare"]
pub mod st2;
#[doc = "Sleep Timer 3 count and compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [st3](st3) module"]
pub type ST3 = crate::Reg<u32, _ST3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ST3;
#[doc = "`read()` method returns [st3::R](st3::R) reader structure"]
impl crate::Readable for ST3 {}
#[doc = "`write(|w| ..)` method takes [st3::W](st3::W) writer structure"]
impl crate::Writable for ST3 {}
#[doc = "Sleep Timer 3 count and compare"]
pub mod st3;
#[doc = "Sleep Timer load status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stload](stload) module"]
pub type STLOAD = crate::Reg<u32, _STLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STLOAD;
#[doc = "`read()` method returns [stload::R](stload::R) reader structure"]
impl crate::Readable for STLOAD {}
#[doc = "`write(|w| ..)` method takes [stload::W](stload::W) writer structure"]
impl crate::Writable for STLOAD {}
#[doc = "Sleep Timer load status"]
pub mod stload;
#[doc = "Sleep Timer Capture control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcc](stcc) module"]
pub type STCC = crate::Reg<u32, _STCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCC;
#[doc = "`read()` method returns [stcc::R](stcc::R) reader structure"]
impl crate::Readable for STCC {}
#[doc = "`write(|w| ..)` method takes [stcc::W](stcc::W) writer structure"]
impl crate::Writable for STCC {}
#[doc = "Sleep Timer Capture control"]
pub mod stcc;
#[doc = "Sleep Timer Capture status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcs](stcs) module"]
pub type STCS = crate::Reg<u32, _STCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCS;
#[doc = "`read()` method returns [stcs::R](stcs::R) reader structure"]
impl crate::Readable for STCS {}
#[doc = "`write(|w| ..)` method takes [stcs::W](stcs::W) writer structure"]
impl crate::Writable for STCS {}
#[doc = "Sleep Timer Capture status"]
pub mod stcs;
#[doc = "Sleep Timer Capture value byte 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcv0](stcv0) module"]
pub type STCV0 = crate::Reg<u32, _STCV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCV0;
#[doc = "`read()` method returns [stcv0::R](stcv0::R) reader structure"]
impl crate::Readable for STCV0 {}
#[doc = "`write(|w| ..)` method takes [stcv0::W](stcv0::W) writer structure"]
impl crate::Writable for STCV0 {}
#[doc = "Sleep Timer Capture value byte 0"]
pub mod stcv0;
#[doc = "Sleep Timer Capture value byte 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcv1](stcv1) module"]
pub type STCV1 = crate::Reg<u32, _STCV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCV1;
#[doc = "`read()` method returns [stcv1::R](stcv1::R) reader structure"]
impl crate::Readable for STCV1 {}
#[doc = "`write(|w| ..)` method takes [stcv1::W](stcv1::W) writer structure"]
impl crate::Writable for STCV1 {}
#[doc = "Sleep Timer Capture value byte 1"]
pub mod stcv1;
#[doc = "Sleep Timer Capture value byte 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcv2](stcv2) module"]
pub type STCV2 = crate::Reg<u32, _STCV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCV2;
#[doc = "`read()` method returns [stcv2::R](stcv2::R) reader structure"]
impl crate::Readable for STCV2 {}
#[doc = "`write(|w| ..)` method takes [stcv2::W](stcv2::W) writer structure"]
impl crate::Writable for STCV2 {}
#[doc = "Sleep Timer Capture value byte 2"]
pub mod stcv2;
#[doc = "Sleep Timer Capture value byte 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stcv3](stcv3) module"]
pub type STCV3 = crate::Reg<u32, _STCV3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCV3;
#[doc = "`read()` method returns [stcv3::R](stcv3::R) reader structure"]
impl crate::Readable for STCV3 {}
#[doc = "`write(|w| ..)` method takes [stcv3::W](stcv3::W) writer structure"]
impl crate::Writable for STCV3 {}
#[doc = "Sleep Timer Capture value byte 3"]
pub mod stcv3;
