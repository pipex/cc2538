#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state."]
    pub stat: STAT,
    #[doc = "0x04 - DMA configuration The CFG register controls the configuration of the uDMA controller."]
    pub cfg: CFG,
    #[doc = "0x08 - DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state."]
    pub ctlbase: CTLBASE,
    #[doc = "0x0c - DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state."]
    pub altbase: ALTBASE,
    #[doc = "0x10 - DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state."]
    pub waitstat: WAITSTAT,
    #[doc = "0x14 - DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel."]
    pub swreq: SWREQ,
    #[doc = "0x18 - DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\] bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\] bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model."]
    pub useburstset: USEBURSTSET,
    #[doc = "0x1c - DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the USEBURSTSET register."]
    pub useburstclr: USEBURSTCLR,
    #[doc = "0x20 - DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers."]
    pub reqmaskset: REQMASKSET,
    #[doc = "0x24 - DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the REQMASKSET register."]
    pub reqmaskclr: REQMASKCLR,
    #[doc = "0x28 - DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers."]
    pub enaset: ENASET,
    #[doc = "0x2c - DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the ENASET register."]
    pub enaclr: ENACLR,
    #[doc = "0x30 - DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel."]
    pub altset: ALTSET,
    #[doc = "0x34 - DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the ALTSET register."]
    pub altclr: ALTCLR,
    #[doc = "0x38 - DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask."]
    pub prioset: PRIOSET,
    #[doc = "0x3c - DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the PRIOSET register."]
    pub prioclr: PRIOCLR,
    _reserved16: [u8; 12usize],
    #[doc = "0x4c - DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected."]
    pub errclr: ERRCLR,
    _reserved17: [u8; 1200usize],
    #[doc = "0x500 - DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\""]
    pub chasgn: CHASGN,
    #[doc = "0x504 - DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1."]
    pub chis: CHIS,
    _reserved19: [u8; 8usize],
    #[doc = "0x510 - DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap0: CHMAP0,
    #[doc = "0x514 - DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap1: CHMAP1,
    #[doc = "0x518 - DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap2: CHMAP2,
    #[doc = "0x51c - DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
    pub chmap3: CHMAP3,
}
#[doc = "DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state."]
pub mod stat;
#[doc = "DMA configuration The CFG register controls the configuration of the uDMA controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "DMA configuration The CFG register controls the configuration of the uDMA controller."]
pub mod cfg;
#[doc = "DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctlbase](ctlbase) module"]
pub type CTLBASE = crate::Reg<u32, _CTLBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLBASE;
#[doc = "`read()` method returns [ctlbase::R](ctlbase::R) reader structure"]
impl crate::Readable for CTLBASE {}
#[doc = "`write(|w| ..)` method takes [ctlbase::W](ctlbase::W) writer structure"]
impl crate::Writable for CTLBASE {}
#[doc = "DMA channel control base pointer The CTLBASE register must be configured so that the base pointer points to a location in system memory. The amount of system memory that must be assigned to the uDMA controller depends on the number of uDMA channels used and whether the alternate channel control data structure is used. See Section 10.2.5 for details about the Channel Control Table. The base address must be aligned on a 1024-byte boundary. This register cannot be read when the uDMA controller is in the reset state."]
pub mod ctlbase;
#[doc = "DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altbase](altbase) module"]
pub type ALTBASE = crate::Reg<u32, _ALTBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTBASE;
#[doc = "`read()` method returns [altbase::R](altbase::R) reader structure"]
impl crate::Readable for ALTBASE {}
#[doc = "`write(|w| ..)` method takes [altbase::W](altbase::W) writer structure"]
impl crate::Writable for ALTBASE {}
#[doc = "DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state."]
pub mod altbase;
#[doc = "DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [waitstat](waitstat) module"]
pub type WAITSTAT = crate::Reg<u32, _WAITSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAITSTAT;
#[doc = "`read()` method returns [waitstat::R](waitstat::R) reader structure"]
impl crate::Readable for WAITSTAT {}
#[doc = "`write(|w| ..)` method takes [waitstat::W](waitstat::W) writer structure"]
impl crate::Writable for WAITSTAT {}
#[doc = "DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state."]
pub mod waitstat;
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swreq](swreq) module"]
pub type SWREQ = crate::Reg<u32, _SWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWREQ;
#[doc = "`read()` method returns [swreq::R](swreq::R) reader structure"]
impl crate::Readable for SWREQ {}
#[doc = "`write(|w| ..)` method takes [swreq::W](swreq::W) writer structure"]
impl crate::Writable for SWREQ {}
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel."]
pub mod swreq;
#[doc = "DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\] bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\] bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [useburstset](useburstset) module"]
pub type USEBURSTSET = crate::Reg<u32, _USEBURSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USEBURSTSET;
#[doc = "`read()` method returns [useburstset::R](useburstset::R) reader structure"]
impl crate::Readable for USEBURSTSET {}
#[doc = "`write(|w| ..)` method takes [useburstset::W](useburstset::W) writer structure"]
impl crate::Writable for USEBURSTSET {}
#[doc = "DMA channel useburst set Each bit of the USEBURSTSET register represents the corresponding uDMA channel. Setting a bit disables the channel single request input from generating requests, configuring the channel to only accept burst requests. Reading the register returns the status of USEBURST. If the amount of data to transfer is a multiple of the arbitration (burst) size, the corresponding SET\\[n\\] bit is cleared after completing the final transfer. If there are fewer items remaining to transfer than the arbitration (burst) size, the uDMA controller automatically clears the corresponding SET\\[n\\] bit, allowing the remaining items to transfer using single requests. To resume transfers using burst requests, the corresponding bit must be set again. A bit must not be set if the corresponding peripheral does not support the burst request model."]
pub mod useburstset;
#[doc = "DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the USEBURSTSET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [useburstclr](useburstclr) module"]
pub type USEBURSTCLR = crate::Reg<u32, _USEBURSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USEBURSTCLR;
#[doc = "`read()` method returns [useburstclr::R](useburstclr::R) reader structure"]
impl crate::Readable for USEBURSTCLR {}
#[doc = "`write(|w| ..)` method takes [useburstclr::W](useburstclr::W) writer structure"]
impl crate::Writable for USEBURSTCLR {}
#[doc = "DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the USEBURSTSET register."]
pub mod useburstclr;
#[doc = "DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reqmaskset](reqmaskset) module"]
pub type REQMASKSET = crate::Reg<u32, _REQMASKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQMASKSET;
#[doc = "`read()` method returns [reqmaskset::R](reqmaskset::R) reader structure"]
impl crate::Readable for REQMASKSET {}
#[doc = "`write(|w| ..)` method takes [reqmaskset::W](reqmaskset::W) writer structure"]
impl crate::Writable for REQMASKSET {}
#[doc = "DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers."]
pub mod reqmaskset;
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the REQMASKSET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reqmaskclr](reqmaskclr) module"]
pub type REQMASKCLR = crate::Reg<u32, _REQMASKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQMASKCLR;
#[doc = "`read()` method returns [reqmaskclr::R](reqmaskclr::R) reader structure"]
impl crate::Readable for REQMASKCLR {}
#[doc = "`write(|w| ..)` method takes [reqmaskclr::W](reqmaskclr::W) writer structure"]
impl crate::Writable for REQMASKCLR {}
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the REQMASKSET register."]
pub mod reqmaskclr;
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enaset](enaset) module"]
pub type ENASET = crate::Reg<u32, _ENASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENASET;
#[doc = "`read()` method returns [enaset::R](enaset::R) reader structure"]
impl crate::Readable for ENASET {}
#[doc = "`write(|w| ..)` method takes [enaset::W](enaset::W) writer structure"]
impl crate::Writable for ENASET {}
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers."]
pub mod enaset;
#[doc = "DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the ENASET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enaclr](enaclr) module"]
pub type ENACLR = crate::Reg<u32, _ENACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENACLR;
#[doc = "`read()` method returns [enaclr::R](enaclr::R) reader structure"]
impl crate::Readable for ENACLR {}
#[doc = "`write(|w| ..)` method takes [enaclr::W](enaclr::W) writer structure"]
impl crate::Writable for ENACLR {}
#[doc = "DMA channel enable clear Each bit of the ENACLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the ENASET register."]
pub mod enaclr;
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altset](altset) module"]
pub type ALTSET = crate::Reg<u32, _ALTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTSET;
#[doc = "`read()` method returns [altset::R](altset::R) reader structure"]
impl crate::Readable for ALTSET {}
#[doc = "`write(|w| ..)` method takes [altset::W](altset::W) writer structure"]
impl crate::Writable for ALTSET {}
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel."]
pub mod altset;
#[doc = "DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the ALTSET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [altclr](altclr) module"]
pub type ALTCLR = crate::Reg<u32, _ALTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTCLR;
#[doc = "`read()` method returns [altclr::R](altclr::R) reader structure"]
impl crate::Readable for ALTCLR {}
#[doc = "`write(|w| ..)` method takes [altclr::W](altclr::W) writer structure"]
impl crate::Writable for ALTCLR {}
#[doc = "DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the ALTSET register."]
pub mod altclr;
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prioset](prioset) module"]
pub type PRIOSET = crate::Reg<u32, _PRIOSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIOSET;
#[doc = "`read()` method returns [prioset::R](prioset::R) reader structure"]
impl crate::Readable for PRIOSET {}
#[doc = "`write(|w| ..)` method takes [prioset::W](prioset::W) writer structure"]
impl crate::Writable for PRIOSET {}
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask."]
pub mod prioset;
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the PRIOSET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prioclr](prioclr) module"]
pub type PRIOCLR = crate::Reg<u32, _PRIOCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIOCLR;
#[doc = "`read()` method returns [prioclr::R](prioclr::R) reader structure"]
impl crate::Readable for PRIOCLR {}
#[doc = "`write(|w| ..)` method takes [prioclr::W](prioclr::W) writer structure"]
impl crate::Writable for PRIOCLR {}
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\] bit in the PRIOSET register."]
pub mod prioclr;
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [errclr](errclr) module"]
pub type ERRCLR = crate::Reg<u32, _ERRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRCLR;
#[doc = "`read()` method returns [errclr::R](errclr::R) reader structure"]
impl crate::Readable for ERRCLR {}
#[doc = "`write(|w| ..)` method takes [errclr::W](errclr::W) writer structure"]
impl crate::Writable for ERRCLR {}
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected."]
pub mod errclr;
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\"\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chasgn](chasgn) module"]
pub type CHASGN = crate::Reg<u32, _CHASGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHASGN;
#[doc = "`read()` method returns [chasgn::R](chasgn::R) reader structure"]
impl crate::Readable for CHASGN {}
#[doc = "`write(|w| ..)` method takes [chasgn::W](chasgn::W) writer structure"]
impl crate::Writable for CHASGN {}
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\""]
pub mod chasgn;
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chis](chis) module"]
pub type CHIS = crate::Reg<u32, _CHIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIS;
#[doc = "`read()` method returns [chis::R](chis::R) reader structure"]
impl crate::Readable for CHIS {}
#[doc = "`write(|w| ..)` method takes [chis::W](chis::W) writer structure"]
impl crate::Writable for CHIS {}
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1."]
pub mod chis;
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chmap0](chmap0) module"]
pub type CHMAP0 = crate::Reg<u32, _CHMAP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP0;
#[doc = "`read()` method returns [chmap0::R](chmap0::R) reader structure"]
impl crate::Readable for CHMAP0 {}
#[doc = "`write(|w| ..)` method takes [chmap0::W](chmap0::W) writer structure"]
impl crate::Writable for CHMAP0 {}
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap0;
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chmap1](chmap1) module"]
pub type CHMAP1 = crate::Reg<u32, _CHMAP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP1;
#[doc = "`read()` method returns [chmap1::R](chmap1::R) reader structure"]
impl crate::Readable for CHMAP1 {}
#[doc = "`write(|w| ..)` method takes [chmap1::W](chmap1::W) writer structure"]
impl crate::Writable for CHMAP1 {}
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap1;
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chmap2](chmap2) module"]
pub type CHMAP2 = crate::Reg<u32, _CHMAP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP2;
#[doc = "`read()` method returns [chmap2::R](chmap2::R) reader structure"]
impl crate::Readable for CHMAP2 {}
#[doc = "`write(|w| ..)` method takes [chmap2::W](chmap2::W) writer structure"]
impl crate::Writable for CHMAP2 {}
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap2;
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chmap3](chmap3) module"]
pub type CHMAP3 = crate::Reg<u32, _CHMAP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHMAP3;
#[doc = "`read()` method returns [chmap3::R](chmap3::R) reader structure"]
impl crate::Readable for CHMAP3 {}
#[doc = "`write(|w| ..)` method takes [chmap3::W](chmap3::W) writer structure"]
impl crate::Writable for CHMAP3 {}
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section."]
pub mod chmap3;
