#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresmask0: SRCRESMASK0,
    #[doc = "0x84 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresmask1: SRCRESMASK1,
    #[doc = "0x88 - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresmask2: SRCRESMASK2,
    #[doc = "0x8c - Source address matching result This register is stored in RAM; the reset value is undefined."]
    pub srcresindex: SRCRESINDEX,
    #[doc = "0x90 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcextpenden0: SRCEXTPENDEN0,
    #[doc = "0x94 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcextpenden1: SRCEXTPENDEN1,
    #[doc = "0x98 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcextpenden2: SRCEXTPENDEN2,
    #[doc = "0x9c - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcshortpenden0: SRCSHORTPENDEN0,
    #[doc = "0xa0 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcshortpenden1: SRCSHORTPENDEN1,
    #[doc = "0xa4 - Source address matching control This register is stored in RAM; the reset value is undefined."]
    pub srcshortpenden2: SRCSHORTPENDEN2,
    #[doc = "0xa8 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr0: EXT_ADDR0,
    #[doc = "0xac - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr1: EXT_ADDR1,
    #[doc = "0xb0 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr2: EXT_ADDR2,
    #[doc = "0xb4 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr3: EXT_ADDR3,
    #[doc = "0xb8 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr4: EXT_ADDR4,
    #[doc = "0xbc - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr5: EXT_ADDR5,
    #[doc = "0xc0 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr6: EXT_ADDR6,
    #[doc = "0xc4 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub ext_addr7: EXT_ADDR7,
    #[doc = "0xc8 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub pan_id0: PAN_ID0,
    #[doc = "0xcc - Local address information This register is stored in RAM; the reset value is undefined."]
    pub pan_id1: PAN_ID1,
    #[doc = "0xd0 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub short_addr0: SHORT_ADDR0,
    #[doc = "0xd4 - Local address information This register is stored in RAM; the reset value is undefined."]
    pub short_addr1: SHORT_ADDR1,
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcresmask0](srcresmask0) module"]
pub type SRCRESMASK0 = crate::Reg<u32, _SRCRESMASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCRESMASK0;
#[doc = "`read()` method returns [srcresmask0::R](srcresmask0::R) reader structure"]
impl crate::Readable for SRCRESMASK0 {}
#[doc = "`write(|w| ..)` method takes [srcresmask0::W](srcresmask0::W) writer structure"]
impl crate::Writable for SRCRESMASK0 {}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask0;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcresmask1](srcresmask1) module"]
pub type SRCRESMASK1 = crate::Reg<u32, _SRCRESMASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCRESMASK1;
#[doc = "`read()` method returns [srcresmask1::R](srcresmask1::R) reader structure"]
impl crate::Readable for SRCRESMASK1 {}
#[doc = "`write(|w| ..)` method takes [srcresmask1::W](srcresmask1::W) writer structure"]
impl crate::Writable for SRCRESMASK1 {}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask1;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcresmask2](srcresmask2) module"]
pub type SRCRESMASK2 = crate::Reg<u32, _SRCRESMASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCRESMASK2;
#[doc = "`read()` method returns [srcresmask2::R](srcresmask2::R) reader structure"]
impl crate::Readable for SRCRESMASK2 {}
#[doc = "`write(|w| ..)` method takes [srcresmask2::W](srcresmask2::W) writer structure"]
impl crate::Writable for SRCRESMASK2 {}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresmask2;
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcresindex](srcresindex) module"]
pub type SRCRESINDEX = crate::Reg<u32, _SRCRESINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCRESINDEX;
#[doc = "`read()` method returns [srcresindex::R](srcresindex::R) reader structure"]
impl crate::Readable for SRCRESINDEX {}
#[doc = "`write(|w| ..)` method takes [srcresindex::W](srcresindex::W) writer structure"]
impl crate::Writable for SRCRESINDEX {}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined."]
pub mod srcresindex;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcextpenden0](srcextpenden0) module"]
pub type SRCEXTPENDEN0 = crate::Reg<u32, _SRCEXTPENDEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCEXTPENDEN0;
#[doc = "`read()` method returns [srcextpenden0::R](srcextpenden0::R) reader structure"]
impl crate::Readable for SRCEXTPENDEN0 {}
#[doc = "`write(|w| ..)` method takes [srcextpenden0::W](srcextpenden0::W) writer structure"]
impl crate::Writable for SRCEXTPENDEN0 {}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden0;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcextpenden1](srcextpenden1) module"]
pub type SRCEXTPENDEN1 = crate::Reg<u32, _SRCEXTPENDEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCEXTPENDEN1;
#[doc = "`read()` method returns [srcextpenden1::R](srcextpenden1::R) reader structure"]
impl crate::Readable for SRCEXTPENDEN1 {}
#[doc = "`write(|w| ..)` method takes [srcextpenden1::W](srcextpenden1::W) writer structure"]
impl crate::Writable for SRCEXTPENDEN1 {}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden1;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcextpenden2](srcextpenden2) module"]
pub type SRCEXTPENDEN2 = crate::Reg<u32, _SRCEXTPENDEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCEXTPENDEN2;
#[doc = "`read()` method returns [srcextpenden2::R](srcextpenden2::R) reader structure"]
impl crate::Readable for SRCEXTPENDEN2 {}
#[doc = "`write(|w| ..)` method takes [srcextpenden2::W](srcextpenden2::W) writer structure"]
impl crate::Writable for SRCEXTPENDEN2 {}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcextpenden2;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcshortpenden0](srcshortpenden0) module"]
pub type SRCSHORTPENDEN0 = crate::Reg<u32, _SRCSHORTPENDEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSHORTPENDEN0;
#[doc = "`read()` method returns [srcshortpenden0::R](srcshortpenden0::R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN0 {}
#[doc = "`write(|w| ..)` method takes [srcshortpenden0::W](srcshortpenden0::W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN0 {}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden0;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcshortpenden1](srcshortpenden1) module"]
pub type SRCSHORTPENDEN1 = crate::Reg<u32, _SRCSHORTPENDEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSHORTPENDEN1;
#[doc = "`read()` method returns [srcshortpenden1::R](srcshortpenden1::R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN1 {}
#[doc = "`write(|w| ..)` method takes [srcshortpenden1::W](srcshortpenden1::W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN1 {}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden1;
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srcshortpenden2](srcshortpenden2) module"]
pub type SRCSHORTPENDEN2 = crate::Reg<u32, _SRCSHORTPENDEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCSHORTPENDEN2;
#[doc = "`read()` method returns [srcshortpenden2::R](srcshortpenden2::R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN2 {}
#[doc = "`write(|w| ..)` method takes [srcshortpenden2::W](srcshortpenden2::W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN2 {}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined."]
pub mod srcshortpenden2;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr0](ext_addr0) module"]
pub type EXT_ADDR0 = crate::Reg<u32, _EXT_ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR0;
#[doc = "`read()` method returns [ext_addr0::R](ext_addr0::R) reader structure"]
impl crate::Readable for EXT_ADDR0 {}
#[doc = "`write(|w| ..)` method takes [ext_addr0::W](ext_addr0::W) writer structure"]
impl crate::Writable for EXT_ADDR0 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr0;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr1](ext_addr1) module"]
pub type EXT_ADDR1 = crate::Reg<u32, _EXT_ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR1;
#[doc = "`read()` method returns [ext_addr1::R](ext_addr1::R) reader structure"]
impl crate::Readable for EXT_ADDR1 {}
#[doc = "`write(|w| ..)` method takes [ext_addr1::W](ext_addr1::W) writer structure"]
impl crate::Writable for EXT_ADDR1 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr1;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr2](ext_addr2) module"]
pub type EXT_ADDR2 = crate::Reg<u32, _EXT_ADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR2;
#[doc = "`read()` method returns [ext_addr2::R](ext_addr2::R) reader structure"]
impl crate::Readable for EXT_ADDR2 {}
#[doc = "`write(|w| ..)` method takes [ext_addr2::W](ext_addr2::W) writer structure"]
impl crate::Writable for EXT_ADDR2 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr2;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr3](ext_addr3) module"]
pub type EXT_ADDR3 = crate::Reg<u32, _EXT_ADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR3;
#[doc = "`read()` method returns [ext_addr3::R](ext_addr3::R) reader structure"]
impl crate::Readable for EXT_ADDR3 {}
#[doc = "`write(|w| ..)` method takes [ext_addr3::W](ext_addr3::W) writer structure"]
impl crate::Writable for EXT_ADDR3 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr3;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr4](ext_addr4) module"]
pub type EXT_ADDR4 = crate::Reg<u32, _EXT_ADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR4;
#[doc = "`read()` method returns [ext_addr4::R](ext_addr4::R) reader structure"]
impl crate::Readable for EXT_ADDR4 {}
#[doc = "`write(|w| ..)` method takes [ext_addr4::W](ext_addr4::W) writer structure"]
impl crate::Writable for EXT_ADDR4 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr4;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr5](ext_addr5) module"]
pub type EXT_ADDR5 = crate::Reg<u32, _EXT_ADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR5;
#[doc = "`read()` method returns [ext_addr5::R](ext_addr5::R) reader structure"]
impl crate::Readable for EXT_ADDR5 {}
#[doc = "`write(|w| ..)` method takes [ext_addr5::W](ext_addr5::W) writer structure"]
impl crate::Writable for EXT_ADDR5 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr5;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr6](ext_addr6) module"]
pub type EXT_ADDR6 = crate::Reg<u32, _EXT_ADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR6;
#[doc = "`read()` method returns [ext_addr6::R](ext_addr6::R) reader structure"]
impl crate::Readable for EXT_ADDR6 {}
#[doc = "`write(|w| ..)` method takes [ext_addr6::W](ext_addr6::W) writer structure"]
impl crate::Writable for EXT_ADDR6 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr6;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ext_addr7](ext_addr7) module"]
pub type EXT_ADDR7 = crate::Reg<u32, _EXT_ADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXT_ADDR7;
#[doc = "`read()` method returns [ext_addr7::R](ext_addr7::R) reader structure"]
impl crate::Readable for EXT_ADDR7 {}
#[doc = "`write(|w| ..)` method takes [ext_addr7::W](ext_addr7::W) writer structure"]
impl crate::Writable for EXT_ADDR7 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod ext_addr7;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pan_id0](pan_id0) module"]
pub type PAN_ID0 = crate::Reg<u32, _PAN_ID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAN_ID0;
#[doc = "`read()` method returns [pan_id0::R](pan_id0::R) reader structure"]
impl crate::Readable for PAN_ID0 {}
#[doc = "`write(|w| ..)` method takes [pan_id0::W](pan_id0::W) writer structure"]
impl crate::Writable for PAN_ID0 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id0;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pan_id1](pan_id1) module"]
pub type PAN_ID1 = crate::Reg<u32, _PAN_ID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAN_ID1;
#[doc = "`read()` method returns [pan_id1::R](pan_id1::R) reader structure"]
impl crate::Readable for PAN_ID1 {}
#[doc = "`write(|w| ..)` method takes [pan_id1::W](pan_id1::W) writer structure"]
impl crate::Writable for PAN_ID1 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod pan_id1;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [short_addr0](short_addr0) module"]
pub type SHORT_ADDR0 = crate::Reg<u32, _SHORT_ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORT_ADDR0;
#[doc = "`read()` method returns [short_addr0::R](short_addr0::R) reader structure"]
impl crate::Readable for SHORT_ADDR0 {}
#[doc = "`write(|w| ..)` method takes [short_addr0::W](short_addr0::W) writer structure"]
impl crate::Writable for SHORT_ADDR0 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr0;
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [short_addr1](short_addr1) module"]
pub type SHORT_ADDR1 = crate::Reg<u32, _SHORT_ADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORT_ADDR1;
#[doc = "`read()` method returns [short_addr1::R](short_addr1::R) reader structure"]
impl crate::Readable for SHORT_ADDR1 {}
#[doc = "`write(|w| ..)` method takes [short_addr1::W](short_addr1::W) writer structure"]
impl crate::Writable for SHORT_ADDR1 {}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined."]
pub mod short_addr1;
