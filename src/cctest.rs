#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output strength control"]
    pub io: IO,
    _reserved1: [u8; 16usize],
    #[doc = "0x14 - Select output signal on observation output 0"]
    pub obssel0: OBSSEL0,
    #[doc = "0x18 - Select output signal on observation output 1"]
    pub obssel1: OBSSEL1,
    #[doc = "0x1c - Select output signal on observation output 2"]
    pub obssel2: OBSSEL2,
    #[doc = "0x20 - Select output signal on observation output 3"]
    pub obssel3: OBSSEL3,
    #[doc = "0x24 - Select output signal on observation output 4"]
    pub obssel4: OBSSEL4,
    #[doc = "0x28 - Select output signal on observation output 5"]
    pub obssel5: OBSSEL5,
    #[doc = "0x2c - Select output signal on observation output 6"]
    pub obssel6: OBSSEL6,
    #[doc = "0x30 - Select output signal on observation output 7"]
    pub obssel7: OBSSEL7,
    #[doc = "0x34 - Test register 0"]
    pub tr0: TR0,
    _reserved10: [u8; 24usize],
    #[doc = "0x50 - USB PHY stand-by control"]
    pub usbctrl: USBCTRL,
}
#[doc = "Output strength control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [io](io) module"]
pub type IO = crate::Reg<u32, _IO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO;
#[doc = "`read()` method returns [io::R](io::R) reader structure"]
impl crate::Readable for IO {}
#[doc = "`write(|w| ..)` method takes [io::W](io::W) writer structure"]
impl crate::Writable for IO {}
#[doc = "Output strength control"]
pub mod io;
#[doc = "Select output signal on observation output 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel0](obssel0) module"]
pub type OBSSEL0 = crate::Reg<u32, _OBSSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL0;
#[doc = "`read()` method returns [obssel0::R](obssel0::R) reader structure"]
impl crate::Readable for OBSSEL0 {}
#[doc = "`write(|w| ..)` method takes [obssel0::W](obssel0::W) writer structure"]
impl crate::Writable for OBSSEL0 {}
#[doc = "Select output signal on observation output 0"]
pub mod obssel0;
#[doc = "Select output signal on observation output 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel1](obssel1) module"]
pub type OBSSEL1 = crate::Reg<u32, _OBSSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL1;
#[doc = "`read()` method returns [obssel1::R](obssel1::R) reader structure"]
impl crate::Readable for OBSSEL1 {}
#[doc = "`write(|w| ..)` method takes [obssel1::W](obssel1::W) writer structure"]
impl crate::Writable for OBSSEL1 {}
#[doc = "Select output signal on observation output 1"]
pub mod obssel1;
#[doc = "Select output signal on observation output 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel2](obssel2) module"]
pub type OBSSEL2 = crate::Reg<u32, _OBSSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL2;
#[doc = "`read()` method returns [obssel2::R](obssel2::R) reader structure"]
impl crate::Readable for OBSSEL2 {}
#[doc = "`write(|w| ..)` method takes [obssel2::W](obssel2::W) writer structure"]
impl crate::Writable for OBSSEL2 {}
#[doc = "Select output signal on observation output 2"]
pub mod obssel2;
#[doc = "Select output signal on observation output 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel3](obssel3) module"]
pub type OBSSEL3 = crate::Reg<u32, _OBSSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL3;
#[doc = "`read()` method returns [obssel3::R](obssel3::R) reader structure"]
impl crate::Readable for OBSSEL3 {}
#[doc = "`write(|w| ..)` method takes [obssel3::W](obssel3::W) writer structure"]
impl crate::Writable for OBSSEL3 {}
#[doc = "Select output signal on observation output 3"]
pub mod obssel3;
#[doc = "Select output signal on observation output 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel4](obssel4) module"]
pub type OBSSEL4 = crate::Reg<u32, _OBSSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL4;
#[doc = "`read()` method returns [obssel4::R](obssel4::R) reader structure"]
impl crate::Readable for OBSSEL4 {}
#[doc = "`write(|w| ..)` method takes [obssel4::W](obssel4::W) writer structure"]
impl crate::Writable for OBSSEL4 {}
#[doc = "Select output signal on observation output 4"]
pub mod obssel4;
#[doc = "Select output signal on observation output 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel5](obssel5) module"]
pub type OBSSEL5 = crate::Reg<u32, _OBSSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL5;
#[doc = "`read()` method returns [obssel5::R](obssel5::R) reader structure"]
impl crate::Readable for OBSSEL5 {}
#[doc = "`write(|w| ..)` method takes [obssel5::W](obssel5::W) writer structure"]
impl crate::Writable for OBSSEL5 {}
#[doc = "Select output signal on observation output 5"]
pub mod obssel5;
#[doc = "Select output signal on observation output 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel6](obssel6) module"]
pub type OBSSEL6 = crate::Reg<u32, _OBSSEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL6;
#[doc = "`read()` method returns [obssel6::R](obssel6::R) reader structure"]
impl crate::Readable for OBSSEL6 {}
#[doc = "`write(|w| ..)` method takes [obssel6::W](obssel6::W) writer structure"]
impl crate::Writable for OBSSEL6 {}
#[doc = "Select output signal on observation output 6"]
pub mod obssel6;
#[doc = "Select output signal on observation output 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [obssel7](obssel7) module"]
pub type OBSSEL7 = crate::Reg<u32, _OBSSEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSSEL7;
#[doc = "`read()` method returns [obssel7::R](obssel7::R) reader structure"]
impl crate::Readable for OBSSEL7 {}
#[doc = "`write(|w| ..)` method takes [obssel7::W](obssel7::W) writer structure"]
impl crate::Writable for OBSSEL7 {}
#[doc = "Select output signal on observation output 7"]
pub mod obssel7;
#[doc = "Test register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tr0](tr0) module"]
pub type TR0 = crate::Reg<u32, _TR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR0;
#[doc = "`read()` method returns [tr0::R](tr0::R) reader structure"]
impl crate::Readable for TR0 {}
#[doc = "`write(|w| ..)` method takes [tr0::W](tr0::W) writer structure"]
impl crate::Writable for TR0 {}
#[doc = "Test register 0"]
pub mod tr0;
#[doc = "USB PHY stand-by control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbctrl](usbctrl) module"]
pub type USBCTRL = crate::Reg<u32, _USBCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCTRL;
#[doc = "`read()` method returns [usbctrl::R](usbctrl::R) reader structure"]
impl crate::Readable for USBCTRL {}
#[doc = "`write(|w| ..)` method takes [usbctrl::W](usbctrl::W) writer structure"]
impl crate::Writable for USBCTRL {}
#[doc = "USB PHY stand-by control"]
pub mod usbctrl;
