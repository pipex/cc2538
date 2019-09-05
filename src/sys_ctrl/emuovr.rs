#[doc = "Reader of register EMUOVR"]
pub type R = crate::R<u32, super::EMUOVR>;
#[doc = "Writer for register EMUOVR"]
pub type W = crate::W<u32, super::EMUOVR>;
#[doc = "Register EMUOVR `reset()`'s with value 0"]
impl crate::ResetValue for super::EMUOVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Reserved32`"]
pub type RESERVED32_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Reserved32`"]
pub struct RESERVED32_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ICEPICK_FORCE_CLOCK_CG`"]
pub type ICEPICK_FORCE_CLOCK_CG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEPICK_FORCE_CLOCK_CG`"]
pub struct ICEPICK_FORCE_CLOCK_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_CLOCK_CG_W<'a> {
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
#[doc = "Reader of field `ICEPICK_FORCE_POWER_CG`"]
pub type ICEPICK_FORCE_POWER_CG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEPICK_FORCE_POWER_CG`"]
pub struct ICEPICK_FORCE_POWER_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_POWER_CG_W<'a> {
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
#[doc = "Reader of field `ICEPICK_INHIBIT_SLEEP_CG`"]
pub type ICEPICK_INHIBIT_SLEEP_CG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEPICK_INHIBIT_SLEEP_CG`"]
pub struct ICEPICK_INHIBIT_SLEEP_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_INHIBIT_SLEEP_CG_W<'a> {
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
#[doc = "Reader of field `ICEMELTER_WKUP_CG`"]
pub type ICEMELTER_WKUP_CG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEMELTER_WKUP_CG`"]
pub struct ICEMELTER_WKUP_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEMELTER_WKUP_CG_W<'a> {
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
#[doc = "Reader of field `ICEPICK_FORCE_CLOCK_PM`"]
pub type ICEPICK_FORCE_CLOCK_PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEPICK_FORCE_CLOCK_PM`"]
pub struct ICEPICK_FORCE_CLOCK_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_CLOCK_PM_W<'a> {
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
#[doc = "Reader of field `ICEPICK_FORCE_POWER_PM`"]
pub type ICEPICK_FORCE_POWER_PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEPICK_FORCE_POWER_PM`"]
pub struct ICEPICK_FORCE_POWER_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_POWER_PM_W<'a> {
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
#[doc = "Reader of field `ICEPICK_INHIBIT_SLEEP_PM`"]
pub type ICEPICK_INHIBIT_SLEEP_PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEPICK_INHIBIT_SLEEP_PM`"]
pub struct ICEPICK_INHIBIT_SLEEP_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_INHIBIT_SLEEP_PM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ICEMELTER_WKUP_PM`"]
pub type ICEMELTER_WKUP_PM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICEMELTER_WKUP_PM`"]
pub struct ICEMELTER_WKUP_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEMELTER_WKUP_PM_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\] Unused. This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&self) -> RESERVED32_R {
        RESERVED32_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\] ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&self) -> ICEPICK_FORCE_CLOCK_CG_R {
        ICEPICK_FORCE_CLOCK_CG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\] ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&self) -> ICEPICK_FORCE_POWER_CG_R {
        ICEPICK_FORCE_POWER_CG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\] ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&self) -> ICEPICK_INHIBIT_SLEEP_CG_R {
        ICEPICK_INHIBIT_SLEEP_CG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\] ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&self) -> ICEMELTER_WKUP_CG_R {
        ICEMELTER_WKUP_CG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\] ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&self) -> ICEPICK_FORCE_CLOCK_PM_R {
        ICEPICK_FORCE_CLOCK_PM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\] ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&self) -> ICEPICK_FORCE_POWER_PM_R {
        ICEPICK_FORCE_POWER_PM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\] ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&self) -> ICEPICK_INHIBIT_SLEEP_PM_R {
        ICEPICK_INHIBIT_SLEEP_PM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\] ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&self) -> ICEMELTER_WKUP_PM_R {
        ICEMELTER_WKUP_PM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\] Unused. This register is 8 bits in a 32-bit address space."]
    #[inline(always)]
    pub fn reserved32(&mut self) -> RESERVED32_W {
        RESERVED32_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\] ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&mut self) -> ICEPICK_FORCE_CLOCK_CG_W {
        ICEPICK_FORCE_CLOCK_CG_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\] ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&mut self) -> ICEPICK_FORCE_POWER_CG_W {
        ICEPICK_FORCE_POWER_CG_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\] ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&mut self) -> ICEPICK_INHIBIT_SLEEP_CG_W {
        ICEPICK_INHIBIT_SLEEP_CG_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\] ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&mut self) -> ICEMELTER_WKUP_CG_W {
        ICEMELTER_WKUP_CG_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\] ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&mut self) -> ICEPICK_FORCE_CLOCK_PM_W {
        ICEPICK_FORCE_CLOCK_PM_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\] ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&mut self) -> ICEPICK_FORCE_POWER_PM_W {
        ICEPICK_FORCE_POWER_PM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\] ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&mut self) -> ICEPICK_INHIBIT_SLEEP_PM_W {
        ICEPICK_INHIBIT_SLEEP_PM_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\] ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&mut self) -> ICEMELTER_WKUP_PM_W {
        ICEMELTER_WKUP_PM_W { w: self }
    }
}
