#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers)."]
    pub cfg: CFG,
    #[doc = "0x04 - GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored."]
    pub tamr: TAMR,
    #[doc = "0x08 - GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B."]
    pub tbmr: TBMR,
    #[doc = "0x0c - GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall."]
    pub ctl: CTL,
    #[doc = "0x10 - GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers."]
    pub sync: SYNC,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it."]
    pub imr: IMR,
    #[doc = "0x1c - GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR."]
    pub ris: RIS,
    #[doc = "0x20 - GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR."]
    pub mis: MIS,
    #[doc = "0x24 - GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers."]
    pub icr: ICR,
    #[doc = "0x28 - GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR."]
    pub tailr: TAILR,
    #[doc = "0x2c - GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\] are used for the load value. Bits \\[31:16\\] are reserved in both cases."]
    pub tbilr: TBILR,
    #[doc = "0x30 - GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR."]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\] are used for the match value. Bits \\[31:16\\] are reserved in both cases."]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
    pub tapr: TAPR,
    #[doc = "0x3c - GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
    pub tbpr: TBPR,
    #[doc = "0x40 - GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
    pub tapmr: TAPMR,
    #[doc = "0x44 - GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\] contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\] always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\] in the TAV register."]
    pub tar: TAR,
    #[doc = "0x4c - GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\] contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\] always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\] in the TBV register."]
    pub tbr: TBR,
    #[doc = "0x50 - GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\] contain the value of the counter and bits \\[23:16\\] contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\] is a true prescaler, meaning bits \\[23:16\\] count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\] always read as 0."]
    pub tav: TAV,
    #[doc = "0x54 - GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\] contain the value of the counter and bits \\[23:16\\] contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\] is a true prescaler, meaning bits \\[23:16\\] count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\] always read as 0."]
    pub tbv: TBV,
    _reserved21: [u8; 4usize],
    #[doc = "0x5c - GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
    pub taps: TAPS,
    #[doc = "0x60 - GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
    pub tbps: TBPS,
    #[doc = "0x64 - GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
    pub tapv: TAPV,
    #[doc = "0x68 - GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
    pub tbpv: TBPV,
    _reserved25: [u8; 3924usize],
    #[doc = "0xfc0 - GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module."]
    pub pp: PP,
}
#[doc = "GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "GPTM configuration This register configures the global operation of the GPTM. The value written to this register determines whether the GPTM is in 32-bit mode (concatenated timers) or in 16-bit mode (individual, split timers)."]
pub mod cfg;
#[doc = "GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tamr](tamr) module"]
pub type TAMR = crate::Reg<u32, _TAMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMR;
#[doc = "`read()` method returns [tamr::R](tamr::R) reader structure"]
impl crate::Readable for TAMR {}
#[doc = "`write(|w| ..)` method takes [tamr::W](tamr::W) writer structure"]
impl crate::Writable for TAMR {}
#[doc = "GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored."]
pub mod tamr;
#[doc = "GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbmr](tbmr) module"]
pub type TBMR = crate::Reg<u32, _TBMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBMR;
#[doc = "`read()` method returns [tbmr::R](tbmr::R) reader structure"]
impl crate::Readable for TBMR {}
#[doc = "`write(|w| ..)` method takes [tbmr::W](tbmr::W) writer structure"]
impl crate::Writable for TBMR {}
#[doc = "GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B."]
pub mod tbmr;
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "GPTM control This register is used alongside the CFG and TnMR registers to fine-tune the timer configuration, and to enable other features such as timer stall."]
pub mod ctl;
#[doc = "GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers."]
pub mod sync;
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it."]
pub mod imr;
#[doc = "GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "GPTM raw interrupt status This register shows the state of the GPTM internal interrupt signal. These bits are set whether or not the interrupt is masked in the IMR register. Each bit can be cleared by writing 1 to its corresponding bit in ICR."]
pub mod ris;
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "GPTM masked interrupt status This register shows the state of the GPTM controller-level interrupt. If an interrupt is unmasked in IMR, and there is an event that causes the interrupt to be asserted, the corresponding bit is set in this register. All bits are cleared by writing 1 to the corresponding bit in ICR."]
pub mod mis;
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers."]
pub mod icr;
#[doc = "GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tailr](tailr) module"]
pub type TAILR = crate::Reg<u32, _TAILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAILR;
#[doc = "`read()` method returns [tailr::R](tailr::R) reader structure"]
impl crate::Readable for TAILR {}
#[doc = "`write(|w| ..)` method takes [tailr::W](tailr::W) writer structure"]
impl crate::Writable for TAILR {}
#[doc = "GPTM Timer A interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the timeout event. When a GPTM is configured to one of the 32-bit modes, TAILR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Interval Load (TBILR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBILR."]
pub mod tailr;
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\] are used for the load value. Bits \\[31:16\\] are reserved in both cases.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbilr](tbilr) module"]
pub type TBILR = crate::Reg<u32, _TBILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBILR;
#[doc = "`read()` method returns [tbilr::R](tbilr::R) reader structure"]
impl crate::Readable for TBILR {}
#[doc = "`write(|w| ..)` method takes [tbilr::W](tbilr::W) writer structure"]
impl crate::Writable for TBILR {}
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\] are used for the load value. Bits \\[31:16\\] are reserved in both cases."]
pub mod tbilr;
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tamatchr](tamatchr) module"]
pub type TAMATCHR = crate::Reg<u32, _TAMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAMATCHR;
#[doc = "`read()` method returns [tamatchr::R](tamatchr::R) reader structure"]
impl crate::Readable for TAMATCHR {}
#[doc = "`write(|w| ..)` method takes [tamatchr::W](tamatchr::W) writer structure"]
impl crate::Writable for TAMATCHR {}
#[doc = "GPTM Timer A match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B match (GPTMTBMATCHR) register). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR."]
pub mod tamatchr;
#[doc = "PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\] are used for the match value. Bits \\[31:16\\] are reserved in both cases.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbmatchr](tbmatchr) module"]
pub type TBMATCHR = crate::Reg<u32, _TBMATCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBMATCHR;
#[doc = "`read()` method returns [tbmatchr::R](tbmatchr::R) reader structure"]
impl crate::Readable for TBMATCHR {}
#[doc = "`write(|w| ..)` method takes [tbmatchr::W](tbmatchr::W) writer structure"]
impl crate::Writable for TBMATCHR {}
#[doc = "PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\] are used for the match value. Bits \\[31:16\\] are reserved in both cases."]
pub mod tbmatchr;
#[doc = "GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tapr](tapr) module"]
pub type TAPR = crate::Reg<u32, _TAPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPR;
#[doc = "`read()` method returns [tapr::R](tapr::R) reader structure"]
impl crate::Readable for TAPR {}
#[doc = "`write(|w| ..)` method takes [tapr::W](tapr::W) writer structure"]
impl crate::Writable for TAPR {}
#[doc = "GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
pub mod tapr;
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbpr](tbpr) module"]
pub type TBPR = crate::Reg<u32, _TBPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPR;
#[doc = "`read()` method returns [tbpr::R](tbpr::R) reader structure"]
impl crate::Readable for TBPR {}
#[doc = "`write(|w| ..)` method takes [tbpr::W](tbpr::W) writer structure"]
impl crate::Writable for TBPR {}
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes."]
pub mod tbpr;
#[doc = "GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tapmr](tapmr) module"]
pub type TAPMR = crate::Reg<u32, _TAPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPMR;
#[doc = "`read()` method returns [tapmr::R](tapmr::R) reader structure"]
impl crate::Readable for TAPMR {}
#[doc = "`write(|w| ..)` method takes [tapmr::W](tapmr::W) writer structure"]
impl crate::Writable for TAPMR {}
#[doc = "GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
pub mod tapmr;
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbpmr](tbpmr) module"]
pub type TBPMR = crate::Reg<u32, _TBPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPMR;
#[doc = "`read()` method returns [tbpmr::R](tbpmr::R) reader structure"]
impl crate::Readable for TBPMR {}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](tbpmr::W) writer structure"]
impl crate::Writable for TBPMR {}
#[doc = "GPTM Timer B prescale match This register effectively extends the range ofMTBMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode."]
pub mod tbpmr;
#[doc = "GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\] contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\] always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\] in the TAV register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "GPTM Timer A This register shows the current value of the Timer A counter. When a GPTM is configured to one of the 32-bit modes, TAR appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B (TBR) register). In the16-bit Input edge count, input edge time, and PWM modes, bits \\[15:0\\] contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits \\[31:24\\] always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\] in the TAV register."]
pub mod tar;
#[doc = "GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\] contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\] always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\] in the TBV register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbr](tbr) module"]
pub type TBR = crate::Reg<u32, _TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBR;
#[doc = "`read()` method returns [tbr::R](tbr::R) reader structure"]
impl crate::Readable for TBR {}
#[doc = "`write(|w| ..)` method takes [tbr::W](tbr::W) writer structure"]
impl crate::Writable for TBR {}
#[doc = "GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\] in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\] contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\] always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\] in the TBV register."]
pub mod tbr;
#[doc = "GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\] contain the value of the counter and bits \\[23:16\\] contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\] is a true prescaler, meaning bits \\[23:16\\] count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\] always read as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tav](tav) module"]
pub type TAV = crate::Reg<u32, _TAV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAV;
#[doc = "`read()` method returns [tav::R](tav::R) reader structure"]
impl crate::Readable for TAV {}
#[doc = "`write(|w| ..)` method takes [tav::W](tav::W) writer structure"]
impl crate::Writable for TAV {}
#[doc = "GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\] contain the value of the counter and bits \\[23:16\\] contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\] is a true prescaler, meaning bits \\[23:16\\] count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\] always read as 0."]
pub mod tav;
#[doc = "GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\] contain the value of the counter and bits \\[23:16\\] contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\] is a true prescaler, meaning bits \\[23:16\\] count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\] always read as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbv](tbv) module"]
pub type TBV = crate::Reg<u32, _TBV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBV;
#[doc = "`read()` method returns [tbv::R](tbv::R) reader structure"]
impl crate::Readable for TBV {}
#[doc = "`write(|w| ..)` method takes [tbv::W](tbv::W) writer structure"]
impl crate::Writable for TBV {}
#[doc = "GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\] contain the value of the counter and bits \\[23:16\\] contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\] is a true prescaler, meaning bits \\[23:16\\] count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\] always read as 0."]
pub mod tbv;
#[doc = "GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [taps](taps) module"]
pub type TAPS = crate::Reg<u32, _TAPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPS;
#[doc = "`read()` method returns [taps::R](taps::R) reader structure"]
impl crate::Readable for TAPS {}
#[doc = "`write(|w| ..)` method takes [taps::W](taps::W) writer structure"]
impl crate::Writable for TAPS {}
#[doc = "GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
pub mod taps;
#[doc = "GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbps](tbps) module"]
pub type TBPS = crate::Reg<u32, _TBPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPS;
#[doc = "`read()` method returns [tbps::R](tbps::R) reader structure"]
impl crate::Readable for TBPS {}
#[doc = "`write(|w| ..)` method takes [tbps::W](tbps::W) writer structure"]
impl crate::Writable for TBPS {}
#[doc = "GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode."]
pub mod tbps;
#[doc = "GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tapv](tapv) module"]
pub type TAPV = crate::Reg<u32, _TAPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAPV;
#[doc = "`read()` method returns [tapv::R](tapv::R) reader structure"]
impl crate::Readable for TAPV {}
#[doc = "`write(|w| ..)` method takes [tapv::W](tapv::W) writer structure"]
impl crate::Writable for TAPV {}
#[doc = "GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
pub mod tapv;
#[doc = "GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbpv](tbpv) module"]
pub type TBPV = crate::Reg<u32, _TBPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBPV;
#[doc = "`read()` method returns [tbpv::R](tbpv::R) reader structure"]
impl crate::Readable for TBPV {}
#[doc = "`write(|w| ..)` method takes [tbpv::W](tbpv::W) writer structure"]
impl crate::Writable for TBPV {}
#[doc = "GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode."]
pub mod tbpv;
#[doc = "GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "`write(|w| ..)` method takes [pp::W](pp::W) writer structure"]
impl crate::Writable for PP {}
#[doc = "GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module."]
pub mod pp;
