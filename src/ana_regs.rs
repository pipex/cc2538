#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Analog control register"]
    pub ivctrl: IVCTRL,
}
#[doc = "Analog control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ivctrl](ivctrl) module"]
pub type IVCTRL = crate::Reg<u32, _IVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IVCTRL;
#[doc = "`read()` method returns [ivctrl::R](ivctrl::R) reader structure"]
impl crate::Readable for IVCTRL {}
#[doc = "`write(|w| ..)` method takes [ivctrl::W](ivctrl::W) writer structure"]
impl crate::Writable for IVCTRL {}
#[doc = "Analog control register"]
pub mod ivctrl;
