#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus."]
    pub oar: OAR,
    _reserved_1_ctrl: [u8; 4usize],
    #[doc = "0x08 - I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state."]
    pub dr: DR,
    #[doc = "0x0c - I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub imr: IMR,
    #[doc = "0x10 - I2C slave raw interrupt status This register specifies whether an interrupt is pending."]
    pub ris: RIS,
    #[doc = "0x14 - I2C slave masked interrupt status This register specifies whether an interrupt was signaled."]
    pub mis: MIS,
    #[doc = "0x18 - I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data."]
    pub icr: ICR,
}
impl RegisterBlock {
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn ctrl(&self) -> &CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const CTRL) }
    }
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn ctrl_mut(&self) -> &mut CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut CTRL) }
    }
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn stat(&self) -> &STAT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const STAT) }
    }
    #[doc = "0x04 - I2C slave control and status This register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn stat_mut(&self) -> &mut STAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut STAT) }
    }
}
#[doc = "I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oar](oar) module"]
pub type OAR = crate::Reg<u32, _OAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OAR;
#[doc = "`read()` method returns [oar::R](oar::R) reader structure"]
impl crate::Readable for OAR {}
#[doc = "`write(|w| ..)` method takes [oar::W](oar::W) writer structure"]
impl crate::Writable for OAR {}
#[doc = "I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus."]
pub mod oar;
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read."]
pub mod stat;
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "I2C slave control and status This register functions as a control register when written, and a status register when read."]
pub mod ctrl;
#[doc = "I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "I2C slave data This register contains the data to be transmitted when in the slave transmit state, and the data received when in the slave receive state."]
pub mod dr;
#[doc = "I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "I2C slave interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod imr;
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "I2C slave raw interrupt status This register specifies whether an interrupt is pending."]
pub mod ris;
#[doc = "I2C slave masked interrupt status This register specifies whether an interrupt was signaled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "I2C slave masked interrupt status This register specifies whether an interrupt was signaled."]
pub mod mis;
#[doc = "I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data."]
pub mod icr;
