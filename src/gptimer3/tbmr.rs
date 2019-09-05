#[doc = "Reader of register TBMR"]
pub type R = crate::R<u32, super::TBMR>;
#[doc = "Writer for register TBMR"]
pub type W = crate::W<u32, super::TBMR>;
#[doc = "Register TBMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBMR {
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
#[doc = "Reader of field `TBPLO`"]
pub type TBPLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPLO`"]
pub struct TBPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPLO_W<'a> {
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
#[doc = "Reader of field `TBMRSU`"]
pub type TBMRSU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMRSU`"]
pub struct TBMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMRSU_W<'a> {
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
#[doc = "Reader of field `TBPWMIE`"]
pub type TBPWMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPWMIE`"]
pub struct TBPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWMIE_W<'a> {
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
#[doc = "Reader of field `TBILD`"]
pub type TBILD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBILD`"]
pub struct TBILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILD_W<'a> {
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
#[doc = "Reader of field `TBSNAPS`"]
pub type TBSNAPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBSNAPS`"]
pub struct TBSNAPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSNAPS_W<'a> {
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
#[doc = "Reader of field `TBWOT`"]
pub type TBWOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBWOT`"]
pub struct TBWOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBWOT_W<'a> {
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
#[doc = "Reader of field `TBMIE`"]
pub type TBMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMIE`"]
pub struct TBMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIE_W<'a> {
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
#[doc = "Reader of field `TBCDIR`"]
pub type TBCDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCDIR`"]
pub struct TBCDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCDIR_W<'a> {
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
#[doc = "Reader of field `TBAMS`"]
pub type TBAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBAMS`"]
pub struct TBAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBAMS_W<'a> {
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
#[doc = "Reader of field `TBCMR`"]
pub type TBCMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCMR`"]
pub struct TBCMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCMR_W<'a> {
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
#[doc = "Reader of field `TBMR`"]
pub type TBMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBMR`"]
pub struct TBMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMR_W<'a> {
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
    pub fn tbplo(&self) -> TBPLO_R {
        TBPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\] Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TBMRSU_R {
        TBMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TBPWMIE_R {
        TBPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
    #[inline(always)]
    pub fn tbild(&self) -> TBILD_R {
        TBILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\] GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TBSNAPS_R {
        TBSNAPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
    #[inline(always)]
    pub fn tbwot(&self) -> TBWOT_R {
        TBWOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tbmie(&self) -> TBMIE_R {
        TBMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tbcdir(&self) -> TBCDIR_R {
        TBCDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn tbams(&self) -> TBAMS_R {
        TBAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tbcmr(&self) -> TBCMR_R {
        TBCMR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - 1:0\\] GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\] in the GPTMCFG register."]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0x03) as u8)
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
    pub fn tbplo(&mut self) -> TBPLO_W {
        TBPLO_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\] Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&mut self) -> TBMRSU_W {
        TBMRSU_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\] GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&mut self) -> TBPWMIE_W {
        TBPWMIE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\] GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
    #[inline(always)]
    pub fn tbild(&mut self) -> TBILD_W {
        TBILD_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
    #[inline(always)]
    pub fn tbsnaps(&mut self) -> TBSNAPS_W {
        TBSNAPS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
    #[inline(always)]
    pub fn tbwot(&mut self) -> TBWOT_W {
        TBWOT_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tbmie(&mut self) -> TBMIE_W {
        TBMIE_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tbcdir(&mut self) -> TBCDIR_W {
        TBCDIR_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn tbams(&mut self) -> TBAMS_W {
        TBAMS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tbcmr(&mut self) -> TBCMR_W {
        TBCMR_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\] GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\] in the GPTMCFG register."]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W {
        TBMR_W { w: self }
    }
}
