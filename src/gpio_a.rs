#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs."]
    pub data: DATA,
    _reserved1: [u8; 1020usize],
    #[doc = "0x400 - The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default."]
    pub dir: DIR,
    #[doc = "0x404 - The IS register is the interrupt sense register."]
    pub is: IS,
    #[doc = "0x408 - The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV."]
    pub ibe: IBE,
    #[doc = "0x40c - The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS."]
    pub iev: IEV,
    #[doc = "0x410 - The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin."]
    pub ie: IE,
    #[doc = "0x414 - The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt."]
    pub ris: RIS,
    #[doc = "0x418 - The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking."]
    pub mis: MIS,
    #[doc = "0x41c - The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect."]
    pub ic: IC,
    #[doc = "0x420 - The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default."]
    pub afsel: AFSEL,
    _reserved10: [u8; 252usize],
    #[doc = "0x520 - A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked."]
    pub gpiolock: GPIOLOCK,
    #[doc = "0x524 - The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked."]
    pub gpiocr: GPIOCR,
    _reserved12: [u8; 472usize],
    #[doc = "0x700 - The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled."]
    pub pmux: PMUX,
    #[doc = "0x704 - The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system."]
    pub p_edge_ctrl: P_EDGE_CTRL,
    #[doc = "0x708 - This register is used to control which edge of the USB controller input generates a power-up interrupt to the system."]
    pub usb_ctrl: USB_CTRL,
    _reserved15: [u8; 4usize],
    #[doc = "0x710 - The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled."]
    pub pi_ien: PI_IEN,
    _reserved16: [u8; 4usize],
    #[doc = "0x718 - If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1."]
    pub irq_detect_ack: IRQ_DETECT_ACK,
    #[doc = "0x71c - Same functionality as IRQ_DETECT_ACK, but for USB"]
    pub usb_irq_ack: USB_IRQ_ACK,
    #[doc = "0x720 - Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts"]
    pub irq_detect_unmask: IRQ_DETECT_UNMASK,
}
#[doc = "This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "This is the data register. In software control mode, values written in the GPIODATA register are transferred onto the GPOUT pins if the respective pins have been configured as outputs through the GPIODIR register. A read from GPIODATA returns the last bit value written if the respective pins are configured as output, or it returns the value on the corresponding input GPIN bit when these are configured as inputs."]
pub mod data;
#[doc = "The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "The DIR register is the data direction register. All bits are cleared by a reset; therefore, the GPIO pins are input by default."]
pub mod dir;
#[doc = "The IS register is the interrupt sense register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [is](is) module"]
pub type IS = crate::Reg<u32, _IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS;
#[doc = "`read()` method returns [is::R](is::R) reader structure"]
impl crate::Readable for IS {}
#[doc = "`write(|w| ..)` method takes [is::W](is::W) writer structure"]
impl crate::Writable for IS {}
#[doc = "The IS register is the interrupt sense register."]
pub mod is;
#[doc = "The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ibe](ibe) module"]
pub type IBE = crate::Reg<u32, _IBE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBE;
#[doc = "`read()` method returns [ibe::R](ibe::R) reader structure"]
impl crate::Readable for IBE {}
#[doc = "`write(|w| ..)` method takes [ibe::W](ibe::W) writer structure"]
impl crate::Writable for IBE {}
#[doc = "The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV."]
pub mod ibe;
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iev](iev) module"]
pub type IEV = crate::Reg<u32, _IEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEV;
#[doc = "`read()` method returns [iev::R](iev::R) reader structure"]
impl crate::Readable for IEV {}
#[doc = "`write(|w| ..)` method takes [iev::W](iev::W) writer structure"]
impl crate::Writable for IEV {}
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS."]
pub mod iev;
#[doc = "The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "The IE register is the interrupt mask register. Bits set to high in IE allow the corresponding pins to trigger their individual interrupts and the combined GPIOINTR line. Clearing a bit disables interrupt triggering on that pin."]
pub mod ie;
#[doc = "The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "The RIS register is the raw interrupt status register. Bits read high in RIS reflect the status of interrupts trigger conditions detected (raw, before masking), indicating that all the requirements are met, before they are finally allowed to trigger by IE. Bits read as 0 indicate that corresponding input pins have not initiated an interrupt."]
pub mod ris;
#[doc = "The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "`write(|w| ..)` method takes [mis::W](mis::W) writer structure"]
impl crate::Writable for MIS {}
#[doc = "The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking."]
pub mod mis;
#[doc = "The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ic](ic) module"]
pub type IC = crate::Reg<u32, _IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC;
#[doc = "`read()` method returns [ic::R](ic::R) reader structure"]
impl crate::Readable for IC {}
#[doc = "`write(|w| ..)` method takes [ic::W](ic::W) writer structure"]
impl crate::Writable for IC {}
#[doc = "The IC register is the interrupt clear register. Writing 1 to a bit in this register clears the corresponding interrupt edge detection logic register. Writing 0 has no effect."]
pub mod ic;
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [afsel](afsel) module"]
pub type AFSEL = crate::Reg<u32, _AFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSEL;
#[doc = "`read()` method returns [afsel::R](afsel::R) reader structure"]
impl crate::Readable for AFSEL {}
#[doc = "`write(|w| ..)` method takes [afsel::W](afsel::W) writer structure"]
impl crate::Writable for AFSEL {}
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default."]
pub mod afsel;
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiolock](gpiolock) module"]
pub type GPIOLOCK = crate::Reg<u32, _GPIOLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOLOCK;
#[doc = "`read()` method returns [gpiolock::R](gpiolock::R) reader structure"]
impl crate::Readable for GPIOLOCK {}
#[doc = "`write(|w| ..)` method takes [gpiolock::W](gpiolock::W) writer structure"]
impl crate::Writable for GPIOLOCK {}
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked."]
pub mod gpiolock;
#[doc = "The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpiocr](gpiocr) module"]
pub type GPIOCR = crate::Reg<u32, _GPIOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOCR;
#[doc = "`read()` method returns [gpiocr::R](gpiocr::R) reader structure"]
impl crate::Readable for GPIOCR {}
#[doc = "`write(|w| ..)` method takes [gpiocr::W](gpiocr::W) writer structure"]
impl crate::Writable for GPIOCR {}
#[doc = "The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked."]
pub mod gpiocr;
#[doc = "The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pmux](pmux) module"]
pub type PMUX = crate::Reg<u32, _PMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMUX;
#[doc = "`read()` method returns [pmux::R](pmux::R) reader structure"]
impl crate::Readable for PMUX {}
#[doc = "`write(|w| ..)` method takes [pmux::W](pmux::W) writer structure"]
impl crate::Writable for PMUX {}
#[doc = "The PMUX register can be used to output external decouple control and clock_32k on I/O pins. Decouple control can be output on specific PB pins and clock_32k can be output on a specific PA or PB pin. These features override the current setting of the selected pin when enabled. The pin is set to output, pull-up and -down disabled, and analog mode disabled."]
pub mod pmux;
#[doc = "The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p_edge_ctrl](p_edge_ctrl) module"]
pub type P_EDGE_CTRL = crate::Reg<u32, _P_EDGE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P_EDGE_CTRL;
#[doc = "`read()` method returns [p_edge_ctrl::R](p_edge_ctrl::R) reader structure"]
impl crate::Readable for P_EDGE_CTRL {}
#[doc = "`write(|w| ..)` method takes [p_edge_ctrl::W](p_edge_ctrl::W) writer structure"]
impl crate::Writable for P_EDGE_CTRL {}
#[doc = "The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system."]
pub mod p_edge_ctrl;
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_ctrl](usb_ctrl) module"]
pub type USB_CTRL = crate::Reg<u32, _USB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_CTRL;
#[doc = "`read()` method returns [usb_ctrl::R](usb_ctrl::R) reader structure"]
impl crate::Readable for USB_CTRL {}
#[doc = "`write(|w| ..)` method takes [usb_ctrl::W](usb_ctrl::W) writer structure"]
impl crate::Writable for USB_CTRL {}
#[doc = "This register is used to control which edge of the USB controller input generates a power-up interrupt to the system."]
pub mod usb_ctrl;
#[doc = "The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pi_ien](pi_ien) module"]
pub type PI_IEN = crate::Reg<u32, _PI_IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_IEN;
#[doc = "`read()` method returns [pi_ien::R](pi_ien::R) reader structure"]
impl crate::Readable for PI_IEN {}
#[doc = "`write(|w| ..)` method takes [pi_ien::W](pi_ien::W) writer structure"]
impl crate::Writable for PI_IEN {}
#[doc = "The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled."]
pub mod pi_ien;
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq_detect_ack](irq_detect_ack) module"]
pub type IRQ_DETECT_ACK = crate::Reg<u32, _IRQ_DETECT_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_DETECT_ACK;
#[doc = "`read()` method returns [irq_detect_ack::R](irq_detect_ack::R) reader structure"]
impl crate::Readable for IRQ_DETECT_ACK {}
#[doc = "`write(|w| ..)` method takes [irq_detect_ack::W](irq_detect_ack::W) writer structure"]
impl crate::Writable for IRQ_DETECT_ACK {}
#[doc = "If the IRQ detect ACK register is read, the value returned can be used to determine which enabled I/O port is responsible for creating a power-up interrupt to the system. Writing the IRQ detect ACK register is used to clear any number of individual port bits that may be signaling that an edge was detected as configured by the port edge control register and the interrupt control register. There is a self-clearing function to this register that generates a reset pulse to clear any interrupt which has its corresponding bit set to 1."]
pub mod irq_detect_ack;
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usb_irq_ack](usb_irq_ack) module"]
pub type USB_IRQ_ACK = crate::Reg<u32, _USB_IRQ_ACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USB_IRQ_ACK;
#[doc = "`read()` method returns [usb_irq_ack::R](usb_irq_ack::R) reader structure"]
impl crate::Readable for USB_IRQ_ACK {}
#[doc = "`write(|w| ..)` method takes [usb_irq_ack::W](usb_irq_ack::W) writer structure"]
impl crate::Writable for USB_IRQ_ACK {}
#[doc = "Same functionality as IRQ_DETECT_ACK, but for USB"]
pub mod usb_irq_ack;
#[doc = "Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irq_detect_unmask](irq_detect_unmask) module"]
pub type IRQ_DETECT_UNMASK = crate::Reg<u32, _IRQ_DETECT_UNMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_DETECT_UNMASK;
#[doc = "`read()` method returns [irq_detect_unmask::R](irq_detect_unmask::R) reader structure"]
impl crate::Readable for IRQ_DETECT_UNMASK {}
#[doc = "`write(|w| ..)` method takes [irq_detect_unmask::W](irq_detect_unmask::W) writer structure"]
impl crate::Writable for IRQ_DETECT_UNMASK {}
#[doc = "Same functionality as IRQ_DETECT_ACK, but this register handles masked interrupts"]
pub mod irq_detect_unmask;
