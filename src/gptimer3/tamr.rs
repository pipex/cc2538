#[doc = "Reader of register TAMR"]
pub type R = crate::R<u32, super::TAMR>;
#[doc = "Writer for register TAMR"]
pub type W = crate::W<u32, super::TAMR>;
#[doc = "Register TAMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TAMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved12`"]
pub type RESERVED12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `TAPLO`"]
pub type TAPLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAPLO`"]
pub struct TAPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPLO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TAMRSU`"]
pub type TAMRSU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMRSU`"]
pub struct TAMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMRSU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TAPWMIE`"]
pub type TAPWMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAPWMIE`"]
pub struct TAPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWMIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TAILD`"]
pub type TAILD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAILD`"]
pub struct TAILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TASNAPS`"]
pub type TASNAPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TASNAPS`"]
pub struct TASNAPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TASNAPS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TAWOT`"]
pub type TAWOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAWOT`"]
pub struct TAWOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAWOT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TAMIE`"]
pub type TAMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMIE`"]
pub struct TAMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TACDIR`"]
pub type TACDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACDIR`"]
pub struct TACDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACDIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TAAMS`"]
pub type TAAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAAMS`"]
pub struct TAAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAAMS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TACMR`"]
pub type TACMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TACMR`"]
pub struct TACMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TACMR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TAMR`"]
pub type TAMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAMR`"]
pub struct TAMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\] Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 11 - 11:11\\] Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn taplo(&self) -> TAPLO_R {
        TAPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&self) -> TAMRSU_R {
        TAMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&self) -> TAPWMIE_R {
        TAPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
    #[inline(always)]
    pub fn taild(&self) -> TAILD_R {
        TAILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
    #[inline(always)]
    pub fn tasnaps(&self) -> TASNAPS_R {
        TASNAPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
    #[inline(always)]
    pub fn tawot(&self) -> TAWOT_R {
        TAWOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tamie(&self) -> TAMIE_R {
        TAMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tacdir(&self) -> TACDIR_R {
        TACDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn taams(&self) -> TAAMS_R {
        TAAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tacmr(&self) -> TACMR_R {
        TACMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\] GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\] in the GPTMCFG register."]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\] Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\] Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn taplo(&mut self) -> TAPLO_W {
        TAPLO_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&mut self) -> TAMRSU_W {
        TAMRSU_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&mut self) -> TAPWMIE_W {
        TAPWMIE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
    #[inline(always)]
    pub fn taild(&mut self) -> TAILD_W {
        TAILD_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
    #[inline(always)]
    pub fn tasnaps(&mut self) -> TASNAPS_W {
        TASNAPS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
    #[inline(always)]
    pub fn tawot(&mut self) -> TAWOT_W {
        TAWOT_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tamie(&mut self) -> TAMIE_W {
        TAMIE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tacdir(&mut self) -> TACDIR_W {
        TACDIR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn taams(&mut self) -> TAAMS_W {
        TAAMS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tacmr(&mut self) -> TACMR_W {
        TACMR_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\] in the GPTMCFG register."]
    #[inline(always)]
    pub fn tamr(&mut self) -> TAMR_W {
        TAMR_W { w: self }
    }
}
